# Row 2162-S7: does `$$IsSales` as a filter term cut exactly the non-Sales rows on a mixed book, does its speed survive a reversed order, and what does the year cost read a month at a time?

Not for merge. Part of #2162. Written 8 Nov 2026. Lines below are at master 76705cfb, and this branch starts there. It settles what the provisional picks for the number rule's read (Y0, paired calendar months, the control checked inside its month) leave unmeasured. Confidence of everything here until the row runs: unverified.

## Question
Row S6 (`lab/2162-s6-answers` at a7dc73f0) left three things open.
- **The cut (part A).** Y1 is Y0 with ` AND $$IsSales:$VoucherTypeName` added to its filter. On S6's book Y1 equalled Y0 byte for byte, because all 29,900 rows were Sales. So nothing is known about what Y1 does to a non-Sales row, or whether it keeps every Sales row on a book that has both. This part measures it on `BRIDGE PILOT LAB`, whose year S5 read as 41 rows, 35 Sales and 6 not.
- **The order (part B).** Each Y1 in S6 was sent after its Y0, so its speed (3.08 s against 8.38 s for the month) may be a warm cache. This part changes one variable, the order.
- **The month ladder (part C).** The provisional pick reads the year as twelve Y0 months, paired. Only April was measured. This part sends every month twice. It records each month's rows, bytes and seconds, and the year's total. It also checks that the control invoice's row inside its month answer equals a narrow N read for it.

## Books
Synthetic only: the two books named below and no other. **No client book, and nothing seeded from client data.** If either book is not on the PC, has another GUID, or Tally is in Education mode: stop, the part has no book.

- **A: `BRIDGE PILOT LAB`.** TallyPrime 7.2 Silver, on 9001, loaded alone.
  - S5 recorded ALTVCHID 68 and ALTMSTID 245, and its own writes were restored. Later rows (S13, S13a2) also restored.
  - Record the marks found at a1 and go on; nothing in this row writes.
- **B and C: `BRIDGE CORPUS DENSE`.** The book of rows 725-D1, 1029-T and S6, GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, loaded alone.
  - EXPECT V 29,900 and M 328 (S6 RESULTS, m1 and m2).
  - The data folder holds 20 files while the company is shut and 23 while it is loaded.

**Read only.** No write, no backup, no restore. No key is pressed in Tally beyond loading and shutting a company. Any prompt when a company opens: press no key, screenshot, stop, and report its words.

## Kit
No Bridge build. Raw requests, one at a time, through S6's `timed.py`, unchanged. Close every other Bridge process first.

`gen7.py` writes every request from `template-y-pilot.utf16le.xml`. The template is S6's template, byte-equal, and so S5's `Y-all.xml`, byte-equal. gen7.py changes only the company, the four dates and the one Y1 or number term, and asserts each change occurs exactly as often as expected, as S6's gen.py did. The generated requests are committed beside it. `SHA256.txt` lists every file with its sha256 and size: check it first.

| file | what |
|---|---|
| `A-M-marks.xml` | S5's `M-marks.xml`, byte-equal: the marks request for `BRIDGE PILOT LAB` |
| `M-marks.xml` | S6's `M-marks.xml`, byte-equal: the marks request for `BRIDGE CORPUS DENSE` |
| `A-Y0-year.xml`, `A-Y1-year.xml` | Y0 is S5's `Y-all.xml`, byte-equal: `BRIDGE PILOT LAB` over 1 Apr 2026 to 31 Mar 2027, the year S5 read. Y1 is the same with ` AND $$IsSales:$VoucherTypeName` at the end of its filter. **Y1 has not been sent to this book.** |
| `B-Y0-month.xml`, `B-Y1-month.xml` | S6's `Y0-month.xml` and `Y1-month.xml`, byte-equal (April 2025) |
| `C-Y0-2025-04.xml` … `C-Y0-2026-03.xml` | twelve Y0 reads for the dense book, one per calendar month of 1 Apr 2025 to 31 Mar 2026, each from the 1st to the month's last day (April's equals `B-Y0-month.xml`) |
| `C-N1-2025-04.xml` | the number read for number `1` over April 2025 only: S6's `N1-year.xml` with its end date set to 30 Apr 2025 |
| `gen7.py`, `template-y-pilot.utf16le.xml` | the generator and its template (above) |
| `timed.py` | S6's `timed.py`, byte-equal |
| `count7.py` | S6's `count6.py` with this sheet's names and comparisons (below) |

