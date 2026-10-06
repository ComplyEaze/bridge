# Sanitized Tally JSONEX structure fixtures

These fixtures are synthetic structural derivatives of Tally Solutions'
official TallyPrime 7.0+ JSON integration examples, reviewed on 2026-07-15:

- https://help.tallysolutions.com/tally-prime-integration-using-json-1/
- https://help.tallysolutions.com/wp-content/uploads/2025/11/Ledger-Collection-Response.docx
- https://help.tallysolutions.com/wp-content/uploads/2025/11/voucher-collection-response.docx

The two fixtures are `ledger_collection_sanitized.json` (from the Ledger
Collection example) and `voucher_collection_nested_sanitized.json` (from the
voucher collection example). The original downloadable examples are not committed. The checked-in files use
synthetic Bridge names, identifiers, and voucher numbers while retaining the
documented envelope, wrapper, omitted-versus-empty, multilingual, accounting-
value, and nested-array shapes needed for parser tests. They contain no live
Tally capture, customer data, phone/email/address, GST registration, bank
detail, local path, or developer identity.

This corpus is structure evidence only. It does not prove Bridge's custom TDL
JSONEX profile, company identity binding, date-range filtering, completeness,
source atomicity, Education-mode availability, performance, or production
support. Redistribution of the official DOCX assets is not required; this
repository stores only independently authored synthetic test JSON.

## Integrity digests

Each row is the SHA-256 of the file's committed bytes, which
`scripts/check-fixture-provenance.mjs` checks (#838). A digest pins the bytes as
committed and claims nothing about where they came from: the Capture column
repeats only what this note says above.

| Fixture | Bytes | SHA-256 (integrity digest) | Capture |
| --- | ---: | --- | --- |
| `ledger_collection_sanitized.json` | 1,624 | `97c3267251ba59fb4acda1ae28c243f3406314b4a325d617b8fd8b16c9866925` | not a capture: a synthetic derivative, as above |
| `voucher_collection_nested_sanitized.json` | 6,257 | `89290a024a3a32e00af088b14393f217c5ca838c51822c21b20cba29855a47b0` | not a capture: a synthetic derivative, as above |
