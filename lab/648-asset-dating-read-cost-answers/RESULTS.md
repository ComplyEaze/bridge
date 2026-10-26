# Results: #648, the cost of dating an asset ledger's movement by trial-balance windows, through the built tools on two synthetic books, 26 Oct 2026. The sequence works: 1 + 12 + 5 + 1 calls date the movement to 5 Nov 2025 in 14.1 s, at about 0.67 s a window; on BRIDGE LARGE LAB the window's length does not change the cost

The sheet is `lab/648-asset-dating-read-cost` at 76423064215286d06364e354b77af85dbee165bc (`SHA256.txt` 3/3 OK). The run's review ordered it, and it is owner-free: reads only. The owner told the lab in its chat to work autonomously for the afternoon.

TallyPrime 7.2, Silver, Licensed, Education mode false, from `tally_status` at A0; the About screen was not opened.

The build is the staged `w1-2306d86d`. The client checked `bridge_mcp.exe` sha256 **92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524** before every call. It ran over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, port 9001, one new empty data folder per part, `BRIDGE_TERMS_ACCEPTED=true`, and posting, import and redaction unset. No request file was sent by curl.

**How the cost was measured.** The lab's client timed each tools/call round trip ("Round trip"). In the same server session it then called `egress_log`, which sends nothing to Tally, and took that call's `response_prepared` record:
- its `request_trail`: the requests sent to Tally, each with kind (post or status), request bytes and response bytes;
- `bytes_prepared`: the answer's size to the client.

The answer's own `evidence.duration_ms` is given as "Tally read (ms)".

**Stop rules.** No answer held `request_deadline_exceeded` or `response_size_limit_exceeded`. No dialog appeared. No call took more than 2 s. The marks were equal before and after in both parts.

## Part A: BRIDGE GST LEDGER LAB (100077, aae372c5-a894-491c-8843-5532e43d2d2f, books from 1-Apr-2025)
Part A was read between the items of its own sequence; none was under way. The company was loaded alone, with no dialog. C2 ran without MARKS.

| Step | Result |
|---|---|
| A0 | `list_companies`: exactly THE COMPANY, `verified_tuple`. Marks **M0 225 / V0 64** (`ledger_masters` snapshot, `reused` false) |
| A1 | `ledger_masters` basic, group Fixed Assets, ancestry: **1 row, GS1 Computer** (Fixed Assets, opening 0.00). It is the book's only asset ledger |
| A2 | `trial_balance` for the year: one page, 18 ledgers. GS1 Computer: opening 0.00, **debit -40000.00**, credit empty, closing -40000.00 |
| A3 | Twelve months: **GS1 Computer moved in November 2025 only** (debit -40000.00). In every other month its debit and credit are `present_empty` |
| A4 | Bisection of November, **five calls**: [1,15] moved, [1,8] moved, [1,4] did not, [5,6] moved, [5,5] moved. **Ended on 5 November 2025**, as EXPECTED |
| A5 | `vouchers` for 5-Nov-2025: complete, total 1. **Purchase 2, party SYN GS1 Supplier: SYN GS1 Supplier 47200.00; GS1 Computer -40000.00**; Input CGST -3600.00; Input SGST -3600.00, as EXPECTED |
| A6 | Marks **M1 225 / V1 64 = M0 / V0**. C10 done |

Recorded: inside a window shorter than a month, the tool returned debit and credit as `present` with value 0.00 where the ledger did not move. In the whole-month windows it returned `present_empty` instead. Both read as "no movement".

## Part B: BRIDGE LARGE LAB (100068, 2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab, books from 1-Apr-2026)
| Step | Window | Round trip | Rows | Answer bytes | Largest Tally response |
|---|---|---|---|---|---|
| B0 | marks **M0 210 / V0 29,930**, 5 ledgers | | | | |
| B1 | year 2026-04-01..2027-03-31 | 0.653 s | 5 | 6,677 | 8,918 |
| B2 | March 2027 | 0.646 s | 5 | 6,665 | 8,910 |
| B3 | 31-Mar-2027 | 0.902 s | 5 | 6,687 | 8,910 |
| B4 | year, again | 0.677 s | 5 | 6,677 | 8,918 |
| B5 | marks **M1 210 / V1 29,930 = M0 / V0**. C10 done | | | | |

On a book of 29,930 vouchers and 5 ledgers, **a window's length does not change a trial balance's cost**: a year, a month and a day all cost about 0.65 s and the same 26 requests. The one-day call (B3) was slower at 0.902 s, but the repeat of the year (B4) shows a call-to-call spread of about ±0.1 s; one sample is not a trend. This agrees with #648's design note that the cost follows the ledger count, not the window.

## Cost of the dating sequence (Part A)
| Stage | Calls | Round trip each | Requests each (posts + status) | Tally response bytes each |
|---|---|---|---|---|
| Year (A2) | 1 | 0.681 s | 26 (14 + 12) | 95,900 |
| Months (A3) | 12 | 0.663 to 0.703 s | 26 (14 + 12) | 94,712 to 95,900 |
| Bisection (A4) | 5 | 0.662 to 0.811 s | 26 (14 + 12) | 95,720 to 95,996 |
| Day's vouchers (A5) | 1 | 1.870 s | 22 (14 + 8) | 178,456 (largest single response 64,102) |
| **Total to date the movement** | **19** | **14.149 s** | **490** | **1,893,276 bytes** |

Each `trial_balance` call sends the same 26 requests whatever its window. In the trail, each post appears twice with a status check between, so the 14 posts are 7 reads each sent twice. On this 18-ledger book a window costs about 0.67 s.

The full per-call table is in `tables/cost-table.md` (and `.json`). Every call's `egress_log` answer is kept beside it.

## Limits (as the sheet says)
Not measured: a book of thousands of ledgers (#648's sizing case), several movements in one month, a disposal, Gold, or any release other than 7.2.

## In this commit
- `notes.txt`: UTC times, one line per step.
- `answers/A`, `answers/B`: every tool answer whole (with the client's timing) and each call's `egress_log` answer; the C2 and C10 observations with the /status answers.
- `tables/`: the cost table.
- `cost-log.txt`: the client's one-line log of each call.
- `call648.py`, `costtable.py`: the lab's client and table script, not product code. In `call648.py` the exe path, the item folder and the client name are replaced by placeholders, and two printed messages read "halt" where the run copy printed the stop word.
- `SCREENSHOTS.txt` (name and sha256) and `SHA256.txt`.

Not committed: the screenshots and the clients' data folders.
