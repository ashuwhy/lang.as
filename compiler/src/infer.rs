//! Candidate loop invariants for Houdini-style inference. The verifier keeps only the candidates
//! that hold when the loop starts and survive every iteration; the survivors are then proved
//! like hand-written invariants, so a bad guess can cost time but never soundness.

use std::collections::HashSet;

use crate::ast::BinOp;
use crate::diag::Span;
use crate::tir::*;

/// Stands for the bound variable `t` in generated `forall t in 0..a.len: ...` candidates.
pub const QUANT_VAR: LocalId = u32::MAX - 1;

#[derive(Clone)]
pub struct Cand {
    pub e: TExpr,
    pub text: String,
    /// Candidates in the same group bound the same thing; only the tightest survivor is kept.
    /// The number is the strength: larger is tighter.
    pub group: Option<(String, i128)>,
}

/// Keep the tightest candidate of each group, and every candidate without a group.
pub fn tightest(cands: Vec<Cand>) -> Vec<Cand> {
    let mut best: std::collections::HashMap<String, i128> = std::collections::HashMap::new();
    for c in &cands {
        if let Some((g, k)) = &c.group {
            let e = best.entry(g.clone()).or_insert(*k);
            if *k > *e {
                *e = *k;
            }
        }
    }
    let mut taken = HashSet::new();
    cands
        .into_iter()
        .filter(|c| match &c.group {
            None => true,
            Some((g, k)) => best[g] == *k && taken.insert(g.clone()),
        })
        .collect()
}

fn int(v: i128, sp: Span) -> TExpr {
    TExpr { kind: TExprKind::Int(v), ty: Ty::Int, span: sp }
}

fn bin(op: BinOp, a: TExpr, b: TExpr, ty: Ty, sp: Span) -> TExpr {
    TExpr { kind: TExprKind::Binary(op, Box::new(a), Box::new(b)), ty, span: sp }
}

fn num(v: i128) -> String {
    let s = v.abs().to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 && s.len() > 4 {
            out.push('_');
        }
        out.push(c);
    }
    if v < 0 { format!("-{out}") } else { out }
}

/// Integer literals of a function, used as the constants in candidate templates.
pub fn constants(m: &Module, f: &Func) -> Vec<i128> {
    fn walk(e: &TExpr, out: &mut Vec<i128>) {
        match &e.kind {
            TExprKind::Int(v) => {
                if v.abs() < (1i128 << 62) {
                    out.push(v.abs());
                }
            }
            TExprKind::Field(x, _) | TExprKind::Unary(_, x) | TExprKind::Coerce(x, _) | TExprKind::Len(x) | TExprKind::Is(x, _) => walk(x, out),
            TExprKind::Binary(_, a, b) | TExprKind::Index(a, b) | TExprKind::ArrayRepeat(a, b) => {
                walk(a, out);
                walk(b, out);
            }
            TExprKind::Quant { lo, hi, body, .. } => {
                walk(lo, out);
                walk(hi, out);
                walk(body, out);
            }
            TExprKind::If(c, t, f) => {
                walk(c, out);
                walk(t, out);
                walk(f, out);
            }
            TExprKind::Match(s, arms) => {
                walk(s, out);
                arms.iter().for_each(|(_, b)| walk(b, out));
            }
            TExprKind::Record(xs) | TExprKind::Ctor(_, xs) | TExprKind::Call(_, xs) | TExprKind::Print(xs) | TExprKind::ArrayLit(xs) => xs.iter().for_each(|x| walk(x, out)),
            TExprKind::Update(b, fs) => {
                walk(b, out);
                fs.iter().for_each(|(_, x)| walk(x, out));
            }
            TExprKind::Block(ss, t) => {
                for s in ss {
                    match s {
                        TStmt::Let(_, v) | TStmt::Assign(_, v) | TStmt::Expr(v) | TStmt::Return(v, _) | TStmt::Push(_, v, _) => walk(v, out),
                        TStmt::IndexAssign(_, i, v, _) => {
                            walk(i, out);
                            walk(v, out);
                        }
                        TStmt::While { cond, invariants, body, .. } => {
                            walk(cond, out);
                            invariants.iter().for_each(|x| walk(x, out));
                            walk(body, out);
                        }
                    }
                }
                walk(t, out);
            }
            _ => {}
        }
    }
    let mut out = vec![0, 1];
    for e in f.requires.iter().chain(&f.ensures).chain(std::iter::once(&f.body)) {
        walk(e, &mut out);
    }
    let _ = m;
    let mut seen = HashSet::new();
    out.retain(|v| seen.insert(*v));
    out.truncate(16);
    out
}

