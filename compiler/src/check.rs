//! Name resolution, type checking and effect checking. Lowers the AST to the typed IR.

use std::collections::{HashMap, HashSet};

use crate::ast::*;
use crate::diag::{Diagnostic, Source, Span};
use crate::tir::*;

pub const EFFECTS: &[&str] = &["io", "fs", "net", "clock", "rand", "env", "ffi"];
pub const NAT: usize = 0;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Code,
    Requires,
    Ensures,
}

struct FnSig {
    params: Vec<(String, Ty)>,
    ret: Ty,
    uses: Vec<String>,
}

pub struct Checker<'a> {
    pub m: Module,
    src: &'a Source,
    pub diags: Vec<Diagnostic>,
    type_names: HashMap<String, Ty>,
    variants: HashMap<String, (Ty, usize)>,
    fn_index: HashMap<String, usize>,
    sigs: Vec<FnSig>,
    scopes: Vec<HashMap<String, (LocalId, bool)>>,
    mode: Mode,
    cur_fn: Option<usize>,
    cur_ret: Ty,
}

type CResult<T> = Result<T, Diagnostic>;

fn texpr(kind: TExprKind, ty: Ty, span: Span) -> TExpr {
    TExpr { kind, ty, span }
}

pub fn check_program(prog: &Program, src: &Source) -> (Module, Vec<Diagnostic>) {
    let (m, d, _) = check_program_with(prog, src, &[]);
    (m, d)
}

/// Pinned contract clauses to type-check against a function of the current program.
pub struct PinnedClauses {
    pub func: String,
    pub requires: Vec<Expr>,
    pub ensures: Vec<Expr>,
}

/// Like `check_program`, and also type-checks pinned clauses in the scope of their function.
/// Each entry of the result is `None` if the function is gone or a clause no longer checks.
pub fn check_program_with(prog: &Program, src: &Source, pinned: &[PinnedClauses]) -> (Module, Vec<Diagnostic>, Vec<Option<(Vec<TExpr>, Vec<TExpr>)>>) {
    let mut c = Checker {
        m: Module::default(),
        src,
        diags: vec![],
        type_names: HashMap::new(),
        variants: HashMap::new(),
        fn_index: HashMap::new(),
        sigs: vec![],
        scopes: vec![],
        mode: Mode::Code,
        cur_fn: None,
        cur_ret: Ty::Unit,
    };
    c.run(prog);
    let pinned_out = pinned.iter().map(|p| c.check_pinned(p)).collect();
    (c.m, c.diags, pinned_out)
}

