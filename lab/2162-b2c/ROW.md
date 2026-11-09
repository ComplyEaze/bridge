# Row 2162-B2C: where does Tally's GSTR-1 place an invoice to an unregistered customer?

Not for merge. Part of #2162 (ADR 0004, the 2026-10-31 checklist, item 19: where Tally's GSTR-1 places a posted invoice, its B2C half). Written 9 Nov 2026. Reads and screen reads only: nothing is sent that writes, and nothing is keyed that saves. Confidence of everything here until the row runs: unverified.

## Question
Item 19 owes where Tally's GSTR-1 places an invoice of the build's shape, under B2B for a registered customer and under B2C for an unregistered one. The B2B half was read on this book by row 2162-S13a2 (`lab/2162-s13a2-answers` at 58c5efa4, PARTIAL: a screen read): one invoice to a registered customer, counted under B2B Invoices - 4A, 4B, 4C, 6B, 6C. That row's summary also showed B2C (Small) Invoices - 7 with a count, but no list under it was opened.
- **This row reads the B2C half:** whether `BP/26-27/0010`, the invoice the build's request created on this book for the unregistered `BRIDGE Walk-in` (dated 1 Aug 2026, read included by S13a2's `s2`), is listed under B2C (Small), and on which line.
- **The month is read beside the screen.** The build's month read (W) lists every voucher of 2026-08 with its class, so the screen's counts can be set against the vouchers the book holds.
- **The cause item 19 also names is not read here.** Section 9.16 records the rehearsal's GSTR-1 counting every voucher of its month as uncertain, for the company's GST registration details being invalid or not specified. That was another company, not one of the lab's synthetic books, and it is not readable from here: the cause is recorded as not isolatable on a book we hold. This book's registration reads valid (S13a2's GSTR-1 opened on it directly).

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2162-KC's) applies in full. Where it and this sheet differ, the stricter one holds.
- **The book is pinned by GUID.** `count11.py` holds BRIDGE PILOT LAB's GUID as `BOOK`. Its M kind prints whether the COMPANY row's GUID is the book's, and `--same-row` STOPs on any other GUID, whatever the company is called. STOP before the screen is opened unless s1's COMPANY row is the book's.
- Its state, from the lab's last notes on it: rows queued before this one each end with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245.
- **One BK, as a fallback only.** Nothing in this row writes, and row S13a2 found its look at GSTR-1 changed neither the marks nor the read-back. If step 5 finds a change all the same, BK is restored only on the run coordinator's word (C8).
- Write down the release shown on Tally's screen. Row 2162-S13a2 read TallyPrime 7.2 Silver.

**No Bridge build.** Raw reads, one at a time, through `run.py`.

**The sends** are row 2162-KC's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a read answer with no `BODY/DATA/COLLECTION`, or an answer that does not parse (`count11.py`, which prints STOP and exits non-zero). A W answer of more than 300 rows also stops `count11.py`, which then prints only the count; the answer is kept. Check each file's sha256 against `SHA256.txt` before its send. Count every answer with `count11.py` and copy its whole output into RESULTS.md. Close every other Bridge process first.

## Files
Every file is byte-equal to row 2162-KC's file of the same name (`lab/2162-kc-sheet` at 604bd4e0, folder `lab/2162-kc/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml` (the build's read-back for `BP/26-27/0010`), `W-2026-08.xml` (the build's month read for 2026-08) and `count11.py`. Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count11.py a-<label>-*`. A STOP from either script stops the row there.
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date, and write it down.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to a new empty folder. Record the file's name, size and sha256.
2. **The month as the book holds it.** `s2`: `R-0010.xml W-2026-08.xml M-marks.xml`.
   - EXPECT R-0010 one row: `BRIDGE Walk-in`, Unregistered/Consumer, Rajasthan, included Yes and every other status flag No, as S13a2's `s2` read it.
   - Record every W row, and count the rows with BRIDGEVCHISSALES Yes.
   - R-0010 with no row, or not included: record it and go on to step 3 all the same; the screen then shows where Tally puts an invoice that does not read included.
3. **Tally's GSTR-1 screen, read only.** No request is sent and nothing is keyed but what is listed here. Screenshot the whole window at each point marked (shot).
   - **Stop rule for this step:** a prompt, a screen not described here, or a button bar that would have to be used other than as named. Stop, press nothing more, screenshot, and ask.
   - **Never press Enter on a voucher's own line,** which opens it for alteration. Never press a key that resolves, accepts or marks anything.
   - **If any Enter opens a voucher or a master screen, press nothing at all, not even Esc** (C7), and stop. Esc in this step is for report screens only.
   1. At the Gateway (shot), press Alt+G (Go To) and type `GSTR-1`. S13a2 found the List of Reports with no line highlighted: one Down put the highlight on `GSTR-1`, then Enter. Press Enter only on the line that reads `GSTR-1`. If Tally first shows a list of GST registrations, it should hold one line, `08ZZZZZ0000Z1ZQ` (Rajasthan): screenshot it and press Enter on that line. A list of more than one line is a stop. (shot)
   2. The period. S13a2 found the report opened on 1-Aug-26 to 31-Aug-26, with a button bar offering `F2: Period` and no Alt+F2. If it opens on that period, press no period key. If it opens on any other, write down what it shows and the period keys its button bar offers, and stop.
   3. (shot) Write down, as text, the whole summary: Total Vouchers, Included in Return with each of its lines, Not Relevant for This Return, Uncertain Transactions, and every line of the Return View with its count and amounts.
   4. Move with the arrow keys to the line `B2C (Small) Invoices - 7` and press Enter (shot). Write down the screen's title and every line it lists, as text.
      - **If the lines are vouchers,** each with a voucher number: write down whether one reads `BP/26-27/0010`, and that line's every column.
      - **If the lines are places of supply and rates:** write down each. Then move to the line for Rajasthan at the rate `BP/26-27/0010` carries (its read-back at step 2 shows the tax legs) and press Enter (shot). If that screen lists vouchers, write down every line, and whether one reads `BP/26-27/0010`.
      - **If no screen lists voucher numbers,** and the button bar offers `F5: Voucher-wise` (S13a2 saw it on the B2B list), press F5 once (shot) and write down every line. Press F5 once more to set the view back (shot). If the bar does not offer it, write down what it offers, and press nothing more on that screen.
   5. Press Esc one screen at a time back to the Gateway (shot). Write down each screen passed.
4. **The reads after the look.** `s4`: `M-marks.xml R-0010.xml W-2026-08.xml`.
   - `count11.py --same-row a-s1-M-marks.xml a-s4-M-marks.xml`. EXPECT it passes: the look changed neither counter.
   - `count11.py --diff a-s2-R-0010.xml a-s4-R-0010.xml`. EXPECT only `lists compared` and `legs: 6 -> 6`: the look changed no field or flag.
   - EXPECT W's rows as at s2.
   - Any check failing: record it, send nothing more, and report before the close. A screen look that changes the book is a finding of its own.
5. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.

## Record
- Each answer's bytes and sha256, and `count11.py`'s lines.
- The `--diff` and `--same-row` outputs, whole.
- Step 3's screenshots, listed by name, bytes and sha256; the summary and every list written down as text, word for word, with every key pressed.
- The BK file, and every prompt word for word with the key that answered it.
- The release on Tally's screen and the Gateway's date.

## Expected
Estimates from row S13a2, not predictions.
- **s2:** R-0010 as S13a2's `s2` read it.
- **Step 3:** the summary as S13a2's, less its `0053` (B2B empty). `BP/26-27/0010` under B2C (Small), on the Rajasthan line at its rate. No earlier row has opened that list, so where it appears is the row's result.
- **s4:** unchanged from s2 and s1.

## Decides
- **`BP/26-27/0010` listed under B2C (Small):** item 19's B2C half for an invoice of the build's shape to an unregistered customer, PARTIAL: a screen read, one invoice, one book, one release. With S13a2's B2B half, item 19's placement is then read for both kinds of customer; the cause the rehearsal's company showed stays not isolatable on a book we hold.
- **`BP/26-27/0010` under another section, under Uncertain Transactions, or nowhere:** record where. An invoice the read-back calls included that GSTR-1 does not list under B2C goes to the run coordinator before anything is written up.
- **The screen's counts and W's Sales-class rows disagree:** record both. That is a note about the screen, not a finding about this invoice, unless `0010` is the difference.
- **s4 not unchanged:** looking at GSTR-1 changed the book. Report it before any other row uses that screen.

## Code admits the trigger
Lines are at master ae38a75c.
- **The invoice is the build's.** `BP/26-27/0010` was created by the build's request (`render_sales_invoice_xml`, `agent_import_invoice.rs:2664`), the file rows RS, KC and S13a2 start from as `template-import-0010`.
- **The post reads the status the screen reflects.** `gst_status_differences` (`agent_import_invoice.rs:1649`) refuses an invoice that does not read included, so a posted invoice is one that reads as `0010` reads. Where GSTR-1 then lists it is Tally's report, which no request of the build reads.

## Not measured
The rehearsal company's cause; an invoice that reads uncertain; B2C (Large); a customer in another state; any month but 2026-08; an invoice built and posted through Bridge (row S12); and any other book or release.

Row: lab/2162-b2c/