/// A term the candidates may compare against: a variable or an array length.
pub struct Term {
    pub e: TExpr,
    pub text: String,
}

/// Variables updated as `v = v + e` (accumulators) and as `v = v + 1` (counters) in a loop body.
fn shapes(body: &TExpr) -> (HashSet<LocalId>, HashSet<LocalId>) {
    fn go(e: &TExpr, acc: &mut HashSet<LocalId>, cnt: &mut HashSet<LocalId>) {
        match &e.kind {
            TExprKind::Block(ss, t) => {
                for s in ss {
                    match s {
                        TStmt::Assign(id, v) => {
                            if let TExprKind::Binary(BinOp::Add, l, r) = &v.kind {
                                if matches!(l.kind, TExprKind::Local(x) if x == *id) {
                                    if matches!(r.kind, TExprKind::Int(1)) {
                                        cnt.insert(*id);
                                    } else {
                                        acc.insert(*id);
                                    }
                                }
                            }
                            go(v, acc, cnt);
                        }
                        TStmt::Let(_, v) | TStmt::Expr(v) | TStmt::Return(v, _) | TStmt::Push(_, v, _) => go(v, acc, cnt),
                        TStmt::IndexAssign(_, i, v, _) => {
                            go(i, acc, cnt);
                            go(v, acc, cnt);
                        }
                        TStmt::While { body, .. } => go(body, acc, cnt),
                    }
                }
                go(t, acc, cnt);
            }
            TExprKind::If(c, t, f) => {
                go(c, acc, cnt);
                go(t, acc, cnt);
                go(f, acc, cnt);
            }
            TExprKind::Match(s, arms) => {
                go(s, acc, cnt);
                arms.iter().for_each(|(_, b)| go(b, acc, cnt));
            }
            _ => {}
        }
    }
    let (mut acc, mut cnt) = (HashSet::new(), HashSet::new());
    go(body, &mut acc, &mut cnt);
    (acc, cnt)
}

