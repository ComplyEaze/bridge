# Results: #899-P v2 Part D, the census window ladder on BRIDGE CORPUS DENSE (synthetic), 29 Oct 2026. **With the product's last span (the one ending at the mark, 5,324 ids wide), every window held rows: a day answered 82 rows in 0.14 s, a month 2,542 rows in 1.81 s and the financial year 5,324 rows (every id of the span) in 13.22 s, all under 20 s. The full span below it over the financial year (8,192 rows, byte-equal to v1's W3 answer) took 14.78 s, against 24.99 s in v1. W1 again took 0.14 s. Marks unchanged.**

| step | window | span | request bytes | rows | answer bytes = CONTENT-LENGTH | bytes per row | time_starttransfer | time_total | answer sha256 | census_read line |
|---|---|---|---|---|---|---|---|---|---|---|
| m0 | (marks) | - | 982 | 1 company | 3406 = 3406 | - | 0.009 s | 0.011 s | fcc9086dfa112813c447d23e98eb4500c9a1badbe983d3f8d0c0ceff11dc8430 | V0 29900, M0 328 |
| W1 | day: 20260331 | first, **last**: (24576, 29900], width 5324 | 1540 | 82 | 202614 = 202614 | 2471 | 0.138 s | **0.141 s** | f66fecbab6732128b127bc282dd7dbb46d34905057fdd95ce4064449e3fc23cf | census ok rows 82, all four counts 0, min_alterid_offset 5243 max 5324 |
| W2 | month: 20260301 to 20260331 | first | 1540 | 2542 | 6190254 = 6190254 | 2435 | 1.780 s | **1.810 s** | 0c59d290fe27783ece96f79a2b731f650956ae93eb6d79dbc4ec610d9cd6adf2 | census ok rows 2542, all four counts 0, min 2783 max 5324 |
| W3 | FY: 20250401 to 20260331 | first | 1540 | 5324 | 12960794 = 12960794 | 2434 | 13.121 s | **13.223 s** | 87cc16c0ae4a42512c8d9acc1e4e95625a7b6f79cefa3242efb0c8f924d20108 | census ok rows 5324, all four counts 0, min 1 max 5324 |
| S2 | WIDEST = FY | second: (16384, 24576], full, before the first | 1540 | 8192 | 19925970 = 19925970 | 2432 | 14.550 s | **14.776 s** | 8d0b0c7470d6db7bc1fa8f72e7efb50f555b98fe3b710e2bceda98a363342d2c | census ok rows 8192, all four counts 0, min 1 max 8192 |
| R1 | day (W1's file) | first | 1540 | 82 | 202614 = 202614 | 2471 | 0.132 s | 0.135 s | f66fecba... (byte-equal to W1) | census ok rows 82, all four counts 0 |
| m1 | (marks) | - | 982 | 1 company | 3406 = 3406 | - | 0.010 s | 0.011 s | fcc9086d... (byte-equal to m0) | V1 29900, M1 328 |

- **Marks:** V1 = V0 (29900), M1 = M0 (328): equal.
- **Step 2:** V0 above 2097152: **no**. V0 at most 8192: **no** (the census is sent spanned, as the product does).
- **Step 3:** the first span is the product's **last** span, (24576, 29900], width 5,324 (not below 1,024, so the newest full span was not used).
- **Every census answered** with curl exit 0, status 200, bytes = CONTENT-LENGTH; no exit 63; every `census_read.py` line exit 0 with all four EXPECT counts 0. After each send a capture of Tally's window and `GET /status` 200 in 0.05 to 0.06 s.

## Reading (Part D, by the sheet's table)
- **W1 and W2 hold rows, W1 under 1 s and W2 under 5 s:** yes (82 rows in 0.14 s; 2,542 rows in 1.81 s). On this book the date window sets the time even with the span's rows in it: a census divided by date keeps each request well inside 20 s.
- **W3 under 20 s:** yes for the last span (13.22 s for 5,324 rows, about 2.5 ms a row). The full span's FY census (S2) also answered under 20 s this time: 14.78 s for 8,192 rows (about 1.8 ms a row).
- **The same request, two sittings:** S2 sent the same window and span as v1's W3; the answer is byte-equal (sha256 8d0b0c74...), and the time fell from 24.99 s (v1, its third census of the sitting) to 14.78 s (v2, its fourth). The 20 s line is crossed or not by the same request depending on the sitting, so one sitting's time near 20 s does not settle whether a full span fits.
- **Time against rows:** across W1, W2, W3 and S2 the time grows with the rows returned (0.14 s for 82, 1.8 s for 2,542, 13.2 s for 5,324, 14.8 s for 8,192), not with the window's length alone; bytes per row stay at about 2,430 to 2,470.
- **R1 against W1:** 0.135 s against 0.141 s, within the 2-times line: no state effect at this size.
- Compared with v1's Part D: v1's day and month cells held no rows (0.45 s and 2.95 s for nothing); v2's hold 82 and 2,542 rows and are faster.

## Checks
- **Step 0** (no company loaded): one tally.exe owns 9001; `/status` 200. Tally's release and licence: TallyPrime 7.2 Silver (title band; the About screen was not opened). **C10:** BRIDGE CORPUS DENSE shut from the Company menu; Select Company, none loaded; one tally.exe owns 9001; `/status` 200; no XML request after the shut.
- **Sheet:** `lab/899-census-window-ladder` at 5cf923fe3f7ca7d20ebbeb922f85c7eb8e20bf76 (v2); `SHA256.txt` 7 of 7 equal (sha256 and bytes); `gen_census.py --check` printed its expected line. The working folder was new and empty; v1's folder was not touched.
- **The book:** BRIDGE CORPUS DENSE (100012) loaded alone by Down 13 from the demo row (the highlight read its name before Enter); Current Period 1-Apr-25 to 31-Mar-26, Tally's date 31-Mar-2026, last entry 31-Mar-26.

## Deviations
1. **W1's day** was taken from the Gateway's "date of last entry" (31-Mar-26), which shows a voucher on that day, without opening the Day Book (as in v1). W1 then returned 82 rows, which confirms vouchers on that day in the span.
2. The About screen was not opened; the release and licence come from Tally's title band.
3. The captures after W3 and S2 were looked at (the Gateway, no dialog); the others are kept and listed, not each looked at. The C10 capture was taken after the shut (Select Company), not at the Gateway just before it; the capture after m1, about 40 s before the shut, shows the Gateway.
4. The commands ran from the lab's own shells (each Bash call with history off; PowerShell with PSReadLine saving nothing), not one Git Bash window opened for the sheet.

**Confidence:** one sitting, one synthetic book: PARTIAL.

## Files
`D/requests/` and `D/answers/` (every request and answer, with headers files and the `/status` bodies), `evidence/times.txt`, the step-0 and C10 files in `evidence/`, `notes.txt`, `SCREENSHOTS.txt`, `substitutions.txt`, `SHA256.txt`; the scripts in `scripts/`.
