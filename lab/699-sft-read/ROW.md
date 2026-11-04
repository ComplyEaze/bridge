# Row 699-R1: the audit read of BRIDGE SFT LAB, sent by the build's own read path and recorded byte for byte. READ ONLY.

Not for merge. Part of #699 (clause 42, step 1). Lines below are at d77d0682 (master, the build's head), and this branch starts there. Confidence of everything here until the row runs: unverified.

## Question
What does Tally answer, byte for byte, to each request ComplyEaze Bridge's audit read sends for BRIDGE SFT LAB over FY 2025-26, and does that read admit the book end to end?

## Why
Clause 42 (Form 61A row SFT-013) is built in the tax-audit crate first, as a declared divergence, and its tests need this book's read as fixtures captured from Tally (AGENTS.md P1), not written by hand. The book was keyed on 26 Oct 2026 (`lab/699-sft-lab-answers`), but its read was never run: that run's review did not admit builder-rendered requests sent by curl. Here every request to Tally except the two marks reads comes from the build, through a relay that records it and refuses anything that is not a read. The captures this row keeps become those fixtures. This row checks nothing against a CA: no CA key exists yet.

## Book
`BRIDGE SFT LAB`, synthetic, TallyPrime 7.2 Silver, on the lab PC, loaded alone. Company number 100079, books from 1-Apr-2025, and the GUID that `lab/699-sft-lab-answers` RESULTS.md:6 records and `requests/marks-sft.utf16le.xml` here filters on; "this GUID" below means that one. It holds 34 vouchers keyed by hand (13 Sales, 19 Receipts, 1 Payment, 1 Contra; A5-VOUCHERS.md) and 13 ledgers, among them `Cash` under Cash-in-Hand and `Bank SFT` under Bank Accounts. Its last marks on record are V (ALTVCHID) 34 and M (ALTMSTID) 219, in `answers/a4-marks-after-vouchers.xml` of that branch (4,888 B).

Not a client book, and nothing in it is seeded from client data. Not on the PC, another GUID, or Education mode: stop, the row has no book. Any prompt when it opens: press no key, screenshot, stop and report its words.

The row writes nothing, so it takes no backup and makes no restore. The book is left as it was found, and step 9 proves it.

## The one variable
None is changed. The row reads the book as it stands, once, with one build.

## Build
The Windows bundle of CI run 41770250126 (the push of master at `d77d0682040ee29470e890b7a280ffcf264c5bae`). Artifact `unsigned-bundle-smoke-windows-latest`, id 13295026003, digest `sha256:bd7e796174a185b1523258c3052acad67c29a7d8b2e68d99d5b12d6ce60cc2bf`. Its Windows bundle smoke passed. Master has moved since, but the files of the audit read (agent_audit_read.rs, agent_audit_run.rs, agent.rs, agent_catalog.rs) are unchanged from d77d0682 to master at 91059155. Check the zip's digest first: another digest is a stop. Record `bridge_mcp.exe`'s sha256 before step 3 and after step 7.

Over stdio, through the relay: `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true`, `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9011`, one new empty `BRIDGE_AGENT_DATA_DIR` for the whole row. `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_BATCH_POST` and `BRIDGE_AGENT_REDACTION` are absent. One `bridge_mcp` process for the whole row. Record the client and its call limit. A call over 60 s is recorded, not a stop.

## Kit
| file | what |
|---|---|
| `lost_answer_relay.py` | the recording relay of `lab/1650-masters-lost-answer`, unchanged (sha256 there and here: `060c3b6c…`). It listens on 127.0.0.1, forwards to 127.0.0.1:9001 only, and writes no request of its own. It forwards only GET /status and POST reads (one TALLYREQUEST, Export, no import marker, no ACTION attribute) and answers anything else 403 without forwarding it. It records every request and answer byte for byte as `NNNN-request.bin`, `NNNN-response.bin` and `NNNN-response-headers.txt`, with one line per request in `manifest.tsv` |
| `templates/suspense_create_request.utf16le.xml` | the template the relay loads at start, unchanged. Its withholding applies only to a ledger create, which no call in this row sends: posting is off and the relay refuses every write |
| `requests/marks-sft.utf16le.xml` | the marks read of `lab/699-sft-lab-answers` (`requests/c1-marks-sft.utf16le.xml`, 1,636 B), unchanged: this company's GUID, ALTVCHID and ALTMSTID, UTF-16LE. Send it with `Content-Type: text/xml; charset=utf-16` |
| `.gitattributes` | `* -text`, so no file here is changed on checkout |

`SHA256.txt` lists all of it. Check it first. Run `python -I lost_answer_relay.py --self-test` before step 3: EXPECT `self-test: ok`. It passed here on macOS.

## Prompts this row expects
None. Nothing is keyed in Tally except loading the company at step 0 and shutting it at step 10. Any prompt or dialog: press no key, screenshot, record its words, stop.