impl<'a> Checker<'a> {
    fn run(&mut self, prog: &Program) {
        // Built-in `nat` is `int where it >= 0`.
        let it = self.new_local("it", Ty::Int);
        let pred = texpr(
            TExprKind::Binary(BinOp::Ge, Box::new(texpr(TExprKind::Local(it), Ty::Int, Span::default())), Box::new(texpr(TExprKind::Int(0), Ty::Int, Span::default()))),
            Ty::Bool,
            Span::default(),
        );
        self.m.aliases.push(AliasDef { name: "nat".into(), base: Ty::Int, pred: Some(pred), it, pred_src: Some("it >= 0".into()) });
        for (n, t) in [("int", Ty::Int), ("bool", Ty::Bool), ("str", Ty::Str), ("nat", Ty::Alias(NAT))] {
            self.type_names.insert(n.into(), t);
        }
        for (i, n) in ["None", "Some"].iter().enumerate() {
            self.variants.insert(n.to_string(), (Ty::Option(Box::new(Ty::Never)), i));
        }
        for (i, n) in ["Ok", "Err"].iter().enumerate() {
            self.variants.insert(n.to_string(), (Ty::Result(Box::new(Ty::Never), Box::new(Ty::Never)), i));
        }

        // Pass 1: declare type names so declarations can refer to each other.
        for item in &prog.items {
            let (name, span) = match item {
                Item::Type(t) => (&t.name, t.span),
                Item::Enum(e) => (&e.name, e.span),
                Item::Fn(_) => continue,
            };
            if self.type_names.contains_key(name) {
                self.diags.push(Diagnostic::error("E0109", span, format!("the type `{name}` is defined twice")));
                continue;
            }
            let ty = match item {
                Item::Type(TypeDecl { ty: TypeExpr::Record(..), refine: None, .. }) => {
                    self.m.records.push(RecordDef { name: name.clone(), fields: vec![] });
                    Ty::Record(self.m.records.len() - 1)
                }
                Item::Type(_) => {
                    self.m.aliases.push(AliasDef { name: name.clone(), base: Ty::Never, pred: None, it: 0, pred_src: None });
                    Ty::Alias(self.m.aliases.len() - 1)
                }
                Item::Enum(_) => {
                    self.m.enums.push(EnumDef { name: name.clone(), variants: vec![] });
                    Ty::Enum(self.m.enums.len() - 1)
                }
                Item::Fn(_) => unreachable!(),
            };
            self.type_names.insert(name.clone(), ty);
        }

        // Pass 2: fill in type bodies.
        for item in &prog.items {
            match item {
                Item::Type(t) => {
                    let Some(ty) = self.type_names.get(&t.name).cloned() else { continue };
                    match ty {
                        Ty::Record(r) => {
                            let TypeExpr::Record(fields, _) = &t.ty else { unreachable!() };
                            let mut seen = HashSet::new();
                            let mut out = vec![];
                            for (f, fte) in fields {
                                if !seen.insert(f.clone()) {
                                    self.diags.push(Diagnostic::error("E0109", fte.span(), format!("field `{f}` appears twice")));
                                }
                                match self.resolve_type(fte) {
                                    Ok(ft) => out.push((f.clone(), ft)),
                                    Err(d) => self.diags.push(d),
                                }
                            }
                            self.m.records[r].fields = out;
                        }
                        Ty::Alias(a) => {
                            if let TypeExpr::Record(_, sp) = &t.ty {
                                self.diags.push(Diagnostic::error("E0112", *sp, "a refined record type needs a name first").with_fix(format!("declare `type {}Base = {{ ... }}` and write `type {} = {}Base where ...`", t.name, t.name, t.name)));
                                continue;
                            }
                            match self.resolve_type(&t.ty) {
                                Ok(base) => self.m.aliases[a].base = base,
                                Err(d) => self.diags.push(d),
                            }
                        }
                        _ => {}
                    }
                }
                Item::Enum(e) => {
                    let Some(Ty::Enum(ei)) = self.type_names.get(&e.name).cloned() else { continue };
                    let mut vs = vec![];
                    for (vi, v) in e.variants.iter().enumerate() {
                        if self.variants.contains_key(&v.name) {
                            self.diags.push(Diagnostic::error("E0109", v.span, format!("variant name `{}` is already used", v.name)).with_note("variant names are global so code stays greppable"));
                            continue;
                        }
                        self.variants.insert(v.name.clone(), (Ty::Enum(ei), vi));
                        let mut fields = vec![];
                        for (f, fte) in &v.fields {
                            match self.resolve_type(fte) {
                                Ok(ft) => fields.push((f.clone(), ft)),
                                Err(d) => self.diags.push(d),
                            }
                        }
                        vs.push(VariantDef { name: v.name.clone(), fields });
                    }
                    self.m.enums[ei].variants = vs;
                }
                Item::Fn(_) => {}
            }
        }
        self.check_recursive_types(prog);

        // Refinement predicates.
        for item in &prog.items {
            if let Item::Type(t) = item {
                if let (Some(pred), Some(Ty::Alias(a))) = (&t.refine, self.type_names.get(&t.name).cloned()) {
                    let base = self.m.aliases[a].base.clone();
                    let it = self.new_local("it", base);
                    self.scopes = vec![HashMap::from([("it".to_string(), (it, false))])];
                    self.mode = Mode::Requires;
                    match self.check_expr(pred, Some(&Ty::Bool)) {
                        Ok(p) => {
                            self.m.aliases[a].pred = Some(p);
                            self.m.aliases[a].it = it;
                            self.m.aliases[a].pred_src = Some(self.src.slice(pred.span).to_string());
                        }
                        Err(d) => self.diags.push(d),
                    }
                    self.mode = Mode::Code;
                }
            }
        }

        // Function signatures.
        for item in &prog.items {
            if let Item::Fn(f) = item {
                if self.fn_index.contains_key(&f.name) || self.variants.contains_key(&f.name) {
                    self.diags.push(Diagnostic::error("E0109", f.sig_span, format!("`{}` is defined twice", f.name)));
                    continue;
                }
                let mut params = vec![];
                for p in &f.params {
                    match self.resolve_type(&p.ty) {
                        Ok(t) => params.push((p.name.clone(), t)),
                        Err(d) => {
                            self.diags.push(d);
                            params.push((p.name.clone(), Ty::Never));
                        }
                    }
                }
                let ret = match &f.ret {
                    None => Ty::Unit,
                    Some(t) => self.resolve_type(t).unwrap_or_else(|d| {
                        self.diags.push(d);
                        Ty::Never
                    }),
                };
                let mut uses = vec![];
                for (e, sp) in &f.uses {
                    if !EFFECTS.contains(&e.as_str()) {
                        self.diags.push(Diagnostic::error("E0106", *sp, format!("unknown effect `{e}`")).with_note(format!("effects are: {}", EFFECTS.join(", "))));
                    } else {
                        uses.push(e.clone());
                    }
                }
                if f.name == "main" && (!f.params.is_empty() || ret != Ty::Unit) {
                    self.diags.push(Diagnostic::error("E0113", f.sig_span, "`main` takes no parameters and returns nothing"));
                }
                self.fn_index.insert(f.name.clone(), self.sigs.len());
                self.sigs.push(FnSig { params, ret, uses });
            }
        }

        // Function bodies.
        for item in &prog.items {
            if let Item::Fn(f) = item {
                if let Some(&idx) = self.fn_index.get(&f.name) {
                    if idx == self.m.funcs.len() {
                        match self.check_fn(f, idx) {
                            Ok(func) => self.m.funcs.push(func),
                            Err(d) => {
                                self.diags.push(d);
                                let dummy = self.dummy_fn(f, idx);
                                self.m.funcs.push(dummy);
                            }
                        }
                    }
                }
            }
        }
    }

    fn check_pinned(&mut self, p: &PinnedClauses) -> Option<(Vec<TExpr>, Vec<TExpr>)> {
        let fi = *self.fn_index.get(&p.func)?;
        let f = self.m.funcs.get(fi)?.clone();
        self.cur_fn = Some(fi);
        let mut scope = HashMap::new();
        for &id in &f.params {
            scope.insert(self.m.locals[id as usize].name.clone(), (id, false));
        }
        self.scopes = vec![scope];
        self.mode = Mode::Requires;
        let req: Result<Vec<TExpr>, _> = p.requires.iter().map(|e| self.check_expr(e, Some(&Ty::Bool))).collect();
        self.mode = Mode::Ensures;
        self.scopes.push(HashMap::from([("result".to_string(), (f.result, false))]));
        let ens: Result<Vec<TExpr>, _> = p.ensures.iter().map(|e| self.check_expr(e, Some(&Ty::Bool))).collect();
        self.mode = Mode::Code;
        self.cur_fn = None;
        Some((req.ok()?, ens.ok()?))
    }

    fn dummy_fn(&mut self, f: &FnDecl, idx: usize) -> Func {
        let result = self.new_local("result", self.sigs[idx].ret.clone());
        Func {
            name: f.name.clone(),
            is_pub: f.is_pub,
            params: vec![],
            ret: self.sigs[idx].ret.clone(),
            requires: vec![],
            ensures: vec![],
            requires_src: vec![],
            ensures_src: vec![],
            result,
            uses: vec![],
            body: texpr(TExprKind::Unit, Ty::Unit, f.span),
            span: f.span,
            sig_span: f.sig_span,
            sig_src: String::new(),
        }
    }

