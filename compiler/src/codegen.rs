//! C backend. Emits GNU C (statement expressions) and lets the system compiler do the rest.
//! Every check the verifier proved disappears; every other check becomes a run-time check that
//! stops the program with the source location instead of misbehaving.

use std::collections::HashMap;

use crate::ast::{BinOp, UnOp};
use crate::diag::{Source, Span};
use crate::tir::*;
use crate::verify::{mangle, Site, SiteKind, Verdict};

const PRELUDE: &str = r#"#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
typedef uint8_t as_unit;
__attribute__((noreturn, cold)) static void as_fail(const char *at, const char *what) {
  fflush(stdout);
  fprintf(stderr, "%s: run-time check failed: %s\n", at, what);
  exit(101);
}
static inline int64_t as_add(int64_t a, int64_t b, const char *at) { int64_t r; if (__builtin_add_overflow(a, b, &r)) as_fail(at, "integer overflow"); return r; }
static inline int64_t as_sub(int64_t a, int64_t b, const char *at) { int64_t r; if (__builtin_sub_overflow(a, b, &r)) as_fail(at, "integer overflow"); return r; }
static inline int64_t as_mul(int64_t a, int64_t b, const char *at) { int64_t r; if (__builtin_mul_overflow(a, b, &r)) as_fail(at, "integer overflow"); return r; }
static inline int64_t as_neg(int64_t a, const char *at) { if (a == INT64_MIN) as_fail(at, "integer overflow"); return -a; }
static inline int64_t as_div(int64_t a, int64_t b, const char *at) { if (b == 0) as_fail(at, "division by zero"); if (a == INT64_MIN && b == -1) as_fail(at, "integer overflow"); return a / b; }
static inline int64_t as_rem(int64_t a, int64_t b, const char *at) { if (b == 0) as_fail(at, "division by zero"); if (a == INT64_MIN && b == -1) as_fail(at, "integer overflow"); return a % b; }
static void as_print_int(int64_t v) { printf("%lld", (long long)v); }
static void as_print_bool(bool v) { fputs(v ? "true" : "false", stdout); }
"#;

pub struct Codegen<'a> {
    m: &'a Module,
    src: &'a Source,
    verdicts: &'a HashMap<Site, Verdict>,
    out: String,
    tmp: usize,
    names: HashMap<LocalId, String>,
    cur: usize,
}

fn cname(s: &str) -> String {
    s.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' }).collect()
}

fn c_str(s: &str) -> String {
    let mut o = String::from("\"");
    for b in s.bytes() {
        match b {
            b'"' => o += "\\\"",
            b'\\' => o += "\\\\",
            b'\n' => o += "\\n",
            b'\t' => o += "\\t",
            32..=126 => o.push(b as char),
            _ => o += &format!("\\{:03o}", b),
        }
    }
    o.push('"');
    o
}

pub fn generate(m: &Module, src: &Source, verdicts: &HashMap<Site, Verdict>) -> String {
    let mut g = Codegen { m, src, verdicts, out: String::new(), tmp: 0, names: HashMap::new(), cur: 0 };
    g.run();
    g.out
}

impl<'a> Codegen<'a> {
    fn ty(&self, t: &Ty) -> String {
        match self.m.erase(t) {
            Ty::Int => "int64_t".into(),
            Ty::Bool => "bool".into(),
            Ty::Unit | Ty::Never => "as_unit".into(),
            Ty::Str => "const char *".into(),
            t => mangle(self.m, &t),
        }
    }

    fn fresh(&mut self) -> String {
        self.tmp += 1;
        format!("_t{}", self.tmp)
    }

    fn at(&self, s: Span) -> String {
        let (l, c) = self.src.line_col(s.lo);
        c_str(&format!("{}:{l}:{c}", self.src.name))
    }

    fn proved(&self, kind: SiteKind, span: Span) -> bool {
        matches!(self.verdicts.get(&Site { kind, span }), Some(Verdict::Proved))
    }

    fn local(&self, id: LocalId) -> String {
        self.names.get(&id).cloned().unwrap_or_else(|| format!("{}_{id}", cname(&self.m.locals[id as usize].name)))
    }

