# Release process

Bridge supports development and unsigned bundle smoke validation on Windows
and macOS. A smoke bundle is not a production release.

## Supported build baseline

- Source release line: 0.2.0 and later under Apache-2.0 (`v0.1.0` was MIT)
- Node.js: supported 24.x releases (>=24.15.0); CI uses `.node-version`
- pnpm: the exact `packageManager` version in `package.json`
- Rust: the exact channel and components in `rust-toolchain.toml`
- Hosts: current GitHub-hosted Windows and macOS runners plus native maintainer
  validation for vendor integrations

## Candidate gates

### Compatibility-surface reseal

The heading keeps its old name so existing links resolve. Since schema 3 there is
**no reseal**: nothing is regenerated, no hash is stored, and no tool rewrites the
surface. What replaces it is an acknowledgement filed with the pull request.

Any change to a pinned file needs a deliberate acknowledgement filed with the
pull request. That includes `package.json`, `src-tauri/Cargo.toml`,
`src-tauri/Cargo.lock` and either workflow, and it is not only dependency
updates. **The canonical `docs/tally/TALLY_PROTOCOL_REFERENCE.md` index and each
part it declares are pinned sources, so a documentation-only edit to either needs
an acknowledgement. Adding or removing a declared part also changes the pin list;
see "Adding or removing a pin" below.** Nothing in a docs diff suggests a
compatibility gate is involved.

**Enforcement.** The GitHub check enforces. On a pull request and in the merge
queue, the `workflow-consistency` job fails with
`surface ack check FAILED: <reason>` when a pinned path changed without its
acknowledgement file, or the acknowledgement is wrong, edited or unneeded, and
that job is part of `Required checks`. A push to master is enforced the same way
(below). `scripts/merge-gate.sh`, run locally by whoever merges, is a second,
independent check of the same rule. The step first ran with `--report-only`,
which printed `WOULD FAIL: <reason>` and exited 0; the script
still accepts that flag for local use, but the CI step must not carry it, and
`scripts/check-ci-workflow-consistency.mjs` pins the step's exact text so that
putting it back is a pinned change with its own acknowledgement.

What this trades away, stated plainly. Under schema 2 the required
`Tally portable core` job failed a pull request, and again the master push, when a
pinned file's bytes differed from its stored hash. Nothing stored remains to
compare, so nothing can fail on a byte difference; the acknowledgement replaces it.
It is procedural assurance, not authentication: a missing or wrong file fails the
check, but a hand-written file that names the right paths and a reviewer passes it,
so the review is what stands behind it. The base race is still real: the pin list
is read when the check runs but the merge binds only the head, so another pull
request that pins a file after this one was checked lets an unacknowledged change
land. A push to master is checked like a pull request and is never report-only.
Every first-parent commit the push landed (`before..HEAD`, `before` from the event
payload; each one a squashed pull request, attributed by the `(#N)` in its
subject) must carry exactly the acknowledgement its pinned changes need, and the
master run goes red if any does not. A push whose `before` is missing, new or not
an ancestor of HEAD (a force-push) cannot be verified and fails. That red master
needs an acknowledgement-only follow-up pull request to clear.

#### What the surface is

`docs/tally/compatibility/compatibility-surface.json` (schema 3) is an authored,
sorted, unique list:

```json
{
  "schema_version": 3,
  "files": [
    { "path": ".github/workflows/ci.yml" },
    { "path": "src/NewScreen.tsx", "reason": "Decides what the client sees before a post." }
  ]
}
```

- `files` rows hold a repository-relative `path` and an optional `reason`
  (at most 500 characters). There is no `sha256`, and no other key.
- The top level has exactly `files` and `schema_version`. A schema 2 file, with
  stored hashes, is refused with a message naming the migration.
- No surface digest is stored or computed. The crate that computed one from the
  live bytes of every pinned file, and the receipts that bound it, were removed
  (#1745); the acknowledgement file and the review are the control.
- Because no per-file value is stored, two pull requests that change different
  pinned files, or even the same pinned file, no longer conflict in the surface.
  Each adds its own uniquely named acknowledgement.

#### Acknowledging a change to a pinned file

A pull request whose diff changes a pinned file (any status, under its old and its
new name), adds a pin, or removes a pin adds exactly one file,
`docs/tally/compatibility/acks/pr-<N>.txt`, where `<N>` is the pull request number.
An edit to the pin list that adds or removes no pin (only a `reason` changed, or
rows reordered) needs no acknowledgement, and adding one when nothing pinned
changed is an error. Its lines are:

```text
docs/tally/TALLY_PROTOCOL_REFERENCE.md
src/ClientSwitcher.tsx
removed-pin: src/OldScreen.tsx
reviewer: some-github-login
```

- One repository path per line, sorted and unique: every pinned path the diff
  changes, and every pin the diff adds. "Changes" means any status: added,
  modified, deleted, type or mode change, and both the old and the new name of a
  rename. Pinned means pinned at the base or at the head.
- One `removed-pin: <path>` line for each pin the diff removes from the list.
  A removed pin is a decision, so it is named.
- Exactly one `reviewer: <github login>` line.
- No hashes, and no other content: a 64-hex token is refused, so that two pull
  requests editing the same file never disagree over an acknowledgement.
- `.gitattributes` (its end-of-line rules decide the bytes that are hashed) and
  `scripts/check-surface-ack.mjs` are ordinary pins, each with its own `reason` in
  the list; there is no special rule for them.
- A pull request that touches no pinned path adds no acknowledgement. A pull
  request that only deletes old acknowledgements (a cleanup) passes.
- An acknowledgement is append-only for its own pull request: an existing one is
  never modified. After merge it means nothing, and old files may be cleaned up.
- The branch name is not the file name, so any branch name is fine.
- The number is known only once the pull request is open, so the first run of a
  pull request that touches a pinned path fails with `found 0` until
  `pr-<N>.txt` is pushed to its branch. That is expected, not a breakage: open
  the pull request, then push the file.
- Dependency-update pull requests (Dependabot's npm, cargo and GitHub Actions
  updates) change pinned files (`package.json`, `pnpm-lock.yaml`,
  `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock` and pinned workflows), so each
  one fails `Required checks` until a person pushes its `pr-<N>.txt`. The local
  merge gate already asked for the same file; the difference is that CI now
  reports it too.

The `reviewer:` login is procedural assurance, not authentication: the file is
written by hand and proves nothing by itself. What it records is that a named
person read the pinned changes and stands behind them. Say which files you
looked at in the review.

**What checks it**

- CI runs `scripts/check-surface-ack.mjs` in the `workflow-consistency` job. It
  fails on a missing, wrong, edited or unneeded acknowledgement (see above).
  Its modes:
  - `pull_request`: diffs the merge commit against its first parent (`HEAD^1`),
    after asserting that `HEAD^2` equals the pull request's head commit.
  - `merge_group`: checks each first-parent commit in
    `base_sha..head_sha` against its own acknowledgement, and fails closed when a
    commit cannot be attributed to one pull request.
  - `push` (master): every first-parent commit in `before..HEAD` is checked like a
    pull request, each attributed by the `(#N)` in its subject, and `--report-only`
    never softens a failure; a missing, all-zero or non-ancestor `before` fails closed. It also
    validates that every file in the acknowledgements directory is well formed.
  - `workflow_dispatch`: only the acknowledgements directory is validated.
  The checker runs from the pull request's own tree, so a pull request could weaken
  it; that is why the script and `ci.yml` are pinned, which makes the change
  visible and acknowledged. A change that makes the checker skip itself shows no
  failure in its own pull request run, so the pin alone does not make it red
  in CI; `scripts/merge-gate.sh`, which is also pinned (with
  `scripts/check-ci-workflow-consistency.mjs`, which enforces the step's shape), is
  what sees it. Run the gate from a checkout of the base branch, not from the pull
  request's own tree, or a weakened copy gates itself.
- `scripts/merge-gate.sh` (run locally by whoever merges) reads the pin list at
  the head and requires the same `pr-<N>.txt`. It measures removed and added pins
  against the pull request's merge base (not the base tip), and reads the base tip
  only to know which files are pinned there, so a branch that is behind a
  master commit that added a pin is not reported as indeterminate. It no longer
  matches a review to the acknowledgement's `reviewer:` line (the line is still read for
  its shape); the owner's merge verdict, below, replaces the old review-naming checks.
- **Branch history, in both checks.** The diff against the base cannot see a pin
  the branch added and then lost, for example when a merge of master is resolved by
  taking master's pin list: at the head the list equals the base's. So
  `scripts/check-surface-ack.mjs` and `scripts/merge-gate.sh` also read the pin list at
  every commit of the pull request and at each commit's parents (a merge is compared
  with all of them, never the first only). A path that a commit pins and none of its
  parents pinned, and that is in neither the base list nor the head list, counts as a
  removed pin: the acknowledgement must declare it with a `removed-pin:` line, or the
  pin must be restored. In an older commit only the paths matter, so both read them
  leniently (valid JSON with a `files` array whose rows each have a path string); the
  strict rules (sorted, unique, exact keys, schema) stay as they are for the list at
  the head and at the base, so a branch is never stuck because an old commit had its
  list out of order. Both fail closed on a shallow clone and on a commit whose list
  cannot be read at all (the script fails, the gate is indeterminate). The CI step
  blocks (it no longer runs with `--report-only`, so a finding fails the job), and the
  merge gate blocks too. A pin that follows a rename inside the branch (one commit pins a new
  file, a later one renames it and moves the pin) reports the old name as withdrawn:
  declare it with one `removed-pin:` line for the old name. What these checks cannot
  see: a pin lost in a rebase, a force-push, an amend or a squash before the push
  leaves no history to read, and the merge group and the push to master each check one
  squash commit, which has no branch history, so there the pull-request-time check
  and the merge gate are the control. Only a review that compares the pin list with
  the last reviewed head and a stay-pinned guard test (for the pins it names) can see
  a rewritten history.
