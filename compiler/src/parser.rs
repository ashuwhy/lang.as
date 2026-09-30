//! Recursive-descent parser with precedence climbing for expressions.

use crate::ast::*;
use crate::diag::{Diagnostic, Span};
use crate::lexer::{Tok, Token};

pub struct Parser {
    toks: Vec<Token>,
    pos: usize,
}

type PResult<T> = Result<T, Diagnostic>;

pub fn parse(toks: Vec<Token>) -> PResult<Program> {
    let mut p = Parser { toks, pos: 0 };
    let mut items = Vec::new();
    p.skip_semis();
    while p.peek() != &Tok::Eof {
        items.push(p.item()?);
        p.skip_semis();
    }
    Ok(Program { items })
}

/// Parse a standalone expression (used for contracts stored in `touchplate.lock`).
pub fn parse_expr(toks: Vec<Token>) -> PResult<Expr> {
    let mut p = Parser { toks, pos: 0 };
    let e = p.expr()?;
    p.skip_semis();
    if p.peek() != &Tok::Eof {
        return Err(p.err_here("expected the end of the expression"));
    }
    Ok(e)
}

fn describe(t: &Tok) -> String {
    match t {
        Tok::Ident(s) => format!("identifier `{s}`"),
        Tok::Int(v) => format!("number `{v}`"),
        Tok::Str(_) => "a string".into(),
        Tok::Kw(k) => format!("`{k}`"),
        Tok::P(p) => format!("`{p}`"),
        Tok::Semi => "end of line".into(),
        Tok::Eof => "end of file".into(),
    }
}

impl Parser {
    fn peek(&self) -> &Tok {
        &self.toks[self.pos].tok
    }
    fn peek_at(&self, n: usize) -> &Tok {
        &self.toks[(self.pos + n).min(self.toks.len() - 1)].tok
    }
    fn span(&self) -> Span {
        self.toks[self.pos].span
    }
    fn prev_span(&self) -> Span {
        self.toks[self.pos.saturating_sub(1)].span
    }
    fn bump(&mut self) -> Token {
        let t = self.toks[self.pos].clone();
        if self.pos + 1 < self.toks.len() {
            self.pos += 1;
        }
        t
    }
    fn is_p(&self, p: &str) -> bool {
        matches!(self.peek(), Tok::P(q) if *q == p)
    }
    fn is_kw(&self, k: &str) -> bool {
        matches!(self.peek(), Tok::Kw(q) if *q == k)
    }
    fn eat_p(&mut self, p: &str) -> bool {
        if self.is_p(p) {
            self.bump();
            true
        } else {
            false
        }
    }
    fn eat_kw(&mut self, k: &str) -> bool {
        if self.is_kw(k) {
            self.bump();
            true
        } else {
            false
        }
    }
    fn skip_semis(&mut self) {
        while self.peek() == &Tok::Semi {
            self.bump();
        }
    }
    fn err_here(&self, msg: &str) -> Diagnostic {
        Diagnostic::error("E0002", self.span(), format!("{msg}, found {}", describe(self.peek())))
    }
    fn expect_p(&mut self, p: &str) -> PResult<Span> {
        if self.is_p(p) {
            Ok(self.bump().span)
        } else {
            Err(self.err_here(&format!("expected `{p}`")))
        }
    }
    fn ident(&mut self) -> PResult<(String, Span)> {
        match self.peek().clone() {
            Tok::Ident(s) => {
                let sp = self.bump().span;
                Ok((s, sp))
            }
            _ => Err(self.err_here("expected a name")),
        }
    }