    fn run(&mut self) {
        self.out += PRELUDE;
        // Types, dependencies first.
        for t in crate::verify::datatypes(self.m) {
            let name = mangle(self.m, &t);
            match self.m.erase(&t) {
                Ty::Record(r) => {
                    let fields: Vec<String> = self.m.records[r].fields.iter().map(|(f, ft)| format!("{} f_{};", self.ty(ft), cname(f))).collect();
                    self.out += &format!("typedef struct {{ {} }} {name};\n", fields.join(" "));
                }
                _ => {
                    let vs = self.m.variants(&t);
                    let members: Vec<String> = vs
                        .iter()
                        .filter(|(_, fs)| !fs.is_empty())
                        .map(|(v, fs)| format!("struct {{ {} }} v_{v};", fs.iter().map(|(f, ft)| format!("{} f_{};", self.ty(ft), cname(f))).collect::<Vec<_>>().join(" ")))
                        .collect();
                    if members.is_empty() {
                        self.out += &format!("typedef struct {{ uint8_t tag; }} {name};\n");
                    } else {
                        self.out += &format!("typedef struct {{ uint8_t tag; union {{ {} }} u; }} {name};\n", members.join(" "));
                    }
                }
            }
        }
        // Structural equality, used by run-time contract checks.
        for t in crate::verify::datatypes(self.m) {
            let name = mangle(self.m, &t);
            let body = match self.m.erase(&t) {
                Ty::Record(r) => {
                    let parts: Vec<String> = self.m.records[r].fields.iter().map(|(f, ft)| self.eq(ft, &format!("a.f_{}", cname(f)), &format!("b.f_{}", cname(f)))).collect();
                    if parts.is_empty() { "true".into() } else { parts.join(" && ") }
                }
                _ => {
                    let vs = self.m.variants(&t);
                    let cases: Vec<String> = vs
                        .iter()
                        .enumerate()
                        .map(|(i, (v, fs))| {
                            let parts: Vec<String> = fs.iter().map(|(f, ft)| self.eq(ft, &format!("a.u.v_{v}.f_{}", cname(f)), &format!("b.u.v_{v}.f_{}", cname(f)))).collect();
                            format!("(a.tag == {i} && ({}))", if parts.is_empty() { "true".into() } else { parts.join(" && ") })
                        })
                        .collect();
                    format!("a.tag == b.tag && ({})", cases.join(" || "))
                }
            };
            self.out += &format!("static inline bool as_eq_{name}({name} a, {name} b) {{ return {body}; }}\n");
        }
        for (i, f) in self.m.funcs.iter().enumerate() {
            self.out += &format!("static {} as_fn_{}({});\n", self.ty(&f.ret), cname(&f.name), self.params(i));
        }
        for i in 0..self.m.funcs.len() {
            self.func(i);
        }
        if self.m.funcs.iter().any(|f| f.name == "main") {
            self.out += "int main(void) { as_fn_main(); return 0; }\n";
        }
    }

    fn eq(&self, t: &Ty, a: &str, b: &str) -> String {
        match self.m.erase(t) {
            Ty::Int | Ty::Bool | Ty::Unit | Ty::Never | Ty::Str => format!("{a} == {b}"),
            t => format!("as_eq_{}({a}, {b})", mangle(self.m, &t)),
        }
    }

    fn params(&self, fi: usize) -> String {
        let f = &self.m.funcs[fi];
        if f.params.is_empty() {
            return "void".into();
        }
        f.params.iter().map(|p| format!("{} {}", self.ty(&self.m.locals[*p as usize].ty), self.local(*p))).collect::<Vec<_>>().join(", ")
    }

