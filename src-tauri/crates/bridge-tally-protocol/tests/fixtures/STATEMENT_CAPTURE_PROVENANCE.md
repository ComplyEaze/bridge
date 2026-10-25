# `statement_*` — provenance

Tally's built-in Balance Sheet and Profit and Loss, requested by report name (protocol reference §12a.1, §12a.11). These are the fixtures for `native_statement_reports.rs` (#692). Every file is a live capture, never hand-written.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-09-26.
  - The statement captures were taken between 11:14 and 11:17 IST. `tally_status` was healthy before and after that session.
  - The `BRIDGE READS LAB` group tree and Trial Balance were taken between 16:53 and 16:55 IST. `tally_status` was healthy before them. In the same minutes, the Balance Sheet and Profit and Loss were requested again and came back byte-identical to the 11:14 captures (the same SHA-256), so the book had not changed.
  - The `BRIDGE CORPUS DENSE` group tree, one-month Trial Balance and one-month Profit and Loss were taken between 17:10 and 17:12 IST, with `tally_status` healthy before and after. The one-month Balance Sheet requested with them was byte-identical to the morning's capture.
- **Encoding:** responses are **BOM-less UTF-16LE**, exactly as received. Requests are the exact bytes sent: **UTF-16LE with a BOM**. `.gitattributes` marks this tree `-text`.
- **Request shape:**
  - A statement request is §12a.1's built-in report request. Its text is what `render_native_statement_request` renders (a test asserts this).
  - The group tree and Trial Balance requests are the production `render_native_group_snapshot_request` and `render_native_trial_balance_request` text, taken from those functions' format strings.
- **Companies:** synthetic lab books only.
  - `BRIDGE READS LAB`: 3 vouchers.
  - `BRIDGE CORPUS DENSE`: a generated book of 29,900 vouchers.
  - The responses carry no company identity (§12a.1).

