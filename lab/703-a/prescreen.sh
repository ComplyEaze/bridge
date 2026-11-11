#!/bin/sh
# The macOS CLI pre-screen for row 703-A (#703): the same server and text as the lab row, through `claude -p`.
#   sh prescreen.sh <empty results folder> [runs, default 3] [model, default sonnet] [id|arguments, default id]
# `arguments` (sheet version 3) serves tools-args.json with --continue-by arguments: no read_id, the same arguments continue.
# Each run is a new non-interactive session in an empty folder, with only this server's two tools allowed and no
# other MCP server, at --step-scale 0.05 (steps of 1.5 to 2.75 s: this measures the model, not a host's wait).
# Writes, per run, the answer text, the server log and steps.py's output; prints one verdict line per run:
#   PASS only when the run started 1 read, finished it, made exactly the plan's vouchers calls with no refusal and one
#   set of arguments (read_id aside), every
#   other call was answered, and the answer names the last step's party, Lab Party <code>. Anything else is FAIL with the facts line.
# It does not stand in for the Windows row: the host, its tool prompts and its wait differ.
set -u
out=${1:?results folder}; runs=${2:-3}; model=${3:-sonnet}; mode=${4:-id}
case $mode in id) tools=tools.json ;; arguments) tools=tools-args.json ;; *) echo "FAIL: mode is id or arguments"; exit 2 ;; esac
kit=$(cd "$(dirname "$0")" && pwd -P)
[ -d "$out" ] && [ -z "$(ls -A "$out")" ] || { echo "FAIL: $out must be an existing empty folder"; exit 2; }
out=$(cd "$out" && pwd -P)
mkdir "$out/cwd"
P5="Show me my company's vouchers from 2 to 8 November 2026."
P12="Show me my company's vouchers for October 2026."
for plan in five twelve; do
  [ $plan = five ] && prompt=$P5 || prompt=$P12
  n=1
  while [ $n -le "$runs" ]; do
    tag=$plan-$n
    log="$out/$tag.jsonl"
    cat > "$out/$tag.mcp.json" <<JSON
{"mcpServers":{"lab703":{"command":"node","args":["$kit/stepped-vouchers-server.mjs","--tools","$kit/$tools","--log","$log","--step-scale","0.05","--continue-by","$mode"]}}}
JSON
    (cd "$out/cwd" && perl -e 'alarm shift; exec @ARGV' 900 claude -p "$prompt" --model "$model" \
      --strict-mcp-config --mcp-config "$out/$tag.mcp.json" --setting-sources "" --no-session-persistence \
      --allowedTools mcp__lab703__list_companies mcp__lab703__vouchers \
      --disallowedTools Bash Read Write Edit Glob Grep WebFetch WebSearch Task NotebookEdit \
      > "$out/$tag.answer.txt" 2> "$out/$tag.stderr.txt")
    echo "exit $?" >> "$out/$tag.stderr.txt"
    if ! python3 -I "$kit/steps.py" "$log" > "$out/$tag.steps.txt"; then
      echo "$tag FAIL steps.py: $(tail -1 "$out/$tag.steps.txt")"; n=$((n + 1)); continue
    fi
    facts=$(grep '^facts ' "$out/$tag.steps.txt")
    readline=$(grep '^read ' "$out/$tag.steps.txt")
    code=$(echo "$readline" | sed -n 's/.* code \([A-Z]*\) .*/\1/p' | head -1)
    of=$([ $plan = five ] && echo 5 || echo 12)
    named=0
    [ -n "$code" ] && grep -q "Lab Party $code" "$out/$tag.answer.txt" && named=1
    if [ "$(echo "$readline" | grep -c .)" = 1 ] && echo "$readline" | grep -q " steps $of of $of done true" \
       && echo "$facts" | grep -q " vouchers_calls $of answered $of cut 0 busy 0 refused 0 arguments_changed 0 open 0 " \
       && echo "$facts" | grep -q " leftover_dropped 0 continued_leftover 0 distinct_arguments 1 other_calls_not_answered 0$" \
       && [ "$named" -gt 0 ]; then
      echo "$tag PASS"
    else
      echo "$tag FAIL code_named $named | $readline | $facts"
    fi
    n=$((n + 1))
  done
done
