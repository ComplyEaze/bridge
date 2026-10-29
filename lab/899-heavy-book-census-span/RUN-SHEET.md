# Windows item 899-H v1: how heavy a census row is, and how long Tally takes for a census span of about 4,000 rows, on the heaviest real book the lab holds (#899). Reads only

Not for merge. About 60 minutes. READS ONLY: nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: the book does not change). No ComplyEaze Bridge build is used. `SHA256.txt` lists every other file here with its bytes; check it before step 0.

## What it decides
Each census span of ComplyEaze Bridge is sized from one assumed figure: 4 KiB of response per voucher row, so 8,192 rows against the transport's 32 MiB (33,554,432 byte) cap (`CENSUS_WIRE_BYTES_PER_VOUCHER` and `census_capacity`, agent_voucher_window.rs). The rows measured so far were 2.42 to 2.72 KB on earlier books and about 3.27 to 3.4 KB on the heaviest one read, a headroom of about 1.2 times, and a span near the cap has never been timed against the 20-second request limit (#899, decision of 29 Oct 2026). This sheet measures, on the heaviest real book the lab holds, with the product's own census request bytes:
1. the bytes per row on the wire (UTF-16LE), for spans of about 250, 1,000 and 4,000 rows;
2. the seconds Tally takes to answer each, to the first byte and in all.

It decides no code change. It reports what a span costs, so that the width of a span can be judged against both limits.

## Rules and the book
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step here on the owner's standing word of 27 Oct 2026, and records each step as LAB-COMMON C11 directs. The scope of this sitting is the maintainers' decision on #899 of 29 Oct 2026; using a real book in the lab is within the owner's standing word.

**THE BOOK** is the heaviest real book the lab holds: the lab knows which from the run coordinator's message. **Its name, GUID, company number, dates, marks and every alteration id are never written in this sheet, in the notes, in a file name, in the results, in a commit message or in the list of screenshots.** Everywhere that is pushed it is "THE BOOK".
- Kept on the lab PC only: `company.txt` (the book's name as Tally's Select Company lists it, one line), `dates.txt`, the request files, every answer, every command line that carries an id, and every screenshot (whole-screen captures show the book).
- Pushed: sizes, counts, widths of spans (not their positions), seconds, the sha256 of each answer, and the rows-per-id and bytes-per-row figures computed from them. A request is never hashed (it holds the name); only its size is recorded.
- The answers are deleted after the run coordinator's review (step 9).

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **Raw reads only.** Every request is a committed product request shape (`templates/`, the files captured live on 26 Oct 2026 and committed on master as `spanned-census-two-books/o-c2.request` and `o-m.request`), changed by `gen_census.py` in the four ways its header lists. Nothing read from Tally or from an answer chooses a report, a collection, a fetch list or a formula.
2. **C3's line** is sent with two named differences: the `-w` extended to `"%{http_code} %{size_download} %{size_upload} %{time_connect} %{time_starttransfer} %{time_total}"`, and `--max-filesize 25165824` (24 MiB; Tally announces CONTENT-LENGTH, so curl refuses a larger answer, exit 63, before it downloads). Still no `--max-time`: a request without an answer in 10 minutes is C3's STOP (leave that curl running, send nothing, report). Note the UTC clock time just before it starts and just after it exits.
3. **Safety rule for the spans.** The first span is 250 ids wide. Every later span is sized by `gen_census.py plan` from the span before it, so that its estimated answer (with a margin of 1.25) fits **half the cap (16 MiB)**, its estimated time (with a margin of 1.5) stays within **15 s**, it is never wider than the product's own 8,192 ids, and it is never more than the book's voucher mark allows. `plan` prints `STOP` when no larger step is allowed. **If the first span takes more than 20 s, no other span is sent.** Any span that takes more than 20 s ends the ladder after it.
4. **One request at a time.** After each send a screenshot of Tally's whole window is taken before the next request (a dialog there is C7), then `GET /status`.
5. **The number format of this PC** (row 1780-D, deviation 2): a comma is a digit separator there. No command in this sheet takes a list, and no number may be written with a comma.
6. **Marks.** The voucher mark V0 and the master mark M0 are read before the first span and again after the last (M1 equal to M0, V1 to V0). They are recorded locally and are NOT written in anything pushed; only "equal" or "not equal" is.

