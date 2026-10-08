# Parse record, app crate (bridge#1198, slice 2)

These files are **generated, never edited by hand**. Each one is what one of this crate's own XML
parsers returns today for one response fixture, written by `src/agent_parse_record_tests.rs`. The
protocol crate's record (`crates/bridge-tally-protocol/tests/parse_record/`) covers that crate's
parsers and leaves these fixtures to this one. They are not captures: the fixtures stay in the
protocol crate's `tests/fixtures/`, with their provenance and byte-integrity checks.

**What a file holds.** Each file is named `<fixture path>.<parser>`, with `/` written as `__`.
- A header names the fixture, the parser with its fixed arguments, and whether the fixture is
  `captured` or `synthetic` (authored or derived, not evidence of what Tally sends).
- After the header comes the parser's whole result: the value's pretty `Debug`, or its error.
- An error that is one of this crate's codes (a lower-case snake-case word such as
  `agent_read_protocol_invalid`) is recorded as that code. Any other error text, which the XML
  library may have written, is recorded as `Err(xml library error)`, never by its own words.
- A record that is an error shows which refusal the parser reaches first, not how the rest of the
  fixture's content parses.

**The fixed arguments** are copied from the tests that read each fixture: the company GUID its
test names (the GUID its capture carries), and for a voucher census the one day its capture asked
for (the day its provenance gives, or its paired request's `SVFROMDATE`/`SVTODATE`).

**Full and hashed records.** A rendering up to 128 KB is committed in full (`.txt`). A longer one is
committed as its SHA-256 and its length (`.sha256`).

**Checking.** `cargo test -p bridge --lib parse_record_tests` compares every rendering with its
record. On a mismatch it prints the fixture, the parser and the first differing line, and no more.

**Re-recording.** Do this only when a parser's output is meant to change:

    BRIDGE_RECORD_PARSES=1 cargo test -p bridge --lib parse_record_tests

That rewrites every record and removes any record no row writes. It refuses to run where `CI` is
set. Review the diff of this directory like any other change of behaviour.

**Comparing hashed records.** To compare full renderings between two revisions, run the test with
`BRIDGE_PARSE_RECORD_DUMP=/an/absolute/path/outside/the/repository` at each revision, and diff the
two directories.
