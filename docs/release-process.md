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

Any change to a pinned file requires a deliberate compatibility-surface reseal
before the claim gate can pass. That includes `package.json`,
`src-tauri/Cargo.toml`, `src-tauri/Cargo.lock` and either workflow — and it is
not only dependency updates. **The canonical `docs/tally/TALLY_PROTOCOL_REFERENCE.md`
index and each part it declares must be pinned sources, so a documentation-only edit to either
stales its digest and fails the gate. Adding or removing a declared part also changes the pin
list; see "Adding or removing a pin" below.** Nothing in a docs diff suggests a
compatibility gate is involved, and PRs have failed CI for exactly this.

Run this from `tools`, **with the pinned toolchain**. A Homebrew
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

`--output` asks the
compatibility tool to stage and replace the destination itself, and it is
required — without it the command prints to stdout and changes nothing on
disk, which looks like success:

```bash
cargo run --locked -p bridge-tally-compatibility -- rehash-surface \
  ../docs/tally/compatibility/compatibility-surface.json .. \
  --output ../docs/tally/compatibility/compatibility-surface.json
```

```powershell
cargo run --locked -p bridge-tally-compatibility -- rehash-surface `
  ../docs/tally/compatibility/compatibility-surface.json .. `
  --output ../docs/tally/compatibility/compatibility-surface.json
```

`rehash-surface` reads the raw bytes of every existing pin, rewrites its
`sha256`, and reports the changed-entry count; it neither adds nor removes
pins. That is the whole reseal. The surface (schema 2) stores only the per-file
hashes: the surface digest that receipts and attestations bind is computed by
the gate from those hashes, never stored, and the matrix carries no copy of it
(bridge#760). So a reseal never touches the matrix, and two changes that pin
different files merge without conflict. CI intentionally checks the resulting
surface but never reseals it. Without `--output`, the command retains its
stdout contract. With `--output`,
the tool writes raw UTF-8 without a BOM to a temporary file in the destination
directory and replaces the destination only after successful serialization.
This keeps failure fail-closed without relying on shell redirection or move
semantics.
On Unix, replacement preserves an existing destination's mode; a new
destination uses the normal `0666` mode subject to the process umask. Windows
uses its normal ACL semantics rather than POSIX mode bits.

#### Adding or removing a pin

Edit the file list by hand: add the entry in sorted path order with any
64-hex placeholder `sha256`, or delete it. Then reseal as above.
`rehash-surface` validates the list's shape (sorted, unique, relative paths,
64-hex hashes, within `MAX_SURFACE_FILES`) and writes every entry's real hash,
the new one included. A malformed list is refused and left unchanged.
`scripts/reseal.sh --pins-changed` still works and is now the same as an
ordinary reseal.

#### What the reseal reports after it succeeds

A successful `scripts/reseal.sh` (not `--verify`) prints a surface coverage
report from `scripts/surface_coverage_report.py`. It never fails the reseal.
Against the merge-base with `origin/master` (override with
`SURFACE_REPORT_BASE`) it lists two things the gate cannot see: a pin that was
dropped, and a module declared directly by a pinned module and newly left
unpinned. Modules left unpinned before the branch are not reprinted, test-only
modules are only counted, and feature-gated ones are labelled.

**A clean report is not evidence that nothing left the seal.** It does not see
code moved between files that already existed; a new module declared by an
*unpinned* module, even one carved out of a pinned file (a new file under an
unpinned `db/mod.rs`, say); deeper descendants of a pinned module; a pinned
file that stops being compiled; a test-only or feature-gated module becoming
production; or a new crate root. The script's docstring keeps the full list.
The merge driver (`scripts/reseal-merge-driver.mjs`) writes the reconciled lists
itself and does not print the report; run `scripts/reseal.sh` after resolving.

Read it, then pin each listed file that decides what Bridge posts or lets leave
the machine, and leave the rest; see the comment on `MAX_SURFACE_FILES` for the
rule and bridge#416 for the reasoning.

