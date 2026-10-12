# Bridge Tally protocol

Portable production parsing for Bridge's Tally integration. This crate owns:

- bounded byte decoding for UTF-8, UTF-8 BOM, UTF-16LE, and UTF-16BE;
- strict export-envelope `STATUS` handling;
- company, ledger, and voucher parsing;
- exact import counters without retaining raw `LINEERROR` text;
- optional company-context and duplicate-identity evidence.

It intentionally has no HTTP client, Tauri, database, SQLCipher, or native
dependency. `tally-protocol-simulator` is a development dependency and supplies
the synthetic compatibility corpus.

Bridge retains its existing `tally::xml_parser` API as a thin re-export of this
crate. New callers that need verification evidence can use
`parse_*_with_evidence`; legacy `parse_*` functions still return the original
record vectors.

Duplicate source identities are exposed only as occurrence counts and SHA-256
digests. Raw duplicate identifiers and raw line-error messages are not retained
in evidence or error strings.