    fn check_recursive_types(&mut self, prog: &Program) {
        fn contains(m: &Module, t: &Ty, target: &Ty, seen: &mut HashSet<Ty>) -> bool {
            let t = m.peel(t);
            if &t == target {
                return true;
            }
            if !seen.insert(t.clone()) {
                return false;
            }
            match &t {
                Ty::Record(r) => m.records[*r].fields.iter().any(|(_, f)| contains(m, f, target, seen)),
                Ty::Enum(e) => m.enums[*e].variants.iter().any(|v| v.fields.iter().any(|(_, f)| contains(m, f, target, seen))),
                Ty::Option(x) => contains(m, x, target, seen),
                Ty::Result(a, b) => contains(m, a, target, seen) || contains(m, b, target, seen),
                _ => false,
            }
        }
        for item in &prog.items {
            let (name, span) = match item {
                Item::Type(t) => (&t.name, t.span),
                Item::Enum(e) => (&e.name, e.span),
                _ => continue,
            };
            let Some(t) = self.type_names.get(name).cloned() else { continue };
            let t = self.m.peel(&t);
            let children: Vec<Ty> = match &t {
                Ty::Record(r) => self.m.records[*r].fields.iter().map(|f| f.1.clone()).collect(),
                Ty::Enum(e) => self.m.enums[*e].variants.iter().flat_map(|v| v.fields.iter().map(|f| f.1.clone())).collect(),
                _ => vec![],
            };
            if children.iter().any(|c| contains(&self.m, c, &t, &mut HashSet::new())) {
                self.diags.push(Diagnostic::error("E0110", span, format!("`{name}` contains itself")).with_note("recursive types need heap values, which arrive in v0.2"));
            }
        }
    }

    fn resolve_type(&mut self, te: &TypeExpr) -> CResult<Ty> {
        match te {
            TypeExpr::Opt(inner, _) => Ok(Ty::Option(Box::new(self.resolve_type(inner)?))),
            TypeExpr::Record(_, sp) => Err(Diagnostic::error("E0112", *sp, "record types must be named").with_fix("declare it with `type Name = { ... }` and use `Name` here")),
            TypeExpr::Name(n, args, sp) => {
                let want = |k: usize, this: &Self| -> CResult<()> {
                    if args.len() != k {
                        Err(Diagnostic::error("E0102", *sp, format!("`{n}` takes {k} type argument(s), found {}", args.len())))
                    } else {
                        let _ = this;
                        Ok(())
                    }
                };
                match n.as_str() {
                    "Option" => {
                        want(1, self)?;
                        Ok(Ty::Option(Box::new(self.resolve_type(&args[0])?)))
                    }
                    "Result" => {
                        want(2, self)?;
                        Ok(Ty::Result(Box::new(self.resolve_type(&args[0])?), Box::new(self.resolve_type(&args[1])?)))
                    }
                    _ => {
                        want(0, self)?;
                        self.type_names.get(n).cloned().ok_or_else(|| {
                            let mut d = Diagnostic::error("E0101", *sp, format!("unknown type `{n}`"));
                            if let Some(s) = suggest(n, self.type_names.keys()) {
                                d = d.with_fix(format!("did you mean `{s}`?"));
                            }
                            d
                        })
                    }
                }
            }
        }
    }

    fn new_local(&mut self, name: &str, ty: Ty) -> LocalId {
        self.m.locals.push(Local { name: name.to_string(), ty });
        (self.m.locals.len() - 1) as LocalId
    }

    fn bind(&mut self, name: &str, ty: Ty, mutable: bool) -> LocalId {
        let id = self.new_local(name, ty);
        self.scopes.last_mut().unwrap().insert(name.to_string(), (id, mutable));
        id
    }

    fn lookup(&self, name: &str) -> Option<(LocalId, bool)> {
        self.scopes.iter().rev().find_map(|s| s.get(name).copied())
    }

    fn check_fn(&mut self, f: &FnDecl, idx: usize) -> CResult<Func> {
        self.cur_fn = Some(idx);
        self.scopes = vec![HashMap::new()];
        let sig_params: Vec<(String, Ty)> = self.sigs[idx].params.clone();
        let ret = self.sigs[idx].ret.clone();
        self.cur_ret = ret.clone();
        let mut params = vec![];
        let mut seen = HashSet::new();
        for (p, (name, ty)) in f.params.iter().zip(sig_params) {
            if !seen.insert(name.clone()) {
                return Err(Diagnostic::error("E0109", p.span, format!("parameter `{name}` appears twice")));
            }
            params.push(self.bind(&name, ty, false));
        }
        self.mode = Mode::Requires;
        let mut requires = vec![];
        for r in &f.requires {
            requires.push(self.check_expr(r, Some(&Ty::Bool))?);
        }
        self.mode = Mode::Ensures;
        let result = self.new_local("result", ret.clone());
        self.scopes.push(HashMap::from([("result".to_string(), (result, false))]));
        let mut ensures = vec![];
        for e in &f.ensures {
            ensures.push(self.check_expr(e, Some(&Ty::Bool))?);
        }
        self.scopes.pop();
        if ret == Ty::Unit && !f.ensures.is_empty() {
            // Allowed: it can still talk about the parameters.
        }
        self.mode = Mode::Code;
        let body = self.check_block(&f.body, Some(&ret))?;
        if ret != Ty::Unit && body.ty != Ty::Never && !self.block_ends_in_return(&body) && !self.subtype(&body.ty, &ret) {
            return Err(Diagnostic::error("E0102", f.body.span, format!("this function must return `{}`", self.m.show(&ret))));
        }
        self.cur_fn = None;
        Ok(Func {
            name: f.name.clone(),
            is_pub: f.is_pub,
            params,
            ret,
            requires,
            ensures,
            requires_src: f.requires.iter().map(|e| self.src.slice(e.span).to_string()).collect(),
            ensures_src: f.ensures.iter().map(|e| self.src.slice(e.span).to_string()).collect(),
            result,
            uses: self.sigs[idx].uses.clone(),
            body,
            span: f.span,
            sig_span: f.sig_span,
            sig_src: self.src.slice(f.sig_span).split_whitespace().collect::<Vec<_>>().join(" "),
        })
    }

    fn block_ends_in_return(&self, b: &TExpr) -> bool {
        matches!(&b.kind, TExprKind::Block(stmts, tail) if matches!(tail.kind, TExprKind::Unit) && matches!(stmts.last(), Some(TStmt::Return(..))))
    }

    // ---- types ----