pub fn candidates(m: &Module, consts: &[i128], modified: &[LocalId], scope: &[Term], body: &TExpr, sp: Span) -> Vec<Cand> {
    let (accumulators, counters) = shapes(body);
    let is_int = |id: LocalId| m.peel(&m.locals[id as usize].ty) == Ty::Int;
    let local = |id: LocalId| TExpr { kind: TExprKind::Local(id), ty: m.locals[id as usize].ty.clone(), span: sp };
    let name = |id: LocalId| m.locals[id as usize].name.clone();
    let mut products: Vec<i128> = vec![];
    for (i, a) in consts.iter().enumerate() {
        for b in &consts[i..] {
            if *a > 1 && *b > 1 && a * b < (1i128 << 62) {
                products.push(a * b);
            }
        }
    }
    let mut scale: Vec<i128> = consts.iter().copied().filter(|c| *c > 1).chain(products).collect();
    let mut seen = HashSet::new();
    scale.retain(|v| seen.insert(*v));
    scale.truncate(24);

    let mut out = vec![];
    let cmp = |op: BinOp, a: TExpr, b: TExpr| bin(op, a, b, Ty::Bool, sp);
    let ints: Vec<LocalId> = modified.iter().copied().filter(|id| is_int(*id)).collect();
    for &v in &ints {
        let vn = name(v);
        for &c in consts {
            out.push(Cand { e: cmp(BinOp::Ge, local(v), int(c, sp)), text: format!("{vn} >= {}", num(c)), group: Some((format!("{v}>="), c)) });
            if c > 0 {
                out.push(Cand { e: cmp(BinOp::Le, local(v), int(c, sp)), text: format!("{vn} <= {}", num(c)), group: Some((format!("{v}<="), -c)) });
                out.push(Cand { e: cmp(BinOp::Lt, local(v), int(c, sp)), text: format!("{vn} < {}", num(c)), group: Some((format!("{v}<="), -(c - 1))) });
            }
        }
        let others = scope.iter().map(|t| (t.e.clone(), t.text.clone())).chain(ints.iter().filter(|w| **w != v).map(|w| (local(*w), name(*w))));
        for (w, wn) in others.collect::<Vec<_>>() {
            out.push(Cand { e: cmp(BinOp::Ge, local(v), w.clone()), text: format!("{vn} >= {wn}"), group: None });
            out.push(Cand { e: cmp(BinOp::Le, local(v), w.clone()), text: format!("{vn} <= {wn}"), group: Some((format!("{v}<=var{wn}"), 0)) });
            out.push(Cand { e: cmp(BinOp::Lt, local(v), w.clone()), text: format!("{vn} < {wn}"), group: Some((format!("{v}<=var{wn}"), 1)) });
            // `s <= i * C`: an accumulator grows at most C per step of a counter.
            let w_is_counter = matches!(w.kind, TExprKind::Local(x) if counters.contains(&x));
            if accumulators.contains(&v) && w_is_counter {
                for &c in &scale {
                    let prod = bin(BinOp::Mul, w.clone(), int(c, sp), Ty::Int, sp);
                    out.push(Cand { e: cmp(BinOp::Le, local(v), prod), text: format!("{vn} <= {wn} * {}", num(c)), group: Some((format!("{v}<={wn}*"), -c)) });
                }
            }
        }
    }
    // Element ranges of arrays the loop writes.
    for &a in modified {
        let Ty::Array(elem) = m.peel(&m.locals[a as usize].ty) else { continue };
        if m.peel(&elem) != Ty::Int {
            continue;
        }
        let an = name(a);
        let t = TExpr { kind: TExprKind::Local(QUANT_VAR), ty: Ty::Int, span: sp };
        let at = TExpr { kind: TExprKind::Index(Box::new(local(a)), Box::new(t)), ty: Ty::Int, span: sp };
        let len = TExpr { kind: TExprKind::Len(Box::new(local(a))), ty: Ty::Int, span: sp };
        let forall = |body: TExpr| TExpr { kind: TExprKind::Quant { forall: true, var: QUANT_VAR, lo: Box::new(int(0, sp)), hi: Box::new(len.clone()), body: Box::new(body) }, ty: Ty::Bool, span: sp };
        let mut bodies: Vec<(TExpr, String, Option<(String, i128)>)> = vec![];
        for &c in consts {
            bodies.push((cmp(BinOp::Ge, at.clone(), int(c, sp)), format!("{an}[t] >= {}", num(c)), Some((format!("{a}[]>="), c))));
            if c > 0 {
                bodies.push((cmp(BinOp::Le, at.clone(), int(c, sp)), format!("{an}[t] <= {}", num(c)), Some((format!("{a}[]<="), -c))));
                bodies.push((cmp(BinOp::Lt, at.clone(), int(c, sp)), format!("{an}[t] < {}", num(c)), Some((format!("{a}[]<="), -(c - 1)))));
            }
        }
        // Scaled bounds only for the largest few constants, to keep quantified queries cheap.
        let mut big: Vec<i128> = scale.clone();
        big.sort_by(|a, b| b.cmp(a));
        big.truncate(3);
        for t in scope.iter().filter(|t| matches!(t.e.kind, TExprKind::Local(_))) {
            for &c in &big {
                let prod = bin(BinOp::Mul, t.e.clone(), int(c, sp), Ty::Int, sp);
                bodies.push((cmp(BinOp::Le, at.clone(), prod), format!("{an}[t] <= {} * {}", t.text, num(c)), Some((format!("{a}[]<={}*", t.text), -c))));
            }
        }
        for (b, text, group) in bodies {
            out.push(Cand { e: forall(b), text: format!("forall t in 0..{an}.len: {text}"), group });
        }
    }
    out
}

