# Row 2162-S7: `$$IsSales` as a filter term on a mixed book, the reversed order, and the dense year read a month at a time: results

**First line: every EXPECT met, no dialog, no stop rule reached.**
- **A, the cut:** on BRIDGE PILOT LAB, Y1 for the year answered exactly Y0's 35 Sales-class rows, element for element, and none of its 6 non-Sales rows. The Gateway showed no dialog after it. (verified)
- **B, the order:** on BRIDGE CORPUS DENSE, Y1 sent first took 1.984 s, then Y0 1.545 s, Y1 1.546 s and Y0 1.620 s. All four answers are byte-equal to S6's Y0-month answer. S6's gap (Y0 8.38 s, then Y1 3.08 s) did not recur. (verified)
- **C, the month ladder:** all twelve months were sent twice, and every first send stayed under the ladder's 15 s and 24 MiB. The largest month is 7,130,794 B (March 2026), and the slowest first send took 3.072 s (April 2025). The months hold 29,900 rows in all, equal to S6's year. The narrow control read equals April's row numbered `1`. First sends sum to 13.643 s and second sends to 9.017 s. (verified)
- The closing marks equal the opening marks on both books. (verified)

Sheet `lab/2162-s7/ROW.md` at b63accd9; SHA256.txt 25 of 25 equal. Reads only, with no Bridge build and one request at a time. Every send is in `run/log-*.txt` and `notes.txt` (UTC). count7.py's whole output after each step is in `evidence/count-*.txt`, and the last one, `evidence/count-cend.txt`, holds every answer's whole sha256.

## Each send

