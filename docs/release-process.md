# Release process

Bridge supports development and unsigned bundle smoke validation on Windows
and macOS. A smoke bundle is not a production release.

## Supported build baseline

- Source release line: `0.2.x` under Apache-2.0
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
orchestrator), not CI. Making the CI check enforcing is a later one-line change to
`ci.yml`, which is itself a pinned file and needs its own acknowledgement.

What this trades away, stated plainly. Under schema 2 the required
`Tally portable core` job failed a pull request, and again the master push, when a
pinned file's bytes differed from its stored hash. Nothing stored remains to
compare, so no CI run can now go red on an unacknowledged pinned change after the
fact: a pull request merged without `scripts/merge-gate.sh` reaches master with
every check green, and while the check is report-only that holds even on the pull
request. When enforcement is switched on, the pull request check will fail, but the
master push run will still only check that acknowledgement files are well formed.
Detecting a bypass after merge means comparing a merged pinned change with its
acknowledgement file by hand.

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
  - `push` and `workflow_dispatch`: the diff is already merged, so it only
    validates that every file in the acknowledgements directory is well formed.
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
moves the digest too). Adding the two gate scripts as pins, in the cut-over itself,
moved the digest once. Print the current digest with
`cargo run -p bridge-tally-compatibility -- surface-digest docs/tally/compatibility/compatibility-surface.json .`
(the `surface-digest` subcommand resolves the surface the same way the gate does).

Two byte changes have no pinned path in a diff, and are closed differently. A
`.gitattributes` below the repository root (end-of-line, `ident` or
`working-tree-encoding` rules) changes the bytes a pinned file is hashed as, so the
acknowledgement checks refuse any nested `.gitattributes` outright. A pinned path
that is, or sits under, a symbolic link would make the digest follow an unpinned
target, so `resolve` refuses it (`surface_file_symlink`). Known, unclosed: a file
marked `assume-unchanged` or `skip-worktree` reads as clean to the live-read drift
refusal; the gate still hashes its bytes. A pull request stacked on another carries
its own acknowledgement, so when the child merges into the parent branch and the
parent then goes to master, it holds two. Fold them into the parent's `pr-<N>.txt`
(the union of the two path lists) before the parent merges; both checkers require
exactly one new acknowledgement.

#### Adding or removing a pin

Edit the sorted list by hand.

1. Insert the entry in sorted path order with a `reason` (required for a pin added
   by a pull request, at most 500 characters): why this file decides what Bridge
   posts or lets leave the machine. Paths are relative, unique and sorted.
2. Raise `MAX_SURFACE_FILES` in `tools/bridge-tally-compatibility/src/lib.rs` to the
   new pin count. The convention is to pin exactly the count in use, and
   `RESERVED_SURFACE_FILES` bounds how far the cap may exceed it. Raising it is an
   explicit compatibility-surface decision: record the reason in the commit and
   the pull request. That file is pinned, so this edit is itself covered by the
   acknowledgement.
3. Add the acknowledgement, listing the new pin's path (and a `removed-pin:` line for
   each removal). Then run the tool's tests: a cap change can invalidate a test that
   hard-codes the old bound.

Removing a pin is done the same way, with a `removed-pin:` line and the cap kept
in step. A malformed list is refused by the tool and by CI.

**`MAX_SURFACE_FILES` is the line most likely to be silently wrong, and it is
worse when it does NOT conflict.** If two branches start from the same cap and
each add one pin, both change it from N to N+1: an **identical edit**, which git
merges without a conflict. The merged surface then holds N+2 pins against a cap
of N+1, and the gate fails with `surface_file_count_invalid`. That failure is
loud, so it is not dangerous; what is misleading is expecting a conflict to
prompt you. Recompute the cap from the merged pin count whether or not git
stopped to ask.

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
3. **Recompute `MAX_SURFACE_FILES` from the reconciled pin count.** Do not carry a
   number derived from either side's cap.
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
   `{ "path": "...", "reason": "..." }` in sorted order, and keep
   `MAX_SURFACE_FILES` equal to the merged pin count. A branch that only changed the
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
must start with `mcp-preview-`; they cannot reuse a production `v*` tag. A
preview is unsigned and must never be described as signed, notarized, or ready
for production use.

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
  claim (the platforms tested, the unsigned-preview label, what the product does
  not do). A claim the release notes could not make does not belong on the site.
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
release a prerelease, and labels every download as an unsigned preview. It does
not proxy Tally, create an account, or run a cloud relay.

## Release rhythm and notes

A release is how a CA, an accountant or a developer learns what changed. Write
every note for them first, and for maintainers second.

**Rhythm**

- Cut an `mcp-preview-*` build at most every two weeks, and only when both of
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
- It reads the pull requests squash-merged since the last `mcp-preview-*` or
  `v*` tag, from `git log`, up to `origin/master` (`--to REF` changes that;
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
   unsigned-preview text, then this generated list.

**`CHANGELOG.md`**

- Each release gets an "In plain words" section above the detailed entries,
  written from the merged pull requests since the last build.
- Keep the detailed entries as they are. They serve maintainers and
  integrators.
- `CHANGELOG.md` is the one source for the notes. The publish workflow takes
  the `## [X.Y.Z]` section that matches the tag (for `mcp-preview-X.Y.Z`),
  puts it above the unsigned-preview text, and appends GitHub's list. The
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
  plain-words block under `## [Unreleased]` to `## [X.Y.Z] - YYYY-MM-DD`, and
  leave a fresh `## [Unreleased]` above it. If the section is missing, the
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
- Never say "signed" or "production" about an unsigned preview (see *Signing
  and publication*).
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