/// Candidate preconditions (over the parameters) and postconditions (over `result` and the
/// parameters) of a private function. A precondition is kept only if every call site
/// establishes it, a postcondition only if every return proves it. Candidates come in order of
/// preference: relations between terms first, then constant bounds, then element ranges.
pub fn contract_candidates(m: &Module, f: &Func, consts: &[i128], sp: Span) -> (Vec<Cand>, Vec<Cand>) {
    let local = |id: LocalId| TExpr { kind: TExprKind::Local(id), ty: m.locals[id as usize].ty.clone(), span: sp };
    let name = |id: LocalId| m.locals[id as usize].name.clone();
    let cmp = |op: BinOp, a: TExpr, b: TExpr| bin(op, a, b, Ty::Bool, sp);
    let len_of = |e: TExpr| TExpr { kind: TExprKind::Len(Box::new(e)), ty: Ty::Int, span: sp };
    let int_params: Vec<LocalId> = f.params.iter().copied().filter(|p| m.peel(&m.locals[*p as usize].ty) == Ty::Int).collect();
    let arr_params: Vec<LocalId> = f.params.iter().copied().filter(|p| matches!(m.peel(&m.locals[*p as usize].ty), Ty::Array(ref e) if m.peel(e) == Ty::Int)).collect();

    // Integer terms about the inputs: each int parameter, each array length, n * m products.
    let mut terms: Vec<(TExpr, String)> = int_params.iter().map(|p| (local(*p), name(*p))).collect();
    terms.extend(arr_params.iter().map(|p| (len_of(local(*p)), format!("{}.len", name(*p)))));
    let mut products = vec![];
    for (i, p) in int_params.iter().enumerate() {
        for q in &int_params[i..] {
            products.push((bin(BinOp::Mul, local(*p), local(*q), Ty::Int, sp), format!("{} * {}", name(*p), name(*q))));
        }
    }

    let relations = |a: &(TExpr, String), b: &(TExpr, String), out: &mut Vec<Cand>| {
        let key = format!("{}~{}", a.1, b.1);
        out.push(Cand { e: cmp(BinOp::Eq, a.0.clone(), b.0.clone()), text: format!("{} == {}", a.1, b.1), group: None });
        out.push(Cand { e: cmp(BinOp::Lt, a.0.clone(), b.0.clone()), text: format!("{} < {}", a.1, b.1), group: Some((format!("{key}<"), 1)) });
        out.push(Cand { e: cmp(BinOp::Le, a.0.clone(), b.0.clone()), text: format!("{} <= {}", a.1, b.1), group: Some((format!("{key}<"), 0)) });
        out.push(Cand { e: cmp(BinOp::Gt, a.0.clone(), b.0.clone()), text: format!("{} > {}", a.1, b.1), group: Some((format!("{key}>"), 1)) });
        out.push(Cand { e: cmp(BinOp::Ge, a.0.clone(), b.0.clone()), text: format!("{} >= {}", a.1, b.1), group: Some((format!("{key}>"), 0)) });
    };
    let bounds = |a: &(TExpr, String), out: &mut Vec<Cand>| {
        for &c in consts {
            out.push(Cand { e: cmp(BinOp::Ge, a.0.clone(), int(c, sp)), text: format!("{} >= {}", a.1, num(c)), group: Some((format!("{}>=", a.1), c)) });
            if c > 0 {
                out.push(Cand { e: cmp(BinOp::Le, a.0.clone(), int(c, sp)), text: format!("{} <= {}", a.1, num(c)), group: Some((format!("{}<=", a.1), -c)) });
            }
        }
    };
    // Element ranges of an array: constant bounds, and `<= n * C` for an int parameter n.
    let elements = |arr: TExpr, an: &str, out: &mut Vec<Cand>| {
        let t = TExpr { kind: TExprKind::Local(QUANT_VAR), ty: Ty::Int, span: sp };
        let at = TExpr { kind: TExprKind::Index(Box::new(arr.clone()), Box::new(t)), ty: Ty::Int, span: sp };
        let len = len_of(arr);
        let forall = |body: TExpr| TExpr { kind: TExprKind::Quant { forall: true, var: QUANT_VAR, lo: Box::new(int(0, sp)), hi: Box::new(len.clone()), body: Box::new(body) }, ty: Ty::Bool, span: sp };
        let each = |b: String| format!("forall t in 0..{an}.len: {an}[t] {b}");
        out.push(Cand { e: forall(cmp(BinOp::Ge, at.clone(), int(0, sp))), text: each(">= 0".into()), group: None });
        for &c in consts.iter().filter(|c| **c > 0) {
            out.push(Cand { e: forall(cmp(BinOp::Le, at.clone(), int(c, sp))), text: each(format!("<= {}", num(c))), group: Some((format!("{an}[]<="), -c)) });
        }
        for p in &int_params {
            for &c in consts.iter().filter(|c| **c > 1) {
                let prod = bin(BinOp::Mul, local(*p), int(c, sp), Ty::Int, sp);
                out.push(Cand { e: forall(cmp(BinOp::Le, at.clone(), prod)), text: each(format!("<= {} * {}", name(*p), num(c))), group: Some((format!("{an}[]<={}*", name(*p)), -c)) });
            }
        }
    };

    let mut pre = vec![];
    for (i, a) in terms.iter().enumerate() {
        for b in &terms[i + 1..] {
            relations(a, b, &mut pre);
        }
    }
    for a in &terms {
        for p in &products {
            if !p.1.split(" * ").any(|x| x == a.1) {
                pre.push(Cand { e: cmp(BinOp::Eq, a.0.clone(), p.0.clone()), text: format!("{} == {}", a.1, p.1), group: None });
            }
        }
    }
    for a in &terms {
        bounds(a, &mut pre);
    }
    for p in &arr_params {
        elements(local(*p), &name(*p), &mut pre);
    }

    let mut post = vec![];
    let result = TExpr { kind: TExprKind::Local(f.result), ty: f.ret.clone(), span: sp };
    let subject = match m.peel(&f.ret) {
        Ty::Int => Some((result.clone(), "result".to_string())),
        Ty::Array(e) if m.peel(&e) == Ty::Int => Some((len_of(result.clone()), "result.len".to_string())),
        _ => None,
    };
    if let Some(r) = subject {
        for t in terms.iter().chain(&products) {
            relations(&r, t, &mut post);
        }
        bounds(&r, &mut post);
        if r.1 == "result.len" {
            elements(result, "result", &mut post);
        }
    }
    (pre, post)
}

