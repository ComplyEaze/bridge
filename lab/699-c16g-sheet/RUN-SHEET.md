# 699-C16G: the audit read of BRIDGE EINVOICE LAB, a GST-on book, recorded byte for byte. READ ONLY.

Not for merge. Part of #699 (clause 16(a)/(d), step 1). Written 4 Nov 2026 at master `1400175e`, and this branch starts there.
`SHA256.txt` lists every file here with its bytes; check it before step 0. Confidence of everything here until the row runs:
unverified. It runs after 699-C16 and does not depend on it: it reads a different book.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy of the one on `lab/699-c16-sheet` (sha256
5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6). Where this sheet and LAB-COMMON differ, the stricter rule holds.
This row writes nothing, so LAB-COMMON's backup and restore rules (C5, C6) are not used.

THE COMPANY is `BRIDGE EINVOICE LAB`: synthetic, GST on, TallyPrime 7.2 Silver, on the lab PC, GUID
`eb2f5190-292e-4ed8-bfd3-eace839fe699`. "This GUID" below means that one. Its last state on record is in `lab/2027-dn2-answers`
RESULTS.md ("The book now", at f748c46e):
- restored to its state before that sitting;
- three vouchers, all dated 3-Apr-25: Sales `1` (with e-invoice details), Sales `2` and a Payment `1`;
- `ALTVCHID` 3 and `ALTMSTID` 215.

Not on the PC, another GUID, or Educational mode: STOP, the row has no book.

Not a client book, and nothing in it is seeded from client data. Its parties, GSTINs and e-invoice details are invented.

## Why
A list of candidates for Form 3CD clause 16(a)/(d) is being built in the tax-audit crate. It treats a tax ledger credited on a sale in its
own way. Row 699-C16 can only key such a tax line by hand on a book with GST off. This row records Tally's own tax lines on a GST-on book,
read by Bridge's own audit read (AGENTS.md P1). The row also answers whether that read admits a GST book with e-invoice details at all.
Either answer is a result.

## The one variable
None is changed. The row reads the book as it stands, once, with one build. The book's GST setting is not touched.

## Build
The build R1 ran (`lab/699-sft-read` ROW.md at 66cc762c, "Build"): the Windows bundle of the push of master at `d77d0682`.
- Zip sha256 `bd7e796174a185b1523258c3052acad67c29a7d8b2e68d99d5b12d6ce60cc2bf`.
- `bridge_mcp.exe` sha256 `c41fe5df31b3dc6a81c1eb90848837012579c041b6034531721c80c2cabb531c`, as R1 recorded it.
- Another digest is a stop.

The files of the audit read (`agent_audit_read.rs`, `agent_audit_run.rs`, `agent.rs`, `agent_catalog.rs`, `agent_company.rs` under
`src-tauri/src/`) are unchanged from `d77d0682` to `1400175e`.

The environment is R1's, with one `bridge_mcp` process for the whole row and one new empty `BRIDGE_AGENT_DATA_DIR`:
- `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true`, `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9011`;
- `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_BATCH_POST` and `BRIDGE_AGENT_REDACTION` absent.

Record `bridge_mcp.exe`'s sha256 before step 3 and after step 7.

## Kit
| file | what |
|---|---|
| `LAB-COMMON.md` | the lab's common rules, v3, byte copy |
| `lost_answer_relay.py` | the recording relay R1 and 699-C16 use, byte copy (sha256 `060c3b6c…`). It forwards only GET /status and POST reads to 127.0.0.1:9001, answers anything else 403 without forwarding it, and records every exchange byte for byte as `NNNN-request.bin`, `NNNN-response.bin` and `NNNN-response-headers.txt`, one line each in `manifest.tsv` |
| `templates/suspense_create_request.utf16le.xml` | the template the relay loads at start, byte copy. No call in this row sends a ledger create |
| `requests/marks-einvoice.utf16le.xml` | the marks read of `lab/2027-d2` (`M-marks.xml`, 982 B, sha256 `f8b04aca…`, the one rows 2027-DN and 2027-DN2 sent), byte copy: the loaded company's GUID, ALTVCHID and ALTMSTID, UTF-16LE. Send it with `Content-Type: text/xml; charset=utf-16` |
| `.gitattributes` | `* -text`, so no file here is changed on checkout |

