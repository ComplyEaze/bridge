# Windows item 899-P v2: how the census's date window changes Tally's time, with the product's own 8,192-id span, on a synthetic dense book and then on the heaviest real book the lab holds (#899). Reads only

Not for merge. About 60 to 90 minutes, in two parts with a review between them. READS ONLY: nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply: neither book changes). No ComplyEaze Bridge build is used. `SHA256.txt` lists every other file here with its bytes; check it before step 0.

## What it decides
Sheet 899-H sent the product's census request for 250 AlterIDs over THE BOOK's whole period. Tally took 487 s, then 192.8 s on the repeat, against the product's 20 s request limit. The product does not always send a window that wide: the tax audit's census reads one financial year, and a `vouchers` call reads the window it is given. Readings on other books suggest that the request's date window, not the width of its AlterID span, sets how long Tally takes. This sheet tests that on the product's own span, changing one thing, the dates:
1. the seconds a census of one product span takes over one day, one calendar month and one financial year;
2. the same for a second product span at the widest of those windows that answered within 20 s (does a second span cost as much as the first?);
3. the one-day window again, last (does the order or Tally's state move the time?);
4. for THE BOOK only: whether its voucher mark is above 2,097,152 (256 spans of 8,192, the product's limit) and whether it is at most 8,192 (one census, sent without a span), recorded as yes or no only.

It decides no code change. It says whether narrowing the census's dates can keep each census request inside 20 s.

**v2, after v1's Part D** (`lab/899-census-window-ladder-answers` at 3243b75c). v1 took the newest span that is a full 8,192 wide. On the dense book that span held no voucher of the day or month at the end of the books, so its day and month cells timed windows with no rows in them (0.45 s and 2.95 s), and only the financial year held rows (8,192 rows, 25.0 s). v2 takes as its first span **the product's last span, the one that ends at the mark**: the newest AlterIDs, where the vouchers entered last usually lie, so a day and a month at the end of the books should hold rows. The second span is the full one below it. Part D is run again under v2 before Part H, in a new working folder, with its evidence on a new branch.

## Rules and the books
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step here on the owner's standing word of 27 Oct 2026, and records each step as LAB-COMMON C11 directs. Using a real book in the lab is within the owner's standing word. If Tally asks for a company login, the lab presses no key there and waits for the owner, who logs in himself; the lab never types or records a credential.

**Part D, the synthetic book:** `BRIDGE CORPUS DENSE` (synthetic; about 82 vouchers a day). Its name, marks, dates, spans and counts may be written in the notes and results, and its requests and answers are kept (they may later become test fixtures, with provenance).

**Part H, THE BOOK:** the heaviest real book the lab holds, the same book as sheet 899-H. **Part H starts only on the run coordinator's word, given in the lab chat after it has read Part D's results; that message names THE BOOK and its data folder. Nothing else does.** If Part D stopped, or did not read as expected, Part H is not run.
- **Its name, GUID, company number, data folder, dates, marks and every alteration id are never written in this sheet, in the notes, in a file name, in the results, in a commit message or in the list of screenshots.** Everywhere that is pushed it is "THE BOOK".
- Kept on the lab PC only: `company.txt`, `dates.txt`, `span.txt`, the request files and their sha256, every answer, every command line that carries an id or a date, the output of `gen_census.py aligned`, the `census_read.py` lines, and every screenshot (whole-screen captures show the book).
- Pushed for Part H: counts of rows (and of rows missing a field, outside the span, outside the dates, repeated), answer sizes, window kinds ("day", "month", "FY", never the dates), span labels ("first", "second", never their bounds), seconds, the sha256 of each answer, and the yes/no answers of step 2. **Not pushed for Part H:** request sizes (they show the name's length) and the AlterID offsets `census_read.py` prints.
- Part H's files are deleted after the run coordinator's review (step 11).

Differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **Raw reads only.** Every request is a committed product request shape (`templates/`: the files captured live on 26 Oct 2026 and committed on master as `spanned-census-two-books/o-c2.request.utf16le.xml` and `o-m.request.utf16le.xml`), changed by `gen_census.py` in the three ways its header lists (company, dates, span). Against sheet 899-H's copy, `gen_census.py` admits any real calendar date (the from-date not after the to-date), as licensed TallyPrime accepts and the product sends, substitutes the two dates in one pass, and adds `aligned`, which names the product's own spans; its `plan` is removed. Nothing read from Tally or from an answer chooses a report, a collection, a fetch list or a formula.
2. **C3's line** is sent with two named differences: the `-w` is extended to `"%{http_code} %{size_download} %{size_upload} %{time_connect} %{time_starttransfer} %{time_total}"`, and `--max-filesize 33554432` (the product's 32 MiB response cap; Tally announces CONTENT-LENGTH, so curl refuses a larger answer, exit 63, before it downloads). Still no `--max-time`. Before each send, the request file's sha256 is computed and compared with the one recorded when it was written (C3); the hash stays local for Part H.
3. **Time cap and a held gateway.** Each census request has **600 s** to answer. The gateway counts as **held** if a census has no answer at 600 s, or if the `GET /status` sent after it has no answer 60 s after it was sent. Either is a STOP: leave that curl running, send nothing more, press no key in Tally, report at once, and wait. **A held gateway gets no C8 state read and no C10 shut** until the run coordinator, having seen the request answer or Tally's state, says what to do (C9's rule: nothing is ended or re-opened). A census that answers within 600 s but over 20 s is not a stop: it ends the ladder (rule 4).
4. **The window ladder.** For each part the census windows are sent in this order, each with the same first span: W1 one day, W2 one calendar month, W3 one financial year. **A census whose `time_total` is above 20 s ends the ladder after it**: no wider window is sent. If W1 itself is above 20 s, the part sends nothing more but its closing marks (step 9).
5. **Curl exit 63** (an answer over 33,554,432 bytes, this item's C3 size bound, so rows heavier than 4 KiB on average) is recorded as a result: the part sends no further census and goes to step 9. There is no answer to read; record CONTENT-LENGTH from the headers file as the answer's size.
6. **One request at a time.** After each send a screenshot of Tally's whole window is taken before the next request (a dialog there is C7), then `GET /status`.
7. **Numbers** are written with no comma: on this PC a comma is a digit separator.
8. **Marks.** V0 and M0 are read before the first census of a part, and V1 and M1 after its last. For Part D they are written down. For Part H they are recorded locally only; what is pushed is only "equal" or "not equal".

Licensed mode only; Education = STOP.

## Files
`gen_census.py`, `census_read.py` (sheet 899-H's reader as its repeat ran it, with the `UDF:` prefix declared before parsing, and one addition: it counts rows dated outside the window asked for), `templates/` (the two product requests), `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Before the sitting
1. Check every file here against `SHA256.txt` (each line: sha256, bytes, path).
2. `python -I gen_census.py --check`: EXPECT exactly `gen_census: ok (the census and marks templates reproduced byte for byte; an ampersand and quotes escaped once; the span, dates (one pass), a one-day window, the product-aligned spans and parse checked)`.
3. The working folder is `C:/bridge-lab/899p2/` (local; absent or empty before, else STOP; v1's `899p` folder is not touched), with `D/requests/`, `D/answers/`, `H/requests/` and `H/answers/` in it. Every path in this sheet is written with forward slashes and is typed that way in Bash.
4. **No command history.** Every command of this sheet runs in ONE Git Bash window opened for it, whose first command is `unset HISTFILE; set +o history`. In any PowerShell window used (for screenshots or `Get-FileHash`), first run `Set-PSReadLineOption -HistorySaveStyle SaveNothing`.

## Steps: Part D in full; then, on the run coordinator's word, Part H in full (`<P>` is `C:/bridge-lab/899p2/D` or `C:/bridge-lab/899p2/H`)
0. LAB-COMMON step 0 (C2) with no company loaded: the screenshot, the process list and port 9001, `GET /status`. Tally's About screen: record ONLY the release and the licence mode and edition; Education = STOP.
1. **Load the part's book alone** (C1). **Part D:** by the arrow keys of Select Company. **Part H:** in Select Company choose Specify Path (reached with the Up key), enter the data folder the run coordinator named in the lab chat, then THE BOOK's one row. A screenshot of the Gateway (local). Write `<P>/company.txt` (the name exactly as Select Company lists it, one line, UTF-8 without a byte-order mark).
   - **Both parts:** write `<P>/dates.txt` with three windows read from the Gateway (its date of last entry) and the Day Book: W3 is the financial year (1 April to 31 March) that holds the book's last voucher; W2 is the last calendar month of W3 that holds at least one voucher; W1 is a day in W2 on which the Day Book shows vouchers. These lie among the book's newest vouchers, as the first span (step 3) does. **Part D:** write the three windows in the notes. **Part H:** the file stays local and is never quoted; record in the notes only that it exists.
   - Leave the Day Book by Escape from the report screen only (it is a report, not a master or voucher screen); never press Escape at the Gateway.
2. **M0.** `python -I gen_census.py marks --company-file <P>/company.txt --out <P>/requests/marks.utf16le.xml`; record the size it prints and the file's sha256 (Part H: locally). Send it by C3's line (answer to `<P>/answers/m0.response`). Read it by structure: EXPECT exactly one COMPANY row; record V0 (`ALTVCHID`) and M0 (`ALTMSTID`) (Part H: locally). **STOP the part if there is no row or more than one.**
   - Record "V0 above 2097152: yes/no" and "V0 at most 8192: yes/no" (these two are pushed for Part H too).
   - If V0 is at most 8192, the product sends this book's census without a span, so this sheet's request is not the product's: send no census; go to step 9.
   - If V0 is above 2097152, the product refuses this book before any census. The ladder is still run: it measures what the dates do, which is the question.
3. **The first span.** `python -I gen_census.py aligned --mark <V0> --last` prints `lo hi`: the product's last span, which ends at the mark and so holds the newest AlterIDs, where the vouchers entered last usually lie. **If `hi` minus `lo` is below 1024** (a last span too narrow to hold a day's vouchers), run `python -I gen_census.py aligned --mark <V0> --newest` instead and use the `lo hi` it prints (the newest full span). Record which one was used ("last" or "newest full"). Part D also records the width `hi` minus `lo`; Part H records only whether it is 8,192 or narrower (the exact width is a figure of the book). Part H: write the two numbers to `<P>/span.txt` (local).
4. **W1.** `python -I gen_census.py census --company-file <P>/company.txt --from <W1 from> --to <W1 to> --lo <lo> --hi <hi> --out <P>/requests/w1.utf16le.xml`; record the size it prints and the file's sha256. Send by C3's line to `<P>/answers/w1.response`. Record curl's exit code, the printed status and bytes, CONTENT-LENGTH (EXPECT equal), `time_starttransfer`, `time_total` and the answer's sha256. Then run `python -I census_read.py <P>/answers/w1.response <lo> <hi> <W1 from> <W1 to>` and keep its line. EXPECT exit 0 and `rows_missing_a_field 0`, `rows_outside_span 0`, `rows_outside_dates 0`, `repeated_guids 0`; any other = STOP, keep the file. **Zero rows is a result, not a stop:** the time still measures what Tally does for that window; bytes per row is then "n/a". For W1 and W2 rows are expected (the window and the span both hold the newest vouchers); record whether there were any. Then the screenshot and `GET /status` (rule 3).
   - Curl exit 63: rule 5. `time_total` above 20 s: go to step 9.
5. **W2** as step 4, with W2's dates, to `w2`. Exit 63: rule 5. Above 20 s: go to step 7.
6. **W3** as step 4, with W3's dates, to `w3`. Exit 63: rule 5.
7. **S2, the second span** (whatever its time, step 8 follows it unless S2 met rule 5 or a STOP). WIDEST is the widest of W1, W2 and W3 whose `time_total` was 20 s or less. If `lo` is 8192 or more: `python -I gen_census.py aligned --mark <V0> --before <lo>` (the full span below the first). Otherwise: `python -I gen_census.py aligned --mark <V0> --after <hi>` (the span above it, which may be narrower than 8,192; record its width as "full" or "narrower"). Either prints `lo2 hi2` (Part H: add them to `span.txt`). Send as step 4 with WIDEST's dates and `lo2 hi2`, to `s2`, read with `census_read.py <answer> <lo2> <hi2> <WIDEST from> <WIDEST to>`. Record which window WIDEST was (day, month or FY). Exit 63: rule 5.
8. **R1, W1 again,** exactly as step 4 (the same request file `w1.utf16le.xml`, its sha256 checked against the one recorded), to `r1`.
9. **M1.** `marks` again (to `<P>/answers/m1.response`): EXPECT the same V and M as step 2. A difference is recorded and is not a STOP: the book is only read. After a STOP this is C8's one state read, sent only if C8 allows it.
10. LAB-COMMON C10 for the part's book: at the Gateway a screenshot, Company > Shut from the Gateway's menu (never Ctrl+Q at the Gateway); Select Company with none loaded; one tally.exe; 9001 LISTENING; `GET /status` 200. No XML request after the shut. **After Part H's shut:** in Select Company, Specify Path back to the lab's default data folder, and check that Select Company lists the lab's companies again.
11. Write the results (below). Send the evidence branch's SHA to the run coordinator before pushing. **After Part D:** push Part D's evidence on the run coordinator's yes, send it that branch's SHA and `RESULTS.md`'s sha256, and wait for its word before Part H. **After Part H, on the run coordinator's word after its review:** delete from `C:/bridge-lab/899p2/H/`, one file at a time by its full name (no wildcard), every request, every answer, every headers file, `company.txt`, `dates.txt`, `span.txt` and the screenshots. Take a directory listing (names, sizes) before and after, and record that the after-listing is empty of those names. Search the PSReadLine history file and Git Bash's `~/.bash_history` for `gen_census`, `company-file`, `census_read` and `aligned`: EXPECT no line; if there is one, delete those lines and record only that you did. Part D's files stay.

STOP RULES: C7 and C8 of LAB-COMMON; any dialog; a second `tally.exe`; a held gateway (rule 3); a `census_read.py` failure or a count other than its EXPECT; Education mode; Ctrl+Q or Escape at the Gateway (an incident: if a prompt shows, press no key, take a screenshot and STOP; only if the process list shows NO `tally.exe`, C9). After any STOP: C8, then C10, and the part ends; Part H is not started after a STOP in Part D.

## Results (one table per part)
| step | window | span | request bytes (Part D only) | rows | answer bytes = CONTENT-LENGTH | bytes per row | time_starttransfer | time_total | answer sha256 | census_read line (Part H: without the offsets) |
|---|---|---|---|---|---|---|---|---|---|---|

Then, per part: the marks equal or not; the two yes/no answers of step 2; Tally's release and licence mode and edition ONLY; the step-0 and close checks.

## Reading it (for the requester, not the lab)
| reading on Part H | meaning |
|---|---|
| step 2: V0 above 2097152 | the product refuses THE BOOK before any census today (`voucher_window_book_too_large`); the 20 s limit is not where it fails |
| W1 and W2 hold rows, W1 under 1 s and W2 under 5 s | the date window sets the time even with the span's rows in it: a census divided by date can keep each request inside 20 s |
| W1 or W2 holds no rows | the window and the span did not meet at the end of the books; that cell times an empty intersection, as v1's Part D did |
| W2 over 20 s, W1 under | only a one-day window answers in time on THE BOOK |
| W1 over 20 s | the date window does not set the time on THE BOOK: dividing by date cannot help |
| W3 under 20 s | the tax audit's census request for one span fits the limit on THE BOOK |
| S2 much faster than the first span at the same window | later spans are cheaper than the first (state or caching), so a time per span cannot be read from one span |
| R1 and W1 differ by more than 2 times | Tally's state moves the time; one more sitting is needed before any figure is used |
| zero rows with a time like a full span's | the time follows the window, not the rows returned |
| curl exit 63 | rows heavier than 4 KiB on average: the product's census span is too wide for THE BOOK |

Part D gives the same readings on a synthetic book, and shows whether they hold beyond one real book. Its v2 cells compare with v1's: the same windows, with rows in the span.

- **FAIL of the row:** a request other than a committed product shape is sent; a name, id, date, data folder or mark of THE BOOK appears in anything pushed; a mark moves in a way a read cannot explain; THE BOOK is not the only company loaded; Part H's files are not deleted after review.

**Confidence:** one sitting on one synthetic and one real book, PARTIAL until repeated. Not covered: other real books, any window between those listed, more than two spans, the product's own reader and pairing (each census is sent once here), any Tally release other than the one recorded.

## Evidence
On a new branch, `lab/899-census-window-ladder-v2-answers`, pushed only after the pre-push scan and the run coordinator's yes (C11): `RESULTS.md`; `times.txt` (per request: UTC start and end, curl's exit code and `-w` fields, CONTENT-LENGTH, the answer's sha256, the `census_read.py` line; Part H without request sizes or offsets); `notes.txt` (UTC, one line per step; for Part H it names no id, date, folder or mark); for Part H, curl's `size_upload` field (the request's size) is left out of `times.txt`; `SCREENSHOTS.txt` (name, size, sha256; the captures stay local); `SHA256.txt` over the committed files. Part D's requests and answers are committed in `D/` with their sha256. Netstat captures are cut to the port-9001 lines, with the cut recorded in `substitutions.txt`.
