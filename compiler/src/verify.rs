//! Verifier. Walks each function symbolically, turns every safety check and contract into an
//! SMT-LIB query, and asks Z3. Each check site gets a verdict: proved, refuted (with a
//! counterexample) or unknown (the backend then keeps a run-time check).
//!
//! Contracts are modular: a call is checked against the callee's `requires` and then assumed
//! to satisfy its `ensures`. Contract expressions use mathematical integers; code uses 64-bit
//! integers with an overflow check on every operation.

use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::process::{Command, Stdio};

use crate::ast::{BinOp, UnOp};
use crate::diag::{Diagnostic, Source, Span};
use crate::tir::*;

const MIN: &str = "(- 9223372036854775808)";
const MAX: &str = "9223372036854775807";
/// Largest array length the compiler admits (2^40 elements).
pub const MAX_LEN: &str = "1099511627776";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SiteKind {
    Overflow,
    DivZero,
    DivOverflow,
    Refine,
    Requires(usize),
    Ensures(usize),
    InvEntry(usize),
    InvKeep(usize),
    DecreasesBound,
    Decreases,
    Bounds,
    Length,
    /// Optimisation facts, never errors: both operands of a division are non-negative and
    /// fit in 32 bits, or the dividend is non-negative and the divisor positive.
    FastDiv32,
    FastDivUnsigned,
}

