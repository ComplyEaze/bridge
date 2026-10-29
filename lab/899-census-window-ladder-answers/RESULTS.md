# Results: #899-P Part D, the census window ladder on BRIDGE CORPUS DENSE (synthetic), 29 Oct 2026. **With the product's newest full 8,192-id span, a one-day and a one-month window answered in 0.45 s and 2.95 s (no rows in the span), and the financial year took 24.99 s for a full 8,192 rows (19.9 MB): over 20 s, so the ladder ended there. A second span at the month window took 0.98 s; W1 again took 0.11 s. Marks unchanged. Part H waits for the run lead's word.**

| step | window | span | request bytes | rows | answer bytes = CONTENT-LENGTH | bytes per row | time_starttransfer | time_total | answer sha256 | census_read line |
|---|---|---|---|---|---|---|---|---|---|---|
| m0 | (marks) | - | 982 | 1 company | 3406 = 3406 | - | 0.012 s | 0.015 s | fcc9086dfa112813c447d23e98eb4500c9a1badbe983d3f8d0c0ceff11dc8430 | V0 29900, M0 328 |
| W1 | day: 20260331 | first: (16384, 24576] | 1540 | 0 | 3022 = 3022 | n/a | 0.450 s | 0.452 s | 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 | census ok bytes 3022 rows 0 rows_missing_a_field 0 rows_outside_span 0 rows_outside_dates 0 repeated_guids 0 min none max none |
| W2 | month: 20260301 to 20260331 | first | 1540 | 0 | 3022 = 3022 | n/a | 2.948 s | 2.950 s | 947fbf36... (same bytes as W1) | census ok rows 0, all counts 0 |
| W3 | FY: 20250401 to 20260331 | first | 1540 | 8192 | 19925970 = 19925970 | 2432 | 24.741 s | **24.991 s** | 8d0b0c7470d6db7bc1fa8f72e7efb50f555b98fe3b710e2bceda98a363342d2c | census ok rows 8192 rows_missing_a_field 0 rows_outside_span 0 rows_outside_dates 0 repeated_guids 0 min_alterid_offset 1 max_alterid_offset 8192 |
| S2 | WIDEST = month | second: (8192, 16384], full, before the first | 1538 | 0 | 3022 = 3022 | n/a | 0.975 s | 0.976 s | 947fbf36... | census ok rows 0, all counts 0 |
| R1 | day (W1's file) | first | 1540 | 0 | 3022 = 3022 | n/a | 0.106 s | 0.107 s | 947fbf36... | census ok rows 0, all counts 0 |
| m1 | (marks) | - | 982 | 1 company | 3406 = 3406 | - | 0.008 s | 0.009 s | fcc9086d... (byte-equal to m0) | V1 29900, M1 328 |

- **Marks:** V1 = V0 (29900), M1 = M0 (328): equal.
- **Step 2:** V0 above 2097152: **no**. V0 at most 8192: **no** (so the census is sent spanned, as the product does).
- **Every census answered** with curl exit 0, status 200, bytes = CONTENT-LENGTH; no exit 63 (W3's 19.9 MB is under the 32 MiB bound); every `census_read.py` line exit 0 with all four EXPECT counts 0. After each send a capture of Tally's window (no dialog) and `GET /status` 200 in 0.05 to 0.2 s.

## Reading (Part D)
- **W1 under 1 s and W2 under 5 s:** on this book the date window sets the time when the span's ids fall outside it. Here the newest *full* product span, (16384, 24576], holds no voucher dated in March 2026 (the book's newest vouchers are above 24576, in the partial span up to the mark 29900), so W1, W2 and S2 return zero rows quickly.
- **W3 over 20 s with a full span of rows:** 24.74 s to the first byte for 8,192 rows (about 3.0 ms a row), 2,432 bytes a row on the wire. A full product span over a financial year does not fit the 20 s limit on this synthetic book.
- **Zero rows with a short time:** the time follows the rows the window and span select together; an empty intersection is cheap (0.1 to 3 s).
- **R1 0.11 s against W1 0.45 s:** about 4 times faster on the repeat; with times this small that is noise-level, but it is over the sheet's 2-times line, so Tally's state moves short times.
- Not answered by Part D: a window that holds the span's vouchers *and* is short (a month whose vouchers are in the span). The ladder's rule fixes the first span as the newest full one, which here lies before March.

## Checks
- **Step 0** (no company loaded): one tally.exe owns 9001; `/status` 200. Tally's release and licence: TallyPrime 7.2 Silver (title band; the About screen was not opened). **C10:** BRIDGE CORPUS DENSE shut from the Company menu; Select Company, none loaded; one tally.exe owns 9001; `/status` 200; no XML request after the shut.
- **Sheet:** `lab/899-census-window-ladder` at c24c3d6a1ba2238c2b4c2f5ccef3cdbb6c8d39de; `SHA256.txt` 7 of 7 equal; `gen_census.py --check` printed its expected line.
- **The book:** BRIDGE CORPUS DENSE (100012) loaded alone by Down 13 from the demo row (the highlight read its name before Enter; one row higher than earlier rows, after another lab company was renamed tonight); Current Period 1-Apr-25 to 31-Mar-26, Tally's date 31-Mar-2026, last entry 31-Mar-26.

## Deviations
1. **W1's day** was taken from the Gateway's "date of last entry" (31-Mar-26), which shows a voucher on that day, not by opening the Day Book (no Escape step needed). W2 and W3 follow from it as the sheet defines them.
2. The About screen was not opened; the release and licence come from Tally's title band.
3. The commands ran from the lab's own shells, not one Git Bash window opened for the sheet; no command carried a real book's name or id (Part D is synthetic).

**Confidence:** one sitting, one synthetic book: PARTIAL.

## Files
`D/requests/` and `D/answers/` (every request and answer, with headers files and the `/status` bodies), `times.txt`, `notes.txt`, `SCREENSHOTS.txt`, `substitutions.txt`, `SHA256.txt`.
