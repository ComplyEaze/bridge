# Lab run sheet: the 0.7.0 reads on TallyPrime 7.2

All live evidence so far is from TallyPrime 7.1 Silver. Order: part A first, a regression pass of the committed requests (the request files are in `requests/`, UTF-16LE with a BOM, checked against `SHA256.txt` with `shasum -c`); then part B, the tool rows, run through the built tool of the release-candidate build and never as a hand-sent request.

Rules (all of them, for every group):
- Read-only. No key, no import, no write of any kind, and no company is created, renamed or restored in this sitting. Posting in the built tool stays OFF.
- Synthetic BRIDGE companies only. Before the first group read the company list (`requests/c0-company-list`, a read of the loaded companies). If any company that is not a BRIDGE synthetic book is loaded, STOP: no call against any book, report it. A company that is not listed is skipped and said so; it is never read.
- Before EACH group, send `c0-company-list` again and record the named company's identifier from the answer. Only when the company is listed may its group be sent. An empty answer from a report (an empty envelope) counts as a pass only after that proof, because a company that is not loaded also answers an empty envelope.
- One request at a time, from one sender whose client has no timeout (or one longer than the request needs) and never cancels, retries or re-sends a request in flight; wait for the answer. The Gateway of Tally shows no dialog before each call; if a dialog appears or Education mode is shown, STOP.
- Save every answer as received (BOM-less UTF-16LE) together with its HTTP status and headers, its size and its sha256. A failed request is reported as failed, never as an empty answer.
- Voucher mark: before and after each group send `requests/c1-marks-TEMPLATE` (placeholders `@@COMPANY@@` and `@@GUID@@` only, the identifier from the company list) and keep the two answers; the marks must be equal (reads must not move them).
- Request files are UTF-16LE with a BOM exactly as in `requests/` (`shasum -c SHA256.txt`); do not edit a request except the placeholders of the marks template.
- Screen comparisons (rows B1 and B2 name them) are steps for the person at the lab computer: they look at Tally's screen, one at a time, and returns to the Gateway of Tally before the next call.
- Record the build hash of the built tool for part B.

## A. Regression pass (shape drift, 7.1 to 7.2)
Send each file to the company named inside it, save the answer, and compare it with the 7.1 answer named below by structure: the same root, the same element names and order, the same number of rows, the same number format, sign and empty form. Equal bytes are a pass (an empty envelope only after the company has been proved loaded); count rows by element (the number of top-level item elements of each answer) and compare the counts; any difference is a finding to list (both answers kept), not a failure.

| file | company | 7.1 answer to compare with |
|---|---|---|
| a1-1-pl-tieout-b | BRIDGE TIEOUT B | `statement_profit_and_loss_stock_book_fy_live.utf16le.xml` in the repository's protocol fixtures |
| a1-2-bs-tieout-b | BRIDGE TIEOUT B | `statement_balance_sheet_stock_book_fy_live.utf16le.xml` |
| a1-3-ledgers-tieout-b | BRIDGE TIEOUT B | `statement_trial_balance_stock_book_fy_live.utf16le.xml` |
| a1-4-groups-tieout-b | BRIDGE TIEOUT B | `statement_groups_stock_book_live.utf16le.xml` |
| a1-5-flags-tieout-b | BRIDGE TIEOUT B | `company_inventory_flags_stock_book_live.utf16le.xml` |
| a2-1-negative-stock-negstock-lab-2-fy | BRIDGE NEGSTOCK LAB 2 | `ans-n1-negative-stock-negstock-fy.xml` on the lab branch `lab/1358-negative-stock` |
| a2-2-negative-stock-negstock-lab-2-to-may | BRIDGE NEGSTOCK LAB 2 | `ans-n3-negative-stock-negstock-to-20250531.xml` on the same branch |
| a2-3-negative-stock-tieout-c-fy | BRIDGE TIEOUT C | `ans-n2-negative-stock-tieout-c-fy.xml` on the same branch |
| a2-4-negative-ledgers-shape-lab-fy | BRIDGE SHAPE LAB | `ans-R09-negative-ledgers-shape-lab-fy.xml` on the lab branch `lab/tonight-read-answers-2026-10-21` |
| a3-1-stock-items-shape-lab | BRIDGE SHAPE LAB | `ans-R04-stock-items-shape-lab.xml` on the same branch |
| a3-2-stock-items-tieout-b | BRIDGE TIEOUT B | `ans-R06-stock-items-tieout-b.xml` on the same branch |
| a4-1-vouchers-shape-lab-2025-06 | BRIDGE SHAPE LAB | `ans-R03a-vouchers-shape-lab-2025-06.xml` on the same branch |
| a5-numbering-pilot-lab, -shape-lab, -tieout-b, -tieout-c | the company named in each | `ans-R08-voucher-type-numbering-<book>.xml` on the same branch |

Not in this sheet: the party mailing-details read and its workbook. They need a book with a keyed mailing address, which means keying by hand in a named synthetic company with a backup and a restore, done by the person at the lab computer; that is a separate write item and is not run here.

## B. Tool rows (the built tool of the release-candidate build)
- B1 `negative_stock`: BRIDGE NEGSTOCK LAB 2 for 1 April 2025 to 31 March 2026 returns one item, quantity -3 Nos, rate 120.00, value 360.00; to 31 May 2025 it returns -5 Nos with rate and value empty (shown as empty, not zero); BRIDGE TIEOUT C for the year returns `not_established` (an empty envelope). Screen step: compare the first with Tally's own Negative Stock screen.
- B2 `negative_ledgers`: BRIDGE SHAPE LAB for the year returns eleven ledgers on both sides in Tally's order; BRIDGE TIEOUT B and C return `not_established`. Screen step: compare with Tally's own Negative Ledgers screen.
- B3 `profit_and_loss` and `balance_sheet`: BRIDGE TIEOUT B for the year returns `closing_stock` applied (2,100.00) and the statements established; BRIDGE TIEOUT C and D return `closing_stock` declined with `opening_stock_not_shown_empty` and the statements refused as before.
- B4 `masters` with `kind=stock_items`: BRIDGE SHAPE LAB (eleven items, one with two dated HSN and GST entries) and BRIDGE TIEOUT B (three items whose default entries are withheld as `tally_default_entry`).
- B5 `vouchers` for BRIDGE SHAPE LAB on 5 June 2025: the journal's cost-centre rows (category Business Line; Trading -600.00, Assembly -400.00), and an entry with none carries no such key.
- B6 `voucher_presence` numbering check, one proposed voucher of the named type dated 1 June 2025, declaring that type manual, window 1 to 30 June 2025: BRIDGE SHAPE LAB with type `Shape Purchase` (Automatic) is refused with `presence_numbering_method_differs_from_book` and sends no voucher request; BRIDGE PILOT LAB with type `BRIDGE Sales` (Manual, duplicates prevented) proceeds and `numbering_read` shows the book's reading; a call declaring nothing manual returns `numbering_read` `not_read`.

Pass for B: each row equals the screen where one is named and equals the 7.1 result otherwise; any difference is recorded with both answers.
