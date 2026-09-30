//! Typed IR produced by the checker and shared by the verifier and the C backend.

use crate::ast::{BinOp, Quantifier, UnOp};
use crate::diag::Span;

pub type LocalId = u32;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Unit,
    Never,
    Int,
    I128,
    Bool,
    Str,
    Record(usize),
    Enum(usize),
    Alias(usize),
    Option(Box<Ty>),
    Result(Box<Ty>, Box<Ty>),
    Array(Box<Ty>),
}

#[derive(Clone, Debug)]
pub struct RecordDef {
    pub name: String,
    pub fields: Vec<(String, Ty)>,
}

#[derive(Clone, Debug)]
pub struct VariantDef {
    pub name: String,
    pub fields: Vec<(String, Ty)>,
}

#[derive(Clone, Debug)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<VariantDef>,
}

/// `type Name = base where pred`. The predicate talks about the value through `it`.
#[derive(Clone, Debug)]
pub struct AliasDef {
    pub name: String,
    pub base: Ty,
    pub pred: Option<TExpr>,
    pub it: LocalId,
    pub pred_src: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Local {
    pub name: String,
    pub ty: Ty,
}

#[derive(Clone, Debug)]
pub struct Func {
    pub name: String,
    pub is_pub: bool,
    pub params: Vec<LocalId>,
    pub ret: Ty,
    pub requires: Vec<TExpr>,
    pub ensures: Vec<TExpr>,
    pub requires_src: Vec<String>,
    pub ensures_src: Vec<String>,
    pub result: LocalId,
    pub uses: Vec<String>,
    pub body: TExpr,
    pub span: Span,
    pub sig_span: Span,
    pub sig_src: String,
}

#[derive(Clone, Debug, Default)]
pub struct Module {
    pub records: Vec<RecordDef>,
    pub enums: Vec<EnumDef>,
    pub aliases: Vec<AliasDef>,
    pub funcs: Vec<Func>,
    pub locals: Vec<Local>,
}

#[derive(Clone, Debug)]
pub struct TExpr {
    pub kind: TExprKind,
    pub ty: Ty,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum TExprKind {
    Int(i128),
    Bool(bool),
    Str(String),
    Unit,
    Local(LocalId),
    Field(Box<TExpr>, usize),
    Record(Vec<TExpr>),
    Update(Box<TExpr>, Vec<(usize, TExpr)>),
    /// Variant constructor. For `Option`: 0 = None, 1 = Some. For `Result`: 0 = Ok, 1 = Err.
    Ctor(usize, Vec<TExpr>),
    Call(usize, Vec<TExpr>),
    Print(Vec<TExpr>),
    Unary(UnOp, Box<TExpr>),
    Binary(BinOp, Box<TExpr>, Box<TExpr>),
    Is(Box<TExpr>, TPat),
    If(Box<TExpr>, Box<TExpr>, Box<TExpr>),
    Match(Box<TExpr>, Vec<(TPat, TExpr)>),
    Block(Vec<TStmt>, Box<TExpr>),
    /// The value must satisfy the refinement of this alias.
    Coerce(Box<TExpr>, usize),
    Index(Box<TExpr>, Box<TExpr>),
    Len(Box<TExpr>),
    ArrayLit(Vec<TExpr>),
    ArrayRepeat(Box<TExpr>, Box<TExpr>),
    Quant { q: Quantifier, var: LocalId, lo: Box<TExpr>, hi: Box<TExpr>, body: Box<TExpr> },
}

#[derive(Clone, Debug)]
pub enum TStmt {
    Let(LocalId, TExpr),
    Assign(LocalId, TExpr),
    Expr(TExpr),
    Return(TExpr, Span),
    While { cond: TExpr, invariants: Vec<TExpr>, decreases: Option<TExpr>, body: TExpr, modified: Vec<LocalId>, span: Span },
    IndexAssign(LocalId, TExpr, TExpr, Span),
    Push(LocalId, TExpr, Span),
}

#[derive(Clone, Debug)]
pub enum TPat {
    Wild,
    Bind(LocalId),
    Ctor(usize, Vec<TPat>),
    Int(i128),
    Bool(bool),
}

impl TPat {
    pub fn bindings(&self, out: &mut Vec<LocalId>) {
        match self {
            TPat::Bind(id) => out.push(*id),
            TPat::Ctor(_, subs) => subs.iter().for_each(|p| p.bindings(out)),
            _ => {}
        }
    }
}

impl Module {
    pub fn peel(&self, t: &Ty) -> Ty {
        match t {
            Ty::Alias(a) => self.peel(&self.aliases[*a].base),
            _ => t.clone(),
        }
    }

