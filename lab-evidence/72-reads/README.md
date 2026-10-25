# Lab run sheet: the 0.7.0 reads on TallyPrime 7.2

All live evidence so far is from TallyPrime 7.1 Silver. Order: part A first, a regression pass of the committed requests (the request files are in `requests/`, UTF-16LE with a BOM, checked against `SHA256.txt` with `shasum -c`); then part B, the tool rows, run through the built tool of the release-candidate build and never as a hand-sent request.

Rules: synthetic BRIDGE companies only; one request at a time; every answer saved as received (BOM-less UTF-16LE) with size and sha256; a failed request is reported as failed, never as an empty answer; read the company's voucher mark before and after every group (reads must not move it); no key and no write of any kind; a company that is not loaded is skipped and said so. Do not edit a request file.

## A. Regression pass (shape drift, 7.1 to 7.2)
Send each file to the company named inside it, save the answer, and compare it with the 7.1 answer named below by structure: the same root, the same element names and order, the same number of rows, the same number format, sign and empty form. Equal bytes are a pass; any difference is a finding to list (both answers kept), not a failure.

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

Not in part A: the party mailing-details request, because its source book is not loaded; row B1 needs a book with a keyed mailing address (key one by hand in a synthetic company and say so).

## B. Tool rows (the built tool of the release-candidate build)
- B1 `ledger_masters` with `fields=compliance` on a book with a keyed mailing address: the entry in force on the date, the PIN code, state and country; under `mask_parties` the mailing name, each address line and the flat address are masked and the PIN code, state and country are not. Then the party/ledger master workbook export once, opened in Excel: the same values, five new columns at the right end.
- B2 `negative_stock`: BRIDGE NEGSTOCK LAB 2 for 1 April 2025 to 31 March 2026 returns one item, quantity -3 Nos, rate 120.00, value 360.00; to 31 May 2025 it returns -5 Nos with rate and value empty (shown as empty, not zero); BRIDGE TIEOUT C for the year returns `not_established` (an empty envelope). Compare the first with Tally's own Negative Stock screen.
- B3 `negative_ledgers`: BRIDGE SHAPE LAB for the year returns eleven ledgers on both sides in Tally's order; BRIDGE TIEOUT B and C return `not_established`. Compare with Tally's own Negative Ledgers screen.
- B4 `profit_and_loss` and `balance_sheet`: BRIDGE TIEOUT B for the year returns `closing_stock` applied (2,100.00) and the statements established; BRIDGE TIEOUT C and D return `closing_stock` declined with `opening_stock_not_shown_empty` and the statements refused as before.
- B5 `masters` with `kind=stock_items`: BRIDGE SHAPE LAB (eleven items, one with two dated HSN and GST entries) and BRIDGE TIEOUT B (three items whose default entries are withheld as `tally_default_entry`).
- B6 `vouchers` for BRIDGE SHAPE LAB on 5 June 2025: the journal's cost-centre rows (category Business Line; Trading -600.00, Assembly -400.00), and an entry with none carries no such key.
- B7 `voucher_presence` numbering check: an Automatic book is refused with `presence_numbering_method_differs_from_book`; a Manual book that prevents duplicates proceeds; a call declaring nothing manual returns `numbering_read` `not_read`.

Pass for B: each row equals the screen where one is named and equals the 7.1 result otherwise; any difference is recorded with both answers.
