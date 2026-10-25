# #1331: the scoped compliance listing in its production shape, and its cost on a large book

Not for merge. READS ONLY, on two companies the lab already holds, one after the other. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15) applies in full to each part, with that part's company as THE COMPANY; where this sheet and it differ, the stricter holds.

**WHY.** On 10 October the compliance listing restricted to four parents was captured on BRIDGE PILOT LAB, in the variant that also fetches each ledger's GST details (`q1`, fork branch `lab/1331-capture-answers` at b5c0ff86). It answered exactly the 11 ledgers under those parents, and `$Parent` folded case. Two things are still not captured:
- **(a)** The production parent-part listing (`render_party_ledger_master_request_for_parents`): the same request without the GST-details fields.
- **(b)** Its cost on a large book against the whole listing it replaces. That cost is the reason the invoice build's 5,000-mark and 2,000-ledger limits exist.

**THE REQUESTS** are committed requests, changed only as `SHA256.txt` says:
- `p02` is the committed scoped rates request with its GST-details `FETCH` suffix removed, which is the production parent-part listing.
- `d03` is `p02` with the company and period changed.
- `d02` is the committed whole listing. `d03` differs from `d02` only by the parent formula and its `FILTERS` (checked).

**OWNER steps** are marked [OWNER].

## Before the sitting
1. [OWNER] Tally licensed. Educational mode = STOP (C1).
2. Check every file here against `SHA256.txt`.
3. [OWNER] Go-ahead.

## Part P (THE COMPANY = BRIDGE PILOT LAB, synthetic; STOP if absent)
1. C2 step 0. Then `requests/p01_marks.request.utf16le.xml`.

   EXPECT: GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`. Record ALTVCHID and ALTMSTID; on 10 October they were 35 and 234.
2. `p02_listing_four_parents.request.utf16le.xml`. Size bound 200,000 bytes. Record the UTC time before the send and after the answer.
3. C10 for this company.

## Part D (THE COMPANY = BRIDGE CORPUS DENSE, synthetic, voucher mark about 29,900; STOP if absent)
1. Load THE COMPANY alone (C1). C2 step 0. Then `d01_marks.request.utf16le.xml`. Record the books' first date, as the Gateway or the company's details show it.
2. GET /status. Then `d02_listing_whole.request.utf16le.xml`, the whole listing. Size bound 16,000,000 bytes. An answer above it = STOP before it is used (C3). Record the send and answer times.
3. GET /status. Then `d03_listing_four_parents.request.utf16le.xml`, the scoped listing. Size bound 16,000,000 bytes. Record the send and answer times.
4. GET /status. Then `d02` again, and then `d03` again (a second timing of each, in the same order).
5. C10.

## EXPECT (recorded; the lab judges only the STOP rules)
- **p02:**
  - 11 `LEDGER` rows, the same 11 GUIDs as the 10 October `q1`.
  - Every element of each row equal to `q1`'s row, except the GST-details elements `q1` alone fetched.
  - The computed company GUID is PILOT LAB's.
- **d02 and d03:**
  - Row counts, bytes, and send-to-answer times, both runs.
  - d03's GUIDs equal exactly the d02 rows whose PARENT is one of `Duties & Taxes`, `Indirect Expenses`, `Sales Accounts` or `Sundry Debtors`, folding case as on 10 October. Any row missing or extra is the finding.
- **Recorded for the build** (no EXPECT): the book's ledger count (d02's rows) and its master mark (ALTMSTID), beside the invoice build's limits of 2,000 ledgers and 5,000 marks.

## The books
Neither book changes; every request is a read.

## Close and evidence
- **C10** after each part.
- **C11:** evidence to a new branch `lab/1331-scoped-listing-answers`, after the pre-push scan and [OWNER] yes. It holds:
  - every answer, whole, with headers, size, sha256 and its send and answer times;
  - the GUID comparison tables;
  - the notes.

  Screenshots stay on the lab PC, listed by name, size and sha256. d02's answer can be large: if it is above 2,000,000 bytes, keep it on the lab PC listed by hash, and push only its row table.