/// Functions called anywhere in `e`.
pub fn callees(e: &TExpr) -> HashSet<usize> {
    fn walk(e: &TExpr, out: &mut HashSet<usize>) {
        match &e.kind {
            TExprKind::Call(fi, xs) => {
                out.insert(*fi);
                xs.iter().for_each(|x| walk(x, out));
            }
            TExprKind::Field(x, _) | TExprKind::Unary(_, x) | TExprKind::Coerce(x, _) | TExprKind::Len(x) | TExprKind::Is(x, _) => walk(x, out),
            TExprKind::Binary(_, a, b) | TExprKind::Index(a, b) | TExprKind::ArrayRepeat(a, b) => {
                walk(a, out);
                walk(b, out);
            }
            TExprKind::Quant { lo, hi, body, .. } => {
                walk(lo, out);
                walk(hi, out);
                walk(body, out);
            }
            TExprKind::If(c, t, f) => {
                walk(c, out);
                walk(t, out);
                walk(f, out);
            }
            TExprKind::Match(s, arms) => {
                walk(s, out);
                arms.iter().for_each(|(_, b)| walk(b, out));
            }
            TExprKind::Record(xs) | TExprKind::Ctor(_, xs) | TExprKind::Print(xs) | TExprKind::ArrayLit(xs) => xs.iter().for_each(|x| walk(x, out)),
            TExprKind::Update(b, fs) => {
                walk(b, out);
                fs.iter().for_each(|(_, x)| walk(x, out));
            }
            TExprKind::Block(ss, t) => {
                for s in ss {
                    match s {
                        TStmt::Let(_, v) | TStmt::Assign(_, v) | TStmt::Expr(v) | TStmt::Return(v, _) | TStmt::Push(_, v, _) => walk(v, out),
                        TStmt::IndexAssign(_, i, v, _) => {
                            walk(i, out);
                            walk(v, out);
                        }
                        TStmt::While { cond, body, .. } => {
                            walk(cond, out);
                            walk(body, out);
                        }
                    }
                }
                walk(t, out);
            }
            _ => {}
        }
    }
    let mut out = HashSet::new();
    walk(e, &mut out);
    out
}