## Steps
One at a time. Every answer goes to a file, whole. A file with no answer in it is a failed step, not an empty one. "Marks" means `requests/marks-sft.utf16le.xml` sent raw to 9001 by curl, never through the relay. Write down V and M and keep the answer with its sha256. Time every tool call: its start and end from the client's log, in seconds. Each relay folder is new and empty.

0. Check `SHA256.txt`. Load BRIDGE SFT LAB alone. Screenshot. `GET /status` on 9001. Record Tally's release, Tally's date and the PC's date.
1. **s1.** Marks. EXPECT one COMPANY row with this GUID, V 34 and M 219. Another V or M is a stop: the book is not as keyed. Keep the answer as `s1-marks.xml`. Record whether its sha256 equals that of `answers/a4-marks-after-vouchers.xml` on `lab/699-sft-lab-answers`; a difference is recorded, not a stop.
2. Run the relay's self-test (Kit).
3. Start the relay: `python -I lost_answer_relay.py --listen-port 9011 --record <new folder>\r1 --company "BRIDGE SFT LAB" --ledger-name "Unused 699 R1"`. Then start `bridge_mcp` pointed at 9011.
   - The client's tools list: record that `audit_test_run` is in it.
   - `list_companies`: EXPECT one open company, this GUID. Anything else: stop.
   Stop the relay with Ctrl+C. Keep `r1`.
4. Start the relay as at step 3 into `<new folder>\r2`.
   - `audit_test_run`, once: `company_guid` this GUID, `from` `2025-04-01`, `to` `2026-03-31`, `test` `cash_44ab`, `cash_groups` [`Cash-in-Hand`], `bank_groups` [`Bank Accounts`]. Keep the whole answer as a file, with its size and sha256.
   - A refusal: record its code. Only `C6-moved` or `voucher_window_changed_during_read` may be called once more, into a new folder `r2b` after step 1's marks again. `audit_part_deadline_exceeded` is a stop: its own text says not to retry. Any other refusal is a stop.
   - EXPECT from `r2\manifest.tsv`, read by its columns (never by a substring search): every line's verdict `forwarded`, HTTP status 200 and answer_returned `yes`; no `refused` line and no `events.txt`. Record the number of lines: it counts the requests the build sent to Tally for this one call. Record each line's method, request bytes and response bytes. Every answer at most 2,000,000 bytes; a larger one is kept and is a stop.
   - EXPECT (recorded, not predicted): `result.state`, each figure and finding id with its value, and the code and count of every failed book check.
   Stop the relay. Keep `r2`.
5. `read_evidence` with `limit` 40, with no relay running. Keep the answer. EXPECT no record in a state other than complete. It sends nothing to Tally.
6. Start the relay as at step 3 into `<new folder>\r3`.
   - `vouchers` for `2025-04-01` to `2026-03-31`, every page: offset 0, then each next offset the answer names, until it names none. Keep each page. EXPECT 34 vouchers in all, each with this GUID's prefix: 13 Sales, 19 Receipt, 1 Payment and 1 Contra, on the dates A5-VOUCHERS.md records.
   - From `r3\manifest.tsv`: EXPECT as step 4. Record the count of lines.
   Stop the relay. Keep `r3`.
7. Stop `bridge_mcp`. Record `bridge_mcp.exe`'s sha256 again: EXPECT it equal to step 3's.
8. Wait one minute.
9. **s9.** Marks. EXPECT the s9 answer byte-identical to s1's (compare their sha256): V 34, M 219. Another answer: stop and report, and run nothing more on this book.
10. Shut the company with Tally's own Shut Company. Screenshot: no company loaded.

No one keys in Tally except at steps 0 and 10.

## What to report
| step | result |
|---|---|
| s1: V, M, sha256; equal to the 26 Oct a4 answer or not; Tally's release; the dates | |
| build: zip digest; `bridge_mcp.exe` sha256 at steps 3 and 7 | |
| relay self-test | |
| r1: `list_companies`, seconds; manifest lines | |
| r2: `audit_test_run` answer, seconds; refusal code if any; manifest lines, each with method and bytes; largest answer | |
| r2: `result.state`, figures, findings, failed book checks | |
| `read_evidence`: records, their states | |
| r3: vouchers by type and in all; pages; manifest lines | |
| s9: V, M, sha256 equal to s1's or not | |

Read every number from its file, not from the screen. Put any refusal, and any EXPECT not met, in the first line.

## Decides
| what the row shows | reading on #699 |
|---|---|
| r2 holds every answer, `audit_test_run` answers with a result, `read_evidence` all complete, r3 counts 34, s9 equal to s1 | `r2`'s exchanges become the fixtures of clause 42's tests, measured against the synthetic lab book only |
| a refusal in step 4 | record the code. The read of this book does not admit it end to end; code waits for its cause |
| a `refused` line in a manifest | the build sent a request the relay does not pass as a read: stop, and nothing from that folder is a fixture |
| r3 not 34, or the wrong types | the book is not as keyed: no fixture from this row |
| s9 not equal to s1 | the book changed during a read-only row: stop and report. No fixture from this row |