    fn func(&mut self, fi: usize) {
        self.cur = fi;
        self.names.clear();
        let m = self.m;
        let f = &m.funcs[fi];
        let mut locals = vec![];
        collect_locals(&f.body, &mut locals);
        for e in f.requires.iter().chain(&f.ensures) {
            collect_locals(e, &mut locals);
        }
        let mut callees = vec![];
        calls_in(&f.body, &mut callees);
        for c in callees {
            for r in &m.funcs[c].requires {
                collect_locals(r, &mut locals);
            }
        }
        for a in &m.aliases {
            if let Some(p) = &a.pred {
                collect_locals(p, &mut locals);
            }
        }
        locals.sort();
        locals.dedup();
        let mut s = format!("static {} as_fn_{}({}) {{\n", self.ty(&f.ret), cname(&f.name), self.params(fi));
        for id in locals {
            s += &format!("  {} {} = {{0}};\n", self.ty(&self.m.locals[id as usize].ty), self.local(id));
        }
        // Every call site proved or checked the precondition and the parameter refinements,
        // so the optimizer may rely on them.
        for r in &f.requires {
            s += &format!("  {}\n", self.assume(r));
        }
        for &p in &f.params {
            if let Ty::Alias(a) = m.locals[p as usize].ty {
                s += &format!("  {}\n", self.assume_refinement(a, &self.local(p)));
            }
        }
        let body = self.expr(&f.body);
        let tail_span = match &f.body.kind {
            TExprKind::Block(_, t) => t.span,
            _ => f.body.span,
        };
        if f.body.ty == Ty::Never {
            s += &format!("  (void)({body});\n  as_fail({}, \"unreachable\");\n}}\n", self.at(f.span));
        } else if f.ret == Ty::Unit {
            let checks = self.ensures_checks(tail_span, "0");
            s += &format!("  (void)({body});\n{checks}  return 0;\n}}\n");
        } else {
            let checks = self.ensures_checks(tail_span, "_res");
            s += &format!("  {} _res = {body};\n{checks}  return _res;\n}}\n", self.ty(&f.ret));
        }
        self.out += &s;
    }

    fn ensures_checks(&mut self, span: Span, res: &str) -> String {
        let m = self.m;
        let f = &m.funcs[self.cur];
        let mut s = String::new();
        for (i, e) in f.ensures.iter().enumerate() {
            if self.proved(SiteKind::Ensures(i), span) {
                continue;
            }
            self.names.insert(f.result, res.to_string());
            let c = self.spec(e);
            s += &format!("  if (!({c})) as_fail({}, {});\n", self.at(span), c_str(&format!("ensures {}", f.ensures_src[i])));
        }
        s
    }

