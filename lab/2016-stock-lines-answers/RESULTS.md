# Results: #2016 live row, `vouchers` with `stock_lines` on the class shape, 3 Nov 2026

**Every EXPECT holds, and no Fail line was met.** With `stock_lines`, one window answered all twelve rows. Lines were read for the Stock Journal, the Delivery and Receipt Notes and the Credit Note, `[]` for the accounts-only Credit Note, and `stock_lines_not_read` for Sales, Purchase, Debit Note and both own types, listed in `kinds_not_read`. The FETCH is exactly the sheet's, with the class COMPUTEs. One thing to look at: `next_offset` reads null on both pages of a 4-row page of 12.

Sheet: `lab/2016-stock-lines-sheet` at fbeeff7b. Build: head 1348b37d on fix/2016-stock-lines, run 41567860325, artifact 13201075359, zip digest equal, `bridge_mcp.exe` sha256 0521ed8ac29d82d5e3f95f2ad9525c61eadaa851758f014da10e75089a268400, equal to the package's smoke file, before and after. Posting off, new data folder. Requests went through a loopback relay that recorded every request and answer in `out/bodies/`; it changed nothing. [verified]

## Book and set-up
BRIDGE SHAPE LAB, the #2230 and #2016 capture book: synthetic, batches on, godowns Main Location and Factory Floor, TallyPrime 7.2 Silver, licensed, loaded alone. A backup was taken first by Tally's screen and is kept on the lab PC; it was not restored. Marks before V 112, M 292.

Set-up only, never evidence of its own shape. On 20 May 2026, an empty day before, seeded by direct XML one at a time, each `CREATED` 1 or `ALTERED` as named:
- Delivery Note and Receipt Note voucher types made active (`ALTERED` 2). Two own types, `Lab Own DN` under Delivery Note and `Lab Own CN` under Credit Note (`CREATED` 2). M 300.
- **Twelve vouchers**, all on Hydrochloric Acid 33pc, 1 Kgs at 30.00 unless named:
  1. Stock Journal, 2 Kgs, Main Location to Factory Floor, batch H-2504-A.
  2. Delivery Note.
  3. Receipt Note, into Factory Floor, H-2504-B.
  4. Credit Note.
  5. `Lab Own DN`.
  6. `Lab Own CN`.
  7. Credit Note, accounts only.
  8. Delivery Note, then cancelled on Tally's screen (Alt+X).
  9. Delivery Note with `ISOPTIONAL` Yes.
  10. Sales.
  11. Purchase.
  12. Debit Note.
- V 124 after the seed, 125 after the cancel.

## Steps
| step | call | answer | seconds |
|---|---|---|---:|
| 1 | marks, `list_companies` | V 125, M 300 | 0.16 |
| 2 | `vouchers` 20 May | complete, 12 rows, no stock key anywhere | 2.62 |
| 3 | the same, `stock_lines` | complete, 12 rows | 2.39 |
| 4 | step 3 again | equal | 2.30 |
| 5 | `stock_lines` + `ledger` | refused `voucher_stock_lines_excludes_ledger` | 0.09 |
| 6 | `stock_lines` + `voucher_type` Delivery Note | complete, 3 rows | 2.66 |
| 7 | `stock_lines`, `limit` 4; then `offset` 4 with the snapshot | 4 rows each; second `reused` true | 2.39, 0.69 |
| 9 | `stock_summary` 31 May, a Delivery Note seeded, again | below | 1.50, 1.13 |
| 10 | marks | V 126, M 300 | |

[verified]

**Step 3's request as sent** (`raw/step3-window-request.utf16le.xml`, 3,122 B): the FETCH is exactly `DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,NARRATION,GUID,ALTERID,MASTERID,ISCANCELLED,ISOPTIONAL,ISPOSTDATED,ALLINVENTORYENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE`, with the voucher type COMPUTEs beside it: type GUID, `$ReservedName`, and the class words. One window; twelve rows; each row's date, number, type and GUID equal to step 2's, in order. The optional voucher is in both reads. Step 2 carries no `stock_lines`, `stock_lines_not_read` or `kinds_not_read` key. [verified]

