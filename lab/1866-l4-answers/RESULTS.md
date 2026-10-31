# 1866-L4 results: an item invoice with a round-off ledger, read through `gst_by_counter_ledger` (Part of #1866)

Sheet `lab/1866-l4/WINDOWS-1866-L4.md` at f9d65b15 (SHA256.txt 3 of 3 equal). TallyPrime 7.2 Silver, BRIDGE GST LEDGER LAB as 1867-L3 left it. Build: pull request 1867's `bridge_mcp.exe` (head 796e94f1), sha256 `bf8cdf82e10bf22f693d3c5d6d89bd074b35b2dbf6dc2cbad748ac8609916f46` at every launch, serverInfo bridge-tally 0.6.0. Writes were screen accepts on the synthetic book; the book was restored from BK-1866L4 at the end. One sitting, one voucher, one release: PARTIAL.

## The recorded day (7b)

`gst_by_counter_ledger` for 2025-12-07 answered `complete`:

| field | value |
|---|---|
| `vouchers_observed` | 1 |
| `counter_ledgers` | `[]` |
| `set_aside.several_counter_ledgers.vouchers` | 1 (Purchase No. 60, 20251207) |
| `set_aside.several_counter_ledgers.debit_side_tax` | `cgst` −449.96, `sgst_utgst` −449.96 |
| `set_aside.several_counter_ledgers.credit_side_tax` | `{}` |
| `vouchers_cancelled_optional_or_post_dated` | 0 |
| `vouchers_without_gst_entry` | 0 |

This is the sheet's first reading (`set_aside.several_counter_ledgers`, 1 voucher, cgst −449.96 and sgst_utgst −449.96). No tax was attributed to `GS1 Round Off` or to `Purchase - Goods`.

## December (7c)

`gst_by_counter_ledger` for 2025-12-01..2025-12-31 answered `complete`: `vouchers_observed` 7; `counter_ledgers` Purchase - Goods 2 vouchers (cgst −1800, sgst_utgst −1800) and GS1 Rent 1 voucher (cgst −450, sgst_utgst −450), as 1867-L3 recorded; the same one voucher set aside under `several_counter_ledgers` (−449.96 and −449.96); `vouchers_cancelled_optional_or_post_dated` 2; `vouchers_without_gst_entry` 1. Every field the sheet names equals its EXPECT line.

## Marks (ledger_masters, port 9001)

| step | M | V | reused |
|---|---|---|---|
| 2 (marks-0) | 228 | 71 | false |
| 3 (ms0, after BK-1866L4) | 228 | 71 | false |
| 4 (after GS1 Round Off) | 229 | 71 | false |
| 5 (after the Purchase) | 229 | 72 | false |
| 8 (marks-1, after the reads) | 229 | 72 | false |
| 9 (after the restore) | 228 | 71 | false |

The reads wrote nothing (step 8 equals step 5). After the restore the marks equal ms0, the 19 ledger names equal step 2's, the December Day Book equals step 1's six vouchers, and no GS1 Round Off is listed.

## The voucher as keyed (step 5)

Purchase No. 60, Item Invoice mode, 7-Dec-25; supplier invoice GS1/P/0005 dated 7-12-2025; party SYN GS1 Supplier B; Purchase - Goods; GS1 Item 1 Nos at 4,999.60 (Tally offered 5,000.00; 4999.60 typed); Input CGST 449.96 and Input SGST 449.96 typed into empty amounts; GS1 Round Off with Rate (%) empty and 0.48 typed into an empty Amount. Tally computed no amount. Total 5,900.00; one bill allocation, New Ref GS1/P/0005, 5,900.00 Cr (Tally's defaults). GS1 Round Off was created at step 4 under Indirect Expenses with Type of Ledger `Not Applicable` (the default).

## The relay

lab/gs1-relay at e5dffc2b (SHA256.txt 21 of 21 equal, self-test ok) recorded steps 7a to 7c: 208 requests (118 POST /, 90 GET /status), every one forwarded, every answer 200, none refused. The recording is in `relay/` with its `manifest.tsv`; the SHA256 list of the recording as made is `relay-SHA256-original.txt`.

## Deviations, recorded

- Step 1: a click on the Day Book's side-bar F2 opened a single-date Change Date, not a period. Two typed dates moved the report's date (to 29-Nov-25, an empty day). Alt+F2 then set the period. Leaving the Day Book, the Gateway's current date read 29-Nov-2025; it was set back at once with the Gateway's F2 to 8-12-2025. Nothing was altered.
- Launch 1's first start exited after the server's initialize because the client's `step0` folder was missing; no tool call had been sent. The folder was made and the client restarted.
- Step 4: the sheet's two-Escape route met a `Quit ? Yes or No` prompt on the blank Ledger Creation after the accept. On the recorded word: Y, then Master Creation's List of Masters, then one Escape to the Gateway; marks then read exactly 229/71.
- Step 5: on the recorded word, the blank Purchase after the accept was left with one Ctrl+Q after a screenshot of its title, not with Escapes; the Gateway showed no dialog.
- Step 10: the restore had already put the current date back to Monday, 8-Dec-2025 (step 0's), so no F2 was pressed.

## Not covered

A round-off ledger typed for invoice rounding; a round-off on the credit side; item invoices of other shapes; Gold.
