#!/usr/bin/env bash
# Validate a pull request's pinned-path acknowledgement (compatibility surface),
# run the privacy/PII scan, and confirm review evidence names the current head
# SHA (and, when a pinned path is touched, the acknowledgement and those paths).
#
# Usage: scripts/merge-gate.sh <pr-number> [--repo OWNER/NAME]
#        [--independent-review-sha FULL_SHA] (manual attestation naming the head)
#        [--binary-review-sha FULL_SHA] (manual binary-byte, ownership, license, and NOTICE review)
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
INDEPENDENT_REVIEW_SHA=""
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
    --independent-review-sha)
      INDEPENDENT_REVIEW_SHA="${2:-}"
      [ -n "$INDEPENDENT_REVIEW_SHA" ] || { echo "--independent-review-sha needs a full commit SHA" >&2; exit 2; }
      shift 2
      ;;
    --independent-review-sha=*)
      INDEPENDENT_REVIEW_SHA="${1#*=}"
      [ -n "$INDEPENDENT_REVIEW_SHA" ] || { echo "--independent-review-sha= needs a full commit SHA" >&2; exit 2; }
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
        --json headRefOid,baseRefName,title,body,changedFiles 2>"$errfile"); then
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
base=$(jq -r '.baseRefName' <<<"$meta")
changed_files_expected=$(jq -r '.changedFiles' <<<"$meta")
short=${head:0:7}
title=$(jq -r '.title' <<<"$meta")
raw_prbody=$(jq -r '.body // ""' <<<"$meta")
prbody="$raw_prbody"
visible_body_status=0
prbody=$(python3 -c 'import re, sys
text = sys.stdin.read()
print(re.sub(r"<!--.*?(?:-->|\Z)", "", text, flags=re.S), end="")' <<<"$prbody") || visible_body_status=$?
if [ "$visible_body_status" -ne 0 ]; then
  unknown "could not extract visible PR description content"
  prbody=""
fi

if [ -n "$INDEPENDENT_REVIEW_SHA" ] && ! [[ "$INDEPENDENT_REVIEW_SHA" =~ ^[0-9a-fA-F]{40}$ ]]; then
  bad "independent review attestation must be a full 40-hex commit SHA"
elif [ -n "$INDEPENDENT_REVIEW_SHA" ] && [ "$INDEPENDENT_REVIEW_SHA" != "$head" ]; then
  bad "independent review attestation names a different commit than the PR head"
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
  # Author and committer emails were structurally validated above and are an
  # explicit identity-only source class. Do not mix them into payload/path/
  # metadata scan input, where an identical address would be customer data.
  # Commit messages are their own source class. Before scanning them, redact
  # only the one known public-agent address when it occupies an exact
  # terminal Co-Authored-By Git trailer. The helper leaves every contributor
  # name, other trailer, PR field, destination, and payload untouched.
  commit_message_json=$(jq -c '(if all(.[]; type == "array") then flatten else . end) | map(.commit.message)' <<<"$metadata_commits")
  sanitized_messages_status=0
  sanitized_message_json=$(printf '%s' "$commit_message_json" | python3 "$script_dir/merge_gate_privacy.py" --redact-public-agent-attribution-messages) || sanitized_messages_status=$?
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
# touched paths, and a review or comment by the acknowledging reviewer must name
# the head, the acknowledgement path and every touched path (checked below,
# beside the head-SHA review evidence).
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

if [ -n "$changed" ] && [ -n "$pinned" ] && [ -n "$base_pinned" ] && [ -n "$merge_base_pinned" ]; then
  removed_pins=$(LC_ALL=C comm -23 <(path_set "$merge_base_pinned") <(path_set "$pinned"))
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
        removed_missing=$(LC_ALL=C comm -13 <(path_set "$ack_removed") <(path_set "$removed_pins") | sed '/^$/d' | first_lines)
        removed_extra=$(LC_ALL=C comm -23 <(path_set "$ack_removed") <(path_set "$removed_pins") | sed '/^$/d' | first_lines)
        [ -z "$removed_missing" ] || ack_note "lacks a removed-pin line for: $removed_missing"
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
  parsed_diff=$(python3 "$script_dir/merge_gate_diff.py" <<<"$diff") || parsed_diff_status=$?
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
  gitlink_count=0
  bounded_coverage_name() {
    local item
    item=$(sed -E 's#/(Users|home)/[^/[:space:]]+#<home>#g; s#[A-Za-z]:[\\/]+Users[\\/]+[^\\/[:space:]]+#<home>#g' <<<"$1")
    # The privacy classifier's own identifier-shape detection also runs on
    # the (already home-dir-redacted) filename: a coverage/metadata-only
    # example is echoed verbatim into gate output, and a filename can itself
    # carry an identifier, digest, or email shape.
    item=$(printf '%s' "$item" | python3 "$script_dir/merge_gate_privacy.py" --redact-shapes 2>/dev/null) || item="$item"
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
      if [ "$BINARY_REVIEW_SHA" = "$head" ] && [ "$INDEPENDENT_REVIEW_SHA" = "$head" ]; then
        say "ok" "$binary_count binary addition/change(s) have explicit current-head binary and independent review attestations"
      else
        bad "$binary_count binary addition/change(s) require matching --binary-review-sha and --independent-review-sha human attestations"
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
    privacy_result=$(python3 "$script_dir/merge_gate_privacy.py" --head "$head" <"$scan_input_file") || privacy_result_status=$?
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
    fi
  fi
  fi
fi
# A review naming the exact current head SHA. Closes #317: PRs were merged
# 2-8 minutes after opening, where "zero unresolved review threads" meant
# "the reviewer had not started", not "reviewed clean". A clean review can
# leave no review object at all -- only a reaction plus a summary comment --
# so the absence of any review artifact naming this exact head is a FAIL,
# never a silent pass.
: >"$errfile"
review_evidence_status=0
head_reviews=$(gh api --paginate --slurp "repos/$REPO/pulls/$PR/reviews" 2>"$errfile") || review_evidence_status=$?
review_names_head=false
reviews_flat=""
if [ "$review_evidence_status" -eq 0 ] && jq -e '
  type == "array" and (all(.[]; type == "array") or all(.[]; type == "object"))
' <<<"$head_reviews" >/dev/null 2>&1; then
  review_names_head=$(jq -r --arg head "$head" '
    (if all(.[]; type == "array") then flatten else . end) |
    any(.[]; .commit_id == $head)
  ' <<<"$head_reviews")
  reviews_flat=$(jq -c 'if all(.[]; type == "array") then flatten else . end' <<<"$head_reviews")
fi
comment_names_head=false
comments_flat=""
short_marker="\`${short}\`"
# Comments are also a place the pinned-path review may live, so they are read
# whenever an acknowledgement is required, not only as a head-SHA fallback.
if [ "$review_names_head" != "true" ] || [ "$ack_required" -eq 1 ]; then
  : >"$errfile"
  head_comments_status=0
  head_comments=$(gh api --paginate --slurp "repos/$REPO/issues/$PR/comments" 2>"$errfile") || head_comments_status=$?
  if [ "$head_comments_status" -eq 0 ] && jq -e '
    type == "array" and (all(.[]; type == "array") or all(.[]; type == "object"))
  ' <<<"$head_comments" >/dev/null 2>&1; then
    comment_names_head=$(jq -r --arg head "$head" --arg marker "$short_marker" '
      (if all(.[]; type == "array") then flatten else . end) |
      any(.[]; ((.body // "") | contains($head)) or ((.body // "") | contains($marker)))
    ' <<<"$head_comments")
    comments_flat=$(jq -c 'if all(.[]; type == "array") then flatten else . end' <<<"$head_comments")
  fi
fi
if [ "$review_names_head" = "true" ] || [ "$comment_names_head" = "true" ]; then
  say "ok" "review evidence names the current head $short"
else
  bad "no review evidence (review or comment) names current head $short — see #317"
fi
# A PR that touches a pinned path also needs one review or comment that names
# the head, the acknowledgement path and every touched path, written by the
# login the acknowledgement names as reviewer. The login proves nothing when
# several lanes post under one account; the file list is what the reviewer must
# have read. A review counts as naming the head when GitHub bound it to the
# head commit or when its body names the head.
if [ "$ack_required" -eq 1 ]; then
  if [ -z "$reviews_flat" ] || [ -z "$comments_flat" ]; then
    unknown "could not read the review and comment bodies that must name the pinned paths"
  else
    touched_json=$(printf '%s\n' "$ack_touched" | jq -R -s -c 'split("\n") | map(select(length > 0))')
    ack_review=""
    ack_review_status=0
    ack_review=$(jq -n -c --argjson reviews "$reviews_flat" --argjson comments "$comments_flat" \
      --argjson paths "$touched_json" --arg head "$head" --arg marker "$short_marker" \
      --arg ackpath "$ACK_PATH" --arg reviewer "$ack_reviewer" '
      # A path is named only as a whole token: delimited by whitespace, a backtick, a quote, a
      # parenthesis, a comma, a colon or the ends of the text, so "src/a.rs.bak" does not name
      # "src/a.rs" and "pr-3210.txt" does not name "pr-321.txt".
      def esc: gsub("(?<c>[\\\\.^$*+?(){}\\[\\]|])"; "\\" + .c);
      def names($b; $p): $b | test("(^|[\\s`\"\u0027\u201c\u201d\u2018\u2019(),:])" + ($p | esc) + "($|[\\s`\"\u0027\u201c\u201d\u2018\u2019(),:])");
      ([$reviews[] | {login: (.user.login // ""), body: (.body // ""), bound: (.commit_id == $head)}] +
       [$comments[] | {login: (.user.login // ""), body: (.body // ""), bound: false}]) as $all |
      [$all[] | select(.bound or (.body | contains($head)) or (.body | contains($marker))) |
        . + {missing: (.body as $b | ([$ackpath] + $paths) | map(. as $p | select(names($b; $p) | not)))}] as $named |
      ($named | map(select(.missing | length == 0))) as $full |
      {named: ($named | length),
       full: ($full | length),
       matching: ($full | map(select($reviewer == "" or ((.login | ascii_downcase) == ($reviewer | ascii_downcase)))) | length),
       missing: (if ($named | length) == 0 then [] else ($named | min_by(.missing | length) | .missing) end),
       logins: ($full | map(.login) | unique)}' 2>"$errfile") || ack_review_status=$?
    if [ "$ack_review_status" -ne 0 ] || ! jq -e 'type == "object" and (.named | type == "number")' <<<"$ack_review" >/dev/null 2>&1; then
      unknown "could not evaluate the review and comment bodies for the pinned paths"
    elif [ "$(jq -r '.matching' <<<"$ack_review")" -gt 0 ]; then
      say "ok" "review by ${ack_reviewer:-the reviewer} names $short, $ACK_PATH and all pinned paths touched"
    elif [ "$(jq -r '.full' <<<"$ack_review")" -gt 0 ]; then
      bad "the review naming the pinned paths is by $(jq -r '.logins | join(", ")' <<<"$ack_review"), not the acknowledgement's reviewer '$ack_reviewer'"
    elif [ "$(jq -r '.named' <<<"$ack_review")" -gt 0 ]; then
      bad "no review or comment names every pinned path touched; the closest lacks: $(jq -r '.missing | join(", ")' <<<"$ack_review")"
    else
      bad "no review or comment names head $short together with $ACK_PATH and the touched pinned path(s)"
    fi
  fi
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

echo "MAY MERGE — compatibility-surface acknowledgement, privacy, and head-SHA review-evidence checks passed for $short."
echo "Branch protection on $base enforces the remaining required status checks separately."
printf '  [ "$(gh pr view %s --repo %s --json baseRefName -q .baseRefName)" = "%s" ] \\\n    && gh pr merge %s --repo %s --squash --match-head-commit %s\n' \
  "$PR" "$REPO" "$base" "$PR" "$REPO" "$head"
exit 0
