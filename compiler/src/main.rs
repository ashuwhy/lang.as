use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use aslang::diag::{Diagnostic, Source};
use aslang::verify::{Options, Report};
use serde_json::json;

const USAGE: &str = "aslang - compiler for AS

usage:
  aslang check <file.as>            type-check, verify, and compare with aslang.lock
  aslang build <file.as> [-o out]   check, then compile to a native binary
  aslang run <file.as>              build and run
  aslang lock <file.as>             pin the public contracts in aslang.lock
  aslang emit-c <file.as>           print the generated C
  aslang emit-smt <file.as>         print the SMT-LIB queries
  aslang explain <code>             explain a diagnostic, e.g. `aslang explain E0201`

options:
  --json          machine-readable output
  --no-verify     skip the prover; every check stays at run time
  --proved        fail unless every check is proved (no run-time checks)
  --show-inferred print the loop invariants the compiler inferred
  --no-infer      do not infer loop invariants
  --timeout <ms>  solver time budget per check (default 2000)
";

struct Args {
    cmd: String,
    file: String,
    out: Option<String>,
    json: bool,
    verify: bool,
    proved: bool,
    timeout: u32,
    show_inferred: bool,
    infer: bool,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut a = Args { cmd: String::new(), file: String::new(), out: None, json: false, verify: true, proved: false, timeout: 2000, show_inferred: false, infer: true };
    let mut it = argv.into_iter();
    while let Some(x) = it.next() {
        match x.as_str() {
            "--json" => a.json = true,
            "--no-verify" => a.verify = false,
            "--proved" => a.proved = true,
            "--show-inferred" => a.show_inferred = true,
            "--no-infer" => a.infer = false,
            "-o" => a.out = it.next(),
            "--timeout" => a.timeout = it.next().and_then(|t| t.parse().ok()).unwrap_or(2000),
            "-h" | "--help" => {
                print!("{USAGE}");
                exit(0)
            }
            "--version" => {
                println!("aslang {}", env!("CARGO_PKG_VERSION"));
                exit(0)
            }
            _ if a.cmd.is_empty() => a.cmd = x,
            _ if a.file.is_empty() => a.file = x,
            _ => {
                eprintln!("unexpected argument `{x}`\n\n{USAGE}");
                exit(2)
            }
        }
    }
    if a.cmd.is_empty() || a.file.is_empty() {
        eprint!("{USAGE}");
        exit(2)
    }
    a
}

struct Outcome {
    src: Source,
    diags: Vec<Diagnostic>,
    notes: Vec<String>,
    report: Option<Report>,
    module: Option<aslang::tir::Module>,
}

fn lock_path(file: &str) -> PathBuf {
    Path::new(file).parent().map(|p| p.join("aslang.lock")).unwrap_or_else(|| PathBuf::from("aslang.lock"))
}

fn file_key(file: &str) -> String {
    Path::new(file).file_name().map(|s| s.to_string_lossy().to_string()).unwrap_or_default()
}

fn read_lock(file: &str) -> Option<serde_json::Value> {
    let text = std::fs::read_to_string(lock_path(file)).ok()?;
    let v: serde_json::Value = serde_json::from_str(&text).ok()?;
    v["files"].get(file_key(file)).cloned()
}

