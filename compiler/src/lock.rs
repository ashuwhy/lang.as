//! The contract lock. `tmk lock` pins every public function's signature, contract and
//! effects, plus the definitions of the types they use. Later checks must refine the pin:
//! a weaker contract, a new effect or a changed type fails until a human re-pins it.

use std::collections::BTreeMap;

use serde_json::{json, Value};

use crate::ast::Program;
use crate::check::{check_program_with, PinnedClauses};
use crate::diag::{Diagnostic, Source};
use crate::tir::*;
use crate::verify::{check_refinement, Options, Verdict};

pub fn type_defs(m: &Module) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for r in &m.records {
        let fs: Vec<String> = r.fields.iter().map(|(n, t)| format!("{n}: {}", m.show(t))).collect();
        out.insert(r.name.clone(), format!("{{ {} }}", fs.join(", ")));
    }
    for e in &m.enums {
        let vs: Vec<String> = e
            .variants
            .iter()
            .map(|v| if v.fields.is_empty() { v.name.clone() } else { format!("{}({})", v.name, v.fields.iter().map(|(n, t)| format!("{n}: {}", m.show(t))).collect::<Vec<_>>().join(", ")) })
            .collect();
        out.insert(e.name.clone(), format!("enum {{ {} }}", vs.join(", ")));
    }
    for a in m.aliases.iter().skip(1) {
        let pred = a.pred_src.as_ref().map(|p| format!(" where {}", p.split_whitespace().collect::<Vec<_>>().join(" "))).unwrap_or_default();
        out.insert(a.name.clone(), format!("{}{pred}", m.show(&a.base)));
    }
    out
}

pub fn snapshot(m: &Module) -> Value {
    let mut fns = serde_json::Map::new();
    for f in m.funcs.iter().filter(|f| f.is_pub) {
        let norm = |xs: &[String]| xs.iter().map(|x| x.split_whitespace().collect::<Vec<_>>().join(" ")).collect::<Vec<_>>();
        fns.insert(
            f.name.clone(),
            json!({ "signature": f.sig_src, "requires": norm(&f.requires_src), "ensures": norm(&f.ensures_src), "uses": f.uses }),
        );
    }
    json!({ "functions": fns, "types": type_defs(m) })
}

fn strings(v: &Value) -> Vec<String> {
    v.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default()
}

