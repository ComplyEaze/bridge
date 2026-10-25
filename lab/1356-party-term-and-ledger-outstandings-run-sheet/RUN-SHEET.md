# #1356 captures 1 and 2: a party term against the unfiltered window, and Tally's own Ledger Outstandings

Not for merge. One item on a company the lab already holds. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15) applies in full; where this sheet and it differ, the stricter holds.

THE COMPANY is BRIDGE BILLTRAIL LAB: synthetic, built for #1356 capture 4 on 9 October 2026 (see the issue's comments). It holds books from 1 April 2025 and the party Lab Party P1 (Sundry Debtors, bill-wise), which has:
- the opening OB-1, 10,000.00 Dr, and On Account 5,000.00;
- invoices INV-1 and INV-2;
- the advances ADV-1 (never adjusted) and ADV-2 (adjusted by the 15 July invoice);
- the credit note CN-1 600.00, with On Account 400.00;
- a post-dated receipt of 10 March 2026.

If THE COMPANY is not on the lab PC: STOP. Never rebuild it.

Capture 3 (the cost on a book of thousands of vouchers) is not here: it needs the large synthetic book that sitting 5 builds.

**WHY.**
- **Capture 1:** the `outstandings` detail reads the whole voucher window and keeps each entry by its own ledger. A party read could be narrower only if a `$PartyLedgerName` term loses no allocation of that party. The case it could lose is a voucher whose party ledger is someone else, such as a journal whose second party is this one.
- **Capture 2:** Tally's native `Ledger Outstandings` report for one party might replace the whole-window read. At most three requests discover its answer; no response grammar is held yet.

**OWNER steps** are marked [OWNER].

## Before the sitting
1. [OWNER] Tally is LICENSED. The keyed date in step 5 is not day 1, 2 or 31. Educational mode = STOP (C1).
2. Check every file here against `SHA256.txt`.
3. [OWNER] Go-ahead.

## Requests (raw, C3, one at a time; size bound 200,000 bytes, the marks read 20,000)

| file | what | shape |
| --- | --- | --- |
| `requests/r01_marks.request.utf16le.xml` | ALTVCHID, ALTMSTID | committed |
| `requests/r02_vouchers_fy_unfiltered.request.utf16le.xml` | every voucher, 1 Apr 2025 to 31 Mar 2026, with `ALLLEDGERENTRIES.*` (the window the detail reads) | committed, dates changed |
| `requests/r03_vouchers_fy_party_term.request.utf16le.xml` | r02 with `AND $PartyLedgerName = "Lab Party P1"` added to its filter formula | NEW SHAPE (C3 screenshot after) |
| `requests/r04_ledger_outstandings_p1_to_20260331.request.utf16le.xml` | `Ledger Outstandings`, Lab Party P1, to 31 Mar 2026, exploded | NEW SHAPE |
| `requests/r05_ledger_outstandings_p1_to_20250731.request.utf16le.xml` | the same to 31 Jul 2025 | NEW SHAPE |

r04 and r05 are the request text of the dormant `Ledger Outstandings` probe removed by #1747 (`bills_native_outstandings_probe.rs` at bc5077bff^), checked byte for byte, rendered for this company, party and date.

## Steps
0. C2 step 0, THE COMPANY loaded alone. Marks **M0** (r01).
1. **Capture 2, discovery** (three requests, no more):
   1. r04.
   2. r05.
   3. r04 again.

   Each is a NEW SHAPE: take a screenshot after each and before anything else. A dialog = C7.
   
   **Oracle:** Tally's own Ledger Outstandings screen for Lab Party P1, "For 31-Mar-26" and then "For 31-Jul-25". Open it under Display > Statements of Accounts > Outstandings > Ledger and screenshot each date. It is a report, not a master or voucher screen, so leave it with Escape. Navigation only; no key that alters anything.
2. **Capture 1, before any change:** r02, then r03.
3. Backup **BK1** (C5) into a NEW empty folder `C:\bridge-lab\1356\backups\BK1\`.
4. **By hand:** create ledger `BRIDGE Party P2` under Sundry Debtors, bill-wise ON, no opening. Screenshot before the accept.
5. **By hand:** key Journal J1, dated 20-09-2025, with the first line BRIDGE Party P2 so that it becomes the voucher's party:
   - BRIDGE Party P2 Dr 500.00, bill New Ref `J-1`;
   - Lab Party P1 Cr 500.00, bill Agst Ref `INV-1`.

   Screenshot before the accept. Any prompt beyond the bill-wise breakups and the accept = C7.
6. Marks **M1**. Then r02, then r03 again.
7. Restore **BK1** (C6). Marks **M2**: EXPECT M2 = M0. Then r02 once more: EXPECT the same vouchers and allocations as step 2's r02.

## EXPECT (recorded; the lab judges nothing but the STOP rules)
- **Capture 2:**
  - Each answer either parses as XML (C4) or is recorded as what it is (an empty envelope, an error line), with its size.
  - r04's two answers are compared byte for byte (repeatability).
  - Every bill or line it holds is compared with the screen for the same date: reference, date, amount, Dr/Cr, due date.
  - Recorded from the 9 October run, for comparison only (the screen is the oracle tonight):
    - at 31 Mar 2026: INV-1 12,000.00 Dr, INV-2 3,000.00 Dr, ADV-1 3,000.00 Cr, CN-1 600.00 Cr, OB-1 10,000.00 Dr;
    - ADV-2 is not listed.
- **Capture 1:**
  - For every voucher, list each ledger entry of Lab Party P1 with its bill allocations (name, type, amount), from r02 and from r03 at step 2 and again at step 6.
  - The question is whether r03 holds every Lab Party P1 allocation r02 holds.
  - At step 6, J1 is in r02. Record whether r03 holds J1 at all, and what J1's PARTYLEDGERNAME is in r02.
  - Neither answer is "right" in advance; the comparison is the capture.

## The book after each step
- Through step 2: unchanged.
- After step 4: plus BRIDGE Party P2.
- After step 5: plus J1.
- After step 7: as at BK1, the book of 9 October.

## Close and evidence
- **C10:** Shut Company; no company loaded.
- **C11:** evidence to a new branch `lab/1356-party-term-and-ledger-outstandings-answers`, after the pre-push scan and [OWNER] yes. It holds:
  - every answer, whole, with headers, size and sha256;
  - the allocation tables of capture 1;
  - the bill tables of capture 2, beside the screen's figures typed from the screenshots;
  - the notes;
  - the backup's name, size and hash.

  Screenshots stay on the lab PC, listed by name, size and sha256.
