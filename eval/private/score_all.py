"""Score every finished run that has not been scored yet; results go to results.json."""
import json, os, sys
import harness as h
from usage import usage
T = "/private/tmp/claude-501/-Volumes-part-one-Coding-Projects-lang-as/81c3387e-5192-4c2d-aca4-3fa6920cf370/tasks"
runs = json.load(open("runs.json"))
res = json.load(open("results.json")) if os.path.exists("results.json") else {}
only = set(sys.argv[1:])
for r in runs:
    if not r["agent"] or (only and r["id"] not in only) or (r["id"] in res and not only):
        continue
    sol = os.path.join(r["dir"], "solution." + h.EXT[r["lang"]])
    rec = {"task": r["task"], "lang": r["lang"], "trial": r["trial"]}
    rec.update(usage(os.path.join(T, r["agent"] + ".output"), r["dir"]))
    if os.path.exists(sol):
        s = h.score(h.BY_NAME[r["task"]], r["lang"], open(sol).read())
        rec.update({"passed": s["passed"], "total": s["total"], "results": s["results"], "build_log": s["build_log"][:1500]})
    else:
        rec.update({"passed": 0, "total": len(h.BY_NAME[r["task"]]["tests"]), "results": ["missing"], "build_log": "no solution file"})
    res[r["id"]] = rec
    print(r["id"], rec["passed"], "/", rec["total"], "T_ctx", rec["T_ctx"], "T_work", rec["T_work"], "calls", rec["tool_calls"], "outside", rec["outside_paths"])
json.dump(res, open("results.json", "w"), indent=1)