    /// Contract expressions checked at run time use 128-bit arithmetic, so they cannot overflow
    /// where the proof would have used mathematical integers.
    fn spec(&mut self, e: &TExpr) -> String {
        match &e.kind {
            TExprKind::Binary(op, l, r) => {
                let (a, b) = (self.spec(l), self.spec(r));
                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul => format!("((__int128)({a}) {} (__int128)({b}))", op.symbol()),
                    BinOp::Div | BinOp::Rem => format!("(({b}) == 0 ? 0 : (__int128)({a}) {} (__int128)({b}))", op.symbol()),
                    BinOp::Implies => format!("(!({a}) || ({b}))"),
                    BinOp::Eq | BinOp::Ne => {
                        let eq = self.eq(&l.ty, &format!("({a})"), &format!("({b})"));
                        if *op == BinOp::Eq { format!("({eq})") } else { format!("(!({eq}))") }
                    }
                    _ => format!("(({a}) {} ({b}))", op.symbol()),
                }
            }
            TExprKind::Unary(UnOp::Neg, x) => format!("(-(__int128)({}))", self.spec(x)),
            TExprKind::Unary(UnOp::Not, x) => format!("(!({}))", self.spec(x)),
            TExprKind::Field(x, i) => {
                let r = match self.m.peel(&x.ty) {
                    Ty::Record(r) => r,
                    _ => unreachable!(),
                };
                format!("({}).f_{}", self.spec(x), cname(&self.m.records[r].fields[*i].0))
            }
            TExprKind::Is(x, p) => {
                let v = self.fresh();
                let xs = self.spec(x);
                let test = self.pat(p, &x.ty, &v);
                format!("({{ {} {v} = {xs}; {test}; }})", self.ty(&x.ty))
            }
            TExprKind::If(c, t, f) => format!("(({}) ? ({}) : ({}))", self.spec(c), self.spec(t), self.spec(f)),
            TExprKind::Block(_, t) => self.spec(t),
            _ => self.expr(e),
        }
    }

    fn pat(&mut self, p: &TPat, t: &Ty, v: &str) -> String {
        match p {
            TPat::Wild => "1".into(),
            TPat::Bind(id) => format!("(({} = {v}), 1)", self.local(*id)),
            TPat::Int(i) => format!("({v} == {})", int_lit(*i)),
            TPat::Bool(b) => if *b { format!("({v})") } else { format!("(!{v})") },
            TPat::Ctor(vi, subs) => {
                let vs = self.m.variants(t);
                let (vname, fields) = vs[*vi].clone();
                let mut parts = vec![format!("({v}.tag == {vi})")];
                for (sp, (f, ft)) in subs.iter().zip(fields) {
                    if matches!(sp, TPat::Wild) {
                        continue;
                    }
                    parts.push(self.pat(sp, &ft, &format!("{v}.u.v_{vname}.f_{}", cname(&f))));
                }
                format!("({})", parts.join(" && "))
            }
        }
    }

    fn expr(&mut self, e: &TExpr) -> String {
        match &e.kind {
            TExprKind::Int(v) => int_lit(*v),
            TExprKind::Bool(b) => b.to_string(),
            TExprKind::Str(s) => c_str(s),
            TExprKind::Unit => "((as_unit)0)".into(),
            TExprKind::Local(id) => self.local(*id),
            TExprKind::Field(x, i) => {
                let Ty::Record(r) = self.m.peel(&x.ty) else { unreachable!() };
                format!("({}).f_{}", self.expr(x), cname(&self.m.records[r].fields[*i].0))
            }
            TExprKind::Record(fields) => {
                let t = self.fresh();
                let Ty::Record(r) = self.m.peel(&e.ty) else { unreachable!() };
                let mut s = format!("({{ {} {t}; ", self.ty(&e.ty));
                for (x, (f, _)) in fields.iter().zip(self.m.records[r].fields.clone()) {
                    s += &format!("{t}.f_{} = {}; ", cname(&f), self.expr(x));
                }
                s + &format!("{t}; }})")
            }
            TExprKind::Update(b, upd) => {
                let t = self.fresh();
                let Ty::Record(r) = self.m.peel(&e.ty) else { unreachable!() };
                let mut s = format!("({{ {} {t} = {}; ", self.ty(&e.ty), self.expr(b));
                for (i, x) in upd {
                    s += &format!("{t}.f_{} = {}; ", cname(&self.m.records[r].fields[*i].0), self.expr(x));
                }
                s + &format!("{t}; }})")
            }
            TExprKind::Ctor(vi, args) => {
                let t = self.fresh();
                let (vname, fields) = self.m.variants(&e.ty)[*vi].clone();
                let mut s = format!("({{ {} {t}; {t}.tag = {vi}; ", self.ty(&e.ty));
                for (x, (f, _)) in args.iter().zip(fields) {
                    s += &format!("{t}.u.v_{vname}.f_{} = {}; ", cname(&f), self.expr(x));
                }
                s + &format!("{t}; }})")
            }
            TExprKind::Print(args) => {
                let mut s = String::from("({ ");
                for (i, a) in args.iter().enumerate() {
                    if i > 0 {
                        s += "putchar(' '); ";
                    }
                    let x = self.expr(a);
                    s += &match self.m.peel(&a.ty) {
                        Ty::Int => format!("as_print_int({x}); "),
                        Ty::Bool => format!("as_print_bool({x}); "),
                        _ => format!("fputs({x}, stdout); "),
                    };
                }
                s + "putchar('\\n'); ((as_unit)0); })"
            }
            TExprKind::Call(fi, args) => {
                let m = self.m;
                let callee = &m.funcs[*fi];
                let mut s = String::from("({ ");
                let mut temps = vec![];
                for (a, p) in args.iter().zip(callee.params.clone()) {
                    let t = self.fresh();
                    s += &format!("{} {t} = {}; ", self.ty(&self.m.locals[p as usize].ty), self.expr(a));
                    temps.push((p, t));
                }
                let reqs = callee.requires.clone();
                let saved = self.names.clone();
                for (p, t) in &temps {
                    self.names.insert(*p, t.clone());
                }
                for (i, r) in reqs.iter().enumerate() {
                    if !self.proved(SiteKind::Requires(i), e.span) {
                        let c = self.spec(r);
                        s += &format!("if (!({c})) as_fail({}, {}); ", self.at(e.span), c_str(&format!("requires {} of {}", callee.requires_src[i], callee.name)));
                    }
                }
                self.names = saved;
                let args: Vec<String> = temps.iter().map(|(_, t)| t.clone()).collect();
                s + &format!("as_fn_{}({}); }})", cname(&callee.name), args.join(", "))
            }
            TExprKind::Unary(UnOp::Not, x) => format!("(!({}))", self.expr(x)),
            TExprKind::Unary(UnOp::Neg, x) => {
                let xs = self.expr(x);
                if self.proved(SiteKind::Overflow, e.span) { format!("(-({xs}))") } else { format!("as_neg({xs}, {})", self.at(e.span)) }
            }
            TExprKind::Binary(op, l, r) => {
                let (a, b) = (self.expr(l), self.expr(r));
                match op {
                    BinOp::And => format!("(({a}) && ({b}))"),
                    BinOp::Or => format!("(({a}) || ({b}))"),
                    BinOp::Implies => format!("(!({a}) || ({b}))"),
                    BinOp::Add | BinOp::Sub | BinOp::Mul => {
                        if self.proved(SiteKind::Overflow, e.span) {
                            format!("(({a}) {} ({b}))", op.symbol())
                        } else {
                            let f = match op { BinOp::Add => "as_add", BinOp::Sub => "as_sub", _ => "as_mul" };
                            format!("{f}({a}, {b}, {})", self.at(e.span))
                        }
                    }
                    BinOp::Div | BinOp::Rem => {
                        let sym = op.symbol();
                        if self.proved(SiteKind::FastDiv32, e.span) {
                            // Proved to fit in 32 bits and be non-negative: 32-bit division is
                            // several times faster than 64-bit on most CPUs.
                            format!("((int64_t)((uint32_t)({a}) {sym} (uint32_t)({b})))")
                        } else if self.proved(SiteKind::FastDivUnsigned, e.span) {
                            format!("((int64_t)((uint64_t)({a}) {sym} (uint64_t)({b})))")
                        } else if self.proved(SiteKind::DivZero, e.span) && self.proved(SiteKind::DivOverflow, e.span) {
                            format!("(({a}) {sym} ({b}))")
                        } else {
                            format!("{}({a}, {b}, {})", if *op == BinOp::Div { "as_div" } else { "as_rem" }, self.at(e.span))
                        }
                    }
                    _ => format!("(({a}) {} ({b}))", op.symbol()),
                }
            }
            TExprKind::Is(x, p) => {
                let v = self.fresh();
                let xs = self.expr(x);
                let test = self.pat(p, &x.ty, &v);
                format!("({{ {} {v} = {xs}; {test}; }})", self.ty(&x.ty))
            }
            TExprKind::Coerce(x, a) => {
                let xs = self.expr(x);
                if self.proved(SiteKind::Refine, x.span) {
                    return xs;
                }
                let v = self.fresh();
                let check = self.refine_check(*a, &v);
                let what = c_str(&format!("value is not a valid {}", self.m.aliases[*a].name));
                format!("({{ {} {v} = {xs}; if (!({check})) as_fail({}, {what}); {v}; }})", self.ty(&e.ty), self.at(x.span))
            }
            TExprKind::If(c, t, f) => {
                let cs = self.expr(c);
                let (ts, fs) = (self.expr(t), self.expr(f));
                if matches!(e.ty, Ty::Unit | Ty::Never) {
                    format!("({{ if ({cs}) {{ (void)({ts}); }} else {{ (void)({fs}); }} ((as_unit)0); }})")
                } else {
                    let r = self.fresh();
                    let assign = |x: &TExpr, s: String| if x.ty == Ty::Never { format!("(void)({s});") } else { format!("{r} = {s};") };
                    format!("({{ {} {r}; if ({cs}) {{ {} }} else {{ {} }} {r}; }})", self.ty(&e.ty), assign(t, ts), assign(f, fs))
                }
            }
            TExprKind::Match(scrut, arms) => {
                let v = self.fresh();
                let r = self.fresh();
                let unit = matches!(e.ty, Ty::Unit | Ty::Never);
                let mut s = format!("({{ {} {v} = {}; ", self.ty(&scrut.ty), self.expr(scrut));
                if !unit {
                    s += &format!("{} {r}; ", self.ty(&e.ty));
                }
                for (i, (p, body)) in arms.iter().enumerate() {
                    let test = self.pat(p, &scrut.ty, &v);
                    let b = self.expr(body);
                    let act = if unit || body.ty == Ty::Never { format!("(void)({b});") } else { format!("{r} = {b};") };
                    s += &format!("{}if ({test}) {{ {act} }} ", if i > 0 { "else " } else { "" });
                }
                s += &format!("else as_fail({}, \"no match arm\"); ", self.at(e.span));
                s + &if unit { "((as_unit)0); })".to_string() } else { format!("{r}; }})") }
            }
            TExprKind::Block(stmts, tail) => {
                let mut s = String::from("({ ");
                for st in stmts {
                    s += &self.stmt(st);
                }
                s + &format!("{}; }})", self.expr(tail))
            }
        }
    }

    /// Turn a proved fact into an optimizer hint. Only plain comparisons of variables, fields
    /// and constants are passed on; anything else is left out, which is always safe.
    fn assume(&mut self, e: &TExpr) -> String {
        let mut parts = vec![];
        self.simple_conjuncts(e, &mut parts);
        parts.iter().map(|c| format!("if (!({c})) __builtin_unreachable();")).collect::<Vec<_>>().join(" ")
    }

    fn assume_refinement(&mut self, a: usize, v: &str) -> String {
        let def = self.m.aliases[a].clone();
        let mut out = String::new();
        if let Ty::Alias(b) = def.base {
            out += &self.assume_refinement(b, v);
        }
        if let Some(pred) = &def.pred {
            self.names.insert(def.it, v.to_string());
            out += &self.assume(pred);
        }
        out
    }

    fn simple_conjuncts(&mut self, e: &TExpr, out: &mut Vec<String>) {
        match &e.kind {
            TExprKind::Binary(BinOp::And, l, r) => {
                self.simple_conjuncts(l, out);
                self.simple_conjuncts(r, out);
            }
            TExprKind::Binary(op @ (BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge | BinOp::Eq | BinOp::Ne), l, r) => {
                if let (Some(a), Some(b)) = (self.atom(l), self.atom(r)) {
                    out.push(format!("{a} {} {b}", op.symbol()));
                }
            }
            _ => {}
        }
    }

    fn atom(&mut self, e: &TExpr) -> Option<String> {
        match &e.kind {
            TExprKind::Int(v) => Some(int_lit(*v)),
            TExprKind::Local(id) if self.m.peel(&e.ty) == Ty::Int => Some(self.local(*id)),
            TExprKind::Field(x, _) if self.m.peel(&e.ty) == Ty::Int && self.atom_path(x) => Some(self.expr(e)),
            _ => None,
        }
    }

    fn atom_path(&self, e: &TExpr) -> bool {
        match &e.kind {
            TExprKind::Local(_) => true,
            TExprKind::Field(x, _) => self.atom_path(x),
            _ => false,
        }
    }

    fn refine_check(&mut self, a: usize, v: &str) -> String {
        let def = self.m.aliases[a].clone();
        let mut parts = vec![];
        if let Ty::Alias(b) = def.base {
            parts.push(self.refine_check(b, v));
        }
        if let Some(pred) = &def.pred {
            self.names.insert(def.it, v.to_string());
            parts.push(self.spec(pred));
        }
        if parts.is_empty() { "1".into() } else { parts.join(" && ") }
    }

    fn stmt(&mut self, s: &TStmt) -> String {
        match s {
            TStmt::Let(id, e) | TStmt::Assign(id, e) => format!("{} = {}; ", self.local(*id), self.expr(e)),
            TStmt::Expr(e) => format!("(void)({}); ", self.expr(e)),
            TStmt::Return(e, span) => {
                let m = self.m;
                let f = &m.funcs[self.cur];
                let v = self.expr(e);
                if f.ret == Ty::Unit {
                    let checks = self.ensures_checks(*span, "0");
                    format!("{{ (void)({v}); {checks} return 0; }} ")
                } else {
                    let checks = self.ensures_checks(*span, "_ret");
                    format!("{{ {} _ret = {v}; {checks} return _ret; }} ", self.ty(&f.ret))
                }
            }
            TStmt::While { cond, invariants, decreases, body, span, .. } => {
                let mut s = String::from("while (1) { ");
                for (i, inv) in invariants.iter().enumerate() {
                    if !(self.proved(SiteKind::InvEntry(i), *span) && self.proved(SiteKind::InvKeep(i), *span)) {
                        let c = self.spec(inv);
                        s += &format!("if (!({c})) as_fail({}, \"loop invariant\"); ", self.at(*span));
                    } else {
                        s += &self.assume(inv);
                        s.push(' ');
                    }
                }
                s += &format!("if (!({})) break; ", self.expr(cond));
                let dec = decreases.as_ref().filter(|_| !(self.proved(SiteKind::DecreasesBound, *span) && self.proved(SiteKind::Decreases, *span)));
                let d0 = self.fresh();
                if let Some(d) = dec {
                    let c = self.spec(d);
                    s += &format!("__int128 {d0} = {c}; if ({d0} < 0) as_fail({}, \"loop measure is negative\"); ", self.at(*span));
                }
                s += &format!("(void)({}); ", self.expr(body));
                if let Some(d) = dec {
                    let c = self.spec(d);
                    s += &format!("if (!(({c}) < {d0})) as_fail({}, \"loop measure did not decrease\"); ", self.at(*span));
                }
                s + "} "
            }
        }
    }
}

