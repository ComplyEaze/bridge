# Row 2162-S6: how long does the year's number read take on a dense book, as it is and with `$$IsSales` as a filter term? Rows, bytes and seconds for each

Not for merge. Part of #2162. Written 4 Nov 2026. Lines below are at master 1400175e, and this branch starts there. Confidence of everything here until the row runs: unverified.

## Question
#2162's rule (adopted 4 Nov) needs the year's invoice numbers. Two numbers match when they are equal after ignoring case and the leading zeros of each digit run. Row S5 (`lab/2162-s5-answers` at ab3d2e77) found that Y, today's number read with its number term removed, returns both `BP/26-27/0051` and `BP/26-27/051`, at 2,768 to 2,812 bytes a row on a small book.
- **Why the time matters.** The code change sends Y once per invoice build, and every admitted read is sent twice (agent.rs:2830, "paired"). The transport refuses an answer over 32 MiB (`XML_RESPONSE_MAX_BYTES`, bridge-tally-transport/src/lib.rs:94) and gives up after 20 s (`DEFAULT_REQUEST_TIMEOUT`, :95). Master 1400175e (#2334) recorded one import outlasting those 20 s on this same book.
- **The bound.** A bound counted over every voucher type was ruled out: on a book above a few thousand vouchers a year it would stop Sales invoices. So this row measures whether Tally can cut the rows to Sales vouchers itself.
- **One variable a step.** Y0 is Y as built today. Y1 is Y0 with one term added to its filter: ` AND $$IsSales:$VoucherTypeName`, the expression Y already computes as `BRIDGEVCHISSALES`. The window grows a day, then a month, then the year.
- **The control read.** The build also sends the number read for an invoice the company is known to hold, which must find it (agent_import_invoice.rs:2301-2323). On this book that read is N1, the number read for `1` over the year. Its time is recorded here, and whether its row is inside Y's answer.

## Book
`BRIDGE CORPUS DENSE`, synthetic, TallyPrime 7.2 Silver, licensed, on the lab PC, on 9001, loaded alone. GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, `books_from` 20250401.
- **Its state, from the lab's last note on it:** V 29,900, M 328 (rows 725-D1 and 1029-T). Its Sales vouchers run `1` to `574` over 1–7 Apr 2025 (`lab/1029-dense-range-read-answers`). How many of its vouchers are not Sales is not known: this row counts it.
- **The data folder** holds 20 files while the company is shut and 23 while it is loaded (725-D1's count).

Not a client book, and nothing seeded from client data. Not on the PC, another GUID, or Education mode: stop, the row has no book. Any prompt when it opens: press no key, screenshot, stop, and report its words.

**Read only.** No write, no backup, no restore, and no key pressed in Tally beyond loading and shutting the company. If another row has written to this book since 725-D1 restored it, the marks show it at m1: record them and go on, nothing here writes.

## Kit
No Bridge build. Raw requests, one at a time, through `timed.py`. Close every other Bridge process first.

| file | what |
|---|---|
| `Y0-day.xml`, `Y0-month.xml`, `Y0-year.xml` | Y as built today, for this book over 1 Apr 2025, April 2025, and 1 Apr 2025 to 31 Mar 2026 |
| `Y1-day.xml`, `Y1-month.xml`, `Y1-year.xml` | the same three, each with ` AND $$IsSales:$VoucherTypeName` at the end of its filter formula. **A shape Tally has not been sent.** |
| `N1-year.xml` | the number read as the build renders it today, for number `1` over the year: the control read's shape on this book |
| `M-marks.xml` | the company's GUID, ALTVCHID and ALTMSTID. A byte copy of `lab/1029-t/marks.utf16le.xml`, itself a byte copy of `lab/725-dense-200`'s |
| `gen.py`, `template-y-pilot.utf16le.xml` | gen.py makes the seven reads from the template, which is S5's `Y-all.xml` byte-equal. It changes the company, the four dates, and adds the one term; it asserts each change occurs exactly as often as expected |
| `timed.py` | S5's `run.py` with each send timed (below) |
| `count6.py` | S5's `count.py` reshaped for 30,000 rows (below) |

Requests are UTF-16LE with a byte-order mark, as sent. `SHA256.txt` lists every file with its sha256 and size: check it first.

When the sheet was written, `N1-year.xml` was byte-equal to `render_invoice_number_request` rendered with this book's name, number `1` and the year, and each `Y0` was byte-equal to that render with its number term removed and its dates set to the window.

**timed.py** (`python3 -I timed.py <gateway URL> <label> <files>`):
- sends `Content-Type: text/xml; charset=utf-16`, as every answered sheet since S1, by `curl.exe -sS --fail-with-body`;
- **gives curl no time limit.** A read abandoned by its client keeps Tally busy for every company, so nothing here gives up on a send;
- refuses an answer file that exists already, and writes `<answer>.pending` before each send, removed only after the send's log line;
- writes one line per send to `log-<label>.txt`: UTC start and end to the millisecond, the file, the answer, curl's exit, the HTTP code, curl's seconds to the first byte and in all, and the bytes received;
- stops at a curl failure, an HTTP code other than 200, and an answer that says Unknown Request.

**count6.py** (`python3 -I count6.py`) reads every answer with expat, never a text search, and its output is bounded: counts, sizes and lengths, the first 3 rows of each answer, and at most 20 lines of names. For each answer:
- bytes and sha256;
- **M:** whether the GUID is the book's, ALTVCHID and ALTMSTID;
- **Y and N:** the rows (a `VOUCHER` under `COLLECTION` with a `VOUCHERNUMBER`), their total bytes, the rest of the answer, the per-row minimum, maximum and mean; the counts of `BRIDGEVCHISSALES`, of each type name, of `ISCANCELLED`; the first and last date; each e-invoice tag's count; and, of the Sales-class rows, how many fold to a number another Sales-class row also has;
- whether each `Y1` equals the Sales-class rows of the `Y0` of its window, element for element; whether `N1-year` equals the rows of `Y0-year` numbered `1`; and whether two sends of one request hold the same rows;
- each send's timing from the logs;
- an answer still marked `.pending` as no answer, never as 0 rows.

It was run, when the sheet was written, on S5's captured answers, renamed to this sheet's names. Its row bytes (2,768 to 2,812, the rest 3,518) and its fold count (2, the seed and its dropped-zero twin) equal S5's own count. With the number set to S5's seed it found S5's N for the seed equal to Y's row of that number; with `1`, that same N against Y's rows numbered `1`, it found them unequal. timed.py was run against a local stand-in server: answers byte-equal, a second send refused, an Unknown Request and a refused connection each stopped it.

## Steps
One at a time. Run `count6.py` after each step and keep its whole output. **Record the wall clock at each send** beside timed.py's own lines.

0. **Load and check.**
   - Check `SHA256.txt`. Count the data folder's files while the company is shut: EXPECT 20.
   - Load BRIDGE CORPUS DENSE alone. Screenshot. Count the folder again: EXPECT 23.
   - Record Tally's release, Tally's date and the PC's date.
1. **m1.** `timed.py URL m1 M-marks.xml`. EXPECT the book's GUID, V 29,900 and M 328. Other marks: record them and go on.
2. **s1, the day as built.** `timed.py URL s1 Y0-day.xml`.
3. **s2, the filter, the day. The P7 guard.** `timed.py URL s2 Y1-day.xml`: **one send, watched.** Screenshot Tally's window as soon as the answer is written. Any dialog, or a window other than the Gateway: **STOP** (below). An answer that is not a collection of rows, or an Unknown Request: record it whole and stop before step 4; the shape does not serve.
4. **s3, the month as built.** `timed.py URL s3 Y0-month.xml`.
5. **s4, the filter, the month.** `timed.py URL s4 Y1-month.xml`.
6. **s5, the control read.** `timed.py URL s5 N1-year.xml`. This is the number read a release sends today, over the year.
7. **The ladder.** A year read is sent only if its month read's curl total, times 12, is at most 300 s:
   - **s6:** `timed.py URL s6 Y0-year.xml`, if s3's total × 12 is at most 300 s;
   - **s7:** `timed.py URL s7 Y1-year.xml`, if s4's total × 12 is at most 300 s.
   - A read not sent is recorded as **not sent by the ladder**, with the month figure that held it back. Never as 0 rows.
8. **s8, the second send.** Whichever of s6 and s7 was sent last, once more, as `timed.py URL s8 <that file>`. It times the paired read's second half.
9. **m2.** `timed.py URL m2 M-marks.xml`. EXPECT the answer byte-equal to m1's.
10. **Close.** Shut the company by Tally's own Shut Company and write down the path and keys. Count the folder: EXPECT 20. Screenshot: no company loaded. One tally.exe owns 9001.

## Stop rules
- **Any dialog or prompt** on Tally's screen, at any step: press no key, screenshot, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- **No answer in 15 minutes:** press nothing and kill nothing, curl included. Screenshot Tally's window and ask the run coordinator. A read given up by its client keeps the gateway busy; the lab has seen one wedge it for 40 minutes.
- **The gateway stops answering**, a curl failure, an HTTP code other than 200, or an Unknown Request: timed.py stops itself. Record it and ask; resend nothing.
- **Education mode**, or the book is not on the PC.
- **m2 not byte-equal to m1:** record both, report, and send nothing more.

## Record
For each send:
- the answer's bytes and sha256, its rows, and its row bytes (total, rest, minimum, maximum, mean);
- **its seconds:** curl's first-byte and total, and the wall clock between start and end;
- the class counts, the type-name counts, and the date range;
- count6.py's comparison lines.

And: m1 and m2 (bytes, sha256, V, M, byte-equal or not), the folder counts at each point, the release and the dates, Tally's window after s2 (and after any step where it changed), and every ladder decision with its figure.

## Expected
Estimates from earlier rows, not predictions. 29,900 is the voucher mark, not a count of vouchers.
- **s1 and s3:** about 80 rows a day and 2,500 a month, at about 2.8 KB a row. Time not predicted.
- **s2 and s4:** the Sales-class rows of s1 and s3, element for element. If every voucher on this book is Sales, Y1 cuts nothing, and the counts equal.
- **s5:** 1 row, the Sales voucher numbered `1`, Sales class.
- **s6 and s7, if sent:** about 30,000 rows and 80 MB, above the transport's 32 MiB.

## Decides
- **The bound.** It is set from s3, s4, s6 and s7's rows, bytes and seconds against the transport's 32 MiB and 20 s, each counted twice for the paired read (s8). The code change states which figure it rests on.
- **Whether the filter serves.** Y1 equal to Y0's Sales-class rows on every window, with no dialog: the build can read Sales vouchers alone, and the bound counts Sales vouchers only. Y1 not equal, any dialog, or Unknown Request: the filter is not used, and the bound counts every voucher.
- **Whether the filter cuts anything on this book.** The class counts of s1 and s3. If every voucher is Sales, this book does not measure the cut, and that is said with the bound.
- **The control read.** Kept for now. If `N1-year` equals the rows of `Y0-year` numbered `1`, the control can be checked inside Y's own answer, and s5's seconds are what dropping it saves. Either way the coordinator decides from these figures.
- **A dialog anywhere:** the row stops.

## Code admits the trigger (1400175e)
- **N1, the number read and the control read:**
  - Rendered by `render_invoice_number_request` (agent_import_invoice_wire.rs:429; the literal at :436). Its filter is the date window AND `$VoucherNumber = "{number}"`, for vouchers of any type; its compute is `BRIDGEVCHISSALES:$$IsSales:$VoucherTypeName`.
  - Wrapped by `invoice_number_read` (agent_read_profiles.rs:680). The invoice build asks for it as `Need::Number` (agent_import_invoice.rs:2113, asked at :2286) and as `Need::NumberControl` for the known invoice (`number_control_request`, :2134, asked at :2313).
  - Sent by `post_admitted_read` (agent.rs:2833), paired.
  - Its answer: a Sales-class row refuses the build (agent_import_invoice.rs:2295-2297); the control's row must be found (`control_row_found`, agent_import_invoice_wire.rs:482).
- **Y0, Y as built:** that render less its number term. No build sends it today: every caller renders the literal with its number. It is S5's Y, sent to this book.
- **Y1, the candidate:** Y0 with one filter term. No code renders or sends it. It is sent here raw to a lab book, as P7 requires of a new shape, before a builder for it is written.
- **M, the marks:** `render_agent_company_high_water` (agent_read_profiles.rs:574), the request of rows 725-D1 and 1029-T, byte-equal.
- **No write is possible:** no request here is an import, and no Bridge build runs.

## Keep
On `lab/2162-s6-answers`, with `SHA256.txt`:
- every answer of 10 MB or less whole, with each log;
- for each larger answer, its name, size and sha256 in `kept-local.txt`; the file stays on the lab PC until the coordinator says;
- count6.py's whole output after each step;
- a RESULTS.md with one line per send (rows, bytes, seconds) first, then the ladder decisions;
- a notes file with UTC times, one line per step.

Screens stay on the lab PC, listed by name, size and sha256. Send the sha to the run coordinator before any push.

Row: lab/2162-s6/