    pub fn subtype(&self, a: &Ty, b: &Ty) -> bool {
        if a == b || *a == Ty::Never {
            return true;
        }
        match (a, b) {
            (Ty::Alias(x), _) => self.subtype(&self.m.aliases[*x].base, b),
            (Ty::Option(x), Ty::Option(y)) => self.subtype(x, y),
            (Ty::Result(a1, b1), Ty::Result(a2, b2)) => self.subtype(a1, a2) && self.subtype(b1, b2),
            _ => false,
        }
    }

    fn has_pred(&self, a: usize) -> bool {
        self.m.aliases[a].pred.is_some() || matches!(self.m.aliases[a].base, Ty::Alias(b) if self.has_pred(b))
    }

    /// Accept `e` where `expected` is wanted, inserting a refinement check if needed.
    fn coerce(&self, e: TExpr, expected: &Ty) -> CResult<TExpr> {
        if self.subtype(&e.ty, expected) {
            return Ok(e);
        }
        if let Ty::Alias(a) = expected {
            if !self.has_pred(*a) && self.subtype(&e.ty, &self.m.erase(expected)) {
                return Ok(e);
            }
            let base = self.m.aliases[*a].base.clone();
            if self.subtype(&e.ty, &base) || matches!(base, Ty::Alias(_)) && self.coerce(e.clone(), &base).is_ok() {
                if self.mode != Mode::Code {
                    return Ok(e);
                }
                let inner = if self.subtype(&e.ty, &base) { e } else { self.coerce(e, &base)? };
                let span = inner.span;
                return Ok(texpr(TExprKind::Coerce(Box::new(inner), *a), expected.clone(), span));
            }
        }
        let mut d = Diagnostic::error("E0102", e.span, format!("expected `{}`, found `{}`", self.m.show(expected), self.m.show(&e.ty)));
        if self.m.erase(&e.ty) == self.m.erase(expected) {
            d = d.with_note("the types differ only in a refinement nested inside; v0.1 checks refinements only at the top level").with_fix("rebuild the value so each refined part is checked, e.g. `Some(x)` instead of passing the whole option");
        }
        Err(d)
    }

    fn check_block(&mut self, b: &Block, expected: Option<&Ty>) -> CResult<TExpr> {
        self.scopes.push(HashMap::new());
        let r = self.check_block_inner(b, expected);
        self.scopes.pop();
        r
    }

    fn check_block_inner(&mut self, b: &Block, expected: Option<&Ty>) -> CResult<TExpr> {
        let mut stmts = vec![];
        let n = b.stmts.len();
        let mut never = false;
        for (i, s) in b.stmts.iter().enumerate() {
            let last = i + 1 == n;
            if last {
                if let Stmt::Expr(e) = s {
                    if expected == Some(&Ty::Unit) {
                        let te = self.check_expr(e, None)?;
                        let ty = if te.ty == Ty::Never || never { Ty::Never } else { Ty::Unit };
                        stmts.push(TStmt::Expr(te));
                        return Ok(texpr(TExprKind::Block(stmts, Box::new(texpr(TExprKind::Unit, Ty::Unit, b.span))), ty, b.span));
                    }
                    let te = self.check_expr(e, expected)?;
                    let ty = if never { Ty::Never } else { te.ty.clone() };
                    return Ok(texpr(TExprKind::Block(stmts, Box::new(te)), ty, b.span));
                }
            }
            let ts = self.check_stmt(s)?;
            if matches!(ts, TStmt::Return(..)) || matches!(&ts, TStmt::Expr(e) if e.ty == Ty::Never) {
                never = true;
            }
            stmts.push(ts);
        }
        let ty = if never { Ty::Never } else { Ty::Unit };
        Ok(texpr(TExprKind::Block(stmts, Box::new(texpr(TExprKind::Unit, Ty::Unit, b.span))), ty, b.span))
    }

    fn check_stmt(&mut self, s: &Stmt) -> CResult<TStmt> {
        match s {
            Stmt::Let { name, mutable, ty, init, span } => {
                let ann = match ty {
                    Some(t) => Some(self.resolve_type(t)?),
                    None => None,
                };
                let e = self.check_expr(init, ann.as_ref())?;
                let lty = ann.unwrap_or_else(|| e.ty.clone());
                if lty == Ty::Never {
                    return Err(Diagnostic::error("E0105", *span, format!("`{name}` never gets a value")));
                }
                let id = self.bind(name, lty, *mutable);
                Ok(TStmt::Let(id, e))
            }
            Stmt::Assign { name, op, value, span } => {
                let Some((id, mutable)) = self.lookup(name) else {
                    return Err(Diagnostic::error("E0101", *span, format!("unknown variable `{name}`")).with_fix(format!("declare it first with `var {name} = ...`")));
                };
                if !mutable {
                    return Err(Diagnostic::error("E0111", *span, format!("`{name}` cannot be changed")).with_fix(format!("declare it with `var {name} = ...` instead of `let`")));
                }
                let lty = self.m.locals[id as usize].ty.clone();
                let rhs = match op {
                    None => value.clone(),
                    Some(op) => Expr {
                        kind: ExprKind::Binary(*op, Box::new(Expr { kind: ExprKind::Name(name.clone()), span: *span }), Box::new(value.clone())),
                        span: *span,
                    },
                };
                let e = self.check_expr(&rhs, Some(&lty))?;
                Ok(TStmt::Assign(id, e))
            }
            Stmt::Expr(e) => {
                let te = self.check_expr(e, None)?;
                if matches!(self.m.peel(&te.ty), Ty::Result(..)) {
                    return Err(Diagnostic::error("E0114", e.span, "this `Result` is ignored").with_fix("handle it with `match`, or bind it with `let _ = ...` if ignoring it is intended"));
                }
                Ok(TStmt::Expr(te))
            }
            Stmt::Return(e, span) => {
                let ret = self.cur_ret.clone();
                let te = match e {
                    Some(e) => self.check_expr(e, Some(&ret))?,
                    None if ret == Ty::Unit => texpr(TExprKind::Unit, Ty::Unit, *span),
                    None => return Err(Diagnostic::error("E0102", *span, format!("this function must return a `{}`", self.m.show(&ret)))),
                };
                Ok(TStmt::Return(te, *span))
            }
            Stmt::While { cond, invariants, decreases, body, span } => {
                let c = self.check_expr(cond, Some(&Ty::Bool))?;
                let saved = self.mode;
                self.mode = Mode::Requires;
                let mut invs = vec![];
                for i in invariants {
                    invs.push(self.check_expr(i, Some(&Ty::Bool))?);
                }
                let dec = match decreases {
                    Some(d) => Some(self.check_expr(d, Some(&Ty::Int))?),
                    None => None,
                };
                self.mode = saved;
                let b = self.check_block(body, Some(&Ty::Unit))?;
                let mut modified = vec![];
                collect_assigned(&b, &mut modified);
                modified.sort();
                modified.dedup();
                Ok(TStmt::While { cond: c, invariants: invs, decreases: dec, body: b, modified, span: *span })
            }
        }
    }

