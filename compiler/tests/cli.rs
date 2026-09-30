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
        ("examples/arrays.as", "primes below 1000 = 168"),
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

#[test]
fn array_bugs_are_rejected() {
    let arrays = example("examples/arrays.as");
    expect_error("search_off_by_one", &arrays.replace("      hi = mid\n", "      hi = mid - 1\n"), "E0207");
    expect_error("search_no_progress", &arrays.replace("      lo = mid + 1", "      lo = mid"), "E0208");
    expect_error("sieve_overrun", &arrays.replace("      while j < n\n", "      while j <= n\n"), "E0210");
    expect_error("unsorted_input", &arrays.replace("let a = [1, 3, 5, 7, 9, 11]", "let a = [1, 3, 5, 7, 2, 11]"), "E0203");
    expect_error("loop_too_far", &arrays.replace("  for i in 2..n {", "  for i in 2..n + 1 {"), "E0210");
    expect_error("quantifier_in_code", "fn f(a: [int]) -> bool {\n  forall i in 0..a.len: a[i] > 0\n}\n", "E0108");
    expect_error("array_in_record", "type Bag = { items: [int] }\n", "E0118");
}

#[test]
fn arrays_are_memory_safe_under_asan() {
    // Build the generated C with AddressSanitizer: no leaks, no use-after-free, no overflow.
    let (code, c, err) = aslang(&["emit-c", "examples/arrays.as"], &root());
    assert_eq!(code, 0, "{err}");
    let dir = std::env::temp_dir().join(format!("aslang_asan_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("a.c"), c).unwrap();
    let cc = Command::new("gcc").args(["-g", "-fsanitize=address,undefined", "-std=gnu11", "-w", "-o"]).arg(dir.join("a")).arg(dir.join("a.c")).status();
    if !cc.map(|s| s.success()).unwrap_or(false) {
        eprintln!("skipping: gcc with AddressSanitizer is not available");
        return;
    }
    let out = Command::new(dir.join("a")).env("ASAN_OPTIONS", "detect_leaks=1").output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(String::from_utf8_lossy(&out.stdout).contains("primes below 1000 = 168"));
}

#[test]
fn inference_removes_bound_invariants_but_never_hides_bugs() {
    // No invariants written: the compiler infers the bounds it needs.
    let ok = "fn main() uses io {\n  var a = [0; 1_000]\n  var x = 7\n  for i in 0..a.len {\n    x = (x * 1_103 + 12_345) % 1_000_003\n    a[i] = x\n  }\n  var s = 0\n  for i in 0..a.len {\n    s += a[i]\n  }\n  io.print(s)\n}\n";
    let dir = scratch("inferred", ok);
    let (code, _, err) = aslang(&["check", "--show-inferred", "inferred.as"], &dir);
    assert_eq!(code, 0, "{err}");
    assert!(err.contains("invariant x < 1_000_003"), "{err}");

    // Nothing bounds the elements here, so the sum really can overflow: inference must not
    // make this pass.
    let bad = "fn total(a: [int]) -> int {\n  var s = 0\n  for i in 0..a.len {\n    s += a[i]\n  }\n  s\n}\n";
    expect_error("unbounded_sum", bad, "E0201");

    // Without inference the same good program needs hand-written invariants.
    let (code, _, _) = aslang(&["check", "--no-infer", "inferred.as"], &dir);
    assert_eq!(code, 1);
}

#[test]
fn contracts_of_private_functions_are_inferred_from_their_calls() {
    // No contracts written: `get` is only called with an index in bounds, and `fill` returns
    // an array as long as asked for; both facts are inferred and proved.
    let ok = "fn fill(n: int) -> [int] {\n  var a = [0; n]\n  for i in 0..n {\n    a[i] = i % 10\n  }\n  a\n}\n\nfn get(a: [int], i: int) -> int {\n  a[i]\n}\n\nfn main() uses io {\n  let a = fill(5)\n  io.print(get(a, 4) + get(a, 0))\n}\n";
    let dir = scratch("contracts", ok);
    let (code, _, err) = aslang(&["check", "--show-inferred", "contracts.as"], &dir);
    assert_eq!(code, 0, "{err}");
    assert!(err.contains("ensures result.len == n"), "{err}");
    assert!(err.contains("inferred for `get`"), "{err}");
    let (code, out, err) = aslang(&["run", "contracts.as"], &dir);
    assert_eq!(code, 0, "{err}");
    assert_eq!(out.trim(), "4");

    // One call passes an index one past the end: no inferred contract can hide that.
    expect_error("past_end", &ok.replace("get(a, 0)", "get(a, 5)"), "E0210");
    // A result the caller cannot safely add twice.
    let big = "fn pick(x: int) -> int {\n  if x > 0 { return x }\n  9_000_000_000_000_000_000\n}\n\nfn main() uses io {\n  io.print(pick(0) + pick(0))\n}\n";
    expect_error("big_result", big, "E0201");
    let (code, _, err) = aslang(&["check", "fine.as"], &scratch("fine", &big.replace("pick(0)", "pick(5)")));
    assert_eq!(code, 0, "{err}");
    // A public function is API: its callers here say nothing about callers elsewhere.
    expect_error("public", &ok.replace("fn get", "pub fn get"), "E0210");
}