#### When the surface itself conflicts in a merge or rebase

Every pin's `sha256` is generated: never hand-merge or hand-edit a hash. The pin
*list* and the matrix's *claims* are authored, and a merge must reconcile them
(below).

Be precise about what the gate does and does not protect, because the two halves
behave oppositely.

**Stale bytes cannot slip through.** `validate_files` re-reads the raw bytes of
every pinned file present and compares the SHA-256, so a surface pinning stale
content fails with `surface_file_changed`. A reseal re-reads the bytes, so the
only way to green is a hash that matches the merged file. For hashes the gate is byte-exact and
fail-closed, so hand-merging them is futile rather than unsafe: every wrong
resolution is loud, and regenerating is the only route to green.

**A dropped entry slips through silently.** The gate can only check pins that are
still in the list, and claims that are still in the matrix. Lose one in the
resolution and the gate passes. That asymmetry is the whole hazard, and it is why
the authored half below must be merged rather than regenerated.

**When does this section apply at all?** With schema 2, two changes that touch
different pinned files, even adjacent entries, merge without any conflict, on
GitHub as well as locally, because each changes only its own `sha256` line. A
conflict here now means both sides changed the same pinned file, or both
inserted pins at the same sorted position, or one appended a pin after the last
entry while the other edited the last pinned file, or one renamed a pin while
the other edited that file, or both edited the claims. One case
merges cleanly and is still caught: one side adds a pin for a file (its hash
from that side's bytes) while the other edits that file. The merged hash line
and the merged bytes then disagree, and `validate_files` fails with
`surface_file_changed` on the merge result. That rests on CI running on the
merge result, which `strict: true` branch protection (or a merge queue)
ensures.

**What does matter is the order.** Resolve every genuine *source* conflict first,
and only then regenerate. `tools/bridge-tally-compatibility/src/lib.rs` is itself a
pinned file: the tool pins its own source into the surface it produces. Regenerate
before that file is final and you pin a half-merged copy -- the gate will catch it,
but only after you have spent the cycle.

**These two files are not wholly generated, and that is what makes the conflict
dangerous.** Each carries two kinds of content:

- **derived** -- every pinned file's `sha256`. Regenerating rewrites these, so
  conflicts in them are noise. (Schema 1 also stored the aggregate digest in both
  files; schema 2 stores none, bridge#760.)
- **authored** -- the surface's *pin list*, and the matrix's *claims and promotion
  constraints*. **Nothing regenerates these.** `rehash-surface` re-reads the bytes
  of every entry that is present; it cannot restore an entry that is absent.

So "take one side wholesale" is safe for the derived half and **silently lossy for
the authored half**, and the gate will not catch it. Measured: delete one
judgment-pinned entry from the surface, reseal, and the gate returns
`compatibility_gate_passed`.

**Where a pin matters enough that this is unacceptable, make it REQUIRED rather than
relying on this procedure.** `REQUIRED_SURFACE_FILES` and `REQUIRED_SURFACE_DIRECTORIES`
are checked for presence, not merely hashed, so an entry in either cannot be dropped
silently at all. The same deletion that returned `compatibility_gate_passed` as a
judgment pin returns `surface_required_directory_file_unpinned` once the path is
required. `gate_rejects_each_omitted_required_lifecycle_path` iterates that list, so
adding a path is also what tests it. A procedure a maintainer must follow is weaker
than a constant they cannot circumvent; this section exists for the pins that are not
worth promoting, not as a substitute for promoting the ones that are. `validate_files` enforces the required
directories and `REQUIRED_SURFACE_FILES`; a judgment pin is in neither, so its
absence is invisible. The matrix is worse: a dropped claim leaves no trace at all.

1. Resolve every non-generated conflict and settle those files completely.
2. Take **one side wholesale** for `compatibility-surface.json` and
   `compatibility-matrix.json` -- but only as a starting point for the derived half.
3. **Reconcile the authored half by hand, against the merge base.** This is the one
   part of these files that must be *merged* rather than regenerated. List the pins
   each side added and confirm the union is present:

   **Name the two sides explicitly — during a rebase `HEAD` is not your branch.**
   When a rebase stops on a conflict, `HEAD` is the upstream plus whatever has
   already been replayed, and the commit being applied is `REBASE_HEAD`. Comparing
   `HEAD` against `origin/master` there compares upstream with itself and never
   reads the feature-side manifest at all — so it misses exactly the pin it is
   meant to preserve. The conflict stages say it without either name: stage 2 is
   the side you are replaying onto, stage 3 the side being applied.

   ```bash
   surface=docs/tally/compatibility/compatibility-surface.json
   pins() { python3 -c 'import json,sys; [print(f["path"]) for f in json.load(sys.stdin)["files"]]' | sort; }

   # during a rebase or merge conflict, read the stages -- they are unambiguous
   git show ":1:$surface" | pins > /tmp/pins-base.txt   # merge base
   git show ":2:$surface" | pins > /tmp/pins-ours.txt   # replayed onto / current
   git show ":3:$surface" | pins > /tmp/pins-theirs.txt # being applied / incoming

   # every pin either side ADDED since the base must survive the resolution
   comm -13 /tmp/pins-base.txt /tmp/pins-ours.txt   # added by one side
   comm -13 /tmp/pins-base.txt /tmp/pins-theirs.txt # added by the other

   # and every pin either side REMOVED must stay removed -- additions alone are
   # not enough, see below
   comm -23 /tmp/pins-base.txt /tmp/pins-ours.txt   # removed by one side
   comm -23 /tmp/pins-base.txt /tmp/pins-theirs.txt # removed by the other
   ```

   **Removals need the same treatment, and checking only additions hides them.**
   A pin or claim that one side deliberately retired is still present in the base,
   so it appears in neither `comm -13` output. Take the other side wholesale and it
   comes back; reseal and the gate accepts it, because a resurrected pin hashes
   fine. The retirement is silently undone, and which way it goes depends only on
   which side step 2 happened to start from.

   The union of additions minus the union of removals is the answer. Where one side
   removed an entry the other side *modified*, that is a genuine add/remove conflict
   and wants a decision, not a default -- resolve it explicitly and say which way in
   the commit.

   If the conflict is already resolved and the stages are gone, use `REBASE_HEAD`
   (rebase) or `MERGE_HEAD` (merge) for the incoming side, never `origin/master`.

   Do the same for the matrix's claims. A pin or claim that exists on one side and
   not in your result is being deleted, and nothing downstream will say so.
4. **Recompute `MAX_SURFACE_FILES` from the reconciled pin count, BEFORE regenerating.**
   Do not carry a number derived from either side's cap. The ordering is not a
   preference: `tools/bridge-tally-compatibility/src/lib.rs` is itself pinned, so
   editing the constant after regenerating leaves its own digest stale and the gate
   fails `surface_file_changed` until you reseal again. Every edit to a pinned file,
   the cap included, belongs before the regeneration that hashes it, or the
   regeneration has to be repeated.
5. Regenerate: `scripts/reseal.sh` (or the one `rehash-surface` command above).
6. Run the gate, and **check the pin count against the union you computed in step
   3** -- the gate cannot do this for you.

   `rehash-surface` also reports a changed-entry count, which is a check on your
   reasoning **once you know what it counts**: only entries already in the list
   whose digest on disk differs from the digest recorded. A newly added entry whose
   digest you computed from disk is therefore **not** counted -- it already matches.
   So adding one pin and raising the cap reports **one** (the tool's own pinned
   source, changed by the cap edit) if you computed the new entry's digest from
   disk, and **two** if you added it with a placeholder digest, as "Adding or
   removing a pin" above suggests. Reconcile the number with how you added the pin; do not adjust a
   digest to reach an expected count.

A rebase carrying several commits that touch pinned files needs this at **each**
commit that does, not once at the end. CI gates the final tree, but a history whose
intermediate commits do not gate is not bisectable.

**`MAX_SURFACE_FILES` is the line most likely to be silently wrong, and it is worse
when it does NOT conflict.** The convention is to pin exactly the count in use, so
any branch adding a pin must raise it. If two branches start from the same cap and
each add one pin, both change it from N to N+1 -- an **identical edit**, which git
merges automatically without ever showing you a conflict. The reconciled surface
then holds N+2 pins against a cap of N+1, and step 4 fails with
`surface_file_count_invalid`.

That failure is loud, so it is not dangerous; what is misleading is expecting a
conflict to prompt you. Recompute the cap from the reconciled pin count every time,
whether or not git stopped to ask. The cap *test* derives its size from the constant
precisely so that changing the cap does not also rewrite the test.

Two further constraints apply:

- `MAX_SURFACE_FILES` caps the pin count, and `RESERVED_SURFACE_FILES` bounds
  how far the cap may exceed it. When the surface is at its cap, adding a pin
  requires raising the constant, which the constant's own comment calls an
  explicit compatibility-surface decision -- record the reason in the commit.
- The tool pins its own source, so editing `tools/bridge-tally-compatibility`
  to raise that cap stales its digest and needs another reseal after the edit.
  Expect two passes, and run the tool's tests between them: a cap change can
  invalidate a test that hard-codes the old bound.

The PowerShell commands are intended for Windows PowerShell 5.1 and PowerShell
7+. They deliberately do not use `>`: Windows PowerShell 5.1 redirection was
measured to produce UTF-16LE. The output-path procedure is reasoned from the
tool's byte writer, not host-verified; before relying on it on a Windows host,
confirm the result with `Format-Hex` and require no UTF-8 BOM (`EF BB BF`).

#### `scripts/reseal.sh` (wrapper)

`scripts/reseal.sh` wraps the `rehash-surface` command above into one call, on
Unix hosts. It resolves the pinned toolchain, prints the tool's own
changed-entry count so a reseal that changed nothing (a no-op run) is visible as
exactly that, and then prints the surface coverage report.

```sh
scripts/reseal.sh                  # reseal: rehash every pinned file
scripts/reseal.sh --pins-changed   # the same; kept for existing instructions
scripts/reseal.sh --verify         # reseals into a scratch copy and fails if
                                    # it differs from the committed surface;
                                    # never mutates the working tree. For CI --
                                    # not wired into any workflow yet, since
                                    # this repository's automation may not
                                    # touch `.github/` here; see
                                    # docs/proposed-dependency-policy.md for
                                    # the proposed step.
```

It resolves the pinned toolchain itself (the same `rustc --version` shadowing
hazard described above), and always runs from the repository root regardless
of the caller's current directory. It has no PowerShell equivalent; run the
`rehash-surface` command by hand on Windows.

#### Merge driver (local only)

Under schema 1, two PRs that each resealed after touching DIFFERENT
already-pinned files always conflicted, on exactly two lines: the stored
aggregate digests `manifest_sha256` (in the surface) and
`compatibility_surface_sha256` (in the matrix). Schema 2 stores neither
(bridge#760), so that case no longer conflicts anywhere, and
`scripts/reseal-merge-driver.test.mjs` checks it with the driver overridden by
git's built-in text merge (an `info/attributes` entry `merge=text`), as GitHub
merges. What remains for the driver is the rarer case of both sides
changing the pin list or the claims.

A git merge driver at `scripts/reseal-merge-driver.mjs`, wired via
`.gitattributes`, resolves the common case of that automatically: it
reconciles the surface's pin list and the matrix's claim list with a real
three-way merge (independent additions and removals from either side are
both kept/honored automatically; the SAME entry changed on both sides to
DIFFERENT content is refused, not guessed at, and falls back to git's
ordinary conflict markers for manual resolution exactly as described above),
then writes the reconciled lists. Every pinned file's post-merge hash is computed from git refs
(`git diff --name-only`/`git show` against the merge base, "ours" and
"theirs"), never from the working tree -- an earlier version of this driver
read the working tree instead and a real merge experiment caught it sealing
a wrong hash, because git does not guarantee every other path has already
been checked out to its final post-merge content by the time this driver
runs for the compatibility files specifically. It runs no tool, so it needs no
Rust toolchain. See the extensive comments in
`scripts/reseal-merge-driver.mjs` for the exact mechanics, including why
`.git/MERGE_HEAD` -- the seemingly obvious way to learn "ours"/"theirs" from
inside a running merge driver -- does not work (it is not written until
*after* the whole tree-level merge finishes, i.e. after every driver
invocation, and in a linked git worktree `.git` is a redirect file rather
than a directory besides) and the gitattributes placeholders (`%S`/`%X`/`%Y`)
used instead.

**This is a LOCAL-ONLY convenience.** A `.gitattributes` `merge=` driver
requires local git configuration to activate (below) and runs only when
*your own* `git merge`/`git rebase` executes on *your* machine. **GitHub's
server-side merge -- the "Merge pull request" button, and the mergeability
check GitHub computes for an open PR -- does not run repository merge
drivers at all; GitHub has no mechanism to execute arbitrary repository code
as part of that merge.** So this reduces the pain of a maintainer juggling
several compatibility-surface branches locally; it does **not** change what
a PR shows as conflicting on GitHub, and does not touch anything under
`.github/`. A PR that would conflict on GitHub still needs a rebase/merge
performed locally (with this configured) to resolve automatically, then
pushed.

One-time setup per local checkout (not committed -- `.gitattributes` names
the driver, but the driver's actual command has to come from local git
config, by design: git will not execute arbitrary commands named in a
version-controlled file without an explicit local opt-in):

```sh
git config merge.bridge-compat-reseal.name "Bridge compatibility-surface reseal driver"
git config merge.bridge-compat-reseal.driver "node scripts/reseal-merge-driver.mjs %O %A %B %P %S %X %Y"
```

When it declines to resolve (a genuine conflict on the pin/claim list, or on
a pinned file's own content, or an operation other than an ordinary `git
merge`), it falls back to git's plain three-way text merge and prints why --
resolve the conflict markers by hand following the procedure above, then run
`scripts/reseal.sh` yourself.

#### Migrating an open branch across bridge#760

A branch cut before bridge#760 still carries schema-1 files and the old driver.
During a local merge git runs the driver of the side that is **checked out**, so
merging master into such a branch runs the *old* driver. It writes schema-1
files, which the new tool, `scripts/reseal.sh` and the gate all refuse
(`artifact_json_invalid`). That fails closed, but it has to be finished by hand,
once per branch:

1. Merge master without committing, and resolve every source conflict:

   ```sh
   git fetch origin
   git merge --no-commit origin/master
   ```

2. Take master's two files, which have the schema-2 shape (`schema_version: 2`,
   no `manifest_sha256`, no `compatibility_surface_sha256`):

   ```sh
   git checkout origin/master -- docs/tally/compatibility/compatibility-surface.json \
     docs/tally/compatibility/compatibility-matrix.json
   ```

3. Re-apply the branch's own authored changes, if it had any: pins it added (in
   sorted order, with any 64-hex placeholder `sha256`) or removed, and claims it
   changed. Compare with the merge base as in "When the surface itself conflicts
   in a merge or rebase" above. A branch that only changed the contents of
   already-pinned files has nothing to re-apply.
4. Reseal, verify and commit:

   ```sh
   scripts/reseal.sh
   scripts/reseal.sh --verify
   git add docs/tally/compatibility/compatibility-surface.json docs/tally/compatibility/compatibility-matrix.json
   git commit
   ```

The merge commit then holds schema 2, and every later merge runs the new driver.
Updating such a branch on GitHub ("Update branch") instead reports a conflict
on the two files whenever the branch had resealed; resolve it locally the same
way.

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

1. Run `scripts/reseal.sh`, because three of those files are pinned.
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
