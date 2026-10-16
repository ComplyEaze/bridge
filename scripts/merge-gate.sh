#!/usr/bin/env bash
# Validate a pull request's pinned-path acknowledgement (compatibility surface),
# run the privacy/PII scan, and confirm that the owner's two machine lines (MERGE-VERDICT CLEAN and
# MERGE-ADVERSARY RECOMMEND, bound to the head and the merge base) stand on the pull request.
#
# Usage: scripts/merge-gate.sh <pr-number> [--repo OWNER/NAME]
#        [--binary-review-sha FULL_SHA] (the owner's own act: binary bytes, ownership, license, and NOTICE reviewed)
# Exit:  0 may merge, 1 must not, 2 could not determine.
#
# This script intentionally does not re-derive PR/head/base identity binding,
# branch-protection required-check contexts, or a checks rollup: those are
# GitHub's own job, enforced natively by required status checks on master
# (see docs/proposed-merge-gate-ci.md). Every check here is bound to one
# server-observed head SHA; unknown or incomplete evidence is never
# converted into an empty successful set.

set -uo pipefail

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd) || {
  echo "could not resolve merge-gate script directory" >&2
  exit 2
}

PR=""
REPO=""
BINARY_REVIEW_SHA=""
while [ $# -gt 0 ]; do
  case "$1" in
    --repo)
      REPO="${2:-}"
      [ -n "$REPO" ] || { echo "--repo needs OWNER/NAME" >&2; exit 2; }
      shift 2
      ;;
    --repo=*)
      REPO="${1#--repo=}"
      [ -n "$REPO" ] || { echo "--repo= needs OWNER/NAME" >&2; exit 2; }
      shift
      ;;
    --binary-review-sha)
      BINARY_REVIEW_SHA="${2:-}"
      [ -n "$BINARY_REVIEW_SHA" ] || { echo "--binary-review-sha needs a full commit SHA" >&2; exit 2; }
      shift 2
      ;;
    --binary-review-sha=*)
      BINARY_REVIEW_SHA="${1#*=}"
      [ -n "$BINARY_REVIEW_SHA" ] || { echo "--binary-review-sha= needs a full commit SHA" >&2; exit 2; }
      shift
      ;;
    -h|--help)
      sed -n '2,16p' "$0"
      exit 0
      ;;
    -*)
      echo "unknown option: $1" >&2
      exit 2
      ;;
    *)
      if [ -z "$PR" ]; then
        PR="$1"
      else
        echo "unexpected argument: $1" >&2
        exit 2
      fi
      shift
      ;;
  esac
done
[ -n "$PR" ] || { echo "usage: $0 <pr-number> [--repo OWNER/NAME]" >&2; exit 2; }
[[ "$PR" =~ ^[0-9]+$ ]] || { echo "PR selector must be numeric" >&2; exit 2; }

# Resolve the repository once. An explicit target must never fall back to the
# current checkout if one later API call fails.
if [ -z "$REPO" ]; then
  if ! REPO=$(gh repo view --json nameWithOwner -q .nameWithOwner 2>/dev/null); then
    echo "could not determine repository; pass --repo OWNER/NAME" >&2
    exit 2
  fi