    fn item(&mut self) -> PResult<Item> {
        let start = self.span();
        let is_pub = self.eat_kw("pub");
        if self.eat_kw("fn") {
            return Ok(Item::Fn(self.fn_decl(start, is_pub)?));
        }
        if is_pub {
            return Err(self.err_here("expected `fn` after `pub`").with_note("only functions can be `pub` in v0.1"));
        }
        if self.eat_kw("type") {
            let (name, _) = self.ident()?;
            self.expect_p("=")?;
            let ty = self.type_expr()?;
            let refine = if self.eat_kw("where") { Some(self.expr()?) } else { None };
            return Ok(Item::Type(TypeDecl { name, ty, refine, span: start.to(self.prev_span()) }));
        }
        if self.eat_kw("enum") {
            let (name, _) = self.ident()?;
            self.expect_p("{")?;
            let mut variants = Vec::new();
            loop {
                self.skip_semis();
                if self.eat_p("}") {
                    break;
                }
                let (vname, vspan) = self.ident()?;
                let mut fields = Vec::new();
                if self.eat_p("(") {
                    loop {
                        self.skip_semis();
                        if self.eat_p(")") {
                            break;
                        }
                        let (fname, _) = self.ident()?;
                        self.expect_p(":")?;
                        fields.push((fname, self.type_expr()?));
                        self.skip_semis();
                        if !self.eat_p(",") {
                            self.skip_semis();
                            self.expect_p(")")?;
                            break;
                        }
                    }
                }
                variants.push(VariantDecl { name: vname, fields, span: vspan.to(self.prev_span()) });
                self.skip_semis();
                if !self.eat_p(",") {
                    self.skip_semis();
                    self.expect_p("}")?;
                    break;
                }
            }
            return Ok(Item::Enum(EnumDecl { name, variants, span: start.to(self.prev_span()) }));
        }
        Err(self.err_here("expected `fn`, `type` or `enum` at the top level"))
    }

