# 699-C16G: the audit read of two GST-on books, recorded byte for byte. Row G (BRIDGE EINVOICE LAB) is READ ONLY; row P (BRIDGE CLASS PURCHASE LAB) WRITES one purchase under a backup, reads, and restores.

Not for merge. Part of #699 (clause 16(a)/(d), step 1). Written 4 Nov 2026 at master `1400175e`, and this branch starts there.
`SHA256.txt` lists every file here with its bytes; check it before step 0. Confidence of everything here until the row runs:
unverified. It runs after 699-C16 and does not depend on it: it reads two other books. Row P was added the same day, before any of it
was run. Row G runs first, then row P; row P starts only after row G's step 10.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy of the one on `lab/699-c16-sheet` (sha256
5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6). Where this sheet and LAB-COMMON differ, the stricter rule holds.
Row G writes nothing, so C5 and C6 are not used in it. Row P uses both, as its steps say.

In row G, THE COMPANY is `BRIDGE EINVOICE LAB`: synthetic, GST on, TallyPrime 7.2 Silver, on the lab PC, GUID
`eb2f5190-292e-4ed8-bfd3-eace839fe699`. "This GUID" below means that one. Its last state on record is in `lab/2027-dn2-answers`
RESULTS.md ("The book now", at f748c46e):
- restored to its state before that sitting;
- three vouchers, all dated 3-Apr-25: Sales `1` (with e-invoice details), Sales `2` and a Payment `1`;
- `ALTVCHID` 3 and `ALTMSTID` 215.

Not on the PC, another GUID, or Educational mode: STOP, the row has no book.

Not a client book, and nothing in it is seeded from client data. Its parties, GSTINs and e-invoice details are invented.

In row P, THE COMPANY is `BRIDGE CLASS PURCHASE LAB`: synthetic, GST on, TallyPrime 7.2 Silver, on the lab PC, GUID
`92a7ef97-dd28-4286-b02c-c059fcb8690e`. "This GUID" in row P means that one. Its last state on record is in `lab/2057-b-answers`
RESULTS.md ("Back to the start", at d9005ab3): restored from that row's backup BK3, "No Vouchers Entered", the Gateway's Current Date
1-Apr-2025, marks M 213 and no voucher mark. Its masters as imported: unit Nos, item `Lab Class Item`, ledgers `Lab Purchases`,
`Lab Supplier` (Sundry Creditors), `Lab CGST` and `Lab SGST` (Duties & Taxes), and no voucher class. Not on the PC, another GUID, or
Educational mode: STOP, row P has no book. Its identifiers are synthetic: only those row 2057-B keyed are keyed here.

Only one company is loaded at a time (C1): row G's is shut at its step 10 before row P loads its own.

## Why
A list of candidates for Form 3CD clause 16(a)/(d) is being built in the tax-audit crate. It treats a tax ledger credited on a sale in its
own way. Row 699-C16 can only key such a tax line by hand on a book with GST off. Row G records Tally's own tax lines on a GST-on book,
read by Bridge's own audit read (AGENTS.md P1). It also answers whether that read admits a GST book with e-invoice details at all.
Either answer is a result.

Row P records the other side: a purchase on credit whose input tax Tally itself debits to Duties & Taxes, the shape 699-C16's C19 can
only imitate. Its book holds no voucher today, so row P keys one, by the steps row 2057-B proved on this book on 4 Nov 2026, reads, and
restores the book.

## The one variable
Row G: none is changed. It reads the book as it stands, once, with one build. The book's GST setting is not touched.

Row P: the book gains row 2057-B's GST setup and one Purchase invoice, the same as that row's Purchase 1, and loses them again at the
restore. Nothing else is changed.

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

Record `bridge_mcp.exe`'s sha256 before step 3 and after step 7. Row P starts its own `bridge_mcp` process, with its own new empty
`BRIDGE_AGENT_DATA_DIR` and the same environment, and records the sha256 at P3 and P5.

## Kit
| file | what |
|---|---|
| `LAB-COMMON.md` | the lab's common rules, v3, byte copy |
| `lost_answer_relay.py` | the recording relay R1 and 699-C16 use, byte copy (sha256 `060c3b6c…`). It forwards only GET /status and POST reads to 127.0.0.1:9001, answers anything else 403 without forwarding it, and records every exchange byte for byte as `NNNN-request.bin`, `NNNN-response.bin` and `NNNN-response-headers.txt`, one line each in `manifest.tsv` |
| `templates/suspense_create_request.utf16le.xml` | the template the relay loads at start, byte copy. No call in this row sends a ledger create |
| `requests/marks-einvoice.utf16le.xml` | the marks read of `lab/2027-d2` (`M-marks.xml`, 982 B, sha256 `f8b04aca…`, the one rows 2027-DN and 2027-DN2 sent), byte copy: the loaded company's GUID, ALTVCHID and ALTMSTID, UTF-16LE. Send it with `Content-Type: text/xml; charset=utf-16` |
| `requests/marks-classpurchase.utf16le.xml` | row P's marks read: `requests/marks-einvoice.utf16le.xml` with one change, the company name `BRIDGE CLASS PURCHASE LAB` in place of `BRIDGE EINVOICE LAB` (994 B). Send it as the other |
| `ref/2057-b-RUN-SHEET.md` | row 2057-B's sheet, byte copy of `lab/2057-b` RUN-SHEET.md at 656633d4 (28,168 B). Row P keys by its steps 3 to 10 |
| `.gitattributes` | `* -text`, so no file here is changed on checkout |

