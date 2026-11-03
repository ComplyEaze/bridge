# Results: #2210, what `verify_import` reads back after Bridge's own posts on a fresh book, 3 Nov 2026

**One EXPECT missed: neither `verify_import` sent a census request.** The sheet expects `SEESESEHSHSEECSCSEEVSVSEEVSVSE`. Each call sent `SEESESEHSHSEEVSVSEEVSVSEEHSHSE`: 30 requests, with no census pair and one more marks pair at the end. No other request of the row was a census. Every other EXPECT held: both posts and both reviews `posted_verified`, the marks moved by exactly 50 and by one, `batch_step` `matched`. The row is stopped here for the run lead.

Sheet: `lab/2210-review-capture-sheet` at 50a2e913, `relay.py` 44abb451..., `batch-vouchers.json` fce9f569..., `payment-vouchers.json` 1e0ba099..., all equal. Build: master 8520f071, run 41537873751, artifact 13185636227, `bridge_mcp.exe` sha256 bb8454bc6b52fe800a61ba7faf553427ebae1948a25771a787ed0dad77ba195f, before and after. TallyPrime 7.2 Silver, licensed. [verified]

## The book
`BRIDGE REVIEW LAB`, created on Tally's screen this sitting: books from 1-Apr-26. GST was on by default and was set to No. Loaded alone. GUID 63292d7c-d541-42e5-aa9d-2ab5f8b993ff; the company number is in the local files only.

Set-up only, by direct XML, each `CREATED` 1:
- `Test Expense B` under Indirect Expenses, bill-wise off. `Cash` as Tally made it. M 207.
- **One Journal, 1-Apr-26, Test Expense B Dr / Cash Cr 0.01, narration `set-up`**, on the run lead's word. The first attempt (`run-first/`, `out-first/`) was refused at the batch build with `empty_book_first_import`: a book that never held a voucher reports no voucher mark, and nothing was sent. With the set-up voucher, V 1 before step 2. The census reads would include it; the import verification reads do include it (below).

## Steps
| step | call | answer | seconds | V after |
|---|---|---|---:|---:|
| 1 | relay `pass`; MARKS; `list_companies`, `tally_status` | fine | | 1 |
| 2 | build 50 Journals, post, Yes by mouse | `posted_verified`, created 50, `batch_step` `matched`, step 50 from 1 | 22.48 | 51 |
| 3 | build the Payment, post, Yes by mouse | `posted_verified`, created 1 | 21.80 | 52 |
| 4 | `verify_import` batch, then payment | `posted_verified` 50 of 50, 1 of 1 | 3.54, 3.32 | 52 |
| 5 | MARKS, relay stopped | | | 52 |

Each window showed what was built: 50 Journals on 1-Apr-2026, Dr 1,275 Test Expense B and Cr 1,275 Cash; then one Payment, 3-Apr-2026, Dr 1.00 Test Expense B and Cr 1.00 Cash. Each post answered in one call. M stayed 207. Beyond the sheet, I also read MARKS by curl between steps, all on 9001 outside the relay. [verified]

## Checks, by parsing `out/relay.jsonl` and `out/bodies/`
- One `open`, one `close`, no `tally_closed` or `relay_error`; 438 requests on one connection. [verified]
- **Request order of each `verify_import`:** `SEESESE HSHSE EVSVSE EVSVSE EHSHSE` against the sheet's `SEESESE HSHSE ECSCSE EVSVSE EVSVSE`, identical for both calls. S is the empty `/status`, E the company extent, H the marks, V the import verification, C the census. **EXPECT missed.** [verified]
- **Census:** no request with the census ID in the whole row, posts included, so none to decode. [verified]
- **Repeats:** within each call, every repeat of a request has one `answer_sha256`. [verified]
- **Import verification rows, by parsing:** the batch's window, 1 April, holds **51** VOUCHER rows: the 50 and the set-up Journal, which shares the date. The payment's window, 3 April, holds 1. Its request is a date window (`$Date` from and to the day), not an `$AlterID` span. [verified]
- Each verify read the voucher mark 52.

## Kept
Tool answers, both `relay.jsonl`, `bodies/`, the journal and both import files from the data folder (copied untouched, byte-equal), MARKS, the company GUID. The journal and import files hold no machine path, and their narrations are the sheet's own: `Review row N`, `Review row payment`, and the set-up Journal's `set-up`. [verified]

## The book now
BRIDGE REVIEW LAB holds the set-up Journal, the 50 Journals and the Payment: V 52, M 207. No backup was asked for and none was taken. Company to be shut after the run lead's word.

**Confidence:** PARTIAL: one book, one sitting, and the row's central check was not met.