    fn check_expr(&mut self, e: &Expr, expected: Option<&Ty>) -> CResult<TExpr> {
        let te = self.infer(e, expected)?;
        match expected {
            Some(t) if *t != Ty::Unit || te.ty != Ty::Unit => self.coerce(te, t),
            _ => Ok(te),
        }
    }

    fn expect_int(&self, e: &TExpr) -> CResult<()> {
        if self.m.peel(&e.ty) == Ty::Int || e.ty == Ty::Never {
            Ok(())
        } else {
            Err(Diagnostic::error("E0102", e.span, format!("expected a number, found `{}`", self.m.show(&e.ty))))
        }
    }

    fn infer(&mut self, e: &Expr, expected: Option<&Ty>) -> CResult<TExpr> {
        let sp = e.span;
        let hint = expected.map(|t| self.m.peel(t));
        Ok(match &e.kind {
            ExprKind::Int(v) => {
                if *v < i64::MIN as i128 || *v > i64::MAX as i128 {
                    return Err(Diagnostic::error("E0115", sp, "this number does not fit in `int` (64-bit)"));
                }
                texpr(TExprKind::Int(*v), Ty::Int, sp)
            }
            ExprKind::Bool(b) => texpr(TExprKind::Bool(*b), Ty::Bool, sp),
            ExprKind::Str(s) => texpr(TExprKind::Str(s.clone()), Ty::Str, sp),
            ExprKind::Name(n) => {
                if let Some((id, _)) = self.lookup(n) {
                    return Ok(texpr(TExprKind::Local(id), self.m.locals[id as usize].ty.clone(), sp));
                }
                if self.variants.contains_key(n) {
                    return self.ctor(n, &[], hint.as_ref(), sp);
                }
                if n == "result" && self.mode != Mode::Ensures {
                    return Err(Diagnostic::error("E0108", sp, "`result` can only be used in `ensures`"));
                }
                let mut d = Diagnostic::error("E0101", sp, format!("unknown name `{n}`"));
                let names: Vec<String> = self.scopes.iter().flat_map(|s| s.keys().cloned()).collect();
                if let Some(s) = suggest(n, names.iter()) {
                    d = d.with_fix(format!("did you mean `{s}`?"));
                }
                return Err(d);
            }
            ExprKind::Field(base, f) => {
                if let ExprKind::Name(n) = &base.kind {
                    if n == "int" && self.lookup(n).is_none() {
                        let v = match f.as_str() {
                            "min" => i64::MIN as i128,
                            "max" => i64::MAX as i128,
                            _ => return Err(Diagnostic::error("E0103", sp, format!("`int` has no constant `{f}`")).with_fix("use `int.min` or `int.max`")),
                        };
                        return Ok(texpr(TExprKind::Int(v), Ty::Int, sp));
                    }
                }
                let b = self.infer(base, None)?;
                let Ty::Record(r) = self.m.peel(&b.ty) else {
                    return Err(Diagnostic::error("E0103", sp, format!("`{}` has no fields", self.m.show(&b.ty))));
                };
                let Some(i) = self.m.records[r].fields.iter().position(|(n, _)| n == f) else {
                    let names: Vec<&String> = self.m.records[r].fields.iter().map(|(n, _)| n).collect();
                    let mut d = Diagnostic::error("E0103", sp, format!("`{}` has no field `{f}`", self.m.records[r].name));
                    if let Some(s) = suggest(f, names.into_iter()) {
                        d = d.with_fix(format!("did you mean `{s}`?"));
                    }
                    return Err(d);
                };
                let ft = self.m.records[r].fields[i].1.clone();
                texpr(TExprKind::Field(Box::new(b), i), ft, sp)
            }
            ExprKind::Call(callee, args) => return self.call(callee, args, hint.as_ref(), sp),
            ExprKind::Record { spread, fields } => {
                let target = match (&hint, spread) {
                    (Some(Ty::Record(r)), _) => Some(*r),
                    (_, Some(base)) => {
                        let b = self.infer(base, None)?;
                        match self.m.peel(&b.ty) {
                            Ty::Record(r) => Some(r),
                            _ => return Err(Diagnostic::error("E0102", base.span, "`...` needs a record")),
                        }
                    }
                    _ => {
                        let names: HashSet<&str> = fields.iter().map(|(n, _)| n.as_str()).collect();
                        let cands: Vec<usize> = (0..self.m.records.len())
                            .filter(|&r| self.m.records[r].fields.len() == names.len() && self.m.records[r].fields.iter().all(|(n, _)| names.contains(n.as_str())))
                            .collect();
                        match cands.as_slice() {
                            [r] => Some(*r),
                            [] => return Err(Diagnostic::error("E0105", sp, "no record type has exactly these fields").with_fix("declare one with `type Name = { ... }`")),
                            _ => return Err(Diagnostic::error("E0105", sp, "several record types have these fields").with_fix("annotate the type, e.g. `let x: Name = { ... }`")),
                        }
                    }
                };
                let r = target.unwrap();
                let rdef = self.m.records[r].clone();
                let mut seen = HashSet::new();
                let mut given = vec![];
                for (f, fe) in fields {
                    let Some(i) = rdef.fields.iter().position(|(n, _)| n == f) else {
                        return Err(Diagnostic::error("E0103", fe.span, format!("`{}` has no field `{f}`", rdef.name)));
                    };
                    if !seen.insert(i) {
                        return Err(Diagnostic::error("E0109", fe.span, format!("field `{f}` is given twice")));
                    }
                    let ft = rdef.fields[i].1.clone();
                    given.push((i, self.check_expr(fe, Some(&ft))?));
                }
                if let Some(base) = spread {
                    let b = self.check_expr(base, Some(&Ty::Record(r)))?;
                    texpr(TExprKind::Update(Box::new(b), given), Ty::Record(r), sp)
                } else {
                    let missing: Vec<&str> = rdef.fields.iter().enumerate().filter(|(i, _)| !seen.contains(i)).map(|(_, (n, _))| n.as_str()).collect();
                    if !missing.is_empty() {
                        return Err(Diagnostic::error("E0103", sp, format!("missing field(s) {} for `{}`", missing.iter().map(|m| format!("`{m}`")).collect::<Vec<_>>().join(", "), rdef.name)));
                    }
                    given.sort_by_key(|(i, _)| *i);
                    texpr(TExprKind::Record(given.into_iter().map(|(_, e)| e).collect()), Ty::Record(r), sp)
                }
            }
            ExprKind::Unary(op, x) => {
                let t = match op {
                    UnOp::Neg => {
                        let t = self.check_expr(x, None)?;
                        self.expect_int(&t)?;
                        t
                    }
                    UnOp::Not => self.check_expr(x, Some(&Ty::Bool))?,
                };
                let ty = if *op == UnOp::Neg { Ty::Int } else { Ty::Bool };
                texpr(TExprKind::Unary(*op, Box::new(t)), ty, sp)
            }
            ExprKind::Binary(op, l, r) => {
                use BinOp::*;
                match op {
                    And | Or | Implies => {
                        let lt = self.check_expr(l, Some(&Ty::Bool))?;
                        let rt = if *op == Or {
                            self.check_expr(r, Some(&Ty::Bool))?
                        } else {
                            self.with_bindings(&lt, |c| c.check_expr(r, Some(&Ty::Bool)))?
                        };
                        texpr(TExprKind::Binary(*op, Box::new(lt), Box::new(rt)), Ty::Bool, sp)
                    }
                    Add | Sub | Mul | Div | Rem | Lt | Le | Gt | Ge => {
                        let lt = self.check_expr(l, None)?;
                        self.expect_int(&lt)?;
                        let rt = self.check_expr(r, None)?;
                        self.expect_int(&rt)?;
                        let ty = if matches!(op, Lt | Le | Gt | Ge) { Ty::Bool } else { Ty::Int };
                        texpr(TExprKind::Binary(*op, Box::new(lt), Box::new(rt)), ty, sp)
                    }
                    Eq | Ne => {
                        let lt = self.check_expr(l, None)?;
                        let rt = self.check_expr(r, None)?;
                        if self.m.erase(&lt.ty) != self.m.erase(&rt.ty) && lt.ty != Ty::Never && rt.ty != Ty::Never {
                            return Err(Diagnostic::error("E0102", sp, format!("cannot compare `{}` with `{}`", self.m.show(&lt.ty), self.m.show(&rt.ty))));
                        }
                        if self.mode == Mode::Code && !matches!(self.m.peel(&lt.ty), Ty::Int | Ty::Bool) {
                            return Err(Diagnostic::error("E0116", sp, "v0.1 compares only numbers and booleans in code").with_fix("compare fields, or use `match`/`is`; whole values can be compared in contracts"));
                        }
                        texpr(TExprKind::Binary(*op, Box::new(lt), Box::new(rt)), Ty::Bool, sp)
                    }
                }
            }
            ExprKind::Is(x, pat) => {
                let xt = self.check_expr(x, None)?;
                let p = self.pattern(pat, &xt.ty)?;
                texpr(TExprKind::Is(Box::new(xt), p), Ty::Bool, sp)
            }
            ExprKind::If(cond, then, els) => {
                let c = self.check_expr(cond, Some(&Ty::Bool))?;
                let want = if els.is_none() { Some(Ty::Unit) } else { expected.cloned() };
                let t = self.with_bindings(&c, |ch| ch.check_block(then, want.as_ref()))?;
                let el = match els {
                    Some(e) => {
                        let want2 = want.clone().or_else(|| if t.ty == Ty::Never { None } else { Some(t.ty.clone()) });
                        self.check_expr(e, want2.as_ref())?
                    }
                    None => texpr(TExprKind::Unit, Ty::Unit, sp),
                };
                let ty = if els.is_none() {
                    if t.ty == Ty::Never { Ty::Unit } else { t.ty.clone() }
                } else if t.ty == Ty::Never {
                    el.ty.clone()
                } else if el.ty == Ty::Never || want.is_some() {
                    want.clone().unwrap_or(t.ty.clone())
                } else {
                    t.ty.clone()
                };
                let ty = if t.ty == Ty::Never && el.ty == Ty::Never { Ty::Never } else { ty };
                texpr(TExprKind::If(Box::new(c), Box::new(t), Box::new(el)), ty, sp)
            }
            ExprKind::Match(scrut, arms) => {
                let s = self.check_expr(scrut, None)?;
                let mut tarms = vec![];
                let mut ty: Option<Ty> = expected.cloned();
                for arm in arms {
                    self.scopes.push(HashMap::new());
                    let p = self.pattern(&arm.pat, &s.ty);
                    let p = match p {
                        Ok(p) => p,
                        Err(d) => {
                            self.scopes.pop();
                            return Err(d);
                        }
                    };
                    let mut binds = vec![];
                    p.bindings(&mut binds);
                    for id in binds {
                        let name = self.m.locals[id as usize].name.clone();
                        self.scopes.last_mut().unwrap().insert(name, (id, false));
                    }
                    let body = self.check_expr(&arm.body, ty.as_ref());
                    self.scopes.pop();
                    let body = body?;
                    if ty.is_none() && body.ty != Ty::Never {
                        ty = Some(body.ty.clone());
                    }
                    tarms.push((p, body));
                }
                self.exhaustive(&s.ty, &tarms.iter().map(|(p, _)| p.clone()).collect::<Vec<_>>(), sp)?;
                let ty = ty.unwrap_or(Ty::Never);
                texpr(TExprKind::Match(Box::new(s), tarms), ty, sp)
            }
            ExprKind::Block(b) => self.check_block(b, expected)?,
        })
    }