Run `python -I lost_answer_relay.py --self-test` before step 3: EXPECT `self-test: ok`.

## Prompts this row expects
Row G: none. Nothing is keyed in Tally except loading the company at step 0 and shutting it at step 10. Any prompt or dialog: press no
key, screenshot, record its words, stop.

Row P: only those its steps name (C5's, C6's, and the one dialog row 2057-B's rule 4 answers). Any other: C7.

## Steps, row G (BRIDGE EINVOICE LAB)
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

## Steps, row P (BRIDGE CLASS PURCHASE LAB)
Row P starts after row G's step 10, with no company loaded. "Marks" here means `requests/marks-classpurchase.utf16le.xml` sent raw to
9001 by curl, never through the relay, recorded as in row G. Each relay folder is new and empty.

**Row 2057-B's steps, used word for word.** Row P's keying is steps 3 to 10 of `lab/2057-b` RUN-SHEET.md at 656633d4, byte-copied here
as `ref/2057-b-RUN-SHEET.md` (in `SHA256.txt`). Those steps, and that sheet's rules 4 (keys and the one dialog), 5 (never
type a tax amount) and its "Identifiers", apply to P2 as written, with three changes:
- its marks are row P's raw marks read, not `ledger_masters`: record M and V after each save, with the rise;
- its step 9 e (a `ledger_masters` read) is not made; 9 a to d are;
- its EXPECTs are recorded, not stops, except where that sheet says STOP.

Its row recorded these marks after each save (`lab/2057-b-answers` RESULTS.md, "The setup"): M 215, 216, 217, 218, 218 (look only),
220 and 221 after steps 3 to 9, V unchanged; and V 1 after its step 10.

P0. Load BRIDGE CLASS PURCHASE LAB alone (C1). Screenshot. `GET /status` on 9001. Record Tally's release and the Gateway's Current Date
    and "No Vouchers Entered" or not. **p0.** Marks: EXPECT one COMPANY row with this GUID. Record V and M (last on record: M 213, no
    V). Keep the answer as `p0-marks.xml`.
P1. **Backup BK-C16G** of THE COMPANY alone by C5, into a new empty local folder `BK-C16G`. Record the file's name, bytes and sha256
    and the time the Restore list will show. Then the marks: EXPECT V and M equal to p0's.
