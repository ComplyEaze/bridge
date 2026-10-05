# CompanyBookExtentV2 field observation

Exact decoded XML captured by the supervised, single-attempt read-only field observation on 2026-09-09. Source artifact SHA-256: `53f9fe484a1bc658bfaa37da5ed8c76f241dfe28e519bb7345478fc599bda334`.

The capture establishes that this response shape returned `COMPANYNUMBER` alongside the existing extent witnesses for the observed sandbox endpoint. It does not qualify the versioned V2 request collection ID, change production admission, or establish support on another endpoint.

A separate native V2 observation on the same date used the exact `BridgeCompanyBookExtentV2` renderer through the shared runtime and production extent client. The V2 paired response produced these same decoded bytes. Fresh full-tuple identity, licensed-mode, paired response, opening/closing extent equality, and master-witness checks passed. This is one-endpoint read evidence, not financial completeness, write authority, or a compatibility-cell promotion. V2 request source SHA-256: `ad6615a7ffb2b9a5e524df26a8b1a4583f8e393c353c83db0596d2eca88981b0`.

## Integrity digests

Each row is the SHA-256 of the file's committed bytes, which
`scripts/check-fixture-provenance.mjs` checks (#838). A digest pins the bytes as
committed and claims nothing about where they came from: the Capture column
repeats only what this note says above.

| Fixture | Bytes | SHA-256 (integrity digest) | Capture |
| --- | ---: | --- | --- |
| `native-company-book-extents-with-number.utf8.xml` | 8,817 | `53f9fe484a1bc658bfaa37da5ed8c76f241dfe28e519bb7345478fc599bda334` | exact decoded XML capture, as above |