**Checked when the sheet was written:**
- `A-Y0-year.xml` is byte-equal to S5's `Y-all.xml`.
- `B-Y0-month.xml`, `B-Y1-month.xml` and `M-marks.xml` are byte-equal to S6's files of those names, and `timed.py` to S6's.
- `C-Y0-2025-04.xml` is byte-equal to `B-Y0-month.xml`.
- `C-N1-2025-04.xml` is byte-equal to S6's `N1-year.xml` with `20260331` replaced by `20250430` twice.
- S6 checked its N1 and Y0 files against `render_invoice_number_request` at 1400175e. Neither `agent_import_invoice_wire.rs` nor `agent_read_profiles.rs` has changed between 1400175e and 76705cfb, so those renders stand.

**count7.py** is S6's `count6.py` with the file names of this sheet. It reads every answer with expat, never a text search, and its output stays bounded. For each answer it gives:
- the bytes and sha256;
- the rows (a `VOUCHER` under `COLLECTION` with a `VOUCHERNUMBER`), with their minimum, maximum and mean bytes;
- the counts of `BRIDGEVCHISSALES` Yes and No, and of each type name;
- the first and last date;
- of the Sales-class rows, how many fold to a number that another Sales-class row also has.

Its comparisons:
- **A:** whether `A-Y1-year` equals the Sales-class rows of `A-Y0-year`, element for element. If it does not, the rows missing from Y1 that are Sales class, and the rows in Y1 that are not, at most 20 lines;
- **B:** whether each pair of sends of one request holds the same rows;
- **C:** whether `C-N1-2025-04` equals the rows of `C-Y0-2025-04` numbered `1`; that the twelve months' dates tile the year with no row in two months; and the sum of the twelve months' rows. That sum is checked against S6's Y0-year count of 29,900.

An answer still marked `.pending` is no answer, never 0 rows.

It was run, when the sheet was written, on S6's captured answers (`lab/2162-s6-answers`, `run/`) renamed to this sheet's names:
- S6's m1 as b0: the book's GUID, ALTVCHID 29900, ALTMSTID 328;
- S6's s4 and s3 as b1 and b2: 2,430 rows each, per row 2,766 to 2,784 B, and b1 equal to b2's Sales-class rows element for element;
- S6's s3 also as c01a: month 202504, 2,430 rows, none dated outside the month, and "months answered 1 of 12";
- the mismatch path: S6's s3 as a2 and s1 as a3. It reported them unequal, 2,348 Sales-class rows missing from a3 (the first 20 listed), and 0 rows in a3 that are not Sales class.

## Steps
One at a time. Run `count7.py` after each step and keep its whole output. Record the wall clock at each send.

### Part A, `BRIDGE PILOT LAB`
0. Check `SHA256.txt`. Load `BRIDGE PILOT LAB` alone. Screenshot. Record Tally's release, Tally's date and the PC's date.
1. **a1.** `timed.py URL a1 A-M-marks.xml`. Record the GUID, V and M.
2. **a2, Y0 for the year.** `timed.py URL a2 A-Y0-year.xml`. EXPECT about 41 rows, both classes present. **If every row is Sales class, the book does not measure the cut:** record it, skip a3, and go to a4.
3. **a3, Y1 for the year. The P7 guard.** `timed.py URL a3 A-Y1-year.xml`, **one send, watched.**
   - Screenshot Tally's window as soon as the answer is written.
   - Any dialog, or a window other than the Gateway: **STOP** (below).
   - An answer that is not a collection of rows, or an Unknown Request: record it whole, and skip to a4.
4. **a4.** `timed.py URL a4 A-M-marks.xml`. EXPECT byte-equal to a1. Shut the company by Tally's own Shut Company. Screenshot: no company loaded.

### Part B, `BRIDGE CORPUS DENSE`, order reversed
5. Load `BRIDGE CORPUS DENSE` alone. Count the folder: EXPECT 23. **b0.** `timed.py URL b0 M-marks.xml`: EXPECT the GUID, V 29,900, M 328.
6. **b1.** `timed.py URL b1 B-Y1-month.xml`: Y1 first, on a cold start for this sitting.
7. **b2.** `timed.py URL b2 B-Y0-month.xml`.
8. **b3.** `timed.py URL b3 B-Y1-month.xml`, again.
9. **b4.** `timed.py URL b4 B-Y0-month.xml`, again.

