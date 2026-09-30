//! Ownership for heap values (arrays). Every array variable owns one reference. Reading a
//! variable for the last time moves the reference instead of adding one, so an array passed
//! along or reassigned is never copied; writes copy only if someone else still holds it.
//!
//! This module finds those last reads with a backward liveness pass over a function body.
//! Contract expressions count as reads, because a run-time check may need them.

use std::collections::HashSet;

use crate::diag::Span;
use crate::tir::*;
use crate::verify::SiteKind;

pub type ReadSite = (u32, u32, LocalId);

pub struct Liveness<'a> {
    m: &'a Module,
    f: &'a Func,
    pub last: HashSet<ReadSite>,
    record: bool,
    /// Is this check proved (so it will not read anything at run time)?
    proved: &'a dyn Fn(SiteKind, Span) -> bool,
}

pub fn last_uses(m: &Module, f: &Func, proved: &dyn Fn(SiteKind, Span) -> bool) -> HashSet<ReadSite> {
    let mut l = Liveness { m, f, last: HashSet::new(), record: true, proved };
    let mut live = HashSet::new();
    let tail = match &f.body.kind {
        TExprKind::Block(_, t) => t.span,
        _ => f.body.span,
    };
    l.ensures_reads(tail, &mut live);
    l.expr(&f.body, &mut live);
    l.last
}

impl<'a> Liveness<'a> {
    fn is_array(&self, id: LocalId) -> bool {
        matches!(self.m.peel(&self.m.locals[id as usize].ty), Ty::Array(_))
    }

    fn read(&mut self, id: LocalId, e: &TExpr, live: &mut HashSet<LocalId>) {
        if !self.is_array(id) {
            return;
        }
        if self.record && !live.contains(&id) {
            self.last.insert((e.span.lo, e.span.hi, id));
        }
        live.insert(id);
    }

    /// Reads made by the postcondition checks that stay at run time at this return.
    fn ensures_reads(&mut self, at: Span, live: &mut HashSet<LocalId>) {
        for (i, en) in self.f.ensures.clone().iter().enumerate() {
            if !(self.proved)(SiteKind::Ensures(i), at) {
                self.spec(en, live);
            }
        }
    }

    /// Reads made by contract expressions (checked at run time when not proved).
    fn spec(&mut self, e: &TExpr, live: &mut HashSet<LocalId>) {
        let saved = self.record;
        self.record = false;
        self.expr(e, live);
        self.record = saved;
    }

    /// Process `e` backwards: `live` holds the variables read after `e` and becomes the set
    /// read after the point just before `e`.
    fn expr(&mut self, e: &TExpr, live: &mut HashSet<LocalId>) {
        match &e.kind {
            TExprKind::Local(id) => self.read(*id, e, live),
            TExprKind::Int(_) | TExprKind::Bool(_) | TExprKind::Str(_) | TExprKind::Unit => {}
            TExprKind::Field(x, _) | TExprKind::Unary(_, x) | TExprKind::Coerce(x, _) | TExprKind::Len(x) => self.expr(x, live),
            TExprKind::Is(x, _) => self.expr(x, live),
            TExprKind::Binary(_, a, b) | TExprKind::Index(a, b) | TExprKind::ArrayRepeat(a, b) => {
                self.expr(b, live);
                self.expr(a, live);
            }
            TExprKind::Record(xs) | TExprKind::Ctor(_, xs) | TExprKind::Print(xs) | TExprKind::ArrayLit(xs) => {
                for x in xs.iter().rev() {
                    self.expr(x, live);
                }
            }
            TExprKind::Call(fi, xs) => {
                // The callee's `requires` may be checked at run time against these arguments;
                // the arguments are evaluated into temporaries first, so no extra reads.
                let _ = fi;
                for x in xs.iter().rev() {
                    self.expr(x, live);
                }
            }
            TExprKind::Update(b, fs) => {
                for (_, x) in fs.iter().rev() {
                    self.expr(x, live);
                }
                self.expr(b, live);
            }
            TExprKind::If(c, t, f) => {
                let mut lt = live.clone();
                let mut lf = live.clone();
                self.expr(t, &mut lt);
                self.expr(f, &mut lf);
                *live = &lt | &lf;
                self.expr(c, live);
            }
            TExprKind::Match(s, arms) => {
                let mut out = HashSet::new();
                for (_, b) in arms {
                    let mut l = live.clone();
                    self.expr(b, &mut l);
                    out = &out | &l;
                }
                *live = out;
                self.expr(s, live);
            }
            TExprKind::Block(stmts, tail) => {
                self.expr(tail, live);
                for s in stmts.iter().rev() {
                    self.stmt(s, live);
                }
            }
            TExprKind::Quant { lo, hi, body, .. } => {
                self.expr(body, live);
                self.expr(hi, live);
                self.expr(lo, live);
            }
        }
    }