    pub fn erase(&self, t: &Ty) -> Ty {
        match t {
            Ty::Alias(a) => self.erase(&self.aliases[*a].base),
            Ty::Option(x) => Ty::Option(Box::new(self.erase(x))),
            Ty::Result(a, b) => Ty::Result(Box::new(self.erase(a)), Box::new(self.erase(b))),
            Ty::Array(x) => Ty::Array(Box::new(self.erase(x))),
            _ => t.clone(),
        }
    }

    pub fn contains_array(&self, t: &Ty) -> bool {
        match self.peel(t) {
            Ty::Array(_) => true,
            Ty::Option(x) => self.contains_array(&x),
            Ty::Result(a, b) => self.contains_array(&a) || self.contains_array(&b),
            Ty::Record(r) => self.records[r].fields.iter().any(|(_, f)| self.contains_array(f)),
            Ty::Enum(e) => self.enums[e].variants.iter().any(|v| v.fields.iter().any(|(_, f)| self.contains_array(f))),
            _ => false,
        }
    }

    /// Payload types of each variant of an enum-like type.
    pub fn variants(&self, t: &Ty) -> Vec<(String, Vec<(String, Ty)>)> {
        match self.peel(t) {
            Ty::Enum(e) => self.enums[e].variants.iter().map(|v| (v.name.clone(), v.fields.clone())).collect(),
            Ty::Option(x) => vec![("None".into(), vec![]), ("Some".into(), vec![("value".into(), *x)])],
            Ty::Result(a, b) => vec![("Ok".into(), vec![("value".into(), *a)]), ("Err".into(), vec![("error".into(), *b)])],
            _ => vec![],
        }
    }