fn analyse(a: &Args, with_lock: bool) -> Outcome {
    let text = std::fs::read_to_string(&a.file).unwrap_or_else(|e| {
        eprintln!("cannot read {}: {e}", a.file);
        exit(2)
    });
    let src = Source::new(a.file.clone(), text);
    let mut out = Outcome { src, diags: vec![], notes: vec![], report: None, module: None };
    let toks = match aslang::lexer::lex(&out.src.text) {
        Ok(t) => t,
        Err(d) => {
            out.diags.push(d);
            return out;
        }
    };
    let prog = match aslang::parser::parse(toks) {
        Ok(p) => p,
        Err(d) => {
            out.diags.push(d);
            return out;
        }
    };
    let (m, diags) = aslang::check::check_program(&prog, &out.src);
    out.diags = diags;
    if out.diags.iter().any(|d| d.is_error()) {
        return out;
    }
    let opts = Options { timeout_ms: a.timeout, infer: a.infer, ..Default::default() };
    if a.verify {
        let rep = aslang::verify::verify(&m, &out.src, &opts);
        out.diags.extend(rep.diags.iter().cloned());
        if a.proved && rep.runtime > 0 {
            out.diags.push(Diagnostic::error("E0250", Default::default(), format!("--proved: {} check(s) could not be proved", rep.runtime)));
        }
        out.report = Some(rep);
        if with_lock {
            if let Some(pinned) = read_lock(&a.file) {
                let (d, n) = aslang::lock::check_against(&prog, &out.src, &m, &pinned, &opts);
                out.diags.extend(d);
                out.notes.extend(n);
            }
        }
    }
    out.module = Some(m);
    out
}

fn report(a: &Args, o: &Outcome, extra: serde_json::Value) -> bool {
    let errors = o.diags.iter().filter(|d| d.is_error()).count();
    let ok = errors == 0;
    let (p, r, x) = o.report.as_ref().map(|r| (r.proved, r.runtime, r.refuted)).unwrap_or((0, 0, 0));
    if a.json {
        let v = json!({
            "file": a.file, "ok": ok,
            "summary": { "proved": p, "runtime_checked": r, "refuted": x, "errors": errors },
            "inferred_invariants": o.report.as_ref().map(|r| r.inferred.iter().map(|(s, v)| { let (l, c) = o.src.line_col(s.lo); json!({"line": l, "col": c, "invariants": v}) }).collect::<Vec<_>>()).unwrap_or_default(),
            "diagnostics": o.diags.iter().map(|d| d.to_json(&o.src)).collect::<Vec<_>>(),
            "notes": o.notes,
            "result": extra,
        });
        println!("{}", serde_json::to_string_pretty(&v).unwrap());
    } else {
        for d in &o.diags {
            eprint!("{}", d.render(&o.src));
            eprintln!();
        }
        for n in &o.notes {
            eprintln!("note: {n}");
        }
        if a.show_inferred {
            if let Some(r) = &o.report {
                for (span, invs) in &r.inferred {
                    let (l, c) = o.src.line_col(span.lo);
                    eprintln!("inferred for the loop at {}:{l}:{c}:", o.src.name);
                    for i in invs {
                        eprintln!("    invariant {i}");
                    }
                }
            }
        }
        let name = file_key(&a.file);
        let inferred: usize = o.report.as_ref().map(|r| r.inferred.iter().map(|(_, v)| v.len()).sum()).unwrap_or(0);
        let inferred_note = if inferred > 0 { format!(" ({inferred} loop invariants inferred)") } else { String::new() };
        if ok {
            if o.report.is_some() {
                eprintln!("ok {name}: {p} checks proved, {r} kept at run time{inferred_note}");
            } else {
                eprintln!("ok {name}: not verified (--no-verify), every check kept at run time");
            }
        } else {
            eprintln!("FAILED {name}: {errors} error(s); {p} proved, {r} kept at run time, {x} refuted");
        }
    }
    ok
}

