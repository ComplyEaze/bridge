# Results: #899-P v2 Part H, the census window ladder on THE BOOK (a real book, kept on the lab PC), 2 Nov 2026

**All seven requests answered with the marks equal before and after. Bytes per row were 3,238 to 3,375 on the three windows with many rows, under the product's assumed 4 KiB. The financial year crossed 20 s (21.35 s) and ended the ladder. The same day read took 2.62 s first and 0.035 s when repeated, so Tally's state moves these times: PARTIAL, one sitting.**

Only counts, sizes, window kinds, span labels, seconds and answer hashes are here. THE BOOK's name, id, data folder, dates, marks, span bounds, request sizes and offsets stay on the lab PC.

| step | window | span | rows | answer bytes = CONTENT-LENGTH | bytes per row | time_starttransfer | time_total | answer sha256 | census_read line |
|---|---|---|---|---|---|---|---|---|---|
| m0 | (marks) | - | 1 company | 3458 = 3458 | - | 0.010 s | **0.011 s** | a4fc75b3d5287a7f1417f8e01d9d586956ddac3ebb5ed3275a82856762ab748b | one company row |
| w1 | day | first | 1 | 5516 = 5516 | 5516 | 2.622 s | **2.623 s** | a6fd6d715c55fd8a2b060db27ef04105a2662dc034e15e92d59a2b14fc8e2652 | census ok rows 1, missing 0, outside span 0, outside dates 0, repeated 0 |
| w2 | month | first | 1986 | 6703016 = 6703016 | 3375 | 5.507 s | **5.547 s** | e3a4b38081b1cd83b8c88a0d98c6704d8c9102298e6fe104746c82b1d7686d31 | census ok rows 1986, missing 0, outside span 0, outside dates 0, repeated 0 |
| w3 | FY | first | 2022 | 6813390 = 6813390 | 3370 | 21.325 s | **21.349 s** | b44e8e4b04b0c3b7270b4992682c78008d49c8cb2d7bf43b2a71289c18f5eda2 | census ok rows 2022, missing 0, outside span 0, outside dates 0, repeated 0 |
| s2 | month (WIDEST) | second | 5029 | 16286302 = 16286302 | 3238 | 2.531 s | **2.597 s** | 750291b4b2640268803d71a7f993f8de884e8ddc394389e899fadc4922f1bc9c | census ok rows 5029, missing 0, outside span 0, outside dates 0, repeated 0 |
| r1 | day (W1 file) | first | 1 | 5516 = 5516 | 5516 | 0.034 s | **0.035 s** | a6fd6d715c55fd8a2b060db27ef04105a2662dc034e15e92d59a2b14fc8e2652 | census ok rows 1, missing 0, outside span 0, outside dates 0, repeated 0 |
| m1 | (marks) | - | 1 company | 3458 = 3458 | - | 0.004 s | **0.005 s** | a4fc75b3d5287a7f1417f8e01d9d586956ddac3ebb5ed3275a82856762ab748b | one company row |

- **Marks:** V1 equal to V0, M1 equal to M0.
- **Step 2:** V0 above 2097152: **no**. V0 at most 8192: **no** (the census is sent spanned, as the product does).
- **Every request** ended with curl exit 0, status 200 and bytes = CONTENT-LENGTH; no exit 63; every `census_read.py` line exit 0 with all four EXPECT counts 0. After each send a capture and `GET /status` 200.
- **Tally:** TallyPrime 7.2 Silver, from the title band; licensed, not Education. The About screen was not opened.

## Reading (by the sheet's table)
- **Step 2 no:** the product does not refuse THE BOOK before a census for its size. [verified]
- **W1 and W2 hold rows:** yes (1 and 1,986). W1 under 1 s: no on the first send (2.62 s), yes on the repeat (0.035 s). W2 under 5 s: no, 5.55 s. Dividing by date kept each of them inside 20 s. [verified for this sitting]
- **W3 under 20 s:** no, 21.35 s for 2,022 rows, against 5.55 s for 1,986 rows in W2. The time here followed the window, not the rows returned. [verified for this sitting; the cause is a hypothesis]
- **S2 against the first span at the same window:** 5,029 rows in 2.60 s, against 1,986 rows in 5.55 s. A later request was cheaper, so a time per span cannot be read from one span. [partial]
- **R1 against W1:** differ by far more than 2 times (0.035 s against 2.62 s): Tally's state moves the time, and one more sitting is needed before any figure is used. [partial]
- **Bytes per row:** 3,375 (W2), 3,370 (W3) and 3,238 (S2). W1's 5,516 is one row. Under 4 KiB, as issue #899 measured. S2 held 16,286,302 B, inside the 32 MiB response cap. [verified]

## Checks
- **The sheet:** `lab/899-census-window-ladder` at 5cf923fe3f7ca7d20ebbeb922f85c7eb8e20bf76; `SHA256.txt` 7 of 7 equal; `gen_census.py --check` printed its expected line.
- **The book:** the only company loaded (each marks answer held one company row). Its name was checked, on the lab PC, in the company file at the folder the lab recorded for the heavy book.
- **Captures:** the captures after W1 and m1 were looked at: the Gateway, no dialog. All seven after-send captures hash to one of those two images, so none shows a dialog.

## Deviations
1. **The owner loaded the book and logged in, at the PC.** The lab did not reach it through Specify Path this sitting, and typed nothing at the login. Just before the login box appeared, the lab's click and filter keys for another row reached Tally at Select Company. They had no effect on this row, but whether they landed in a field is not established.
2. **Step 0** ran on 29 Oct with no company loaded. Its process, port and `/status` checks were run again with the book loaded, just before step 2.
3. **Step 10 (the shut) has not run:** the run lead asked that the book stay loaded for further reads. Step 11's deletion is still to do, after review.
4. **W1's day** comes from the Gateway's date of last entry and one look at the Day Book; W2 and W3 follow from it by the sheet's rule.
5. The About screen was not opened.
6. The commands ran from the lab's own shells, each with history off, not from one Git Bash window opened for the sheet.

**Confidence:** one sitting, one real book: PARTIAL. Not covered: other real books, windows between those listed, more than two spans, the product's own reader and pairing.