    pub fn show(&self, t: &Ty) -> String {
        match t {
            Ty::Unit => "()".into(),
            Ty::Never => "never".into(),
            Ty::Int => "int".into(),
            Ty::I128 => "i128".into(),
            Ty::Bool => "bool".into(),
            Ty::Str => "str".into(),
            Ty::Record(r) => self.records[*r].name.clone(),
            Ty::Enum(e) => self.enums[*e].name.clone(),
            Ty::Alias(a) => self.aliases[*a].name.clone(),
            Ty::Option(x) => format!("{}?", self.show(x)),
            Ty::Result(a, b) => format!("Result<{}, {}>", self.show(a), self.show(b)),
            Ty::Array(x) => format!("[{}]", self.show(x)),
        }
    }
}

pub fn mentions_any(e: &TExpr, ids: &[LocalId]) -> bool {
    match &e.kind {
        TExprKind::Local(id) => ids.contains(id),
        TExprKind::Field(x, _) | TExprKind::Unary(_, x) | TExprKind::Coerce(x, _) | TExprKind::Len(x) | TExprKind::Is(x, _) => mentions_any(x, ids),
        TExprKind::Binary(_, a, b) | TExprKind::Index(a, b) | TExprKind::ArrayRepeat(a, b) => mentions_any(a, ids) || mentions_any(b, ids),
        TExprKind::Quant { lo, hi, body, .. } => mentions_any(lo, ids) || mentions_any(hi, ids) || mentions_any(body, ids),
        TExprKind::If(c, t, f) => mentions_any(c, ids) || mentions_any(t, ids) || mentions_any(f, ids),
        TExprKind::Record(xs) | TExprKind::Ctor(_, xs) | TExprKind::Call(_, xs) | TExprKind::Print(xs) | TExprKind::ArrayLit(xs) => xs.iter().any(|x| mentions_any(x, ids)),
        TExprKind::Block(_, t) => mentions_any(t, ids),
        _ => false,
    }
}

/// Structural equality of the expressions a contract can index with (spans and types ignored).
pub fn same(a: &TExpr, b: &TExpr) -> bool {
    match (&a.kind, &b.kind) {
        (TExprKind::Int(x), TExprKind::Int(y)) => x == y,
        (TExprKind::Local(x), TExprKind::Local(y)) => x == y,
        (TExprKind::Field(x, i), TExprKind::Field(y, j)) => i == j && same(x, y),
        (TExprKind::Len(x), TExprKind::Len(y)) => same(x, y),
        (TExprKind::Unary(o, x), TExprKind::Unary(p, y)) => o == p && same(x, y),
        (TExprKind::Binary(o, x1, x2), TExprKind::Binary(p, y1, y2)) => o == p && same(x1, y1) && same(x2, y2),
        (TExprKind::Index(x1, x2), TExprKind::Index(y1, y2)) => same(x1, y1) && same(x2, y2),
        _ => false,
    }
}

/// `arr[i] op arr[j]` for every `lo <= i < j < hi`: what each way of writing "sorted" means.
/// Adjacent pairs (`forall k in L..H: a[k - 1] <= a[k]`) and all pairs
/// (`forall i in L..H: forall j in i..H: a[i] <= a[j]`) state the same thing, and the adjacent
/// form is the one that can be checked in one pass.
pub struct Chain<'a> {
    pub arr: &'a TExpr,
    pub lo: TExpr,
    pub hi: TExpr,
    pub op: BinOp,
}

impl Chain<'_> {
    /// `forall k in lo + 1..hi: arr[k - 1] op arr[k]`, bound to `var`.
    pub fn adjacent(&self, var: LocalId, span: Span) -> TExpr {
        let int = |kind| TExpr { kind, ty: Ty::Int, span };
        let lo = match &self.lo.kind {
            TExprKind::Int(v) => int(TExprKind::Int(v + 1)),
            TExprKind::Binary(BinOp::Sub, x, one) if matches!(one.kind, TExprKind::Int(1)) => (**x).clone(),
            _ => int(TExprKind::Binary(BinOp::Add, Box::new(self.lo.clone()), Box::new(int(TExprKind::Int(1))))),
        };
        let k = int(TExprKind::Local(var));
        let prev = int(TExprKind::Binary(BinOp::Sub, Box::new(k.clone()), Box::new(int(TExprKind::Int(1)))));
        let elem = |i: TExpr| TExpr { kind: TExprKind::Index(Box::new(self.arr.clone()), Box::new(i)), ty: Ty::Int, span };
        let body = TExpr { kind: TExprKind::Binary(self.op, Box::new(elem(prev)), Box::new(elem(k))), ty: Ty::Bool, span };
        TExpr { kind: TExprKind::Quant { q: Quantifier::Forall, var, lo: Box::new(lo), hi: Box::new(self.hi.clone()), body: Box::new(body) }, ty: Ty::Bool, span }
    }
}

fn flip(op: BinOp) -> BinOp {
    match op {
        BinOp::Lt => BinOp::Gt,
        BinOp::Le => BinOp::Ge,
        BinOp::Gt => BinOp::Lt,
        BinOp::Ge => BinOp::Le,
        _ => op,
    }
}

/// `x[p] op y[q]` over one array, for an ordering `op`.
fn compare(e: &TExpr) -> Option<(&TExpr, &TExpr, &TExpr, BinOp)> {
    let TExprKind::Binary(op @ (BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge), l, r) = &e.kind else { return None };
    let (TExprKind::Index(x, p), TExprKind::Index(y, q)) = (&l.kind, &r.kind) else { return None };
    same(x, y).then_some((&**x, &**p, &**q, *op))
}

