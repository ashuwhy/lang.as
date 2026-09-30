//! Surface syntax tree, as parsed. Names are resolved and types checked in `check.rs`.

use crate::diag::Span;

#[derive(Clone, Debug)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Clone, Debug)]
pub enum Item {
    Type(TypeDecl),
    Enum(EnumDecl),
    Fn(FnDecl),
}

#[derive(Clone, Debug)]
pub struct TypeDecl {
    pub name: String,
    pub ty: TypeExpr,
    pub refine: Option<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<VariantDecl>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct VariantDecl {
    pub name: String,
    pub fields: Vec<(String, TypeExpr)>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
    pub ty: TypeExpr,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct FnDecl {
    pub name: String,
    pub is_pub: bool,
    pub params: Vec<Param>,
    pub ret: Option<TypeExpr>,
    pub requires: Vec<Expr>,
    pub ensures: Vec<Expr>,
    pub uses: Vec<(String, Span)>,
    pub body: Block,
    pub span: Span,
    /// From `fn` to the end of the return type.
    pub sig_span: Span,
}

#[derive(Clone, Debug)]
pub enum TypeExpr {
    Name(String, Vec<TypeExpr>, Span),
    Record(Vec<(String, TypeExpr)>, Span),
    Opt(Box<TypeExpr>, Span),
}

impl TypeExpr {
    pub fn span(&self) -> Span {
        match self {
            TypeExpr::Name(_, _, s) | TypeExpr::Record(_, s) | TypeExpr::Opt(_, s) => *s,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Let { name: String, mutable: bool, ty: Option<TypeExpr>, init: Expr, span: Span },
    Assign { name: String, op: Option<BinOp>, value: Expr, span: Span },
    Expr(Expr),
    Return(Option<Expr>, Span),
    While { cond: Expr, invariants: Vec<Expr>, decreases: Option<Expr>, body: Block, span: Span },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Implies,
}

impl BinOp {
    pub fn symbol(self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Rem => "%",
            BinOp::Eq => "==",
            BinOp::Ne => "!=",
            BinOp::Lt => "<",
            BinOp::Le => "<=",
            BinOp::Gt => ">",
            BinOp::Ge => ">=",
            BinOp::And => "&&",
            BinOp::Or => "||",
            BinOp::Implies => "==>",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum ExprKind {
    Int(i128),
    Bool(bool),
    Str(String),
    Name(String),
    Field(Box<Expr>, String),
    Call(Box<Expr>, Vec<Expr>),
    Record { spread: Option<Box<Expr>>, fields: Vec<(String, Expr)> },
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Is(Box<Expr>, Pattern),
    If(Box<Expr>, Block, Option<Box<Expr>>),
    Match(Box<Expr>, Vec<Arm>),
    Block(Block),
    /// `name: value` inside a call's argument list.
    Named(String, Box<Expr>),
}

#[derive(Clone, Debug)]
pub struct Arm {
    pub pat: Pattern,
    pub body: Expr,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    pub kind: PatKind,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub enum PatKind {
    Wild,
    Bind(String),
    Ctor(String, Vec<Pattern>),
    Int(i128),
    Bool(bool),
}