Run `python -I lost_answer_relay.py --self-test` before step 3: EXPECT `self-test: ok`.

## Prompts this row expects
None. Nothing is keyed in Tally except loading the company at step 0 and shutting it at step 10. Any prompt or dialog: press no key,
screenshot, record its words, stop.

## Steps
One at a time. Every answer goes to a file, whole. A file with no answer in it is a failed step, not an empty one. "Marks" means
`requests/marks-einvoice.utf16le.xml` sent raw to 9001 by curl, never through the relay. Write down this GUID's presence, V (ALTVCHID)
and M (ALTMSTID), and keep the answer with its sha256. Time every tool call: its start and end from the client's log, in seconds. Each
relay folder is new and empty.

0. Check `SHA256.txt`. Load BRIDGE EINVOICE LAB alone. Screenshot. `GET /status` on 9001. Record Tally's release, Tally's date and the
   PC's date.
1. **g0.** Marks, before anything else. EXPECT one COMPANY row with this GUID. Record V and M. They are recorded, not judged: V 3 and
   M 215 are the last on record, and other values are a result, not a stop. Keep the answer as `g0-marks.xml`.
2. Run the relay's self-test (Kit).
3. Start the relay: `python -I lost_answer_relay.py --listen-port 9011 --record <new folder>\g1-list --company "BRIDGE EINVOICE LAB"
   --ledger-name "Unused 699 C16G"`. Then start `bridge_mcp` pointed at 9011.
   - The client's tools list: record that `audit_test_run` is in it.
   - `list_companies`: EXPECT one open company, this GUID. Anything else: stop.

   Stop the relay with Ctrl+C. Keep `g1-list`.