fi
OWNER="${REPO%%/*}"
NAME="${REPO##*/}"
if [ -z "$OWNER" ] || [ -z "$NAME" ] || [ "$OWNER" = "$REPO" ] || [[ "$REPO" == */*/* ]] \
  || ! [[ "$OWNER" =~ ^[A-Za-z0-9][A-Za-z0-9._-]*$ ]] \
  || ! [[ "$NAME" =~ ^[A-Za-z0-9][A-Za-z0-9._-]*$ ]]; then
  echo "--repo must be OWNER/NAME, got '$REPO'" >&2
  exit 2
fi

# This command encodes Bridge-specific master workflow and surface policy.
# An arbitrary repository's passing checks cannot qualify that contract.
# Two names: the repository moved from the lamemustafa account to the ComplyEaze
# organization, and the old name stays valid as a redirect, so both are accepted.
if [ "$REPO" != "lamemustafa/bridge" ] && [ "$REPO" != "ComplyEaze/bridge" ]; then
  echo "unsupported repository: this gate implements the Bridge policy (lamemustafa/bridge or ComplyEaze/bridge)" >&2
  exit 2
fi

# The gate is run only from origin/master's tree, never from the pull request's checkout: a pull request must not be able to
# change the program that judges it. The gate's own files must equal the branch origin/master (as git compares, that is after any
# .gitattributes conversion) and be committed.
gate_tree=$(git -C "$script_dir" rev-parse --show-toplevel 2>/dev/null) || gate_tree=""
fetch_status=0
if [ -n "$gate_tree" ]; then
  fetch_err=$(git -C "$gate_tree" fetch -q origin +refs/heads/master:refs/merge-gate/master 2>&1 >/dev/null) || fetch_status=$?
else
  fetch_status=1
  fetch_err="this gate is not inside a git checkout"
fi
if [ "$fetch_status" -ne 0 ]; then
  echo "could not fetch the branch master of origin to compare this gate with it: ${fetch_err:0:300}" >&2
  exit 2
fi
if [ "$(cd "$script_dir" && pwd -P)" != "$(cd "$gate_tree" && pwd -P)/scripts" ] \
   || ! git -C "$gate_tree" rev-parse --verify -q refs/merge-gate/master >/dev/null 2>&1 \
   || ! git -C "$gate_tree" diff --quiet refs/merge-gate/master -- scripts 2>/dev/null \
   || [ -n "$(git -C "$gate_tree" status --porcelain --untracked-files=all -- scripts 2>/dev/null)" ]; then
  echo "run this gate from origin/master's tree (freshly fetched; everything under scripts/ must equal it, untracked files included): a pull request must not run the program that judges it" >&2
  exit 2
fi

fail=0
uncertain=0
tmpdir=$(mktemp -d)
errfile="$tmpdir/error"
trap 'rm -rf "$tmpdir"' EXIT
say() { printf '  %-13s %s\n' "$1" "$2"; }
bad() { say "BLOCK" "$1"; fail=1; }
unknown() { say "INDETERMINATE" "$1"; uncertain=1; }
die() { echo "$1" >&2; exit 2; }
# A malformed response is different from a valid empty result. Validate the
# outer shape before extracting fields so jq errors cannot become empty values.
: >"$errfile"
if ! meta=$(gh pr view "$PR" --repo "$REPO" \
        --json headRefOid,baseRefName,title,body,changedFiles,headRefName 2>"$errfile"); then
  die "could not read PR #$PR in $REPO"
fi
if ! jq -e '
  type == "object" and
  (.headRefOid | type == "string" and test("^[0-9a-fA-F]{40}$")) and
  (.baseRefName | type == "string" and length > 0) and
  (.title | type == "string") and
  (.changedFiles | type == "number" and floor == . and . >= 0)
' <<<"$meta" >/dev/null 2>&1; then
  die "PR metadata was not a valid complete JSON object"
fi
head=$(jq -r '.headRefOid' <<<"$meta")
head_ref=$(jq -r '.headRefName // ""' <<<"$meta")
base=$(jq -r '.baseRefName' <<<"$meta")
changed_files_expected=$(jq -r '.changedFiles' <<<"$meta")
short=${head:0:7}
title=$(jq -r '.title' <<<"$meta")
raw_prbody=$(jq -r '.body // ""' <<<"$meta")
prbody="$raw_prbody"
visible_body_status=0
prbody=$(python3 -I -c 'import re, sys
text = sys.stdin.read()
print(re.sub(r"<!--.*?(?:-->|\Z)", "", text, flags=re.S), end="")' <<<"$prbody") || visible_body_status=$?
if [ "$visible_body_status" -ne 0 ]; then
  unknown "could not extract visible PR description content"
  prbody=""
fi

if [ -n "$BINARY_REVIEW_SHA" ] && ! [[ "$BINARY_REVIEW_SHA" =~ ^[0-9a-fA-F]{40}$ ]]; then
  bad "binary review attestation must be a full 40-hex commit SHA"
elif [ -n "$BINARY_REVIEW_SHA" ] && [ "$BINARY_REVIEW_SHA" != "$head" ]; then
  bad "binary review attestation names a different commit than the PR head"
fi
echo "PR #$PR ($REPO)  head=$short  base=$base"

# Capture the base tip independently. A PR can retain the same base name while
# the branch advances during this run.
: >"$errfile"
base_tip_status=0
base_tip=$(gh api "repos/$REPO/branches/$base" --jq '.commit.sha' 2>"$errfile") || base_tip_status=$?
if [ "$base_tip_status" -ne 0 ] || ! [[ "$base_tip" =~ ^[0-9a-fA-F]{40}$ ]]; then
  unknown "could not read the full current tip of base '$base'"
  base_tip=""
else
  say "ok" "captured base tip ${base_tip:0:7}"
fi
# Scan all published PR metadata. Commit messages are paginated because they
# can become squash subjects or release evidence independently of the patch.
: >"$errfile"
metadata_pr_status=0
metadata_pr=$(gh api "repos/$REPO/pulls/$PR" 2>"$errfile") || metadata_pr_status=$?
if [ "$metadata_pr_status" -ne 0 ] || ! jq -e --arg head "$head" '
  type == "object" and
  (.commits | type == "number" and floor == . and . > 0 and . <= 250) and
  (.head | type == "object" and
   (.sha | type == "string" and test("^[0-9a-fA-F]{40}$") and . == $head))
' <<<"$metadata_pr" >/dev/null 2>&1; then
  unknown "could not prove complete head-bound PR commit metadata for the privacy scan"
  privacy_metadata=""
else
  metadata_commit_total=$(jq -r '.commits' <<<"$metadata_pr")
fi

: >"$errfile"
metadata_status=0
metadata_commits_proven=0
metadata_commits=$(gh api --paginate --slurp "repos/$REPO/pulls/$PR/commits?per_page=100" 2>"$errfile") || metadata_status=$?
if [ -z "${metadata_commit_total:-}" ] || [ "$metadata_status" -ne 0 ] || ! jq -e --argjson expected "$metadata_commit_total" --arg head "$head" '
  type == "array" and (all(.[]; type == "array") or all(.[]; type == "object")) and
  ((if all(.[]; type == "array") then flatten else . end) |
   length == $expected and
   ([.[].sha] | unique | length) == $expected and
   any(.[]; .sha == $head) and
   all(.[]; type == "object" and
    (.sha | type == "string" and test("^[0-9a-fA-F]{40}$")) and
    (.commit | type == "object") and
    (.commit.message | type == "string") and
    (.commit.author | type == "object" and
      (.name | type == "string") and (.email | type == "string" and test("^[A-Za-z0-9.!#$%&*+/=?^_`{|}~-]+@[A-Za-z0-9]([A-Za-z0-9.-]*[A-Za-z0-9])?$"))) and
    (.commit.committer | type == "object" and
      (.name | type == "string") and (.email | type == "string" and test("^[A-Za-z0-9.!#$%&*+/=?^_`{|}~-]+@[A-Za-z0-9]([A-Za-z0-9.-]*[A-Za-z0-9])?$"))) and
    ((.author == null) or (.author | type == "object" and (.login | type == "string"))) and
    ((.committer == null) or (.committer | type == "object" and (.login | type == "string")))))
' <<<"$metadata_commits" >/dev/null 2>&1; then
  unknown "could not prove complete head-bound PR commit metadata for the privacy scan"
  privacy_metadata=""
else
  metadata_commits_proven=1
  # Author and committer emails were structurally validated above and are an
  # explicit identity-only source class. Do not mix them into payload/path/
  # metadata scan input, where an identical address would be customer data.
  # Commit messages are their own source class. Before scanning them, redact
  # only the one known public-agent address when it occupies an exact
  # terminal Co-Authored-By Git trailer. The helper leaves every contributor
  # name, other trailer, PR field, destination, and payload untouched.
  commit_message_json=$(jq -c '(if all(.[]; type == "array") then flatten else . end) | map(.commit.message)' <<<"$metadata_commits")
  sanitized_messages_status=0
  sanitized_message_json=$(printf '%s' "$commit_message_json" | python3 -I "$script_dir/merge_gate_privacy.py" --redact-public-agent-attribution-messages) || sanitized_messages_status=$?
  if [ "$sanitized_messages_status" -ne 0 ] || ! jq -e 'type == "array" and all(.[]; type == "string")' <<<"$sanitized_message_json" >/dev/null 2>&1; then
    unknown "could not classify public-agent attribution trailers in complete head-bound commit metadata"
    privacy_metadata=""
  else
    commit_messages=$(jq -r '.[]' <<<"$sanitized_message_json")
    commit_identity_names=$(jq -r '(if all(.[]; type == "array") then flatten else . end)[] |
      [.commit.author.name, .commit.committer.name, (.author.login? // null), (.committer.login? // null)] |
      map(select(. != null))[]' <<<"$metadata_commits")
    privacy_metadata="$title
$raw_prbody
$commit_messages
$commit_identity_names"
  fi
fi
# Paginate changed files through the REST endpoint; gh pr view hard-codes a
# first:100 GraphQL fragment in some versions. Retain the line counts as well:
# the privacy scan can only be complete when the textual diff describes every
# non-removed destination with the byte count GitHub reported.
: >"$errfile"
files_status=0
files=$(gh api --paginate --slurp "repos/$REPO/pulls/$PR/files?per_page=100" 2>"$errfile") || files_status=$?
changed_records="$tmpdir/changed-files.tsv"
if [ "$files_status" -ne 0 ] || ! jq -e '
  type == "array" and
  (all(.[]; type == "array" and all(.[];
      type == "object" and
      ((.filename | type) == "string") and (.filename | length > 0) and (.filename | test("[\u0000-\u001F\u007F]") | not) and
      ((.status | type) == "string") and (.status | length > 0) and
      (((.previous_filename? == null) or (((.previous_filename | type) == "string") and ((.previous_filename | length) > 0) and ((.previous_filename | test("[\u0000-\u001F\u007F]")) | not))) and ((.status != "renamed") or (((.previous_filename | type) == "string") and ((.previous_filename | length) > 0) and ((.previous_filename | test("[\u0000-\u001F\u007F]")) | not)))) and
      ((.additions | type) == "number") and (.additions | floor == . and . >= 0) and
      ((.deletions | type) == "number") and (.deletions | floor == . and . >= 0))) or
   all(.[]; type == "object" and
      ((.filename | type) == "string") and (.filename | length > 0) and (.filename | test("[\u0000-\u001F\u007F]") | not) and
      ((.status | type) == "string") and (.status | length > 0) and
      (((.previous_filename? == null) or (((.previous_filename | type) == "string") and ((.previous_filename | length) > 0) and ((.previous_filename | test("[\u0000-\u001F\u007F]")) | not))) and ((.status != "renamed") or (((.previous_filename | type) == "string") and ((.previous_filename | length) > 0) and ((.previous_filename | test("[\u0000-\u001F\u007F]")) | not)))) and
      ((.additions | type) == "number") and (.additions | floor == . and . >= 0) and
      ((.deletions | type) == "number") and (.deletions | floor == . and . >= 0)))
' <<<"$files" >/dev/null 2>&1; then
  unknown "could not read the complete changed-file set"
  changed=""
else
  jq -r '(if all(.[]; type == "array") then flatten else . end)[] | [.filename, .status, .additions, .deletions, (.previous_filename? // "")] | @tsv' <<<"$files" >"$changed_records"
  changed=$(cut -f1 "$changed_records")
  changed_count=$(jq '(if all(.[]; type == "array") then flatten else . end) | length' <<<"$files")
  unique_changed_count=$(jq '(if all(.[]; type == "array") then flatten else . end) | map(.filename) | unique | length' <<<"$files")
  if [ "$changed_count" -eq 0 ]; then
    unknown "changed-file response contained no filenames"
  elif [ "$changed_files_expected" -gt 3000 ]; then
    unknown "PR reports $changed_files_expected changed files beyond the REST files API cap"
  elif [ "$changed_count" -ne "$changed_files_expected" ] || [ "$unique_changed_count" -ne "$changed_count" ]; then
    unknown "changed-file response has $changed_count unique records; PR metadata reports $changed_files_expected"
  fi
fi
# Read and validate the surface as a required object: schema 3 at the head
# (an authored pin list, no stored hashes), schema 2 or 3 at the base tip and
# at the merge base (a schema-2 base under a schema-3 head is the cut-over PR).
# Any transport, decoding, JSON, or schema failure is indeterminate; an
# unrelated nested `path` must not turn an incomplete manifest into an empty
# pin set.
#
# A pinned file is not hash-checked here (CI computes the digest from live
# bytes). Instead, a PR that touches a pinned path must ADD exactly one
# acknowledgement file, docs/tally/compatibility/acks/pr-<PR>.txt, listing the
# touched paths. (The acknowledgement's reviewer line is still read for its shape,
# but no review is matched against it any more: the owner's merge verdict, checked
# below, replaces the old review-naming checks.)
SURFACE="docs/tally/compatibility/compatibility-surface.json"
ACK_DIR="docs/tally/compatibility/acks"
ACK_PATH="$ACK_DIR/pr-$PR.txt"
ack_required=0
ack_touched=""
ack_reviewer=""

# Read a repository file at a ref through the contents API into
# $contents_text_result. Any failure returns 1 with the result empty.
read_contents_text() {
  local ref="$1"
  local path="$2"
  local response content decoded decode_status
  contents_text_result=""
  : >"$errfile"
  response=$(gh api "repos/$REPO/contents/$path?ref=$ref" 2>"$errfile") || return 1
  content=$(jq -er 'select(.encoding == "base64") | .content | strings' <<<"$response") || return 1
  decoded=""
  decode_status=0
  decoded=$(printf '%s' "${content//$'\n'/}" | base64 --decode 2>"$errfile") || decode_status=$?
  if [ "$decode_status" -ne 0 ]; then
    decode_status=0
    decoded=$(printf '%s' "${content//$'\n'/}" | base64 -D 2>"$errfile") || decode_status=$?
  fi
  [ "$decode_status" -eq 0 ] || return 1
  contents_text_result="$decoded"
}

# $2 is "3" (head: schema 3 only) or "2or3" (base tip and merge base).
read_surface_paths() {
  local ref="$1"
  local accepted="$2"
  surface_paths_result=""
  surface_schema_result=""
  surface_reasoned_result=""
  surface_bad_reason_result=""
  surface_seen_schema_result=""
  read_contents_text "$ref" "$SURFACE" || return 1
  # Remember what schema the file claims, so a stale schema-2 head gets a useful message.
  surface_seen_schema_result=$(jq -r '.schema_version? // empty | tostring' <<<"$contents_text_result" 2>/dev/null || true)
  if ! jq -e --arg accepted "$accepted" '
    type == "object" and
    (.schema_version as $sv |
      (keys == ["files", "schema_version"]) and
      ($sv == 3 or ($accepted == "2or3" and $sv == 2)) and
      (.files | type == "array" and length > 0 and
        all(.[]; type == "object" and
          ((.path | type) == "string") and (.path | length > 0) and
          (if $sv == 3 then
             ((keys - ["path", "reason"]) | length == 0) and
             ((has("reason") | not) or ((.reason | type) == "string"))
           else
             ((.sha256 | type) == "string") and (.sha256 | test("^[0-9a-f]{64}$"))
           end))) and
      (([.files[].path] | length) == ([.files[].path] | unique | length)))
  ' <<<"$contents_text_result" >/dev/null 2>&1; then
    return 1
  fi
  surface_paths_result=$(jq -r '.files[].path' <<<"$contents_text_result")
  surface_schema_result=$(jq -r '.schema_version' <<<"$contents_text_result")
  surface_reasoned_result=$(jq -r '.files[] | select(((.reason // "") | test("\\S"))) | .path' <<<"$contents_text_result")
  # The pin list caps a reason at 500 characters (not bytes) and refuses control characters
  # (Cc), as the Rust loader and scripts/check-surface-ack.mjs do.
  surface_bad_reason_result=$(jq -r '.files[] | select((.reason // "") | (length > 500 or test("[\u0000-\u001F\u007F-\u009F]"))) | .path' <<<"$contents_text_result")
}

head_surface_status=0
read_surface_paths "$head" 3 || head_surface_status=$?
if [ "$head_surface_status" -ne 0 ]; then
  if [ "$surface_seen_schema_result" = "2" ]; then
    unknown "could not read and validate compatibility surface at $short: it is still schema 2 (a branch that predates schema 3); merge master into the branch and migrate the pin list to schema 3, see docs/release-process.md"
  else
    unknown "could not read and validate compatibility surface at $short"
  fi
  pinned=""
  head_reasoned=""
  head_bad_reason=""
else
  pinned="$surface_paths_result"
  head_reasoned="$surface_reasoned_result"
  head_bad_reason="$surface_bad_reason_result"
  say "ok" "validated schema-$surface_schema_result compatibility surface at head $short"
fi

base_surface_status=0
if [ -n "$base_tip" ]; then
  read_surface_paths "$base_tip" 2or3 || base_surface_status=$?
fi
if [ -n "$base_tip" ] && [ "$base_surface_status" -ne 0 ]; then
  unknown "could not read and validate compatibility surface at base ${base_tip:0:7}"
  base_pinned=""
elif [ -n "$base_tip" ]; then
  base_pinned="$surface_paths_result"
  say "ok" "validated schema-$surface_schema_result compatibility surface at base ${base_tip:0:7}"
else
  base_pinned=""
fi

# Pins added or removed by THIS PR are measured against the PR's merge base, not
# the base tip: a PR behind a master commit that added a pin would otherwise
# look as if it removed that pin.
merge_base=""
merge_base_pinned=""
if [ -n "$base_tip" ] && [ -n "$base_pinned" ]; then
  : >"$errfile"
  merge_base_status=0
  merge_base=$(gh api "repos/$REPO/compare/${base_tip}...${head}?per_page=1" --jq '.merge_base_commit.sha' 2>"$errfile") || merge_base_status=$?
  if [ "$merge_base_status" -ne 0 ] || ! [[ "$merge_base" =~ ^[0-9a-fA-F]{40}$ ]]; then
    unknown "could not read the merge base of $short and base tip ${base_tip:0:7}"
    merge_base=""
  elif [ "$merge_base" = "$base_tip" ]; then
    merge_base_pinned="$base_pinned"
  else
    merge_base_surface_status=0
    read_surface_paths "$merge_base" 2or3 || merge_base_surface_status=$?
    if [ "$merge_base_surface_status" -ne 0 ]; then
      unknown "could not read and validate compatibility surface at merge base ${merge_base:0:7}"
    else
      merge_base_pinned="$surface_paths_result"
      say "ok" "validated schema-$surface_schema_result compatibility surface at merge base ${merge_base:0:7}"
    fi
  fi
fi

# One path per line, blank lines dropped, byte-order sorted and unique.
path_set() { printf '%s\n' "$1" | sed '/^$/d' | LC_ALL=C sort -u; }
# Print the lines of a path list that are not plain repository-relative paths.
malformed_paths() {
  awk '{
    bad = 0
    if ($0 ~ /^\// || $0 ~ /\\/ || $0 ~ /^ / || $0 ~ / $/) bad = 1
    n = split($0, seg, "/")
    for (i = 1; i <= n; i++) if (seg[i] == "" || seg[i] == "." || seg[i] == "..") bad = 1
    if (bad) print
  }'
}
first_lines() { head -n 5 | paste -sd ' ' -; }
# Succeeds when the non-blank lines are strictly ascending in byte order.
is_sorted_unique() {
  local text
  text=$(sed '/^$/d' <<<"$1")
  [ -z "$text" ] || LC_ALL=C sort -c -u <<<"$text" 2>/dev/null
}

# The branch history check, as in scripts/check-surface-ack.mjs. The diff against the base cannot
# see a pin the branch added and then lost (a merge resolved by taking the other side's pin list):
# at the head the list equals the base's. So every commit of the PR (the complete, head-bound list
# proved above) is read with its parents: a path is collected when a commit's pin list holds it and
# none of its parents' lists does (a merge is compared with ALL its parents). A collected path that
# is in neither the base tip's list nor the head's was pinned by the branch and withdrawn. It is a
# removed pin: the acknowledgement must declare it with a "removed-pin:" line. Any commit whose
# list cannot be read and parsed makes the answer indeterminate (fail closed).
withdrawn_pins=""
history_cache="$tmpdir/pin-history"
# The sorted, unique pin paths at one commit, in the file named by $pins_at_result.
pins_at_commit() {
  local sha="$1"
  local file="$history_cache/$sha"
  pins_at_result=""
  if ! [ -f "$file" ]; then
    read_contents_text "$sha" "$SURFACE" || return 1
    jq -e '.files | type == "array" and all(.[]; type == "object" and (.path | type == "string" and length > 0))' \
      <<<"$contents_text_result" >/dev/null 2>&1 || return 1
    jq -r '.files[].path' <<<"$contents_text_result" | LC_ALL=C sort -u >"$file.tmp" || return 1
    mv "$file.tmp" "$file"
  fi
  pins_at_result="$file"
}
if [ -n "$pinned" ] && [ -n "$base_pinned" ]; then
  if [ "$metadata_commits_proven" -ne 1 ]; then
    unknown "the PR's complete commit list is unproven, so a pin the branch added and then lost cannot be ruled out"
  else
    mkdir -p "$history_cache"
    commit_records=$(jq -r '(if all(.[]; type == "array") then flatten else . end)[] |
      if (.parents | type) == "array" and all(.parents[]; (.sha | type == "string") and (.sha | test("^[0-9a-fA-F]{40}$"))) then
        ([.sha] + [.parents[].sha]) | join(" ")
      else "NOPARENTS " + .sha end' <<<"$metadata_commits" 2>/dev/null) || commit_records="NOPARENTS unreadable"
    history_status=0
    collected=""
    parents_union="$tmpdir/parents-union"
    while read -r -a record; do
      [ "${#record[@]}" -gt 0 ] || continue
      if [ "${record[0]}" = "NOPARENTS" ]; then history_status=1; break; fi
      pins_at_commit "${record[0]}" || { history_status=1; break; }
      own_file="$pins_at_result"
      : >"$parents_union"
      for parent in "${record[@]:1}"; do
        pins_at_commit "$parent" || { history_status=1; break 2; }
        cat "$pins_at_result" >>"$parents_union"
      done
      LC_ALL=C sort -u -o "$parents_union" "$parents_union"
      collected="$collected"$'\n'"$(LC_ALL=C comm -23 "$own_file" "$parents_union")"
    done <<<"$commit_records"
    if [ "$history_status" -ne 0 ]; then
      unknown "could not read and parse the pin list at every commit of the PR (and its parents), so a pin the branch added and then lost cannot be ruled out"
    else
      withdrawn_pins=$(LC_ALL=C comm -23 <(path_set "$collected") <(path_set "$base_pinned") | LC_ALL=C comm -23 - <(path_set "$pinned"))
    fi
  fi
fi

if [ -n "$changed" ] && [ -n "$pinned" ] && [ -n "$base_pinned" ] && [ -n "$merge_base_pinned" ]; then
  removed_pins=$(LC_ALL=C comm -23 <(path_set "$merge_base_pinned") <(path_set "$pinned"))
  removed_pins=$(path_set "$(printf '%s\n%s\n' "$removed_pins" "$withdrawn_pins")")
  added_pins=$(LC_ALL=C comm -13 <(path_set "$merge_base_pinned") <(path_set "$pinned"))
  union_pinned=$(path_set "$(printf '%s\n%s\n' "$base_pinned" "$pinned")")
  # Both names of a rename count: the old path is what a pinned rename removes.
  changed_names=$(path_set "$(printf '%s\n%s\n' "$changed" "$(awk -F '\t' '$5 != "" { print $5 }' "$changed_records")")")
  # A `.gitattributes` below the root can change the bytes of a pinned file (eol, ident,
  # working-tree-encoding) without any pinned path in the diff, so none is allowed.
  nested_attributes=$(awk -F/ 'NF > 1 && $NF == ".gitattributes"' <<<"$changed_names" | first_lines)
  [ -z "$nested_attributes" ] || bad "a nested .gitattributes can change the bytes of a pinned file without a pinned path changing; none is allowed: $nested_attributes"
  changed_pinned=$(LC_ALL=C comm -12 <(printf '%s\n' "$union_pinned") <(printf '%s\n' "$changed_names") \
    | awk -v surface="$SURFACE" '$0 != surface' \
    | LC_ALL=C comm -23 - <(path_set "$removed_pins"))
  ack_expected=$(path_set "$(printf '%s\n%s\n' "$changed_pinned" "$added_pins")")
  ack_touched=$(path_set "$(printf '%s\n%s\n' "$ack_expected" "$removed_pins")")
  # Records that touch the ack directory. An ack may only be ADDED: a modification, a deletion,
  # and a rename of an ack (a delete plus an add, including out of the directory) are changes.
  # A copy leaves its source alone, so a copy out of the directory is not an ack change.
  ack_records=$(awk -F '\t' -v dir="$ACK_DIR/" 'index($1, dir) == 1 || ($2 != "copied" && index($5, dir) == 1)' "$changed_records")
  # awk helpers over these records. own(): the record adds THIS PR's ack: status added, or copied,
  # or renamed FROM a file outside the ack directory (git pairs an added ack with an unrelated
  # deleted file of similar content). name(): what to report: the old ack a rename removed,
  # else the changed path.
  ack_awk_defs='function own() { return $1 == p && ($2 == "added" || $2 == "copied" || ($2 == "renamed" && index($5, dir) != 1)) }
    function name() { return ($2 == "renamed" && index($5, dir) == 1) ? $5 : $1 }
'
  if [ -z "$ack_touched" ]; then
    added_acks=$(awk -F '\t' -v dir="$ACK_DIR/" "$ack_awk_defs"'$2 != "removed" { print name() }' <<<"$ack_records" | first_lines)
    if [ -n "$added_acks" ]; then
      bad "no pinned path is touched, so no acknowledgement may be added or changed: $added_acks"
    else
      say "ok" "changed files contain no pinned path requiring an acknowledgement"
    fi
  else
    ack_required=1
    touched_count=$(sed '/^$/d' <<<"$ack_touched" | wc -l | tr -d ' ')
    have_ack=$(awk -F '\t' -v p="$ACK_PATH" -v dir="$ACK_DIR/" "$ack_awk_defs"'own() { n++ } END { print n + 0 }' <<<"$ack_records")
    other_acks=$(awk -F '\t' -v p="$ACK_PATH" -v dir="$ACK_DIR/" "$ack_awk_defs"'!own() { print name() }' <<<"$ack_records" | sed '/^$/d' | first_lines)
    if [ "$have_ack" -ne 1 ]; then
      bad "$touched_count pinned path(s) touched (e.g. $(first_lines <<<"$ack_touched")) but the PR does not add $ACK_PATH"
    fi
    if [ -n "$other_acks" ]; then
      bad "acknowledgement files are append-only; the PR also changes: $other_acks"
    fi
    if [ "$have_ack" -eq 1 ]; then
      if ! read_contents_text "$head" "$ACK_PATH"; then
        unknown "could not read $ACK_PATH at $short"
      else
        ack_text="$contents_text_result"
        ack_problems=""
        ack_note() { ack_problems="${ack_problems}${ack_problems:+; }$1"; }
        # Here-strings, not `printf | grep -q`: under pipefail a match that makes grep exit early
        # kills printf with SIGPIPE on a large ack, and the pipeline then reports "no match".
        if LC_ALL=C grep -q '[[:cntrl:]]' <<<"$ack_text"; then
          ack_note "contains a control character"
        fi
        if LC_ALL=C grep -Eiq '[0-9a-f]{64}' <<<"$ack_text"; then
          ack_note "contains a 64-hex token (paths only; a hash would re-create same-file conflicts)"
        fi
        reviewer_lines=$(grep -E '^reviewer: ' <<<"$ack_text" || true)
        # A GitHub login (1-39 characters, no hyphen at either end, optional [bot]); the same
        # expression is in scripts/check-surface-ack.mjs.
        login_re='^reviewer: [A-Za-z0-9]([A-Za-z0-9-]{0,37}[A-Za-z0-9])?(\[bot\])?$'
        reviewer_count=$(sed '/^$/d' <<<"$reviewer_lines" | wc -l | tr -d ' ')
        if [ "$reviewer_count" -ne 1 ] || ! [[ "$reviewer_lines" =~ $login_re ]]; then
          ack_note "needs exactly one 'reviewer: <github login>' line"
        else
          ack_reviewer="${reviewer_lines#reviewer: }"
        fi
        ack_paths=$(grep -Ev '^(reviewer|removed-pin): ' <<<"$ack_text" || true)
        ack_removed=$(sed -n 's/^removed-pin: //p' <<<"$ack_text")
        if [ -n "$(malformed_paths <<<"$ack_paths")" ] || [ -n "$(malformed_paths <<<"$ack_removed")" ]; then
          ack_note "has a line that is not a repository path"
        fi
        # Byte order (LC_ALL=C), the order Rust and scripts/check-surface-ack.mjs use.
        if ! is_sorted_unique "$ack_paths"; then
          ack_note "paths must be sorted and unique"
        fi
        if ! is_sorted_unique "$ack_removed"; then
          ack_note "removed-pin lines must be sorted and unique"
        fi
        ack_missing=$(LC_ALL=C comm -13 <(path_set "$ack_paths") <(printf '%s\n' "$ack_expected") | sed '/^$/d' | first_lines)
        ack_extra=$(LC_ALL=C comm -23 <(path_set "$ack_paths") <(printf '%s\n' "$ack_expected") | sed '/^$/d' | first_lines)
        [ -z "$ack_missing" ] || ack_note "omits touched pinned path(s): $ack_missing"
        [ -z "$ack_extra" ] || ack_note "lists path(s) the PR does not touch: $ack_extra"
        removed_undeclared=$(LC_ALL=C comm -13 <(path_set "$ack_removed") <(path_set "$removed_pins"))
        removed_missing=$(LC_ALL=C comm -23 <(path_set "$removed_undeclared") <(path_set "$withdrawn_pins") | sed '/^$/d' | first_lines)
        withdrawn_missing=$(LC_ALL=C comm -12 <(path_set "$removed_undeclared") <(path_set "$withdrawn_pins") | sed '/^$/d' | first_lines)
        removed_extra=$(LC_ALL=C comm -23 <(path_set "$ack_removed") <(path_set "$removed_pins") | sed '/^$/d' | first_lines)
        [ -z "$removed_missing" ] || ack_note "lacks a removed-pin line for: $removed_missing"
        [ -z "$withdrawn_missing" ] || ack_note "pin(s) pinned by a commit of this PR but not in the pin list at the head; restore the pin, or declare the withdrawal with a removed-pin line: $withdrawn_missing"
        [ -z "$removed_extra" ] || ack_note "has removed-pin line(s) for pin(s) the PR does not remove: $removed_extra"
        no_reason=$(LC_ALL=C comm -23 <(path_set "$added_pins") <(path_set "$head_reasoned") | sed '/^$/d' | first_lines)
        [ -z "$no_reason" ] || ack_note "pin(s) added without a non-empty reason: $no_reason"
        bad_reason=$(LC_ALL=C comm -12 <(path_set "$added_pins") <(path_set "$head_bad_reason") | sed '/^$/d' | first_lines)
        [ -z "$bad_reason" ] || ack_note "pin(s) added with a reason over 500 characters or containing a control character: $bad_reason"
        if [ -n "$ack_problems" ]; then
          bad "$ACK_PATH is invalid: $ack_problems"
        else
          say "ok" "$ACK_PATH lists the $touched_count touched pinned path(s), reviewer $ack_reviewer"
        fi
      fi
    fi
  fi
fi
# Scan destination paths and added payload lines. Removed/context lines never
# enter the privacy scan. Binary additions are an explicit human-inspection
# hold because their bytes are absent from a textual patch.
: >"$errfile"
diff_status=0
diff=$(gh pr diff "$PR" --repo "$REPO" 2>"$errfile") || diff_status=$?
if [ "$diff_status" -ne 0 ] || [ -z "$diff" ]; then
  unknown "could not read diff for the privacy scan"
else
  # Parse Git's C-style quoted paths with the existing Python runtime. The
  # shell receives only JSON/TSV data after the parser has matched every diff
  # section, so non-ASCII destinations retain their REST filename identity.
  diff_stats="$tmpdir/diff-stats.tsv"
  added_payload="$tmpdir/added-payload"
  parsed_diff_status=0
  parsed_diff=$(python3 -I "$script_dir/merge_gate_diff.py" <<<"$diff") || parsed_diff_status=$?
  if [ "$parsed_diff_status" -ne 0 ] || ! jq -e '
    type == "object" and
    (.records | type == "array" and all(.[]; type == "object" and
      (.destination | type == "string" and length > 0) and
      ((.textual_destination == null) or (.textual_destination | type == "string" and length > 0)) and
      (.added | type == "number" and floor == . and . >= 0) and
      (.deleted | type == "number" and floor == . and . >= 0) and
      (.binary | type == "boolean") and (.gitlink | type == "boolean"))) and
    (.added_payload | type == "array" and all(.[]; type == "string"))
  ' <<<"$parsed_diff" >/dev/null 2>&1; then
    unknown "could not parse diff sections for the privacy scan"
    : >"$diff_stats"
    : >"$added_payload"
  else
    jq -r '.records[] | [.destination, .added, .deleted, (if .textual_destination == null then 0 else 1 end), (if .binary then 1 else 0 end), (if .gitlink then 1 else 0 end)] | @tsv' <<<"$parsed_diff" >"$diff_stats"
    jq -r '.added_payload[]' <<<"$parsed_diff" >"$added_payload"

  coverage_count=0
  coverage_examples=""
  metadata_only_count=0
  metadata_only_examples=""
  binary_count=0
  binary_examples=""
  gitlink_count=0
  bounded_coverage_name() {
    local item
    item=$(sed -E 's#/(Users|home)/[^/[:space:]]+#<home>#g; s#[A-Za-z]:[\\/]+Users[\\/]+[^\\/[:space:]]+#<home>#g' <<<"$1")
    # The privacy classifier's own identifier-shape detection also runs on
    # the (already home-dir-redacted) filename: a coverage/metadata-only
    # example is echoed verbatim into gate output, and a filename can itself
    # carry an identifier, digest, or email shape.
    item=$(printf '%s' "$item" | python3 -I "$script_dir/merge_gate_privacy.py" --redact-shapes 2>/dev/null) || item="$item"
    printf '%s' "${item:0:160}"
  }
  record_coverage_issue() {
    coverage_count=$((coverage_count + 1))
    if [ "$coverage_count" -le 8 ]; then
      local item
      item=$(bounded_coverage_name "$1")
      coverage_examples="${coverage_examples}${coverage_examples:+; }$item"
    fi
  }
  record_metadata_only() {
    metadata_only_count=$((metadata_only_count + 1))
    if [ "$metadata_only_count" -le 8 ]; then
      metadata_only_examples="${metadata_only_examples}${metadata_only_examples:+; }$(bounded_coverage_name "$1")"
    fi
  }
  path_text=""
  if [ -s "$changed_records" ]; then
    while IFS=$'\t' read -r filename status rest_added rest_deleted previous_filename; do
      match_count=$(awk -F '\t' -v filename="$filename" '$1 == filename { count++ } END { print count+0 }' "$diff_stats")
      if [ "$match_count" -ne 1 ]; then
        record_coverage_issue "omits or duplicates '$filename'"
        continue
      fi
      diff_record=$(awk -F '\t' -v filename="$filename" '$1 == filename { print; exit }' "$diff_stats")
      IFS=$'\t' read -r _ diff_added diff_deleted textual binary gitlink <<<"$diff_record"
      if [ "$gitlink" -eq 1 ]; then
        gitlink_count=$((gitlink_count + 1))
      fi
      if [ "$binary" -eq 1 ]; then
        # Its bytes cannot be reconciled through textual hunks.
        binary_count=$((binary_count + 1))
        if [ "$binary_count" -le 8 ]; then
          binary_examples="${binary_examples}${binary_examples:+; }$(bounded_coverage_name "$filename")"
        fi
        continue
      fi
      if [ "$status" = "removed" ]; then
        # A removed record has no new content, so it is excluded from the
        # metadata-only/textual-destination checks below (its +++ destination
        # is legitimately /dev/null). It must still be reconciled first: a
        # removed record absent from the diff was already caught by the
        # match_count check above, and its line totals must still agree with
        # REST before it is excluded from further scanning.
        if [ "$diff_added" -ne "$rest_added" ] || [ "$diff_deleted" -ne "$rest_deleted" ]; then
          record_coverage_issue "line totals for '$filename' differ from REST metadata"
        fi
        continue
      fi
      if [ "$textual" -ne 1 ] && [ "$diff_added" -eq 0 ] && [ "$diff_deleted" -eq 0 ]; then
        if [ "$rest_added" -eq 0 ] && [ "$rest_deleted" -eq 0 ]; then
          record_metadata_only "$filename"
        else
          record_coverage_issue "metadata-only '$filename' conflicts with REST line totals"
        fi
      elif [ "$textual" -ne 1 ]; then
        record_coverage_issue "lacks a textual destination for '$filename'"
      elif [ "$diff_added" -ne "$rest_added" ] || [ "$diff_deleted" -ne "$rest_deleted" ]; then
        record_coverage_issue "line totals for '$filename' differ from REST metadata"
      fi
    done <"$changed_records"
    # Reconciliation above is REST-to-diff only: it proves every REST record
    # has exactly one matching diff section, but says nothing about a diff
    # destination that has no REST record at all. Check the reverse direction
    # too, so a diff section smuggled in outside the REST changed-file set
    # cannot silently skip the privacy scan's REST-driven bookkeeping.
    extra_diff_destinations=$(comm -13 <(cut -f1 "$changed_records" | sort -u) <(cut -f1 "$diff_stats" | sort -u))
    if [ -n "$extra_diff_destinations" ]; then
      while IFS= read -r extra; do
        record_coverage_issue "diff destination '$extra' has no corresponding REST record"
      done <<<"$extra_diff_destinations"
    fi
    # This option is an operator statement, never evidence inferred from the
    # PR body: it attests that every current binary addition/change byte and
    # its ownership, license, and NOTICE obligations were independently read.
    if [ "$binary_count" -gt 0 ]; then
      if [ "$BINARY_REVIEW_SHA" = "$head" ]; then
        say "ok" "$binary_count binary addition/change(s) have the owner's current-head binary review attestation"
      else
        bad "$binary_count binary addition/change(s) require the owner's own --binary-review-sha attestation (no review line can vouch for binary bytes; first 8 at most: $binary_examples)"
      fi
    fi
    [ "$gitlink_count" -eq 0 ] || unknown "$gitlink_count gitlink change(s) require explicit provenance, license, and NOTICE review"
    if [ "$coverage_count" -gt 0 ]; then
      unknown "privacy diff coverage failed for $coverage_count non-removed REST file(s): $coverage_examples"
    fi
    if [ "$metadata_only_count" -gt 0 ]; then
      say "note" "$metadata_only_count metadata-only diff section(s) have REST 0/0 totals: $metadata_only_examples"
    fi
    # Destination paths are scan input from the complete REST set, not only
    # from whatever textual patch GitHub happened to render. Removed paths
    # carry no newly added material and are deliberately excluded.
    path_text=$(awk -F '\t' '$2 != "removed" { print $1 }' "$changed_records")
  fi
  # Where the scan's hits are: location and category per line, never a value. Runs only when the
  # scan blocked or was indeterminate; a failure here changes no verdict.
  explain_privacy_hits() {
    local input="$tmpdir/privacy-explain-input.json" located
    printf '%s' "$title" >"$tmpdir/ex-title"
    printf '%s' "$raw_prbody" >"$tmpdir/ex-body"
    printf '%s' "${commit_identity_names:-}" >"$tmpdir/ex-names"
    printf '%s' "${path_text:-}" >"$tmpdir/ex-paths"
    printf '%s' "${sanitized_message_json:-[]}" >"$tmpdir/ex-messages.json"
    printf '%s' "$parsed_diff" >"$tmpdir/ex-diff.json"
    if ! jq -n --rawfile title "$tmpdir/ex-title" --rawfile body "$tmpdir/ex-body" \
          --rawfile names "$tmpdir/ex-names" --rawfile paths "$tmpdir/ex-paths" \
          --slurpfile messages "$tmpdir/ex-messages.json" --slurpfile diff "$tmpdir/ex-diff.json" '
        ($diff[0]) as $d |
        {sources: ([{label: "pr-title", text: $title}, {label: "pr-body", text: $body},
                    {label: "commit-identity-names", text: $names}] +
                   ($messages[0] | to_entries | map({label: ("commit-message-" + ((.key + 1) | tostring)), text: .value}))),
         added: ([$paths | split("\n")[] | select(length > 0) | {path: ., line: 0, text: .}] +
                 [range(0; $d.added_payload | length) as $i |
                  {path: $d.added_locations[$i][0], line: $d.added_locations[$i][1], text: $d.added_payload[$i]}])}
      ' >"$input" 2>/dev/null \
       || ! located=$(python3 -I "$script_dir/merge_gate_privacy.py" --head "$head" ${uuid_manifest_file:+--fixture-provenance "$uuid_manifest_file"} --explain <"$input" 2>/dev/null) \
       || ! jq -e '(.hits | type == "array") and (.truncated | type == "number")' <<<"$located" >/dev/null 2>&1; then
      say "note" "could not locate the lines that tripped the privacy scan"
      return 0
    fi
    if [ "$(jq -r '.hits | length' <<<"$located")" -eq 0 ]; then
      say "note" "no single line reproduces the privacy finding (it needs context from several lines, or comes from the whole PR text)"
    else
      while IFS= read -r line; do say "note" "$line"; done < <(jq -r '.hits[] | "at \(.where): \(.messages | join("; "))"' <<<"$located")
      [ "$(jq -r '.truncated' <<<"$located")" -eq 0 ] || say "note" "$(jq -r '.truncated' <<<"$located") more line(s) not listed"
    fi
  }
  # The synthetic-UUID registry: UUID values the owner has listed on master as synthetic test values. It is read as a blob
  # of the master ref fetched above, never from the checkout (a file ignored by git, or a path spelled in another case on a
  # case-insensitive file system, would otherwise carry a pull request's own list into its own run), and only as a regular
  # file. Anything wrong with it means no list, which is what the scan did before the registry existed: an unlisted UUID is
  # indeterminate. A pull request cannot add or remove an entry for its own run.
  uuid_registry_path="scripts/merge-gate-synthetic-uuids.json"
  uuid_manifest_file=""
  load_uuid_registry() {
    local tip entry mode count manifest="$tmpdir/synthetic-uuid-manifest.json"
    # One commit is resolved once and used for every read below, so a second gate run that fetches meanwhile cannot make
    # the mode, the bytes and the printed blob come from different states of master.
    tip=$(git -C "$gate_tree" rev-parse --verify -q "refs/merge-gate/master^{commit}" 2>/dev/null) || {
      say "note" "synthetic-UUID registry not applied: could not resolve master"; return 0; }
    entry=$(git -C "$gate_tree" ls-tree "$tip" -- "$uuid_registry_path" 2>/dev/null) || {
      say "note" "synthetic-UUID registry not applied: could not look it up on master"; return 0; }
    [ -n "$entry" ] || return 0
    mode=${entry%% *}
    if [ "$mode" != "100644" ]; then
      say "note" "synthetic-UUID registry not applied: it is not a regular file on master"; return 0
    fi
    git -C "$gate_tree" cat-file -e "$tip:$uuid_registry_path" 2>/dev/null || {
      say "note" "synthetic-UUID registry not applied: could not read it from master"; return 0; }
    # The bytes go straight from git into jq: capturing them in a shell variable would drop a NUL byte (bash 3.2) and could
    # make an invalid file valid.
    if ! git -C "$gate_tree" cat-file blob "$tip:$uuid_registry_path" 2>/dev/null | jq -e '
      type == "object" and (keys == ["entries", "schema_version"]) and .schema_version == 1 and
      (.entries | type == "array" and length <= 500 and
        all(.[]; type == "object" and (keys == ["note", "uuid"]) and
          (.uuid | type == "string" and length == 36 and test("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$") and . != "00000000-0000-0000-0000-000000000000") and
          (.note | type == "string" and test("[^\\s]"))) and
        (([.[].uuid] | length) == ([.[].uuid] | unique | length)))
    ' >/dev/null 2>&1; then
      say "note" "synthetic-UUID registry not applied: it is not valid (schema 1, at most 500 entries, each with a lowercase non-nil uuid and a note, no duplicates)"; return 0
    fi
    if ! git -C "$gate_tree" cat-file blob "$tip:$uuid_registry_path" 2>/dev/null | jq --arg head "$head" '[.entries[] | {uuid: .uuid, head: ($head | ascii_downcase), source: "fixture"}]' >"$manifest" 2>/dev/null; then
      say "note" "synthetic-UUID registry not applied: could not write the list for the scan"; return 0
    fi
    count=$(jq -r 'length' "$manifest")
    uuid_manifest_file="$manifest"
    say "note" "synthetic-UUID registry: $count listed value(s) applied (blob $(git -C "$gate_tree" rev-parse --short=12 "$tip:$uuid_registry_path" 2>/dev/null))"
  }
  load_uuid_registry
  scan_input_file="$tmpdir/privacy-scan-input"
  scan_input_write_status=0
  {
    printf '%s\n%s\n' "$privacy_metadata" "$path_text"
    cat "$added_payload"
  } >"$scan_input_file" || scan_input_write_status=$?
  if [ "$scan_input_write_status" -ne 0 ]; then
    unknown "could not assemble complete privacy scan input"
  else
    privacy_result_status=0
    privacy_result=$(python3 -I "$script_dir/merge_gate_privacy.py" --head "$head" ${uuid_manifest_file:+--fixture-provenance "$uuid_manifest_file"} <"$scan_input_file") || privacy_result_status=$?
    if [ "$privacy_result_status" -ne 0 ] || ! jq -e '
      type == "object" and
      (.blockers | type == "array" and all(.[]; type == "string" and length > 0)) and
      (.indeterminate | type == "array" and all(.[]; type == "string" and length > 0)) and
      (.notes | type == "array" and all(.[]; type == "string" and length > 0))
    ' <<<"$privacy_result" >/dev/null 2>&1; then
      unknown "privacy classifier failed or returned malformed output"
    else
      while IFS= read -r message; do bad "$message"; done < <(jq -r '.blockers[]' <<<"$privacy_result")
      while IFS= read -r message; do unknown "$message"; done < <(jq -r '.indeterminate[]' <<<"$privacy_result")
      while IFS= read -r message; do say "note" "$message"; done < <(jq -r '.notes[]' <<<"$privacy_result")
      if jq -e '(.blockers | length) + (.indeterminate | length) > 0' <<<"$privacy_result" >/dev/null 2>&1; then
        explain_privacy_hits
      fi
    fi
  fi
  fi
fi
# The owner's merge verdict (merge-check spec v3). This replaces the old "review names the head" and "review names the pinned
# paths" checks. Two lines count, each the WHOLE body of a pull request review or an issue comment by the owner's numeric user id
# (scripts/merge_gate_verdict.py decides, with every rule in its header): MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND, both
# bound to this head and to the merge base this gate computed itself. Any edit, deletion or dismissal of an owner item, a later
# NOT-CLEAN or RETRACT line, or a token in the title, body, branch name, file names, commits, commit comments, added lines or anyone else's comment, blocks. Unreadable
# history is indeterminate. Binary files stay the owner's own act (--binary-review-sha); a pull request that changes this gate or
# the workflows needs the owner's own act too, because a verdict about the gate cannot clear a change to the gate.
: >"$errfile"
verdict_status=0
verdict_pages() {
  gh api --paginate --slurp "$1" 2>>"$errfile" </dev/null \
    | jq -c 'if type == "array" and all(.[]; type == "array") then flatten else error("not a list of pages") end'
}
verdict_reviews=$(verdict_pages "repos/$REPO/pulls/$PR/reviews?per_page=100") || verdict_status=$?
verdict_comments=$(verdict_pages "repos/$REPO/issues/$PR/comments?per_page=100") || verdict_status=$?
verdict_inline=$(verdict_pages "repos/$REPO/pulls/$PR/comments?per_page=100") || verdict_status=$?
verdict_commit_comments="[]"
verdict_shas=$(verdict_pages "repos/$REPO/pulls/$PR/commits?per_page=100" | jq -r '.[].sha') || verdict_status=$?
if [ "$(printf '%s\n' "$verdict_shas" | sed '/^$/d' | wc -l | tr -d ' ')" -gt 60 ]; then
  verdict_status=1
  echo "more than 60 commits: their comments are not read" >>"$errfile"
fi
while IFS= read -r verdict_sha; do
  [[ "$verdict_sha" =~ ^[0-9a-f]{40}$ ]] || continue
  verdict_page=$(verdict_pages "repos/$REPO/commits/$verdict_sha/comments?per_page=100") || { verdict_status=$?; continue; }
  verdict_commit_comments=$(jq -c -n --argjson a "$verdict_commit_comments" --argjson b "$verdict_page" '$a + $b') || verdict_status=$?
done <<<"$verdict_shas"
verdict_query='query($owner:String!,$name:String!,$pr:Int!){repository(owner:$owner,name:$name){pullRequest(number:$pr){
 reviews(first:100){pageInfo{hasNextPage} nodes{databaseId lastEditedAt userContentEdits(first:1){totalCount} author{login ... on User{databaseId}}}}
 comments(first:100){pageInfo{hasNextPage} nodes{databaseId lastEditedAt userContentEdits(first:1){totalCount} author{login ... on User{databaseId}}}}
 timelineItems(first:100,itemTypes:[COMMENT_DELETED_EVENT,REVIEW_DISMISSED_EVENT]){pageInfo{hasNextPage} nodes{__typename ... on CommentDeletedEvent{createdAt deletedCommentAuthor{login ... on User{databaseId}}} ... on ReviewDismissedEvent{createdAt review{author{login ... on User{databaseId}}}}}}
}}}'
verdict_history=$(gh api graphql -F owner="$OWNER" -F name="$NAME" -F pr="$PR" -f query="$verdict_query" 2>>"$errfile" </dev/null) || verdict_status=$?
if [ "$verdict_status" -ne 0 ] || [ -z "$merge_base" ] || [ ! -f "${added_payload:-/nonexistent}" ]; then
  unknown "could not read the reviews, comments, edit history, merge base or added lines that the owner's merge verdict is judged on"
else
  printf '%s' "$verdict_reviews" >"$tmpdir/v-reviews.json"
  printf '%s' "$verdict_comments" >"$tmpdir/v-comments.json"
  printf '%s' "$verdict_inline" >"$tmpdir/v-inline.json"
  printf '%s' "$verdict_commit_comments" >"$tmpdir/v-commit-comments.json"
  printf '%s\n%s\n' "$changed" "$(awk -F '\t' '$5 != "" { print $5 }' "$changed_records" 2>/dev/null)" >"$tmpdir/v-filenames.txt"
  printf '%s' "$verdict_history" >"$tmpdir/v-history.json"
  printf '%s' "${sanitized_message_json:-[]}" >"$tmpdir/v-commits.json"
  verdict_doc_status=0
  jq -n -c --slurpfile reviews "$tmpdir/v-reviews.json" --slurpfile comments "$tmpdir/v-comments.json" --slurpfile inline "$tmpdir/v-inline.json" \
    --slurpfile history "$tmpdir/v-history.json" --slurpfile commits "$tmpdir/v-commits.json" --rawfile added "$added_payload" \
    --slurpfile commitcomments "$tmpdir/v-commit-comments.json" --rawfile filenames "$tmpdir/v-filenames.txt" --arg headref "$head_ref" \
    --arg title "$title" --arg body "$raw_prbody" --arg head "$head" --arg base "$merge_base" '
    ($history[0].data.repository.pullRequest // null) as $pr |
    def edits: [(.nodes // [])[] | {authorId: (.author.databaseId // null), lastEditedAt: .lastEditedAt, editCount: (.userContentEdits.totalCount // 0)}];
    {head: $head, base: $base,
     reviews: $reviews[0], comments: $comments[0], inline_comments: $inline[0],
     commit_comments: $commitcomments[0],
     texts: {title: $title, body: $body, head_ref: $headref, filenames: ($filenames | split("\n") | map(select(length > 0))),
             commit_messages: $commits[0], added_lines: ($added | split("\n"))},
     graphql: {complete: (($history[0].errors == null) and ($pr != null)
                          and ($pr.reviews.pageInfo.hasNextPage == false) and ($pr.comments.pageInfo.hasNextPage == false)
                          and ($pr.timelineItems.pageInfo.hasNextPage == false)),
               reviews: ($pr.reviews // {} | edits), comments: ($pr.comments // {} | edits),
               events: [($pr.timelineItems.nodes // [])[] | if .__typename == "CommentDeletedEvent"
                          then {type: .__typename, createdAt: .createdAt, authorId: (.deletedCommentAuthor.databaseId // null)}
                          else {type: .__typename, createdAt: .createdAt, authorId: (.review.author.databaseId // null)} end]}}' \
    >"$tmpdir/verdict-doc.json" 2>"$errfile" || verdict_doc_status=$?
  if [ "$verdict_doc_status" -ne 0 ]; then
    unknown "could not assemble the evidence for the owner's merge verdict"
  else
    verdict_exit=0
    verdict_result=$(python3 -I "$script_dir/merge_gate_verdict.py" <"$tmpdir/verdict-doc.json" 2>/dev/null) || verdict_exit=$?
    if ! jq -e 'type == "object" and (.status | type == "string")' <<<"$verdict_result" >/dev/null 2>&1; then
      unknown "could not evaluate the owner's merge verdict"
    else
      while IFS= read -r line; do unknown "$line"; done < <(jq -r '.unreadable[]?' <<<"$verdict_result")
      while IFS= read -r line; do bad "$line"; done < <(jq -r '.refusals[]?' <<<"$verdict_result")
      while IFS= read -r line; do say "note" "$line"; done < <(jq -r '.reasons[]?' <<<"$verdict_result")
      if [ "$(jq -r '.status' <<<"$verdict_result")" = "pass" ]; then
        say "ok" "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND stand at head $short and merge base ${merge_base:0:7}"
      elif [ "$(jq -r '.status' <<<"$verdict_result")" = "block" ]; then
        bad "the owner's MERGE-VERDICT CLEAN and MERGE-ADVERSARY RECOMMEND do not both stand at head $short and merge base ${merge_base:0:7}"
      else
        unknown "the owner's merge verdict came back as neither a pass nor a block, so it is not read as a pass"
      fi
    fi
  fi
fi
gate_paths=$(printf '%s\n%s\n' "$changed" "$(awk -F '\t' '$5 != "" { print $5 }' "$changed_records" 2>/dev/null)")
if grep -iE '^(scripts/merge-gate|scripts/merge_gate[^/]*\.py$|\.github/)' <<<"$gate_paths" >/dev/null 2>&1; then
  bad "this pull request changes the merge gate or a workflow: a verdict about the gate cannot clear a change to the gate, so the merge is the owner's own act"
fi
echo
if [ "$fail" -ne 0 ]; then
  echo "MUST NOT MERGE"
  exit 1
fi
if [ "$uncertain" -ne 0 ]; then
  echo "INDETERMINATE — do not merge until the missing evidence is obtained"
  exit 2
fi

echo "MAY MERGE — compatibility-surface acknowledgement, privacy, and the owner's merge verdict checks passed for $short."
echo "Branch protection on $base enforces the remaining required status checks separately."
printf '  [ "$(gh pr view %s --repo %s --json baseRefName -q .baseRefName)" = "%s" ] \\\n    && gh pr merge %s --repo %s --squash --match-head-commit %s\n' \
  "$PR" "$REPO" "$base" "$PR" "$REPO" "$head"
exit 0