impl SiteKind {
    pub fn is_hint(self) -> bool {
        matches!(self, SiteKind::FastDiv32 | SiteKind::FastDivUnsigned)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Site {
    pub kind: SiteKind,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Verdict {
    Proved,
    Refuted(Vec<(String, String)>),
    Unknown,
}

#[derive(Default)]
pub struct Report {
    pub verdicts: HashMap<Site, Verdict>,
    pub diags: Vec<Diagnostic>,
    pub proved: usize,
    pub runtime: usize,
    pub refuted: usize,
    pub smt: String,
    pub solver_missing: bool,
    /// Invariants the compiler inferred, per loop.
    pub inferred: Vec<(Span, Vec<String>)>,
    failed_inferred: Vec<(Span, String)>,
}

pub struct Options {
    pub z3: String,
    pub timeout_ms: u32,
    /// Infer loop invariants (on by default).
    pub infer: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { z3: std::env::var("ASLANG_Z3").unwrap_or_else(|_| "z3".into()), timeout_ms: 2000, infer: true }
    }
}

#[derive(Clone)]
struct St {
    pc: String,
    env: HashMap<LocalId, String>,
}

enum Event {
    Fact(String),
    Check { site: Site, pc: String, goal: String, show: Vec<(String, String)>, what: String, fix: Option<String>, notes: Vec<String> },
}

struct Fv<'a> {
    m: &'a Module,
    src: &'a Source,
    f: &'a Func,
    decls: Vec<String>,
    events: Vec<Event>,
    fresh: usize,
    spec: bool,
    params_show: Vec<(String, String)>,
    /// While inferring invariants, symbolic runs must not report obligations.
    sandbox: bool,
    infer: bool,
    z3: String,
    timeout_ms: u32,
    preamble: String,
    consts: Vec<i128>,
    inferred: Vec<(Span, Vec<String>)>,
    /// Obligations of inferred invariants: proved like any other, but not counted.
    inferred_sites: HashMap<Site, String>,
    /// Survivors of earlier inference runs per loop, used to warm-start exploratory runs.
    warm: HashMap<Span, Vec<crate::infer::Cand>>,
    retry: bool,
    /// Inferred invariants that failed a final proof; not offered again.
    skip: HashSet<(Span, String)>,
    started: std::time::Instant,
    budget_ms: u64,
}

pub fn sort(m: &Module, t: &Ty) -> String {
    match m.erase(t) {
        Ty::Int | Ty::Never | Ty::Unit => "Int".into(),
        Ty::Bool => "Bool".into(),
        Ty::Str => "String".into(),
        t => mangle(m, &t),
    }
}

pub fn mangle(m: &Module, t: &Ty) -> String {
    match m.erase(t) {
        Ty::Int | Ty::Never | Ty::Unit => "Int".into(),
        Ty::Bool => "Bool".into(),
        Ty::Str => "Str".into(),
        Ty::Record(r) => format!("R_{}", m.records[r].name),
        Ty::Enum(e) => format!("E_{}", m.enums[e].name),
        Ty::Option(x) => format!("Opt_{}", mangle(m, &x)),
        Ty::Result(a, b) => format!("Res_{}_{}", mangle(m, &a), mangle(m, &b)),
        Ty::Array(x) => format!("Arr_{}", mangle(m, &x)),
        Ty::Alias(_) => unreachable!(),
    }
}

/// SMT names of a type's constructors and their fields.
pub fn ctors(m: &Module, t: &Ty) -> Vec<(String, Vec<String>)> {
    let t = m.erase(t);
    let name = mangle(m, &t);
    match &t {
        Ty::Record(r) => vec![(format!("mk_{name}"), m.records[*r].fields.iter().map(|(f, _)| format!("{name}__{f}")).collect())],
        Ty::Array(_) => vec![(format!("mk_{name}"), vec![format!("{name}__data"), format!("{name}__len")])],
        _ => m.variants(&t).iter().map(|(v, fs)| (format!("{name}__{v}"), fs.iter().map(|(f, _)| format!("{name}__{v}__{f}")).collect())).collect(),
    }
}

pub fn datatypes(m: &Module) -> Vec<Ty> {
    fn walk(m: &Module, t: &Ty, seen: &mut Vec<Ty>, set: &mut HashSet<Ty>) {
        let t = m.erase(t);
        match &t {
            Ty::Record(r) => {
                for (_, f) in &m.records[*r].fields {
                    walk(m, f, seen, set);
                }
            }
            Ty::Enum(e) => {
                for v in &m.enums[*e].variants {
                    for (_, f) in &v.fields {
                        walk(m, f, seen, set);
                    }
                }
            }
            Ty::Option(x) | Ty::Array(x) => walk(m, x, seen, set),
            Ty::Result(a, b) => {
                walk(m, a, seen, set);
                walk(m, b, seen, set);
            }
            _ => return,
        }
        if set.insert(t.clone()) {
            seen.push(t);
        }
    }
    let (mut seen, mut set) = (vec![], HashSet::new());
    for r in 0..m.records.len() {
        walk(m, &Ty::Record(r), &mut seen, &mut set);
    }
    for e in 0..m.enums.len() {
        walk(m, &Ty::Enum(e), &mut seen, &mut set);
    }
    for l in &m.locals {
        walk(m, &l.ty, &mut seen, &mut set);
    }
    seen
}

pub fn preamble(m: &Module) -> String {
    let mut s = String::from("(set-logic ALL)\n(set-option :produce-models true)\n");
    s += "(define-fun tdiv ((a Int) (b Int)) Int (ite (>= a 0) (ite (>= b 0) (div a b) (- (div a (- b)))) (ite (>= b 0) (- (div (- a) b)) (div (- a) (- b)))))\n";
    s += "(define-fun trem ((a Int) (b Int)) Int (- a (* b (tdiv a b))))\n";
    let dts = datatypes(m);
    if !dts.is_empty() {
        let names: Vec<String> = dts.iter().map(|t| format!("({} 0)", mangle(m, t))).collect();
        let bodies: Vec<String> = dts
            .iter()
            .map(|t| {
                if let Ty::Array(x) = m.erase(t) {
                    let n = mangle(m, t);
                    return format!("((mk_{n} ({n}__data (Array Int {})) ({n}__len Int)))", sort(m, &x));
                }
                let fields_of = |i: usize| -> Vec<Ty> {
                    match m.erase(t) {
                        Ty::Record(r) => m.records[r].fields.iter().map(|f| f.1.clone()).collect(),
                        _ => m.variants(t)[i].1.iter().map(|f| f.1.clone()).collect(),
                    }
                };
                let cs: Vec<String> = ctors(m, t)
                    .iter()
                    .enumerate()
                    .map(|(i, (c, fs))| {
                        let tys = fields_of(i);
                        let fs: Vec<String> = fs.iter().zip(tys).map(|(f, ft)| format!("({f} {})", sort(m, &ft))).collect();
                        format!("({c}{}{})", if fs.is_empty() { "" } else { " " }, fs.join(" "))
                    })
                    .collect();
                format!("({})", cs.join(" "))
            })
            .collect();
        s += &format!("(declare-datatypes ({}) ({}))\n", names.join(" "), bodies.join(" "));
    }
    s
}

pub fn verify(m: &Module, src: &Source, opts: &Options) -> Report {
    let mut rep = Report::default();
    let pre = preamble(m);
    rep.smt += &pre;
    for f in &m.funcs {
        let mut skip: HashSet<(Span, String)> = HashSet::new();
        let mut warm = HashMap::new();
        for attempt in 0..4 {
            let mut fv = Fv::new(m, src, f, opts, &pre);
            fv.infer = opts.infer && attempt < 3;
            fv.skip = skip.clone();
            fv.retry = attempt > 0;
            fv.warm = std::mem::take(&mut warm);
            let script = fv.run();
            let full = format!("{pre}(set-option :timeout {})\n{script}", opts.timeout_ms);
            let t0 = std::time::Instant::now();
            let answers = match run_z3(&opts.z3, &full) {
                Some(a) => a,
                None => {
                    rep.solver_missing = true;
                    vec![]
                }
            };
            let mut one = Report::default();
            fv.collect(answers, &mut one);
            if std::env::var("ASLANG_TRACE").is_ok() {
                let n: usize = fv.inferred.iter().map(|(_, v)| v.len()).sum();
                eprintln!("[final] {} attempt {attempt}: {n} inferred, {} failed inferred, proved {} unknown {} refuted {} in {} ms; failed: {:?}", f.name, one.failed_inferred.len(), one.proved, one.runtime, one.refuted, t0.elapsed().as_millis(), one.failed_inferred.iter().map(|(_, t)| t.clone()).collect::<Vec<_>>());
            }
            warm = std::mem::take(&mut fv.warm);
            if !one.failed_inferred.is_empty() && attempt < 3 {
                // Some inferred invariants did not re-prove (usually a solver time-out). Drop
                // just those and verify again; the last attempt uses no inference at all.
                skip.extend(one.failed_inferred);
                continue;
            }
            rep.smt += &format!("; ---- {} ----\n{script}", f.name);
            rep.inferred.extend(fv.inferred.drain(..));
            rep.verdicts.extend(one.verdicts);
            rep.diags.extend(one.diags);
            rep.proved += one.proved;
            rep.runtime += one.runtime;
            rep.refuted += one.refuted;
            break;
        }
    }
    if rep.solver_missing {
        rep.diags.push(Diagnostic::warning("W0251", Span::default(), format!("the SMT solver `{}` was not found, so nothing was proved", opts.z3)).with_fix("install Z3 or set ASLANG_Z3 to its path; until then every check is kept at run time"));
    }
    rep
}

fn run_z3(z3: &str, script: &str) -> Option<Vec<SExp>> {
    let t0 = std::time::Instant::now();
    if let Ok(dir) = std::env::var("ASLANG_DUMP") {
        static N: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let _ = std::fs::write(format!("{dir}/q{n}.smt2"), script);
    }
    let r = run_z3_inner(z3, script);
    if std::env::var("ASLANG_TRACE").is_ok() {
        eprintln!("[z3] {} bytes, {} ms, ok={}", script.len(), t0.elapsed().as_millis(), r.is_some());
    }
    r
}

fn run_z3_inner(z3: &str, script: &str) -> Option<Vec<SExp>> {
    let mut child = Command::new(z3).args(["-in", "-smt2"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().ok()?;
    child.stdin.take()?.write_all(script.as_bytes()).ok()?;
    let out = child.wait_with_output().ok()?;
    Some(parse_sexps(&String::from_utf8_lossy(&out.stdout)))
}

impl<'a> Fv<'a> {
    fn new(m: &'a Module, src: &'a Source, f: &'a Func, opts: &Options, pre: &str) -> Fv<'a> {
        Fv {
            m,
            src,
            f,
            decls: vec![],
            events: vec![],
            fresh: 0,
            spec: false,
            params_show: vec![],
            sandbox: false,
            infer: opts.infer,
            z3: opts.z3.clone(),
            timeout_ms: opts.timeout_ms,
            preamble: pre.to_string(),
            consts: crate::infer::constants(m, f),
            inferred: vec![],
            inferred_sites: HashMap::new(),
            skip: HashSet::new(),
            warm: HashMap::new(),
            retry: false,
            started: std::time::Instant::now(),
            budget_ms: 10_000,
        }
    }

    fn fresh(&mut self, base: &str, t: &Ty) -> String {
        let name = format!("{}_{}", base.replace(|c: char| !c.is_ascii_alphanumeric(), "_"), self.fresh);
        self.fresh += 1;
        self.decls.push(format!("(declare-const {name} {})", sort(self.m, t)));
        name
    }

    fn fact(&mut self, st: &St, f: String) {
        if f == "true" {
            return;
        }
        let g = if st.pc == "true" { f } else { format!("(=> {} {f})", st.pc) };
        self.events.push(Event::Fact(g));
    }

    fn check(&mut self, st: &St, site: Site, goal: String, what: String, fix: Option<String>, extra: Vec<(String, String)>) {
        if self.spec || self.sandbox {
            return;
        }
        let mut show = self.params_show.clone();
        show.extend(extra);
        self.events.push(Event::Check { site, pc: st.pc.clone(), goal, show, what, fix, notes: vec![] });
    }

    fn fresh_array(&mut self, t: &Ty) -> String {
        let Ty::Array(elem) = self.m.erase(t) else { unreachable!() };
        let name = format!("arr_{}", self.fresh);
        self.fresh += 1;
        self.decls.push(format!("(declare-const {name} (Array Int {}))", sort(self.m, &elem)));
        name
    }

    /// Bounds on the integer parts of the parameters, used to ask for a readable counterexample.
    fn small(&self) -> String {
        fn leaves(m: &Module, t: &Ty, v: &str, depth: usize, out: &mut Vec<String>) {
            if depth > 3 {
                return;
            }
            match m.peel(t) {
                Ty::Int => out.push(format!("(<= (- 1000) {v} 1000)")),
                Ty::Record(r) => {
                    let (_, sels) = ctors(m, &Ty::Record(r)).remove(0);
                    for ((_, ft), sel) in m.records[r].fields.iter().zip(sels) {
                        leaves(m, ft, &format!("({sel} {v})"), depth + 1, out);
                    }
                }
                _ => {}
            }
        }
        let mut out = vec![];
        for &p in &self.f.params {
            let l = &self.m.locals[p as usize];
            if let Some((_, v)) = self.params_show.iter().find(|(n, _)| *n == l.name) {
                leaves(self.m, &l.ty, v, 0, &mut out);
            }
        }
        and(out)
    }

    fn text(&self, s: Span) -> String {
        self.src.slice(s).split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Well-formedness of a value of type `t`: integer range, refinements, nested fields.
    fn wf(&mut self, t: &Ty, v: &str) -> String {
        let m = self.m;
        let parts: Vec<String> = match t {
            Ty::Int => vec![format!("(<= {MIN} {v})"), format!("(<= {v} {MAX})")],
            Ty::Alias(a) => {
                let def = &m.aliases[*a];
                let mut p = vec![self.wf(&def.base.clone(), v)];
                if let Some(pred) = def.pred.clone() {
                    p.push(self.pred_at(&pred, def.it, v));
                }
                p
            }
            Ty::Record(r) => {
                let (_, sels) = ctors(m, t).remove(0);
                m.records[*r].fields.iter().zip(sels).map(|((_, ft), s)| self.wf(&ft.clone(), &format!("({s} {v})"))).collect()
            }
            Ty::Array(elem) => {
                let (_, sels) = ctors(m, t).remove(0);
                let len = format!("({} {v})", sels[1]);
                let mut p = vec![format!("(<= 0 {len})"), format!("(<= {len} {MAX_LEN})")];
                // Plain integers are not constrained element by element; that only makes proofs
                // harder, never unsound, and keeps quantifiers out of most queries.
                if **elem != Ty::Int {
                    let k = format!("qk_{}", self.fresh);
                    self.fresh += 1;
                    let ew = self.wf(&elem.clone(), &format!("(select ({} {v}) {k})", sels[0]));
                    if ew != "true" {
                        p.push(format!("(forall (({k} Int)) (=> (and (<= 0 {k}) (< {k} {len})) {ew}))"));
                    }
                }
                p
            }
            Ty::Enum(_) | Ty::Option(_) | Ty::Result(..) => {
                let cs = ctors(m, t);
                m.variants(t)
                    .iter()
                    .zip(cs)
                    .filter(|((_, fs), _)| !fs.is_empty())
                    .map(|((_, fs), (c, sels))| {
                        let inner: Vec<String> = fs.iter().zip(sels).map(|((_, ft), s)| self.wf(&ft.clone(), &format!("({s} {v})"))).collect();
                        format!("(=> ((_ is {c}) {v}) {})", and(inner))
                    })
                    .collect()
            }
            _ => vec![],
        };
        and(parts)
    }

    fn pred_at(&mut self, pred: &TExpr, it: LocalId, v: &str) -> String {
        let st = St { pc: "true".into(), env: HashMap::from([(it, v.to_string())]) };
        let saved = self.spec;
        self.spec = true;
        let (_, t) = self.expr(pred, st);
        self.spec = saved;
        t
    }

    fn run(&mut self) -> String {
        let f = self.f;
        let mut env = HashMap::new();
        for &p in &f.params {
            let l = &self.m.locals[p as usize];
            let v = self.fresh(&l.name, &l.ty.clone());
            self.params_show.push((l.name.clone(), v.clone()));
            env.insert(p, v);
        }
        let mut st = St { pc: "true".into(), env };
        for &p in &f.params {
            let ty = self.m.locals[p as usize].ty.clone();
            let v = st.env[&p].clone();
            let w = self.wf(&ty, &v);
            self.fact(&st, w);
        }
        // An unsatisfiable precondition would make every proof below vacuous.
        let mut reqs = vec![];
        self.spec = true;
        for r in &f.requires {
            let (_, t) = self.expr(r, st.clone());
            reqs.push(t);
        }
        self.spec = false;
        if !f.requires.is_empty() {
            let site = Site { kind: SiteKind::Requires(usize::MAX), span: f.sig_span };
            self.events.push(Event::Check { site, pc: "true".into(), goal: format!("(not {})", and(reqs.clone())), show: vec![], what: String::new(), fix: None, notes: vec![] });
        }
        for r in reqs {
            self.fact(&st, r);
        }
        let body = &f.body;
        let (end, val) = self.expr(body, st.clone());
        if let Some(end) = end {
            let span = match &body.kind {
                TExprKind::Block(_, tail) => tail.span,
                _ => body.span,
            };
            self.ensure(&end, &val, span);
        }
        st.pc = "true".into();
        let mut out = String::new();
        for d in &self.decls {
            out += d;
            out.push('\n');
        }
        for e in &self.events {
            match e {
                Event::Fact(f) => out += &format!("(assert {f})\n"),
                Event::Check { pc, goal, show, .. } => {
                    out += &format!("(push 1)\n(assert {pc})\n(assert (not {goal}))\n(check-sat)\n");
                    let vars: Vec<&str> = show.iter().map(|(_, v)| v.as_str()).collect();
                    let get = if vars.is_empty() { "(get-value (0))\n".to_string() } else { format!("(get-value ({}))\n", vars.join(" ")) };
                    out += &get;
                    out += &format!("(assert {})\n(check-sat)\n{get}", self.small());
                    out += "(pop 1)\n";
                }
            }
        }
        out
    }

    fn ensure(&mut self, st: &St, val: &str, span: Span) {
        let f = self.f;
        let mut st2 = st.clone();
        st2.env.insert(f.result, val.to_string());
        for (i, e) in f.ensures.iter().enumerate() {
            let saved = self.spec;
            self.spec = true;
            let (_, g) = self.expr(e, st2.clone());
            self.spec = saved;
            let what = format!("`{}` may not return a value satisfying `ensures {}`", f.name, f.ensures_src[i]);
            let extra = if f.ret == Ty::Unit { vec![] } else { vec![("result".to_string(), val.to_string())] };
            self.check(st, Site { kind: SiteKind::Ensures(i), span }, g, what, Some("fix the code, or state a weaker `ensures` only if that is the real intent (a pinned contract cannot be weakened silently)".into()), extra);
        }
    }

    fn ranged(&mut self, st: &St, t: &str, span: Span, op: &str) {
        let goal = format!("(and (<= {MIN} {t}) (<= {t} {MAX}))");
        let text = self.text(span);
        self.check(st, Site { kind: SiteKind::Overflow, span }, goal, format!("`{text}` can overflow a 64-bit int"), Some(format!("bound the operands with a `requires`, e.g. `requires {text} <= int.max`, or check them before the `{op}`")), vec![]);
    }

    fn expr(&mut self, e: &TExpr, st: St) -> (Option<St>, String) {
        let m = self.m;
        match &e.kind {
            TExprKind::Int(v) => (Some(st), if *v < 0 { format!("(- {})", -v) } else { v.to_string() }),
            TExprKind::Bool(b) => (Some(st), b.to_string()),
            TExprKind::Str(_) | TExprKind::Unit => (Some(st), "0".into()),
            TExprKind::Local(id) => {
                let v = st.env.get(id).cloned().unwrap_or_else(|| "0".into());
                (Some(st), v)
            }
            TExprKind::Field(x, i) => {
                let (st, xv) = self.expr(x, st);
                let (_, sels) = ctors(m, &x.ty).remove(0);
                (st, format!("({} {xv})", sels[*i]))
            }
            TExprKind::Record(fields) => {
                let (st, vs) = self.exprs(fields, st);
                let (c, _) = ctors(m, &e.ty).remove(0);
                (st, format!("({c} {})", vs.join(" ")))
            }
            TExprKind::Update(base, upd) => {
                let (st, bv) = self.expr(base, st);
                let Some(mut st) = st else { return (None, "0".into()) };
                let (c, sels) = ctors(m, &e.ty).remove(0);
                let mut vals: Vec<String> = sels.iter().map(|s| format!("({s} {bv})")).collect();
                for (i, x) in upd {
                    let (s2, v) = self.expr(x, st);
                    let Some(s2) = s2 else { return (None, "0".into()) };
                    st = s2;
                    vals[*i] = v;
                }
                (Some(st), format!("({c} {})", vals.join(" ")))
            }
            TExprKind::Ctor(vi, args) => {
                let (st, vs) = self.exprs(args, st);
                let (c, _) = ctors(m, &e.ty).remove(*vi);
                (st, if vs.is_empty() { c } else { format!("({c} {})", vs.join(" ")) })
            }
            TExprKind::Print(args) => {
                let (st, _) = self.exprs(args, st);
                (st, "0".into())
            }
            TExprKind::Call(fi, args) => {
                let (st, vs) = self.exprs(args, st);
                let Some(st) = st else { return (None, "0".into()) };
                let callee = &m.funcs[*fi];
                let mut cenv: HashMap<LocalId, String> = callee.params.iter().copied().zip(vs.iter().cloned()).collect();
                let arg_show: Vec<(String, String)> = callee.params.iter().zip(&vs).map(|(p, v)| (format!("{}.{}", callee.name, m.locals[*p as usize].name), v.clone())).collect();
                for (i, r) in callee.requires.iter().enumerate() {
                    let saved = self.spec;
                    self.spec = true;
                    let (_, g) = self.expr(r, St { pc: "true".into(), env: cenv.clone() });
                    self.spec = saved;
                    let what = format!("this call may break `requires {}` of `{}`", callee.requires_src[i], callee.name);
                    self.check(&st, Site { kind: SiteKind::Requires(i), span: e.span }, g, what, Some(format!("check `{}` before calling, or add it to the `requires` of `{}`", callee.requires_src[i], self.f.name)), arg_show.clone());
                }
                let r = self.fresh(&format!("{}_ret", callee.name), &callee.ret);
                let w = self.wf(&callee.ret.clone(), &r);
                self.fact(&st, w);
                cenv.insert(callee.result, r.clone());
                for en in &callee.ensures {
                    let saved = self.spec;
                    self.spec = true;
                    let (_, g) = self.expr(en, St { pc: "true".into(), env: cenv.clone() });
                    self.spec = saved;
                    self.fact(&st, g);
                }
                (Some(st), r)
            }
            TExprKind::Unary(op, x) => {
                let (st, xv) = self.expr(x, st);
                let Some(st) = st else { return (None, "0".into()) };
                match op {
                    UnOp::Not => (Some(st), format!("(not {xv})")),
                    UnOp::Neg => {
                        let t = format!("(- {xv})");
                        if !self.spec {
                            self.ranged(&st, &t, e.span, "-");
                        }
                        (Some(st), t)
                    }
                }
            }
            TExprKind::Binary(op, l, r) => {
                let (st, lv) = self.expr(l, st);
                let Some(st) = st else { return (None, "0".into()) };
                if matches!(op, BinOp::And | BinOp::Or | BinOp::Implies) {
                    let guard = if *op == BinOp::Or { format!("(not {lv})") } else { lv.clone() };
                    let inner = St { pc: and(vec![st.pc.clone(), guard]), env: st.env.clone() };
                    let (after, rv) = self.expr(r, inner);
                    let mut out = St { pc: st.pc.clone(), env: after.map(|a| a.env).unwrap_or(st.env) };
                    out.pc = st.pc.clone();
                    let t = match op {
                        BinOp::And => format!("(and {lv} {rv})"),
                        BinOp::Or => format!("(or {lv} {rv})"),
                        _ => format!("(=> {lv} {rv})"),
                    };
                    return (Some(out), t);
                }
                let (st, rv) = self.expr(r, st);
                let Some(st) = st else { return (None, "0".into()) };
                let t = match op {
                    BinOp::Add => format!("(+ {lv} {rv})"),
                    BinOp::Sub => format!("(- {lv} {rv})"),
                    BinOp::Mul => format!("(* {lv} {rv})"),
                    BinOp::Div => format!("(tdiv {lv} {rv})"),
                    BinOp::Rem => format!("(trem {lv} {rv})"),
                    BinOp::Eq => format!("(= {lv} {rv})"),
                    BinOp::Ne => format!("(not (= {lv} {rv}))"),
                    BinOp::Lt => format!("(< {lv} {rv})"),
                    BinOp::Le => format!("(<= {lv} {rv})"),
                    BinOp::Gt => format!("(> {lv} {rv})"),
                    BinOp::Ge => format!("(>= {lv} {rv})"),
                    _ => unreachable!(),
                };
                if !self.spec {
                    match op {
                        BinOp::Add | BinOp::Sub | BinOp::Mul => self.ranged(&st, &t, e.span, op.symbol()),
                        BinOp::Div | BinOp::Rem => {
                            let rt = self.text(r.span);
                            self.check(&st, Site { kind: SiteKind::DivZero, span: e.span }, format!("(not (= {rv} 0))"), format!("`{rt}` can be zero here"), Some(format!("add `requires {rt} != 0`, or handle zero before dividing")), vec![]);
                            self.check(&st, Site { kind: SiteKind::DivOverflow, span: e.span }, format!("(not (and (= {lv} {MIN}) (= {rv} (- 1))))"), format!("`{}` overflows when dividing int.min by -1", self.text(e.span)), Some("exclude `int.min` with a `requires`".into()), vec![]);
                            self.check(&st, Site { kind: SiteKind::FastDiv32, span: e.span }, format!("(and (<= 0 {lv} 2147483647) (< 0 {rv} 2147483648))"), String::new(), None, vec![]);
                            self.check(&st, Site { kind: SiteKind::FastDivUnsigned, span: e.span }, format!("(and (<= 0 {lv}) (< 0 {rv}))"), String::new(), None, vec![]);
                        }
                        _ => {}
                    }
                }
                (Some(st), t)
            }
            TExprKind::Is(x, p) => {
                let (st, xv) = self.expr(x, st);
                let Some(mut st) = st else { return (None, "0".into()) };
                let t = self.pattern(p, &x.ty, &xv, &mut st);
                (Some(st), t)
            }
            TExprKind::Coerce(x, a) => {
                let (st, xv) = self.expr(x, st);
                let Some(st) = st else { return (None, "0".into()) };
                if !self.spec {
                    let g = self.wf(&Ty::Alias(*a), &xv);
                    let def = &m.aliases[*a];
                    let what = format!("this value may not be a valid `{}` ({})", def.name, def.pred_src.clone().unwrap_or_default());
                    let fix = format!("make sure `{}` holds here, e.g. with an `if` or a `requires`", replace_word(&def.pred_src.clone().unwrap_or_default(), "it", &self.text(x.span)));
                    self.check(&st, Site { kind: SiteKind::Refine, span: x.span }, g, what, Some(fix), vec![("value".into(), xv.clone())]);
                }
                (Some(st), xv)
            }
            TExprKind::If(c, t, f) => {
                let (st, cv) = self.expr(c, st);
                let Some(st) = st else { return (None, "0".into()) };
                let ts = St { pc: and(vec![st.pc.clone(), cv.clone()]), env: st.env.clone() };
                let fs = St { pc: and(vec![st.pc.clone(), format!("(not {cv})")]), env: st.env.clone() };
                let (a, av) = self.expr(t, ts);
                let (b, bv) = self.expr(f, fs);
                merge(&st, &cv, a, av, b, bv)
            }
            TExprKind::Match(s, arms) => {
                let (st, sv) = self.expr(s, st);
                let Some(st) = st else { return (None, "0".into()) };
                let mut remaining = st.pc.clone();
                let mut outs = vec![];
                for (p, body) in arms {
                    let mut ast = St { pc: remaining.clone(), env: st.env.clone() };
                    let test = self.pattern(p, &s.ty, &sv, &mut ast);
                    ast.pc = and(vec![remaining.clone(), test.clone()]);
                    let (o, v) = self.expr(body, ast);
                    outs.push((test.clone(), o, v));
                    remaining = and(vec![remaining, format!("(not {test})")]);
                }
                let mut acc: (Option<St>, String) = (None, "0".into());
                for (test, o, v) in outs.into_iter().rev() {
                    let (prev, pv) = acc;
                    acc = if prev.is_none() { (o, v) } else { merge(&st, &test, o, v, prev, pv) };
                }
                acc
            }
            TExprKind::Index(a, i) => {
                let (st, av) = self.expr(a, st);
                let Some(st) = st else { return (None, "0".into()) };
                let (st, iv) = self.expr(i, st);
                let Some(st) = st else { return (None, "0".into()) };
                let (_, sels) = ctors(m, &a.ty).remove(0);
                let len = format!("({} {av})", sels[1]);
                let it = self.text(i.span);
                self.check(&st, Site { kind: SiteKind::Bounds, span: e.span }, format!("(and (<= 0 {iv}) (< {iv} {len}))"), format!("index `{it}` may be out of bounds for `{}`", self.text(a.span)), Some(format!("make sure `0 <= {it} && {it} < {}.len` holds here, e.g. with a loop invariant or a `requires`", self.text(a.span))), vec![("index".into(), iv.clone())]);
                (Some(st), format!("(select ({} {av}) {iv})", sels[0]))
            }
            TExprKind::Len(a) => {
                let (st, av) = self.expr(a, st);
                let (_, sels) = ctors(m, &a.ty).remove(0);
                (st, format!("({} {av})", sels[1]))
            }
            TExprKind::ArrayLit(xs) => {
                let (st, vs) = self.exprs(xs, st);
                let base = self.fresh_array(&e.ty);
                let (c, _) = ctors(m, &e.ty).remove(0);
                let mut data = base;
                for (i, v) in vs.iter().enumerate() {
                    data = format!("(store {data} {i} {v})");
                }
                (st, format!("({c} {data} {})", vs.len()))
            }
            TExprKind::ArrayRepeat(v, n) => {
                let (st, vv) = self.expr(v, st);
                let Some(st) = st else { return (None, "0".into()) };
                let (st, nv) = self.expr(n, st);
                let Some(st) = st else { return (None, "0".into()) };
                let nt = self.text(n.span);
                self.check(&st, Site { kind: SiteKind::Length, span: e.span }, format!("(and (<= 0 {nv}) (<= {nv} {MAX_LEN}))"), format!("the length `{nt}` may be negative or too large"), Some(format!("make sure `0 <= {nt}` holds here")), vec![]);
                let Ty::Array(elem) = m.erase(&e.ty) else { unreachable!() };
                let (c, _) = ctors(m, &e.ty).remove(0);
                (Some(st), format!("({c} ((as const (Array Int {})) {vv}) {nv})", sort(m, &elem)))
            }
            TExprKind::Quant { forall, var, lo, hi, body } => {
                let (_, lv) = self.expr(lo, st.clone());
                let (_, hv) = self.expr(hi, st.clone());
                let k = format!("q_{}", self.fresh);
                self.fresh += 1;
                let mut inner = st.clone();
                inner.env.insert(*var, k.clone());
                let (_, bv) = self.expr(body, inner);
                let range = format!("(and (<= {lv} {k}) (< {k} {hv}))");
                let t = if *forall { format!("(forall (({k} Int)) (=> {range} {bv}))") } else { format!("(exists (({k} Int)) (and {range} {bv}))") };
                (Some(st), t)
            }
            TExprKind::Block(stmts, tail) => {
                let mut cur = st;
                for s in stmts {
                    match self.stmt(s, cur) {
                        Some(n) => cur = n,
                        None => return (None, "0".into()),
                    }
                }
                self.expr(tail, cur)
            }
        }
    }

    fn exprs(&mut self, xs: &[TExpr], st: St) -> (Option<St>, Vec<String>) {
        let mut cur = st;
        let mut vs = vec![];
        for x in xs {
            let (n, v) = self.expr(x, cur);
            match n {
                Some(n) => cur = n,
                None => return (None, vs),
            }
            vs.push(v);
        }
        (Some(cur), vs)
    }

    fn pattern(&mut self, p: &TPat, t: &Ty, v: &str, st: &mut St) -> String {
        match p {
            TPat::Wild => "true".into(),
            TPat::Bind(id) => {
                st.env.insert(*id, v.to_string());
                "true".into()
            }
            TPat::Int(i) => format!("(= {v} {})", if *i < 0 { format!("(- {})", -i) } else { i.to_string() }),
            TPat::Bool(b) => if *b { v.to_string() } else { format!("(not {v})") },
            TPat::Ctor(vi, subs) => {
                let (c, sels) = ctors(self.m, t).remove(*vi);
                let ftys: Vec<Ty> = self.m.variants(t)[*vi].1.iter().map(|f| f.1.clone()).collect();
                let mut parts = vec![format!("((_ is {c}) {v})")];
                for ((sp, s), ft) in subs.iter().zip(sels).zip(ftys) {
                    parts.push(self.pattern(sp, &ft, &format!("({s} {v})"), st));
                }
                and(parts)
            }
        }
    }

    fn stmt(&mut self, s: &TStmt, st: St) -> Option<St> {
        match s {
            TStmt::Let(id, e) | TStmt::Assign(id, e) => {
                let (st, v) = self.expr(e, st);
                let mut st = st?;
                st.env.insert(*id, v);
                Some(st)
            }
            TStmt::Expr(e) => self.expr(e, st).0,
            TStmt::IndexAssign(id, i, v, span) => {
                let (st, iv) = self.expr(i, st);
                let (st, vv) = self.expr(v, st?);
                let mut st = st?;
                let ty = self.m.locals[*id as usize].ty.clone();
                let (c, sels) = ctors(self.m, &ty).remove(0);
                let a = st.env.get(id).cloned().unwrap_or_else(|| "0".into());
                let len = format!("({} {a})", sels[1]);
                let it = self.text(i.span);
                let name = self.m.locals[*id as usize].name.clone();
                self.check(&st, Site { kind: SiteKind::Bounds, span: *span }, format!("(and (<= 0 {iv}) (< {iv} {len}))"), format!("index `{it}` may be out of bounds for `{name}`"), Some(format!("make sure `0 <= {it} && {it} < {name}.len` holds here")), vec![("index".into(), iv.clone())]);
                st.env.insert(*id, format!("({c} (store ({} {a}) {iv} {vv}) {len})", sels[0]));
                Some(st)
            }
            TStmt::Push(id, v, span) => {
                let (st, vv) = self.expr(v, st);
                let mut st = st?;
                let ty = self.m.locals[*id as usize].ty.clone();
                let (c, sels) = ctors(self.m, &ty).remove(0);
                let a = st.env.get(id).cloned().unwrap_or_else(|| "0".into());
                let len = format!("({} {a})", sels[1]);
                self.check(&st, Site { kind: SiteKind::Length, span: *span }, format!("(< {len} {MAX_LEN})"), "the array may already be at the maximum length".into(), None, vec![]);
                st.env.insert(*id, format!("({c} (store ({} {a}) {len} {vv}) (+ {len} 1))", sels[0]));
                Some(st)
            }
            TStmt::Return(e, span) => {
                let (st, v) = self.expr(e, st);
                if let Some(st) = st {
                    self.ensure(&st, &v, *span);
                }
                None
            }
            TStmt::While { cond, invariants, decreases, body, modified, span } => {
                let mut invs: Vec<(TExpr, String)> = invariants.iter().map(|i| (i.clone(), self.text(i.span))).collect();
                if self.infer {
                    let found = self.infer_loop(&st, cond, invariants, modified, body, *span);
                    if !found.is_empty() && !self.sandbox {
                        self.inferred.push((*span, found.iter().map(|c| c.text.clone()).collect()));
                        for (k, c) in found.iter().enumerate() {
                            let i = invs.len() + k;
                            self.inferred_sites.insert(Site { kind: SiteKind::InvEntry(i), span: *span }, c.text.clone());
                            self.inferred_sites.insert(Site { kind: SiteKind::InvKeep(i), span: *span }, c.text.clone());
                        }
                    }
                    invs.extend(found.into_iter().map(|c| (c.e, c.text)));
                }
                let invariants: Vec<TExpr> = invs.iter().map(|(e, _)| e.clone()).collect();
                for (i, (inv, text)) in invs.iter().enumerate() {
                    let g = self.spec_expr(inv, &st);
                    self.check(&st, Site { kind: SiteKind::InvEntry(i), span: *span }, g, format!("the loop invariant `{text}` may not hold when the loop starts"), Some("initialise the variables so it holds, or weaken the invariant".into()), vec![]);
                }
                let head = self.loop_head(&st, modified, body);
                for inv in &invariants {
                    let g = self.spec_expr(inv, &head);
                    self.fact(&head, g);
                }
                let (hs, cv) = self.expr(cond, head.clone());
                let hs = hs?;
                let inside = St { pc: and(vec![hs.pc.clone(), cv.clone()]), env: hs.env.clone() };
                let d0 = decreases.as_ref().map(|d| self.spec_expr(d, &inside));
                if let (Some(d), Some(d0)) = (decreases, &d0) {
                    let text = self.text(d.span);
                    self.check(&inside, Site { kind: SiteKind::DecreasesBound, span: *span }, format!("(>= {d0} 0)"), format!("the loop measure `{text}` may be negative"), Some("choose a measure that stays at or above zero while the loop runs".into()), vec![]);
                }
                let (end, _) = self.expr(body, inside);
                if let Some(end) = end {
                    for (i, (inv, text)) in invs.iter().enumerate() {
                        let g = self.spec_expr(inv, &end);
                        self.check(&end, Site { kind: SiteKind::InvKeep(i), span: *span }, g, format!("the loop body may break the invariant `{text}`"), Some("strengthen the invariant or fix the body".into()), vec![]);
                    }
                    if let (Some(d), Some(d0)) = (decreases, d0) {
                        let d1 = self.spec_expr(d, &end);
                        let text = self.text(d.span);
                        self.check(&end, Site { kind: SiteKind::Decreases, span: *span }, format!("(< {d1} {d0})"), format!("the loop measure `{text}` may not decrease"), Some("make each iteration reduce the measure".into()), vec![]);
                    }
                }
                Some(St { pc: and(vec![hs.pc, format!("(not {cv})")]), env: hs.env })
            }
        }
    }

    /// The state at the top of an arbitrary iteration: variables the loop changes get fresh
    /// values; arrays whose elements are only written keep their length.
    fn loop_head(&mut self, st: &St, modified: &[LocalId], body: &TExpr) -> St {
        let mut head = St { pc: st.pc.clone(), env: st.env.clone() };
        let resized = resized_arrays(body);
        for id in modified {
            let l = &self.m.locals[*id as usize];
            let (name, ty) = (l.name.clone(), l.ty.clone());
            if matches!(self.m.peel(&ty), Ty::Array(_)) && !resized.contains(id) {
                let (c, sels) = ctors(self.m, &ty).remove(0);
                let old = head.env.get(id).cloned().unwrap_or_else(|| "0".into());
                let data = self.fresh_array(&ty);
                head.env.insert(*id, format!("({c} {data} ({} {old}))", sels[1]));
                continue;
            }
            let v = self.fresh(&name, &ty);
            let w = self.wf(&ty, &v);
            head.env.insert(*id, v);
            self.fact(&head, w);
        }
        head
    }

    /// Houdini: start from template candidates, keep those true on entry, then repeatedly
    /// drop any the loop body can break until the rest are inductive.
    fn infer_loop(&mut self, st: &St, cond: &TExpr, user: &[TExpr], modified: &[LocalId], body: &TExpr, span: Span) -> Vec<crate::infer::Cand> {
        let mut scope = vec![];
        let mut ids: Vec<LocalId> = st.env.keys().copied().filter(|id| *id != crate::infer::QUANT_VAR).collect();
        ids.sort();
        for id in ids {
            let l = &self.m.locals[id as usize];
            let e = TExpr { kind: TExprKind::Local(id), ty: l.ty.clone(), span };
            match self.m.peel(&l.ty) {
                Ty::Int if !modified.contains(&id) && !l.name.starts_with('_') => scope.push(crate::infer::Term { e, text: l.name.clone() }),
                Ty::Array(_) => scope.push(crate::infer::Term { e: TExpr { kind: TExprKind::Len(Box::new(e)), ty: Ty::Int, span }, text: format!("{}.len", l.name) }),
                _ => {}
            }
        }
        let t0 = std::time::Instant::now();
        let trace = std::env::var("ASLANG_TRACE").is_ok();
        if trace {
            eprintln!("[loop] start {:?} sandbox={} elapsed {} ms", span, self.sandbox, self.started.elapsed().as_millis());
        }
        let mut all = match self.warm.get(&span) {
            // Inside an outer loop's exploratory run, or when re-verifying after a failed
            // inferred invariant, start from what survived before.
            Some(prev) if self.sandbox || self.retry => prev.clone(),
            _ => crate::infer::candidates(self.m, &self.consts, modified, &scope, body, span),
        };
        all.retain(|c| !self.skip.contains(&(span, c.text.clone())));
        // Cheap scalar bounds first; element ranges of arrays second, with the scalars known.
        let (quantified, scalar): (Vec<_>, Vec<_>) = all.into_iter().partition(|c| matches!(c.e.kind, TExprKind::Quant { .. }));
        let mut found = self.houdini(st, cond, user, modified, body, scalar);
        if !quantified.is_empty() {
            let mut known: Vec<TExpr> = user.to_vec();
            known.extend(found.iter().map(|c| c.e.clone()));
            let more = self.houdini(st, cond, &known, modified, body, quantified);
            found.extend(more);
        }
        if trace {
            eprintln!("[loop] done {:?} -> {} found in {} ms", span, found.len(), t0.elapsed().as_millis());
        }
        self.warm.insert(span, found.clone());
        found
    }

    fn houdini(&mut self, st: &St, cond: &TExpr, user: &[TExpr], modified: &[LocalId], body: &TExpr, mut cands: Vec<crate::infer::Cand>) -> Vec<crate::infer::Cand> {
        if cands.is_empty() {
            return cands;
        }
        let entry: Vec<String> = cands.iter().map(|c| self.spec_expr(&c.e, st)).collect();
        let keep = self.all_valid(&st.pc, &entry);
        cands = cands.into_iter().zip(keep).filter(|(_, k)| *k).map(|(c, _)| c).collect();
        let mut converged = false;
        for _round in 0..60 {
            if cands.is_empty() {
                converged = true;
                break;
            }
            let (events, saved) = (self.events.len(), self.sandbox);
            self.sandbox = true;
            let head = self.loop_head(st, modified, body);
            for inv in user.iter().chain(cands.iter().map(|c| &c.e)) {
                let g = self.spec_expr(inv, &head);
                self.fact(&head, g);
            }
            let keep = match self.expr(cond, head.clone()) {
                (Some(hs), cv) => {
                    let inside = St { pc: and(vec![hs.pc.clone(), cv]), env: hs.env.clone() };
                    match self.expr(body, inside) {
                        (Some(end), _) => {
                            let goals: Vec<String> = cands.iter().map(|c| self.spec_expr(&c.e, &end)).collect();
                            self.all_valid(&end.pc, &goals)
                        }
                        (None, _) => vec![true; cands.len()],
                    }
                }
                (None, _) => vec![true; cands.len()],
            };
            self.events.truncate(events);
            self.sandbox = saved;
            if keep.iter().all(|k| *k) {
                converged = true;
                break;
            }
            cands = cands.into_iter().zip(keep).filter(|(_, k)| *k).map(|(c, _)| c).collect();
        }
        if !converged {
            return vec![];
        }
        let before = cands.len();
        let pruned = crate::infer::tightest(cands);
        if pruned.len() == before {
            return pruned;
        }
        // Dropping looser bounds can break inductiveness when a bound only implies another
        // under side conditions, so the pruned set is checked again.
        self.houdini(st, cond, user, modified, body, pruned)
    }

    /// Which goals hold in every state satisfying the facts so far and `pc`? One solver call
    /// per round: a counterexample to the conjunction rules out every goal it falsifies.
    /// Which goals hold in every state satisfying the facts so far and `pc`? One solver
    /// process checks each goal separately with a short time limit, so a hard goal only
    /// costs its own limit and never hides the easy ones.
    fn all_valid(&mut self, pc: &str, goals: &[String]) -> Vec<bool> {
        if goals.is_empty() || self.out_of_budget() {
            return vec![false; goals.len()];
        }
        // True quantified goals are proved fast; false ones rarely get a model, so they only
        // get a short limit.
        let limit = if goals.iter().any(|g| g.contains("(forall")) { 150 } else { 500 };
        let mut script = format!("{}(set-option :timeout {limit})\n", self.preamble);
        for d in &self.decls {
            script += d;
            script.push('\n');
        }
        for e in &self.events {
            if let Event::Fact(f) = e {
                script += &format!("(assert {f})\n");
            }
        }
        script += &format!("(assert {pc})\n");
        for g in goals {
            script += &format!("(push 1)\n(assert (not {g}))\n(check-sat)\n(pop 1)\n");
        }
        let Some(ans) = run_z3(&self.z3, &script) else {
            return vec![false; goals.len()];
        };
        let mut out: Vec<bool> = ans.iter().map(|a| matches!(a, SExp::Atom(s) if s == "unsat")).collect();
        out.resize(goals.len(), false);
        out
    }

    fn out_of_budget(&self) -> bool {
        self.started.elapsed().as_millis() as u64 > self.budget_ms
    }

    fn spec_expr(&mut self, e: &TExpr, st: &St) -> String {
        let saved = self.spec;
        self.spec = true;
        let (_, v) = self.expr(e, st.clone());
        self.spec = saved;
        v
    }

    fn collect(&self, answers: Vec<SExp>, rep: &mut Report) {
        let mut it = answers.into_iter();
        for e in &self.events {
            let Event::Check { site, show, what, fix, notes, .. } = e else { continue };
            let status = it.next();
            let mut values = it.next();
            let status2 = it.next();
            let values2 = it.next();
            if matches!(&status2, Some(SExp::Atom(s)) if s == "sat") {
                values = values2;
            }
            let verdict = match status {
                Some(SExp::Atom(s)) if s == "unsat" => Verdict::Proved,
                Some(SExp::Atom(s)) if s == "sat" => {
                    let mut ce = vec![];
                    if let Some(SExp::List(pairs)) = values {
                        for (i, p) in pairs.iter().enumerate() {
                            if let (SExp::List(kv), Some((name, _))) = (p, show.get(i)) {
                                if kv.len() == 2 {
                                    ce.push((name.clone(), pretty(self.m, &kv[1])));
                                }
                            }
                        }
                    }
                    Verdict::Refuted(ce)
                }
                _ => Verdict::Unknown,
            };
            if site.kind.is_hint() {
                rep.verdicts.insert(*site, verdict);
                continue;
            }
            if let Some(text) = self.inferred_sites.get(site) {
                if verdict != Verdict::Proved {
                    rep.failed_inferred.push((site.span, text.clone()));
                }
                rep.verdicts.insert(*site, verdict);
                continue;
            }
            if matches!(site.kind, SiteKind::Requires(usize::MAX)) {
                if verdict == Verdict::Proved {
                    rep.diags.push(Diagnostic::error("E0209", site.span, format!("the `requires` of `{}` can never hold, so every proof about it would be vacuous", self.f.name)).with_fix("remove the contradictory clause"));
                }
                continue;
            }
            match &verdict {
                Verdict::Proved => rep.proved += 1,
                Verdict::Unknown => {
                    rep.runtime += 1;
                    let mut d = Diagnostic::warning("W0250", site.span, format!("could not prove within the time budget: {what}")).with_note("a run-time check is kept here");
                    if let Some(f) = fix {
                        d = d.with_fix(f.clone());
                    }
                    rep.diags.push(d);
                }
                Verdict::Refuted(ce) => {
                    rep.refuted += 1;
                    let code = match site.kind {
                        SiteKind::Overflow | SiteKind::DivOverflow | SiteKind::FastDiv32 | SiteKind::FastDivUnsigned => "E0201",
                        SiteKind::DivZero => "E0202",
                        SiteKind::Bounds => "E0210",
                        SiteKind::Length => "E0211",
                        SiteKind::Requires(_) => "E0203",
                        SiteKind::Ensures(_) => "E0204",
                        SiteKind::Refine => "E0205",
                        SiteKind::InvEntry(_) => "E0206",
                        SiteKind::InvKeep(_) => "E0207",
                        SiteKind::DecreasesBound | SiteKind::Decreases => "E0208",
                    };
                    let mut d = Diagnostic::error(code, site.span, what.clone());
                    d.counterexample = ce.clone();
                    d.notes = notes.clone();
                    if let Some(f) = fix {
                        d = d.with_fix(f.clone());
                    }
                    rep.diags.push(d);
                }
            }
            rep.verdicts.insert(*site, verdict);
        }
    }
}

fn merge(base: &St, cond: &str, a: Option<St>, av: String, b: Option<St>, bv: String) -> (Option<St>, String) {
    match (a, b) {
        (None, None) => (None, "0".into()),
        (Some(a), None) => (Some(a), av),
        (None, Some(b)) => (Some(b), bv),
        (Some(a), Some(b)) => {
            let simple_a = a.pc == and(vec![base.pc.clone(), cond.to_string()]);
            let simple_b = b.pc.starts_with(&and(vec![base.pc.clone(), format!("(not {cond})")]));
            let pc = if simple_a && simple_b && b.pc == and(vec![base.pc.clone(), format!("(not {cond})")]) { base.pc.clone() } else { format!("(or {} {})", a.pc, b.pc) };
            let mut env = HashMap::new();
            let keys: HashSet<&LocalId> = a.env.keys().chain(b.env.keys()).collect();
            for k in keys {
                match (a.env.get(k), b.env.get(k)) {
                    (Some(x), Some(y)) if x == y => {
                        env.insert(*k, x.clone());
                    }
                    (Some(x), Some(y)) => {
                        env.insert(*k, format!("(ite {cond} {x} {y})"));
                    }
                    (Some(x), None) | (None, Some(x)) => {
                        env.insert(*k, x.clone());
                    }
                    _ => {}
                }
            }
            let v = if av == bv { av } else { format!("(ite {cond} {av} {bv})") };
            (Some(St { pc, env }), v)
        }
    }
}

fn replace_word(s: &str, word: &str, with: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find(word) {
        let before = rest[..i].chars().last();
        let after = rest[i + word.len()..].chars().next();
        let ident = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
        out += &rest[..i];
        out += if ident(before) || ident(after) { word } else { with };
        rest = &rest[i + word.len()..];
    }
    out + rest
}

fn and(parts: Vec<String>) -> String {
    let parts: Vec<String> = parts.into_iter().filter(|p| p != "true").collect();
    match parts.len() {
        0 => "true".into(),
        1 => parts.into_iter().next().unwrap(),
        _ => format!("(and {})", parts.join(" ")),
    }
}

#[derive(Clone, Debug)]
pub enum SExp {
    Atom(String),
    List(Vec<SExp>),
}

pub fn parse_sexps(s: &str) -> Vec<SExp> {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut out = vec![];
    fn one(c: &[char], i: &mut usize) -> Option<SExp> {
        while *i < c.len() && c[*i].is_whitespace() {
            *i += 1;
        }
        if *i >= c.len() {
            return None;
        }
        if c[*i] == '(' {
            *i += 1;
            let mut items = vec![];
            loop {
                while *i < c.len() && c[*i].is_whitespace() {
                    *i += 1;
                }
                if *i >= c.len() {
                    break;
                }
                if c[*i] == ')' {
                    *i += 1;
                    break;
                }
                items.push(one(c, i)?);
            }
            return Some(SExp::List(items));
        }
        if c[*i] == '"' {
            let start = *i;
            *i += 1;
            while *i < c.len() && !(c[*i] == '"' && !(c.get(*i + 1) == Some(&'"'))) {
                if c[*i] == '"' {
                    *i += 1;
                }
                *i += 1;
            }
            *i += 1;
            return Some(SExp::Atom(c[start..(*i).min(c.len())].iter().collect()));
        }
        let start = *i;
        while *i < c.len() && !c[*i].is_whitespace() && c[*i] != '(' && c[*i] != ')' {
            *i += 1;
        }
        Some(SExp::Atom(c[start..*i].iter().collect()))
    }
    while let Some(e) = one(&chars, &mut i) {
        out.push(e);
    }
    out
}

/// Render an SMT array (`store` chains over a constant array) with its length as `[a, b, c]`.
fn show_array(data: &SExp, len: &SExp, names: &HashMap<String, (Option<Vec<String>>, String)>) -> String {
    fn go(e: &SExp, names: &HashMap<String, (Option<Vec<String>>, String)>) -> String {
        pretty_inner(e, names)
    }
    let n: i64 = match len {
        SExp::Atom(a) => a.parse().unwrap_or(0),
        _ => 0,
    };
    let mut default = None;
    let mut vals: HashMap<i64, String> = HashMap::new();
    let mut cur = data.clone();
    loop {
        match &cur {
            SExp::List(items) if matches!(items.first(), Some(SExp::Atom(s)) if s == "store") && items.len() == 4 => {
                if let SExp::Atom(i) = &items[2] {
                    if let Ok(i) = i.parse::<i64>() {
                        vals.entry(i).or_insert_with(|| go(&items[3], names));
                    }
                }
                cur = items[1].clone();
            }
            SExp::List(items) if items.len() == 2 && matches!(&items[0], SExp::List(h) if matches!(h.get(1), Some(SExp::Atom(s)) if s == "const")) => {
                default = Some(go(&items[1], names));
                break;
            }
            _ => break,
        }
    }
    if n > 12 {
        let shown: Vec<String> = (0..8).map(|i| vals.get(&i).cloned().or(default.clone()).unwrap_or_else(|| "?".into())).collect();
        return format!("[{}, ... ({n} elements)]", shown.join(", "));
    }
    let shown: Vec<String> = (0..n.max(0)).map(|i| vals.get(&i).cloned().or(default.clone()).unwrap_or_else(|| "?".into())).collect();
    format!("[{}]", shown.join(", "))
}

/// Render a model value in AS syntax.
fn pretty(m: &Module, e: &SExp) -> String {
    let mut names: HashMap<String, (Option<Vec<String>>, String)> = HashMap::new();
    for t in datatypes(m) {
        if matches!(m.erase(&t), Ty::Array(_)) {
            names.insert(ctors(m, &t).remove(0).0, (None, "[]".into()));
            continue;
        }
        let is_record = matches!(m.erase(&t), Ty::Record(_));
        let fields: Vec<String> = match m.erase(&t) {
            Ty::Record(r) => m.records[r].fields.iter().map(|f| f.0.clone()).collect(),
            _ => vec![],
        };
        for (i, (c, _)) in ctors(m, &t).into_iter().enumerate() {
            let vname = if is_record { String::new() } else { m.variants(&t)[i].0.clone() };
            names.insert(c, (if is_record { Some(fields.clone()) } else { None }, vname));
        }
    }
    return pretty_inner(e, &names);
}

fn pretty_inner(e: &SExp, names: &HashMap<String, (Option<Vec<String>>, String)>) -> String {
    let go = pretty_inner;
    {
        match e {
            SExp::Atom(a) => names.get(a).map(|(_, v)| v.clone()).unwrap_or_else(|| a.clone()),
            SExp::List(items) => {
                if let [SExp::Atom(op), SExp::Atom(n)] = items.as_slice() {
                    if op == "-" {
                        return format!("-{n}");
                    }
                }
                if let Some(SExp::Atom(head)) = items.first() {
                    if names.get(head).is_some_and(|(_, v)| v == "[]") && items.len() == 3 {
                        return show_array(&items[1], &items[2], names);
                    }
                    if let Some((fields, vname)) = names.get(head) {
                        let args: Vec<String> = items[1..].iter().map(|x| go(x, names)).collect();
                        return match fields {
                            Some(fs) => format!("{{ {} }}", fs.iter().zip(&args).map(|(f, a)| format!("{f}: {a}")).collect::<Vec<_>>().join(", ")),
                            None => format!("{vname}({})", args.join(", ")),
                        };
                    }
                }
                format!("({})", items.iter().map(|x| go(x, names)).collect::<Vec<_>>().join(" "))
            }
        }
    }
}

/// Does the current contract of `f` refine a pinned one? Returns verdicts for
/// "old precondition implies new precondition" and
/// "under the old precondition, new postcondition implies old postcondition".
pub fn check_refinement(m: &Module, src: &Source, f: &Func, old_req: &[TExpr], old_ens: &[TExpr], opts: &Options) -> Option<(Verdict, Verdict)> {
    let pre = preamble(m);
    let mut fv = Fv::new(m, src, f, opts, &pre);
    fv.spec = true;
    let mut env = HashMap::new();
    for &p in &f.params {
        let l = &m.locals[p as usize];
        let v = fv.fresh(&l.name, &l.ty.clone());
        fv.params_show.push((l.name.clone(), v.clone()));
        env.insert(p, v);
    }
    let st = St { pc: "true".into(), env };
    let mut wf = vec![];
    for &p in &f.params {
        let v = st.env[&p].clone();
        wf.push(fv.wf(&m.locals[p as usize].ty.clone(), &v));
    }
    let terms = |fv: &mut Fv, xs: &[TExpr], st: &St| -> String { and(xs.iter().map(|x| fv.spec_expr(x, st)).collect()) };
    let (oreq, nreq) = (terms(&mut fv, old_req, &st), terms(&mut fv, &f.requires, &st));
    let r = fv.fresh("result", &f.ret);
    let mut st2 = st.clone();
    st2.env.insert(f.result, r.clone());
    wf.push(fv.wf(&f.ret.clone(), &r));
    let (oens, nens) = (terms(&mut fv, old_ens, &st2), terms(&mut fv, &f.ensures, &st2));
    let wf = and(wf);
    let params: Vec<String> = fv.params_show.iter().map(|(_, v)| v.clone()).collect();
    let pvals = if params.is_empty() { "0".to_string() } else { params.join(" ") };
    let mut script = format!("{}(set-option :timeout {})\n", preamble(m), opts.timeout_ms);
    script += &fv.decls.join("\n");
    let small = fv.small();
    script += &format!("\n(assert {wf})\n(push 1)\n(assert {oreq})\n(assert (not {nreq}))\n(check-sat)\n(get-value ({pvals}))\n(assert {small})\n(check-sat)\n(get-value ({pvals}))\n(pop 1)\n");
    script += &format!("(push 1)\n(assert {oreq})\n(assert {nens})\n(assert (not {oens}))\n(check-sat)\n(get-value ({pvals} {r}))\n(assert {small})\n(check-sat)\n(get-value ({pvals} {r}))\n(pop 1)\n");
    let ans = run_z3(&opts.z3, &script)?;
    let mut names: Vec<String> = fv.params_show.iter().map(|(n, _)| n.clone()).collect();
    let verdict = |status: Option<&SExp>, vals: Option<&SExp>, names: &[String]| -> Verdict {
        match status {
            Some(SExp::Atom(s)) if s == "unsat" => Verdict::Proved,
            Some(SExp::Atom(s)) if s == "sat" => {
                let mut ce = vec![];
                if let Some(SExp::List(pairs)) = vals {
                    for (p, n) in pairs.iter().zip(names) {
                        if let SExp::List(kv) = p {
                            if kv.len() == 2 {
                                ce.push((n.clone(), pretty(m, &kv[1])));
                            }
                        }
                    }
                }
                Verdict::Refuted(ce)
            }
            _ => Verdict::Unknown,
        }
    };
    let pick = |i: usize| if matches!(ans.get(i + 2), Some(SExp::Atom(s)) if s == "sat") { ans.get(i + 3) } else { ans.get(i + 1) };
    let pre = verdict(ans.first(), pick(0), &names);
    names.push("result".into());
    let post = verdict(ans.get(4), pick(4), &names);
    Some((pre, post))
}

/// Arrays that a loop body pushes to or replaces (as opposed to writing elements in place).
fn resized_arrays(e: &TExpr) -> HashSet<LocalId> {
    fn go(e: &TExpr, out: &mut HashSet<LocalId>) {
        match &e.kind {
            TExprKind::Block(ss, t) => {
                for s in ss {
                    match s {
                        TStmt::Let(id, v) | TStmt::Assign(id, v) => {
                            out.insert(*id);
                            go(v, out);
                        }
                        TStmt::Push(id, v, _) => {
                            out.insert(*id);
                            go(v, out);
                        }
                        TStmt::Expr(v) | TStmt::Return(v, _) => go(v, out),
                        TStmt::IndexAssign(_, i, v, _) => {
                            go(i, out);
                            go(v, out);
                        }
                        TStmt::While { body, .. } => go(body, out),
                    }
                }
                go(t, out);
            }
            TExprKind::If(c, t, f) => {
                go(c, out);
                go(t, out);
                go(f, out);
            }
            TExprKind::Match(s, arms) => {
                go(s, out);
                arms.iter().for_each(|(_, b)| go(b, out));
            }
            _ => {}
        }
    }
    let mut out = HashSet::new();
    go(e, &mut out);
    out
}