fn is_local(e: &TExpr, id: LocalId) -> bool {
    matches!(e.kind, TExprKind::Local(v) if v == id)
}

/// `id + d` (or `d + id`), `id - d`: the offset `d`.
fn offset(e: &TExpr, id: LocalId) -> Option<i128> {
    match &e.kind {
        TExprKind::Local(v) if *v == id => Some(0),
        TExprKind::Binary(BinOp::Add, a, b) => match (&a.kind, &b.kind) {
            (_, TExprKind::Int(d)) if is_local(a, id) => Some(*d),
            (TExprKind::Int(d), _) if is_local(b, id) => Some(*d),
            _ => None,
        },
        TExprKind::Binary(BinOp::Sub, a, b) => match &b.kind {
            TExprKind::Int(d) if is_local(a, id) => Some(-d),
            _ => None,
        },
        _ => None,
    }
}

pub fn chain(e: &TExpr) -> Option<Chain<'_>> {
    let TExprKind::Quant { q: Quantifier::Forall, var: i, lo, hi, body } = &e.kind else { return None };
    let int = |kind| TExpr { kind, ty: Ty::Int, span: e.span };
    if let Some((arr, p, q, op)) = compare(body) {
        if mentions_any(arr, &[*i]) {
            return None;
        }
        let (p, q) = (offset(p, *i)?, offset(q, *i)?);
        let op = if p < q { op } else { flip(op) };
        return match (p.min(q), p.max(q)) {
            (-1, 0) => {
                let lo = match lo.kind {
                    TExprKind::Int(v) => int(TExprKind::Int(v - 1)),
                    _ => int(TExprKind::Binary(BinOp::Sub, lo.clone(), Box::new(int(TExprKind::Int(1))))),
                };
                Some(Chain { arr, lo, hi: (**hi).clone(), op })
            }
            (0, 1) => {
                let hi = match &hi.kind {
                    TExprKind::Binary(BinOp::Sub, h, one) if matches!(one.kind, TExprKind::Int(1)) => (**h).clone(),
                    _ => int(TExprKind::Binary(BinOp::Add, hi.clone(), Box::new(int(TExprKind::Int(1))))),
                };
                Some(Chain { arr, lo: (**lo).clone(), hi, op })
            }
            _ => None,
        };
    }
    let TExprKind::Quant { q: Quantifier::Forall, var: j, lo: jlo, hi: jhi, body: inner } = &body.kind else { return None };
    if !same(jhi, hi) || mentions_any(jlo, &[*j]) {
        return None;
    }
    // How far above `i` the range of `j` starts: 0, 1, or "anywhere, with a guard".
    let (cmp, strict_only) = match offset(jlo, *i) {
        Some(0) => (&**inner, false),
        Some(1) => (&**inner, true),
        _ if same(jlo, lo) => {
            let TExprKind::Binary(BinOp::Implies, guard, cmp) = &inner.kind else { return None };
            let TExprKind::Binary(g, a, b) = &guard.kind else { return None };
            let (a, b) = if matches!(g, BinOp::Gt | BinOp::Ge) { (b, a) } else { (a, b) };
            if !is_local(a, *i) || !is_local(b, *j) {
                return None;
            }
            match g {
                BinOp::Lt | BinOp::Gt => (&**cmp, true),
                BinOp::Le | BinOp::Ge => (&**cmp, false),
                _ => return None,
            }
        }
        _ => return None,
    };
    let (arr, p, q, op) = compare(cmp)?;
    if mentions_any(arr, &[*i, *j]) {
        return None;
    }
    let op = match (is_local(p, *i) && is_local(q, *j), is_local(p, *j) && is_local(q, *i)) {
        (true, _) => op,
        (_, true) => flip(op),
        _ => return None,
    };
    // With `j` starting at `i`, `a[i] < a[i]` would be false: that is not a sortedness fact.
    if !strict_only && matches!(op, BinOp::Lt | BinOp::Gt) {
        return None;
    }
    Some(Chain { arr, lo: (**lo).clone(), hi: (**hi).clone(), op })
}
