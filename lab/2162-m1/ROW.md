# Row 2162-M1: the number rule's month reads on the two books its replay tests run on, and where Tally's GSTR-1 places the unregistered invoice and splits its two heading codes

Not for merge. Part of #2162. Written 9 Nov 2026. Lines below are at the branch `fix/2162-number-fold` at f80da634, which starts at master d04f67a9. This branch starts at master e24f1244, and `agent_import.rs`, `agent_import_invoice.rs` and `agent_import_invoice_wire.rs` are unchanged between the two masters, so a line cited on master holds on either. Confidence of everything here until the row runs: unverified.

## Question
The number rule (decided 4 Nov 2026: two invoice numbers match after case and the leading zeros of each digit run are ignored) is to be built as twelve calendar-month reads of the invoice's financial year, each paired, with the control checked inside its month (row S7, `lab/2162-s7-answers` at efbfd95d, met every EXPECT). The build's tests replay captured answers (P1). No month answer has been captured on either book those tests run on.
- **Part A, `BRIDGE PILOT LAB`.** The twelve months of 1 Apr 2026 to 31 Mar 2027, each sent twice, between two marks reads, and the whole year once as a cross-check: S7's `a2` read that year as 41 rows. The twelve months' rows should be the year's rows, element for element, with no row in two months.
- **Part B, the same book, Tally's GSTR-1 screen, read only.** Invoice `BP/26-27/0010` (1 Aug 2026) is to the unregistered `BRIDGE Walk-in` and has two sales ledgers, heading codes 998313 and 998314, on one pair of tax ledgers. Row S13a2 read the August summary: B2C (Small) Invoices - 7 counts 27 vouchers (S13a2 RESULTS, step 5). It did not open that section, nor the HSN Summary. This part opens both, and nothing else.
- **Part C, `BRIDGE EINVOICE LAB`.** The twelve months of 1 Apr 2025 to 31 Mar 2026, each sent twice, between two marks reads. Its April holds the Sales invoice `1` with its e-invoice details: the case the build refuses with its own words (`invoice_number_already_used_with_irn`).

