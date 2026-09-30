"""Prompts, test programs and scoring for the agent evaluation."""

import json, os, re, subprocess, sys, tempfile
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from tasks import TASKS, MIN, MAX

TMK = "/usr/local/bin/tmk"
REF = "/opt/touchmark/llms.txt"
BY_NAME = {t["name"]: t for t in TASKS}

TY = {
    "touchmark": {"int": "int", "ints": "[int]", "opt": "int?"},
    "rust": {"int": "i64", "ints": "&[i64]", "opt": "Option<i64>"},
}
RET = {
    "touchmark": {"int": "int", "ints": "[int]", "opt": "int?"},
    "rust": {"int": "i64", "ints": "Vec<i64>", "opt": "Option<i64>"},
}
EXT = {"touchmark": "tmk", "rust": "rs"}


def signature(t, lang):
    ps = ", ".join(f"{n}: {TY[lang][ty]}" for n, ty in t["params"])
    return f"pub fn {t['name']}({ps}) -> {RET[lang][t['ret']]}"


def prompt(t, lang, workdir):
    if lang == "touchmark":
        tool = (f"Touchmark is a new compiled language. Its complete reference is the file {REF}; read it first. "
                f"The compiler is `tmk` (on PATH).")
        name = "Touchmark"
    else:
        tool = "Use Rust (stable, edition 2021). `rustc` and `cargo` are on PATH."
        name = "Rust"
    return f"""You are implementing one function for a production codebase, in {name}.

{tool}

Work only inside the directory {workdir} (it exists and is empty). Do not read or search files outside it{' other than the language reference' if lang == 'touchmark' else ''}.

Write your final solution to {workdir}/solution.{EXT[lang]}. It must define a public function with exactly this signature:

    {signature(t, lang)}

It may also define helper functions and types. It must not define `main`: the team calls your function from its own test program.

Specification:

{t['spec']}

Your function will be reviewed and then run against hidden tests covering the whole input domain described above, including its edge cases. It must return the specified result for every input the specification allows. You may write and run your own tests in other files inside your directory.

When you are finished, reply with the single word DONE."""


def lit(v, lang):
    if isinstance(v, list):
        if not v:
            return "[0; 0]" if lang == "touchmark" else "&Vec::<i64>::new()"
        body = ", ".join(lit(x, lang) for x in v)
        return f"[{body}]" if lang == "touchmark" else f"&[{body}]"
    if v == MIN:
        return "int.min" if lang == "touchmark" else "i64::MIN"
    return str(v)


def expected_lines(t, args):
    r = t["ref"](*args)
    if t["ret"] == "opt":
        return ["none" if r is None else str(r)]
    if t["ret"] == "ints":
        return [str(len(r))] + [str(x) for x in r]
    return [str(r)]


def call_code(t, args, lang, k):
    call = f"{t['name']}({', '.join(lit(a, lang) for a in args)})"
    if lang == "touchmark":
        if t["ret"] == "int":
            return f"  io.print({call})\n"
        if t["ret"] == "opt":
            return f"  match {call} {{\n    Some(v) => io.print(v)\n    None => io.print(\"none\")\n  }}\n"
        return f"  let r{k} = {call}\n  io.print(r{k}.len)\n  for i in 0..r{k}.len {{\n    io.print(r{k}[i])\n  }}\n"
    if t["ret"] == "int":
        return f"    println!(\"{{}}\", {call});\n"
    if t["ret"] == "opt":
        return f"    match {call} {{ Some(v) => println!(\"{{}}\", v), None => println!(\"none\") }}\n"
    return f"    let r{k} = {call};\n    println!(\"{{}}\", r{k}.len());\n    for x in r{k}.iter() {{ println!(\"{{}}\", x); }}\n"


def strip_main(src):
    """Remove a `main` function the agent may have left in, by brace matching."""
    m = re.search(r"(^|\n)\s*(pub\s+)?fn\s+main\s*\(", src)
    if not m:
        return src
    i = src.index("{", m.end())
    depth = 0
    j = i
    while j < len(src):
        if src[j] == "{":
            depth += 1
        elif src[j] == "}":
            depth -= 1
            if depth == 0:
                break
        j += 1
    return src[:m.start()] + "\n" + src[j + 1:]


def program(t, lang, src, tests):
    body = "".join(call_code(t, args, lang, k) for k, args in enumerate(tests))
    if lang == "touchmark":
        return strip_main(src) + "\n\nfn main() uses io {\n" + body + "}\n"
    return strip_main(src) + "\n\n#[allow(unused)]\nfn main() {\n" + body + "}\n"


def build(lang, path, out):
    if lang == "touchmark":
        cmd = [TMK, "build", path, "-o", out]
    else:
        cmd = ["rustc", "-O", "--edition", "2021", "-A", "warnings", path, "-o", out]
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=600)
    except subprocess.TimeoutExpired:
        return False, "build timed out"
    return r.returncode == 0, (r.stdout + r.stderr)[-3000:]


def score(t, lang, src):
    """Run every hidden test; a crash costs only the test that crashed."""
    tests = t["tests"]
    results = [None] * len(tests)
    start = 0
    build_log = ""
    with tempfile.TemporaryDirectory() as d:
        while start < len(tests):
            path = os.path.join(d, f"p{start}.{EXT[lang]}")
            out = os.path.join(d, f"b{start}")
            open(path, "w").write(program(t, lang, src, tests[start:]))
            ok, log = build(lang, path, out)
            if not ok:
                build_log = log
                for k in range(start, len(tests)):
                    results[k] = "build"
                break
            try:
                r = subprocess.run([out], capture_output=True, text=True, timeout=30)
                lines, crashed = r.stdout.splitlines(), r.returncode != 0
            except subprocess.TimeoutExpired as e:
                lines, crashed = (e.stdout or b"").decode(errors="replace").splitlines() if isinstance(e.stdout, bytes) else (e.stdout or "").splitlines(), True
            pos = 0
            k = start
            while k < len(tests):
                exp = expected_lines(t, tests[k])
                got = lines[pos:pos + len(exp)]
                if len(got) < len(exp):
                    break
                results[k] = "pass" if got == exp else "wrong"
                pos += len(exp)
                k += 1
            if k < len(tests):
                results[k] = "crash" if crashed else "wrong"
                start = k + 1
            else:
                break
    return {"passed": results.count("pass"), "total": len(tests), "results": results, "build_log": build_log if "build" in results else ""}


if __name__ == "__main__":
    # usage: harness.py score <task> <lang> <solution-file>
    _, cmd, name, lang, path = sys.argv
    print(json.dumps(score(BY_NAME[name], lang, open(path).read()), indent=1))