- The `Tally portable core` job runs the `compatibility_surface_stays_whole` test of
  `bridge-tally-core` (`cargo test -p bridge-tally-core --test compatibility_surface_stays_whole`),
  which checks required-file coverage and that every pinned file exists as a regular
  file. It compares no hash, so a changed pinned file does not fail it by itself.

A surface digest, which receipts and attestations once bound, was computed from the
live bytes of every pinned file by the `bridge-tally-compatibility` crate. That crate
and its receipts were removed (#1745); the acknowledgement file and the review are
the control.

Two byte changes have no pinned path in a diff, and are closed differently. A
`.gitattributes` below the repository root (end-of-line, `ident` or
`working-tree-encoding` rules) changes the bytes a pinned file is hashed as, so the
acknowledgement checks refuse any nested `.gitattributes` outright. A pinned path
that is, or sits under, a symbolic link would make a pin follow an unpinned
target, so the surface test refuses it (`surface_file_symlink`).

A pull request stacked on another carries
its own acknowledgement, so when the child merges into the parent branch and the
parent then goes to master, it holds two. Fold them into the parent's `pr-<N>.txt`
(the union of the two path lists) before the parent merges; both checkers require
exactly one new acknowledgement.

#### Adding or removing a pin

Edit the sorted list by hand.

1. Insert the entry in sorted path order with a `reason` (required for a pin added
   by a pull request, at most 500 characters): why this file decides what Bridge
   posts or lets leave the machine. Paths are relative, unique and sorted.
2. Leave `MAX_PINS` in `src-tauri/crates/bridge-tally-core/tests/compatibility_surface_stays_whole.rs`
   alone. It is a fixed parse bound against a runaway list (1024), not a count to
   maintain, so adding or removing a pin does not touch it and two pull requests
   that each add a pin no longer collide on it. Only a list that approaches the
   bound would justify raising it, which is a deliberate change to a pinned file
   with its own reason and acknowledgement.
3. Add the acknowledgement, listing the new pin's path (and a `removed-pin:` line for
   each removal). Then run `cargo test -p bridge-tally-core --test compatibility_surface_stays_whole`.

Removing a pin is done the same way, with a `removed-pin:` line. A malformed list
is refused by the tool and by CI.

**What decides a pin now.** Until the bound replaced the exact count, adding a pin
meant editing that constant in a pinned file, so the change could not go unnoticed
in a diff of `lib.rs`. That no longer holds. A pin is now decided by its own
`reason` (required on every pin a pull request adds), the sorted-and-unique and
declared-removal rules the tool and the acknowledgement check enforce, and the
acknowledgement file, which `workflow-consistency` enforces in CI on pull requests,
in the merge queue and on the push to master. A pull request that adds or removes a
pin without its acknowledgement fails, whatever else it edits.

One narrower case remains. A branch adds a pin, and a later merge of master is
resolved by taking master's pin list, which drops that pin. The branch-history check
(see "What checks it") catches that, including when the acknowledgement is edited or
deleted to match: the pin counts as withdrawn, so the acknowledgement must declare it
with a `removed-pin:` line, or the pin must be restored. It is silent only when
history is rewritten (a rebase, a force-push, an amend or a squash before the push)
so that no commit records the pin. Before the bound replaced the exact count, a
reviewer comparing the constant with the pin count could notice that mismatch too;
that signal is gone. So a review of a pull request that touches the pin list compares
the list with the last reviewed head as well as with the base, and names any pin that
was present at the last reviewed head and is gone. That review comparison and a
stay-pinned guard test (for the pins it names) are the only things that can see a
rewritten history.

What remains of the old collision: two pull requests that insert a pin at the same
sorted position still conflict in the pin list itself. Resolve it by keeping both
entries (see "When the surface conflicts in a merge"); the cap is no longer part of
it.

#### What the coverage report shows

`scripts/surface_coverage_report.py` prints a surface coverage report. Run it by
hand before opening a pull request that moves code. It never fails. Against the
merge-base with `origin/master` (override with `--base`) it lists two things the
gate cannot see: a pin that was dropped, and a module declared directly by a
pinned module and newly left unpinned. Modules left unpinned before the branch
are not reprinted, test-only modules are only counted, and feature-gated ones are
labelled.

**A clean report is not evidence that nothing left the pinned surface.** It does
not see code moved between files that already existed; a new module declared by
an *unpinned* module, even one carved out of a pinned file (a new file under an
unpinned `db/mod.rs`, say); deeper descendants of a pinned module; a pinned
file that stops being compiled; a test-only or feature-gated module becoming
production; or a new crate root. The script's docstring keeps the full list.