| file | bytes | sha256 | company | report | window |
|---|---|---|---|---|---|
| `statement_balance_sheet_fy_live.utf16le.xml` | 1822 | `6c13df52e220f5be9881af6c2b98bdf5554af2791c79465a6c5535058868bf45` | `BRIDGE READS LAB` | Balance Sheet | 2025-04-01 to 2026-03-31 |
| `statement_profit_and_loss_fy_live.utf16le.xml` | 678 | `b0c50c1676fbc83585ed52dac8942709b05f6a77ef5c5f2b20a51ad68339cbed` | `BRIDGE READS LAB` | Profit and Loss | 2025-04-01 to 2026-03-31 |
| `statement_balance_sheet_empty_month_live.utf16le.xml` | 1792 | `8e4c608d4864bb49a90abe78263846a58c0189d66693077f4f1719a79abb9213` | `BRIDGE READS LAB` | Balance Sheet | 2025-05-01 to 2025-05-31 (no activity) |
| `statement_balance_sheet_dense_month_live.utf16le.xml` | 1842 | `b06b1cef7a83457bed5d85e45c6d2ac6d36421a330764d786e05d5ac0ef40430` | `BRIDGE CORPUS DENSE` | Balance Sheet | 2025-05-01 to 2025-05-31 |
| `statement_balance_sheet_fy_request.utf16le.xml` | 758 | `82e01ff2547407d05c68c03bccdceaa3ea87a3a5b938e606baf0135fbd404daf` | `BRIDGE READS LAB` | request for the first row | as above |
| `statement_profit_and_loss_fy_request.utf16le.xml` | 762 | `0f2c8db6b265d2609d5e426f3fd33ef82df623c1f8b08e30c7d47b5799fd0a9b` | `BRIDGE READS LAB` | request for the second row | as above |
| `statement_groups_fy_live.utf16le.xml` | 54280 | `56c36221867d15e229f1f944d06be6d5f01bf69242151b362b3931740b9c14d8` | `BRIDGE READS LAB` | List of Groups (28 groups) | none (a master collection) |
| `statement_trial_balance_fy_live.utf16le.xml` | 11238 | `c85ddd0d95c4a0863a96e760bfe2eb19cd53227255b771a0b7b8143c0bfd2a2d` | `BRIDGE READS LAB` | native Trial Balance (7 ledgers) | 2025-04-01 to 2026-03-31 |
| `statement_groups_fy_request.utf16le.xml` | 1066 | `1eea1e25ad0a3bca798f3baf233afd7f9226dfaca253174674b7b0f15e8fe9cc` | `BRIDGE READS LAB` | request for the group tree | none |
| `statement_trial_balance_fy_request.utf16le.xml` | 1144 | `2b66b57d39579ec152f6c41ff96ea98185862af7cf55efdc536813bd470bec3d` | `BRIDGE READS LAB` | request for the Trial Balance | as above |
| `statement_groups_dense_live.utf16le.xml` | 54280 | `c4e1fbe12c1ec9b3e1c6f51216b47719ccc65b97b221af559fa4aae34f8c1368` | `BRIDGE CORPUS DENSE` | List of Groups (28 groups) | none (a master collection) |
| `statement_trial_balance_dense_month_live.utf16le.xml` | 150310 | `854f7d5d9af632d5e3d77c274cf0c56d0dcaf33c59fb668a8a13385b90b5d0a9` | `BRIDGE CORPUS DENSE` | native Trial Balance (123 ledgers) | 2025-05-01 to 2025-05-31 |
| `statement_profit_and_loss_dense_month_live.utf16le.xml` | 368 | `0c001bd969026ea82cd5e15af8bd48c8e896f31b89be9aa3ba308d4b67b8a62a` | `BRIDGE CORPUS DENSE` | Profit and Loss | 2025-05-01 to 2025-05-31 |
| `statement_groups_dense_request.utf16le.xml` | 1072 | `2bd1e72ec6d5fffe6a5802c86ef956fff5dbe6ea605e721e814cac18e71f993b` | `BRIDGE CORPUS DENSE` | request for the group tree | none |
| `statement_trial_balance_dense_month_request.utf16le.xml` | 1150 | `eee3528ba911a5a7c080949eb65b85acac78cb4a99fbe370e1f74a8b02a324ef` | `BRIDGE CORPUS DENSE` | request for the Trial Balance | 2025-05-01 to 2025-05-31 |
| `statement_balance_sheet_dense_month_request.utf16le.xml` | 764 | `0f893f2dce868cfee3c37a22007dba7df12a66bd2e65787049a4e53c3382c301` | `BRIDGE CORPUS DENSE` | request for the one-month Balance Sheet | 2025-05-01 to 2025-05-31 |
| `statement_profit_and_loss_dense_month_request.utf16le.xml` | 768 | `b50c2e7ec3dc132e85ec7df6b2b5a0b643374d4812ced547f17b76fee66caf5d` | `BRIDGE CORPUS DENSE` | request for the one-month Profit and Loss | 2025-05-01 to 2025-05-31 |

## What each capture establishes

- **Full-year Balance Sheet.** Five `BSNAME`/`BSAMT` pairs. `Current Liabilities` is `4250.00` and the `Profit & Loss A/c` line is `-4250.00`; the other three lines are empty. The name `Profit & Loss A/c` arrives with an `&amp;` entity.
- **Full-year Profit and Loss.** Two `DSPACCNAME`/`PLAMT` pairs: a `Cost of Sales :` heading with its main amount `-4250.00`, and `Purchase Accounts` with its sub-amount `-4250.00`.
- **Empty month.** The same five Balance Sheet lines with every amount element present and empty. This is the case a parser must not read as zero.
- **Heavy book, one month.** Large plain signed amounts (`222962422.38` and `-222962422.38`) with no digit grouping. The response took 0.23 s.
- **Tie to the native trial balance.** For the full year, the statement figures matched `trial_balance` for the same window (§12a.11).
- **One company's four sources.** The group tree, Trial Balance and both full-year statements are all `BRIDGE READS LAB` over one window.
  - `reports::statements` derives both statements from the first two and ties them to the last two.
  - Every compared line matched: Current Liabilities `4250.00`, the carried `Profit & Loss A/c` `-4250.00` and `Purchase Accounts` `-4250.00`.
  - Primary groups with no ledger are empty in Tally's statements and zero in the derivation.