fn int_lit(v: i128) -> String {
    if v == i64::MIN as i128 {
        "INT64_MIN".into()
    } else if v < 0 {
        format!("(-INT64_C({}))", -v)
    } else {
        format!("INT64_C({v})")
    }
}

fn collect_locals(e: &TExpr, out: &mut Vec<LocalId>) {
    let visit_pat = |p: &TPat, out: &mut Vec<LocalId>| p.bindings(out);
    match &e.kind {
        TExprKind::Block(ss, t) => {
            for s in ss {
                match s {
                    TStmt::Let(id, v) => {
                        out.push(*id);
                        collect_locals(v, out);
                    }
                    TStmt::Assign(_, v) | TStmt::Expr(v) | TStmt::Return(v, _) => collect_locals(v, out),
                    TStmt::While { cond, body, invariants, decreases, .. } => {
                        collect_locals(cond, out);
                        collect_locals(body, out);
                        invariants.iter().chain(decreases.iter()).for_each(|x| collect_locals(x, out));
                    }
                }
            }
            collect_locals(t, out);
        }
        TExprKind::Is(x, p) => {
            visit_pat(p, out);
            collect_locals(x, out);
        }
        TExprKind::Match(s, arms) => {
            collect_locals(s, out);
            for (p, b) in arms {
                visit_pat(p, out);
                collect_locals(b, out);
            }
        }
        TExprKind::If(c, t, f) => {
            collect_locals(c, out);
            collect_locals(t, out);
            collect_locals(f, out);
        }
        TExprKind::Field(x, _) | TExprKind::Unary(_, x) | TExprKind::Coerce(x, _) => collect_locals(x, out),
        TExprKind::Binary(_, a, b) => {
            collect_locals(a, out);
            collect_locals(b, out);
        }
        TExprKind::Record(xs) | TExprKind::Ctor(_, xs) | TExprKind::Call(_, xs) | TExprKind::Print(xs) => xs.iter().for_each(|x| collect_locals(x, out)),
        TExprKind::Update(b, fs) => {
            collect_locals(b, out);
            fs.iter().for_each(|(_, x)| collect_locals(x, out));
        }
        _ => {}
    }
}

