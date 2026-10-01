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

**Enforcement today.** The GitHub check is report-only: it prints
`WOULD FAIL: <reason>` and exits 0, so it does not block a merge. The blocking
leg is `scripts/merge-gate.sh`, a local tool run by whoever merges (the
orchestrator), not CI. The one exception is a push to master, which is always
enforced (below). Making the pull request check enforcing is a later change to two
pinned files: drop `--report-only` from the step in
`.github/workflows/ci.yml` and change the exact step text that
`scripts/check-ci-workflow-consistency.mjs` requires, both pinned, with their own
acknowledgement.

What this trades away, stated plainly. Under schema 2 the required
`Tally portable core` job failed a pull request, and again the master push, when a
pinned file's bytes differed from its stored hash. Nothing stored remains to
compare, so that job can no longer fail on a changed pinned file. On a pull request,
while the check is report-only, a change merged without `scripts/merge-gate.sh`
reaches master with every check green. What restores the after-the-fact tripwire: a
push to master is checked like a pull request and is never report-only. Every
first-parent commit the push landed (`before..HEAD`, `before` from the event payload;
each one a squashed pull request, attributed by the `(#N)` in its subject) must carry
exactly the acknowledgement its pinned changes need, and the master run goes red if
any does not. A push whose `before` is missing, new or not an ancestor of HEAD (a
force-push) cannot be verified and fails. This covers the base race: `merge-gate.sh` reads
the pin list when it runs but the merge binds only the head, so another pull request
that pins a file after this one was gated lets an unacknowledged change land; the
master run then fails instead of nobody noticing. It does not stop the merge, and a
red master needs an acknowledgement-only follow-up pull request to clear.

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
- The surface digest that receipts and attestations bind is **computed by the
  gate from the live bytes of every pinned file**, never stored. Change one byte
  of a pinned file and the digest moves, so evidence bound to the old digest goes
  stale. The matrix carries no copy of the digest (bridge#760).
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
- The branch name is not the file name, so a lane branch is fine.

The `reviewer:` login is procedural assurance, not authentication: the file is
written by hand and proves nothing by itself. What it records is that a named
person read the pinned changes and stands behind them. Say which files you
looked at in the review.

**What checks it**

- CI runs `scripts/check-surface-ack.mjs` in the `workflow-consistency` job. It
  reports a missing, wrong, edited or unneeded acknowledgement, but is
  report-only today (see above). Its modes:
  - `pull_request`: diffs the merge commit against its first parent (`HEAD^1`),
    after asserting that `HEAD^2` equals the pull request's head commit.
  - `merge_group` (once a merge queue exists): checks each first-parent commit in
    `base_sha..head_sha` against its own acknowledgement, and fails closed when a
    commit cannot be attributed to one pull request.
  - `push` (master): every first-parent commit in `before..HEAD` is checked like a
    pull request, each attributed by the `(#N)` in its subject, and a failure is never
    report-only; a missing, all-zero or non-ancestor `before` fails closed. It also
    validates that every file in the acknowledgements directory is well formed.
  - `workflow_dispatch`: only the acknowledgements directory is validated.
  The checker runs from the pull request's own tree, so a pull request could weaken
  it; that is why the script and `ci.yml` are pinned, which makes the change
  visible and acknowledged. A change that makes the checker skip itself shows no
  `WOULD FAIL` in its own pull request run, so the pin alone does not make it red
  in CI; `scripts/merge-gate.sh`, which is also pinned (with
  `scripts/check-ci-workflow-consistency.mjs`, which enforces the step's shape), is
  what sees it. Run the gate from a checkout of the base branch, not from the pull
  request's own tree, or a weakened copy gates itself.
- `scripts/merge-gate.sh` (run locally by whoever merges) reads the pin list at
  the head and requires the same `pr-<N>.txt`. It measures removed and added pins
  against the pull request's merge base (not the base tip), and reads the base tip
  only to know which files are pinned there, so a branch that is behind a
  master commit that added a pin is not reported as indeterminate. It also
  requires a review or comment that names the head commit, the acknowledgement
  path and every touched path. The `reviewer:` login must be that review's author.
- The `Tally portable core` job still runs the `gate` command, which checks
  required-file coverage and that every pinned file exists, and computes the
  digest from the files. It no longer compares a stored hash, so a changed
  pinned file does not fail it by itself.

The digest is the same one schema 2 produced for the same bytes, so an
attestation or receipt made against the old surface stays valid exactly as long
as no pinned byte changes and the pin list is unchanged (adding or removing a pin
moves the digest too). Adding four pins (`.gitattributes`, the acknowledgement checker,
`scripts/merge-gate.sh` and `scripts/check-ci-workflow-consistency.mjs`) in the
cut-over itself moved the digest once. Print the current digest with
`cargo run -p bridge-tally-compatibility -- surface-digest docs/tally/compatibility/compatibility-surface.json .`
(the `surface-digest` subcommand resolves the surface the same way the gate does).

Two byte changes have no pinned path in a diff, and are closed differently. A
`.gitattributes` below the repository root (end-of-line, `ident` or
`working-tree-encoding` rules) changes the bytes a pinned file is hashed as, so the
acknowledgement checks refuse any nested `.gitattributes` outright. A pinned path
that is, or sits under, a symbolic link would make the digest follow an unpinned
target, so `resolve` refuses it (`surface_file_symlink`).

The live-read collectors (`bridge-tally-live-read` and its native outstandings
qualification) refuse to start unless every pinned file on disk is byte-identical
to the blob committed at `HEAD` (`git hash-object --no-filters` against
`git ls-tree`, so `assume-unchanged`, `skip-worktree`, clean filters, line-ending
rules and a stale stat cache cannot hide an edit; an untracked or ignored pin, a
symlink or a tree that is not the repository at the root refuses; git runs
without `GIT_DIR` and the other redirecting variables). The reference is `HEAD`,
not reviewed master: this refuses uncommitted edits only. A committed but not yet
reviewed edit to a pinned request builder runs against live Tally, and is caught
at `scripts/merge-gate.sh`, where the acknowledgement and the review name the file.
Schema 2's reference was the stored hash, which a reseal in the same pull request
moved too, so that case was not caught before either. A pull request stacked on another carries
its own acknowledgement, so when the child merges into the parent branch and the
parent then goes to master, it holds two. Fold them into the parent's `pr-<N>.txt`
(the union of the two path lists) before the parent merges; both checkers require
exactly one new acknowledgement.

#### Adding or removing a pin

Edit the sorted list by hand.

1. Insert the entry in sorted path order with a `reason` (required for a pin added
   by a pull request, at most 500 characters): why this file decides what Bridge
   posts or lets leave the machine. Paths are relative, unique and sorted.
2. Leave `MAX_SURFACE_FILES` in `tools/bridge-tally-compatibility/src/lib.rs`
   alone. It is a fixed parse bound against a runaway list (1024), not a count to
   maintain, so adding or removing a pin does not touch it and two pull requests
   that each add a pin no longer collide on it. Only a list that approaches the
   bound would justify raising it, which is a deliberate change to a pinned file
   with its own reason and acknowledgement.
3. Add the acknowledgement, listing the new pin's path (and a `removed-pin:` line for
   each removal). Then run the tool's tests.

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
the machine, and leave the rest; see the comment on `MAX_SURFACE_FILES` for the
rule and bridge#416 for the reasoning.

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
`REQUIRED_SURFACE_FILES` and `REQUIRED_SURFACE_DIRECTORIES` are checked for
presence, so an entry in either cannot be dropped silently at all.
`gate_rejects_each_omitted_required_lifecycle_path` iterates that list, so adding
a path also tests it. A procedure a maintainer must follow is weaker than a
constant they cannot circumvent. The matrix is worse: a dropped claim leaves no
trace at all, so reconcile its claims the same way.

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
3. Leave `MAX_SURFACE_FILES` alone: it is a fixed bound, not a count.
4. Run the gate, and check the pin count against the union you computed; the gate
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
   `MAX_SURFACE_FILES`. A branch that only changed the
   contents of already-pinned files has nothing to re-add.
4. Add `docs/tally/compatibility/acks/pr-<N>.txt` as above if the branch changes any
   pinned path. Until the branch has merged master, its head still holds a schema 2
   file, which the acknowledgement check and `scripts/merge-gate.sh` refuse to read
   at the head (the merge gate reports the branch as indeterminate).
5. Delete the leftover local merge-driver configuration, once per clone (harmless
   if absent): `git config --remove-section merge.bridge-compat-reseal`.

`scripts/reseal.sh`, `rehash-surface` and the merge driver no longer exist. Do not
recreate them.

#### Running the compatibility tool

The tool's tests and the `gate` command need the pinned toolchain. A Homebrew
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
   Keep repository-synthetic parser qualification receipts separate from the
   live Tally compatibility matrix: they cannot establish a product release,
   Education/licensed mode, HTTP/runtime behavior, or a performance budget.
   Do not broaden a Tally support claim without current exact-profile live
   evidence; state missing matrix evidence in the release notes.
   Run the executable Tally claim gate:

   ```sh
   cd tools
   cargo run --locked -p bridge-tally-compatibility -- gate \
     ../docs/tally/compatibility/compatibility-matrix.json \
     ../docs/tally/compatibility/compatibility-surface.json \
     ../docs/tally/compatibility/trusted-evidence-keys.json \
     ../docs/tally/compatibility/evidence ..
   ```

   Any positive cell without an exact fresh signed receipt is a release
   blocker. Unknown cells remain visible limitations; they are not failures
   unless release notes or product copy claim that scope.
   Live Education collection, when legitimately available, must follow the
   [read-only runbook](./tally/compatibility/live-education-runbook.md). Never
   upload `.bridge-live/` automatically or substitute the parser-only CI
   receipt for reviewed live evidence.
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

**Binary files.** Git shows a file it reads as binary (a NUL byte, so every UTF-16
fixture, images, archives) as `Binary files differ` or a `GIT binary patch`, with no
text to scan. The gate cannot read those bytes, so it holds the PR until a human
attests: the merger runs
`scripts/merge-gate.sh <PR> --binary-review-sha <head> --independent-review-sha <head>`
with the full 40-hex head SHA. The two flags are an operator's statement, never read
from the PR body or from a comment, and each must equal the head the gate is running
on, so a later push invalidates them.
- `--binary-review-sha`: the merger states that they read every binary addition or
  change (the bytes, not the file name), its ownership and licence, and its NOTICE
  obligation. For a captured fixture this means its `PROVENANCE` entry matches the
  bytes (`scripts/check-fixture-provenance.mjs`) and the capture is synthetic.
- `--independent-review-sha`: a second person, who is not the author, states the same
  reading of the same head.
A review or comment by a reviewer (V4 included) is evidence for the merger, not the
attestation: the gate does not look at comments for these flags. The gate lists the
first eight binary paths in its message so the merger knows what to read. Letting a
named review comment carry the attestation, as the acknowledgement review does, would
be a separate change to the gate.

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
or ready for production use.

This workflow intentionally has no production-signed channel. A raw MCPB
archive is not a notarization-and-stapling carrier for the enclosed macOS
binary. Before adding one, maintainers need a separately reviewed signed
distribution design, an organization-controlled Developer ID, notarization
credentials, a timestamped Windows signing certificate, protected release
environments, and host validation of the complete shipped carriers. Self-signed
certificates and OS-warning bypass instructions are not acceptable substitutes.

`site/` is a small static installer page. Its workflow runs when a maintainer
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

- Cut a release (`mcp-v*`; `mcp-preview-*` for 0.2.0 and 0.3.0) at most every two
  weeks, and only when both of
  these hold: at least one user-visible change has landed, and CI is green on
  both hosts.
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
2. Rewrite the draft notes it prints in plain words, in `CHANGELOG.md`.
3. Commit, and open the version pull request.
4. After it merges, dispatch the preview release with the matching tag.
5. After the release is published, change the tag and date in the two managed
   spans that name the newest published build (`managed:current-release` in
   `README.md` and `managed:latest-preview` in `SECURITY.md`; search both files
   for `managed:`), and check that the install page and the repository
   description name the same build.

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

   On the release page the written parts come first, then the standard
   release-notes footer, then this generated list.

**`CHANGELOG.md`**

- Each release gets an "In plain words" section above the detailed entries,
  written from the merged pull requests since the last build.
- While the changes are unreleased, head the first part "What the next build
  adds", so nothing unpublished reads as available. Rename it "What you can do
  now" when the section becomes the release.
- Keep the detailed entries as they are. They serve maintainers and
  integrators.
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
  `## [Unreleased]` above it. If the section is missing, the
  release carries the `[Unreleased]` text instead (with a warning), which may
  describe changes that build does not have, so read the release body; if that
  is empty too, only the standard text and GitHub's list. Headings must read
  `## [X.Y.Z] - date` (a dash or en dash also works); a fence that is never
  closed, or another level-two heading, is reported as a warning in the run.
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

## Rollback

1. Mark the affected GitHub release as withdrawn and remove unsafe downloadable
   artifacts without moving or reusing its tag.
2. Publish a security advisory when coordinated disclosure is required.
3. Revert or rectify the source change through a pull request with migration
   compatibility notes.
4. Cut a new patch version; never replace an already published artifact under
   the same version or checksum.
5. Preserve release notes explaining impact, upgrade/rollback steps, and the
   last known-good version without including customer data.

<!-- Merge-queue canary, 30 Sep 2026: this comment tests the queue and can be removed. -->