- **A part-year window on the heavy book.** The group tree, Trial Balance and both statements are all `BRIDGE CORPUS DENSE` for May 2025, and every line ties. Confidence: partial (one book, one month).
  - In a part-year window a P&L ledger's Trial Balance covers the window only: the sales ledger opens May at `0.00`.
  - The year's earlier result sits in the `Profit & Loss A/c` ledger's opening, `109235760.65`.
  - Tally's Balance Sheet `Profit & Loss A/c` line, `222962422.38`, is that ledger's closing plus the P&L ledgers' closings: the derivation's carried line.
  - Current Assets ties at `-222962422.38`, and Sales Accounts at `113726661.73`, the window's movement.
- **Screening.** The heavy book's 123 ledgers are generated names (`DN Party 001` to `DN Party 120`, `DN Sales`, `Cash`, `Profit & Loss A/c`). All captures carry no GSTIN, PAN, contact or address field.

## Known limits

- One release (7.1), Silver, two synthetic books, one full year and one month.
  - The full-year statements and the one-month heavy-book Balance Sheet were each run twice, hours apart, with identical bytes.
  - The one-month heavy-book Profit and Loss, each group tree and each Trial Balance were run once.
- No book here holds inventory, so no closing-stock line was observed.
- The explode flag, Education, Gold and foreign-currency books were not measured.

## `*_stock_book_*`: the inventory book (BRIDGE TIEOUT B), 21 Oct 2026

Tally's Profit and Loss and Balance Sheet by name, and the Trial Balance and group tree the statements are derived from, for one synthetic company with inventory integrated with accounts (#692). Every file is a live capture, never hand-written, taken read-only by the Windows lab and committed to the lab branch `lab/692-tieout-b-answers` (the first eight files at `20ef79c46c87d4b10571b2fd21f6a76375ccfc4e`, the inventory-flags pair at `8af487be217086536a863a60d5912a73089b08b3`) (its `MANIFEST.txt` holds the lab's hashes), then copied here byte for byte.

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, a lab instance, the company loaded alone, one request at a time, all answers HTTP 200.
- **Company:** the synthetic `BRIDGE TIEOUT B`: three stock items directly under Primary, one purchase (5 May 2025: 10 at 100, 20 at 50, 5 at 200), one sale (20 May: A 4 at 150, B 10 at 80), freight 500 on 1 June, no opening stock; its last voucher is dated 1 June 2025, so a half-year window returned the same bytes as the full year and is not committed.
- **Encoding:** answers are BOM-less UTF-16LE as received; requests are the exact bytes sent, UTF-16LE with a BOM. The Trial Balance answer holds one `&#4;` character reference and the group tree 30 of them plus 82 raw U+0003 characters inside `PARENTSTRUCTURE`: kept as received.
- **Request shape:** the statement requests are §12a.1's built-in report request with this company's name; the Trial Balance and group requests are the production texts of the neighbouring `statement_*` fixtures with only the company name changed.

| file | bytes | sha256 | report | window |
|---|---|---|---|---|
| `statement_profit_and_loss_stock_book_fy_live.utf16le.xml` | 1920 | `d5dcdea6d99839492ddbdd17a0c809f8d25d2d415f523f5db7b8897e4c736835` | Profit and Loss | 2025-04-01 to 2026-03-31 |
| `statement_balance_sheet_stock_book_fy_live.utf16le.xml` | 1840 | `fb590bb683df71b661ca1a203a55a1aa6952e97d591cd2cf7c94d108bcaf9ea6` | Balance Sheet | 2025-04-01 to 2026-03-31 |
| `statement_trial_balance_stock_book_fy_live.utf16le.xml` | 13476 | `7ff82ce27ed95949fae25a08a4df4268542c2bfd3c4a77fb0719a05a0281a4ec` | native Trial Balance (9 ledgers) | 2025-04-01 to 2026-03-31 |
| `statement_groups_stock_book_live.utf16le.xml` | 54280 | `48678d9701f6b169bc03dfb1521373eee79bce2ab949940d2252502085ea033c` | List of Groups (28 groups) | none (a master collection) |
| `statement_profit_and_loss_stock_book_fy_request.utf16le.xml` | 760 | `2da98ea3fc23fea1fb121914296bb41676397306c1497b390295e1743eb9045b` | request for the Profit and Loss | as above |
| `statement_balance_sheet_stock_book_fy_request.utf16le.xml` | 756 | `fcc94a8a4b6bea5019d7cfb28b1469e6894e803aff8af5748d8b0c7e6117a10c` | request for the Balance Sheet | as above |
| `statement_trial_balance_stock_book_fy_request.utf16le.xml` | 1142 | `e25f2e4e5892ff497990726fb73451eda5c28fa5c8e186bcc6d92136cc0f8249` | request for the Trial Balance | as above |
| `statement_groups_stock_book_request.utf16le.xml` | 1064 | `130d5ffa6295243da2751e1cc1845e31da3763e2790276b2cc0dd9dc46e51757` | request for the group tree | none |
| `company_inventory_flags_stock_book_live.utf16le.xml` | 4886 | `c58d3ba017b17bc71120a6b0e61bdd58ad3f2181011120cd65eb06e2961b4b24` | company inventory flags (`ISINTEGRATED` `Yes`, `ISINVENTORYON` `Yes`, 3 stock items) | none (a company object) |
| `company_inventory_flags_stock_book_request.utf16le.xml` | 1638 | `8f964258be6506b08782b87bb548871e3fa18b284213baf855428f71d44f210d` | request for the company inventory flags | none |

