# Results: #692 D3, five experiments on BRIDGE TIEOUT D, each from the same starting state, on TallyPrime 7.2, 27 Oct 2026

The request is `lab-evidence/692-opening-stock-d3-requests/README.md` at 6f4f7869909bbff7081ac29f6faa3ebdc3eebfdb (MANIFEST.sha256: 14 of 14 equal; request-ID census ok 3, skipped 7, FAIL 0).

## Setting
- TallyPrime 7.2 (Latest), Silver, licensed. One tally.exe, listening on port 9001, at step 0 and at the close; GET /status 200 both times.
- BRIDGE TIEOUT D, company number 100075, GUID 6a9518b2-a88c-417c-a700-40243f9d9119, loaded alone; no prompt on opening it on 7.2. Gateway at step 0: current period 1-Apr-25 to 31-Mar-26, current date 15-Oct-2025.
- Reads by curl, one at a time: 72 sends, all HTTP 200 with the size equal to CONTENT-LENGTH (`send-log.txt`). Every answer is in `answers/`.
- BK3: `TDBK1800_100075.001`, 29,493 bytes, sha256 b297593b6a4d27d6be466771c65d303f92dc0e70ab9c3a0a0cd21800d676bf1a. It stays on the lab PC.
- After each of the five restores (R1 to R5) the marks read 4 / 213 and the year's Profit and Loss and Balance Sheet answers were byte-equal to those on S.

## Marks (ALTVCHID / ALTMSTID)
| point | V | M |
|---|---|---|
| S (step 0, BK3, X0, X0s, after each restore) | 4 | 213 |
| X1 after the stock item | 4 | 214 |
| X1 after the sale | 5 | 214 |
| X2 after the sale | 5 | 213 |
| X3 after the group | 4 | 214 |
| X3 after the item alteration | 4 | 215 |
| X4 after the costing change | 4 | 214 |
| X5 after the opening change | 4 | 214 |

The marks were equal before and after every read group.

## X0: July to September 2025 on S
The Balance Sheet for 1 Jul to 30 Sep 2025 shows the position at 30 September: Current Assets 4,240.00 and P&L A/c 640.00. It is not the year's answer (1,848 bytes against 1,850). The window's Profit and Loss carries Opening Stock and Less: Closing Stock both at 2,640.00, and the Stock Summary from 1 Apr to 30 Sep shows Lab Gadget 22 Nos at 120.00 = 2,640.00.

## The five experiments (year 1 Apr 2025 to 31 Mar 2026)
| | Opening Stock | Less: Closing Stock | Cost of Sales | Sales | P&L A/c (BS) | Current Assets | Balances? | Stock Summary | Negative Stock |
|---|---|---|---|---|---|---|---|---|---|
| S | 1,000.00 | 2,040.00 | 1,560.00 | 2,850.00 | 1,290.00 | 4,890.00 | yes | Lab Gadget 17 Nos @ 120.00 = 2,040.00 | empty |
| X1 negative stock | 1,000.00 | 2,040.00 | 1,560.00 | 3,300.00 | 1,740.00 | 5,340.00 | yes | + Lab Short -3 Nos, no value | Lab Short -3 Nos |
| X2 item sold out | 1,000.00 | line present, amounts empty | 3,600.00 | 7,100.00 | 3,500.00 | 7,100.00 | yes | empty envelope | empty envelope |
| X3 item in a group | 1,000.00 | 2,040.00 | 1,560.00 | 2,850.00 | 1,290.00 | 4,890.00 | yes | Lab Group 2,040.00, no quantity | empty |
| X4 FIFO | 1,000.00 | 2,240.00 | 1,360.00 | 2,850.00 | 1,490.00 | 5,090.00 | yes | Lab Gadget 17 Nos @ 131.76 = 2,240.00 | empty |
| X5 opening edited | 1,100.00 | 2,096.67 | 1,603.33 | 2,850.00 | 1,246.67 | 4,946.67 | **no** (4,846.67 against 4,946.67) | Lab Gadget 17 Nos @ 123.33 = 2,096.67 | empty |

- **X1.** Negative Stock lists `Lab Short` at -3 Nos. The negative item carries no value, so Closing Stock stays at 2,040.00. The nett profit rises by the sale's 450.00, and the Balance Sheet still balances. Tally's Stock Summary and Negative Stock Items screens show the same: Lab Short (-)3 Nos.
- **X2.** The Profit and Loss answer keeps a `Less: Closing Stock` line, with both its amount fields empty: not zero and not absent. The Stock Summary answer is an empty envelope, `<ENVELOPE></ENVELOPE>`, 46 bytes, byte-equal to the empty Negative Stock answer.
- **X3.** The Profit and Loss and Balance Sheet answers are byte-equal to S. There is one "Opening Stock" line and one "Less: Closing Stock" line, under the same names. The Stock Summary shows only the group `Lab Group`, with value 2,040.00 and no quantity (the group's "Should quantities of items be added" is No, the screen's default). The ledger answer has the same rows; only its CMPINFO count lines changed.
- **X4.** Tally offers the costing field without any feature switch. It sits under the item screen's own "More Details" (Ctrl+I) as Costing Method, which reads Avg. Cost. With FIFO, Closing Stock is 2,240.00, which equals the request's first-in first-out figure. Cost of Sales falls by 200.00, and Current Assets and the nett profit each rise by 200.00.
- **X5.** Raising the opening value by 100.00 changes Opening Stock, the average cost (123.33) and Closing Stock. Tally's Balance Sheet screen then shows **"Difference in opening balances" 100.00** and totals 4,946.67 on both sides. The Balance Sheet answer carries no line for that difference, so its liability lines sum to 100.00 less than its Current Assets. The ledger answer is byte-equal to X4's (the same CMPINFO count change).

## Unplanned on the screen
- Each Sales voucher (X1, X2) passed through Tally's Dispatch Details and Party Details sub-screens, left blank with their defaults as the book's own sales were keyed. Each then went through the Bill-wise Details sub-screen for `Customer` (a bill-by-bill ledger), accepted with Tally's defaults: New Ref, named by the voucher number, for the full amount. This adds a bill reference the request did not plan; the restore after each experiment removes it.
- X2's rate was offered by Tally as 250.00, the book's last sale rate, which equals the request's.

## Not measured
The items the request lists: a split or carried-forward second year, godowns, batches, GST, and another release. Also not measured: the Bill-wise reference's effect on any outstanding read.

## Files
- `answers/`: every answer as received, each with its response headers.
- `requests/m-marks-D.utf16le.xml`: the marks template with the company GUID filled in.
- `send-log.txt`: each send in order, with the request's sha256, curl's exit code, status, CONTENT-LENGTH, and the answer's file, bytes and sha256.
- `notes.txt`: one line per step, in UTC.
- `restore-plan.txt`.
- `SCREENSHOTS.txt`: the screen captures by name, bytes and sha256. The captures, BK3 and the data-folder listings stay on the lab PC.