fn calls_in(e: &TExpr, out: &mut Vec<usize>) {
    match &e.kind {
        TExprKind::Call(fi, xs) => {
            out.push(*fi);
            xs.iter().for_each(|x| calls_in(x, out));
        }
        TExprKind::Block(ss, t) => {
            for s in ss {
                match s {
                    TStmt::Let(_, v) | TStmt::Assign(_, v) | TStmt::Expr(v) | TStmt::Return(v, _) => calls_in(v, out),
                    TStmt::While { cond, body, .. } => {
                        calls_in(cond, out);
                        calls_in(body, out);
                    }
                }
            }
            calls_in(t, out);
        }
        TExprKind::If(c, t, f) => [c, t, f].iter().for_each(|x| calls_in(x, out)),
        TExprKind::Match(s, arms) => {
            calls_in(s, out);
            arms.iter().for_each(|(_, b)| calls_in(b, out));
        }
        TExprKind::Field(x, _) | TExprKind::Unary(_, x) | TExprKind::Coerce(x, _) | TExprKind::Is(x, _) => calls_in(x, out),
        TExprKind::Binary(_, a, b) => {
            calls_in(a, out);
            calls_in(b, out);
        }
        TExprKind::Record(xs) | TExprKind::Ctor(_, xs) | TExprKind::Print(xs) => xs.iter().for_each(|x| calls_in(x, out)),
        TExprKind::Update(b, fs) => {
            calls_in(b, out);
            fs.iter().for_each(|(_, x)| calls_in(x, out));
        }
        _ => {}
    }
}
