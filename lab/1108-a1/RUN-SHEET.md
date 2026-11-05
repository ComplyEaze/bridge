# Lab rows 1108-A1 and 1108-B: does Tally refuse a Journal line that ComplyEaze Bridge admits? SYNTHETIC COMPANY ONLY. Not for merge.

Part of #1108. Written 5 Nov 2026 from master cf2497e0. About 40 minutes.

This sheet changes no Bridge code and runs no Bridge build. `SHA256.txt` lists every file here
with its bytes; check it before step 0.

## Why

#1108 is closed only by a recorded post through `post_import` that lands partly. That needs a
fault Tally refuses for one voucher after every check of Bridge's has passed. A design note
looked for one in the code (confidence: unverified). Two candidates are shapes the code admits:

- **A1, a Stock-in-Hand ledger on a Journal line, in a book that integrates accounts with
  inventory.**
  - A Journal takes any balanced set of entries (`agent_import_schema.rs:67`).
  - Stock-in-Hand is on the non-money list (`agent_import_cash_bank.rs:133`), so no group check
    refuses it.
  - Tally's own voucher screen does not offer such a ledger on an integrated book. Whether an
    import refuses it is not known.
- **B, a cash ledger on a Journal line.**
  - The post's summary says "Journals may also move cash/bank ledgers"
    (`agent_import_post.rs:4074`).
  - Whether Tally applies a voucher type's entry settings to an import is not known.

Each row is one raw Journal of the shape `agent_import.rs:5618` renders. This sheet asks only
whether Tally refuses it. The `post_import` batch (A2, then the twins case A2′) is written only
if A1 refuses per voucher. It is not part of this sheet.

Each write is a candidate shape, sent by `curl` to this lab book only, as the owner allowed on
26 Oct 2026 for shape discovery.

## Rules and company

