"""Token usage and an access audit from a sub-agent's JSONL transcript (never printed whole)."""
import json, re, sys


def usage(path, workdir):
    calls = {}
    order = []
    tool_calls = 0
    outside = []
    stamps = []
    for line in open(path):
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("timestamp"):
            stamps.append(d["timestamp"])
        msg = d.get("message") or {}
        u = msg.get("usage")
        mid = msg.get("id")
        if u and mid:
            if mid not in calls:
                order.append(mid)
            calls[mid] = (u.get("input_tokens", 0) or 0) + (u.get("cache_read_input_tokens", 0) or 0) + (u.get("cache_creation_input_tokens", 0) or 0)
        for c in msg.get("content") or []:
            if isinstance(c, dict) and c.get("type") == "tool_use":
                tool_calls += 1
                blob = json.dumps(c.get("input", {}))
                for p in re.findall(r"(/(?:home|tmp|root|opt|work|usr|etc)[\w./-]*)", blob):
                    allowed = p.startswith(workdir) or p.startswith("/opt/touchmark") or p.startswith("/usr/local/bin/tmk") or (p.startswith("/tmp/") and "evalprivate" not in p and "lang.as" not in p)
                    if not allowed:
                        outside.append(p)
    prompts = [calls[m] for m in order]
    out = {"model_calls": len(prompts), "T_ctx": prompts[-1] if prompts else 0, "T_proc": sum(prompts),
           "overhead": prompts[0] if prompts else 0, "tool_calls": tool_calls, "outside_paths": sorted(set(outside))[:20]}
    out["T_work"] = out["T_ctx"] - out["overhead"]
    if len(stamps) >= 2:
        from datetime import datetime
        f = lambda s: datetime.fromisoformat(s.replace("Z", "+00:00"))
        out["seconds"] = round((f(stamps[-1]) - f(stamps[0])).total_seconds(), 1)
    return out


if __name__ == "__main__":
    print(json.dumps(usage(sys.argv[1], sys.argv[2]), indent=1))
