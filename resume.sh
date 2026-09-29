#!/usr/bin/env bash
# Scratch helper (not for merging): runs every k-of-N shard missing from this branch on the
# worktree ../e4, pushing each result as it finishes. Usage: resume.sh N
set -uo pipefail
S=/tmp/claude-0/-home-user-bridge/077ab0a6-a615-50d6-85c8-0417896d1f5b/scratchpad
N=$1; W="$S/shards-644d"; BR=cloud/lane-e-644-shards-4; SHA=$(git -C "$S/e4" log -1 --format=%h)
export PATH="$(dirname "$(rustup which --toolchain 1.96.0 rustc)"):$PATH"
export RUSTC="$(rustup which --toolchain 1.96.0 rustc)" RUSTDOC="$(rustup which --toolchain 1.96.0 rustdoc)"
for k in $(seq 1 "$N"); do
  [ -f "$W/shard-$k-of-$N.json" ] && git -C "$W" ls-files --error-unmatch "shard-$k-of-$N.json" >/dev/null 2>&1 && continue
  echo "shard $k/$N start $(date -u +%T)" >> "$S/shards-644d.log"
  ( cd "$S/e4" && python3 src-tauri/crates/bridge-tax-audit/parity/mutations.py --full --jobs 4 --shard "$k/$N" \
    --results "$W/shard-$k-of-$N.json" > "$W/shard-$k-of-$N.log" 2>&1 )
  echo "shard $k/$N rc=$? end $(date -u +%T)" >> "$S/shards-644d.log"
  ( cd "$W" && git add -f "shard-$k-of-$N.json" "shard-$k-of-$N.log" && \
    git commit -qm "644 full run at $SHA: shard $k of $N

Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_01BDtFvVBFVAsRhKf11SwKmb" && \
    git push -q origin "$BR" && echo "pushed $k $(date -u +%T)" ) >> "$S/shards-644d.log" 2>&1
done
echo "all done $(date -u +%T)" >> "$S/shards-644d.log"
