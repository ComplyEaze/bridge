# Windows item 899-P v1: how the census's date window changes Tally's time, with the product's own 8,192-id span, on a synthetic dense book and then on the heaviest real book the lab holds (#899). Reads only

Not for merge. About 60 to 90 minutes. READS ONLY: nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: neither book changes). No ComplyEaze Bridge build is used. `SHA256.txt` lists every other file here with its bytes; check it before step 0.

## What it decides
Sheet 899-H sent the product's census request for 250 AlterIDs over THE BOOK's whole period. Tally took 487 s, then 192.8 s on the repeat, against the product's 20 s request limit. The product does not always send a window that wide: the tax audit's census reads one financial year, and a `vouchers` call reads the window it is given. Readings on other books suggest that the request's date window, not the width of its AlterID span, sets how long Tally takes. This sheet tests that on the product's own span, changing one thing, the dates:
1. the seconds a census of one product span takes over one day, one calendar month and one financial year;
2. the same for the next product span at the widest of those windows that answered within 20 s (does a second span cost as much as the first?);
3. the one-day window again, last (does the order or Tally's state move the time?);
4. for THE BOOK only: whether its voucher mark is above 2,097,152 (256 spans of 8,192, the product's limit) and whether it is at most 8,192, recorded as yes or no only.

It decides no code change. It says whether narrowing the census's dates can keep each census request inside 20 s.

## Rules and the books
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step here on the owner's standing word of 27 Oct 2026, and records each step as LAB-COMMON C11 directs. Using a real book in the lab is within the owner's standing word; THE BOOK is loaded as it was for sheet 899-H. If Tally asks for a company login, the lab presses no key there and waits for the owner, who logs in himself; the lab never types or records a credential.

**Part D, the synthetic book:** `BRIDGE CORPUS DENSE` (synthetic; about 82 vouchers a day). Its name, marks, dates and counts may be written in the notes and results.

**Part H, THE BOOK:** the heaviest real book the lab holds, the same book as sheet 899-H (the lab knows which from the run coordinator's message). **Its name, GUID, company number, dates, marks and every alteration id are never written in this sheet, in the notes, in a file name, in the results, in a commit message or in the list of screenshots.** Everywhere that is pushed it is "THE BOOK".
- Kept on the lab PC only: `company.txt`, `dates.txt`, the request files, every answer, every command line that carries an id or a date, the output of `gen_census.py aligned`, and every screenshot (whole-screen captures show the book).
- Pushed: sizes, counts, window kinds ("day", "month", "FY", never the dates), span labels ("first", "second", never their bounds), seconds, the sha256 of each answer, and the yes/no answers of step 2. A request is never hashed (it holds the name); only its size is recorded.
- The answers of Part H are deleted after the run coordinator's review (step 11).

Differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **Raw reads only.** Every request is a committed product request shape (`templates/`: the files captured live on 26 Oct 2026 and committed on master as `spanned-census-two-books/o-c2.request` and `o-m.request`), changed by `gen_census.py` in the three ways its header lists (company, dates, span). Against sheet 899-H's copy, `gen_census.py` admits any real calendar date (the from-date not after the to-date), as licensed TallyPrime accepts and the product sends, and adds `aligned`, which names the product's own span; its `plan` is removed. Nothing read from Tally or from an answer chooses a report, a collection, a fetch list or a formula.
2. **C3's line** is sent with two named differences: the `-w` is extended to `"%{http_code} %{size_download} %{size_upload} %{time_connect} %{time_starttransfer} %{time_total}"`, and `--max-filesize 33554432` (the product's 32 MiB response cap; Tally announces CONTENT-LENGTH, so curl refuses a larger answer, exit 63, before it downloads). Exit 63 is recorded as a result (rows heavier than 4 KiB on average) and ends that part's census sends, as any other STOP does. Still no `--max-time`: a request without an answer in 10 minutes is C3's STOP (leave that curl running, send nothing, report). Note the UTC clock time just before each send and just after it exits.
3. **The window ladder.** For each part the census windows are sent in this order, each with the same first span: W1 one day, W2 one calendar month, W3 one financial year. **Any census whose `time_total` is above 20 s ends the ladder after it**: no wider window is sent. If W1 itself is above 20 s, the part sends nothing more but its closing marks (step 9).
4. **One request at a time.** After each send a screenshot of Tally's whole window is taken before the next request (a dialog there is C7), then `GET /status`.
5. **The number format of this PC** (row 1780-D, deviation 2): a comma is a digit separator there. No command in this sheet takes a list, and no number may be written with a comma.
6. **Marks.** V0 and M0 are read before the first census of a part and V1 and M1 after its last. For Part D they are written down. For Part H they are recorded locally only; what is pushed is only "equal" or "not equal".

Licensed mode only; Education = STOP.

## Files
`gen_census.py`, `census_read.py` (sheet 899-H's reader as the 899-H repeat ran it, with the `UDF:` prefix declared before parsing, and one addition: it counts rows dated outside the window asked for), `templates/` (the two product requests), `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Before the sitting
1. Check every file here against `SHA256.txt`.
2. `python -I gen_census.py --check`: EXPECT exactly `gen_census: ok (the census and marks templates reproduced byte for byte; an ampersand and quotes escaped once; the span, dates, a one-day window, the product-aligned spans and parse checked)`.
3. The working folder is `C:\bridge-lab\899p\` (local; absent or empty before, else STOP), with `D\requests\`, `D\answers\`, `H\requests\` and `H\answers\` in it.
4. **Command history is off for the sitting.** In every PowerShell window used for this sheet, first run `Set-PSReadLineOption -HistorySaveStyle SaveNothing`. Commands carrying THE BOOK's name, a date or an id run from Bash, as in the 899-H repeat.

## Steps (run Part D in full, then Part H in full; `<P>` is `D` or `H`)
0. LAB-COMMON step 0 (C2) with no company loaded: the screenshot, the process list and port 9001, `GET /status`. Tally's About screen: record ONLY the release and the licence mode and edition; Education = STOP.
1. **Load the part's book alone**, by the arrow keys of Select Company (C1). A screenshot of the Gateway (local). Write `company.txt` in `<P>\` (the name exactly as Select Company lists it, one line, UTF-8 without a byte-order mark).
   - **Part D:** W1 is `20250805` to `20250805`, W2 `20250801` to `20250831`, W3 `20250401` to `20260331`. If the Day Book shows no voucher on 5 August 2025, W1 is the first later day of August 2025 that shows one; record which. If the book holds no voucher in August 2025, STOP Part D and report.
   - **Part H:** write `dates.txt` (local, never quoted) with three windows read from the Gateway and the Day Book: W3 is the financial year (1 April to 31 March) that holds the book's last voucher; W2 is the last calendar month of W3 with at least one voucher, preferring a 31-day month; W1 is a day in W2 on which the Day Book shows vouchers. Record in the notes only that the file exists.
2. **M0.** `python -I gen_census.py marks --company-file <P>\company.txt --out <P>\requests\marks.utf16le.xml`; record the size N it prints. Send it by C3's line (answer to `<P>\answers\m0.response`). Read it by structure: EXPECT exactly one COMPANY row; record V0 (`ALTVCHID`) and M0 (`ALTMSTID`) (Part H: locally). **STOP the part if there is no row or more than one.**
   - **Part H only, the two answers that are pushed:** "V0 above 2097152: yes/no" and "V0 at most 8192: yes/no". If V0 is at most 8192, the product sends this book's census without a span, so this sheet's request is not the product's: send no census, go to step 9.
   - **Part D:** if V0 is at most 8192, the same: go to step 9.
3. **The first span.** `python -I gen_census.py aligned --mark <V0> --at <V0 divided by 8, rounded down>` prints `lo hi`, the product span that holds that AlterID. For Part H it stays on the screen and in a local file.
4. **W1.** `python -I gen_census.py census --company-file <P>\company.txt --from <W1 from> --to <W1 to> --lo <lo> --hi <hi> --out <P>\requests\w1.utf16le.xml`; record the size it prints. Send by C3's line to `<P>\answers\w1.response`. Record curl's exit code, the printed status and bytes, CONTENT-LENGTH (EXPECT equal), `time_starttransfer`, `time_total`, the answer's sha256, and `python -I census_read.py <answer> <lo> <hi> <W1 from> <W1 to>` whole. EXPECT exit 0 and `rows_missing_a_field 0`, `rows_outside_span 0`, `rows_outside_dates 0`, `repeated_guids 0`; any other = STOP, keep the file. Then the screenshot and `GET /status`.
   - If `time_total` is above 20 s: go to step 9.
5. **W2** as step 4, with W2's dates, to `w2`. If `time_total` is above 20 s: go to step 7.
6. **W3** as step 4, with W3's dates, to `w3`.
7. **S2, the second span.** WIDEST is the widest of W1, W2 and W3 whose `time_total` was 20 s or less. If `hi` is below V0: `python -I gen_census.py aligned --mark <V0> --after <hi>`; otherwise `python -I gen_census.py aligned --mark <V0> --at <lo>` (the span before). Either prints `lo2 hi2`. Send as step 4 with WIDEST's dates and `lo2 hi2`, to `s2`, read with `census_read.py <answer> <lo2> <hi2> <WIDEST from> <WIDEST to>`. Record which window WIDEST was (day, month or FY).
8. **R1, W1 again,** exactly as step 4 (the same request file `w1.utf16le.xml`, its sha256 checked as C3 requires), to `r1`.
9. **M1.** `marks` again (to `<P>\answers\m1.response`): EXPECT the same V and M as step 2. A difference is recorded and is not a STOP: the book is only read.
10. LAB-COMMON C10 for the part's book: at the Gateway a screenshot, Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway); Select Company with none loaded; one tally.exe; 9001 LISTENING; `GET /status` 200. No XML request after the shut. **After Part H's shut**, if its data path was changed to load it, set it back to the lab's default data folder (as in 899-H's C10 note).
11. Write the results (below). Send the evidence branch's SHA to the run coordinator before pushing. **After the run coordinator's review, on its word:** delete from `H\` (Part H only), one file at a time by its full name (no wildcard), every request, every answer, `company.txt`, `dates.txt`, the local span file and the screenshots. Take a directory listing (names, sizes) before and after, and record that the after-listing is empty of those names. Search the PSReadLine history file for `gen_census`, `company-file` and `census_read`: EXPECT no line; if there is one, delete those lines and record only that you did. Part D's files may stay.

STOP RULES: C7 and C8 of LAB-COMMON; any dialog; a second `tally.exe`; an answer above 33,554,432 bytes (curl exit 63: record it); a request without an answer in 10 minutes; a `census_read.py` failure or a count other than its EXPECT; Education mode; Ctrl+Q or Escape at the Gateway (an incident: if a prompt shows, press no key, take a screenshot and STOP; only if the process list shows NO `tally.exe`, C9). A census over 20 s is NOT a stop: it ends the ladder as rule 3 says.

## Results (one table per part)
| step | window | span | request bytes | rows | answer bytes = CONTENT-LENGTH | bytes per row | time_starttransfer | time_total | sha256 | census_read line |
|---|---|---|---|---|---|---|---|---|---|---|

Then, per part: the marks equal or not; for Part H the two yes/no answers of step 2; Tally's release and licence mode and edition ONLY; the step-0 and close checks.

## Reading it (for the requester, not the lab)
| reading on Part H | meaning |
|---|---|
| step 2: V0 above 2097152 | the product refuses THE BOOK before any census today (`voucher_window_book_too_large`); the 20 s limit is not where it fails |
| W1 under 1 s and W2 under 5 s | the date window sets the time: a census divided by date can keep each request inside 20 s |
| W2 over 20 s, W1 under | only a one-day window answers in time on THE BOOK |
| W1 over 20 s | the date window does not set the time on THE BOOK: dividing by date cannot help |
| W3 under 20 s | the tax audit's census request for one span fits the limit on THE BOOK |
| S2 much faster than the first span at the same window | later spans are cheaper than the first (state or caching), so a time per span cannot be read from one span |
| R1 and W1 differ by more than 2 times | Tally's state moves the time; one more sitting is needed before any figure is used |
| curl exit 63 | rows heavier than 4 KiB on average: the product's census span is too wide for THE BOOK |

Part D gives the same readings on a synthetic book, and shows whether they hold beyond one real book.

- **FAIL of the row:** a request other than a committed product shape is sent; a name, id, date or mark of THE BOOK appears in anything pushed; a mark moves in a way a read cannot explain; THE BOOK is not the only company loaded; Part H's answers are not deleted after review.

**Confidence:** one sitting on one synthetic and one real book, PARTIAL until repeated. Not covered: other real books, any window between those listed, more than two spans, the product's own reader and pairing (each census is sent once here), any Tally release other than the one recorded.

## Evidence
On a new branch, `lab/899-census-window-ladder-answers`, pushed only after the pre-push scan and the run coordinator's yes (C11): `RESULTS.md`; `times.txt` (per request: UTC start and end, curl's exit code and `-w` fields, CONTENT-LENGTH, the answer's sha256, the `census_read.py` line); `notes.txt` (UTC, one line per step; for Part H it names no id, date or mark); `SCREENSHOTS.txt` (name, size, sha256; the captures stay local); `SHA256.txt` over the committed files. Netstat captures are cut to the port-9001 lines, with the cut recorded in `substitutions.txt`.
