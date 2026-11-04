# Row 2162-S5: today's number read with its number term removed. What does it return for a year: rows, bytes and class per row? And do `BP/26-27/0051` and `BP/26-27/051` both come back when both are in the book?

Question (#2162, the leading-zero rule adopted on 4 Nov). The rule fails closed. Two invoice numbers match when they are equal after ignoring case and the leading zeros of each digit run, and a match refuses the build as `invoice_number_already_used`.
- **Why the year's rows are needed.** Row S4 (`lab/2162-s4-answers` at 3651c746) found that Tally's own `$VoucherNumber =` filter ignores case but not a dropped zero. The read for `BP/26-27/051` missed the seed `BP/26-27/0051`, and the type's guard then created a second invoice under `BP/26-27/051`. So Tally cannot be asked for the rule's match. The build has to read the year's numbers and compare them itself.
- **The candidate request.** The read that returns the year's numbers is today's number read with its number term removed. The protocol reference's §9.16 records that form once: "every voucher of the year with its class (about 2.6 KB a voucher on the wire)". That was before #2325 added `IRN, IRNACKNO, IRNACKDATE` to the read's FETCH. It has not been sent in its current form, so it is a candidate shape (P7), and this row sends it to a lab book before any code is built.
- **What is recorded.** Y's rows, bytes and class per row, on the book as found and with both numbers in it. And whether both numbered rows come back, each equal element for element to the number read's own row for that number.

Book: BRIDGE PILOT LAB (synthetic), loaded alone, in a sitting of its own. **Read only, apart from keying the two numbers and the restore (steps 3, 4 and 6).**
- **Its state, from the lab's last note on it** (S4's RESULTS, "The book now"): it was restored, with marks byte-identical to S4's step 1 (ALTVCHID 68, ALTMSTID 245), then shut with no company loaded. S4's step 2 found neither `BP/26-27/0051` nor `BP/26-27/051` in use at that state. No row since has used this book. **So neither number is expected in the book**, and this sheet keys both and then restores.
- **How they are keyed.** By S4's own two imports, byte-equal: `I-0051.xml` (the seed, S4's step 3) and `I-z051.xml` (S4's sub-row b3, step 7). In S4 each created exactly that invoice, with no dialog, and the restore after them was byte-identical. Nothing is keyed on Tally's screen.
- One BK before the first write and one restore after the last, with the restore checked byte-identical.
- Write down the release shown on Tally's screen.

No Bridge build. Raw requests, one at a time, through `run.py` (S4's, byte-equal).
- It sends `Content-Type: text/xml; charset=utf-16`.
- It stops at a curl failure, and at an answer that says Unknown Request.
- It decodes each answer as UTF-16 little-endian, with or without a byte-order mark (Tally's answers have none).
- Count every answer with `count.py` (an XML parser, never a text search), and copy its whole output into RESULTS.md.
- Close every other Bridge process first.

Files (requests UTF-16LE with a BOM, as sent; `SHA256.txt` lists every file with its size):
- `Y-all.xml`: the number read as the build renders it today, less ` AND $VoucherNumber = "..."`, for BRIDGE PILOT LAB over 1 Apr 2026 to 31 Mar 2027. Its formula is the date window alone. Its FETCH is `DATE, VOUCHERNUMBER, VOUCHERTYPENAME, ISCANCELLED, IRN, IRNACKNO, IRNACKDATE`, and its compute is `BRIDGEVCHISSALES:$$IsSales:$VoucherTypeName`.
- `N-0051.xml`, `N-z051.xml`: the number read as the build renders it today, for `BP/26-27/0051` and `BP/26-27/051` over the same year.
- `M-marks.xml`, `run.py`, `I-0051.xml` (REMOTEID ending 0783) and `I-z051.xml` (0786): `lab/2162-s4-sheet:lab/2162-s4/`, byte-equal.
- `gen.py` makes `Y-all.xml`, `N-0051.xml` and `N-z051.xml` again from `template-n-0001.utf16le.xml`. That template is S4's, byte-equal, and is `lab/1342-capture-requests:lab/1342-capture-requests/r10-number-control-k0.xml`. Its company and window are already this book's. gen.py makes three changes and asserts each:
  1. the FETCH gains the three e-invoice words;
  2. the number is changed, for N;
  3. or the number term is removed, for Y.
- `count.py` prints, for every answer:
  - **M:** whether the GUID is the book's, ALTVCHID and ALTMSTID;
  - **I:** the import counters and every `LINEERROR`;
  - **Y and N:**
    - the rows (a `VOUCHER` with a `VOUCHERNUMBER` child) and the bytes of each row on the wire, from the start of its start tag to the end of its end tag, by expat's byte index over the answer as sent;
    - the total bytes of the rows, and of the rest of the answer;
    - `BRIDGEVCHISSALES` per row;
    - how many rows are numbered `BP/26-27/0051` and how many `BP/26-27/051`;
    - for each row: its five fields, and each e-invoice tag's count and length.

  It also says whether each Y's two numbered rows equal the N rows of the same label, element for element, and whether each pair of Y answers holds the same rows. It was run, when the sheet was written, on S4's captured answers and on one IRN-read answer: its per-row byte spans equal the spans found by hand.

Steps (`python3 -I run.py <gateway URL> <label> <files>`):
1. **Load and check.**
   - Load BRIDGE PILOT LAB alone from Select Company (`LAB-COMMON.md` C1). The IRN row saw no prompt on its load. Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date. If it is before 1-Aug-2026, stop: the seeds are dated 1 Aug 2026, and a post-dated invoice is S1's variable.
   - `run.py URL s1 M-marks.xml`. EXPECT the GUID the book's, with ALTVCHID 68 and ALTMSTID 245. Other marks: record them and go on; step 2 decides the writes.
   - **Take BK:** Tally's Backup screen, this one company, to a new empty folder. The one prompt expected is "Accept ?", answered y. Record the file's name, size and sha256. Any other prompt: stop, press nothing, and ask.
2. **The book as found:** `run.py URL s2 Y-all.xml N-0051.xml N-z051.xml`, then `count.py`.
   - EXPECT 0 rows in each N.
   - If N-0051 holds a Sales-class row, skip step 3. If N-z051 holds one, skip step 4's import.
   - If a row of either number is not Sales class, record it and stop before any write. The refusal reads the class, so such a book is not this row's.
3. **The seed:** `run.py URL s3 I-0051.xml`, then `run.py URL s3r N-0051.xml`. EXPECT CREATED 1, ERRORS 0 and EXCEPTIONS 0, and 1 row in N-0051. Otherwise go to step 6.
4. **The dropped zero:** `run.py URL s4 I-z051.xml`, then `run.py URL s4r Y-all.xml N-0051.xml N-z051.xml M-marks.xml`.
   - EXPECT CREATED 1, ERRORS 0 and EXCEPTIONS 0: S4's b3 measured that the guard does not decline it.
   - A decline here (CREATED 0 with ERRORS 1 or EXCEPTIONS 1) contradicts S4. Record it, run step 5, then step 6.
5. **The repeat:** `run.py URL s5 Y-all.xml`.
6. **The restore**, if anything was created:
   1. Shut the company from the Company menu.
   2. Data > Backup & Restore > Restore, Specify Path, BK's folder. EXPECT one row, BRIDGE PILOT LAB, with BK's date and time. Select it alone, and capture the screen before accepting.
   3. The prompts expected, and the only ones answered: "Accept ?", y; then "Company data folder (<company number>) already exists ...", Replace (r). Any other prompt: stop, press nothing, and ask.
   4. Load the company alone, as in step 1.
   5. `run.py URL s9 M-marks.xml Y-all.xml`. EXPECT M byte-identical to s1's answer, and Y's rows equal to s2's.
7. **Close.** Shut the company. One tally.exe owns 9001.

Capture Tally's window after each import (s3 and s4): the Gateway, or a dialog.

**Stop**, with no key pressed, if Tally's window shows a dialog after an import, or the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. Also stop at an import whose answer is neither CREATED 1 with ERRORS 0 and EXCEPTIONS 0, nor a decline in the answer.

Record:
- for each answer, its bytes and sha256 and `count.py`'s lines;
- for each Y: the rows, the per-row minimum, maximum and mean bytes, the rest of the answer, and the class counts;
- the BK file and every prompt, word for word, with the key that answered it.

Expected:
- **s1:** marks 68 / 245.
- **s2:** N-0051 and N-z051 0 rows each. Y: every voucher the book holds in 2026-27, each with the three e-invoice tags once and empty. The count is not predicted.
- **s3:** CREATED 1, and N-0051 1 row, Sales class.
- **s4r:**
  - Y holds s2's rows plus two: one numbered `BP/26-27/0051` and one `BP/26-27/051`, each Sales class;
  - each of the two equals the N row of its number, element for element;
  - M's ALTVCHID is above s1's.
- **s5:** Y's rows equal s4r's, element for element. The bytes may differ only in the header counters (`CMPINFO`), as in earlier rows.
- **s9:** M byte-identical to s1's, and Y's rows equal to s2's.

Decides:
- **Y returns both numbered rows** (each Sales class, each equal to its N row): the build can apply the adopted rule itself from this one read. The code change then sends this request and compares each row's number by the rule. Y's rows and per-row bytes, times a book's vouchers in a year, size that read. The change sets its bound from them and states it.
- **Y misses either row, or a row differs from its N row:** this form does not serve the rule as built. Report the rows; the code change waits.
- **The per-row bytes** with the e-invoice words replace §9.16's figure, which was measured without them.
- **A dialog anywhere:** the row stops (see Stop).

Code admits the trigger: each request's sender at file:line, and what decides whether it is sent. Lines are at master 187e882b.
- **N, the number read:**
  - It is rendered by `render_invoice_number_request` (agent_import_invoice_wire.rs:429); the literal is at :436. Its filter is the date window AND `$VoucherNumber = "{number}"`, for vouchers of any type. Its FETCH carries the e-invoice words since #2325. Its compute is `$$IsSales:$VoucherTypeName`.
  - It is wrapped by `invoice_number_read` (agent_read_profiles.rs:680). The invoice build asks for it as `Need::Number` (agent_import_invoice.rs:2286) and renders it at :2113.
  - Its answer is counted by `count_sales_vouchers` and `sales_holder_has_irn` (agent_import_invoice_wire.rs:461, :469). A Sales-class row refuses the build as `invoice_number_already_used` or `invoice_number_already_used_with_irn` (agent_import_invoice.rs:2295-2297).
  - When the sheet was written, a render of the literal with this book's company and window was byte-equal to N-0051 and N-z051, and less its number term, to Y-all.
- **Y, the candidate:** that render less its number term. No build sends it today: every caller renders the literal with its number. It is sent here raw to a lab book, as P7 requires of a new shape, before a builder for it is written.
- **I, the two keying imports:**
  - Rendered by `render_sales_invoice_xml` (agent_import_invoice.rs:2655). Whether a Sales post is sent at all is decided by `LIVE_QUALIFIED_VOUCHER_TYPES` (agent_import.rs:419). That list does not include Sales, so no release sends this request.
  - They are S4's requests, byte-equal, and are sent here only to put the two numbers in the book.
- **M, the marks:** `render_agent_company_high_water` (agent_read_profiles.rs:574), S4's request, byte-equal.

Row: lab/2162-s5/