P2. **The keying:** row 2057-B's steps 3 to 9, then its step 10 (I1-B, Purchase 1 of 10-Jun-25 through `Lab Class`, `Lab Class Item`
    5 Nos @ 100.00), as set out above. Record, for I1-B, the mode it opened in and every line and amount on Tally's screen before its
    Accept, with a screenshot. Row 2057-B recorded Lab CGST 45.00, Lab SGST 45.00 and total 590.00 on that screen; what this screen
    shows is recorded and accepted as it stands (that sheet's rule 5). Row 2057-B's steps 11 to 13 are not keyed. Then the marks:
    V +1 after the Accept.
P3. Run the relay's self-test (Kit). Start the relay into `<new folder>\p1-list` with `--company "BRIDGE CLASS PURCHASE LAB"
    --ledger-name "Unused 699 C16P"`, and start a new `bridge_mcp` pointed at 9011 (Build). Record `bridge_mcp.exe`'s sha256.
    - The client's tools list: record that `audit_test_run` is in it.
    - `list_companies`: EXPECT one open company, this GUID. Anything else: stop.

    Stop the relay. Keep `p1-list`.
P4. **The marks before the read** (P4 marks), then start the relay into `<new folder>\p1`.
    - `audit_test_run`, once, with row G's arguments and this GUID. Keep the whole answer as a file, with its size and sha256.
    - **A refusal is an answer.** If `isError` is true, record `structuredContent.evidence.state` and the whole
      `structuredContent.result.error` member, code and message as written. No repeat is made, whatever the code. Go on to P5.
    - EXPECT from `p1\manifest.tsv`: as row G's step 4. Record the lines, each with method and bytes.
    - Record (not predicted), if it answers with a result: `result.state`, each figure and finding id with its value, and the code and
      count of every failed book check.

    Stop the relay. Keep `p1`. Then the marks: EXPECT equal to the P4 marks (a read changes nothing).
P5. Stop `bridge_mcp`. Record `bridge_mcp.exe`'s sha256: EXPECT it equal to P3's.
P6. **The restore of BK-C16G**, by C6, on the run coordinator's word RESTORE, which the coordinator gives once `p1` is kept. Before
    it, check the backup's sha256 against P1's. After it, load THE COMPANY alone and take the screenshots C6 names.
P7. Wait one minute. **p9.** Marks: EXPECT V and M equal to p0's, and record whether the answer is byte-identical to p0's (compare
    their sha256). The Gateway: EXPECT "No Vouchers Entered". Another V or M: stop and report. The fallback is row 2057-B's BK3,
    restored only on the run coordinator's word.
P8. Shut the company with Tally's own Shut Company. Screenshot: no company loaded.

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
| p0: GUID present, V, M, sha256; Tally's release; the Gateway | |
| P1: BK-C16G's name, bytes, sha256, time; marks | |
| P2: each save's marks and rise; 9 a to d as that sheet asks; I1-B's mode, lines and total on screen; V after | |
| p1-list and p1: as g1-list and g1, with the P4 marks before and after | |
| P5: `bridge_mcp.exe` sha256 at P3 and P5 | |
| P6: the restore as C6 records it | |
| p9: V, M, sha256 equal to p0's or not; the Gateway | |

Read every number from its file, not from the screen. Put any stop, any refusal and any EXPECT not met in the first line.

## Decides
| what the row shows | reading on #699 |
|---|---|
| `audit_test_run` answers with a result, every manifest line forwarded, g9 equal to g0 | `g1`'s exchanges become the fixtures of the GST-sale shape in clause 16(a)/(d)'s tests, measured against the synthetic lab book only |
| `audit_test_run` refuses, g9 equal to g0 | the audit read does not admit this book end to end; the code and `g1`'s exchanges are the evidence for that, and the cause gets its own issue |
| a `refused` line in a manifest | the build sent a request the relay does not pass as a read: stop, and nothing from that folder is a fixture |
| g9 not equal to g0 | the book changed during a read-only row: stop and report. No fixture from this row |
| row P: `audit_test_run` answers with a result, every manifest line forwarded, the P4 marks equal before and after, p9 equal to p0 | `p1`'s exchanges become the fixtures of the input-tax purchase shape, measured against the synthetic lab book only |
| row P: `audit_test_run` refuses | as row G's refusal, for this book |
| row P: I1-B's screen without a tax amount on a head | recorded as it stands; `p1` holds the purchase as Tally stored it, and the shape is named for what it holds |
| p9 not equal to p0 | the restore did not bring the book back: stop and report; the run coordinator decides on BK3 |

## Stop rules
- Any prompt or dialog in row G; in row P, any not its steps name (C7).
- Every STOP of row 2057-B's sheet in the steps P2 uses.
- A `refused` line in any manifest, or a relay that will not start.
- `list_companies` not one company with this GUID, or g0 without this GUID.
- g9 not byte-identical to g0; p9's V or M not equal to p0's.
- The P4 marks not equal before and after the read.
- An answer over 2,000,000 bytes.
- Any call with no answer in 10 minutes.
- Educational mode.

A refusal at step 4 is not a stop: it is recorded as step 4 says, and the row closes with steps 7 to 10. The same holds for P4: row P
goes on to its restore.

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
- TallyPrime 7.2 Silver, one user, two synthetic GST books of a few vouchers, one run each; row P's purchase is intra-state, one
  invoice. Other releases, larger books and a client book
  are not covered.
- The relay reads each answer whole before it returns it, so a call's seconds run a little over a direct call.
- No CA has read this book. Anything from it is measured against the synthetic lab book only.

## Evidence and after
Push branch `lab/699-c16g-answers` with:
- `SHA256.txt` and `RESULTS.md` (the table above);
- g0 and g9;
- p0, p9 and every row P marks answer, the BK-C16G record, and row P's screens list;
- the relay folders `g1-list`, `g1`, `g2`, `p1-list` and `p1` whole, with every request, answer, header file and manifest;
- every tool answer whole;
- the call times, the build's digest and `bridge_mcp.exe` sha256.

A captured file that holds the company number, or a digit run of 10 or more outside `VOUCHERKEY`, stays on the lab PC, listed by size
and sha256 and the tag that holds the run, as R1's results did. Screens stay on the lab PC, listed by name and sha256. Send the SHA to
the run coordinator before pushing.

Restore: none in row G, which writes nothing, as g9 shows. Row P restores BK-C16G at P6, and p9 shows it.
