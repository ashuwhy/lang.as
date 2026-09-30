#!/usr/bin/env bash
# Reproduces every probe in phase0/lemmascript_assessment.md.
#
# Needs on PATH: node >= 22, git, dafny 4.11.x, and the npm packages
#   lemmascript@0.6.4 lemmascript-guard@0.1.0 lemmascript-crosscheck@0.1.0
# (for example: npm install lemmascript@0.6.4 lemmascript-guard lemmascript-crosscheck
#  and export PATH="$PWD/node_modules/.bin:$PATH").
#
# Usage: ./run.sh   (works in a scratch copy; the committed probe files are not modified)
set -u
here="$(cd "$(dirname "$0")" && pwd)"
work="$(mktemp -d)"
cp "$here"/*.ts "$here"/*.mts "$work"/
cd "$work"

row() { printf '%-44s %s\n' "$1" "$2"; }
verdict() {  # file -> "exit=N: <last verifier line>"
  lsc regen --backend=dafny "$1" > "$1.out" 2>&1; local rc=$?
  row "$1" "exit=$rc: $(grep -E 'verified|ERROR|Error:' "$1.out" | head -1)"
}

echo "== E1 baseline and latency"
t0=$(date +%s%N); verdict ledger.ts; t1=$(date +%s%N)
row "  wall time" "$(( (t1 - t0) / 1000000 )) ms"
cp ledger.ts ledger.pinned.ts; cp ledger.dfy ledger.pinned.dfy; cp ledger.dfy.gen ledger.pinned.dfy.gen

echo "== E2 contract weakening at the TypeScript layer"
sed -i 's/balance: to.balance + amount }/balance: to.balance + amount - 1 }/' ledger.ts
verdict ledger.ts                                       # E2a: bug alone must fail
sed -i 's/ \&\& \\result.to.balance === to.balance + amount$//' ledger.ts
verdict ledger.ts                                       # E2b: bug + weakened //@ ensures
row "  lines deleted from generated Dafny" "$(git diff --no-index ledger.dfy.gen ledger.dfy | grep -c '^-[^-]')"

echo "== E3 additions-only proof edits that hide the same bug"
cp ledger.pinned.ts ledger.ts; cp ledger.pinned.dfy ledger.dfy; cp ledger.pinned.dfy.gen ledger.dfy.gen; rm -f ledger.dfy.base
sed -i 's/balance: to.balance + amount }/balance: to.balance + amount - 1 }/' ledger.ts
rm -f ledger.dfy ledger.dfy.gen; lsc regen --backend=dafny ledger.ts > /dev/null 2>&1   # fresh, fails
cp ledger.dfy clean.dfy
python3 - <<'EOF'
s = open("clean.dfy").read()
sig = "lemma transfer_ensures(from: Account, to: Account, amount: int)\n"
open("E3a.dfy", "w").write(s.replace(sig, sig + "  requires false\n", 1))
i = s.rindex("{\n}")
open("E3b.dfy", "w").write(s[:i] + "{\n  assume false;\n}" + s[i + 3:])
open("E3c.dfy", "w").write(s[:i] + "{\n  assume {:axiom} false;\n}" + s[i + 3:])
EOF
for v in E3a E3b E3c; do
  cp $v.dfy ledger.dfy; lsc check --backend=dafny ledger.ts > $v.out 2>&1; rc=$?
  row "  $v ($(git diff --no-index clean.dfy $v.dfy | grep '^+ ' | sed 's/^+ *//'))" "exit=$rc: $(grep -E 'verified|warnings' $v.out | tr '\n' ' ')"
done

echo "== E4-E7 escape hatches, JSON and effects inside //@ verify"
for f in launder launder2 anyparam nonnull nonnull2 jsonparse jsonauto effects effectsauto; do verdict $f.ts; done
grep -h '^autohavoc:' jsonauto.ts.out effectsauto.ts.out

echo "== E8 run time: verified code on hostile inputs, without and with lemmascript-guard"
cp ledger.pinned.ts ledger.ts
node boundary_unguarded.mts 2>&1 | grep -v -i experimental
lemmascript-guard ledger.ts > /dev/null && sed -i 's#"./ledger.js"#"./ledger.ts"#' ledger.guarded.ts
node boundary_guarded.mts 2>&1 | grep -v -i experimental

echo "== E9 lemmascript-crosscheck on transfer"
mkdir cc && cp ledger.pinned.ts cc/ledger.ts
(cd cc && lemmascript-crosscheck ledger.ts --only transfer --cases 40 2>&1 | sed -n '1,2p;/Precision loss/,/^Result/p')
echo "(scratch directory: $work)"
