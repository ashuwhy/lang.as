#!/usr/bin/env python3
"""Build each benchmark as Touchmark, Rust -O, and Rust -O with overflow checks; check that the
outputs agree; print median wall time over several runs as a Markdown table."""

import os
import statistics
import subprocess
import sys
import tempfile
import time

HERE = os.path.dirname(os.path.abspath(__file__))
TMK = os.environ.get("TMK", os.path.join(HERE, "..", "..", "target", "release", "tmk"))
RUNS = int(os.environ.get("RUNS", "5"))
BENCHES = ["primes", "collatz", "isqrt"]


def sh(*cmd, capture=True):
    r = subprocess.run(cmd, capture_output=capture, text=True)
    if r.returncode != 0:
        sys.exit(f"failed: {' '.join(cmd)}\n{r.stdout}{r.stderr}")
    return r


def timed(binary):
    times, out = [], None
    for _ in range(RUNS):
        t = time.perf_counter()
        out = sh(binary).stdout.strip()
        times.append(time.perf_counter() - t)
    return statistics.median(times) * 1000, out


def main():
    tmp = tempfile.mkdtemp()
    print(f"| benchmark | Touchmark (checks proved) | Rust -O (overflow wraps silently) | Rust -O + overflow checks | output |")
    print("|---|---|---|---|---|")
    for b in BENCHES:
        src = os.path.join(HERE, b)
        r = sh(TMK, "build", src + ".tmk", "-o", os.path.join(tmp, b + "_tmk"))
        verdict = r.stderr.strip().splitlines()[-1]
        sh("rustc", "-O", "-o", os.path.join(tmp, b + "_rs"), src + ".rs")
        sh("rustc", "-O", "-C", "overflow-checks=on", "-o", os.path.join(tmp, b + "_rschk"), src + ".rs")
        results = {v: timed(os.path.join(tmp, f"{b}_{v}")) for v in ["as", "rs", "rschk"]}
        outs = {o for _, o in results.values()}
        if len(outs) != 1:
            sys.exit(f"{b}: outputs differ: {results}")
        cell = lambda v: f"{results[v][0]:.0f} ms"
        proved = verdict.split(":", 1)[1].strip() if ":" in verdict else verdict
        print(f"| {b} | {cell('as')} ({proved}) | {cell('rs')} | {cell('rschk')} | `{outs.pop()}` |")


if __name__ == "__main__":
    main()