fn build(a: &Args, o: &Outcome, out: &Path) -> Result<(), String> {
    let m = o.module.as_ref().unwrap();
    if !m.funcs.iter().any(|f| f.name == "main") {
        return Err("no `fn main()` to build; `aslang check` verifies a library".into());
    }
    let empty = Default::default();
    let verdicts = o.report.as_ref().map(|r| &r.verdicts).unwrap_or(&empty);
    let c = aslang::codegen::generate(m, &o.src, verdicts);
    let cfile = std::env::temp_dir().join(format!("aslang_{}_{}.c", std::process::id(), file_key(&a.file)));
    std::fs::write(&cfile, c).map_err(|e| e.to_string())?;
    // Any C compiler with GNU C extensions works; set CC to choose (gcc and clang are both tested).
    let cc = std::env::var("CC").unwrap_or_else(|_| "cc".into());
    let status = Command::new(&cc).args(["-O2", "-std=gnu11", "-w", "-o"]).arg(out).arg(&cfile).status().map_err(|e| format!("cannot run the C compiler `{cc}`: {e}"))?;
    let _ = std::fs::remove_file(&cfile);
    if !status.success() {
        return Err(format!("the C compiler failed (this is a compiler bug; `aslang emit-c {}` shows the code)", a.file));
    }
    Ok(())
}

fn main() {
    let a = parse_args();
    match a.cmd.as_str() {
        "explain" => match aslang::explain::explain(&a.file) {
            Some(t) => print!("{t}"),
            None => {
                eprintln!("no such code `{}`; codes are:", a.file);
                for e in aslang::explain::ENTRIES {
                    eprintln!("  {}  {}", e.code, e.title);
                }
                exit(2)
            }
        },
        "check" => {
            let o = analyse(&a, true);
            exit(if report(&a, &o, json!(null)) { 0 } else { 1 })
        }
        "emit-smt" => {
            let o = analyse(&a, false);
            match &o.report {
                Some(r) => print!("{}", r.smt),
                None => {
                    report(&a, &o, json!(null));
                    exit(1)
                }
            }
        }
        "emit-c" => {
            let o = analyse(&a, false);
            if o.diags.iter().any(|d| d.is_error()) {
                report(&a, &o, json!(null));
                exit(1)
            }
            let empty = Default::default();
            let v = o.report.as_ref().map(|r| &r.verdicts).unwrap_or(&empty);
            print!("{}", aslang::codegen::generate(o.module.as_ref().unwrap(), &o.src, v));
        }
        "build" | "run" => {
            let o = analyse(&a, true);
            let ok = o.diags.iter().all(|d| !d.is_error());
            if !ok {
                report(&a, &o, json!(null));
                exit(1)
            }
            let out = match (&a.cmd[..], &a.out) {
                (_, Some(p)) => PathBuf::from(p),
                ("run", None) => std::env::temp_dir().join(format!("aslang_run_{}", std::process::id())),
                _ => PathBuf::from(Path::new(&a.file).file_stem().unwrap()),
            };
            if let Err(e) = build(&a, &o, &out) {
                eprintln!("error: {e}");
                exit(1)
            }
            if a.cmd == "build" {
                report(&a, &o, json!({ "binary": out }));
            } else {
                if !a.json {
                    report(&a, &o, json!(null));
                }
                let status = Command::new(&out).status();
                if a.out.is_none() {
                    let _ = std::fs::remove_file(&out);
                }
                exit(status.map(|s| s.code().unwrap_or(1)).unwrap_or(1))
            }
        }
        "lock" => {
            let o = analyse(&a, false);
            if o.diags.iter().any(|d| d.is_error()) {
                report(&a, &o, json!(null));
                exit(1)
            }
            let path = lock_path(&a.file);
            let mut root: serde_json::Value = std::fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(json!({ "version": 1, "files": {} }));
            root["files"][file_key(&a.file)] = aslang::lock::snapshot(o.module.as_ref().unwrap());
            std::fs::write(&path, serde_json::to_string_pretty(&root).unwrap() + "\n").unwrap_or_else(|e| {
                eprintln!("cannot write {}: {e}", path.display());
                exit(1)
            });
            let n = o.module.as_ref().unwrap().funcs.iter().filter(|f| f.is_pub).count();
            report(&a, &o, json!({ "lock": path, "pinned_functions": n }));
            if !a.json {
                eprintln!("pinned {n} public function(s) in {}", path.display());
            }
        }
        other => {
            eprintln!("unknown command `{other}`\n\n{USAGE}");
            exit(2)
        }
    }
}
