"""Token usage and an access audit from a sub-agent's JSONL transcript (never printed whole)."""
import json, os, re, sys

# macOS hosts: the repo sits under /Volumes or /Users, /tmp is /private/tmp, and /work is a
# symlink into the home directory, so the audit needs those roots too. A relative path into the
# repo (the agent's starting directory) shows up as one of these markers.
REPO_MARKERS = ("eval/private", "eval/gold", "eval/solutions", "tasks.py", "lang.as")


def usage(path, workdir):
    calls = {}
    order = []
    tool_calls = 0
    outside = []
    stamps = []
    models = set()
    real = os.path.realpath(workdir)
    for line in open(path):
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("timestamp"):
            stamps.append(d["timestamp"])
        msg = d.get("message") or {}
        if msg.get("model"):
            models.add(msg["model"])
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
                for p in re.findall(r"(/(?:home|tmp|root|opt|work|usr|etc|Users|Volumes|private|var)[\w./-]*)", blob):
                    tmp = p.startswith(("/tmp/", "/private/tmp/", "/var/folders/", "/private/var/folders/"))
                    allowed = p.startswith((workdir, real)) or p.startswith("/opt/touchmark") or p.startswith("/usr/local/bin/tmk") or (tmp and "evalprivate" not in p and "lang.as" not in p and "/claude-" not in p)
                    if not allowed:
                        outside.append(p)
                outside += [m for m in REPO_MARKERS if m in blob]
    prompts = [calls[m] for m in order]
    out = {"model_calls": len(prompts), "T_ctx": prompts[-1] if prompts else 0, "T_proc": sum(prompts),
           "overhead": prompts[0] if prompts else 0, "tool_calls": tool_calls, "outside_paths": sorted(set(outside))[:20],
           "models": sorted(models)}
    out["T_work"] = out["T_ctx"] - out["overhead"]
    if len(stamps) >= 2:
        from datetime import datetime
        f = lambda s: datetime.fromisoformat(s.replace("Z", "+00:00"))
        out["seconds"] = round((f(stamps[-1]) - f(stamps[0])).total_seconds(), 1)
    return out


def causes(path):
    """Where a run's tokens went: o200k tokens of each tool call's input and result, grouped by
    what the call did, plus how many compiler runs reported an error or an unproved check.
    Counts only; no transcript content leaves this function."""
    import tiktoken
    enc = tiktoken.get_encoding("o200k_base")
    size = lambda s: len(enc.encode(s, disallowed_special=()))
    kind = {}
    tokens = {"reference": 0, "compiler": 0, "writing": 0, "other": 0, "prose": 0, "thinking": 0}
    compiler = {"runs": 0, "with_error": 0, "with_unproved": 0}
    for line in open(path):
        try:
            msg = json.loads(line).get("message") or {}
        except Exception:
            continue
        content = msg.get("content")
        if not isinstance(content, list):
            continue
        for c in content:
            if not isinstance(c, dict):
                continue
            if c.get("type") == "tool_use":
                blob = json.dumps(c.get("input", {}))
                cmd = (c.get("input") or {}).get("command", "")
                if "llms.txt" in blob:
                    k = "reference"
                elif c.get("name") == "Bash" and re.search(r"\b(tmk|rustc|cargo)\b", cmd):
                    k = "compiler"
                elif c.get("name") in ("Write", "Edit", "MultiEdit", "NotebookEdit"):
                    k = "writing"
                else:
                    k = "other"
                kind[c.get("id")] = k
                tokens[k] += size(blob)
            elif c.get("type") == "tool_result":
                k = kind.get(c.get("tool_use_id"), "other")
                body = c.get("content")
                text = body if isinstance(body, str) else " ".join(b.get("text", "") for b in body or [] if isinstance(b, dict))
                tokens[k] += size(text)
                if k == "compiler":
                    compiler["runs"] += 1
                    compiler["with_error"] += bool(re.search(r"error(\[E\d+\])?:|error\[E\d+\]", text))
                    compiler["with_unproved"] += "W0250" in text
            elif c.get("type") == "text":
                tokens["prose"] += size(c.get("text", ""))
            elif c.get("type") == "thinking":
                tokens["thinking"] += size(c.get("thinking", ""))
    return {"tokens": tokens, "compiler": compiler}


if __name__ == "__main__":
    if sys.argv[1] == "--causes":
        print(json.dumps(causes(sys.argv[2]), indent=1))
    else:
        print(json.dumps(usage(sys.argv[1], sys.argv[2]), indent=1))
