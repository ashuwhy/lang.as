//! End-to-end tests: verified examples must build and run; seeded bugs must be rejected
//! with the right error code. These need `z3` and a C compiler on PATH.

use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf()
}

fn aslang(args: &[&str], dir: &Path) -> (i32, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_aslang")).args(args).current_dir(dir).output().unwrap();
    (out.status.code().unwrap_or(-1), String::from_utf8_lossy(&out.stdout).into(), String::from_utf8_lossy(&out.stderr).into())
}

fn scratch(name: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("aslang_test_{}_{name}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join(format!("{name}.as")), src).unwrap();
    dir
}

fn example(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).unwrap()
}

#[test]
fn verified_examples_run() {
    for (file, expect) in [
        ("examples/ledger.as", "moved, new balance: 30"),
        ("examples/isqrt.as", "isqrt(99) = 9"),
        ("examples/sum.as", "sum_to(1000) = 500500"),
    ] {
        let (code, out, err) = aslang(&["run", file], &root());
        assert_eq!(code, 0, "{file}: {err}");
        assert!(out.contains(expect), "{file}: {out}");
        assert!(err.contains("0 kept at run time"), "{file}: {err}");
    }
}

fn expect_error(name: &str, src: &str, code: &str) {
    let dir = scratch(name, src);
    let (status, _, err) = aslang(&["check", &format!("{name}.as")], &dir);
    assert_eq!(status, 1, "{name} should fail:\n{err}");
    assert!(err.contains(&format!("error[{code}]")), "{name}: expected {code}, got:\n{err}");
}

#[test]
fn seeded_bugs_are_rejected() {
    let ledger = example("examples/ledger.as");
    let sum = example("examples/sum.as");
    let isqrt = example("examples/isqrt.as");
    expect_error("off_by_one", &ledger.replace("balance: to.balance + amount }", "balance: to.balance + amount - 1 }"), "E0204");
    expect_error("missing_bound", &ledger.replace("  requires to.balance + amount <= 1_000_000_000_000\n", ""), "E0205");
    expect_error("same_account", &ledger.replace("transfer(from: a, to: b, amount: 30)", "transfer(from: a, to: a, amount: 30)"), "E0203");
    expect_error("vacuous", &ledger.replace("requires from.id != to.id", "requires from.id != to.id && from.id == to.id"), "E0209");
    expect_error("bad_invariant", &sum.replace("invariant 2 * total == i * (i + 1)", "invariant 2 * total == i * i"), "E0207");
    expect_error("bad_measure", &sum.replace("decreases n - i", "decreases i"), "E0208");
    expect_error("bad_search", &isqrt.replace("if mid * mid <= n", "if mid * mid < n"), "E0207");
    expect_error("midpoint", &example("examples/bugs/midpoint.as"), "E0201");
    expect_error("effects", &example("examples/bugs/effects.as"), "E0106");
    expect_error("swapped_args", &ledger.replace("transfer(from: a, to: b, amount: 30)", "transfer(to: b, from: a, amount: 30)"), "E0117");
    expect_error("nat_underflow", "pub fn g(x: nat) -> nat {\n  x - 1\n}\n", "E0205");
}

#[test]
fn lock_rejects_weakened_contract() {
    let dir = scratch("locked", &example("examples/ledger.as"));
    let (code, _, err) = aslang(&["lock", "locked.as"], &dir);
    assert_eq!(code, 0, "{err}");
    let weakened = example("examples/ledger.as")
        .replace("ensures result is Ok(m) ==> m.to.balance == to.balance + amount", "ensures result is Ok(m) ==> m.to.balance <= to.balance + amount");
    std::fs::write(dir.join("locked.as"), &weakened).unwrap();
    let (code, _, err) = aslang(&["check", "locked.as"], &dir);
    assert_eq!(code, 1);
    assert!(err.contains("error[E0301]"), "{err}");

    // Adding an effect to a pinned function is also caught.
    let effectful = example("examples/ledger.as").replace("-> Result<Moved, TransferError>\n", "-> Result<Moved, TransferError>\n  uses io\n");
    std::fs::write(dir.join("locked.as"), &effectful).unwrap();
    let (_, _, err) = aslang(&["check", "locked.as"], &dir);
    assert!(err.contains("error[E0303]"), "{err}");
}

#[test]
fn unproved_checks_stay_at_run_time() {
    let src = "fn half(x: int) -> int {\n  x / 2\n}\n\nfn add(a: int, b: int) -> int {\n  a + b\n}\n\nfn main() uses io {\n  io.print(half(7))\n  io.print(add(int.max, 1))\n}\n";
    let dir = scratch("runtime", src);
    let (code, _, err) = aslang(&["check", "runtime.as"], &dir);
    assert_eq!(code, 1, "the overflow in `add` has a counterexample:\n{err}");
    let (code, out, err) = aslang(&["run", "--no-verify", "runtime.as"], &dir);
    assert_eq!(code, 101, "{err}");
    assert!(out.starts_with("3\n"), "{out}");
    assert!(err.contains("run-time check failed: integer overflow"), "{err}");
}

#[test]
fn json_output() {
    let (code, out, _) = aslang(&["check", "--json", "examples/bugs/midpoint.as"], &root());
    assert_eq!(code, 1);
    let v: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(v["ok"], false);
    let d = &v["diagnostics"][0];
    assert_eq!(d["code"], "E0201");
    assert!(d["fix"].is_string());
    assert!(!d["counterexample"].as_array().unwrap().is_empty());
}

#[test]
fn explain_knows_every_code_it_emits() {
    let codes = ["E0101", "E0104", "E0106", "E0117", "E0201", "E0204", "E0209", "E0301", "E0303", "W0250"];
    for c in codes {
        let (status, out, _) = aslang(&["explain", c], &root());
        assert_eq!(status, 0, "{c}");
        assert!(out.starts_with(c), "{out}");
    }
}