Nothing is sent again after a refusal, other than the one repeat step 4 names.

## Stop rules
- Any refusal other than the one repeat step 4 names.
- Any prompt or dialog.
- A `refused` line in any manifest, or a relay that will not start.
- V or M not as EXPECTed at s1 or s9.
- An answer over 2,000,000 bytes.
- Any call with no answer in 10 minutes.
- Education mode.

## Code admits the trigger (d77d0682, the build's head)
Paths are under `src-tauri/src/`. Each line was read at d77d0682. Nothing here was built or run for this note.
- **The tool is on:** `BRIDGE_AGENT_ENABLE_TAX_AUDIT` is read at agent.rs:679 and :684. With it off, the tools list leaves `audit_test_run` out (agent_catalog.rs:333) and a call is refused (agent.rs:3349).
- **The arguments pass:** redaction is off, else `audit_redaction_unsupported` (agent_audit_run.rs:80-81). The dates lie within 2025-04-01 to 2026-03-31 (`RULES_YEAR_FROM`, `RULES_YEAR_TO`, agent_audit_run.rs:75-76, checked at :89). `cash_44ab` is the one admitted test (`ADMITTED`, agent_audit_run.rs:24). `Cash-in-Hand` and `Bank Accounts` do not overlap, else `audit_groups_overlap` (:101-107).
- **No write:** posting is off without `BRIDGE_AGENT_ENABLE_WRITES` (read at agent.rs:704), and `post_import` then refuses `import_posting_disabled` (agent.rs:3339). No call in this row is a write, and the relay answers any write 403 without forwarding it.
- **The read:** `audit_test_run` verifies the company (`verified_company`, called at agent_audit_run.rs:108, defined at agent_company.rs:85), then calls `read_audit_book` (agent_audit_run.rs:122, defined at agent_audit_read.rs:371). That reads, in order (`collect_parts`, agent_audit_read.rs:436): the opening marks (:450), the company object (:468), the groups (:484), the ledgers (:498), the trial balance (:507), the voucher types (:517), and the vouchers through the window reader (:532-533), whose census requests are made but not kept as parts, and whose last request is its closing marks (agent_audit_read.rs:608-613). Each part goes through `fetch` (:175) to `fetch_audit_part`, the one dispatch path.
- **The repeat:** the tool's own text admits one repeat for `C6-moved` or `voucher_window_changed_during_read`, and none for `audit_part_deadline_exceeded` (agent_catalog.rs:661). The read marks a moved bracket as worth repeating (`C6-moved`, agent_audit_read.rs:127). Its other repeat, `tally_endpoint_draining`, is a stop in this row.

## Who sends each EXPECT's request, and what each count counts (d77d0682)
| step | request | sent by | decided by | the count |
|---|---|---|---|---|
| 1, 9 | marks, raw | the operator, by curl on 9001, not Bridge | the step itself | one COMPANY row. V is that company's voucher mark (ALTVCHID), M its master mark (ALTMSTID): equal at s1 and s9 means no voucher or master changed |
| 3 | the company list | `list_companies`, agent.rs:3380 | always sent by that call | one company with this GUID |
| 4 | the audit read | `audit_test_run`, agent.rs:3366, then agent_audit_run.rs:79; the parts as under "The read" | the call itself, once the arguments pass (agent_audit_run.rs:80-107) | the lines of `r2\manifest.tsv` count the requests Bridge sent for the call, one line each |
| 5 | none | `read_evidence`, agent.rs:3413, defined at :3462, reads the build's memory only | the call itself | the records count the requests the build made since it started, at most 40 |
| 6 | the voucher pages | `vouchers`, agent.rs:3436, then agent_vouchers.rs:9 | the call itself | VOUCHER rows with this GUID's prefix: 34 |

## Limits
- TallyPrime 7.2 Silver, one user, one synthetic book of 34 vouchers, one run. 7.1 and larger books are not covered.
- The relay reads each answer whole before it returns it, so a call's seconds run a little over a direct call.
- The book has no PAN, no GSTIN, no bill-wise detail but one party's, and no CA's reading. Fixtures from it are measured against the synthetic lab book only.

## Keep
On `lab/699-sft-read-answers`, with `SHA256.txt`: `r1`, `r2` (and `r2b` if run) and `r3` whole, with every request, answer, header file and manifest; every tool answer whole; s1 and s9; the call times; the build's digest and `bridge_mcp.exe` sha256; the screens by name and hash. Screens stay local. A `RESULTS.md` holding the table above. Send the sha.