## Books
Synthetic only: the two books named below and no other. **No client book, and nothing seeded from client data.** If either book is not on the PC, has another GUID, or Tally is in Education mode: stop, the part has no book. `LAB-COMMON.md` in this folder (v3, byte-equal to S13a2's) applies in full. Where it and this sheet differ, the stricter one holds.

- **A and B: `BRIDGE PILOT LAB`.** TallyPrime 7.2 Silver, on 9001, loaded alone. S13a2 restored it to ALTVCHID 68 and ALTMSTID 245; S7 wrote nothing. Record the marks found at a1 and go on: nothing in this row writes.
- **C: `BRIDGE EINVOICE LAB`,** GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`, loaded alone. Row 2162-IRN restored it to ALTVCHID 3 and ALTMSTID 215, with three vouchers dated 3 Apr 2025: Sales `1` with the e-invoice details, Sales `2` without, and a Payment `1`. Record the marks found at c0 and go on.

**Read only.** No write, no backup, no restore. No key is pressed in Tally beyond loading and shutting a company, and the keys part B lists. Any prompt when a company opens: press no key, screenshot, stop, and report its words.

## Kit
No Bridge build. Raw requests, one at a time, through `run.py` (S13a2's, byte-equal), which stops at a curl failure, an empty answer, an answer above 2,000,000 bytes, and an Unknown Request. Close every other Bridge process first, and record any that remain.

`gen.py` writes every month request from `template-y-pilot.utf16le.xml` (S7's, byte-equal). It changes only the company and the four dates, and asserts each change occurs exactly as often as expected, as S7's gen7.py did. The generated requests are committed beside it. `SHA256.txt` lists every file with its sha256 and size: check it first.

| file | what |
|---|---|
| `A-M-marks.xml` | S7's `A-M-marks.xml`, byte-equal: the marks request for `BRIDGE PILOT LAB` |
| `A-Y0-year.xml` | S7's `A-Y0-year.xml`, byte-equal: the year's read, sent by S7 as `a2` |
| `A-Y0-2026-04.xml` … `A-Y0-2027-03.xml` | the month read for `BRIDGE PILOT LAB`, one per calendar month of 1 Apr 2026 to 31 Mar 2027, from the 1st to the month's last day |
| `C-M-marks.xml` | row 2162-IRN's `M-marks.xml`, byte-equal: the marks request for `BRIDGE EINVOICE LAB` |
| `C-Y0-2025-04.xml` … `C-Y0-2026-03.xml` | the month read for `BRIDGE EINVOICE LAB`, one per calendar month of 1 Apr 2025 to 31 Mar 2026 |
| `gen.py`, `template-y-pilot.utf16le.xml` | the generator and its template |
| `run.py` | S13a2's `run.py`, byte-equal |
| `count.py` | counts every answer and makes the comparisons below |

**Checked when the sheet was written:**
- The 24 month requests are each what `render_invoice_window_numbers_request` renders for that company and window on the branch, by sha256: the test `the_m1_requests_are_the_ones_the_renderer_writes` (`agent_import_invoice_wire_tests.rs:250`). `A-Y0-year.xml` is what it renders for the pilot year, and S7's twelve dense months are what it renders for theirs: `the_window_read_is_the_one_the_lab_sent_for_each_month_and_for_a_year` (`:214`). Both tests pass on the branch.
- `calendar_months` (`agent_import_invoice_wire.rs:465`) gives exactly these 24 windows from the two financial years: the same tests take their windows from it.
- `A-M-marks.xml`, `A-Y0-year.xml` and `template-y-pilot.utf16le.xml` are byte-equal to S7's files of those names (`lab/2162-s7-sheet` at b63accd9); `C-M-marks.xml` to row 2162-IRN's `M-marks.xml` (`lab/2162-irn-read-sheet`); `run.py` and `LAB-COMMON.md` to S13a2's (`lab/2162-s13a2-sheet`).
- `count.py` was run, when the sheet was written, on S7's captured pilot answers (`lab/2162-s7-answers` at efbfd95d, `run/`) under this sheet's names: `a1` and `a4` as a1, aend and bend (one COMPANY row, the book's GUID, ALTVCHID 68, ALTMSTID 245; the marks equal); `a2` as a2, and also as a05a and a05b, a DERIVED stand-in for an August answer (41 rows, 35 Sales class; 1 row dated outside the month; the pair equal; the months' rows equal to the year's in order); and the captured empty number answer `pilot-lab-number-new-invoice` as a01a and a01b (0 rows, one distinct 0-row sha256). On a truncated answer of odd length, one of even length, and an answer with no `COLLECTION`, it printed STOP for each and exited 1. A `.pending` file was reported as no answer.

**count.py** reads every answer with expat, never a text search, and its output stays bounded (at most 20 lines of rows per answer). For each answer it gives:
- the bytes and sha256;
- for a marks answer: the COMPANY rows, and of each its GUID, ALTVCHID and ALTMSTID;
- for a month or year answer: the rows (a `VOUCHER` under `COLLECTION`), the counts of `BRIDGEVCHISSALES` Yes and No and of each type name, the first and last date, and each Sales-class row's date, number and whether its `IRN` is non-empty;
- an answer with no `BODY/DATA/COLLECTION`, or that does not parse: STOP, and a non-zero exit. A file still named `.pending` is no answer, never 0 rows.

Its comparisons (`count.py --compare A` or `C`, after the part's last send):
- each pair of sends of one month holds the same rows, element for element;
- no row is dated outside its month, and the months answered (of 12) are named;
- the sum of the twelve first sends' rows;
- **A only:** whether the twelve first sends' rows, taken together, equal `a-a2`'s rows element for element; if not, the rows in one and not the other, at most 20 lines;
- the opening and closing marks: the same GUID, ALTVCHID and ALTMSTID.

## Steps
One at a time. Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count.py a-<label>-*`, and keep the whole output. Record the wall clock at each send. A STOP from either script stops the row there.

### Part A, `BRIDGE PILOT LAB`
0. Check `SHA256.txt`. Step 0 of C2, with no company loaded. Load `BRIDGE PILOT LAB` alone from Select Company (C1). Screenshot. Record Tally's release, the Gateway's current date and period, and the PC's date.
1. **a1.** `a1`: `A-M-marks.xml`. Record the GUID, V and M.
2. **a2, the year once.** `a2`: `A-Y0-year.xml`. EXPECT 41 rows, as S7's `a2`, both classes present, unless a later row left vouchers (record it and go on).
3. **The months.** For each month in order, April 2026 to March 2027: `a<nn>a`: `A-Y0-<month>.xml`, then the same file again as `a<nn>b`. `<nn>` runs `01` for April 2026 to `12` for March 2027.
4. **aend.** `aend`: `A-M-marks.xml`. EXPECT the same COMPANY row as a1 (GUID, ALTVCHID, ALTMSTID). Then `count.py --compare A`.

### Part B, the same book, Tally's GSTR-1 screen, read only
No request is sent, and nothing is keyed but what is listed here. If a screen's words differ from these, write down the screen's words and go on only if the step is still plain.
- **Never press Enter on a voucher's own line,** which opens it for alteration. Never press a key that resolves, accepts or marks anything (Accept As Is or any other button the screen offers).
- **If any Enter opens a voucher or a master screen, press nothing at all, not even Esc** (C7), and stop. Esc in this part is for report screens only.

5. At the Gateway, press Alt+G (Go To), type `GSTR-1`, and press Enter on the line that reads GSTR-1. S13a2 found it opened directly on GST Registration `08ZZZZZ0000Z1ZQ`. If Tally shows a list of GST registrations, it should hold that one line: screenshot it and press Enter on it. A list of more than one line is a stop.
6. Set the period with Alt+F2 (Period): From `1-8-2026`, To `31-8-2026`, Enter. Do not use F2, which changes Tally's current date. Screenshot the whole return summary, with every section's count.
7. **B2C.** Move with the arrow keys to the line B2C (Small) Invoices - 7 and press Enter. Screenshot. Write down, word for word, every line it shows: its columns, and the line or lines that count `BP/26-27/0010` if the screen names vouchers. If it shows a further level of lines (by rate or by place of supply, not vouchers), open the line for Rajasthan at 5% with Enter, screenshot, and Esc back. Then Esc back to the summary.
8. **HSN.** Move to the line HSN Summary - 12 (B2B - B2C Supplies) and press Enter. Screenshot. Write down every line it shows, word for word, with its HSN/SAC, description, quantity, taxable value and tax. If the screen offers separate B2B and B2C lists, open each in turn, screenshot, and Esc back. Then Esc back to the summary.
9. Press Esc one screen at a time back to the Gateway. Screenshot the Gateway.
10. **bend.** `bend`: `A-M-marks.xml`. EXPECT the same COMPANY row as aend: the look changed no counter. Then shut the company by Tally's own Shut Company. Screenshot: no company loaded.

### Part C, `BRIDGE EINVOICE LAB`
11. Step 0 of C2. Load `BRIDGE EINVOICE LAB` alone from Select Company. Screenshot. Record the Gateway's current date and period.
12. **c0.** `c0`: `C-M-marks.xml`. EXPECT the book's GUID; record V and M.
13. **The months.** As step 3, with `c<nn>a` and `c<nn>b` and `C-Y0-<month>.xml`, `01` for April 2025 to `12` for March 2026.
14. **cend.** `cend`: `C-M-marks.xml`. EXPECT the same COMPANY row as c0. Then `count.py --compare C`. Shut the company by Shut Company. Screenshot. One tally.exe owns 9001 (C10).

## Stop rules
S7's, unchanged:
- **Any dialog or prompt:** press no key, screenshot, and send the screenshot and answers to the run coordinator. Ask before anything else.
- **No answer in 15 minutes:** press nothing and kill nothing, curl included. Screenshot and ask. A read given up by its client keeps the gateway busy.
- **The gateway stops answering,** a curl failure, an HTTP code other than 200, or an Unknown Request: run.py stops itself. Record it and ask; resend nothing.
- **Education mode,** or a book not on the PC.
- **A closing mark whose COMPANY row differs from its opening mark's:** record both, report, and send nothing more.

## Record
For each send: the bytes, sha256 and rows; the wall clock; the class and type counts and the date range; count.py's lines. For the row: the marks at each point, the release and dates, every screenshot by name, bytes and sha256, and part B's screens written down as text, word for word.

## Expected
Estimates from earlier rows, not predictions.
- **a2:** 41 rows, 117,992 bytes, byte-equal to S7's `a2` if nothing was posted since. Its rows fall in August 2026 (40) and on 15 Sep 2026 (1).
- **The A months:** August 40 rows and September 1, every other month 0 rows. Every 0-row answer the same bytes (the empty collection of `pilot-lab-number-new-invoice`'s PROVENANCE). The months' rows equal a2's.
- **Part B:** `0010` counted under B2C (Small) at 5%, place of supply Rajasthan. The HSN Summary with one line each for 998313 and 998314. Neither is predicted beyond that.
- **The C months:** April 3 rows (Sales `1` with an `IRN` of length 64, Sales `2` with it empty, the Payment `1`), every other month 0 rows.

## Decides
- **The replay fixtures (A, C).** If each pair of sends is equal and the months tile the year, the first send of each month is committed as the build's month fixture for that book. The build's tests then serve these answers in place of the one-number answers they serve today.
- **The months against the year (A).** If the twelve months' rows equal a2's, the month reads see what the year read sees on this book (S7 measured it on the dense book). If not, the build waits and the row is reported.
- **The 0-row answer (A, C).** If every 0-row month on a book is the same bytes, one fixture serves them. If not, each month has its own.
- **The HSN split (B, ADR row S6).** Two lines, one per heading code, each with its own ledger's taxable value: Tally splits an invoice by heading code on its own, and Bridge needs no rule. One line, or the values pooled: the build must refuse two heading codes on one pair of tax ledgers. Recorded as partial: one invoice, a screen read.
- **The B2C placement (B, ADR row S14).** The section that counts `0010` is item 19's placement for an unregistered customer. Partial: a screen read.
- **A dialog anywhere:** the row stops.

## Code admits the trigger
- **What the build sends today, and what it will send.** Today the number read is `render_invoice_number_request` (`agent_import_invoice_wire.rs:429`), one number over the financial year. The branch adds `render_invoice_window_numbers_request` (`agent_import_invoice_wire.rs:451`, the literal at `:457`): the same request less its number term, for any window, which S7 sent for every dense month and the pilot year. `calendar_months` (`:465`) splits the year. No build sends the window read yet: the branch is the next change, and its tests are what this row's answers feed.
- **No write is possible:** no request here is an import, and no Bridge build runs. Part B sends nothing.

## Keep
On `lab/2162-m1-answers`, with `SHA256.txt`:
- every answer whole (the largest expected is a2's, about 118 KB), with each log;
- count.py's whole output after each step and of each `--compare`;
- a RESULTS.md with one line per send first, then the comparisons, then part B's screens as text;
- a notes file with UTC times, one line per step.

Screens stay on the lab PC, listed by name, size and sha256. Send the sha to the run coordinator before any push.

Row: lab/2162-m1/
