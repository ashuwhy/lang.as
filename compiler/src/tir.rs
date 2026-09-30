//! Typed IR produced by the checker and shared by the verifier and the C backend.

use crate::ast::{BinOp, UnOp};
use crate::diag::Span;

pub type LocalId = u32;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ty {
    Unit,
    Never,
    Int,
    Bool,
    Str,
    Record(usize),
    Enum(usize),
    Alias(usize),
    Option(Box<Ty>),
    Result(Box<Ty>, Box<Ty>),
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
}

#[derive(Clone, Debug)]
pub enum TStmt {
    Let(LocalId, TExpr),
    Assign(LocalId, TExpr),
    Expr(TExpr),
    Return(TExpr, Span),
    While { cond: TExpr, invariants: Vec<TExpr>, decreases: Option<TExpr>, body: TExpr, modified: Vec<LocalId>, span: Span },
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
            _ => t.clone(),
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
            Ty::Bool => "bool".into(),
            Ty::Str => "str".into(),
            Ty::Record(r) => self.records[*r].name.clone(),
            Ty::Enum(e) => self.enums[*e].name.clone(),
            Ty::Alias(a) => self.aliases[*a].name.clone(),
            Ty::Option(x) => format!("{}?", self.show(x)),
            Ty::Result(a, b) => format!("Result<{}, {}>", self.show(a), self.show(b)),
        }
    }
}