What they establish (confidence: partial; one book, one release, no opening stock):

- **Profit and Loss.** Flat pairs in this order: `Sales Accounts` main `1400.00`; `Cost of Sales :` main `-900.00`; `Opening Stock` empty; `Add: Purchase Accounts` sub `-3000.00`; `Less: Closing Stock` sub `-2100.00` (the 5th `PLAMT`, sub column, main column empty); `Indirect Expenses` main `-500.00`. There is no Gross Profit leaf. Closing stock carries the sign of the purchase line (a debit is negative): `-900.00` is `-3000.00` less `-2100.00`.
- **Balance Sheet.** `Capital Account` `50000.00`, `Current Liabilities` `3000.00`, `Profit & Loss A/c` empty, `Current Assets` `-53000.00`. No closing-stock leaf: stock sits inside Current Assets.
- **Trial Balance and groups.** Seven ledgers with the figures above plus `Cash` and `Profit & Loss A/c` at `0.00`. Current Assets derived from them is `-50900.00`, `2100.00` short of Tally's: the closing stock.
- **Not established:** opening stock, a costing method, a non-integrated book, a window spanning two financial years, more than one purchase rate for an item.

## `statement_groups_stock_book_72_live.utf16le.xml`: List of Groups on TallyPrime 7.2 (BRIDGE TIEOUT B), 25 Oct 2026

| file | bytes | sha256 | report | window |
|---|---|---|---|---|
| `statement_groups_stock_book_72_live.utf16le.xml` | 54280 | `493256b22e131097da479f67d4280a272bec855d20c6314c6b37dfad530180c0` | List of Groups (28 groups) | none (a master collection) |

- **Source.** The lab's 7.2 read pass of 25 Oct 2026: branch `lab/72-reads-answers-b` at `f3c9c7361402`, file `lab-evidence/72-reads-answers/answers/a1-4-groups-tieout-b.72.xml`, with `RESULTS.md` and `evidence/send-log.txt` beside it. Host: TallyPrime Release 7.2 (Latest), Silver, `tally.exe` 1.1.7.2, one process on port 9001. The book is `BRIDGE TIEOUT B`, loaded alone; the company list and the marks read before and after the group were equal.
- **Request.** The request is `statement_groups_stock_book_request.utf16le.xml` of the section above (byte for byte, sha256 `130d5ffa6295243da2751e1cc1845e31da3763e2790276b2cc0dd9dc46e51757`), sent once with one `curl.exe --data-binary` line, HTTP 200, size equal to its `Content-Length`. The answer was saved as received.
- **What it establishes (confidence: partial; one send, not repeated).** The same size, the same 28 `GROUP` rows in the same order and every value equal to `statement_groups_stock_book_live.utf16le.xml` (TallyPrime 7.1, the section above). In every row the request's computed `BRIDGECOMPANYGUID` comes before `PRIMARYGRPPARENT`; on 7.1 it came after it. The reorder was seen in this one send and was not repeated.
