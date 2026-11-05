# Row 1392-E: skipped statement rows judged against a live book, from generated statements: `not_found`, a hand-keyed decoy, a pair by reference and (if that holds) a pair by date

Not for merge. Part of #1392. Written 5 Nov 2026. Lines below are at 187e882b, the build's head, and this branch starts there. Confidence of everything here until the row runs: unverified.

## Question
#1392's transfer pairing has never seen a skipped statement row. Every 1392 row so far sent fixed reads and `tally_status` only, and none parsed a statement. A skip exists only from a statement PDF read by `parse_bank_statement` (Code, 1). This row asks whether the four outcomes the design rests on come out of a real build on a real book:
- **E1:** a skipped row whose other account's statement covers its date, and whose day reads find nothing on the other bank ledger, reads `not_found`;
- **E2:** a Contra keyed by hand in Tally on that day turns it into `tally_unchecked`, and is never paired;
- **E3:** a verified Contra built from the other account's statement, with the same IMPS reference, pairs the row (`skips_paired` 1);
- **E4, conditional:** a row with no reference pairs by date with such a Contra when both statements cover the day.

## What this row does not show
**The parser's fit to real bank PDFs is NOT shown by this row.** The statements are written by the fixture generator restored below. They carry its HDFC geometry and an invented layout of rows, so the parse can only succeed on shapes the parser was written for. What is measured is Tally's side: the build, the post, the day reads and the pairing against a live book. Whether real HDFC statements print a transfer's IMPS reference on both sides is a separate question on the owner's list.

