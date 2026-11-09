# Row 2162-RS: what does an invoice the build's request created read like after a person saves it again, unchanged, on Tally's screen?

Not for merge. Part of #2162 (ADR 0004, the 2026-10-31 checklist, item 8: the divergent-invoice next step). Written 9 Nov 2026. Lines below are at master ae38a75c, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
When a post reads an invoice as divergent, its next step (`INVOICE_DIVERGENT_NEXT_STEP`, `agent_import_verification.rs:887`) tells a person to open it in Tally and check it. The same text says that saving an invoice again changed it on a test company and changed its AlterID, and that what ComplyEaze Bridge's verification reads after such a save "has not been established". ADR 0004's checklist item 8 owes that measurement (UNVERIFIED; section 9.16, the status paragraph's "NOT established"; S4).
- **This row changes one variable: a save, on Tally's screen, of the invoice exactly as it stands.** Nothing in the invoice and no master is changed. A correction before the save is another row's variable.
- One invoice, `BP/26-27/0057`, to the unregistered `BRIDGE Walk-in`, sent once as the build renders it.
- Before the save it is read twice with no key pressed between, so a difference that reads alone make is not taken for one the save made.
- After the save it is read twice in the same way.
- Each read is the build's read-back (R), the number read (N), every native field of the voucher (V) and the marks (M).

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-BC's and 2162-FY14R's) applies in full. Where it and this sheet differ, the stricter one holds.
- **The book is pinned by GUID.** `count10.py` holds BRIDGE PILOT LAB's GUID as `BOOK` (as rows 2162-KD, 2162-FY14 and 2162-FY14R read it). Its M kind prints whether the COMPANY row's GUID is the book's, and `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called. STOP before any write unless s1's COMPANY row is the book's.
- Its state, from the lab's last notes on it: row 2162-FY14R left it restored, its marks byte-equal to its `s1` (ALTVCHID 68, ALTMSTID 245), and sent no invoice. So `BP/26-27/0057` is not expected in it.
- One BK before the write, and one restore after the last read (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's (the ruling of 4 Nov 2026 on unchanged marks).
- Write down the release shown on Tally's screen. Row 2162-FY14R read TallyPrime Silver, `tally.exe` file version 1.1.7.2.

**No Bridge build.** Raw requests, one at a time, through `run.py` (row 2162-BC's, byte-equal). The build cannot send this invoice, because Sales is not a qualified voucher type on master.

**The sends** are row 2162-BC's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count10.py`, which prints STOP and exits non-zero). Check each file's sha256 against `SHA256.txt` before its send, send the write once, and screenshot the whole Tally window after it. The write is sent only after its own marks read, in a `run.py` call of its own, has passed `count10.py --before-write`. Count every answer with `count10.py` and copy its whole output into RESULTS.md. Close every other Bridge process first.

## Files
Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.
- **Byte-equal to row 2162-BC's file of the same name** (`lab/2162-bc-sheet` at e2b1ca82, folder `lab/2162-bc/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml`, `N-0053.xml` and the four templates. BC's ROW.md says where each first came from.
- **`gen8.py`** writes, from those files, and asserts every change:
  - `I-0057.xml`: `template-import-0010.utf16le.xml` (the build's request that created `BP/26-27/0010`) with the number `BP/26-27/0057` (twice) and the REMOTEID tail `0795`. That is how BC's `gen5.py` made its `I-0055.xml`: with that number and tail set back, `I-0057.xml` is BC's `I-0055.xml` byte for byte (checked when the sheet was written).
  - `R-0057.xml` and `N-0057.xml`: `R-0010.xml` and `N-0053.xml` with the number changed;
  - `V-0057.xml`: `template-full-1` for `BRIDGE PILOT LAB` over 1 Apr 2026 to 31 Mar 2027, filtered to `$VoucherNumber = "BP/26-27/0057"`. With `0057` set back to `0055` it is BC's `V-0055.xml` byte for byte.
- **`count10.py`:** row 2162-FY14R's `count9.py` (`lab/2162-fy14r-sheet` at 6ed4c28e) with two changes, each marked RS:
  - V also prints each row's `GUID`, `MASTERID` and `ALTERID`;
  - `--diff A B`, on two answers of one VOUCHER row each (R or V), also compares every direct child that has children of its own, by tag. It prints how many of that tag each answer holds and how many differ position by position, never their content, then how many such tags it compared. The leaf children and the legs are compared as before.

  When the sheet was written, on BC's captured `a-s4cr-R-0055` and `a-s4dr-R-0056` (`lab/2162-bc-answers`), `--diff` printed the same seven identity fields BC's RESULTS records (ALTERID, GUID, MASTERID, REFERENCE, VOUCHERKEY, VOUCHERNUMBER, VOUCHERRETAINKEY), `lists compared: 1` and `legs: 6 -> 6`. On `a-s4cr-R-0055` against itself it printed no field. On a copy of it with one leg's amount changed (made for the check, not committed), it printed that leg and `list ALLLEDGERENTRIES.LIST: 6 -> 6, 1 differ`. Read as a V answer, `a-s4cr-R-0055` printed one row with its GUID, MASTERID 47 and ALTERID 71. On BC's `a-s4cr-M-marks` and `a-s4dr-M-marks`, `--before-write` passed and `--same-row` printed STOP.

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count10.py a-<label>-*`. A STOP from either script stops the row there.
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date. If it is before 1-Aug-2026, stop: the invoice is dated 1 Aug 2026, and a post-dated invoice is another row's variable.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID (`count10.py` prints it). Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to a new empty folder. Record the file's name, size and sha256. Then `s1bk`: `M-marks.xml`.
2. **The book as found.** `s2`: `N-0057.xml R-0057.xml V-0057.xml R-0010.xml`.
   - EXPECT 0 rows in N-0057, R-0057 and V-0057, and 1 row in R-0010.
   - Anything else: record it and stop before the write.
3. **The invoice.** `s3m`: `M-marks.xml`, then `count10.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. Only if that passes, and no key was pressed in Tally since, send the write alone: `s3`: `I-0057.xml`. Screenshot the whole window.
   - Then `s3r`: `R-0057.xml N-0057.xml V-0057.xml M-marks.xml`.
   - EXPECT CREATED 1 and every other counter 0 except `LASTVCHID`, with no `LINEERROR`; R one row; N one row of Sales class; V one row; ALTVCHID one more than s1's.
   - A decline, or any other answer: record its counters and every `LINEERROR`, send `s3r` once, open no screen, and go to step 7.
4. **The reads again, with no key pressed.** `s4`: `R-0057.xml V-0057.xml M-marks.xml`. Then `count10.py --diff a-s3r-R-0057.xml a-s4-R-0057.xml` and `count10.py --diff a-s3r-V-0057.xml a-s4-V-0057.xml`, and `count10.py --same-row a-s3r-M-marks.xml a-s4-M-marks.xml`.
   - EXPECT no field and no list printed by either diff, `legs: 6 -> 6`, and the marks equal.
   - Anything else: record it, open no screen, and go to step 7. The reads alone changed something, and a save could not be told apart from it.
5. **The save, on Tally's screen.** No request is sent during this step. Screenshot the whole window at each point marked (shot).
   - **Never press Escape on a voucher screen** (C7). The only prompt answered is the voucher screen's own accept, below. Any other prompt or dialog: press no key, screenshot it, copy its text exactly, and stop (C7).
   1. At the Gateway (shot), press Alt+G (Go To), type `Day Book`, and press Enter on the line that reads Day Book (shot).
   2. Set the period with Alt+F2 (Period): From `1-8-2026`, To `1-8-2026`, Enter. Do not use F2, which changes Tally's current date. (shot) Write down every line the Day Book lists, by its Vch No.
   3. Move with the Down arrow, one line at a time, until the highlighted line's Vch No. reads `BP/26-27/0057` (shot, showing that line highlighted). If no line reads it, press nothing more and stop.
   4. Press Enter once. EXPECT the voucher's alteration screen: its title names Alteration and Sales, and its number reads `BP/26-27/0057` (shot). Any other screen: press no key and stop. Write down every field the screen shows, as text.
   5. Change nothing. Press Ctrl+A once.
      - If the prompt "Accept ?" appears, screenshot it and answer `y`.
      - Write down every prompt and message, word for word, with the key that answered it.
   6. (shot) Write down which screen Tally shows. EXPECT the Day Book again. If Tally shows a voucher screen instead, leave it with Ctrl+Q after a screenshot that shows its title (C7).
   7. On the Day Book, a report screen, press Esc once to return to the Gateway (shot). If Esc does not return to the Gateway, press nothing more and stop.
6. **The reads after the save.** `s6`: `R-0057.xml N-0057.xml V-0057.xml M-marks.xml`. Then wait 60 seconds by the clock, pressing nothing, and send `s6b`: `R-0057.xml V-0057.xml M-marks.xml`.
   - `count10.py --diff a-s4-R-0057.xml a-s6-R-0057.xml`, `count10.py --diff a-s4-V-0057.xml a-s6-V-0057.xml`, and the same two diffs of `a-s6` against `a-s6b`.
   - `count10.py --before-write a-s1-M-marks.xml a-s6-M-marks.xml` (the GUID), and the marks of `s4`, `s6` and `s6b` as printed.
   - Nothing is expected here. These answers are the row's result.
7. **The restore**, if anything was created. C6, as rows 2162-BC and 2162-FY14R did, word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml N-0057.xml`, and `count10.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes, and 0 rows in N-0057.
8. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog after the write or at any point of step 5 that the step does not name, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. Also stop at a write whose answer is neither CREATED 1 with ERRORS 0 and EXCEPTIONS 0, nor a decline in the answer.

## Record
- Each answer's bytes and sha256, and `count10.py`'s lines.
- The six `--diff` outputs of steps 4 and 6, whole.
- Step 5's screenshots, listed by name, bytes and sha256; the Day Book's lines and the alteration screen's fields written down as text; every prompt word for word with the key that answered it.
- The BK file, and every prompt of the backup and the restore word for word with the key that answered it.
- The release on Tally's screen, the Gateway's date, and the screenshot after the write, listed by name, bytes and sha256.

## Expected
Estimates from earlier rows, not predictions.
- **s3r:** as row 2162-BC's `0055`: stored as sent, included Yes, uncertain No, excluded No.
- **Step 4:** no difference, since nothing is sent or keyed between the two reads. No earlier row has diffed two such reads of one invoice, so this is the row's control, not a known result.
- **Step 6:** not predicted. The next-step text records only that a save changed an invoice's AlterID on a test company.

## Decides
Each outcome is stated against the code at master ae38a75c, which judges an invoice read-back by two things. `invoice_readback_differences` compares the fields the build wrote. `readback_identity_differences` (`agent_import_invoice.rs:1799`, called at `agent_import.rs:2691`) requires the invoice read-back's GUID and ALTERID to equal those of the voucher the standard readback attributed to the batch, both read at the same verification.
- **GUID and MASTERID the same, ALTERID changed, every other field R compares the same:** a person's save keeps the invoice's identity and changes only its AlterID. Whether a later verification still reads it as this post's invoice then rests on how the standard readback attributes a voucher whose AlterID moved. The write-up reads that from the code against these answers, and corrects the next step's sentence to what is then established.
- **Any field R compares differs** (a leg, the reference, the party, the state, the place of supply, the registration type, a GST status flag): a save changes what the read-back compares. A verification after a save then reads divergent on that field, whatever the save fixed, and the next step must say so.
- **GUID or MASTERID changed, or a second voucher numbered `BP/26-27/0057`:** a save makes another voucher in the read-back's eyes. A verification after it cannot find this post's invoice as created. Report it before anything is written up.
- **A V difference R does not fetch:** a save changes the stored invoice beyond what the read-back compares. Report each such field by name.
- **s6 against s6b:** if they differ, Tally changed the invoice after the save, without a key pressed. The time between is the variable of a later row.
- **ALTVCHID or ALTMSTID moved by the save:** record by how much. A verification that reads marks, as the post does straight after its own write, would see that step.
- **A prompt at the save other than "Accept ?":** the row stops (see Stop), and the prompt's words are the result.

## Code admits the trigger
Lines are at master ae38a75c.
- **Sales is not posted today.** `LIVE_QUALIFIED_VOUCHER_TYPES` (`agent_import.rs:421`) holds Journal, Payment, Receipt and Contra. So no release sends this invoice, and this row sends it raw.
- **What the build renders.** `render_sales_invoice_xml` (`agent_import_invoice.rs:2664`) wrote `template-import-0010`, the request that created `BP/26-27/0010`. `I-0057` is that request with its number and REMOTEID changed, as row BC's `I-0055` was.
- **The next step the row tests.** `INVOICE_DIVERGENT_NEXT_STEP` (`agent_import_verification.rs:887`) is what a divergent invoice's result carries (`mark_invoice_readback`, `:894`). It sends a person to Tally to check the invoice, which is where a save happens.
- **The reads.** R is `render_invoice_readback_request` (`agent_import_invoice_wire.rs:505`). N is the number read with the e-invoice words. V is a lab read of every native field, the kind row 2162-IRN sent as its F read; no build sends it.

## Not measured
A save after a change to the invoice or to one of its masters, a save of an invoice to a registered customer, a save of an invoice that read uncertain or divergent, any other screen the invoice can be saved from, the standard readback's own answer (the verification's window read, which this row does not send), a verification run by a Bridge build, and any other book or release.

Row: lab/2162-rs/