Read it, then pin each listed file that decides what Bridge posts or lets leave
the machine, and leave the rest; see the module documentation of
`compatibility_surface_stays_whole` for the rule and bridge#416 for the reasoning.

#### When the surface conflicts in a merge or rebase

The pin list is authored and no hash is stored, so a conflict in
`compatibility-surface.json` means both sides edited the list (or both inserted
at the same sorted position). Merge it by hand and be precise about the hazard:

**A dropped entry can slip through.** The gate checks the pins that are
still in the list. Lose one in the resolution and the gate does not fail. That is why the
list must be *merged*, never resolved by taking one side wholesale. (A dropped pin
that existed at the base is a removed pin, which `scripts/merge-gate.sh` blocks
unless the acknowledgement has a `removed-pin:` line. A pin that only the branch
itself added, dropped in a resolution, is not seen by anything.)

**Where a pin matters enough that this is unacceptable, make it REQUIRED.**
`REQUIRED_FILES` and `REQUIRED_DIRECTORIES` in the `compatibility_surface_stays_whole`
test are checked for presence, so an entry in either cannot be dropped silently at all.
`removing_a_required_file_pin_is_refused` iterates that list, so adding
a path also tests it. A procedure a maintainer must follow is weaker than a
constant they cannot circumvent.

1. Resolve every non-generated conflict and settle those files completely.
2. Reconcile the pin list against the merge base. List the pins each side added and
   removed and confirm the union of additions minus the union of removals is
   present.

   **Name the two sides explicitly; during a rebase `HEAD` is not your branch.**
   When a rebase stops on a conflict, `HEAD` is the upstream plus whatever has
   already been replayed, and the commit being applied is `REBASE_HEAD`. The
   conflict stages say it without either name: stage 2 is the side you are
   replaying onto, stage 3 the side being applied.

   ```bash
   surface=docs/tally/compatibility/compatibility-surface.json
   pins() { python3 -c 'import json,sys; [print(f["path"]) for f in json.load(sys.stdin)["files"]]' | sort; }

   git show ":1:$surface" | pins > /tmp/pins-base.txt   # merge base
   git show ":2:$surface" | pins > /tmp/pins-ours.txt   # replayed onto / current
   git show ":3:$surface" | pins > /tmp/pins-theirs.txt # being applied / incoming

   comm -13 /tmp/pins-base.txt /tmp/pins-ours.txt   # added by one side
   comm -13 /tmp/pins-base.txt /tmp/pins-theirs.txt # added by the other
   comm -23 /tmp/pins-base.txt /tmp/pins-ours.txt   # removed by one side
   comm -23 /tmp/pins-base.txt /tmp/pins-theirs.txt # removed by the other
   ```

   Removals need the same treatment as additions. A pin one side deliberately
   retired is still present in the base, so it appears in neither `comm -13`
   output; take the other side wholesale and it comes back. Where one side removed
   an entry the other side *modified*, that is a genuine add/remove conflict and
   wants a decision, not a default; say which way in the commit.

   If the conflict is already resolved and the stages are gone, use `REBASE_HEAD`
   (rebase) or `MERGE_HEAD` (merge) for the incoming side, never `origin/master`.
3. Leave `MAX_PINS` alone: it is a fixed bound, not a count.
4. Run the surface test, and check the pin count against the union you computed; the test
   cannot do this for you. Add or update the acknowledgement so it lists every
   changed pinned path and every added pin.

A rebase carrying several commits that touch pinned files needs the acknowledgement
on the pull request, not per commit; CI checks the merge result.

#### Migrating an open branch across schema 3

A branch cut before schema 3 still carries a schema 2 surface file (a `sha256` on
every row). Merging master into it conflicts on that file. Once per branch:

1. Merge master and resolve every source conflict:

   ```sh
   git fetch origin
   git merge origin/master
   ```

2. Take master's surface file wholesale (schema 3):

   ```sh
   git checkout origin/master -- docs/tally/compatibility/compatibility-surface.json
   ```

3. Re-add the pins the branch itself added. `git diff <merge-base>..HEAD --
   docs/tally/compatibility/compatibility-surface.json` shows them. Insert each as
   `{ "path": "...", "reason": "..." }` in sorted order; do not touch
   `MAX_PINS`. A branch that only changed the
   contents of already-pinned files has nothing to re-add.
4. Add `docs/tally/compatibility/acks/pr-<N>.txt` as above if the branch changes any
   pinned path. Until the branch has merged master, its head still holds a schema 2
   file, which the acknowledgement check and `scripts/merge-gate.sh` refuse to read
   at the head (the merge gate reports the branch as indeterminate).
5. Delete the leftover local merge-driver configuration, once per clone (harmless
   if absent): `git config --remove-section merge.bridge-compat-reseal`.

`scripts/reseal.sh`, `rehash-surface` and the merge driver no longer exist. Do not
recreate them.

#### Running the surface test

The surface test (and any cargo command here) needs the pinned toolchain. A Homebrew
`rustc` earlier on `PATH` shadows rustup, and this project pins the version in
`rust-toolchain.toml`, so check `rustc --version` first. Setting `RUSTC` alone
is **not** enough to escape the shadow: `cargo clippy` still resolves the wrong
`rustc` unless the toolchain's `bin` is prepended to `PATH`, and doctests need
`RUSTDOC` set or they fail with `E0514`, which reads like a source error and is
not one. Prepending the toolchain's `bin` to `PATH` covers all three:

```bash
channel="$(sed -n 's/^channel *= *"\(.*\)"/\1/p' ../rust-toolchain.toml)"
rustc_path="$(rustup which --toolchain "$channel" rustc)"
export PATH="$(dirname "$rustc_path"):$PATH"
rustc --version   # must match rust-toolchain.toml before you continue
```

Quote `rustc_path` rather than piping it through `xargs dirname`: `xargs` splits
on whitespace, so a home directory containing a space turns one path into
several and the `PATH` entry it builds points nowhere.

Before cutting a candidate, regenerate and verify the Rust third-party notice
with the pinned generator:

```sh
cargo install --locked cargo-about --version 0.9.1 --features cli
corepack pnpm run license:generate:rust
corepack pnpm run license:all
```

1. Update `package.json`, `src-tauri/Cargo.toml`, and
   `src-tauri/tauri.conf.json` to the same version.
2. Move completed changelog entries into that version and retain the MIT notice
   for historical `v0.1.0`.
3. Run `corepack pnpm install --frozen-lockfile` and
   `corepack pnpm run license:all`.
4. Run frontend build, Rust format/check/test/Clippy, and native Tauri bundle
   builds on Windows and macOS.
5. Confirm the bundle-smoke jobs pass their automated MSI, NSIS, staged macOS
   app, and mounted DMG inspections for `LICENSE`, `NOTICE`,
   `THIRD_PARTY_LICENSES.txt`, and `THIRD_PARTY_LICENSES_RUST.txt`; manually
   inspect signed candidates again before publication.