/// Compare the program with its pinned entry and report every way the contract got weaker.
pub fn check_against(prog: &Program, src: &Source, m: &Module, pinned: &Value, opts: &Options) -> (Vec<Diagnostic>, Vec<String>) {
    let mut diags = vec![];
    let mut notes = vec![];
    let fns = pinned["functions"].as_object().cloned().unwrap_or_default();

    // Types used by pinned contracts must not change underneath them.
    let now = type_defs(m);
    if let Some(types) = pinned["types"].as_object() {
        for (name, def) in types {
            let def = def.as_str().unwrap_or_default();
            match now.get(name) {
                Some(d) if d == def => {}
                Some(d) => diags.push(Diagnostic::error("E0305", find_decl(src, name), format!("the pinned type `{name}` changed")).with_note(format!("pinned:   {def}")).with_note(format!("proposed: {d}")).with_fix("if the change is intended, have an owner review it and run `tmk lock` again")),
                None => diags.push(Diagnostic::error("E0305", Default::default(), format!("the pinned type `{name}` was removed"))),
            }
        }
    }

    let mut clauses = vec![];
    let mut order = vec![];
    for (name, entry) in &fns {
        let Some(f) = m.funcs.iter().find(|f| &f.name == name) else {
            diags.push(Diagnostic::error("E0304", Default::default(), format!("the pinned function `{name}` was removed")).with_fix("restore it, or have an owner approve the removal and run `tmk lock`"));
            continue;
        };
        let sig = entry["signature"].as_str().unwrap_or_default();
        if sig != f.sig_src {
            diags.push(Diagnostic::error("E0304", f.sig_span, format!("the signature of `{name}` changed")).with_note(format!("pinned:   {sig}")).with_note(format!("proposed: {}", f.sig_src)).with_fix("if the change is intended, have an owner review it and run `tmk lock` again"));
            continue;
        }
        let old_uses = strings(&entry["uses"]);
        let added: Vec<&String> = f.uses.iter().filter(|u| !old_uses.contains(u)).collect();
        if !added.is_empty() {
            diags.push(
                Diagnostic::error("E0303", f.sig_span, format!("`{name}` gained the effect(s) {}", added.iter().map(|a| format!("`{a}`")).collect::<Vec<_>>().join(", ")))
                    .with_note(format!("pinned effects: {}", if old_uses.is_empty() { "none (pure)".into() } else { old_uses.join(", ") }))
                    .with_fix("remove the new effect, or have an owner approve it and run `tmk lock`"),
            );
        }
        let parse = |xs: Vec<String>| -> Option<Vec<crate::ast::Expr>> { xs.iter().map(|x| crate::parser::parse_expr(crate::lexer::lex(x).ok()?).ok()).collect() };
        match (parse(strings(&entry["requires"])), parse(strings(&entry["ensures"]))) {
            (Some(r), Some(e)) => {
                clauses.push(PinnedClauses { func: name.clone(), requires: r, ensures: e });
                order.push((name.clone(), strings(&entry["requires"]), strings(&entry["ensures"])));
            }
            _ => diags.push(Diagnostic::error("E0304", f.sig_span, format!("the pinned contract of `{name}` no longer parses"))),
        }
    }

    let (m2, _, typed) = check_program_with(prog, src, &clauses);
    for ((name, oreq, oens), typed) in order.into_iter().zip(typed) {
        let f = m2.funcs.iter().find(|f| f.name == name).unwrap();
        let Some((req, ens)) = typed else {
            diags.push(Diagnostic::error("E0304", f.sig_span, format!("the pinned contract of `{name}` no longer type-checks")).with_fix("have an owner review the change and run `tmk lock` again"));
            continue;
        };
        let Some((pre, post)) = check_refinement(&m2, src, f, &req, &ens, opts) else {
            diags.push(Diagnostic::warning("W0251", f.sig_span, "the SMT solver was not found, so the contract lock was not checked"));
            continue;
        };
        let show = |xs: &[String]| if xs.is_empty() { "true".to_string() } else { xs.join(" && ") };
        // Clause by clause, what changed: a reviewer reads the difference, not both contracts.
        let diff = |kw: &str, old: &[String], new: &[String]| -> Vec<String> {
            let norm = |x: &String| x.split_whitespace().collect::<Vec<_>>().join(" ");
            let (o, n): (Vec<String>, Vec<String>) = (old.iter().map(norm).collect(), new.iter().map(norm).collect());
            let mut out: Vec<String> = o.iter().filter(|x| !n.contains(x)).map(|x| format!("removed:  {kw} {x}")).collect();
            out.extend(n.iter().filter(|x| !o.contains(x)).map(|x| format!("added:    {kw} {x}")));
            out
        };
        let pre_ok = pre == Verdict::Proved;
        match pre {
            Verdict::Refuted(ce) => {
                let mut d = Diagnostic::error("E0302", f.sig_span, format!("`{name}` now demands more from its callers than its pinned contract"));
                for line in diff("requires", &oreq, &f.requires_src) {
                    d = d.with_note(line);
                }
                d = d
                    .with_note("the counterexample is a call that was allowed before and is rejected now")
                    .with_fix("keep the pinned precondition, or have an owner approve the change and run `tmk lock`");
                d.counterexample = ce;
                diags.push(d);
            }
            Verdict::Unknown => diags.push(Diagnostic::warning("W0250", f.sig_span, format!("could not decide whether the precondition of `{name}` still matches the pin"))),
            Verdict::Proved => {}
        }
        match post {
            Verdict::Refuted(ce) => {
                let mut d = Diagnostic::error("E0301", f.sig_span, format!("the contract of `{name}` is WEAKER than the pinned one"));
                for line in diff("ensures", &oens, &f.ensures_src) {
                    d = d.with_note(line);
                }
                d = d.with_note("the counterexample is a result the new contract allows and the pinned one forbids");
                if let Some(note) = lost_determinism(&m2, src, f, &ens, &oens, opts) {
                    d = d.with_note(note);
                }
                d = d
                    .with_fix("restore the pinned guarantee; a weaker contract needs an owner-approved `tmk lock`");
                d.counterexample = ce;
                diags.push(d);
            }
            Verdict::Unknown => diags.push(Diagnostic::warning("W0250", f.sig_span, format!("could not decide whether the contract of `{name}` still refines the pin"))),
            Verdict::Proved => {
                if pre_ok && (show(&oens) != show(&f.ensures_src) || show(&oreq) != show(&f.requires_src)) {
                    notes.push(format!("`{name}`: contract changed and still refines the pin; run `tmk lock` to pin the stronger version"));
                }
            }
        }
    }
    for f in m.funcs.iter().filter(|f| f.is_pub && !fns.contains_key(&f.name)) {
        notes.push(format!("`{}` is public but not pinned yet; run `tmk lock`", f.name));
    }
    (diags, notes)
}

/// If the pinned contract decided every result and the proposed one does not, say so with an
/// input that now has two allowed results.
fn lost_determinism(m: &Module, src: &Source, f: &crate::tir::Func, pinned_ens: &[crate::tir::TExpr], pinned_src: &[String], opts: &Options) -> Option<String> {
    let fi = m.funcs.iter().position(|g| g.name == f.name)?;
    let none = crate::verify::Report::default();
    let now = crate::verify::examples(m, src, opts, &none, Some(&f.name)).pop()?;
    let (args, r1, r2) = now.open?;
    let mut old = m.clone();
    old.funcs[fi].ensures = pinned_ens.to_vec();
    old.funcs[fi].ensures_src = pinned_src.to_vec();
    let before = crate::verify::examples(&old, src, opts, &none, Some(&f.name)).pop()?;
    if !before.decided {
        return None;
    }
    let call = format!("{}({})", f.name, args.iter().map(|(n, v)| format!("{n}: {v}")).collect::<Vec<_>>().join(", "));
    Some(format!("the pinned contract decided every result; the proposed one does not: {call} may now return {r1} or {r2}"))
}

fn find_decl(src: &Source, name: &str) -> crate::diag::Span {
    for kw in ["type ", "enum "] {
        if let Some(i) = src.text.find(&format!("{kw}{name}")) {
            return crate::diag::Span::new(i, i + kw.len() + name.len());
        }
    }
    Default::default()
}
