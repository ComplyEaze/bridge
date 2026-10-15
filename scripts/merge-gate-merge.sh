#!/usr/bin/env bash
# Merge a pull request only as the gate judged it: run scripts/merge-gate.sh, then mark a draft ready and squash-merge pinned to
# the head commit the gate printed, refusing on any mismatch. What an allow rule can name instead of a raw `gh pr merge`.
#
# Usage: scripts/merge-gate-merge.sh <pr-number> [--repo OWNER/NAME] [--binary-review-sha FULL_SHA] [--dry-run]
# Exit:  0 merged (or a dry run that passed every check), 1 refused or the merge failed, 2 usage.
#
# The gate runs from this script's own directory, and it refuses to run unless everything under scripts/ (this file too) equals
# master. The commit to merge is the 40 hex digits in the gate's own printed merge command, never one read again from the pull
# request: `gh pr merge --match-head-commit` then makes GitHub refuse if the head moved after the gate looked.
set -uo pipefail

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd) || { echo "could not resolve the script directory" >&2; exit 2; }
usage() { echo "usage: merge-gate-merge.sh PR_NUMBER [--repo OWNER/NAME] [--binary-review-sha FULL_SHA] [--dry-run]" >&2; exit 2; }

PR=""
REPO="ComplyEaze/bridge"
BINARY=""
DRY=0
while [ $# -gt 0 ]; do
  case "$1" in
    --repo) [ $# -ge 2 ] || usage; REPO="$2"; shift 2 ;;
    --binary-review-sha) [ $# -ge 2 ] || usage; BINARY="$2"; shift 2 ;;
    --dry-run) DRY=1; shift ;;
    -*) usage ;;
    *) [ -z "$PR" ] || usage; PR="$1"; shift ;;
  esac
done
[[ "$PR" =~ ^[0-9]{1,7}$ ]] || usage
[[ "$REPO" =~ ^[A-Za-z0-9._-]+/[A-Za-z0-9._-]+$ ]] || usage
[ -z "$BINARY" ] || [[ "$BINARY" =~ ^[0-9a-f]{40}$ ]] || usage

refuse() { echo "REFUSED: $1" >&2; exit 1; }

gate_args=("$PR" --repo "$REPO")
[ -z "$BINARY" ] || gate_args+=(--binary-review-sha "$BINARY")
gate_status=0
gate_out=$("$script_dir/merge-gate.sh" "${gate_args[@]}") || gate_status=$?
printf '%s\n' "$gate_out"
[ "$gate_status" -eq 0 ] || refuse "the gate did not pass (exit $gate_status)"
[ "$(printf '%s\n' "$gate_out" | grep -c '^MAY MERGE')" -eq 1 ] || refuse "the gate did not print exactly one MAY MERGE line"

repo_re=${REPO//./\\.}
merge_re="^    && gh pr merge ${PR} --repo ${repo_re} --squash --match-head-commit [0-9a-f]{40}\$"
[ "$(printf '%s\n' "$gate_out" | grep -Ec "$merge_re")" -eq 1 ] || refuse "the gate did not print exactly one merge command for PR $PR in $REPO"
gated=$(printf '%s\n' "$gate_out" | grep -E "$merge_re" | awk '{print $NF}')
[[ "$gated" =~ ^[0-9a-f]{40}$ ]] || refuse "the merge command's head is not 40 lowercase hex digits"

base_re="^  \\[ \"\\\$\\(gh pr view ${PR} --repo ${repo_re} --json baseRefName -q \\.baseRefName\\)\" = \"[A-Za-z0-9._/-]+\" \\] \\\\\$"
[ "$(printf '%s\n' "$gate_out" | grep -Ec "$base_re")" -eq 1 ] || refuse "the gate did not print exactly one base-branch check for PR $PR"
expected_base=$(printf '%s\n' "$gate_out" | grep -E "$base_re" | sed -E 's/.*= "([^"]+)" \].*/\1/')
[ -n "$expected_base" ] || refuse "could not read the base branch the gate checked"

view=$(gh pr view "$PR" --repo "$REPO" --json state,isDraft,headRefOid,baseRefName 2>/dev/null) || refuse "could not read the pull request"
jq -e '(.state | type == "string") and (.isDraft | type == "boolean") and (.headRefOid | type == "string") and (.baseRefName | type == "string")' <<<"$view" >/dev/null 2>&1 \
  || refuse "could not read the pull request (incomplete answer)"
[ "$(jq -r .state <<<"$view")" = "OPEN" ] || refuse "the pull request is not open"
[ "$(jq -r .baseRefName <<<"$view")" = "$expected_base" ] || refuse "the pull request's base is not the base the gate checked ($expected_base)"
[ "$(jq -r .headRefOid <<<"$view")" = "$gated" ] || refuse "the head moved since the gate looked (gated ${gated:0:12}, now $(jq -r '.headRefOid[0:12]' <<<"$view"))"

if [ "$DRY" -eq 1 ]; then
  echo "dry run: every check passed; would merge PR $PR in $REPO at $gated (draft: $(jq -r .isDraft <<<"$view")); nothing was changed"
  exit 0
fi
made_ready=0
undo_ready() {
  [ "$made_ready" -eq 1 ] || return 0
  gh pr ready "$PR" --repo "$REPO" --undo >/dev/null 2>&1 && echo "the pull request was returned to draft" >&2 || echo "WARNING: could not return PR $PR to draft; it is ready for review" >&2
}
if [ "$(jq -r .isDraft <<<"$view")" = "true" ]; then
  gh pr ready "$PR" --repo "$REPO" || refuse "could not mark the pull request ready"
  made_ready=1
fi
# Read it once more just before the merge: the state, base and head must still be what the gate looked at.
view=$(gh pr view "$PR" --repo "$REPO" --json state,isDraft,headRefOid,baseRefName 2>/dev/null) || { undo_ready; refuse "could not read the pull request again before the merge"; }
if [ "$(jq -r .state <<<"$view" 2>/dev/null)" != "OPEN" ] || [ "$(jq -r .baseRefName <<<"$view" 2>/dev/null)" != "$expected_base" ] || [ "$(jq -r .headRefOid <<<"$view" 2>/dev/null)" != "$gated" ]; then
  undo_ready
  refuse "the pull request changed (state, base or head) between the checks and the merge"
fi
if gh pr merge "$PR" --repo "$REPO" --squash --match-head-commit "$gated"; then
  echo "merged PR $PR in $REPO at ${gated:0:12}"
  exit 0
fi
undo_ready
echo "REFUSED: gh did not merge PR $PR (not merged)" >&2
exit 1
