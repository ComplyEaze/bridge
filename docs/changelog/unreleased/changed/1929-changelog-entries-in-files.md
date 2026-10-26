- The entry for a change that is not yet in a published build is now one file under `docs/changelog/unreleased/`
  (a folder each for added, changed and removed), not lines added to `CHANGELOG.md`, so two changes no longer
  edit the same lines. The install site's "What changed" page and the release notes list these files under
  the unreleased changes, and cutting a release moves them into its section (#1929).
