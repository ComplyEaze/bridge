# Simulator fixture provenance

Every file in this directory is hand-authored synthetic simulator input. None
is a capture from Tally, and none may be read as evidence of what a real Tally
release emits. They were added together in `0100e6b2` (2026-07-16) under the
fixture rules in `../README.md`: `BRIDGE SYNTHETIC` names, GUIDs in the
reserved `00000000-0000-4000-8000-…` range, `BRIDGE_SYNTHETIC_*` line errors,
and no output copied from a real company or Tally installation. The
`COMPANYCONTEXT` schema strings (`bridge.tally.ledgers/1`,
`bridge.tally.vouchers/2`, `/3`) are Bridge's own, not native Tally.

`../src/fixtures.rs` loads each file with `include_str!` into a `Fixture`
variant. The only transformation is `ScenarioPlan::response_bytes`, which
re-encodes the checked-in UTF-8 as UTF-8, UTF-8 with BOM, or UTF-16LE/BE; the
UTF-16 variants are never checked in.

Because these are authored rather than captured, no capture byte count or
SHA-256 is declared here; the integrity digests at the end pin the committed
bytes only. `inconsistent_date_filter.xml` is additionally pinned by
SHA-256 in `docs/tally/compatibility/compatibility-surface.json`.

| Fixture | Models |
| --- | --- |
| `export_status_1.xml` | `STATUS=1` with an empty collection |
| `export_status_0.xml` | `STATUS=0` with a `LINEERROR` — an application rejection |
| `export_status_invalid.xml` | `STATUS=-1` |
| `export_status_missing.xml` | no `STATUS` element |
| `normal_export.xml` | one ledger, `OPENINGBALANCE` -1180.00 |
| `empty_export.xml` | `RECORDCOUNT` 0 |
| `duplicate_identity.xml` | two ledgers sharing one GUID |
| `wrong_company.xml` | a response for a different synthetic company |
| `voucher_export.xml` | one Receipt voucher carrying `REMOTEID` |
| `inconsistent_date_filter.xml` | a declared July window containing a June voucher |
| `record_count_mismatch.xml` | declares two records, contains one |
| `malformed_export_metadata.xml` | `RECORDCOUNT` -1 |
| `duplicate_export_metadata.xml` | `COMPANYCONTEXT` given both as attribute and as child elements |
| `exact_decimals.xml` | boundary decimal amounts, including 999999999999.9999 |
| `import_counters.xml` | clean import counters |
| `import_duplicate.xml` | `IGNORED=1`, `ERRORS=1` with a duplicate-identity line error |
| `import_partial.xml` | mixed import counters with an exception |
| `malformed.xml` | an unclosed element |
| `truncated.xml` | a body cut off mid-element |
| `synthetic_json_semantic_reference.json` | Bridge-shaped JSON restating `normal_export.xml`'s values; not a Tally JSONEX envelope and not a parity fixture |

## Integrity digests

Each row is the SHA-256 of the file's committed bytes, which
`scripts/check-fixture-provenance.mjs` checks (#838). A digest pins the bytes as
committed and claims nothing about where they came from: the Capture column
repeats only what this note says above.

| Fixture | Bytes | SHA-256 (integrity digest) | Capture |
| --- | ---: | --- | --- |
| `export_status_1.xml` | 117 | `50a3c870f1acb8db00ddd5a7a1e43c870ab3c663637991b90b18851f81410cf1` | not a capture: authored, as above |
| `export_status_0.xml` | 164 | `c7a7fb4edd0a21a823e924727dcc5965b8ef2d5b5704f57d4b8da67cd6eb22f4` | not a capture: authored, as above |
| `export_status_invalid.xml` | 118 | `8014c5aac4400b8652ea95d0f27011e3ba84f0cd14e977e7aa5fdccb72f7a26c` | not a capture: authored, as above |
| `export_status_missing.xml` | 99 | `e5631b509a8994f2d7c8eee3c74f0481a418631d02abbef938d6e7d5b14df937` | not a capture: authored, as above |
| `normal_export.xml` | 562 | `51adcd574cc9d5b52d7ab96e85725cac670d574ad1c6db010d6bd7b2509714e4` | not a capture: authored, as above |
| `empty_export.xml` | 317 | `30205d47a04c0c75a0b02fed5ce9e2c0ac571fd5c3a090686f89d6f2067d6456` | not a capture: authored, as above |
| `duplicate_identity.xml` | 560 | `5b2f9b82e9d16821d542c8a116066f322e375506f51e29f8ee85adf6fa89f85d` | not a capture: authored, as above |
| `wrong_company.xml` | 323 | `bcce68996fdedddb31ef799dc8ecb949860ed069922eb89f9376b8ad25b8d475` | not a capture: authored, as above |
| `voucher_export.xml` | 626 | `621c185473bb9fa8e059cb6932117ba0670264820ed41a7999166eb200bb964d` | not a capture: authored, as above |
| `inconsistent_date_filter.xml` | 675 | `2bb6598b08167f3c38f99e7e0fc2902bbc2012b210b2f4c1ba030eacac27068c` | not a capture: authored, as above |
| `record_count_mismatch.xml` | 449 | `44da2028856326617f3bc596b508768711b1481adcf21d6d7339e3282c1a6bc0` | not a capture: authored, as above |
| `malformed_export_metadata.xml` | 320 | `41406e5029ff136132a6649af7b9443968a956129b076279aa923b6e86c5db4b` | not a capture: authored, as above |
| `duplicate_export_metadata.xml` | 351 | `eaa617c9ec831e3416dec7a16eca1c91b92b7ab0ccac06ac1cbecfeb9a282734` | not a capture: authored, as above |
| `exact_decimals.xml` | 226 | `ca8eb379d2293c55fde078e047bbd69c423b2e1dc185b674fec37372cd96eef7` | not a capture: authored, as above |
| `import_counters.xml` | 170 | `eac0f68ac2d7acfefe6f9e7716c2c1de2a4b33dd110ddf869c5e927add073833` | not a capture: authored, as above |
| `import_duplicate.xml` | 228 | `549a89ebb12839a0066f5bc4b31f53da7f0bc370f92fec3697bc1fc28585c115` | not a capture: authored, as above |
| `import_partial.xml` | 230 | `85a26867af5f969a6e5d916c1d4a1a830a9c8ad66fbbc86230299cbac62136df` | not a capture: authored, as above |
| `malformed.xml` | 78 | `26ef82c65fda0aed3af2291947a812a3a3fb1ffdac08c1c0625de227c9a902f5` | not a capture: authored, as above |
| `truncated.xml` | 113 | `b836e44596b6fe4a8d9045d9c32fb42b3aa951b1e2ade25138479130b138372d` | not a capture: authored, as above |
| `synthetic_json_semantic_reference.json` | 247 | `8eb242d02538e4b4af98401860e969ac9d8691bb1e733d5595a7c832be97c9c2` | not a capture: authored, as above |
