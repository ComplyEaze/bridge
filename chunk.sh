#!/usr/bin/env bash
# Scratch helper (not for merging): runs the next CHUNK mutation ids that no committed result
# file in this branch covers, on ../e4, then commits and pushes the result. One chunk per call,
# so each call ends inside a short command time limit. Usage: chunk.sh [CHUNK]
set -uo pipefail
S=/tmp/claude-0/-home-user-bridge/077ab0a6-a615-50d6-85c8-0417896d1f5b/scratchpad
W="$S/shards-644e"; BR=cloud/lane-e-644-shards-5; C="$S/e4/src-tauri/crates/bridge-tax-audit"; N=${1:-30}
export PATH="$(dirname "$(rustup which --toolchain 1.96.0 rustc)"):$PATH"
export RUSTC="$(rustup which --toolchain 1.96.0 rustc)" RUSTDOC="$(rustup which --toolchain 1.96.0 rustdoc)"
IDS=$(cd "$W" && python3 - "$C" "$N" <<'PY'
import json, subprocess, sys
c, n = sys.argv[1], int(sys.argv[2])
done = set()
for f in subprocess.run(["git","ls-files","*.json"],capture_output=True,text=True).stdout.split():
    if not (f.startswith("shard-") or f.startswith("chunk-")): continue
    d = json.load(open(f)); r = d.get("records", d) if isinstance(d, dict) else {}
    done |= set(r) if isinstance(r, dict) else set()
left = [m["id"] for m in json.load(open(c + "/parity/mutations.json")) if m["id"] not in done]
print(" ".join(left[:n]))
PY
)
[ -z "$IDS" ] && { echo "all done $(date -u +%T)" >> "$S/shards-644e.log"; exit 0; }
K=$(printf '%s' "$IDS" | md5sum | cut -c1-8)
echo "chunk $K start $(date -u +%T): $(echo $IDS | wc -w) ids" >> "$S/shards-644e.log"
( cd "$S/e4" && python3 "$C/parity/mutations.py" --full --jobs 4 --results "$W/chunk-$K.json" $IDS > "$W/chunk-$K.log" 2>&1 )
echo "chunk $K rc=$? end $(date -u +%T)" >> "$S/shards-644e.log"
( cd "$W" && git add -f "chunk-$K.json" "chunk-$K.log" && git commit -qm "644 full run at $(git -C "$S/e4" log -1 --format=%h): chunk $K

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_01BDtFvVBFVAsRhKf11SwKmb" && \
  for d in 2 4 8; do git push -q origin "$BR" && break || sleep $d; done && echo "pushed $K $(date -u +%T)" ) >> "$S/shards-644e.log" 2>&1
