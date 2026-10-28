# #1356: a ledger-filtered voucher collection that carries each entry's bill allocations, and its cost on a dense book

Not for merge. READS ONLY, on two companies the lab already holds, one after the other. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/485-ledger-filtered-vouchers-run-sheet` at 4e769a9c, itself a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15) applies in full to each part, with that part's company as THE COMPANY; where this sheet and it differ, the stricter holds.

**WHY.** `outstandings` with `party` and `detail` reads the company's whole voucher window from the start of the books, then keeps each entry by its own ledger. #1356's decision (28 Oct 2026) is to read a party's vouchers by the ledger filter instead: on BRIDGE BILLTRAIL LAB, #485 part T showed that collection (type `Vouchers : Ledger`, the ledger named in `CHILDOF`) returns all 7 of Lab Party P1's vouchers, with GUIDs equal to the whole-window read's. A filter on the voucher's party ledger dropped 3 of them. But that capture fetched identity fields only, and the detail needs each entry's bill allocations. Two things are not yet captured:
- **(A)** whether the same collection, asked for the window read's field list (which includes `ALLLEDGERENTRIES.*`), answers with the party's entries and bill allocations equal to the whole-window read's, voucher for voucher;
- **(B)** what the full field list costs on a dense book against the whole-window read. #485 part D's second sitting (28 Oct, `lab/485-ledger-filtered-vouchers-answers` at 9695483e, `part-D-28oct/`) sent the identity-only candidate `d03` on BRIDGE CORPUS DENSE. Its 82 rows equalled the window's 82 `DN Sales` vouchers, at 226,626 bytes in 0.286 s against the window's 2,900,742 bytes in 1.193 s, with no dialog. The full-field version was never sent.

**THE REQUESTS.** Six are byte copies from `lab/485-ledger-filtered-vouchers-run-sheet` at 4e769a9c:
- the marks (`t01`, `d01`);
- the committed window, as ground truth (`t02`, `d02`);
- #485's identity-only candidate, as the control (`t03`, `d03`).

Two are new:
- `t04_ledger_vouchers_p1_fy_entries` is `t03` with its one `FETCH` element replaced by `t02`'s, byte for byte.
- `d05_ledger_vouchers_dn_sales_20250515_entries` is `d03` with its `FETCH` replaced by `d02`'s.

Nothing else differs from the candidate: the same ID and collection name, the same type, the same `CHILDOF`, the same date-window filter, and no `COMPUTE` lines. So a difference between `t03` and `t04` answers is the field list's. The ledger name is element text in `CHILDOF`, not a function argument.

**REQUESTS BY CURL** are allowed by the owner's decision of 26 Oct 2026 that requests sent by curl may be used to discover shapes. Each request is sent exactly as LAB-COMMON C3 says, one at a time. Before the first send, run `python -I check-request-ids.py requests` in this folder. EXPECT `census: ok 8, skipped 0, FAIL 0`; anything else = STOP, nothing sent. A marks answer's size bound is 8,192 bytes.

**OWNER steps** are marked [OWNER]. A person is AT THE LAB MACHINE for every NEW SHAPE request below: a wrong spelling can raise a modal dialog that holds the gateway, which needs a person, not a retry. The [OWNER] steps are the owner's, or the lab's on the owner's recorded word, as for #485 part D on 28 Oct. The run coordinator's row note says which.

## Before the sitting
1. [OWNER] Present at the machine for both parts. Tally licensed; Educational mode = STOP (C1). Record the release from Tally's About screen.
2. Check every file here against `SHA256.txt`.
3. [OWNER] Go-ahead.

## Part T: the tiny book (THE COMPANY = BRIDGE BILLTRAIL LAB, number 100041, synthetic; STOP if absent)
The book as #485 part T and #1356's capture 1 left it: 7 vouchers in 1 Apr 2025 to 31 Mar 2026, each carrying a `Lab Party P1` entry. #1356's added Journal was restored away on 28 Oct. Record the UTC time before each send and after each answer.
1. Load THE COMPANY alone (C1). C2 step 0. Then `t01_marks` (marks). EXPECT ALTVCHID 7 and ALTMSTID 212, as #485 read them on 26 Oct. A difference means the book is not as those reads left it: record it and STOP.
2. `t02_vouchers_fy`: the ground truth. Size bound 500,000 bytes (it answered 249,288 on 26 Oct).
3. `t03_ledger_vouchers_p1_fy`: the identity-only control. Size bound 200,000 bytes (it answered 21,904 on 26 Oct).
4. GET /status. Then `t04_ledger_vouchers_p1_fy_entries`. NEW SHAPE: take a screenshot of the whole Tally window at once. Size bound 600,000 bytes. Then GET /status again.
5. A dialog, a hang, a non-200 or an answer that does not parse = STOP (C3, C7, C9). The gateway is not probed again.
6. `t01_marks` again. EXPECT the marks of step 1, since the reads write nothing. A difference = record it and STOP.
7. C10 for this company (Shut Company). Go to part D only if `t04` answered in-band: any answer that parses, even an empty one.

## Part D: one dense day (THE COMPANY = BRIDGE CORPUS DENSE, synthetic; STOP if absent)
The ground truth from 28 September: 82 vouchers on 15 May 2025, each with a ledger line named exactly `DN Sales`. #485 part D read the marks as ALTVCHID 29900 and ALTMSTID 328.
1. Load THE COMPANY alone (C1). C2 step 0. Then `d01_marks`.
2. `d02_vouchers_20250515`: the ground truth. Size bound 6,000,000 bytes (it answered 2,900,742 on 26 Oct).
3. GET /status. Then `d03_ledger_vouchers_dn_sales_20250515`, the identity-only candidate, already sent on this book on 28 Oct: it is the cost control for `d05` in the same sitting. Size bound 1,000,000 bytes. Then GET /status.
4. Only if step 3 answered in-band: GET /status, then `d05_ledger_vouchers_dn_sales_20250515_entries`. NEW SHAPE: screenshot at once. Size bound 6,000,000 bytes. Then GET /status.
5. In steps 3 and 4, a dialog, a hang, a non-200 or an answer that does not parse = STOP (C3, C7, C9). No other day, ledger or shape in this item. A STOP leaves the gateway alone; a gateway that stops answering is for the owner to restart.
6. `d01_marks` again. EXPECT the marks of step 1. A difference = record it and STOP.
7. C10.

## EXPECT (recorded; the lab judges only the STOP rules)
For each candidate answer (`t03`, `t04`, `d03`, `d05`), record:
- whether it parses;
- its row element and row count;
- whether every row carries GUID, ALTERID and MASTERID, each non-empty;
- the time from send to answer, and the bytes.

**The comparison is made by `compare1356.py`**, in this folder, run as `python -I compare1356.py LEDGER EXPECTED_COUNT TRUTH CANDIDATE`. Keep its whole output.
- It parses both answers as XML after dropping only what C4 drops, never by a text search.
- Per voucher GUID it compares the voucher's own fields, and each `ALLLEDGERENTRIES.LIST` of the ledger as its whole subtree. That covers the amount, the sign and every bill allocation with its name, type, date and amount. The window read's COMPUTE results are left out, since the candidate has none.
- **Exit 0:** the candidate holds exactly the ground truth's vouchers that have an entry of the ledger, each equal.
- **Exit 1:** differences, each listed.
- **Exit 2:** an error, never a comparison: an answer that does not parse, a missing or repeated GUID, or a ground truth that does not hold EXPECTED_COUNT such vouchers.

What to run:
- `python -I compare1356.py "Lab Party P1" 7 t02-answer t04-answer`: EXPECT exit 0, 7 equal. The result wanted is every voucher equal, Lab Party P1's three bank-ledger Receipts among them. Any difference, either way, is the finding.
- `python -I compare1356.py "DN Sales" 82 d02-answer d05-answer`: EXPECT exit 0, 82 equal. On 28 Oct the `DN Sales` lines carried an empty `BILLALLOCATIONS.LIST`, so this part tests the entries and the cost, not bill allocations.
- `t03` and `d03`: GUIDs only, against the same ground truth. The comparer reports them as different, because they carry no entries. Record their row counts and GUID sets.
- **Cost:** `d02`'s time and bytes against `d03`'s and `d05`'s for the same day, and `t02` against `t04`.

An empty envelope for a candidate is a result: record it, with its size.

## The books
Neither book changes; every request is a read. Marks are taken at each part's start and end, and must be equal.

## Close and evidence
- **C10** after each part.
- **C11:** evidence to a new branch `lab/1356-ledger-filtered-allocations-answers`, after the pre-push scan and the run coordinator's yes. It holds:
  - every answer, whole, with headers, size, sha256 and its send and answer times (an answer above its bound is listed by size and sha256 and kept on the lab PC);
  - each `compare1356.py` output;
  - the notes, with UTC times and one line per step.

  Screenshots stay on the lab PC, listed by name, size and sha256.

## Not covered
These are preconditions on any builder made from this shape:
- **A ledger name outside plain letters, digits and spaces in `CHILDOF`.** Whether `CHILDOF` reads its value as plain text or as a formula, and what `&`, `,`, `(`, `.`, `/` or a leading `$` or `#` do there, is not measured. Until it is, a builder admits only a closed alphabet (AGENTS.md P7) and refuses any other name.
- **A `CHILDOF` naming a ledger that does not exist:** an empty answer or a dialog is not known.
- **Optional and cancelled vouchers:** neither book holds one in these windows.
- **An opening bill.** BRIDGE BILLTRAIL LAB holds one (OB-1, keyed by hand on 9 Oct). No voucher read shows it, and the detail does not list it as a row (#1356, 9 Oct). This item does not change that.
- **The window read's COMPUTE fields on a `Vouchers : Ledger` collection:** `t04` and `d05` carry none, so whether they evaluate there is not measured.
- **A party ledger on the dense book:** part D measures cost on a sales ledger, `DN Sales`.
- **Any build's own read:** this item measures the request shape only. A builder is written after this capture, and goes through review like any other.
