# Results: #692 Profit and Loss and Balance Sheet over a window that spans two financial years, on master fd6f7c0d and TallyPrime 7.2, 27 Oct 2026. The spanning window is established and carries 2025-26's result once

The sheet is `lab/692-two-year-window/README.md` at 006c2cd2f2906bafe1aaacb5d7ee885ca4e8a3dd (SHA256.txt: 3 of 3 equal).

## Build and setting
- `bridge_mcp.exe` of master fd6f7c0d, sha256 63641231787492201133443cb1a887705c7137f495a9205ffd33c2d0068f7484, before the first call and after the last. It reports `bridge-tally` 0.6.0.
- One launch over MCP stdio, 12:35:43 to 13:09:05 UTC (`client-log.txt`). The settings were `BRIDGE_TERMS_ACCEPTED` true, host 127.0.0.1, port 9001 and a new data folder; posting, import, max bytes and redaction were unset. The server's error stream stayed empty.
- TallyPrime 7.2, Silver.
- The company: BRIDGE TIEOUT B, GUID d64eda55-da7c-4106-a171-3e3cb6cdd331, company number 100040, books from 1 April 2025, loaded alone. Gateway at step 0: current period 1-Apr-25 to 31-Mar-26, current date 1-Jun-2025.

## Marks
| point | answer | M | V |
|---|---|---|---|
| step 0 | 001 | 218 | 3 |
| after BK0 | 005 | 218 | 3 |
| after J0 | 006 | 218 | 4 |
| after the period change | 007 | 218 | 4 |
| after the date change | 008 | 218 | 4 |
| after J1 | 009 | 218 | 5 |
| after the set-back | 010 | 218 | 5 |
| before the restore | 019 | 218 | 5 |
| after the restore | 021 | 218 | 3 |

`reused` was false in every answer. No read moved a mark.

## The book
- BK0: `TDBK1800_100040.001`, 34,632 bytes, sha256 7846fad660eb23cb70f511669b813aa7cc87db9523f8b09101d3ac89e9dfa577. It stays on the lab PC.
- J0: Journal, 20 March 2026, Dr Lab Bank / Cr Lab Freight 250.00, narration `LAB 2Y J0`.
  - Journal's ledger list did not offer Lab Bank; it was chosen from Show More.
  - After the Lab Bank amount, Tally opened its Bank Allocations sub-screen (Cheque/DD, Inst No. blank, Inst Date 20-Mar-26). Tally's Bank Allocations sub-screen, defaults accepted.
- J1: Journal, 10 April 2026, Dr Lab Freight / Cr Lab Bank 100.00, narration `LAB 2Y J1`.
  - First refused at the date: "Date cannot be above the current period (20-Mar-26)". There was no prompt to create a 2026-27 year.
  - To key it, the company's period was set to 1-4-2025..31-3-2027 and its current date to 10-4-2026. J1 was keyed (number 1 of the new year), and both were set back to 1-4-2025..31-3-2026 and 1-6-2025.
  - Tally's Bank Allocations sub-screen, defaults accepted (Cheque, Cheque range Not Applicable, Inst Date 10-Apr-26).
  - The marks were read after each change; only the vouchers moved V.
- Report periods for the screens of steps 4 and 7 were set with F2 inside each report.

## The product's answers against Tally's screens
| window | tool | state | result | Tally's screen | |
|---|---|---|---|---|---|
| 1 Apr 2025 to 31 Mar 2026, before J0 (step 1) | profit_and_loss 002 | observed | net 0; closing stock applied -2100.00 | | as of 26 Oct |
| | balance_sheet 003 | observed | carried 0 | | |
| the same, after J0 and J1 (step 4) | profit_and_loss 011 | observed | gross 500, net 250; closing stock applied -2100.00 | Gross Profit 500.00, Nett Profit 250.00, Closing Stock 2,100.00 | equal |
| | balance_sheet 012 | observed | carried 250 | Profit & Loss A/c 250.00, totals 53,250.00 | equal |
| 1 Apr 2026 to 30 Apr 2026 (step 5) | profit_and_loss 014 | not_established | `tally_balance_sheet_differs`; closing stock declined `opening_stock_not_shown_empty` | Opening Stock 2,100.00, Closing Stock 2,100.00, Nett Loss 100.00 | refused |
| | balance_sheet 015 | not_established | `tally_balance_sheet_differs` | Profit & Loss A/c 150.00 (opening 250.00, current (-)100.00), totals 53,150.00 | refused |
| 1 Apr 2025 to 30 Apr 2026, spanning 31 March (step 6) | profit_and_loss 016 | observed | gross 500, net 150; closing stock applied -2100.00 | Gross Profit 500.00, Nett Profit 150.00, Closing Stock 2,100.00 | equal |
| | balance_sheet 017 | observed | carried 150 | Profit & Loss A/c 150.00 (opening blank, current 150.00), totals 53,150.00 | equal |

- In the spanning window, 2025-26's +250.00 is carried **once**: 150 = +250 (J0) - 100 (J1). Not twice (400), not dropped (-100).
- April 2026 alone was refused as the sheet predicted. The tool read Tally's Opening Stock 2,100.00, declined the closing stock, and its derived Current Assets (-51050) differed from Tally's (-53150.00) by the stock. It gave no figure.
- The Profit & Loss A/c ledger's closing reads 0.00 in the year and spanning windows and 250.00 in the April window; the carried figure equals the screen in each established window.
- `stock_summary`: as of 20260331 value_total_matched at -2100.00 (004, 013, 024); as of 20260430 refused `stock_summary_as_of_not_measured`, with no request sent (018).
- Line by line: `comparison-step8.txt`.

## Restore (step 9)
- BK0 restored with Accept, then Replace. Loaded alone, the Gateway showed 1-Apr-25 to 31-Mar-26 and 1-Jun-2025, as at step 0.
- `list_companies` (020) equal to step 0's; marks 218 / 3 (021); `profit_and_loss`, `balance_sheet` and `stock_summary` (022-024) equal to step 1's (002-004) except `read_at`, `evidence` and `snapshot`.
- Select Company's list now gives 100040 the period 1-Apr-25 to 31-Mar-27 (it gave 31-Mar-26 at step 0). The company's own current period is 1-Apr-25 to 31-Mar-26.

## Verdict
PASS by the sheet: every answer whole with its sha256, every window's screen recorded, the step 8 comparison written for each statement and window, the marks unchanged by every read, and the book and its current period restored as at step 0.

## Close
- The server stopped with code 0. The company was shut, leaving no company loaded. Port 9001 is owned by the same tally.exe as at step 0, and GET /status returned 200.
- A second tally.exe was also running. It started at 12:43:31 UTC, when the lab's screen-control tool was asked to bring TallyPrime forward while another window held the foreground. It never owned 9001 and showed its own Gateway with the demo company loaded. Every read and keyed screen above is the 9001 instance's.

## Files
- `answers/000-024`: the tool answers whole.
- `client-log.txt`.
- `notes.txt`: UTC times.
- `comparison-step8.txt`.
- `SCREENSHOTS.txt`: the screen captures by name, bytes and sha256. The captures and BK0 stay on the lab PC.