    fn stmt(&mut self, s: &TStmt, live: &mut HashSet<LocalId>) {
        match s {
            TStmt::Let(id, e) | TStmt::Assign(id, e) => {
                live.remove(id);
                self.expr(e, live);
            }
            TStmt::Expr(e) => self.expr(e, live),
            TStmt::IndexAssign(id, i, v, _) => {
                live.insert(*id);
                self.expr(v, live);
                self.expr(i, live);
            }
            TStmt::Push(id, v, _) => {
                live.insert(*id);
                self.expr(v, live);
            }
            TStmt::Return(e, span) => {
                // Nothing runs after a return except the postcondition checks.
                live.clear();
                self.ensures_reads(*span, live);
                self.expr(e, live);
            }
            TStmt::While { cond, invariants, decreases, body, span, .. } => {
                let span = *span;
                let proved = self.proved;
                let invariants: Vec<&TExpr> = invariants.iter().enumerate().filter(|(i, _)| !(proved(SiteKind::InvEntry(*i), span) && proved(SiteKind::InvKeep(*i), span))).map(|(_, e)| e).collect();
                let decreases: Vec<&TExpr> = decreases.iter().filter(|_| !(proved(SiteKind::DecreasesBound, span) && proved(SiteKind::Decreases, span))).collect();
                let after = live.clone();
                // Iterate to a fixpoint without recording, then once more to record.
                let saved = self.record;
                self.record = false;
                let mut head = after.clone();
                loop {
                    let mut l = head.clone();
                    self.expr(body, &mut l);
                    let mut next = &l | &after;
                    self.expr(cond, &mut next);
                    for inv in invariants.iter().chain(decreases.iter()) {
                        self.spec(inv, &mut next);
                    }
                    let _ = &invariants;
                    let grown = &head | &next;
                    if grown == head {
                        break;
                    }
                    head = grown;
                }
                self.record = saved;
                let mut l = head.clone();
                self.expr(body, &mut l);
                let mut before = &l | &after;
                self.expr(cond, &mut before);
                for inv in invariants.iter().chain(decreases.iter()) {
                    self.spec(inv, &mut before);
                }
                *live = &before | &head;
            }
        }
    }
}

/// Array variables used only in place (indexed, measured, written, pushed) inside `e`, never
/// copied or passed on. Inside a loop over such code their reference count stays at one.
pub fn copied_arrays(m: &Module, e: &TExpr, out: &mut HashSet<LocalId>) {
    fn go(m: &Module, e: &TExpr, out: &mut HashSet<LocalId>, borrowed: bool) {
        match &e.kind {
            TExprKind::Local(id) if !borrowed && matches!(m.peel(&m.locals[*id as usize].ty), Ty::Array(_)) => {
                out.insert(*id);
            }
            TExprKind::Index(a, i) => {
                go(m, a, out, true);
                go(m, i, out, false);
            }
            TExprKind::Len(a) => go(m, a, out, true),
            TExprKind::Field(x, _) | TExprKind::Unary(_, x) | TExprKind::Coerce(x, _) | TExprKind::Is(x, _) => go(m, x, out, false),
            TExprKind::Binary(_, a, b) | TExprKind::ArrayRepeat(a, b) => {
                go(m, a, out, false);
                go(m, b, out, false);
            }
            TExprKind::Record(xs) | TExprKind::Ctor(_, xs) | TExprKind::Print(xs) | TExprKind::ArrayLit(xs) | TExprKind::Call(_, xs) => xs.iter().for_each(|x| go(m, x, out, false)),
            TExprKind::Update(b, fs) => {
                go(m, b, out, false);
                fs.iter().for_each(|(_, x)| go(m, x, out, false));
            }
            TExprKind::If(c, t, f) => [c, t, f].iter().for_each(|x| go(m, x, out, false)),
            TExprKind::Match(s, arms) => {
                go(m, s, out, false);
                arms.iter().for_each(|(_, b)| go(m, b, out, false));
            }
            TExprKind::Block(ss, t) => {
                for s in ss {
                    match s {
                        TStmt::Let(_, v) | TStmt::Assign(_, v) | TStmt::Expr(v) | TStmt::Return(v, _) => go(m, v, out, false),
                        TStmt::IndexAssign(_, i, v, _) => {
                            go(m, i, out, false);
                            go(m, v, out, false);
                        }
                        TStmt::Push(_, v, _) => go(m, v, out, false),
                        TStmt::While { cond, body, .. } => {
                            go(m, cond, out, false);
                            go(m, body, out, false);
                        }
                    }
                }
                go(m, t, out, false);
            }
            _ => {}
        }
    }
    go(m, e, out, false)
}
