# Results: #1356 captures 1 and 2 on BRIDGE BILLTRAIL LAB, TallyPrime 7.2 Silver, 27 Oct 2026 (UTC)

The sheet is `lab/1356-party-term-and-ledger-outstandings-run-sheet/RUN-SHEET.md` at 1f97cdef8d0f69c0d087847de90d057e4cb58dda (SHA256.txt 9 of 9 equal; request census ok 3, skipped 2, FAIL 0). The company is number 100041, GUID 9ab5002c-e910-4242-83e4-a27cc490600b. Every request was sent by curl, one at a time.

## Capture 2: Tally's native Ledger Outstandings for Lab Party P1
- **Repeatable and parsable.** r04 (to 31 Mar 2026, 5,878 B), r05 (to 31 Jul 2025, 3,926 B) and r04 again were sent. The two r04 answers are **byte-equal**. Each parses as an ENVELOPE of BILLFIXED groups (BILLDATE, BILLREF), each with BILLOP, BILLCL, BILLDUE and BILLOVERDUE and its voucher lines (`tables/capture2-bills.txt`).
- **Every bill equals Tally's own Ledger Outstandings screen** for the same date: reference, date, opening, pending amount, Dr/Cr, due date and overdue days.

| at | OB-1 | INV-1 | ADV-1 | INV-2 | CN-1 | On Account | total pending |
|---|---|---|---|---|---|---|---|
| 31 Mar 2026 | 10,000.00 Dr | 12,000.00 Dr | 3,000.00 Cr | 3,000.00 Dr | 600.00 Cr | 4,600.00 Dr | 26,000.00 Dr |
| 31 Jul 2025 | 10,000.00 Dr | 20,000.00 Dr | 3,000.00 Cr | 5,000.00 Dr | — | 5,000.00 Dr | 37,000.00 Dr |

  ADV-2 is in neither answer nor on the screen. The 31 March figures equal the 9 October record.

## Capture 1: the party term against the unfiltered window
**FINDING: the `$PartyLedgerName` term loses Lab Party P1 allocations.**
- **Before any change** (step 2), r02 holds 7 vouchers and r03 holds 4. r03 drops Receipts 1, 2 and 3: 20 May, 10 Jun and 5 Aug 2025. Their PARTYLEDGERNAME is "Lab Bank", yet each carries a Lab Party P1 allocation:
  - ADV-1, Advance, 3,000.00;
  - ADV-2, Advance, 4,000.00;
  - INV-1, Agst Ref, 8,000.00.
- **After J1** (step 6), r02 holds 8 vouchers, including J1 of 20 Sep 2025. **J1's PARTYLEDGERNAME is "BRIDGE Party P2"**, and it carries Lab Party P1's Agst Ref INV-1, 500.00 (Cr). r03 still holds the same 4 vouchers and **not J1**.

Every Lab Party P1 allocation of each answer is listed in `tables/capture1-allocations.txt`.

| voucher | PARTYLEDGERNAME | Lab Party P1 allocations | in r02 | in r03 |
|---|---|---|---|---|
| Sales 1, 10 Apr | Lab Party P1 | INV-1 New Ref -20,000 | yes | yes |
| Receipt 1, 20 May | Lab Bank | ADV-1 Advance 3,000 | yes | **no** |
| Receipt 2, 10 Jun | Lab Bank | ADV-2 Advance 4,000 | yes | **no** |
| Sales 2, 15 Jul | Lab Party P1 | ADV-2 Agst Ref -4,000; INV-2 New Ref -5,000 | yes | yes |
| Receipt 3, 5 Aug | Lab Bank | INV-1 Agst Ref 8,000 | yes | **no** |
| Credit Note 1, 1 Sep | Lab Party P1 | CN-1 New Ref 600; On Account 400 | yes | yes |
| J1, 20 Sep (step 6 only) | BRIDGE Party P2 | INV-1 Agst Ref 500 | yes | **no** |
| Receipt 4, 10 Mar 2026 | Lab Party P1 | INV-2 Agst Ref 2,000 | yes | yes |

## The book and the marks
| point | ALTVCHID | ALTMSTID |
|---|---|---|
| M0 (step 0) | 7 | 212 |
| M0′ (after BK1) | 7 | 212 |
| M1 (after P2 and J1) | 8 | 213 |
| M2 (after the BK1 restore) | 7 | 212 |

M0′ and M2 are byte-equal to M0. The restore asked "Company data folder (100041) already exists … Replace / Skip", and Replace was chosen. The r02 read after the restore is **byte-equal** to step 2's r02. BK1 is `TDBK1800_100041.001`, 33,477 B, sha256 f480f13c22dce2dbac0ec04f9ead37ab23f47dc3f2cb31f999a7483b49cd717d; it stays on the lab PC.

## A deviation, and a slip
- **The size bound.** The sheet set 200,000 B for the reads. r02 answered 249,288 B (200, equal to CONTENT-LENGTH, a complete 7-voucher window). That was a stop, and the row paused there. Before step 3 the bound for this row's r02 and r03 reads was raised to 400,000 B, because the 200,000 was an estimate. Any read above 400,000 B, or any non-200, stays a stop. r02 then measured 283,556 B at step 6 and 249,288 B at step 7.
- **The slip.** At step 2 the send helper reported the stop with exit code 3, but the wrapper around it did not halt. **r03 was sent one second after the stop line.** Both are reads, there was no dialog before or after, and nothing was keyed. The wrapper now halts on any non-zero exit.

## Not measured
Capture 3 (cost on a large book), as the sheet says. TallyPrime 7.1 and Gold.

## Files
- `answers/`: every answer whole, with response headers: m0, m0prime, m1, m2, c2-r04a, c2-r05, c2-r04b, c1-s2-r02, c1-s2-r03, c1-s6-r02, c1-s6-r03 and c1-s7-r02.
- `tables/`: the capture 1 allocation table and the capture 2 bill table, with the screen's figures typed from the captures.
- `send-log.txt`: each send with the request's sha256 and the answer's size and sha256. `notes.txt`: one line per step, in UTC.
- `SCREENSHOTS.txt`: the captures and BK1 by name, bytes and sha256.
