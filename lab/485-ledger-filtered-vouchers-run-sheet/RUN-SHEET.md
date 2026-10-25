# #485: a ledger-filtered voucher collection (its shape, GUID and AlterID, and its cost)

Not for merge. READS ONLY, on two companies the lab already holds, one after the other. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15) applies in full to each part, with that part's company as THE COMPANY; where this sheet and it differ, the stricter holds.

**WHY.** `ledger_movement` for one ledger still reads the whole book's voucher window. On 28 September, Tally's built-in `Ledger Vouchers` report answered a request with `<ENVELOPE></ENVELOPE>`, on a day holding 82 matching vouchers (see the issue). The fallback is a voucher collection limited to one ledger. Before any build it needs a capture of three things:
- whether such a collection answers at all, and in what shape;
- whether each row carries GUID, ALTERID and MASTERID;
- what it costs on a dense book, against the whole-window read.

**THE CANDIDATE** (one variable: the collection). It is the committed voucher-window request with only its collection changed:
- type `Vouchers : Ledger`, Tally's own "vouchers of a ledger";
- the ledger named in `CHILDOF`;
- an explicit field list of identity and flags only (DATE, GUID, ALTERID, MASTERID, VOUCHERTYPENAME, VOUCHERNUMBER, ISCANCELLED, ISOPTIONAL, ISPOSTDATED);
- the same date-window filter.

It uses no `$$FilterCount` or `$$NumItems` and no report stack. Spellings of those have raised blocking dialogs or hung the gateway before, and a spaced name inside a function was one cause. Here the ledger name is element text, not a function argument.

**OWNER steps** are marked [OWNER]. The owner is AT THE LAB MACHINE for every NEW SHAPE request below. A wrong spelling can raise a modal dialog that holds the gateway; that needs a person, not a retry.

## Before the sitting
1. [OWNER] Present at the machine for parts T and D. Tally licensed. Educational mode = STOP (C1).
2. Check every file here against `SHA256.txt`.
3. [OWNER] Go-ahead.

## Part T: a tiny book first (THE COMPANY = BRIDGE BILLTRAIL LAB, synthetic, 9 October; STOP if absent)
For every request in this part, record the UTC time before the send and after the answer.
1. C2 step 0. Then `requests/t01_marks.request.utf16le.xml` (marks).
2. `t02_vouchers_fy.request.utf16le.xml`: the committed window for 1 Apr 2025 to 31 Mar 2026, the ground truth. Size bound 500,000 bytes.
3. GET /status. Then `t03_ledger_vouchers_p1_fy.request.utf16le.xml`: the candidate, ledger `Lab Party P1`, same dates. NEW SHAPE: screenshot of the whole Tally window at once. Size bound 200,000 bytes. Then GET /status again.
4. A dialog, a hang, a non-200 or an answer that does not parse = STOP (C3, C7, C9). The gateway is not probed again.
5. C10 for this company (Shut Company), then go to part D only if part T answered in-band (any answer that parses, even an empty one).

## Part D: one dense day, then one week (THE COMPANY = BRIDGE CORPUS DENSE, synthetic; STOP if absent)
The ground truth from 28 September: 82 vouchers on 15 May 2025, every one with a ledger line named exactly `DN Sales`. The corpus runs about 82 vouchers a day and 574 a week.
1. Load THE COMPANY alone (C1). C2 step 0. Then `d01_marks.request.utf16le.xml`.
2. `d02_vouchers_20250515.request.utf16le.xml`: the committed window for that day, the ground truth tonight. Size bound 1,000,000 bytes.
3. GET /status. Then `d03_ledger_vouchers_dn_sales_20250515.request.utf16le.xml`: the candidate for that day. NEW SHAPE on this book: screenshot at once. Size bound 1,000,000 bytes. Then GET /status.
4. Only if step 3 answered in-band and its row count equals step 2's: GET /status, then `d04_ledger_vouchers_dn_sales_20250512_18.request.utf16le.xml`, the candidate for 12 to 18 May 2025. Size bound 4,000,000 bytes. Then GET /status.
5. No month, no other ledger and no other shape in this item. A STOP leaves the gateway alone, and a gateway that stops answering is for the owner to restart.
6. C10.

## EXPECT (recorded; the lab judges only the STOP rules)
For each candidate answer (t03, d03, d04), record:
- whether it parses;
- its row element names and its row count;
- whether every row carries GUID, ALTERID and MASTERID, each non-empty;
- the time from send to answer, and the bytes.

**Compare with the ground truth:**
- **t03:** the GUIDs of the vouchers in t02 that carry a `Lab Party P1` ledger entry, against t03's GUIDs. Equal sets is the result wanted; any difference, either way, is the finding.
- **d03:** its GUIDs against those vouchers in d02 that carry a `DN Sales` entry (expected 82).
- **d04:** its row count against 574, recorded only (no ground-truth read for the week).
- **Cost:** d02's time and bytes against d03's for the same day.

An empty envelope for t03 or d03 is a result, the same as the built-in report's on 28 September. Record it, with its size.

## The books
Neither book changes; every request is a read. Marks are taken at each part's start only.

## Close and evidence
- **C10** after each part.
- **C11:** evidence to a new branch `lab/485-ledger-filtered-vouchers-answers`, after the pre-push scan and [OWNER] yes. It holds:
  - every answer, whole, with headers, size, sha256 and its send and answer times;
  - the GUID comparison tables;
  - the notes.

  Screenshots stay on the lab PC, listed by name, size and sha256.
