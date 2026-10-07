# Parse record (bridge#1198)

These files are **generated, never edited by hand**. Each one is what one of this crate's parsers
returns today for one response fixture, written by `tests/captured_parse_record.rs`. They are not
captures, so they are kept out of `tests/fixtures/` and its provenance and byte-integrity checks.

**What a file holds.** Each file is named `<fixture path>.<parser>`, with `/` written as `__`.
- A header names the fixture, the parser with its fixed arguments, and whether the fixture is
  `captured` or `synthetic` (authored or derived, not evidence of what Tally sends).
- After the header comes the parser's whole result: the value's pretty `Debug`, or the error's typed
  variant. A few types whose own `Debug` hides fields are rendered field by field instead.
- An error that comes from the XML library itself is recorded as `Err(xml library error)`, never by
  the library's name or text.
- A record that is an error shows which refusal the parser reaches first, not how the rest of the
  fixture's content parses. A fixture whose every row is an error is recorded only that far.

**Full and hashed records.** A rendering up to 128 KB is committed in full (`.txt`). A longer one is
committed as its SHA-256 and its length (`.sha256`).

**Checking.** `cargo test -p bridge-tally-protocol --test captured_parse_record` compares every
rendering with its record. On a mismatch it prints the fixture, the parser and the first differing
line, and no more.

**Re-recording.** Do this only when a parser's output is meant to change:

    BRIDGE_RECORD_PARSES=1 cargo test -p bridge-tally-protocol --test captured_parse_record

That rewrites every record and removes any record no row writes. It refuses to run where `CI` is
set. Review the diff of this directory like any other change of behaviour.

**Comparing hashed records.** To compare full renderings between two revisions, run the test with
`BRIDGE_PARSE_RECORD_DUMP=/an/absolute/path/outside/the/repository` at each revision, and diff the
two directories.