Licensed mode only; Education mode = STOP.

## Files
`gen_census.py`, `census_read.py`, `templates/` (the two product requests), `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Before the sitting
1. Check every file here against `SHA256.txt`.
2. `python -I gen_census.py --check`: EXPECT exactly `gen_census: ok (the census and marks templates reproduced byte for byte; an ampersand and quotes escaped once; the span, dates and parse checked)`.
3. The working folder is `C:\bridge-lab\899h\` (local; absent or empty before, else STOP), with `requests\` and `answers\` in it.
4. **Command history is off for the sitting.** In every PowerShell window used for this sheet, first run `Set-PSReadLineOption -HistorySaveStyle SaveNothing`, so no command line that carries the book's name, a date or an id is saved. (Step 9 checks the history file anyway.)

## Steps
0. LAB-COMMON step 0 (C2) with no company loaded: the screenshot, the process list and port 9001, `GET /status`. Tally's About screen: record ONLY the release and the licence mode and edition (no serial number, no account, no e-mail address, nothing else the screen shows); Education = STOP.
1. Load THE BOOK alone, by the arrow keys of Select Company (C1). A screenshot of the Gateway (local). Then write, in the working folder, with a plain text editor and in UTF-8:
   - `company.txt`: the book's name exactly as Select Company lists it, on one line, saved as UTF-8 **without a byte-order mark** (Notepad: Save As, Encoding UTF-8; the script drops a BOM anyway);
   - `dates.txt`: two lines, FROM (the first day of the books, day 1 or 2) and TO (the 31st of the month that ends the financial year holding the book's last voucher), each as eight digits `YYYYMMDD`, read from Tally's Gateway and Day Book. The window then covers every voucher, so rows are about the ids in the span.
   Neither file is committed or quoted. Record in the notes only that they exist.
2. **M0.** `python -I gen_census.py marks --company-file <working folder>\company.txt --out <working folder>\requests\marks.utf16le.xml` prints `request written (N bytes)`: record N. Send it by C3's line (answer to `answers\m0.response`). Read it by structure: EXPECT exactly one COMPANY row; record V0 (`ALTVCHID`) and M0 (`ALTMSTID`) locally. **STOP if there is no row or more than one, or if V0 is below 2,000.**
3. **S1.** `lo1` is V0 divided by 8, rounded down; `hi1` is `lo1` plus 250. Generate: `python -I gen_census.py census --company-file <folder>\company.txt --from <FROM> --to <TO> --lo <lo1> --hi <hi1> --out <folder>\requests\s1.utf16le.xml` (read FROM and TO from `dates.txt`; the command line stays local). Record the size N it prints. Send by C3's line to `answers\s1.response`. Record curl's exit code, the printed status and bytes, CONTENT-LENGTH (EXPECT equal), the two times (`time_starttransfer`, `time_total`), the answer's sha256, and `python -I census_read.py <answer> <lo1> <hi1>` whole (EXPECT exit 0 and `rows_outside_span 0`, `repeated_guids 0`, `rows_missing_a_field 0`; any other = STOP, keep the file). Then the screenshot and `GET /status`.
   - **If `time_total` is more than 20 s: STOP the ladder here** (record it, go to step 7).
4. **S2.** `python -I gen_census.py plan --prev-rows <rows of S1> --prev-width 250 --prev-bytes <bytes of S1> --prev-seconds <time_total of S1> --target 1000 --room <V0 minus hi1>`.
   - If it prints `STOP`, record the line, send nothing more and go to step 7.
   - Otherwise its `next width` is the width of S2: `lo2` is `hi1`, `hi2` is `lo2` plus that width. Generate, send, record and read as in step 3 (with `lo2` and `hi2`).
5. **S3.** `plan` again with S2's rows, width, bytes and `time_total`, `--target 4000` and `--room` V0 minus `hi2`. If `STOP`: record it and go to step 7. Else `lo3` is `hi2`, `hi3` is `lo3` plus the printed width. Generate, send, record and read as in step 3.
   - S3 is the span of about 4,000 rows; the plan may hold it to fewer rows (it says which limit applied). That is a result, not a STOP.
6. Compute and record, for each span sent: width, rows, rows per id, answer bytes (CONTENT-LENGTH), wire bytes per row (bytes divided by rows) and as a percentage of 4,096, the UTF-8 equivalent per row (half of it), `time_starttransfer`, `time_total`, milliseconds per row. Between S2 and S3 (and S1 and S2) compute the marginal bytes per row (the difference of the answers' bytes divided by the difference of their rows), which leaves out the fixed envelope. Then, at the S3 figures: the bytes of a full 8,192-row span as a share of 33,554,432, and its seconds, if the rate were the same.
7. **M1.** `marks` again (to `answers\m1.response`): EXPECT the same V and M as step 2 (a difference is recorded and is not a STOP: the book is only read). Then LAB-COMMON C10: at the Gateway a screenshot, Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway); Select Company with none loaded; one tally.exe; 9001 LISTENING; `GET /status` 200. No XML request is sent after the shut.
8. Write the results (below). Send the evidence branch's SHA to the run coordinator before pushing.
9. **After the run coordinator's review, on its word:** delete from the working folder, one file at a time by its full name (no wildcard), every request, every answer, `company.txt`, `dates.txt` and the screenshots. Take a directory listing (names, sizes) before and after, and record that the after-listing is empty of those names. The listings carry no name of the book. Then open the PSReadLine history file (`(Get-PSReadLineOption).HistorySavePath`) and search it for `gen_census`, `company-file` and `census_read`: EXPECT no line; if there is one, delete those lines and record only that you did.

STOP RULES: C7 and C8 of LAB-COMMON; any dialog; a second `tally.exe`; an answer above its bound (curl exit 63 is this: record it); a request without an answer in 10 minutes; a `census_read.py` failure; a plan that prints STOP (that one ends the ladder, it is not a fault); Education mode; Ctrl+Q or Escape at the Gateway (an incident: if a prompt shows, press no key, take a screenshot and STOP; only if the process list shows NO `tally.exe`, C9).

## What the row records, and what it does not
Per span: width, rows, rows per id, bytes, bytes per row, seconds to the first byte and in all, the answer's sha256. Across spans: the marginal bytes per row and the seconds per row. From them: whether a full 8,192-row span would fit the cap on this book, and whether it would fit 20 s at the measured rate. **Confidence:** one sitting on one real book, PARTIAL until repeated. Not covered: other books, a span the plan did not allow, a second sitting for repeat timings, a book with a different mix of voucher types, any Tally release other than the one recorded, the tool's own reads.

## Reading it (for the requester, not the lab)
| reading | meaning for the 4 KiB figure and the span width |
|---|---|
| S3's marginal bytes per row below 3.4 KB | in line with the earlier heaviest book: the 1.2 times headroom stands |
| between 3.4 KB and 4.0 KB | the headroom is thinner than documented; the assumed figure, or a narrowing after the first span, needs a decision |
| 4,096 B or more | a full 8,192-row span on this book would pass the 32 MiB cap: the assumed figure is too small for this book |
| seconds per row times 8,192 above 20 s | even a span within the byte cap would time out on this book: the span width needs a time bound as well as a byte bound |
| S1 above 20 s | the product cannot read this book in 250 ids: report at once |

- **FAIL of the row:** a request other than a committed product shape is sent; a name, id, date or mark appears in anything pushed; a mark moves in a way a read cannot explain; THE BOOK is not the only company loaded; the answers are not deleted after review.

## Evidence
- RESULTS.md: one table (a line per span) and the computed figures of step 6; the build is not applicable; Tally's release and licence mode and edition ONLY (no serial number, account or e-mail address); the step-0 and close checks (process list, port 9001, `/status`).
- `times.txt`: per request, the UTC start and end, curl's exit code and `-w` fields, CONTENT-LENGTH, the answer's sha256, the `census_read.py` line (it prints offsets and counts only).
- `notes.txt` (UTC, one line per step; it names no id, date or mark), `SCREENSHOTS.txt` (name, size, sha256; the captures stay local), `SHA256.txt` over the committed files.
- Evidence on a new branch, `lab/899-heavy-book-census-span-answers`; push only after the pre-push scan and the run coordinator's yes (C11).
