# Education import-counter responses — 2026-07-29

`live_education_w1_ledger_sanitized.xml`,
`live_education_w4_voucher_sanitized.xml` and
`live_education_w7_baddate_sanitized.xml` are import responses **derived from**
live captures taken 2026-07-29 in Tally Education mode; the captures themselves
are not committed. The only recorded transformation is that w7's `LINEERROR`
text was replaced with `REDACTED_LIVE_LINEERROR` while its presence was kept
(`../import_evidence.rs`). Whether anything else was changed is not recorded,
nor are port, company or request, so no capture byte count or SHA-256 is
declared. The integrity digests below pin the committed bytes only.

They are counter-shape evidence: w1 and w4 are clean single creations; w7 is a
rejection despite `ERRORS=0`, because it reports `EXCEPTIONS=1` and a
`LINEERROR`. Added in `7bd6123a` (#106).

## Integrity digests

Each row is the SHA-256 of the file's committed bytes, which
`scripts/check-fixture-provenance.mjs` checks (#838). A digest pins the bytes as
committed and claims nothing about where they came from: the Capture column
repeats only what this note says above.

| Fixture | Bytes | SHA-256 (integrity digest) | Capture |
| --- | ---: | --- | --- |
| `live_education_w1_ledger_sanitized.xml` | 236 | `b36f700000f96d5bb3af2f44facf881e0d0e276d485e8da3c7d06df577b73e60` | not established: derived from a capture, as above |
| `live_education_w4_voucher_sanitized.xml` | 238 | `d1bc6bc73199a33968f8223342b605494672259f53ecb281ed5ebc852ed76bcf` | not established: derived from a capture, as above |
| `live_education_w7_baddate_sanitized.xml` | 282 | `e604a38040a6c2f0d3c43507d8f7a4c87101ec8887d86246b42b37fe7fabc33b` | not established: derived from a capture, as above |