| send | request | rows | Sales Yes / No | bytes | sha256 | curl first byte / total (s) | wall (s) |
|---|---|---:|---:|---:|---|---:|---:|
| a1 | A-M-marks | – | – | 3,394 | 8a0c4268… | 0.007718 / 0.008435 | 0.05 |
| a2 | A-Y0-year | 41 | 35 / 6 | 117,992 | f8ccb3ca… | 0.114347 / 0.115457 | 0.14 |
| a3 | A-Y1-year | 35 | 35 / 0 | 101,286 | 937fa773… | 0.094070 / 0.095109 | 0.12 |
| a4 | A-M-marks | – | – | 3,394 | 8a0c4268… (= a1) | 0.006457 / 0.007016 | 0.04 |
| b0 | M-marks | – | – | 3,406 | fcc9086d… | 0.008003 / 0.008909 | 0.05 |
| b1 | B-Y1-month | 2,430 | 2430 / 0 | 6,792,322 | 9262740a… | 1.941049 / 1.984322 | 2.01 |
| b2 | B-Y0-month | 2,430 | 2430 / 0 | 6,792,322 | 9262740a… (= b1) | 1.519449 / 1.544739 | 1.58 |
| b3 | B-Y1-month | 2,430 | 2430 / 0 | 6,792,322 | 9262740a… (= b1) | 1.486535 / 1.545905 | 1.58 |
| b4 | B-Y0-month | 2,430 | 2430 / 0 | 6,792,322 | 9262740a… (= b1) | 1.575137 / 1.620355 | 1.66 |
| c0 | C-N1-2025-04 | 1 | 1 / 0 | 5,804 | 0f848678… | 1.265906 / 1.266656 | 1.30 |
| c01a | C-Y0-2025-04 | 2,430 | 2430 / 0 | 6,792,322 | 9262740a… (= b1) | 3.025582 / 3.072250 | 3.21 |
| c01b | C-Y0-2025-04 | 2,430 | 2430 / 0 | 6,792,322 | 9262740a… (= b1) | 2.212112 / 2.260998 | 2.31 |
| c02a | C-Y0-2025-05 | 2,542 | 2542 / 0 | 7,115,402 | 24e77bd2… | 1.533559 / 1.594347 | 1.60 |
| c02b | C-Y0-2025-05 | 2,542 | 2542 / 0 | 7,115,402 | 24e77bd2… (= c02a) | 0.561127 / 0.585274 | 0.60 |
| c03a | C-Y0-2025-06 | 2,460 | 2460 / 0 | 6,886,106 | 7b0217d9… | 0.886699 / 0.912187 | 0.92 |
| c03b | C-Y0-2025-06 | 2,460 | 2460 / 0 | 6,886,106 | 7b0217d9… (= c03a) | 0.759756 / 0.770732 | 0.78 |
| c04a | C-Y0-2025-07 | 2,542 | 2542 / 0 | 7,115,542 | 472fd2db… | 0.944906 / 0.979642 | 0.99 |
| c04b | C-Y0-2025-07 | 2,542 | 2542 / 0 | 7,115,542 | 472fd2db… (= c04a) | 0.567814 / 0.597608 | 0.62 |
| c05a | C-Y0-2025-08 | 2,542 | 2542 / 0 | 7,125,610 | ad10df5b… | 0.992825 / 1.008173 | 1.03 |
| c05b | C-Y0-2025-08 | 2,542 | 2542 / 0 | 7,125,610 | ad10df5b… (= c05a) | 0.613597 / 0.649196 | 0.67 |
| c06a | C-Y0-2025-09 | 2,460 | 2460 / 0 | 6,895,946 | 216d09ca… | 0.814330 / 0.826785 | 0.84 |
| c06b | C-Y0-2025-09 | 2,460 | 2460 / 0 | 6,895,946 | 216d09ca… (= c06a) | 0.585178 / 0.610261 | 0.62 |
| c07a | C-Y0-2025-10 | 2,542 | 2542 / 0 | 7,125,710 | 912f9e9c… | 0.859381 / 0.875326 | 0.89 |
| c07b | C-Y0-2025-10 | 2,542 | 2542 / 0 | 7,125,710 | 912f9e9c… (= c07a) | 0.593495 / 0.622742 | 0.63 |
| c08a | C-Y0-2025-11 | 2,460 | 2460 / 0 | 6,895,946 | 4c0c5946… | 1.015525 / 1.045888 | 1.05 |
| c08b | C-Y0-2025-11 | 2,460 | 2460 / 0 | 6,895,946 | 4c0c5946… (= c08a) | 0.560471 / 0.586091 | 0.60 |
| c09a | C-Y0-2025-12 | 2,542 | 2542 / 0 | 7,125,710 | 2edde291… | 0.771788 / 0.798516 | 0.81 |
| c09b | C-Y0-2025-12 | 2,542 | 2542 / 0 | 7,125,710 | 2edde291… (= c09a) | 0.557960 / 0.595689 | 0.60 |
| c10a | C-Y0-2026-01 | 2,542 | 2542 / 0 | 7,125,834 | 9ccb6d10… | 0.836271 / 0.863724 | 0.89 |
| c10b | C-Y0-2026-01 | 2,542 | 2542 / 0 | 7,125,834 | 9ccb6d10… (= c10a) | 0.562468 / 0.581051 | 0.60 |
| c11a | C-Y0-2026-02 | 2,296 | 2296 / 0 | 6,441,010 | e811c83d… | 0.768323 / 0.792371 | 0.80 |
| c11b | C-Y0-2026-02 | 2,296 | 2296 / 0 | 6,441,010 | e811c83d… (= c11a) | 0.519596 / 0.538918 | 0.55 |
| c12a | C-Y0-2026-03 | 2,542 | 2542 / 0 | 7,130,794 | 34039b4d… | 0.863332 / 0.873867 | 0.88 |
| c12b | C-Y0-2026-03 | 2,542 | 2542 / 0 | 7,130,794 | 34039b4d… (= c12a) | 0.576484 / 0.618230 | 0.63 |
| cend | M-marks | – | – | 3,406 | fcc9086d… (= b0) | 0.004012 / 0.007280 | 0.02 |

Each of these answers is a collection of rows or a marks answer. None is an Unknown Request. (verified)

## count7.py's comparison lines (evidence/count-cend.txt)

