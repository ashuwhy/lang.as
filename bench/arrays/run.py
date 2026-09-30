#!/usr/bin/env python3
"""Array benchmarks: AS against C, C++, Rust, Go, Java, JavaScript and Python.

Every program implements the same algorithm and must print the same answer. Each language is
built with its usual release settings. Reports the median wall time, peak memory (RSS) and
source size in tokens (o200k, if the tokenizer from bench/tokens is installed).

    RUNS=5 python3 bench/arrays/run.py [benchmark ...]
"""

import os
import shutil
import statistics
import subprocess
import sys
import tempfile
import time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
ASLANG = os.environ.get("ASLANG", os.path.join(ROOT, "target", "release", "aslang"))
RUNS = int(os.environ.get("RUNS", "5"))
SLOW_TIMEOUT = int(os.environ.get("SLOW_TIMEOUT", "120"))
BENCHES = ["sieve", "matmul", "quicksort", "sum"]
TMP = tempfile.mkdtemp()


def java_class(b):
    return b[0].upper() + b[1:]


# language: (source file pattern, build command or None, run command)
LANGS = {
    "AS": ("{b}.as", [ASLANG, "build", "{src}", "-o", "{bin}"], ["{bin}"]),
    "C": ("{b}.c", ["gcc", "-O2", "-o", "{bin}", "{src}"], ["{bin}"]),
    "C++": ("{b}.cpp", ["g++", "-O2", "-o", "{bin}", "{src}"], ["{bin}"]),
    "Rust": ("{b}.rs", ["rustc", "-C", "opt-level=3", "-o", "{bin}", "{src}"], ["{bin}"]),
    "Go": ("{b}.go", ["go", "build", "-o", "{bin}", "{src}"], ["{bin}"]),
    "Java": ("{J}.java", ["javac", "-d", "{dir}", "{src}"], ["java", "-cp", "{dir}", "{J}"]),
    "JavaScript": ("{b}.js", None, ["node", "{src}"]),
    "Python": ("{b}.py", None, ["python3", "{src}"]),
}


def fmt(parts, **kw):
    return [p.format(**kw) for p in parts]


def run_once(cmd, timeout):
    t = time.perf_counter()
    p = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True)
    try:
        out, _ = p.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        p.kill()
        p.communicate()
        return None, None
    return time.perf_counter() - t, out.strip()


def tokens(path):
    tok_dir = os.path.join(ROOT, "bench", "tokens", "node_modules")
    if not os.path.isdir(tok_dir):
        return None
    script = "const fs=require('fs');import('gpt-tokenizer/encoding/o200k_base').then(m=>console.log(m.encode(fs.readFileSync(process.argv[1],'utf8')).length))"
    r = subprocess.run(["node", "-e", script, path], capture_output=True, text=True, cwd=os.path.join(ROOT, "bench", "tokens"))
    return int(r.stdout.strip()) if r.returncode == 0 and r.stdout.strip() else None


def peak_rss_kb(cmd, timeout):
    """Run once more to read the process's peak resident memory (ru_maxrss, in KB)."""
    pid = os.fork()
    if pid == 0:
        devnull = os.open(os.devnull, os.O_WRONLY)
        os.dup2(devnull, 1)
        os.dup2(devnull, 2)
        try:
            os.execvp(cmd[0], cmd)
        finally:
            os._exit(127)
    deadline = time.time() + timeout
    while True:
        wpid, status, usage = os.wait4(pid, os.WNOHANG)
        if wpid:
            return usage.ru_maxrss
        if time.time() > deadline:
            os.kill(pid, 9)
            os.wait4(pid, 0)
            return None
        time.sleep(0.01)


def main():
    benches = sys.argv[1:] or BENCHES
    for b in benches:
        rows = []
        answers = {}
        for lang, (pat, build, run) in LANGS.items():
            if lang in ("Java",) and not shutil.which("java"):
                continue
            src = os.path.join(HERE, pat.format(b=b, J=java_class(b)))
            if not os.path.exists(src):
                continue
            d = os.path.join(TMP, f"{b}_{lang}".replace("+", "p"))
            os.makedirs(d, exist_ok=True)
            binp = os.path.join(d, "prog")
            kw = dict(src=src, bin=binp, dir=d, J=java_class(b))
            note = ""
            if build:
                r = subprocess.run(fmt(build, **kw), capture_output=True, text=True)
                if r.returncode != 0:
                    rows.append((lang, None, None, None, "build failed"))
                    continue
                if lang == "AS":
                    note = r.stderr.strip().splitlines()[-1].split(":", 1)[-1].strip()
            cmd = fmt(run, **kw)
            slow = lang == "Python"
            runs = 1 if slow else RUNS
            times, out = [], None
            for _ in range(runs):
                t, o = run_once(cmd, SLOW_TIMEOUT if slow else 600)
                if t is None:
                    times = None
                    break
                times.append(t)
                out = o
            if times is None:
                rows.append((lang, None, None, tokens(src), f"over {SLOW_TIMEOUT} s"))
                continue
            answers[lang] = out
            rss = peak_rss_kb(cmd, 600)
            rows.append((lang, statistics.median(times) * 1000, rss, tokens(src), note))
        if len(set(answers.values())) > 1:
            print(f"!! {b}: outputs differ: {answers}")
        print(f"\n### {b}  (answer `{next(iter(answers.values()), '?')}`)\n")
        print("| Language | Time | Peak memory | Source tokens | Notes |")
        print("|---|---|---|---|---|")
        base = next((r[1] for r in rows if r[0] == "C" and r[1]), None)
        for lang, ms, rss, tok, note in sorted(rows, key=lambda r: r[1] if r[1] else 1e18):
            t = f"{ms:.0f} ms" + (f" ({ms / base:.2f}x C)" if base and ms else "") if ms else "-"
            m = f"{rss / 1024:.0f} MB" if rss else "-"
            print(f"| {lang} | {t} | {m} | {tok if tok else '-'} | {note} |")


if __name__ == "__main__":
    main()