### Part C, the month ladder (same sitting, same book)
10. **The control, narrow.** `timed.py URL c0 C-N1-2025-04.xml`. EXPECT 1 row, the Sales voucher numbered `1`.
11. **The months.** For each month in order, April 2025 to March 2026: send `timed.py URL c<nn>a C-Y0-<month>.xml`, then the same file again as `c<nn>b` (the paired read's two halves). `<nn>` runs `01` for April 2025 to `12` for March 2026; count7.py reads the month from the file name and checks the label's form.
    - **The ladder:** go on to the next month only if this month's first send's curl total is at most 15 s and its answer at most 24 MiB.
    - Otherwise stop part C there. Record the month and the figure as **not sent by the ladder**, never as 0 rows.
    - The limit leaves a margin below the transport's 20 s and 32 MiB.
12. **cend.** `timed.py URL cend M-marks.xml`. EXPECT byte-equal to b0. Shut the company by Shut Company and write down the path and keys. Count the folder: EXPECT 20. Screenshot. One tally.exe owns 9001.

## Stop rules
S6's stop rules, unchanged:
- **Any dialog or prompt:** press no key, screenshot, and send the screenshot and answers to the run coordinator. Ask before anything else.
- **No answer in 15 minutes:** press nothing and kill nothing, curl included. Screenshot and ask. A read given up by its client keeps the gateway busy.
- **The gateway stops answering,** a curl failure, an HTTP code other than 200, or an Unknown Request: timed.py stops itself. Record it and ask; resend nothing.
- **Education mode,** or a book not on the PC.
- **A closing mark not byte-equal to its opening mark:** record both, report, and send nothing more.

## Record
For each send:
- the bytes, sha256, rows and row bytes;
- curl's first-byte and total seconds, and the wall clock;
- the class and type counts, and the date range;
- count7.py's comparison lines.

For the row: the marks at each point, the folder counts, the release and dates, Tally's window after a3 (and after any step where it changed), and every ladder decision with its figure. Then part C's totals: the sum of every first send's seconds, the sum of every second send's, and the largest month's rows, bytes and seconds.

## Expected
Estimates from earlier rows, not predictions.
- **a2:** about 41 rows, about 35 Sales and 6 not, at 2,768 to 2,812 B a row (S5 RESULTS, step 2), unless later rows left vouchers.
- **a3:** if Y1 serves, exactly a2's Sales-class rows. Not predicted.
- **b1 to b4:** 2,430 rows and 6,792,322 B each, byte-equal to S6's s3. Times not predicted. S6 measured Y0 at 8.38 s after nothing, and Y1 at 3.08 s after Y0.
- **c0:** 1 row. **The C months:** about 2,500 rows a month, summing to 29,900. About 8 to 9 s a send if April's figure holds, about 200 s over all twenty-four sends.

## Decides
- **Whether Y1 may be used (A).** It may be used only if a3 equals a2's Sales-class rows element for element, a2 holds non-Sales rows, and no dialog appeared. If any Sales row is missing from Y1, Y1 is not used, whatever its speed. A non-Sales row present in Y1 is recorded; it costs nothing, because the program reads the class per row.
- **Whether Y1's speed is the term or the order (B).** Compare b1 against b2, and b3 against b4.
- **The window and its bound (C).** Whether every month of the dense book stays under the ladder's 15 s and 24 MiB, and by how much. The largest month is the figure the code change states as its bound. A month stopped by the ladder means the month is not the window for this book. The provisional window pick is then revisited before any code.
- **The control inside the month (C).** It may be checked inside the month answer only if c0 equals the April answer's rows numbered `1`, element for element. The separate control send is then dropped.
- **The cost.** Part C's two sums are the measured cost of one number read on this book. The queue sends it again before a post.
- **A dialog anywhere:** the row stops.

## Code admits the trigger (76705cfb)
- **N, the number read and its control:**
  - `render_invoice_number_request` (`agent_import_invoice_wire.rs:429`, the literal at `:436`), wrapped by `invoice_number_read` (`agent_read_profiles.rs:680`).
  - It is asked as `Need::Number` and `Need::NumberControl` (`agent_import_invoice.rs:2124`, `:2286`, `:2313`) and sent paired by `post_admitted_read` (`agent.rs:2880`).
  - Its answer is judged at `agent_import_invoice.rs:2288-2330`, and the control by `control_row_found` (`agent_import_invoice_wire.rs:482`).
- **Y0 and Y1:** no build sends either. Every caller renders the literal with its number. Y0 is S5's Y. Y1 is S6's candidate, sent here to a second lab book, as P7 requires of a new shape before its builder is written. N1 over a month is today's rendered shape with other literal dates, the shape a release sends.
- **No write is possible:** no request here is an import, and no Bridge build runs.

## Keep
On `lab/2162-s7-answers`, with `SHA256.txt`:
- every answer of 10 MB or less whole, with each log;
- for each larger answer, its name, size and sha256 in `kept-local.txt`;
- count7.py's whole output after each step;
- a RESULTS.md with one line per send first, then the ladder decisions and part C's sums;
- a notes file with UTC times, one line per step.

Screens stay on the lab PC, listed by name, size and sha256. Send the sha to the run coordinator before any push.

Row: lab/2162-s7/