6. Exercise Tally, documents, sync, and persistence using synthetic data;
   attach redacted evidence to the release PR.
   Keep repository-synthetic parser qualification receipts separate from live
   Tally behaviour: they cannot establish a product release,
   Education/licensed mode, HTTP/runtime behavior, or a performance budget.
   Do not broaden a Tally support claim without current exact-profile live
   evidence; state missing live evidence in the release notes. The project keeps no
   compatibility matrix or claim gate (removed with the `tools/` workspace, #1745):
   the evidence is the lab items' answers and their reports.
7. Confirm the release commit and tag contain no PII, machine paths, secrets,
   or unsigned third-party assets. The MCPB's PDFium library is fetched at build
   time, never committed, and admitted only against the SHA-256 digests in
   `packaging/pdfium/pdfium.lock.json`; it ships unsigned inside the preview
   archive, which the preview release notes state.
8. Confirm the `Dependency security` workflow passes and GitHub reports no open
   Dependabot or secret-scanning alerts.

## Merge gate: privacy scan and binary attestations

`scripts/merge-gate.sh` scans the PR title, body, every commit message, the
author and committer names, the changed file names and every added line for
developer-home paths, PEM envelopes, credentials, customer email addresses,
UUIDs without provenance, identifier shapes (PAN, GSTIN, mobile numbers) and long
digit runs. The scan counts what it finds and never prints a value.

**Finding the line.** When the scan blocks or is indeterminate the gate also
prints `at <where>: <categories>` for up to 50 lines, where `<where>` is `pr-title:1`,
`pr-body:<line>`, `commit-message-<n>:<line>`, `commit-identity-names:<line>`, a changed
file name, or `<file>:<new-file line>` for an added line. It reports the location and
the classifier's own counts and never the matched text or an excerpt, so the output
is safe to paste. Open the file at that line to decide whether it is synthetic.
"No single line reproduces the privacy finding" means the hit needs several lines
together (a number split over lines) or comes from the PR text as a whole.
Synthetic test data still trips the shapes: any ten digits starting 6 to 9 reads as an
identifier and any run of eleven or more digits as a long digit run, however obviously
fake. The gate has no allowlist by
design; write the synthetic value in a form the shapes do not match (letters in it,
a shorter run, grouped digits), or have the merger read the listed lines and merge on
that reading.

**Merge commits.** A merge made with conflicts carries git's own comment block
(`# Conflicts:` and `#<TAB>path` lines) after the `Co-Authored-By` trailer. The gate
treats that trailing block as outside the trailer footer, so the public agent address
before it is still redacted; an address placed inside the block is still counted. Even
so, strip the block from a merge commit message (or edit it with `git commit`) so it
carries no noise.

**The owner's merge verdict.** The gate no longer reads a reviewer's comment for its name or its
head. Two machine lines count, each either the whole body of a pull request review or an issue
comment (never an inline review comment) or the exact first line of a longer one (see "A
verdict line that opens a longer comment" below), posted by the owner's account (a numeric user id
the script holds, a real user, not an app; the gate fails closed if the id cannot be
read), after CRLF becomes LF and at most one trailing newline, exactly one line of
lowercase hex:

    MERGE-VERDICT CLEAN head=<40 hex> base=<40 hex>
    MERGE-ADVERSARY RECOMMEND head=<40 hex> base=<40 hex>

The newest line of each kind by server time is the one that counts, and it must name the
pull request's current head and the merge base the gate computes itself from a freshly
fetched base tip, so merging master into the branch voids a verdict (except that a CLEAN
line is carried across when the pull request's own change is byte-identical, see "Carrying a
CLEAN line across a merge of master" below) while a master that
moved under an unchanged branch does not (the merge queue's own run on the merged tree
guards that case). Both lines are needed, except at review tier one, where MERGE-VERDICT CLEAN
alone is enough (see "Review tiers" below). An edit or deletion of any review or issue
comment the owner's account posted, by whoever made it, a dismissal of an owner review, and
any later owner line that is not a positive one (`MERGE-VERDICT NOT-CLEAN`, `MERGE-VERDICT
RETRACT`, `MERGE-ADVERSARY REJECT`, `MERGE-ADVERSARY RETRACT` or any malformed line that
carries either token) void every verdict that is not strictly newer; an owner's inline
review comment that carries a token voids every verdict whatever its time, because its edit
history and deletion are not read (only deleting it lifts that). Edits, deletions and dismissals
of items other accounts posted are ignored, except that an author the gate cannot read (a bot, a
deleted account) is treated as possibly the owner and voids; the owner is matched by numeric id,
not by login. The gate refuses a
pull request whose title, body, branch name, changed file names, commit messages, added lines
or commit comments carry either token, or on which anyone but the owner posted a review or
comment (an inline review comment included) that does, so a stranger's comment that carries a
token blocks the pull request until it is deleted; an owner's commit comment or inline comment
that carries one voids every verdict instead (so those two places never refuse, they void). The tokens are read
without case, with look-alike hyphens, hidden characters and character references folded away, and
read three more ways: with HTML comments removed, with only the comment delimiters removed (a token
hidden inside a comment is still a token), and as an HTML parser reads the text, with and without
its comment text (tags and comment forms between the letters do not hide one). A pull request that
quotes one in any of those places (this very section included) cannot pass the gate: its merge is the
owner's own act. The commit comments are read once, from the repository's list of commit comments, and kept
when they sit on one of the pull request's commits; the commit list must first be proven complete (250 commits
at most). A list that is not proven, a page that cannot be read, or a comment that names no commit makes the
gate `INDETERMINATE`. Other places a person can write in (labels, the milestone, author names, comments on
commits no longer in the pull request) are not read. History it cannot read
completely (more than one page of reviews, comments or timeline events included) is
`INDETERMINATE`, never a pass.

**Review tiers.** The gate asks `scripts/review-tier.mjs` for a tier before it judges the verdict
lines, from the changed paths (a rename counts both names) and the committed list
`scripts/review-tier-paths.json`, which the gate proves equal to master's like the rest of
`scripts/`. The first rule of the list whose match holds decides a path. Documentation, plain
tests and production code that does not touch the write path are tier **one**; a path on the write
path, in credentials or security, in fixture data, or that no rule classifies is **full**, and so
is a change with no paths, an unreadable input, an invalid list and any failure or malformed
answer of the tool. A Rust test file is judged with the module it tests. Only the exact tier `one` lets the
MERGE-ADVERSARY RECOMMEND line be absent: the pull request then stands on MERGE-VERDICT CLEAN
alone. Every refusal and every void still applies at tier one, and an adversary line that voids
(a rejection or a retraction) still voids the CLEAN line. The list is a judgement about names and
directories, not about what a change does: an interface file edited to call a write path stays tier
one unless its name or its neighbours say otherwise, which is what the reviewer's CLEAN is for.
Prose (documents and site pages) is classified by the documentation rules before the name rules,
so a document about a write-path subject is tier one. The gate prints the tier and what decided it
("review tier: ..."). A change to the list or the tool is the owner's own act, like a change to
the gate.

**A verdict line that opens a longer comment.** An owner review or comment may open with the
verdict line: its exact first line is the line, the rest of the body is non-blank prose, and that
prose carries neither token (read the same way as everywhere else). A review that opens with the
line is then the one thing the reviewer posts. Prose before the line, a line with spaces around
it, a token anywhere else in the body, or a rest that is empty makes it a malformed owner line,
which voids as before.

**Carrying a CLEAN line across a merge of master.** When the newest MERGE-VERDICT CLEAN line is
not voided and names an older head and merge base, the gate compares two GitHub compare diffs,
one at that pair and one at the current merge base and head. The line is carried to the current
pair only when they are byte-identical apart from the `index` lines and the text after the hunk
numbers (context lines and hunk numbers are part of the comparison, so a change moved in a file is
not the same change), neither is empty or holds a binary change, the new one lists no file twice
and names exactly the pull request's current file list. A pull request whose binaries are all
listed in the lab-capture manifest, none needing the owner's `--binary-review-sha` attestation
and none removed, renamed or modified (only a binary addition is carried, because a modified
binary's base blob is not bound), can still be carried: each binary section is then replaced by its
path and its full 40-hex blob id from the compare API, so the blob ids are compared in full, the id
for the current range must be the blob the manifest check verified, and a file list the API may
have truncated (300 files or more) or a missing id is not carried. The gate prints the reason a
verdict was not carried as a "verdict not carried" note. When the
CLEAN line is carried, the MERGE-ADVERSARY RECOMMEND line is carried with it under the same proof,
but only if it names exactly the same older head and merge base as the carried CLEAN line (never any
other older pair, and never when CLEAN was not carried); a RECOMMEND line that names the current
pair stands on its own. Every void applies to a carried line exactly as it does to the carried
CLEAN line, and the gate's output says when the RECOMMEND line was carried. The review tier is
decided at the current pair from master's classification, so a reclassification on master can
newly require a RECOMMEND line that the older pair did not need. What
master changed in other files, or in the same file outside the pull request's hunks, is not looked
at: the required checks at the head cover it, not the reviewer. When the pull request's own lines
cannot be shown unchanged the gate says so and the reviewer looks again.

**Merging as the gate judged.** `scripts/merge-gate-merge.sh <pr>` runs the gate, and merges only
if it passed with exactly one MAY MERGE line, one merge command and one base-branch check for that
pull request. It reads the pull request, marks a draft ready, reads it again, and runs
`gh pr merge --squash --match-head-commit` with the head the gate printed, returning a draft it
made ready to draft when the merge does not happen. It refuses any mismatch of state, base or head,
and an interrupt while `gh` runs leaves the pull request as it is.

The gate is run only from origin/master's tree, never from the pull request's checkout: it
fetches the branch `master` of `origin` itself into a ref of its own and refuses to run (exit 2) unless
the directory the script itself lives in is this tree's `scripts/` and everything under `scripts/`
equals it, untracked files included, and its helpers run with an isolated Python interpreter
so that a file placed beside them cannot replace a standard-library module. A pull request
that changes or renames away `scripts/merge-gate*`, `scripts/merge_gate*.py`, `scripts/review-tier*` or anything
under `.github/` is the owner's own act: a verdict about the gate cannot clear a change to
the gate. For a pull request that the owner did not author, the two lines must
come from separately run reviewers, and each reviewer records in its own log which line it posted;
the gate cannot check that. The gate trusts the `origin` remote of the clone it runs in, and
ignored files (caches, logs) are not part of the tree comparison, which is harmless because the
helpers run isolated, and a path typed in a different letter case than the real one is refused. A root `.gitattributes` can change the bytes of the gate's files in a working
tree; that can only break the gate (it then fails), not make it pass.

Accepted residual risks, stated here so nobody reads the gate as stronger than it is: the lines bind
the head and the merge base, not the name of the branch the pull request targets, so retargeting a
pull request to another branch with the same merge base keeps the verdict; a
reviewer session that has been tricked into obeying text it was asked to review can post
both lines under the owner's account; a review posted by the owner's account through an
application token cannot be told from one typed by the owner, because a review object carries
no application field (an issue comment does, and one posted through an application is not
accepted); the contributor's repository administration rights (deliberate) can bypass any
gate.

**Binary files.** Git shows a file it reads as binary (a NUL byte, so every UTF-16
fixture, images, archives) as `Binary files differ` or a `GIT binary patch`, with no
text to scan. The gate cannot read those bytes, so it holds the PR until the owner
attests: `scripts/merge-gate.sh <PR> --binary-review-sha <head>` with the full 40-hex
head SHA. This flag is the owner's own act, never read from the PR body or from a
comment (the gate cannot tell who typed it: that it is the owner is kept by who runs the
command), and no verdict line can stand in for it: a diff never shows binary bytes,
licence or NOTICE obligations. It must equal the head the gate is running on, so a later
push invalidates it. It states that the owner read every binary addition or change (the
bytes, not the file name), its ownership and licence and its NOTICE obligation; for a
captured fixture this means its `PROVENANCE` entry matches the bytes
(`scripts/check-fixture-provenance.mjs`) and the capture is synthetic. The gate lists
the first eight binary paths in its message so the merger knows what to read. The former
`--independent-review-sha` flag is gone.

**Lab-capture manifest.** `scripts/merge-gate-lab-captures.json` (schema 1, at most 1000
entries) lists captures the owner has already read, one entry for each: `path`, `blob` (the git
blob id), `size`, `sha256`, `capture_id`, `lab_ref` (a note, never resolved), `company` and
`seed_basis` (how the lab company's books were built: the owner's statement that the book is
synthetic, which the gate does not check). The gate reads the file only from the
master ref it fetched, as a regular-file blob, the way it reads the synthetic-UUID registry, so
a pull request cannot list its own binaries. A binary addition or change is exempt from
`--binary-review-sha` only if its path and the blob id GitHub reports for it both equal one
entry, and the bytes of that blob, fetched through the API each time the gate runs, hash to
that blob id and have the entry's `size` and `sha256`. One different character in the blob, the
same blob at another path, a missing blob id, a removed file or differing bytes is not exempt.
A blob request that fails, or answers malformed or with bytes that are not that blob, is
reported as "could not check its bytes", apart from "bytes differ" (which names the size or the
sha256), and is not exempt either. The file mode is not checked. A listed blob committed as
executable carries exactly the bytes the owner read; committed as a link (mode 120000) it
points at the path its bytes name up to their first NUL, a path the owner did not read, so a
capture is committed as a regular file. The gate prints the exempt paths and the
manifest's blob id, and anything unlisted still needs the attestation above. An invalid
manifest, a link instead of a file, or a duplicate path means no exemptions, and the gate says
why. Changing the manifest matches `scripts/merge-gate`, so that pull request is the owner's
own merge. There the owner reads each sitting's decoded captures (a UTF-16LE file without a
byte-order mark included, which a plain UTF-8 read shows as noise), their ownership and
licence, their NOTICE obligation and their `PROVENANCE` entry
(`scripts/check-fixture-provenance.mjs`), states `seed_basis`, and confirms on the file they
read both `blob` (`git hash-object --no-filters <file>`) and `sha256` (for example `shasum -a 256`), since
those are the values the gate checks. An entry stays valid for those bytes at that path until
it is removed, so remove it when the capture is deleted or withdrawn. The gate warns (never blocks)
about a stale entry: one whose path master holds with another blob, or once held and has deleted;
an entry whose path master never held is counted as pending. A lab branch is never
read or trusted at gate time: lab branches are unprotected and are pushed with the owner's own
account, so being on one proves nothing about where the bytes came from, and a branch can be
moved or deleted later. A pointer to a lab branch or commit in a pull request body is ignored.

## Signing and publication

- Windows production installers require an organization-controlled code-signing
  certificate and timestamp service.
- macOS production bundles require an organization-controlled Developer ID,
  hardened runtime, notarization, and stapling.
- Signing credentials belong in protected release environments with required
  reviewers; never place them in repository variables, logs, or artifacts.
- Publish SHA-256 checksums and provenance/attestation evidence with every
  downloadable artifact.
  Each preview archive gets a GitHub build attestation (`actions/attest`, in a
  separate job that runs no repository code and is the only one that can mint
  the identity token) that the publish job verifies with `gh attestation
  verify`, requiring this workflow, this commit, the default branch and a
  GitHub-hosted runner, before the release is created; the release notes tell
  readers how to check a download. It records which workflow run and commit built the bytes. It is not
  a code signature, and no client checks it yet.
- Do not create or move a `v*` tag until signed artifacts from both supported
  platforms pass the candidate gates. Release tags must be immutable.

The repository intentionally does not auto-publish unsigned `v*` tag
artifacts. CI bundle jobs produce short-lived smoke evidence only until signing
and notarization ownership is configured. The macOS smoke build sets
`APPLE_SIGNING_IDENTITY=-` for that job so Tauri ad-hoc signs the assembled app
before creating the DMG. The staged app and the app inside the mounted DMG must
both pass `codesign --verify --deep --strict`; a mutation check confirms that
changing a copied app's legal resource fails verification. Ad-hoc signing seals
bundle integrity but supplies no verified publisher identity, Developer ID,
notarization or Gatekeeper approval. These artifacts remain previews without
publisher signing. Production signing defaults and the MCPB publication lane
are unchanged.

## Pull-request type labels

A release proposal reads the type label of each merged pull request. The
`PR labels` workflow therefore fails a pull
request that does not carry exactly one of `type:feature`, `type:bug`,
`type:rectify` or `type:chore`, so a missing label is fixed on the pull request
and does not block a release later. Dependabot pull requests are skipped; their
`dependencies` label is already classified. Whether the check is required to
merge is a repository setting, not something this file decides.

## MCPB previews and the install page

`.github/workflows/release-mcpb-preview.yml` is a manually dispatched,
two-platform preview lane. It produces the actual Windows x64 and macOS arm64
MCPB archives, validates and launches each archive without contacting Tally,
then publishes a durable **GitHub prerelease** only when every archive, checksum,
payload-free smoke result, and source-provenance record is present. Preview tags
must be `mcp-vX.Y.Z` (or the older `mcp-preview-X.Y.Z`, used by 0.2.0 and
0.3.0); they cannot reuse a production `v*` tag. A
build without publisher signing must never be described as signed, notarized,
or ready for production use. The workflow titles the release
`ComplyEaze Bridge MCP <version>` itself, with the version taken from the tag
(`mcp-v0.5.0` gives `0.5.0`), so the title needs no edit after publication.

This workflow intentionally has no production-signed channel. A raw MCPB
archive is not a notarization-and-stapling carrier for the enclosed macOS
binary. Before adding one, maintainers need a separately reviewed signed
distribution design, an organization-controlled Developer ID, notarization
credentials, a timestamped Windows signing certificate, protected release
environments, and host validation of the complete shipped carriers. Self-signed
certificates and OS-warning bypass instructions are not acceptable substitutes.

`site/` is the static website (home, Download, Releases and Changelog). Its
workflow runs when a maintainer
dispatches it, and again when a maintainer-dispatched preview publication in
this repository finishes successfully, so the page's release snapshot follows
the release without a second step. The job requires the publication run's event
and repository, because the trigger matches a workflow by name only. A release
published with the workflow token starts no workflow of its own, which is why
the follow-up is wired to the publication run. A successful publication also
deploys any `site/` change already merged to the default branch but not yet
deployed, so text on the default branch is text that goes live.
Site text is a public claim, so it follows the same rule as release notes:
say only what has been proven on the build a person can download.

- A pull request that changes text under `site/` says so in its body, quoting
  the changed sentence, and the reviewer reads it against the evidence for the
  claim (the platforms tested, the not-yet-code-signed label, what the product
  does not do). A claim the release notes could not make does not belong on the site.
- Each deploy writes "Site text changed since the last deploy" to its run
  summary: the files and the diff under `site/` between the last commit that
  was deployed successfully and the one being deployed. The diff is cut at 300
  lines of 400 characters, and files generated during the deploy (the release
  list and the changelog page) are not compared. Read it after every deploy,
  manual or publication-triggered; a wrong sentence is fixed by a new deploy.
  It only informs. It reads the deployment history with the workflow token, and
  if that read, the comparison or its two-minute limit fails, the step warns
  and the deploy continues, so a missing summary is not a reason to stop a
  release.

Once GitHub Pages is configured for this repository, the page offers a
preview by its tag name and complete asset set, whether or not GitHub marks the
release a prerelease, and labels every download as not yet code-signed. It does
not proxy Tally, create an account, or run a cloud relay.

## Release rhythm and notes

A release is how a CA, an accountant or a developer learns what changed. Write
every note for them first, and for maintainers second.

**Rhythm**

- Cut a release (`mcp-v*`; `mcp-preview-*` for 0.2.0 and 0.3.0) only when both of
  these hold: at least one user-visible change has landed, and CI is green on
  both hosts. The macOS native tests run only on a master push, the daily
  scheduled run, a manual run and the version pull request (the release pull
  request, which changes the version in `src-tauri/tauri.conf.json`), never on
  another pull request or in the merge queue. A master
  push leaves the Windows native, bundle and seam jobs out of its matrix when the merge
  queue ran them on that same commit; its job summary names the queue run. The
  macOS bundle and seam jobs run in the queue and on a push only when a
  packaging path changes, and the daily scheduled run runs everything.
- The workflow publishes each preview as a prerelease. Once its checks are
  read, the maintainer marks the newest preview as the repository's latest
  release (`gh release edit <tag> --prerelease=false --latest`), so the
  releases page never opens on an older line. The install page accepts a
  preview either way.
- Once a month, add a "what changed" entry to `CHANGELOG.md`, even in a month
  without a build.

**Choosing the version**

SemVer 2.0.0 defines major, minor and patch only from 1.0.0. Before that,
"anything MAY change at any time" (rule 4), and its FAQ suggests a minor bump
for each release. Within that freedom, this project's own convention keeps a
patch for a fix-only release:

| What merged since the last release | Before 1.0.0 | From 1.0.0 |
| --- | --- | --- |
| Something an existing user relies on was removed or changed (label `breaking`) | minor (0.3.0 → 0.4.0) | major |
| A new capability (`type:feature`, `enhancement`) | minor | minor |
| Only fixes, rectifications or maintenance (`type:bug`, `type:rectify`, `bug`, `type:chore`, `dependencies`, `github_actions`, `infra`) | patch (0.3.0 → 0.3.1) | patch |
| Documentation only (`documentation`) | no release | no release |

`node scripts/next-version.mjs` proposes the version:
- It reads the pull requests squash-merged since the last `mcp-v*`,
  `mcp-preview-*` or `v*` tag, from `git log`, up to `origin/master` (`--to REF` changes that;
  `HEAD` would count an unmerged working branch's commits as direct pushes).
  It refuses when `origin/master` here is not origin's current master, or when
  the version files here differ from that commit's, so a stale branch cannot
  propose a bump twice. Run `git fetch --tags origin` first.
- It classifies each by its own labels together with the labels of the issues
  it closes, the highest kind winning.
- It refuses, and names them, while any pull request is unclassified. Expect
  this: many merged pull requests carry no classifying label and close no
  labelled issue. Label them, or choose the level yourself with `--level`,
  which the output records and warns about when it is lower than the labels
  imply. `--level` cannot release nothing.
- It refuses an unknown flag, a repeated flag, a stray argument, and the
  `--level=minor` form (write `--level minor`), rather than ignoring them. A `gh` failure or a
  60-second stall ends the run with one line naming the command.
- It refuses when the version files already differ from the last release tag,
  which is the state after a version pull request merges and before its tag
  exists.

`--apply` writes the version to `package.json`, `packaging/mcpb/manifest.json`,
`src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, the `bridge` entry in
`src-tauri/Cargo.lock`, and the README sentence naming the current version.
It checks every file first and writes none if one fails. Then:

1. Add `docs/tally/compatibility/acks/pr-<N>.txt` to the version pull request, because three of those files are pinned (see "Compatibility-surface reseal").
2. Rewrite the draft notes it prints in plain words, in `CHANGELOG.md`, and move the
   unreleased entries into the new section (see "At cut time" under `CHANGELOG.md` below).
3. Commit, and open the version pull request.
4. After it merges, dispatch the preview release with the matching tag.
5. After the release is published, if the release run warned that unreleased entries were
   listed under "Also in this build", put each in the new section of `CHANGELOG.md` and `git rm`
   its file, or the next release lists it again. Then change the tag and date in the two managed
   spans that name the newest published build (`managed:current-release` in
   `README.md` and `managed:latest-preview` in `SECURITY.md`; search both files
   for `managed:`), and check that the install page and the repository
   description name the same build.
   Also update the sentences that say a tool is "in source but not in the
   0.3.0 release" in `docs/agent/README.md` (name the build that now has it),
   and the `README.md` lines that describe the published package, so no line
   describes the previous build as the newest. Add the new tag to the
   "so far" list under `## [Unreleased]` in `CHANGELOG.md`, and the new version
   to the version line of `docs/security-and-privacy.md` (change its tag line
   only when the page is read again at the new tag).
   The publish workflow creates the release as a pre-release and deploys the
   site as soon as it ends, before `gh release edit` marks the release latest, so
   the release snapshot deployed with the site still records it as a
   pre-release. The site workflow has no push trigger: text merged under `site/`
   and in `CHANGELOG.md` goes live only when a maintainer dispatches it or at
   the next publication, so dispatch it once after the post-release pull request
   merges.

`scripts/check-license-metadata.mjs` fails CI when the five version files
disagree.

**Every release note has four parts, in this order**

1. **What you can do now.** Plain sentences a CA would say, one per change.
   Each line names its pull requests.
2. **Safer or fixed.** Refusals, safety checks and bug fixes, described by
   what the user sees.
3. **Known limits.** Name what still does not work, and link the issue.
4. **All changes.** GitHub's generated list, grouped by the existing `type:*`
   labels through `.github/release.yml`. The publish workflow appends it; do
   not paste it by hand.

   On the release page the install-and-check header
   (`packaging/mcpb/INSTALL_AND_CHECK_RELEASE.md`) comes first, then the written
   parts, then the standard release-notes footer, then this generated list.
   If the notes cannot be composed at all, the release carries the footer and
   the generated list without the header.

**`CHANGELOG.md`**

- Each release gets an "In plain words" section above the detailed entries,
  written from the merged pull requests since the last build. It opens with a
  short "Should I upgrade?" block: who should, in one line why (name a published
  advisory if the build fixes one), how to upgrade, and what to check afterwards
  (the version the extension shows, and the posting setting).
  It adds no claim the detailed entries or the published advisory do not support.
- While the changes are unreleased, head the first part "What the next build
  adds", so nothing unpublished reads as available. Rename it "What you can do
  now" when the section becomes the release.
- Keep the detailed entries as they are. They serve maintainers and
  integrators.
- An entry for a change that is not yet in a published build is a file, not lines
  of `CHANGELOG.md`, so two pull requests never edit the same lines. It is
  `docs/changelog/unreleased/<added|changed|removed>/<N>-<words-with-hyphens>.md`: the
  folder names the heading it is listed under, `<N>` is the issue or pull request
  its text cites, and the words tell two entries on one issue apart. A file holds
  one bullet: the first line starts with `- `, every other line is indented two
  spaces, there is no blank line, heading, rule or code fence, and the text cites `#<N>`.
  The words are up to six lowercase words of letters and digits joined by hyphens, and
  must not contain privacy, security, redact or egress, which the review tiers send to the
  full tier whatever the folder (see `scripts/review-tier-paths.json`);
  `scripts/release-notes.test.mjs` fails on a file that is malformed, misplaced (anything
  outside the three folders is reported) or not in the one review tier. The install page joins the files into the groups of
  the `[Unreleased]` text, and a malformed file is left out with a warning, so a bad entry
  cannot stop a release or a deploy. The release notes list them when the tag has no
  section of its own and, with a warning, when it has one (see "At cut time"). Entries already in `CHANGELOG.md` stay where they are
  until a release moves them.
- `CHANGELOG.md` is the one source for the notes. The publish workflow takes
  the `## [X.Y.Z]` section that matches the tag (for `mcp-vX.Y.Z` or `mcp-preview-X.Y.Z`),
  puts it above the release-notes footer, and appends GitHub's list. The
  install page's "What changed" page renders the same file when the install
  page is deployed, so a changelog edit reaches the site with the next
  deploy, not when the file changes.
  Neither step can stop a release or a deploy; each warns and falls back, so a
  missing section shows as a warning in the run, not as a failed release.
- The page's look belongs to a tracked template, `site/changelog.template.html`,
  which holds the line `<!-- changelog -->` exactly once. The generator puts a
  version index and the sections there and writes `site/changelog.html`, which
  is ignored by git and never edited by hand. Each section has an `id` (`v0-3-0`,
  or `unreleased`), a `data-version`, and `data-latest="true"` on the newest
  published one. With no template, or one without the marker, the deploy uses
  a plain built-in page and never fails.
- At cut time, in the pull request that bumps the version: rename the
  plain-words block under `## [Unreleased]` to `## [X.Y.Z] - YYYY-MM-DD`, change
  its first part's heading from "What the next build adds" to "What you can do
  now" and its "in source since" title to name the new build, and leave a fresh
  `## [Unreleased]` above it. In the same pull request, run
  `node scripts/release-notes.mjs unreleased --changelog CHANGELOG.md` (it stops with a
  message on a malformed, misplaced or unreadable file; it prints one group per kind that has
  entries), rewrite what it prints in plain words into the new section's parts, as for the
  other draft notes (a released section has no **Added** or **Changed** groups), and `git rm`
  the files under `docs/changelog/unreleased/`. Run the command again after the last merge of master
  into that pull request, because the build is cut from the head of master. An entry merged
  after that, before the dispatch, is in the build: the release notes then list it too, under "Also in this
  build", and warn in the run; step 5 of the release steps above moves it into the section. If the section is missing, the
  release carries the `[Unreleased]` text instead (with a warning), which may
  describe changes that build does not have, so read the release body; if that
  is empty too, only the standard text and GitHub's list. Headings must read
  `## [X.Y.Z] - date` (a dash or en dash also works); a fence that is never
  closed, or another level-two heading, is reported as a warning in the run.
  In the same pull request, reread every answer on the Questions page (`site/faq.html`)
  and every tool row on the capabilities page (`site/capabilities.html`) against the new
  release and move each page's check stamp (the release and the day; on the Questions page
  also in the structured data), and the release named by "release X offers" or "installs" in the
  capabilities page and in `site/llms.txt`. The tools the catalog serves that no published build installs
  are listed in `scripts/site-pages.test.mjs` under the release the page is stamped for;
  once that release publishes them they belong on the page. `scripts/site-pages.test.mjs`
  fails until each stamp names the version in `packaging/mcpb/manifest.json` and the page
  names the tools the catalog serves, so a version bump cannot merge with answers nobody
  has reread.
- The install page shows this file's headings, paragraphs and bullets, so a
  claim added here is a claim on the public site. Fenced code is left out, and
  tables and quotes show as plain text. Escape nothing by hand: the renderer
  escapes all text and links only `https://` addresses and `#123` issue
  numbers.

**Writing rules for notes, the install page and posts**

- Say "ComplyEaze Bridge" at first mention.
- Name the build that each capability sentence describes. The install page,
  the README and the repository description must agree with the newest
  published build.
- Make no accuracy claim without a published method and result.
- Never say "signed", "notarized" or "ready for production use" about a build
  without publisher signing (see *Signing and publication*).
- User-facing text does not call a release a "preview" or say it is "for
  evaluation only". It says what is true: still being developed, may contain
  errors, try it on test data with backups, not yet code-signed, and what has
  and has not been tried. Internal names (the `mcp-preview-*` tag, the workflow
  and file names) are not user-facing text.
- Put no customer, company or client names, no local paths, and no
  private-repository references in any note.
- Before publishing, check the text for AI-writing patterns and unclear
  phrasing. Offline prose linters such as `write-good` help. Keep an exact
  safety or capability claim even where a linter flags it.

**When a listing exists**

- If the MCP registry or another directory lists the build, update that
  listing's version and SHA-256 in the same release. A listing that points at
  an older package is a stale claim.
- The official MCP registry listing is `io.github.ComplyEaze/bridge-tally`, published
  from `registry/server.json` by the **Publish to the MCP registry** workflow
  (`.github/workflows/publish-mcp-registry.yml`), dispatched by hand and never by a tag.
  Create its `mcp-registry-publish` environment first. As set up on 2 October 2026 it
  allows the default branch only and names one required reviewer, the owner, with
  prevention of self-review off and administrator bypass off: a run waits for the
  owner's own confirmation click, and there is no second person. For each release, after
  it is published, update `registry/server.json` in the post-release pull request: the
  version, the two asset addresses (they carry the version) and both `fileSha256`
  values, copied from the release's `.sha256` files. Merge that pull request, and
  dispatch the workflow from the default branch with the release tag, before any later
  version pull request changes the manifest. The first job refuses to let the publishing
  job run unless the file's version equals `packaging/mcpb/manifest.json`'s, the tag is
  `mcp-v<version>`, the release is final, and each hash equals both the digest GitHub
  records for the asset and its `.sha256` file. The registry accepts a token from any
  job of any ComplyEaze repository that holds `id-token: write` and checks nothing else,
  so the namespace is not protected by this workflow alone.

  A published registry version's configuration cannot be edited or moved. A wrong
  version can be withdrawn: the publish permission also lets a token mark a version
  deprecated or deleted through the registry's status endpoint (`mcp-publisher status`),
  which hides it without removing its metadata, and the next release's publication
  supersedes it. Withdrawal needs its own `id-token` run, so it is an owner-only step
  and is deliberately not part of the workflow. The test `scripts/check-registry-server-
  json.test.mjs` (run by `pnpm test` in every pull request) checks the file's form and
  that its version never runs ahead of the manifest; it cannot require equality in a
  pull request, because the version pull request bumps the manifest before the release
  (and so the hashes) exists.

## Rollback

1. Mark the affected GitHub release as withdrawn and remove unsafe downloadable
   artifacts without moving or reusing its tag. Remove every unsafe artifact: a
   bundle left on the release stays downloadable from GitHub. Removing one
   platform's bundle and its `.sha256` is enough for the live list to stop
   offering the release on the install page (step 2 covers the snapshot),
   because the page offers a release only while both platforms have a bundle
   and a checksum. Delete the release or make it a draft only when the entry
   itself must leave the list. Renaming it or changing its pre-release flag does
   not change whether the page offers it: the page tells builds apart by tag
   and assets.
2. Redeploy the install page and read it back. The page is static and joins
   two lists (`site/release-catalog.mjs`): the live GitHub list, and the
   snapshot written by the last deploy (`site/releases.json`), with the live
   entry winning for each tag. Until the redeploy, two things follow.
   - A tag that is only in the snapshot stays. A deleted or drafted release
     leaves the live list, so the Download button can still offer it, and
     Releases shows it as Latest, with links to artifacts that no longer exist,
     when it is the newest installable tag.
   - A visitor whose browser gets no answer from GitHub (for example after its
     unauthenticated rate limit) sees the snapshot only, with the page's notice,
     so even a kept release whose artifacts were removed is still offered from
     the old snapshot.

   A new deploy writes a snapshot without the release, or without a complete
   asset set. Dispatch "Deploy Bridge install page" from master (a run on another
   ref skips both jobs without saying so) with the owner's yes. A
   deploy also publishes any `site/` change already merged and renders the
   changelog and legal pages again, so that text goes live too; read the run's
   "Site text changed since the last deploy" summary afterwards, as under "MCPB
   previews and the install page" (it does not compare the generated pages).
   Then fetch `releases.json` from the deployed site, with a query string so a
   cache does not answer, and check that the withdrawn tag is absent or present
   with no complete asset set; open Download and Releases once to see the
   merged list. If the withdrawn release was the only installable one, the
   deploy stops at its check for at least one installable release and the old
   page stays: cut the replacement first (step 5). Its publication redeploys the
   page when its run succeeds; if it fails, dispatch the deploy by hand. Or say
   plainly that the page cannot be updated.
3. Publish a security advisory when coordinated disclosure is required.
4. Revert or rectify the source change through a pull request with migration
   compatibility notes.
5. Cut a new patch version; never replace an already published artifact under
   the same version or checksum.
6. Preserve release notes explaining impact, upgrade/rollback steps, and the
   last known-good version without including customer data.
