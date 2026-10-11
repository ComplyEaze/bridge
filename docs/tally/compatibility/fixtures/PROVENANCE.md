# Compatibility fixture-manifest provenance

Neither file in this directory is a Tally capture. Both are authored manifests
that describe a disposable synthetic lab company an operator creates by hand in
Tally Education, and the expectations a live-read run is checked against. Both
were added in `56409e62` (2026-07-16).

- `education-small-v1.json` — the `synthetic_small` tier: company marker,
  ledger and voucher sentinels, date ranges and count bounds. Read by
  `tools/bridge-tally-live-read` as `SyntheticFixtureManifest`; the operator
  steps are in `../live-education-runbook.md`.
- `education-native-outstandings-v0.json` — the accounting facts an operator
  must create, three request scenarios, a request budget and a UI observation
  contract. Read by the native-outstandings qualification runner as
  `NativeFixtureManifest`; see `../bills-native-probe-runbook.md`. Its
  `request_sha256` and `template_sha256` values are digests of Bridge's own
  request templates, not of any Tally response; they were resealed in
  `ad18c0df` when the request bytes changed. The file records its own limits
  (`observation_posture: profile_unobserved`,
  `fixture_facts_are_accounting_expectations_not_xml_semantics`).

Both are pinned by SHA-256 in `../compatibility-surface.json`, which the
runners validate before use; that manifest is the runners' hash check. The
integrity digests below are of the same committed bytes, for the fixture
provenance gate.

## Integrity digests

Each row is the SHA-256 of the file's committed bytes, which
`scripts/check-fixture-provenance.mjs` checks (#838). A digest pins the bytes as
committed and claims nothing about where they came from: the Capture column
repeats only what this note says above.

| Fixture | Bytes | SHA-256 (integrity digest) | Capture |
| --- | ---: | --- | --- |
| `education-small-v1.json` | 677 | `4783d8ae7c0ce81ed63f4dbead855b81b792890f7ea2d5ab49c44b2e78366553` | not a capture: an authored manifest, as above |
| `education-native-outstandings-v0.json` | 6,273 | `b07c10850ed2294c216687dd227b9ab3ae0a6bcc9fadf369402d4994e07b6a4d` | not a capture: an authored manifest, as above |
