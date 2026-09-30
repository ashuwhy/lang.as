"""The numbers in eval/RESULTS.md, from runs.json, results.json and the committed solutions."""
import json, os, statistics as st
import tiktoken
from scipy.stats import fisher_exact

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..")
runs = json.load(open("runs.json"))
res = json.load(open("results.json"))
enc = tiktoken.get_encoding("o200k_base")
EXT = {"touchmark": "tmk", "rust": "rs"}
NAME = {"touchmark": "Touchmark", "rust": "Rust"}


def sol_tokens(r):
    p = os.path.join(ROOT, "solutions", r["id"], "solution." + EXT[r["lang"]])
    return len(enc.encode(open(p).read())) if os.path.exists(p) else 0


rows = []
for r in runs:
    x = res.get(r["id"])
    if x is None:
        continue
    rows.append({**x, "id": r["id"], "rerun": "rerun" in r, "defect": x["passed"] < x["total"], "sol": sol_tokens(r)})


def summary(rs):
    return {
        "runs": len(rs),
        "defects": sum(r["defect"] for r in rs),
        "passed": sum(r["passed"] for r in rs),
        "total": sum(r["total"] for r in rs),
        "work": st.mean(r["T_work"] for r in rs),
        "ctx": st.mean(r["T_ctx"] for r in rs),
        "proc": st.mean(r["T_proc"] for r in rs),
        "calls": st.mean(r["tool_calls"] for r in rs),
        "wall": st.median(r["seconds"] for r in rs),
        "sol": st.mean(r["sol"] for r in rs),
    }


by = {lang: summary([r for r in rows if r["lang"] == lang]) for lang in ("touchmark", "rust")}
print("| Arm | Runs | Runs with a defect | Hidden tests passed | T_work mean | T_ctx mean | T_proc mean | Tool calls mean | Wall s median | Solution tokens mean |")
print("|---|---|---|---|---|---|---|---|---|---|")
for lang, s in by.items():
    print(f"| {NAME[lang]} | {s['runs']} | {s['defects']} | {s['passed']}/{s['total']} | {s['work']:,.0f} | {s['ctx']:,.0f} | {s['proc']:,.0f} | {s['calls']:.1f} | {s['wall']:.1f} | {s['sol']:.0f} |")
t, r = by["touchmark"], by["rust"]
_, p = fisher_exact([[t["defects"], t["runs"] - t["defects"]], [r["defects"], r["runs"] - r["defects"]]])
print(f"\nFisher two-sided: {t['defects']}/{t['runs']} vs {r['defects']}/{r['runs']}: p = {p:.3g}")
print(f"ratios: T_work {t['work'] / r['work']:.1f}x, T_proc {t['proc'] / r['proc']:.1f}x, calls {t['calls'] / r['calls']:.1f}x, wall {t['wall'] / r['wall']:.1f}x, solution {t['sol'] / r['sol']:.1f}x")

print("\n| Task | TMK runs | TMK tests | TMK T_work | RS runs | RS tests | RS T_work |")
print("|---|---|---|---|---|---|---|")
tasks = []
for r0 in runs:
    if r0["task"] not in tasks:
        tasks.append(r0["task"])
for task in tasks:
    cells = []
    for lang in ("touchmark", "rust"):
        rs = [x for x in rows if x["task"] == task and x["lang"] == lang]
        cells += [str(len(rs)), f"{sum(x['passed'] for x in rs)}/{sum(x['total'] for x in rs)}", f"{st.mean(x['T_work'] for x in rs):,.0f}"]
    print(f"| {task} | " + " | ".join(cells) + " |")

print("\nFirst session against the re-run (T_work mean, runs):")
for rerun in (False, True):
    for lang in ("touchmark", "rust"):
        rs = [x for x in rows if x["lang"] == lang and x["rerun"] == rerun]
        print(f"  {'re-run' if rerun else 'first '} {NAME[lang]:9} n={len(rs):2} T_work {st.mean(x['T_work'] for x in rs):8,.0f}  calls {st.mean(x['tool_calls'] for x in rs):4.1f}  wall median {st.median(x['seconds'] for x in rs):7.1f}")
for task in tasks:
    pair = [x for x in rows if x["task"] == task and x["rerun"]]
    if len({x["lang"] for x in pair}) == 2:
        tw = {x["lang"]: x["T_work"] for x in pair}
        print(f"  re-run pair {task}: Touchmark {tw['touchmark']:,} / Rust {tw['rust']:,} = {tw['touchmark'] / tw['rust']:.1f}x")
outside = {x["id"]: x["outside_paths"] for x in rows if x.get("outside_paths")}
print("\nflagged paths:", json.dumps(outside, indent=1))
print("models:", sorted({m for x in rows for m in x.get("models", [])}), "recorded for", sum(1 for x in rows if x.get("models")), "runs")

# Where the tokens went, for the runs whose transcripts this session still has (read via usage.py).
from usage import causes
T = "/private/tmp/claude-501/-Volumes-part-one-Coding-Projects-lang-as/81c3387e-5192-4c2d-aca4-3fa6920cf370/tasks"
print("\n| Run | Reference | Compiler runs (in+out) | Writing | Other tools | Prose | Compiler runs | with an error | with W0250 |")
print("|---|---|---|---|---|---|---|---|---|")
for r in runs:
    p = os.path.join(T, (r["agent"] or "") + ".output")
    if not r.get("rerun") or not os.path.exists(p):
        continue
    c = causes(p)
    t, k = c["tokens"], c["compiler"]
    print(f"| {r['id']} | {t['reference']:,} | {t['compiler']:,} | {t['writing']:,} | {t['other']:,} | {t['prose']:,} | {k['runs']} | {k['with_error']} | {k['with_unproved']} |")