`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy (sha256 5370c6d2) of the
one on `lab/1381-stock-item-create` at 5bbd6fb2.

THE COMPANY is `BRIDGE STOCK CREATE LAB`, the synthetic company of row 1381-M, as BK0 of that row
left it. The 1381-M answers (`lab/1381-stock-item-create-answers` at d8948467) record M 210 and
V blank after its restore.

If it is not on the PC, or its GUID differs from the one in that row's lab notes: STOP. The row
has no book; never rebuild it.

Differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **Import answers.** In A1 and B, C4's EXPECT for an import is replaced by "record every
   counter and each `LINEERROR`": the counters are what these rows measure. A missing counter,
   or an answer that does not parse, is still a STOP.
2. **Size bound** for every read: 200,000 bytes.
3. **Grant.** Nothing else is granted:
   - BK-1108 (step 2);
   - the two ledger Creates W1 and W2 and the two Journals A1 and B, each sent once;
   - the screen read of step 4;
   - the reads;
   - the restore of BK-1108 on the run coordinator's RESTORE.

Licensed mode only; Education mode = STOP.

## The files

- `requests/` is written by `gen_requests.py`. Run `python3 -I gen_requests.py` here; it prints
  `8 request files`.
  - `marks` is the marks request of `lab/690-batch-ledger-create-l0` at 83a5c989, with this
    company's name.
  - W1 and W2 keep that row's ledger Create envelope byte for byte.
  - A1 and B keep the envelope of master's captured fixture `post-span-journal-import.xml`, a
    Journal batch that Bridge rendered and the lab sent live.
  - Only the messages inside `REQUESTDATA` are this row's.
- The reads `flags`, `ledgers` and `vouchers` are NEW SHAPES (C3: a screenshot before the next
  request). Each is a Collection export of fixed fields:
  - `flags`: Company NAME, GUID, ISINVENTORYON and ISINTEGRATED;
  - `ledgers`: NAME, PARENT, GUID, MASTERID and ALTERID;
  - `vouchers`: DATE, VOUCHERTYPENAME, NARRATION, GUID, MASTERID, ALTERID and the ledger
    entries' names and amounts, for 1-Apr-2025 to 31-Mar-2026.
- `check_1108.py` reads every answer as C4 says. Its modes are `marks`, `import`, `rows` and
  `diff BEFORE AFTER`. A read that failed prints `NOT READ` and exits 1; that is never "nothing"
  or "no change".
- `check-request-ids.py`: run `python3 -I check-request-ids.py requests` before the first send.
  EXPECT `census: ok 4, skipped 4, FAIL 0`.

## The writes

| Row | What it sends | The one variable |
|---|---|---|
| W1 | Ledger `A1 Closing Stock` under `Stock-in-Hand` | setup: the ledger A1 needs |
| W2 | Ledger `A1 Capital` under `Capital Account` | setup: the other leg of A1 and B |
| A1 | Journal, 10-Apr-2025: `A1 Closing Stock` Dr 100.00, `A1 Capital` Cr 100.00 | a Stock-in-Hand ledger on an integrated book |
| B | Journal, 10-Apr-2025: `Cash` Dr 200.00, `A1 Capital` Cr 200.00 | a cash ledger on a Journal |

W1 and W2 each EXPECT `CREATED 1` and every other counter 0, as C4 says.

## Steps

0. **LAB-COMMON step 0.**
   - Load THE COMPANY alone.
   - EXPECT one COMPANY row in `marks`, with M 210 and V blank (1381-M's restore). Any other
     value: record it and STOP.
   - Record TallyPrime's release and edition as the screen shows them.
   - Run the census, and check every file against `SHA256.txt`.

1. **The first reads.** Send `flags`, `ledgers` and `vouchers`, each followed by
   `check_1108.py rows`.
   - `flags`: EXPECT one COMPANY row, with ISINVENTORYON `Yes`. If ISINTEGRATED is not `Yes`:
     record it and STOP. A1 needs an integrated book, and no feature is changed in this sheet.
   - `ledgers`: EXPECT a ledger `Cash` under `Cash-in-Hand`. EXPECT no ledger named
     `A1 Closing Stock` or `A1 Capital`. Otherwise STOP.
   - `vouchers`: EXPECT no row.
   - Every GUID begins with THE COMPANY's, else STOP.

2. **BK-1108** (C5), into the new empty folder `C:\bridge-lab\1108a\backups\bk\`. Record its
   name, bytes and sha256. Then send `marks`, `ledgers` and `vouchers`: these are the BASELINE.

3. **W1, then W2.** For each one:
   1. `marks` (C3);
   2. send the write once;
   3. take a screenshot of the whole Tally window;
   4. run `check_1108.py import`;
   5. send `ledgers` and `diff` it against the previous one.

   EXPECT `CREATED 1` and one row ADDED with the sent name and parent. Anything else: record it,
   and skip every row that needs that ledger (W1 for A1; W2 for A1 and B). A skip is a result,
   not a STOP.

4. **Screen read, no write.**
   - Open Vouchers > Journal (F7) and press F12 (Configure). Screenshot it. Record every option
     shown, word for word, and whether any of them is about cash or bank accounts in journals.
   - Then open the Journal screen's ledger list on the first line. Screenshot it, and record
     whether `A1 Closing Stock` and `Cash` are offered.
   - Change nothing. Leave with Esc; if `Quit ?  Yes or No` shows, answer Yes.
   - Then `marks`: EXPECT M and V unchanged. A moved mark = STOP.

5. **A1, then B.** For each one:
   1. `marks` (C3);
   2. send the write once;
   3. take a screenshot of the whole Tally window;
   4. run `check_1108.py import`;
   5. send `marks` and `vouchers`, and `diff` `vouchers` against the previous one.

   Record every counter, each `LINEERROR`, whether V moved and by how much, and every row ADDED
   with its ledger entries. There is no EXPECT on the counters (Difference 1).

6. **The last reads.** Send `marks`, `ledgers` and `vouchers`, and `diff` the last two against the
   BASELINE.

7. **Restore BK-1108** (C6), only on the run coordinator's explicit RESTORE. Then send `marks`,
   `ledgers` and `vouchers`. EXPECT:
   - the COMPANY row's GUID, V and M equal to the BASELINE's;
   - each `diff` against the BASELINE showing no row ADDED, REMOVED or CHANGED.

   Anything else = STOP.

8. LAB-COMMON C10.

STOP RULES:
- C7 and C8;
- any dialog after a write or after a NEW SHAPE read (P7: a box blocks the gateway; screenshot it,
  press no key, report);
- an import answer that does not parse or lacks a counter;
- a MARKS answer with other than one COMPANY row;
- a GUID that does not begin with THE COMPANY's;
- a mark that moves in a step that sends no write.

## What the result decides

Recorded by the coordinator, each with its confidence.

| Answer | A1 | B |
|---|---|---|
| `CREATED 0` with `EXCEPTIONS 1` or `ERRORS 1`, V unchanged | A fault Tally refuses per voucher after Bridge's checks. Write A2 and A2′ (a batch through `post_import` with this voucher among created ones; then twins) for #1108's acceptance. | The same, for a cash line on a Journal. A2 may use either; A1's is preferred if both refuse. |
| `CREATED 1`, V +1 | The candidate is dead: Tally stores it. The note records it, and Bridge's admission is unchanged. | Dead the same way, which is consistent with Bridge's own summary line. |
| A box | A P7 hazard on a shape Bridge admits. Report privately to the coordinator before anything public. | The same. |

If both are dead, #1108's design note answers that no admitted fault was found without timing.

## Not measured by this row

- the same lines through `post_import` (A2, conditional on A1);
- a book not integrated with inventory;
- a Bank ledger on a Journal;
- Payment, Receipt and Contra;
- TallyPrime 7.1.

## Evidence

- Every answer whole, with its size and sha256, and every `check_1108.py` output.
- The screen read's options word for word, and the screenshots by name, size and sha256 (never
  pushed).
- BK-1108's name, bytes and sha256, `restore-plan.txt`, and a notes file with UTC times and one
  line per step.

The evidence shows THE COMPANY's GUID as `<company guid>` and its number as `<company number>`.
It goes on a new branch, `lab/1108-a1-stock-in-hand-journal-answers`, with `SHA256.txt`. Send its
sha to the run coordinator before pushing, and push only after the pre-push scan and the
coordinator's yes.