    /// Check `f` with the bindings introduced by `is` patterns in `cond` in scope.
    fn with_bindings<T>(&mut self, cond: &TExpr, f: impl FnOnce(&mut Self) -> CResult<T>) -> CResult<T> {
        let mut ids = vec![];
        positive_bindings(cond, &mut ids);
        let mut scope = HashMap::new();
        for id in ids {
            scope.insert(self.m.locals[id as usize].name.clone(), (id, false));
        }
        self.scopes.push(scope);
        let r = f(self);
        self.scopes.pop();
        r
    }

    fn pattern(&mut self, p: &Pattern, ty: &Ty) -> CResult<TPat> {
        let peeled = self.m.peel(ty);
        Ok(match &p.kind {
            PatKind::Wild => TPat::Wild,
            PatKind::Bind(n) => {
                if self.variants.contains_key(n) {
                    return self.pattern(&Pattern { kind: PatKind::Ctor(n.clone(), vec![]), span: p.span }, ty);
                }
                TPat::Bind(self.new_local(n, ty.clone()))
            }
            PatKind::Int(v) => {
                if peeled != Ty::Int {
                    return Err(Diagnostic::error("E0102", p.span, format!("a number pattern cannot match `{}`", self.m.show(ty))));
                }
                TPat::Int(*v)
            }
            PatKind::Bool(b) => {
                if peeled != Ty::Bool {
                    return Err(Diagnostic::error("E0102", p.span, format!("a boolean pattern cannot match `{}`", self.m.show(ty))));
                }
                TPat::Bool(*b)
            }
            PatKind::Ctor(n, subs) => {
                let vs = self.m.variants(ty);
                let Some(vi) = vs.iter().position(|(vn, _)| vn == n) else {
                    let names: Vec<&String> = vs.iter().map(|(n, _)| n).collect();
                    let mut d = Diagnostic::error("E0102", p.span, format!("`{n}` is not a variant of `{}`", self.m.show(ty)));
                    if !names.is_empty() {
                        d = d.with_note(format!("its variants are {}", names.iter().map(|n| format!("`{n}`")).collect::<Vec<_>>().join(", ")));
                    }
                    return Err(d);
                };
                let fields = vs[vi].1.clone();
                if subs.len() != fields.len() {
                    return Err(Diagnostic::error("E0107", p.span, format!("`{n}` has {} field(s), the pattern gives {}", fields.len(), subs.len())));
                }
                let mut tsubs = vec![];
                for (s, (_, ft)) in subs.iter().zip(fields) {
                    tsubs.push(self.pattern(s, &ft)?);
                }
                TPat::Ctor(vi, tsubs)
            }
        })
    }