## Step 3, row by row
| row | ALTERID | `voucher_type_reserved_name` | `voucher_class` | reads |
|---|---:|---|---|---|
| Stock Journal 1 | 113 | Stock Journal | absent | in: 2 Kgs, Factory Floor / H-2504-A, rate and amount `empty`; out: 2 Kgs, Main Location / H-2504-A, 30.00/Kgs, 60.00 |
| Delivery Note 1 | 114 | Delivery Note | absent | out: 1 Kgs, Main Location / H-2504-A, 30.00 |
| Receipt Note 1 | 115 | Receipt Note | absent | in: 1 Kgs, Factory Floor / H-2504-B, -30.00 |
| Credit Note 1 | 116 | Credit Note | Credit Note | in: 1 Kgs, Main Location / H-2504-A, -30.00 |
| Lab Own DN 1 | 117 | empty | absent | `stock_lines_not_read` |
| Lab Own CN 1 | 118 | empty | Credit Note | `stock_lines_not_read` |
| Credit Note 2, accounts only | 119 | Credit Note | Credit Note | `stock_lines` `[]`, no `stock_lines_unreadable` |
| Delivery Note 2, cancelled | 125 | Delivery Note | absent | `stock_lines_unreadable` `stock_line_missing`; `cancelled` true |
| Delivery Note 3, optional | 121 | Delivery Note | absent | out: 1 Kgs, Main Location / H-2504-A, 30.00; `optional` true |
| Sales 1 | 122 | Sales | Sales | `stock_lines_not_read` |
| Purchase 2 | 123 | Purchase | Purchase | `stock_lines_not_read` |
| Debit Note 1 | 124 | Debit Note | Debit Note | `stock_lines_not_read` |

- `kinds_not_read`: Debit Note, Lab Own CN, Lab Own DN, Purchase, Sales, one voucher each.
- `stock_summary_tie` `not_compared`, with `stock_summary_tie_reason` beside it.
- No row reads lines where it should not. Each own type answers an empty reserved name. [verified]

**Against Tally's screen:** the Stock Journal shows Source (Consumption) Main Location 2.000 Kgs at 30.00/Kgs, 60.00, and Destination (Production) Factory Floor 2.000 Kgs with no rate or amount. That screen does not show the batch. The Day Book shows Receipt Note 1.000 Kgs inward, the Delivery Notes 1.000 Kgs outward, and the cancelled one as "(cancelled)". No figure differs and no direction contradicts the screen. [verified]

## Steps 4 to 7
- **Step 4** equals step 3 apart from `read_at`, `duration_ms`, the snapshot `id` and `read_at`, and three `ms` timings. Every row is equal. [verified]
- **Step 5** was refused. The relay shows no marks, census or window read in that call's second, only the company-extent and `/status` reads Bridge makes before any call. [verified]
- **Step 6** returns Delivery Notes 1, 2 and 3, read as in step 3. `Lab Own DN` is not included: the filter matched the type name, not the reserved name. `kinds_not_read` is `[]`. [verified]
- **Step 7:** page one holds Stock Journal, Delivery Note 1, Receipt Note, Credit Note 1. Page two holds Lab Own DN, Lab Own CN, Credit Note 2, Delivery Note 2, with `snapshot.reused` true and the same snapshot id. For page two the relay shows only the marks pair, beside the extent and `/status` reads, and no census or window read. `kinds_not_read`, `stock_summary_tie` and its reason are equal on both pages and to step 3's. [verified]
  - **Seen:** `next_offset` is null on both pages, though `total` is 12 and each page holds 4.

## Step 8, sizes on the wire
Step 3's window request is 3,122 B; its answer is 370,470 B, UTF-16LE, read as a pair. The twelve VOUCHER elements take 367,286 B; the CMPINFO header and envelope take 3,184 B. Per voucher: from 35,152 B (Stock Journal, two lines) to 36,212 B (Debit Note); 5,066 B for the accounts-only Credit Note and 4,878 B for the cancelled Delivery Note, each of which holds only an empty `ALLINVENTORYENTRIES.LIST`. Eleven stock lines carry an item: about 33 KiB a line. `evidence/sizes.txt`. [verified]

## Step 9, Stock Summary and a note
Descaler Concentrate had no movement. `stock_summary` as of 31 May 2026, the month's last day the tool takes: closing quantity `none_sent`, value null. Total 3,060.01, `matched`.

Then one Delivery Note was seeded on 20 May, 1 Kgs out of Main Location: `CREATED` 1, V 126. Read again: Descaler's closing quantity in the stock item list is -1.000 Kgs, value null. The total is still 3,060.01. So the note moved the item's quantity and no value. [verified]

Seen in the Day Book afterwards: the new Delivery Note took No. 3, and the optional one shows as "(Optional) 4". Step 3 read the optional voucher as No. 3. [verified]

## Step 10
Marks V 126, M 300. Since step 1 (V 125) they moved only by step 9's voucher. [verified]

## Kept
Tool answers, the requests and answers as sent (`out/`), the step 3 raw answer unchanged (`raw/step3-window-answer.utf16le.xml`, sha256 0385b11a..., no company number in it), both `stock_summary` answers, marks, sizes. Synthetic names only.

## The book now
BRIDGE SHAPE LAB holds the seeded masters and thirteen vouchers on 20 May 2026: V 126, M 300. Tally's current date is 20-Apr-2026, as at the start. Company shut, nothing loaded, one tally.exe on 9001. The backup taken before the seed is on the lab PC; it can restore the book on request.

**Confidence:** one book, one sitting, seeded vouchers, 7.2: PARTIAL.
