# Parser fixtures

`*-bbox-capture.xml` are `pdftotext -bbox-layout` output from **real** bank statements, sanitised.
Every coordinate, word break, line break and entity encoding is the PDF producer's; every customer
value is fabricated. They are the only fixtures in this repository that can catch a change in a
bank's statement template or in poppler's serialisation, because they are the only ones this
repository did not write.

Read the banner comment at the top of each file for exactly what is real and what is not.

`source-draft-capture-bindings.json` is not a bank capture. It is a
`SourceDraftCatalogTargets` payload: its `targets` are the ledger names the production reader
parses from the captured, synthetic-company catalogue
`src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/native-ledger-catalogue.utf16le.xml`
(see that capture's `.json` sidecar), its `bindings` come from running authored source XML through
the production binder, and its `evidence` digests are copied from the sidecar. Its own `provenance`
object says the same. `src-tauri/src/source_draft/catalog_tests.rs` asserts that `targets`,
`bindings` and `evidence` still equal the live producer's output; `scripts/source-draft-screen.test.tsx`
serves it as a mocked backend response. No capture byte count or SHA-256 is declared for it; its
integrity digest, at the end, pins the committed bytes only.

## Re-deriving them

`../sanitise-bbox-capture.py` is the whole procedure and the fixtures reproduce from it byte for byte:

```bash
pdftotext -bbox-layout -opw "$PASSWORD" statement.pdf raw.xml
python3 scripts/sanitise-bbox-capture.py raw.xml \
  scripts/fixtures/hdfc-bbox-capture.xml hdfc \
  0:0-800 2:200-330,700-800 3:200-300
```

```bash
python3 scripts/sanitise-bbox-capture.py raw.xml \
  scripts/fixtures/sbi-bbox-capture.xml sbi \
  0:90-741 1:0-165
```

The page ranges are chosen to keep each parser rule load-bearing: a full first page with its account
header, a continuation page that repeats the column header while a row is in progress, the page that
carries the end-of-statement marker, and — for HDFC — one page *after* that marker, so the marker
cannot be removed without a test noticing.

The bank argument is a closed parser selection (`hdfc` or `sbi`). The sanitiser parses every selected
source region and the complete generated regions, using the same retained geometry before writing the destination. It refuses empty,
incomplete accounting rows, misaligned or party-class-changing evidence; captured geometry remains fixture evidence and does not
qualify raw customer data.

## Adding a capture for a new bank

1. Sanitise, then **diff the result against the source** and scan for surviving tokens (the script's
   `--help` prints the one-liner). Everything that survives should be bank vocabulary.
2. If a customer value survives, do not add it to `TEMPLATE`. Work out why the rule matched it.
3. Byte integrity is enforced: `scripts/fixtures/**` is `-text` in `.gitattributes` and the
   directory is registered in `scripts/check-fixture-byte-integrity.mjs`. Line-ending normalisation
   would rewrite the geometry these fixtures exist to preserve. That is also why the sanitiser
   itself lives in `scripts/`, not here — this directory holds evidence, and a tool whose bytes are
   pinned as evidence is a category error.

## Integrity digests

Each row is the SHA-256 of the file's committed bytes, which
`scripts/check-fixture-provenance.mjs` checks (#838). A digest pins the bytes as
committed and claims nothing about where they came from: the Capture column
repeats only what this note says above.

| Fixture | Bytes | SHA-256 (integrity digest) | Capture |
| --- | ---: | --- | --- |
| `hdfc-bbox-capture.xml` | 34,113 | `ff01bde0f4330141907f2d6e9eeba8e2cf49c77c53437f3f8cdb10fc374bb6e2` | sanitised from a real statement, as above |
| `sbi-bbox-capture.xml` | 21,091 | `615280486efa461bb15a4d0d4103fc66ea9706799171966cf739bd1447828916` | sanitised from a real statement, as above |
| `source-draft-capture-bindings.json` | 1,936 | `8e7b43f90a93086ece50c270b9747c83ba473ee1fc4246c8d23cf160592dfe2e` | not a capture, as above |