    fn exhaustive(&self, ty: &Ty, pats: &[TPat], sp: Span) -> CResult<()> {
        let irrefutable = |p: &TPat| match p {
            TPat::Wild | TPat::Bind(_) => true,
            TPat::Ctor(_, subs) => subs.iter().all(|s| matches!(s, TPat::Wild | TPat::Bind(_))),
            _ => false,
        };
        if pats.iter().any(|p| matches!(p, TPat::Wild | TPat::Bind(_))) {
            return Ok(());
        }
        let vs = self.m.variants(ty);
        let missing: Vec<String> = if !vs.is_empty() {
            (0..vs.len()).filter(|vi| !pats.iter().any(|p| matches!(p, TPat::Ctor(i, _) if i == vi) && irrefutable(p))).map(|vi| {
                let (n, f) = &vs[vi];
                if f.is_empty() { n.clone() } else { format!("{n}({})", vec!["_"; f.len()].join(", ")) }
            }).collect()
        } else if self.m.peel(ty) == Ty::Bool {
            [true, false].iter().filter(|b| !pats.iter().any(|p| matches!(p, TPat::Bool(x) if x == *b))).map(|b| b.to_string()).collect()
        } else {
            vec!["_".into()]
        };
        if missing.is_empty() {
            return Ok(());
        }
        Err(Diagnostic::error("E0104", sp, format!("this `match` does not cover {}", missing.iter().map(|m| format!("`{m}`")).collect::<Vec<_>>().join(", "))).with_fix(format!("add an arm for each, e.g. `{} => ...`, or a catch-all `_ => ...`", missing[0])))
    }

    fn ctor(&mut self, name: &str, args: &[Expr], hint: Option<&Ty>, sp: Span) -> CResult<TExpr> {
        let (kind_ty, vi) = self.variants[name].clone();
        let ty = match (&kind_ty, hint) {
            (Ty::Enum(e), _) => Ty::Enum(*e),
            (Ty::Option(_), Some(t @ Ty::Option(_))) => t.clone(),
            (Ty::Result(..), Some(t @ Ty::Result(..))) => t.clone(),
            (Ty::Option(_), _) if name == "Some" && args.len() == 1 => {
                let a = self.check_expr(&args[0], None)?;
                return Ok(texpr(TExprKind::Ctor(1, vec![a.clone()]), Ty::Option(Box::new(a.ty)), sp));
            }
            _ => {
                return Err(Diagnostic::error("E0105", sp, format!("cannot tell which type `{name}` builds here")).with_fix("add a type annotation, e.g. `let r: Result<int, Error> = ...`, or return it from a function with a declared type"));
            }
        };
        let fields = self.m.variants(&ty)[vi].1.clone();
        if fields.len() != args.len() {
            let d = Diagnostic::error("E0107", sp, format!("`{name}` takes {} value(s), found {}", fields.len(), args.len()));
            return Err(if fields.is_empty() { d.with_fix(format!("write `{name}` without parentheses")) } else { d });
        }
        let mut targs = vec![];
        for (a, (_, ft)) in args.iter().zip(fields) {
            targs.push(self.check_expr(a, Some(&ft))?);
        }
        Ok(texpr(TExprKind::Ctor(vi, targs), ty, sp))
    }

