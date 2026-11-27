# Bridge managed Git guidance

Run commands from the repository root in any local clone. Repository setup is
already complete for an existing clone; do not reinitialize it.

## Verify repository identity

- `package.json` name is `bridge`.
- `src-tauri/Cargo.toml` package and default binary are `bridge`.
- `src-tauri/tauri.conf.json` product name is `Bridge`.
- No configuration refers to a developer-specific absolute path.

## Connect a remote for a new clone or migration

Use the repository URL supplied by its owner instead of embedding a personal
account in project documentation:

```text
git remote add origin <repository-url>
git push -u origin master
```

Do not publish until tracked files and Git author metadata have been checked for
personal information.

## Managed repository settings

- Require pull requests for `master`.
- For a solo-maintainer repository, require the pull-request path with zero
  mandatory approvals, strict status checks, conversation resolution, and
  admin enforcement. Increase to at least one approval and dismiss stale
  approvals as soon as a second trusted maintainer is available; do not create
  a policy that can only be satisfied through routine bypasses.
- Require linear history and these exact status contexts: `Frontend build`,
  `Rust format`, `Required checks`, `Dependency security`, and
  `GitGuardian Security Checks`. The `Required checks` aggregate gate
  transitively covers `Native checks (*)`
  and `Bundle smoke (*)` (on a master
  push, for a commit the merge queue already ran the Windows jobs on, through
  that queue run: `scripts/master-push-reuse.mjs`; a pull request and the queue
  run the native tests on Windows only, and a master push, the nightly run, a
  manual run and a release pull request (one that changes the version in
  `src-tauri/tauri.conf.json`) add macOS; a pull request runs the bundle and seam
  jobs on Windows only, and only when it changes a packaging path, a crate manifest,
  the Windows resource script or the Windows setup action, or is a release
  pull request (which adds macOS); the queue runs them for every change they cover,
  and the queue and a push add macOS only when a packaging path changes, named in
  the scope job; a draft pull request runs no native, bundle or seam job, reports
  its gate as `Required checks (draft)`, which is not a required context, and
  marking it ready starts a run that does); require the
  gate rather than those individual matrix contexts, because they are
  path-filtered and report `skipped` on docs-only changes — requiring them
  directly makes documentation pull requests permanently unmergeable. The gate
  fails when a job the scope selected was skipped, so a skip it accepts is
  always one the scope chose.
- A draft pull request's green run is never called a full run: it ran no
  native, bundle or seam job.
- Working rule for a ready pull request whose change the queue runs the bundle
  and seam jobs for and its own pull-request run does not (#2455):
  - The merge queue run is its bundle and seam proof.
  - For that proof, a manual run of `ci.yml`
    (`gh workflow run ci.yml --ref <branch>`) is requested before queueing only
    when the author needs release-profile proof, and the author gives the
    reason in the pull-request body. The manual run for macOS native evidence
    in `AGENTS.md`, and the one after an ejection below, are separate.
- After the queue ejects a pull request, it is not queued again until the
  failing job has passed on the fixed head outside the queue, on the system it
  failed on: in one manual run, or, for a Windows failure, in the pull-request
  run if the pull request's whole change now selects that job. The exception is
  a log that shows an infrastructure error, such as a download 5xx, a timeout or
  a fetch failure.
- Enable private vulnerability reporting.
- Restrict merge permissions to maintainers.
- Require issue and pull-request templates.
- Require external actions to use full-length commit SHAs and keep the
  human-readable action tag on the same line for Dependabot updates.
- Protect `v*` release tags from updates and deletion after publication.

## Governance labels

- Area: `area:tally`, `area:tax-audit`, `area:infra`,
  `area:security`
- Severity: `severity:p1`, `severity:p2`, `severity:p3`, `severity:p4`
- Type: `type:bug`, `type:feature`, `type:chore`, `type:rectify`

Each actionable bug receives one area, one severity, and one type label.

## Ongoing checks

- Keep [the roadmap](../step-by-step-roadmap.md) current.
- Use [the review checklist](../../review-checklist.md) in every pull request.
- Follow [rectification guidelines](../rectify-guidelines.md) for regressions.
- Validate platform-sensitive changes natively on Windows and macOS.
- Scan public artifacts for secrets, personal/customer data, usernames, home
  directories, and absolute local paths.