## The generator
`generate-bank-statement-fixtures.py` is a byte copy of `scripts/generate-bank-statement-fixtures.py` at 30f9c6c5, the parent of c9e0ba21 (#1783), which removed it from master: blob d401ef8b, sha256 10562675f40db69118bed4566c5c3bcfd48aa0846ad3a3775f92af607e6cf619, 681 lines, standard library only. It is on this lab branch only and is not restored on master. `make_statements.py` imports it and writes nothing of its own but the rows. Every account digit, reference, name and amount is invented.

## Book
`BRIDGE TRANSFER LAB`, GUID begins `d6e4c24f` (whole on the lab PC and in the earlier 1392 sheets), synthetic, in the synthetic-company registry. TallyPrime 7.2 Silver, licensed. Ledgers `TLab Bank A`, `TLab Bank B`, `Cash`, `TLab Clearing` (under Suspense A/c) and `Profit & Loss A/c`. Its vouchers are the earlier 1392 rows' keying, all dated in April 2026. Last marks, row 1392-spanned-census (3 Nov, `lab/1392-spanned-census-answers`): **V 12, M 211**. It was left shut.

No day this row writes or reads (20 Oct to 3 Nov 2026) holds a voucher of the book's own.

Not on the PC, or another GUID: stop, the row has no book; never rebuild it. Any prompt when it opens: press no key, screenshot, stop and report its words. Education mode: stop.

## Rules
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy of the one on `lab/1029-p-presence` at 2a645696, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. THE COMPANY is BRIDGE TRANSFER LAB.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Writes.** Five posts through Bridge's `post_import` (steps 4, 5, 8, 9 and 10), one hand-keyed Contra (step 6), BK-E (step 1) and its restore (step 12). Nothing else is written. No raw import is sent.
2. **Who acts on the box.** The lab answers Bridge's approval box itself, Yes, by mouse on the box's own button, as in rows 852-C1 and 1029-R. Before the click, the box-check helper of 852-C1 (`lab/852-capture-sheet`) reads the box's text and allows Yes only if it names this company, its GUID, this batch's id and this batch's voucher count. A box that names anything else, or that cannot be read: Decline, record its text, and stop.
3. **Nobody keys in Tally** from step 2 to step 11, except the one Contra of step 6.

## Build
BUILD-M, the one already staged for rows 2161-C and 1029-R:
- the `unsigned-bundle-smoke-windows-latest` artifact of CI run 41772223948, the merge-queue run of #2329 at 187e882b5176a3b577d57f257a6ec4319fc2aecd;
- artifact id 13295846440, 59,336,955 bytes, GitHub digest `sha256:1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a`, expiring 11 Nov 2026.

Check the zip's sha256 against the digest, and `bridge_mcp.exe` against `mcpb-smoke.json`, before the first call. Either differing is a stop. Record the exe's sha256 before and after. `parse_bank_statement` reads PDFs with the PDFium the bundle ships; a refusal that names PDFium is a stop.

Over stdio, directly on 9001, no relay: `BRIDGE_AGENT_ENABLE_WRITES=true`, `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, one new empty `BRIDGE_AGENT_DATA_DIR` for the whole row, `BRIDGE_AGENT_ENABLE_BATCH_POST` absent, `BRIDGE_AGENT_REDACTION` unset. One `bridge_mcp` process for the whole row, so the import journal and the skip records carry from each call to the next. The client is the lab's stdio client of 852-C1 and 1029-R: no cancel and no call limit.

## Kit
| file | what |
|---|---|
| `generate-bank-statement-fixtures.py` | the byte copy above |
| `make_statements.py` | `python -I make_statements.py C:\bridge-lab\1392e\pdf` writes the seven statements (the folder new or empty); same bytes on every run. `python -I make_statements.py --calls` prints each statement's `parse_bank_statement` arguments: paths, `account_label`, the four controls, `bank_ledger`, `suspense_ledger` `TLab Clearing` and `mapping`. |
| `statements.sha256` | the seven PDFs' sha256, in `sha256sum -c` form. Check them after writing; any differing is a stop. |
| `batches/f.json` | the `vouchers` array of step 3: three numberless Receipts, `Cash` Dr and `TLab Clearing` Cr, on 22, 23 and 24 Oct 2026, of 101.00, 102.00 and 103.00. |
| `marks.utf16le.xml` | the company's GUID, ALTVCHID and ALTMSTID: a byte copy of `m.utf16le.xml` of `lab/1392-spanned-census-sheet` (862cbd39). Send it with `Content-Type: text/xml; charset=utf-16`. |
| `LAB-COMMON.md` | v3, above |

`SHA256.txt` lists every other file here with its sha256. Check it first.

**The password file.** `C:\bridge-lab\1392e\pw.txt` holds the one line `lab-1392e-user` (`PASSWORD` in `make_statements.py`). A refusal `statement_password_file_permissions` is followed once by its own next step, then the same call is made again; a second refusal is a stop.

**The statements.** Two accounts: A (`TLab A xx7701`, `TLab Bank A`) and B (`TLab B xx7702`, `TLab Bank B`). Every transfer goes from A to B: a withdrawal on A's statement, a deposit on B's. The transfer line's party reads `TLAB SELF XFER`; on A's statements it is mapped `skip` to `TLab Bank B`, on B's `contra` to `TLab Bank A`. Filler lines read `TLAB FILLER` and are left unmapped, so they post to `TLab Clearing` as Receipts.

| file | rows (date, line) | reference on the transfer line | what it is for |
|---|---|---|---|
| `e1392-b1.pdf` | 20 Oct filler 1,201.00; 28 Oct filler 1,202.00 | no transfer line | B's statement, covering 21 to 27 Oct once verified |
| `e1392-a1.pdf` | 23 Oct transfer 5,001.00 | IMPS R1 | E1's skip |
| `e1392-a2.pdf` | 23 Oct transfer 5,002.00 | IMPS R2 | E2's skip, after the decoy |
| `e1392-b3.pdf` | 29 Oct transfer 7,003.00 | IMPS R3 | E3's Contra |
| `e1392-a3.pdf` | 29 Oct transfer 7,003.00 | IMPS R3 | E3's skip |
| `e1392-b4.pdf` | 1 Nov filler 1,204.00; 2 Nov transfer 9,004.00; 3 Nov filler 1,205.00 | IMPS R4 | E4's Contra, covering 2 Nov |
| `e1392-a4.pdf` | 1 Nov filler 1,206.00; 2 Nov transfer 9,004.00; 3 Nov filler 1,207.00 | none (a TPT line) | E4's skip, its statement covering 2 Nov |

R1 to R4 are four distinct twelve-digit numbers, printed in the narration as IMPS lines. Each statement was read locally, before this sheet was sent, by the Python parser removed in c9e0ba21 (the reference the Rust parser was ported from and compared against): every row, the account line, the balance chain, both totals, the two party names and the four references came out as this table says. That is a check of the files, not of the build's parser: step 4 onward is.

## Steps
One at a time. Every answer goes to a file. A file with no answer is a failed step, not an empty one.

0. **Load and check.** Check `SHA256.txt`. LAB-COMMON step 0 with THE COMPANY loaded alone. **m0**: marks, EXPECT one COMPANY row, this GUID, **V 12 and M 211**. Other marks: recorded and reported before step 1, not a stop. Record Tally's release, Tally's date and the PC's date; the PC's date must be on or after 3 Nov 2026.
1. **BK-E** (C5), into the new empty folder `C:\bridge-lab\1392e\backups\bke\`. Record its name, bytes, sha256 and the time the Restore list will show. Then marks **m0b**.
2. **Kit and Bridge.**
   - Write the statements (Kit) and check `statements.sha256`. Write `pw.txt`.
   - Start `bridge_mcp` as above. `list_companies`: EXPECT this GUID; record `books_from`, which must be on or before 20 Oct 2026. `tally_status`: record release, licence and mode.
   - `masters` with `kind=voucher_types`: record the Payment, Receipt and Contra rows' `series_numbering.method` and `sub_method`. EXPECT automatic and auto_retain for all three. Any other: stop. This row alters no type.
   - Marks: EXPECT as m0b.
3. **F, the day fillers.** `build_import_xml` with `{"company_guid": <GUID>, "vouchers": <batches/f.json>}`, then post and check it as **P** below. EXPECT V +3. These three vouchers put one voucher on each day E1's reads cover, none on a bank ledger.
4. **B1.** `parse_bank_statement` with `--calls`' `b1` arguments. EXPECT `reconciled`; counterparties `TLAB FILLER` 2 rows, both to suspense; `skipped` 0. Then `build_import_xml` with the returned `proposals_id` and `sha256` as `proposals_id` and `proposals_sha256`, and P. EXPECT V +2.
5. **E1.** `parse_bank_statement` with `a1`'s arguments. EXPECT `reconciled`; one row, `TLAB SELF XFER`, `skipped` 1. `build_import_xml` with its proposals. EXPECT `result.state` `skips_recorded`, `skipped_rows` 1, nothing sent to Tally but reads, and in `result.transfers` this row's state **`not_found`**. Keep the whole answer, and `egress_log` after it, whole. Marks: EXPECT V unchanged.
6. **The decoy.** On Tally's screen (as the 2 Nov keying: `To` the credited ledger, `By` the debited one), one Contra: 23 Oct 2026, `By` `TLab Bank B`, `To` `TLab Bank A`, 5,002.00, no narration and no reference, the date set with F2. On a bank-allocation screen accept what Tally offers and type nothing. Screenshot the saved voucher. Marks: EXPECT V +1.
7. **E2.** `parse_bank_statement` with `a2`'s arguments, then `build_import_xml` with its proposals. EXPECT `skips_recorded`, `skipped_rows` 1, and in `result.transfers` both 23 Oct rows (E1's and E2's) **`tally_unchecked`**, neither paired, and `skips_paired` 0. Keep the answer and `egress_log`. Marks: EXPECT V unchanged.
8. **B3.** `parse_bank_statement` with `b3`'s arguments. EXPECT one row, `TLAB SELF XFER`, a Contra, `skipped` 0. Build, then P. EXPECT V +1.
9. **E3.** `parse_bank_statement` with `a3`'s arguments, then `build_import_xml`. EXPECT `skips_recorded`, `skipped_rows` 1, and in `result.transfers` **`skips_paired` 1**, with E3's row not among the open rows. E1's and E2's rows are listed as the view finds them; record them, they decide nothing here. Marks: EXPECT V unchanged.
   - **E4 runs only if step 9 gave `skips_paired` 1** (What the result decides). Otherwise go to step 11.
10. **E4.**
    a. **B4.** `parse_bank_statement` with `b4`'s arguments: three rows, the transfer a Contra. Build, then P. EXPECT V +3.
    b. **A4.** `parse_bank_statement` with `a4`'s arguments: three rows, `skipped` 1, the transfer line with no reference. Build. EXPECT a batch of 2 Receipts, and in `result.transfers` A4's row **`matched_unconfirmed`** (A's statement is not yet verified). Then P. EXPECT V +2. In P's `verify_import` answer, `result.transfers`: EXPECT **`skips_paired` 2**, with A4's row not among the open rows.
11. **End of writes.** `egress_log`, whole. Marks **m3**: record. EXPECT V = m0b's V + 7, or + 12 after E4. Stop `bridge_mcp`.
12. **Restore BK-E** (C6). Then marks **m4**. EXPECT the COMPANY row's GUID, V and M equal to m0b's. Anything else = stop.
13. LAB-COMMON C10: shut the company, one tally.exe on 9001, no company loaded.

**P, the post and check of one built batch** (1029-R's step 3b to 3f):
- `post_import` with `company_guid` and the batch id. Record each call's start and end as UTC with milliseconds, when the box showed, and when Yes was clicked. A result with `approval.state` `pending`: call again after its `retry_after_s`. `approved` without a post: call again at once. Never call again after an attempt is recorded. Keep each result whole.
- Screenshot the whole Tally window: the Gateway, no dialog.
- If the post's result names `retry_after_s` before `verify_import`, wait it out. Then `verify_import` with the batch id: the whole answer, its `transfers` section with it. EXPECT `posted_verified` for every voucher of the batch.
- Marks: EXPECT V up by the batch's count and M unchanged. Otherwise: report before the next step.

## What the result decides
| step | result | reading on #1392 |
|---|---|---|
| 5 | `not_found` | **E1 holds:** the whole chain (covered by a verified statement of the other account, three whole days read, no entry on the other ledger) gives `not_found` on a live book. |
| 5 | `tally_unchecked` | the day reads did not rule the Contra out: record `egress_log`'s day lines whole; a finding, before step 6. |
| 5 | `not_established` | B1's window did not count as covering 23 Oct: a finding, before step 6. |
| 7 | both `tally_unchecked`, `skips_paired` 0 | **E2 holds:** a Contra keyed in Tally is seen by the day reads and blocks `not_found`, and is never taken as a pair. |
| 7 | either `not_found`, or a pair | **A finding:** the reads missed an entry on the other ledger. Stop and report before step 8. |
| 9 | `skips_paired` 1 | **E3 holds, and the reference decided it.** A3's statement has one row, so it covers no day (proposals.rs:262), and both lines carry a reference: round 2 neither applies to them nor could confirm a pair without A's cover (Code, 4). E4 runs. |
| 9 | E3's row open (`reference_differs`, `not_established`, `tally_unchecked` or other) | **E3 does not hold:** the reference was not used. E4 is moot and does not run. Recorded whole. |
| 10b | `skips_paired` 2 in P's `verify_import` | **E4 holds:** with both statements verified around 2 Nov, a row with no reference pairs by date. |
| 10b | A4's row `matched_unconfirmed` after verify | a window fell short: recorded with both statements' coverage, not a stop. |
| any | a refusal before a write, or a stop rule | recorded whole; nothing after it runs. |

## Stop rules
- Any dialog on Tally's screen, before or after a call.
- Any refusal before an attempt is recorded, any parse refusal other than the one password-file retry, and any refusal of a build.
- Any call with no answer in 10 minutes.
- A box the helper does not pass.
- A type in step 2 that is not automatic with retain.
- A statement whose sha256 differs from `statements.sha256`.
- V not as expected after a step: report before the next step.
- A mark that moves outside the steps that change it.
- Education mode.

After a stop, nothing more is sent or keyed (C8). The book then holds whatever was posted or keyed before it: F's three Receipts from step 3, B1's two from step 4, the decoy from step 6, B3's Contra from step 8, B4's and A4's vouchers from step 10. The way back is BK-E, restored on the run coordinator's word.

## Limits
- One generated statement layout (HDFC), one book, one sitting, k = 0 (`TRANSFER_K_DAYS`, agent_transfer_pairing.rs:276): only the same printed day pairs by date.
- A statement with one row covers no day. That is why E3's pair can only be by reference, and why E1's skip needs B1's two rows either side.
- The decoy is one Contra on the skip's own day. A decoy one day away, or on another ledger, is not tried.
- Not shown: the parser's fit to real statements (above); `withdraw_skips`; `reversed_same_day`; `double_booked`; a Contra keyed by hand on the other side of a pair.

## Code admits the trigger (187e882b, the build's head)
1. **A skip comes only from a statement PDF.** `parse_bank_statement` (agent.rs:3399) reads an hdfc PDF (agent_bank_statement.rs:80); `skip` is a mapping treatment (:97) whose ledger must be the other account's (proposals.rs, `skip_ledger_is_statement_bank` and `skip_ledger_is_suspense`).
2. **A statement of only skipped rows is recorded, not built** (agent_import.rs:1231-1238), as `skips_recorded` (:2077), with the transfers view computed after the day reads of its `tally_unchecked` rows (`summary_with_days_read`, agent_import_transfers.rs:510). A build carries the same section (agent_import.rs:1959). `verify_import` carries it with no day reads (agent_import.rs:2134; `transfers_for_verification`, agent_import_transfers.rs:1065), so it never says `not_found`.
3. **`not_found`** (`open_state`, agent_transfer_pairing.rs:1485): no candidate; the other ledger's verified windows hold the date (`holds`, :1575); and a read of the date and one day either side (`DAY_READ_WIDTH`, :278) found no entry on the other ledger (`found_no_contra`, :413). At most four days are read a call (`MAX_DAY_READS`, :621); steps 5 and 7 need three. A window is the statement's rows less its first and last day (`coverage`, proposals.rs:262): B1's 20 and 28 Oct give 21 to 27 Oct.
4. **Pairs.** A candidate is a Contra of another lineage built from the other account's statement (`is_edge`, agent_transfer_pairing.rs:1470) and posted and verified (`is_verified`, :1474). Round 1 pairs equal references with no date bound (:1271). Round 2 takes only pairs where one side has no reference (:1298), and it is `Paired` only when both statements cover the dates, else `MatchedUnconfirmed` (:1350). A Contra keyed in Tally is in no lineage, so it is never a candidate.
5. **References.** HDFC reads `IMPS-<digits>-` as an IMPS reference (`hdfc_reference`, bank.rs:855), admitted at twelve digits (`transfer_reference`, :395); a `<digits>-TPT-` line reads as mode TXN, so it carries none, and its party is the text after the third field (:812). The reference column is not read for pairing.
6. **Posting.** Receipt and Contra are live-qualified (`LIVE_QUALIFIED_VOUCHER_TYPES`, agent_import.rs:419); numberless needs automatic with retain (`admit_numbering_for_post`, agent_import_numbering.rs:493); dates must lie between books-from and the PC's today (`validate_dates`, agent_import.rs:4851). The post and its read-back are 1029-R's (Code admits there).

## Who sends each EXPECT's request (187e882b)
| step | request | sent by | the count |
|---|---|---|---|
| 0, 1, 2, P, 6, 11, 12 | marks, raw | the operator's curl on 9001 | one COMPANY row; V is its ALTVCHID, so +n is n vouchers created; M its ALTMSTID |
| 2 | company list, status, voucher types | `list_companies`, `tally_status`, `masters` | the rows named |
| 4, 5, 7 to 10 | none to Tally | `parse_bank_statement` (agent.rs:3399; "Never contacts Tally") | rows, skipped, counterparties |
| 3 to 10 | the build's reads; for a skip, the day reads | `build_import_xml` (agent.rs:3368) | `voucher_count`, `skipped_rows`, `transfers` states and `skips_paired` |
| P | reads, the one import, the read-back | `post_import` (agent.rs:3406) | Tally's `CREATED`; `posted_verified` |
| P | the verification reads | `verify_import` (agent.rs:3432) | `posted_verified`; `transfers` with no day reads |
| 5, 7, 11 | none to Tally | `egress_log` | the log's lines, read from the data folder |

## Keep
On `lab/1392-e-answers`, with `SHA256.txt`:
- every tool answer whole, with its size and sha256;
- each post's times and the box log;
- the marks answers;
- BK-E's name, size and sha256, and `restore-plan.txt`;
- a RESULTS.md with each case's state first, then the table of What the result decides filled in;
- a notes file with UTC times, one line per step.

The statements and `pw.txt` stay on the lab PC (they are this folder's output). Screens stay on the lab PC, listed by name, size and sha256. The data folder stays on the lab PC, listed by file name, size and sha256. Send the sha to the run coordinator before any push.