    fn call(&mut self, callee: &Expr, args: &[Expr], hint: Option<&Ty>, sp: Span) -> CResult<TExpr> {
        if let ExprKind::Field(m, f) = &callee.kind {
            if let ExprKind::Name(mn) = &m.kind {
                if self.lookup(mn).is_none() && EFFECTS.contains(&mn.as_str()) {
                    if mn != "io" || f != "print" {
                        return Err(Diagnostic::error("E0101", callee.span, format!("`{mn}.{f}` does not exist in v0.1")).with_note("v0.1 provides `io.print`; files, network and clocks arrive with the standard library"));
                    }
                    self.need_effect("io", callee.span)?;
                    if self.mode != Mode::Code {
                        return Err(Diagnostic::error("E0108", sp, "contracts cannot perform effects"));
                    }
                    let mut targs = vec![];
                    for a in args {
                        let t = self.check_expr(a, None)?;
                        if !matches!(self.m.peel(&t.ty), Ty::Int | Ty::Bool | Ty::Str) {
                            return Err(Diagnostic::error("E0102", a.span, format!("`io.print` cannot print `{}` yet", self.m.show(&t.ty))).with_fix("print its fields one by one"));
                        }
                        targs.push(t);
                    }
                    return Ok(texpr(TExprKind::Print(targs), Ty::Unit, sp));
                }
            }
        }
        let ExprKind::Name(name) = &callee.kind else {
            return Err(Diagnostic::error("E0102", callee.span, "only named functions can be called"));
        };
        if self.variants.contains_key(name) && self.lookup(name).is_none() {
            return self.ctor(name, args, hint, sp);
        }
        let Some(&fi) = self.fn_index.get(name) else {
            let mut d = Diagnostic::error("E0101", callee.span, format!("unknown function `{name}`"));
            if let Some(s) = suggest(name, self.fn_index.keys()) {
                d = d.with_fix(format!("did you mean `{s}`?"));
            }
            return Err(d);
        };
        if self.mode != Mode::Code {
            return Err(Diagnostic::error("E0108", sp, "contracts cannot call functions in v0.1").with_fix("state the property directly in terms of the parameters"));
        }
        let (params, ret, uses) = (self.sigs[fi].params.clone(), self.sigs[fi].ret.clone(), self.sigs[fi].uses.clone());
        if params.len() != args.len() {
            return Err(Diagnostic::error("E0107", sp, format!("`{name}` takes {} argument(s), found {}", params.len(), args.len())));
        }
        for u in &uses {
            self.need_effect(u, callee.span).map_err(|d| d.with_note(format!("`{name}` uses `{u}`")))?;
        }
        let mut targs = vec![];
        for (a, (_, pt)) in args.iter().zip(params) {
            targs.push(self.check_expr(a, Some(&pt))?);
        }
        Ok(texpr(TExprKind::Call(fi, targs), ret, sp))
    }

    fn need_effect(&self, eff: &str, sp: Span) -> CResult<()> {
        let fi = self.cur_fn.unwrap_or(usize::MAX);
        if fi == usize::MAX || self.sigs[fi].uses.iter().any(|u| u == eff) {
            return Ok(());
        }
        let fname = self.fn_index.iter().find(|(_, &i)| i == fi).map(|(n, _)| n.clone()).unwrap_or_default();
        Err(Diagnostic::error("E0106", sp, format!("`{fname}` does not declare the `{eff}` effect")).with_fix(format!("add `uses {eff}` to the signature of `{fname}`")))
    }
}

fn positive_bindings(e: &TExpr, out: &mut Vec<LocalId>) {
    match &e.kind {
        TExprKind::Is(_, p) => p.bindings(out),
        TExprKind::Binary(BinOp::And, l, r) => {
            positive_bindings(l, out);
            positive_bindings(r, out);
        }
        _ => {}
    }
}

fn collect_assigned(e: &TExpr, out: &mut Vec<LocalId>) {
    fn stmt(s: &TStmt, out: &mut Vec<LocalId>) {
        match s {
            TStmt::Assign(id, v) => {
                out.push(*id);
                collect_assigned(v, out);
            }
            TStmt::Let(_, v) | TStmt::Expr(v) | TStmt::Return(v, _) => collect_assigned(v, out),
            TStmt::While { cond, body, .. } => {
                collect_assigned(cond, out);
                collect_assigned(body, out);
            }
        }
    }
    match &e.kind {
        TExprKind::Block(ss, t) => {
            ss.iter().for_each(|s| stmt(s, out));
            collect_assigned(t, out);
        }
        TExprKind::If(c, t, f) => {
            collect_assigned(c, out);
            collect_assigned(t, out);
            collect_assigned(f, out);
        }
        TExprKind::Match(s, arms) => {
            collect_assigned(s, out);
            arms.iter().for_each(|(_, b)| collect_assigned(b, out));
        }
        TExprKind::Field(x, _) | TExprKind::Unary(_, x) | TExprKind::Is(x, _) | TExprKind::Coerce(x, _) => collect_assigned(x, out),
        TExprKind::Binary(_, a, b) => {
            collect_assigned(a, out);
            collect_assigned(b, out);
        }
        TExprKind::Record(xs) | TExprKind::Ctor(_, xs) | TExprKind::Call(_, xs) | TExprKind::Print(xs) => xs.iter().for_each(|x| collect_assigned(x, out)),
        TExprKind::Update(b, fs) => {
            collect_assigned(b, out);
            fs.iter().for_each(|(_, x)| collect_assigned(x, out));
        }
        _ => {}
    }
}

/// Closest name by edit distance, for "did you mean" fixes.
fn suggest<'b>(name: &str, cands: impl Iterator<Item = &'b String>) -> Option<String> {
    fn dist(a: &str, b: &str) -> usize {
        let b: Vec<char> = b.chars().collect();
        let mut prev: Vec<usize> = (0..=b.len()).collect();
        for (i, ca) in a.chars().enumerate() {
            let mut cur = vec![i + 1];
            for (j, cb) in b.iter().enumerate() {
                cur.push((prev[j] + (ca != *cb) as usize).min(prev[j + 1] + 1).min(cur[j] + 1));
            }
            prev = cur;
        }
        prev[b.len()]
    }
    cands.map(|c| (dist(name, c), c)).filter(|(d, c)| *d <= 2.max(name.len() / 3) && *d > 0 && c.as_str() != "it").min().map(|(_, c)| c.clone())
}