- **A:** a3 against a2's Sales-class rows: 35 and 35 (of 41), element for element equal: yes. a3 holds no row that is not Sales class.
- **B:** b1 against b3 and b2 against b4: 2,430 and 2,430, equal: yes. b1 and b3 against b2's and b4's Sales-class rows: 2,430 and 2,430 (of 2,430), equal: yes.
- **C:** each month's two sends hold the same rows (12 of 12). c0 against the rows numbered `1` of c01a and of c01b: 1 and 1, equal: yes. No row lies outside its month's dates. Months answered: 12 of 12, distinct 12. Rows over them: 29,900, against S6's Y0-year 29,900: equal: yes.
- **c0 against S6:** c0 (N1 over April 2025) is byte-equal to S6's N1-year answer, 5,804 B, 0f84867880f898e114e111ee2f7e2e6cfd76dc093f986ff303abf74f0b648e9e. S6's took 24.81 s; c0 took 1.267 s. (verified by sha256)
- **Marks:** a4 is byte-equal to a1 (8a0c4268…), and cend to b0 (fcc9086d…, also S6's m1). (verified)

## Ladder decisions (part C)

Each month went on because its first send was at most 15 s and at most 24 MiB (25,165,824 B). The ladder stopped no month. The figures for each month:

| month | first send | rows | bytes | first send total (s) | second send total (s) |
|---|---|---:|---:|---:|---:|
| April 2025 | c01a | 2,430 | 6,792,322 | 3.072 | 2.261 |
| May 2025 | c02a | 2,542 | 7,115,402 | 1.594 | 0.585 |
| June 2025 | c03a | 2,460 | 6,886,106 | 0.912 | 0.771 |
| July 2025 | c04a | 2,542 | 7,115,542 | 0.980 | 0.598 |
| August 2025 | c05a | 2,542 | 7,125,610 | 1.008 | 0.649 |
| September 2025 | c06a | 2,460 | 6,895,946 | 0.827 | 0.610 |
| October 2025 | c07a | 2,542 | 7,125,710 | 0.875 | 0.623 |
| November 2025 | c08a | 2,460 | 6,895,946 | 1.046 | 0.586 |
| December 2025 | c09a | 2,542 | 7,125,710 | 0.799 | 0.596 |
| January 2026 | c10a | 2,542 | 7,125,834 | 0.864 | 0.581 |
| February 2026 | c11a | 2,296 | 6,441,010 | 0.792 | 0.539 |
| March 2026 | c12a | 2,542 | 7,130,794 | 0.874 | 0.618 |

(verified)

## Part C's sums

- **First sends:** curl total 13.643 s over twelve sends; first byte 13.313 s.
- **Second sends:** curl total 9.017 s over twelve sends; first byte 8.670 s.
- **All twenty-four:** 22.660 s. The sheet estimated about 200 s.
- **The largest month by bytes:** March 2026 (c12), 2,542 rows, 7,130,794 B. Its first send took 0.874 s and its second 0.618 s. That is 28 % of the ladder's 24 MiB and 21 % of the transport's 32 MiB.
- **The largest month by rows:** 2,542, held by seven months (May, July, August, October, December, January, March).
- **The slowest send:** April 2025's first (c01a), 3.072 s, the first month read of the ladder. That is 20 % of the ladder's 15 s. (verified)

## Conditions that differ from S6

| | S6 (4 Nov) | S7 (8 Nov) |
|---|---|---|
| tally.exe | pid 1200. Its start time was not recorded. It had already served S5 from 06:42Z, so it had run for at least 2 h 44 min when S6 loaded the book. | pid 12672, started 6 Nov 12:33:29 IST (07:03:29Z). The PC had started at 12:09 IST that day. About 48 h 17 min had passed when the book was loaded at 07:20:06Z. |
| The company before | No company was loaded at S6's step 0. The row before it on this process, S5 (06:42Z to 06:48Z), loaded and shut BRIDGE PILOT LAB. Whether anything else loaded a company between 06:48Z and 09:26Z is not established here. | BRIDGE CORPUS DENSE itself had stayed loaded on this process since the 6 Nov restart. It was shut at 06:59Z on 8 Nov at the close of another row, after that row's one import-build read of 1 Apr 2025 (82 rows) at 06:51Z. Part A then loaded and shut BRIDGE PILOT LAB (07:16:47Z to 07:18:45Z), and DENSE was loaded again at 07:20:06Z, 21 minutes after its shut. |
| Cold or warm for the month read | The Y0 month (8.38 s) was S6's third read of the sitting, after a Y0 day and a Y1 day (82 rows each). The book had just been loaded from no company. | The Y1 month (b1, 1.984 s) was the first read of the sitting on this book: cold for the sitting. But the book's files had been open in this process until 21 minutes before, and this row did not measure whether the operating system's file cache still held them. |
| Order | Y0 month, then Y1 month | Y1, Y0, Y1, Y0 |

(verified for the readings named; the file cache is not measured)

## The rest of the Record

- **Marks:**
  - PILOT LAB a1 = a4: GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85 (S5's book), ALTVCHID 68, ALTMSTID 245.
  - DENSE b0 = cend: GUID d45bc1b0-e5e3-4261-b3b2-cce3915f42d3, ALTVCHID 29,900, ALTMSTID 328. (verified)
- **The folder (DENSE):** 20 files shut before, 23 loaded, 20 shut after (151,994,880 B). The two shut lists hold the same names and sizes; only TACCESS, TCMPSCHACCESS, TCMPSCHSTATE and TSTATE have new times. (verified)
- **Tally:** tally.exe FileVersion 1.1.7.2 (TallyPrime 7.2), SILVER, not Education mode.
  - PILOT LAB: Current Date 15-Sep-2026, period 1-Apr-26 to 31-Mar-27.
  - DENSE: Current Date 31-Mar-2026, period 1-Apr-25 to 31-Mar-26.
  - The PC's date is 8-Nov-2026. (verified)
- **Tally's window after a3:** the Gateway of BRIDGE PILOT LAB, no dialog and no other window, captured at 07:18:13Z, one second after the answer. The Gateway shows Tally's own marker "(e) Data exceptions exist" beside PILOT LAB, at load and after a3 alike. It is not a dialog. (verified)
- **Tally's window at the other steps:** the Gateway each time, with no dialog. (verified)
- **Loading and shutting:**
  - Each company was loaded through Select Company by typing its name as a filter (one row), then Enter.
  - Each was shut by Tally's own menu: a click on K: Company, a click on SHut (Ctrl+F3), then Enter on the one line the Shut Company list held.
  - No other key was pressed in Tally.
  - Twice a keystroke did not reach Tally, and nothing was pressed in its place:
    - At step 0, the first Enter was refused because another window had keyboard focus.
    - At step 5, the first typing of the filter did not arrive.
  - Each was repeated after a click on the field. (verified)
- **Close:** no company loaded; one tally.exe (12672) owns 9001, LISTENING on IPv4 and IPv6. (verified)
- **The installed 0.7.0 extension's two connector servers** were left running and untouched, on the run lead's word. The lab made no ComplyEaze Bridge connector call from step 0 to the close. (verified)

## What it decides, by the sheet's rules

- **A:** a3 equals a2's Sales-class rows element for element, a2 holds 6 non-Sales rows, and no dialog appeared. So Y1 may be used. It dropped every non-Sales row on this book.
- **B:** on this sitting the term made no difference to speed beyond 0.44 s. The slowest month read was the first one, whichever term it carried: b1 (Y1) and c01a (Y0). S6's 8.38 s against 3.08 s is not reproduced. One sitting, one book: PARTIAL.
- **C:** every month of the dense book stays under the ladder's bounds by a wide margin, so the month is a window for this book. The figure for the bound is March 2026's 7,130,794 B, and the slowest first send is 3.072 s. c0 equals the April answer's rows numbered `1`, so the control may be checked inside the month answer.
- **The cost:** one number read of this book's year, as twelve paired months, took 22.660 s of curl time on this sitting.

## Kept on the lab PC

- July 2025's two answers, c04a and c04b (7,115,542 B each, 472fd2db…), are kept on the lab PC by size and sha256 in `kept-local.txt`. They hold a figure that the gate's private set matches: a synthetic voucher number and master id. Their logs, count7.py's lines and the figures above stand.
- Every other answer and every log is pushed whole. The largest is 7,130,794 B, under the sheet's 10 MB, and none holds the company number or a counted 10+ digit run.
- Screens are listed in `SCREENSHOTS.txt` by name, size and sha256. Two of them (b0-filter, cend-no-company) show the Select Company list. (verified)
