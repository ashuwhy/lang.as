//! `tmk explain CODE`: what each diagnostic means and how to fix it, with a small example.

pub struct Entry {
    pub code: &'static str,
    pub title: &'static str,
    pub text: &'static str,
}

pub const ENTRIES: &[Entry] = &[
    Entry { code: "E0001", title: "cannot read the source", text: "The lexer found something it cannot turn into a token: an unknown character, an unterminated string or comment, or a number larger than 64 bits." },
    Entry { code: "E0002", title: "syntax error", text: "The parser expected something else here. Statements end at the end of a line; a line that continues must end with an operator or open bracket, or the next line must start with one." },
    Entry { code: "E0101", title: "unknown name", text: "No variable, function, type or variant has this name in scope. The fix line suggests the closest existing name. Names bound by `x is Ok(v)` exist only on the true side of `&&`, `==>` and in the `if` body." },
    Entry { code: "E0102", title: "type mismatch", text: "A value of one type is used where another is required. There are no implicit conversions.\n\n  let x: bool = 1        // error\n  let x: bool = 1 > 0    // ok" },
    Entry { code: "E0103", title: "no such field", text: "The record has no field with that name, or a record literal is missing a field. Record literals must list every field, or start with `...base` to copy the rest." },
    Entry { code: "E0104", title: "non-exhaustive match", text: "A `match` must handle every variant. Add the missing arms listed in the message, or a final `_ => ...`." },
    Entry { code: "E0105", title: "cannot infer the type", text: "The compiler cannot tell which type a record literal, `Ok`, `Err` or `None` builds. Add an annotation (`let r: Result<int, E> = ...`) or use it where the type is known, such as a return value." },
    Entry { code: "E0106", title: "effect not declared", text: "Functions are pure unless they declare effects. A function that prints, or calls a function that prints, needs `uses io`.\n\n  fn report(x: int) uses io {\n    io.print(x)\n  }" },
    Entry { code: "E0107", title: "wrong number of arguments", text: "The call or pattern gives a different number of values than the function or variant takes." },
    Entry { code: "E0108", title: "not allowed in a contract", text: "Contracts (`requires`, `ensures`, `invariant`, refinements) are pure logic: they cannot print, and in v0.1 they cannot call functions. `result` is only available in `ensures`." },
    Entry { code: "E0109", title: "defined twice", text: "Two definitions share a name. Variant names are global, so `Frozen` can belong to only one enum; this keeps every name greppable." },
    Entry { code: "E0110", title: "recursive type", text: "A type contains itself. Recursive types need heap values, which arrive in v0.2." },
    Entry { code: "E0111", title: "cannot assign", text: "Only variables declared with `var` can change. Parameters and `let` bindings are fixed.\n\n  var total = 0\n  total += 1" },
    Entry { code: "E0112", title: "record type needs a name", text: "Record types are declared once with `type Name = { ... }` and then used by name." },
    Entry { code: "E0113", title: "invalid main", text: "`main` takes no parameters and returns nothing. It may declare effects, e.g. `fn main() uses io`." },
    Entry { code: "E0114", title: "ignored Result", text: "A `Result` carries an error that must be handled. Use `match`, or bind it with `let _ = ...` if ignoring it is really intended." },
    Entry { code: "E0115", title: "number out of range", text: "`int` is a 64-bit signed integer: from int.min (-9223372036854775808) to int.max (9223372036854775807)." },
    Entry { code: "E0116", title: "cannot compare these values in code", text: "In v0.1, `==` and `!=` in code work on numbers and booleans. Compare fields, or use `match` or `is`. Contracts may compare whole values." },
    Entry { code: "E0117", title: "argument label does not match", text: "Arguments may be labelled with parameter names, `transfer(from: a, to: b, amount: 30)`. Labels are checked, so swapped arguments are an error instead of a silent bug. When only the order is wrong, the fix line shows the corrected call." },
    Entry { code: "E0118", title: "array stored inside another value", text: "In v0.2 arrays can be variables, parameters and return values, but not fields of records, payloads of enums, options, or elements of other arrays. Nesting arrives with the heap-value work in v0.3." },
    Entry { code: "E0201", title: "integer overflow possible", text: "The prover found inputs for which this arithmetic leaves the 64-bit range; the counterexample shows them. Bound the inputs with `requires`, check them first, or rewrite the computation.\n\n  (lo + hi) / 2         // overflows when lo and hi are large\n  lo + (hi - lo) / 2    // proved safe when 0 <= lo <= hi" },
    Entry { code: "E0202", title: "division by zero possible", text: "The divisor can be zero for the inputs in the counterexample. Add `requires d != 0` or handle zero before dividing." },
    Entry { code: "E0203", title: "precondition may fail at a call", text: "The arguments in the counterexample break the callee's `requires`. Check the condition before calling, or add it to the caller's own `requires` so its callers must establish it." },
    Entry { code: "E0204", title: "postcondition may not hold", text: "For the inputs in the counterexample, the function returns the shown `result`, which breaks its `ensures`. Fix the code. Weaken the `ensures` only if that is the real intent: a pinned contract cannot be weakened without an approved `tmk lock`." },
    Entry { code: "E0205", title: "refinement may not hold", text: "A value flows into a refined type (such as `nat` or `Cents`) without being proved to satisfy its `where` clause. The counterexample gives the offending value." },
    Entry { code: "E0206", title: "loop invariant does not hold on entry", text: "Before the first iteration the invariant is false for the counterexample inputs. Initialise the variables so it holds, or correct the invariant." },
    Entry { code: "E0207", title: "loop invariant not preserved", text: "Assuming the invariant and the loop condition at the top of an iteration, the body can end with the invariant false. Strengthen the invariant with what the body relies on, or fix the body. Simple bounds (ranges of counters, accumulators and array elements) are inferred automatically; `tmk check --show-inferred` lists them, so write invariants only for the properties the compiler cannot guess." },
    Entry { code: "E0208", title: "loop measure does not decrease", text: "`decreases e` promises that `e` stays at or above zero and gets smaller on every iteration, which proves the loop ends. The prover found an iteration where it does not." },
    Entry { code: "E0209", title: "vacuous precondition", text: "The `requires` clauses contradict each other, so the function can never be called and every proof about it would be empty. Remove the contradiction." },
    Entry { code: "E0210", title: "index may be out of bounds", text: "The prover found a case where the index is negative or not below the array's length; the counterexample shows it. Bound the index with the loop range (`for i in 0..a.len`), an `invariant`, or a `requires`. Proved indexes compile to plain memory accesses with no check.\n\n  for i in 0..a.len { s += a[i] }   // proved: 0 <= i < a.len" },
    Entry { code: "E0211", title: "invalid array length", text: "An array length may be negative or above the limit (2^40 elements), in `[value; n]` or `push`. Make sure `n >= 0` holds." },
    Entry { code: "E0250", title: "unproved checks with --proved", text: "`--proved` accepts only programs in which every check is proved. The listed checks would otherwise be kept at run time." },
    Entry { code: "W0250", title: "check kept at run time", text: "The prover could not decide this check within its time budget, so the compiled program checks it at run time and stops with a message if it fails. Adding a `requires`, an `invariant` or a simpler formulation usually makes it provable. Raise the budget with `--timeout`." },
    Entry { code: "W0251", title: "solver not found", text: "Z3 was not found, so nothing was proved and every check stays at run time. Install Z3 or set TMK_Z3 to its path." },
    Entry { code: "E0301", title: "contract weaker than the pinned one", text: "`touchplate.lock` pins this function's contract. The new `ensures` allows a result the pinned one forbids; the counterexample shows it. Restore the guarantee, or have an owner review the change and run `tmk lock`." },
    Entry { code: "E0302", title: "precondition stronger than the pinned one", text: "The new `requires` rejects a call that the pinned contract allowed, which can break existing callers. The counterexample shows such a call." },
    Entry { code: "E0303", title: "new effect on a pinned function", text: "The function declares an effect its pinned contract did not have, for example a pure function that now uses `net`. Remove it, or have an owner approve it and run `tmk lock`." },
    Entry { code: "E0304", title: "pinned signature changed or removed", text: "The function's parameters, return type or existence differ from `touchplate.lock`. Signature changes need an owner review and a new `tmk lock`." },
    Entry { code: "E0305", title: "pinned type changed", text: "A type used by pinned contracts changed its fields or refinement. That can silently change what every contract means, so it needs an owner review and a new `tmk lock`." },
];

pub fn explain(code: &str) -> Option<String> {
    let code = code.to_ascii_uppercase();
    ENTRIES.iter().find(|e| e.code == code).map(|e| format!("{} {}\n\n{}\n", e.code, e.title, e.text))
}