4. Start the relay as at step 3 into `<new folder>\g1`.
   - `audit_test_run`, once, with the arguments R1 used:
     - `company_guid` this GUID;
     - `from` `2025-04-01`, `to` `2026-03-31`;
     - `test` `cash_44ab`;
     - `cash_groups` [`Cash-in-Hand`], `bank_groups` [`Bank Accounts`].

     Keep the whole answer as a file, with its size and sha256.
   - **A refusal is an answer.** If `isError` is true, record:
     - `structuredContent.evidence.state` (the dispatch's state);
     - the whole `structuredContent.result.error` member, code and message as written.

     Then stop the relay, keep `g1`, and make no other tool call: skip steps 5 and 6 and go to step 7. No repeat is made, whatever the
     code.
   - EXPECT from `g1\manifest.tsv`, read by its columns (never by a substring search):
     - every line's verdict `forwarded`, HTTP status 200 and answer_returned `yes`;
     - no `refused` line and no `events.txt`.

     Record the number of lines, and each line's method, request bytes and response bytes. Every answer is at most 2,000,000 bytes; a
     larger one is kept and is a stop.
   - Record (not predicted), if it answers with a result: `result.state`, each figure and finding id with its value, and the code and
     count of every failed book check.

   Stop the relay. Keep `g1`.
5. `read_evidence` with `limit` 40, with no relay running. Keep the answer. Record each record's state. It sends nothing to Tally.
6. Start the relay as at step 3 into `<new folder>\g2`.
   - `vouchers` for `2025-04-01` to `2026-03-31`, every page: offset 0, then each next offset the answer names, until it names none.
     Keep each page.
   - Record the vouchers in all, by type, number and date, and whether each carries this GUID's prefix. If g0 read V 3 and M 215, EXPECT
     the three vouchers the Company section names. Otherwise record them as found.
   - From `g2\manifest.tsv`: EXPECT as step 4. Record the count of lines.

   Stop the relay. Keep `g2`.
7. Stop `bridge_mcp`. Record `bridge_mcp.exe`'s sha256 again: EXPECT it equal to step 3's.
8. Wait one minute.
9. **g9.** Marks. EXPECT the g9 answer byte-identical to g0's (compare their sha256). Another answer: stop and report, and run nothing
   more on this book.
10. Shut the company with Tally's own Shut Company. Screenshot: no company loaded.

Steps 7 to 10 send no tool call and are made after a refusal too, so the book is shown unchanged either way.

## What to report
| step | result |
|---|---|
| g0: GUID present, V, M, sha256; Tally's release; the dates | |
| build: zip digest; `bridge_mcp.exe` sha256 at steps 3 and 7 | |
| relay self-test | |
| g1-list: `list_companies`, seconds; manifest lines | |
| g1: `audit_test_run` answer, bytes, sha256, seconds; manifest lines, each with method and bytes; largest answer | |
| g1: if refused, `evidence.state` and the whole `result.error` member; else `result.state`, figures, findings, failed book checks | |
| `read_evidence`: records, their states (or "not run: refused at step 4") | |
| g2: vouchers by type, number and date; pages; manifest lines (or "not run") | |
| g9: V, M, sha256 equal to g0's or not | |

Read every number from its file, not from the screen. Put any stop, any refusal and any EXPECT not met in the first line.

## Decides
| what the row shows | reading on #699 |
|---|---|
| `audit_test_run` answers with a result, every manifest line forwarded, g9 equal to g0 | `g1`'s exchanges become the fixtures of the GST-sale shape in clause 16(a)/(d)'s tests, measured against the synthetic lab book only |
| `audit_test_run` refuses, g9 equal to g0 | the audit read does not admit this book end to end; the code and `g1`'s exchanges are the evidence for that, and the cause gets its own issue |
| a `refused` line in a manifest | the build sent a request the relay does not pass as a read: stop, and nothing from that folder is a fixture |
| g9 not equal to g0 | the book changed during a read-only row: stop and report. No fixture from this row |

## Stop rules
- Any prompt or dialog.
- A `refused` line in any manifest, or a relay that will not start.
- `list_companies` not one company with this GUID, or g0 without this GUID.
- g9 not byte-identical to g0.
- An answer over 2,000,000 bytes.
- Any call with no answer in 10 minutes.
- Educational mode.

A refusal at step 4 is not a stop: it is recorded as step 4 says, and the row closes with steps 7 to 10.

## Code admits the trigger (master 1400175e; the audit read's files are as at d77d0682)
Paths are under `src-tauri/src/`. Each line was read at `1400175e`. Nothing here was built or run for this note.
- **The tool is on and the arguments pass.** These are R1's conditions, unchanged ("Code admits the trigger" in its ROW.md):
  - `cash_44ab` is the one admitted test (`ADMITTED`, agent_audit_run.rs:24);
  - the dates lie within `RULES_YEAR_FROM` and `RULES_YEAR_TO` (agent_audit_run.rs:75-76);
  - redaction is off.
- **The parts read.** `read_audit_book` reads the same parts for every test and every book, in `collect_parts` (agent_audit_read.rs:436
  onward): the opening marks, the company object, the groups, the ledgers, the trial balance, the voucher types, and the vouchers
  through the window reader, whose last request is its closing marks.
- **How a refusal is shaped.** A refused call's payload is `{"error": …}` (agent.rs:3221). The envelope puts it under `result` beside
  `evidence` (agent.rs:3244-3249), and `isError` is true when `result.error` is present (agent.rs:3280).
- **No write.** Posting is off without `BRIDGE_AGENT_ENABLE_WRITES`, and the relay answers any write 403 without forwarding it.

## Limits
- TallyPrime 7.2 Silver, one user, one synthetic GST book of a few vouchers, one run. Other releases, larger books and a client book
  are not covered.
- The relay reads each answer whole before it returns it, so a call's seconds run a little over a direct call.
- No CA has read this book. Anything from it is measured against the synthetic lab book only.

## Evidence and after
Push branch `lab/699-c16g-answers` with:
- `SHA256.txt` and `RESULTS.md` (the table above);
- g0 and g9;
- the relay folders `g1-list`, `g1` and `g2` whole, with every request, answer, header file and manifest;
- every tool answer whole;
- the call times, the build's digest and `bridge_mcp.exe` sha256.

A captured file that holds the company number, or a digit run of 10 or more outside `VOUCHERKEY`, stays on the lab PC, listed by size
and sha256 and the tag that holds the run, as R1's results did. Screens stay on the lab PC, listed by name and sha256. Send the SHA to
the run coordinator before pushing.

Restore: none. The row writes nothing, and g9 shows it.