    fn fn_decl(&mut self, start: Span, is_pub: bool) -> PResult<FnDecl> {
        let (name, _) = self.ident()?;
        self.expect_p("(")?;
        let mut params = Vec::new();
        loop {
            self.skip_semis();
            if self.eat_p(")") {
                break;
            }
            let (pname, ps) = self.ident()?;
            self.expect_p(":").map_err(|d| d.with_fix(format!("give the parameter a type, e.g. `{pname}: int`")))?;
            let ty = self.type_expr()?;
            params.push(Param { name: pname, span: ps.to(ty.span()), ty });
            self.skip_semis();
            if !self.eat_p(",") {
                self.skip_semis();
                self.expect_p(")")?;
                break;
            }
        }
        let ret = if self.eat_p("->") { Some(self.type_expr()?) } else { None };
        let sig_span = start.to(self.prev_span());
        let (mut requires, mut ensures, mut uses) = (vec![], vec![], vec![]);
        loop {
            self.skip_semis();
            if self.eat_kw("requires") {
                requires.push(self.expr_no_record()?);
            } else if self.eat_kw("ensures") {
                ensures.push(self.expr_no_record()?);
            } else if self.eat_kw("uses") {
                loop {
                    let (e, s) = self.ident()?;
                    uses.push((e, s));
                    if !self.eat_p(",") {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        if !self.is_p("{") {
            return Err(self.err_here("expected `{` to start the function body"));
        }
        let body = self.block()?;
        Ok(FnDecl { name, is_pub, params, ret, requires, ensures, uses, span: start.to(body.span), body, sig_span })
    }

    fn type_expr(&mut self) -> PResult<TypeExpr> {
        let start = self.span();
        let base = if self.eat_p("[") {
            let elem = self.type_expr()?;
            self.expect_p("]")?;
            TypeExpr::Array(Box::new(elem), start.to(self.prev_span()))
        } else if self.eat_p("{") {
            let mut fields = Vec::new();
            loop {
                self.skip_semis();
                if self.eat_p("}") {
                    break;
                }
                let (f, _) = self.ident()?;
                self.expect_p(":")?;
                fields.push((f, self.type_expr()?));
                self.skip_semis();
                if !self.eat_p(",") {
                    self.skip_semis();
                    self.expect_p("}")?;
                    break;
                }
            }
            TypeExpr::Record(fields, start.to(self.prev_span()))
        } else {
            let (name, _) = self.ident()?;
            let mut args = Vec::new();
            if self.eat_p("<") {
                loop {
                    args.push(self.type_expr()?);
                    if !self.eat_p(",") {
                        break;
                    }
                }
                self.expect_p(">")?;
            }
            TypeExpr::Name(name, args, start.to(self.prev_span()))
        };
        if self.eat_p("?") {
            return Ok(TypeExpr::Opt(Box::new(base), start.to(self.prev_span())));
        }
        Ok(base)
    }

    fn block(&mut self) -> PResult<Block> {
        let start = self.expect_p("{")?;
        let mut stmts = Vec::new();
        loop {
            self.skip_semis();
            if self.eat_p("}") {
                break;
            }
            stmts.push(self.stmt()?);
            if !(self.peek() == &Tok::Semi || self.is_p("}")) {
                return Err(self.err_here("expected the end of the statement").with_note("put each statement on its own line or separate them with `;`"));
            }
        }
        Ok(Block { stmts, span: start.to(self.prev_span()) })
    }

    fn stmt(&mut self) -> PResult<Stmt> {
        let start = self.span();
        if self.is_kw("let") || self.is_kw("var") {
            let mutable = self.is_kw("var");
            self.bump();
            let (name, _) = self.ident()?;
            let ty = if self.eat_p(":") { Some(self.type_expr()?) } else { None };
            self.expect_p("=")?;
            let init = self.expr()?;
            return Ok(Stmt::Let { name, mutable, ty, span: start.to(init.span), init });
        }
        if self.eat_kw("return") {
            if self.peek() == &Tok::Semi || self.is_p("}") {
                return Ok(Stmt::Return(None, start));
            }
            let e = self.expr()?;
            return Ok(Stmt::Return(Some(e.clone()), start.to(e.span)));
        }
        if self.eat_kw("while") {
            let cond = self.expr_no_record()?;
            let (mut invariants, mut decreases) = (vec![], None);
            loop {
                self.skip_semis();
                if self.eat_kw("invariant") {
                    invariants.push(self.expr_no_record()?);
                } else if self.eat_kw("decreases") {
                    decreases = Some(self.expr_no_record()?);
                } else {
                    break;
                }
            }
            let body = self.block()?;
            return Ok(Stmt::While { cond, invariants, decreases, span: start.to(body.span), body });
        }
        if self.eat_kw("for") {
            let (var, _) = self.ident()?;
            if !self.eat_kw("in") {
                return Err(self.err_here("expected `in` after the loop variable"));
            }
            let lo = self.expr_no_record()?;
            self.expect_p("..")?;
            let hi = self.expr_no_record()?;
            let mut invariants = vec![];
            loop {
                self.skip_semis();
                if self.eat_kw("invariant") {
                    invariants.push(self.expr_no_record()?);
                } else {
                    break;
                }
            }
            let body = self.block()?;
            return Ok(Stmt::For { var, lo, hi, invariants, span: start.to(body.span), body });
        }
        if let (Tok::Ident(name), Tok::P("[")) = (self.peek().clone(), self.peek_at(1).clone()) {
            let target = self.expr()?;
            let op = match self.peek() {
                Tok::P("=") => Some(None),
                Tok::P("+=") => Some(Some(BinOp::Add)),
                Tok::P("-=") => Some(Some(BinOp::Sub)),
                Tok::P("*=") => Some(Some(BinOp::Mul)),
                _ => None,
            };
            let Some(op) = op else { return Ok(Stmt::Expr(target)) };
            let ExprKind::Index(base, index) = target.kind else {
                return Err(Diagnostic::error("E0002", target.span, "only a variable or `name[index]` can be assigned"));
            };
            if !matches!(&base.kind, ExprKind::Name(n) if *n == name) {
                return Err(Diagnostic::error("E0002", base.span, "only `name[index]` can be assigned"));
            }
            self.bump();
            let value = self.expr()?;
            return Ok(Stmt::IndexAssign { name, index: *index, op, span: start.to(value.span), value });
        }
        if let Tok::Ident(name) = self.peek().clone() {
            let op = match self.peek_at(1) {
                Tok::P("=") => Some(None),
                Tok::P("+=") => Some(Some(BinOp::Add)),
                Tok::P("-=") => Some(Some(BinOp::Sub)),
                Tok::P("*=") => Some(Some(BinOp::Mul)),
                _ => None,
            };
            if let Some(op) = op {
                self.bump();
                self.bump();
                let value = self.expr()?;
                return Ok(Stmt::Assign { name, op, span: start.to(value.span), value });
            }
        }
        Ok(Stmt::Expr(self.expr()?))
    }

    pub fn expr(&mut self) -> PResult<Expr> {
        self.binary(0, true)
    }
    fn expr_no_record(&mut self) -> PResult<Expr> {
        self.binary(0, false)
    }

    fn binop(&self) -> Option<(BinOp, u8, bool)> {
        // (op, precedence, right-assoc)
        let r = match self.peek() {
            Tok::P("==>") => (BinOp::Implies, 1, true),
            Tok::P("||") => (BinOp::Or, 2, false),
            Tok::P("&&") => (BinOp::And, 3, false),
            Tok::P("==") => (BinOp::Eq, 4, false),
            Tok::P("!=") => (BinOp::Ne, 4, false),
            Tok::P("<") => (BinOp::Lt, 4, false),
            Tok::P("<=") => (BinOp::Le, 4, false),
            Tok::P(">") => (BinOp::Gt, 4, false),
            Tok::P(">=") => (BinOp::Ge, 4, false),
            Tok::P("+") => (BinOp::Add, 5, false),
            Tok::P("-") => (BinOp::Sub, 5, false),
            Tok::P("*") => (BinOp::Mul, 6, false),
            Tok::P("/") => (BinOp::Div, 6, false),
            Tok::P("%") => (BinOp::Rem, 6, false),
            _ => return None,
        };
        Some(r)
    }

    fn binary(&mut self, min_prec: u8, rec: bool) -> PResult<Expr> {
        let mut lhs = self.unary(rec)?;
        loop {
            if self.is_kw("is") && min_prec <= 4 {
                self.bump();
                let pat = self.pattern()?;
                let span = lhs.span.to(pat.span);
                lhs = Expr { kind: ExprKind::Is(Box::new(lhs), pat), span };
                continue;
            }
            let Some((op, prec, right)) = self.binop() else { break };
            if prec < min_prec {
                break;
            }
            self.bump();
            self.skip_semis_after_operator();
            let rhs = self.binary(if right { prec } else { prec + 1 }, rec)?;
            let span = lhs.span.to(rhs.span);
            lhs = Expr { kind: ExprKind::Binary(op, Box::new(lhs), Box::new(rhs)), span };
        }
        Ok(lhs)
    }

    fn skip_semis_after_operator(&mut self) {
        // A line ending in a binary operator continues on the next line.
        while self.peek() == &Tok::Semi && self.toks[self.pos].span.lo == self.toks[self.pos].span.hi {
            self.bump();
        }
    }

    fn unary(&mut self, rec: bool) -> PResult<Expr> {
        let start = self.span();
        if self.eat_p("-") {
            let e = self.unary(rec)?;
            if let ExprKind::Int(v) = e.kind {
                return Ok(Expr { kind: ExprKind::Int(-v), span: start.to(e.span) });
            }
            return Ok(Expr { span: start.to(e.span), kind: ExprKind::Unary(UnOp::Neg, Box::new(e)) });
        }
        if self.eat_p("!") {
            let e = self.unary(rec)?;
            return Ok(Expr { span: start.to(e.span), kind: ExprKind::Unary(UnOp::Not, Box::new(e)) });
        }
        self.postfix(rec)
    }

    fn postfix(&mut self, rec: bool) -> PResult<Expr> {
        let mut e = self.primary(rec)?;
        loop {
            if self.eat_p(".") {
                let (f, fs) = self.ident()?;
                e = Expr { span: e.span.to(fs), kind: ExprKind::Field(Box::new(e), f) };
            } else if self.is_p("[") && !self.toks[self.pos].newline_before {
                self.bump();
                self.skip_semis();
                let idx = self.expr()?;
                self.skip_semis();
                self.expect_p("]")?;
                e = Expr { span: e.span.to(self.prev_span()), kind: ExprKind::Index(Box::new(e), Box::new(idx)) };
            } else if self.is_p("(") {
                self.bump();
                let args = self.comma_list(")")?;
                e = Expr { span: e.span.to(self.prev_span()), kind: ExprKind::Call(Box::new(e), args) };
            } else {
                break;
            }
        }
        Ok(e)
    }

    fn comma_list(&mut self, close: &str) -> PResult<Vec<Expr>> {
        let mut v = Vec::new();
        loop {
            self.skip_semis();
            if self.eat_p(close) {
                break;
            }
            if let (Tok::Ident(label), Tok::P(":")) = (self.peek().clone(), self.peek_at(1).clone()) {
                let start = self.span();
                self.bump();
                self.bump();
                let e = self.expr()?;
                v.push(Expr { span: start.to(e.span), kind: ExprKind::Named(label, Box::new(e)) });
            } else {
                v.push(self.expr()?);
            }
            self.skip_semis();
            if !self.eat_p(",") {
                self.skip_semis();
                self.expect_p(close)?;
                break;
            }
        }
        Ok(v)
    }

    fn looks_like_record(&self) -> bool {
        // `{` followed by `}`, `...`, or `name :`
        matches!(self.peek_at(1), Tok::P("}") | Tok::P("..."))
            || (matches!(self.peek_at(1), Tok::Ident(_)) && matches!(self.peek_at(2), Tok::P(":")))
            || (matches!(self.peek_at(1), Tok::Semi) && matches!(self.peek_at(2), Tok::Ident(_)) && matches!(self.peek_at(3), Tok::P(":")))
    }

    fn primary(&mut self, rec: bool) -> PResult<Expr> {
        let start = self.span();
        let kind = match self.peek().clone() {
            Tok::Int(v) => {
                self.bump();
                ExprKind::Int(v)
            }
            Tok::Str(s) => {
                self.bump();
                ExprKind::Str(s)
            }
            Tok::Kw("true") => {
                self.bump();
                ExprKind::Bool(true)
            }
            Tok::Kw("false") => {
                self.bump();
                ExprKind::Bool(false)
            }
            // `sum` is a quantifier only in this shape, so it stays usable as a name.
            Tok::Ident(n) if n == "sum" && matches!(self.peek_at(1), Tok::Ident(_)) && matches!(self.peek_at(2), Tok::Kw("in")) => return self.quantifier(Quantifier::Sum),
            Tok::Ident(n) => {
                self.bump();
                ExprKind::Name(n)
            }
            Tok::P("(") => {
                self.bump();
                self.skip_semis();
                let e = self.expr()?;
                self.skip_semis();
                self.expect_p(")")?;
                return Ok(Expr { kind: e.kind, span: start.to(self.prev_span()) });
            }
            Tok::P("{") if rec && self.looks_like_record() => {
                self.bump();
                let mut spread = None;
                let mut fields = Vec::new();
                loop {
                    self.skip_semis();
                    if self.eat_p("}") {
                        break;
                    }
                    if self.eat_p("...") {
                        if spread.is_some() || !fields.is_empty() {
                            return Err(Diagnostic::error("E0002", self.prev_span(), "`...base` must come first in a record literal, once"));
                        }
                        spread = Some(Box::new(self.expr()?));
                    } else {
                        let (f, _) = self.ident()?;
                        self.expect_p(":")?;
                        fields.push((f, self.expr()?));
                    }
                    self.skip_semis();
                    if !self.eat_p(",") {
                        self.skip_semis();
                        self.expect_p("}")?;
                        break;
                    }
                }
                ExprKind::Record { spread, fields }
            }
            Tok::P("{") => ExprKind::Block(self.block()?),
            Tok::P("[") => {
                self.bump();
                self.skip_semis();
                if self.eat_p("]") {
                    ExprKind::ArrayLit(vec![])
                } else {
                    let first = self.expr()?;
                    // An explicit `;` (not an automatic line end) makes `[value; count]`.
                    if self.peek() == &Tok::Semi && self.toks[self.pos].span.lo != self.toks[self.pos].span.hi {
                        self.bump();
                        let count = self.expr()?;
                        self.expect_p("]")?;
                        ExprKind::ArrayRepeat(Box::new(first), Box::new(count))
                    } else {
                        let mut items = vec![first];
                        self.skip_semis();
                        if self.eat_p(",") {
                            items.extend(self.comma_list("]")?);
                        } else {
                            self.expect_p("]")?;
                        }
                        ExprKind::ArrayLit(items)
                    }
                }
            }
            Tok::Kw("forall") => return self.quantifier(Quantifier::Forall),
            Tok::Kw("exists") => return self.quantifier(Quantifier::Exists),
            Tok::Kw("if") => return self.if_expr(),
            Tok::Kw("match") => {
                self.bump();
                let scrut = self.expr_no_record()?;
                self.expect_p("{")?;
                let mut arms = Vec::new();
                loop {
                    self.skip_semis();
                    if self.eat_p("}") {
                        break;
                    }
                    let pat = self.pattern()?;
                    self.expect_p("=>")?;
                    let body = self.expr()?;
                    arms.push(Arm { span: pat.span.to(body.span), pat, body });
                    self.eat_p(",");
                }
                ExprKind::Match(Box::new(scrut), arms)
            }
            _ => return Err(self.err_here("expected an expression")),
        };
        Ok(Expr { kind, span: start.to(self.prev_span()) })
    }

    fn quantifier(&mut self, q: Quantifier) -> PResult<Expr> {
        let start = self.span();
        self.bump();
        let (var, _) = self.ident()?;
        if !self.eat_kw("in") {
            return Err(self.err_here("expected `in`, as in `forall i in 0..n: ...`"));
        }
        let lo = self.expr_no_record()?;
        self.expect_p("..")?;
        let hi = self.expr_no_record()?;
        self.expect_p(":")?;
        let body = self.expr()?;
        Ok(Expr { kind: ExprKind::Quant { q, var, lo: Box::new(lo), hi: Box::new(hi), body: Box::new(body) }, span: start.to(self.prev_span()) })
    }

    fn if_expr(&mut self) -> PResult<Expr> {
        let start = self.span();
        self.bump(); // if
        let cond = self.expr_no_record()?;
        let then = self.block()?;
        // `else` may follow on the next line.
        let save = self.pos;
        self.skip_semis();
        let els = if self.eat_kw("else") {
            if self.is_kw("if") {
                Some(Box::new(self.if_expr()?))
            } else {
                let b = self.block()?;
                Some(Box::new(Expr { span: b.span, kind: ExprKind::Block(b) }))
            }
        } else {
            self.pos = save;
            None
        };
        Ok(Expr { kind: ExprKind::If(Box::new(cond), then, els), span: start.to(self.prev_span()) })
    }

    fn pattern(&mut self) -> PResult<Pattern> {
        let start = self.span();
        let kind = match self.peek().clone() {
            Tok::Ident(n) if n == "_" => {
                self.bump();
                PatKind::Wild
            }
            Tok::Ident(n) => {
                self.bump();
                if self.eat_p("(") {
                    let mut subs = Vec::new();
                    loop {
                        if self.eat_p(")") {
                            break;
                        }
                        subs.push(self.pattern()?);
                        if !self.eat_p(",") {
                            self.expect_p(")")?;
                            break;
                        }
                    }
                    PatKind::Ctor(n, subs)
                } else if n.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                    PatKind::Ctor(n, vec![])
                } else {
                    PatKind::Bind(n)
                }
            }
            Tok::Int(v) => {
                self.bump();
                PatKind::Int(v)
            }
            Tok::P("-") if matches!(self.peek_at(1), Tok::Int(_)) => {
                self.bump();
                let Tok::Int(v) = self.bump().tok else { unreachable!() };
                PatKind::Int(-v)
            }
            Tok::Kw("true") => {
                self.bump();
                PatKind::Bool(true)
            }
            Tok::Kw("false") => {
                self.bump();
                PatKind::Bool(false)
            }
            _ => return Err(self.err_here("expected a pattern")),
        };
        Ok(Pattern { kind, span: start.to(self.prev_span()) })
    }
}
