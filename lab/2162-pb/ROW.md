# Row 2162-PB: the S13a2 invoice to a registered customer, with a GSTIN under another PAN

Not for merge. Part of #2162 (ADR 0004, the 2026-10-31 checklist, item 18: a registered customer that is not bill-wise; and item 19's B2B placement). Written 9 Nov 2026. Lines below are at master ae38a75c, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
Row 2162-S13a2 (`lab/2162-s13a2-answers` at 58c5efa4) sent the build's invoice `BP/26-27/0053` to `BRIDGE Customer RJ`, a registered customer that is not bill-wise, whose GSTIN shares the company's PAN part. Tally stored it as sent and read it included, not uncertain, and its GSTR-1 counted it under B2B. That result is PARTIAL in one more way than the raw import: a customer under the company's own PAN is not the ordinary case.
- **This row changes one variable: the PAN part of the customer's GSTIN.** Everything else is S13a2's: the ledger, the invoice, the number, the REMOTEID, the state code 08, the entity code 3 and Z. The check character follows from the PAN part.
- The question is S13a2's: does Tally store the invoice as sent, does it read included and not uncertain, and where does GSTR-1 count it?

## The buyer's PAN part stays on the lab PC
Public files of this repository carry GSTINs only in one synthetic family, whose PAN part is the company's. A GSTIN under another PAN is refused by the public-text gate, and this sheet does not carry one, in whole or in pieces.
- The run coordinator sends the lab the PAN part privately. The lab puts it, alone on one line, in `buyer-pan.local` beside this file.
- `gen10.py` builds the two requests that carry the GSTIN from it, on the lab PC. They, `buyer-pan.local` and `filled-sha256.local.txt` are named in `.gitignore` and are never committed. RESULTS.md lists the two requests by sha256 and size, from `filled-sha256.local.txt`.
- `count12.py` will not run without `buyer-pan.local`. Every line it prints shows the PAN part as `<PAN-B>`, so its output may be copied into RESULTS.md. Write the customer's GSTIN in any note as `08<PAN-B>3Z` followed by the check character `gen10.py` printed, never in full.
- **Before anything is pushed**, run `python3 -I count12.py --holds-pan` on every answer, every evidence and notes file, and RESULTS.md. Each file it marks "stays on the lab PC" is not pushed: it is listed in `kept-local.txt` by name, sha256 and size. A notes file it marks is corrected with the masked form first, and checked again. Screenshots stay on the lab PC in any case (C11).

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-S13a2's and 2162-KC's) applies in full. Where it and this sheet differ, the stricter one holds.
- **The book is pinned by GUID.** `count12.py` holds BRIDGE PILOT LAB's GUID as `BOOK`. Its M kind prints whether the COMPANY row's GUID is the book's, and `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called. STOP before any write unless s1's COMPANY row is the book's.
- Its state, from the lab's last notes on it: rows 2162-S13a2 and 2162-KD each left it restored, its marks equal to their `s1` (ALTVCHID 68, ALTMSTID 245), with no ledger named `BRIDGE Customer RJ` and no `BP/26-27/0053`. Rows queued before this one each end with a restore.
- One BK before the first write, and one restore after the last read (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's (the ruling of 4 Nov 2026 on unchanged marks).
- Write down the release shown on Tally's screen. Row 2162-S13a2 read TallyPrime 7.2 Silver.

**No Bridge build.** Raw requests, one at a time, through `run.py`. The build cannot send this invoice, because Sales is not a qualified voucher type on master.

**The sends** are row 2162-KC's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count12.py`, which prints STOP and exits non-zero). Check each file's sha256 against `SHA256.txt`, or for the two filled requests against `filled-sha256.local.txt`, before its send. Send each write once, and screenshot the whole Tally window after it. Count every answer with `count12.py` and copy its whole output into RESULTS.md. Close every other Bridge process first.

**Before each write** (steps 3 and 4): `run.py URL s<n>m M-marks.xml`, then `python3 -I count12.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`. Only if that passes, and no key was pressed in Tally since, send the write alone: `run.py URL s<n> <write>`.

## Files
Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every committed file with its size.
- **Byte-equal to row 2162-S13a2's file of the same name** (`lab/2162-s13a2-sheet` at 6e157faf, folder `lab/2162-s13a2/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `C-catalogue.xml`, `Q-rates.xml`, `N-0053.xml`, `R-0010.xml`, `R-0053.xml`, `L-seed.xml` and `I-0053.xml`. S13a2's ROW.md says where each came from. `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml` and `R-0010.xml` are also byte-equal to row 2162-KC's.
- **`S13A2-a-s4r-R-0053.xml`:** row S13a2's read-back of its `0053`, as captured (`lab/2162-s13a2-answers` at 58c5efa4, `answers/a-s4r-R-0053.xml`), for the diff of step 4.
- **`gen10.py`**, run on the lab PC at step 0, writes from S13a2's two files and asserts every change:
  - `L-seedB.xml`: `L-seed.xml` with its one GSTIN replaced by the buyer's;
  - `I-0053B.xml`: `I-0053.xml` with its one `PARTYGSTIN` replaced by the buyer's.

  The buyer's GSTIN is `08`, the PAN part, `3Z` and the check character (mod 36), S13a2's rule. `gen10.py` refuses a PAN part that is not five letters, four digits and a letter, and the company's own PAN part. It asserts that the GSTIN is neither the company's nor one the book held before (S13a2's list plus S13a2's customer), and that each file is S13a2's again once the GSTIN is set back.
- **`count12.py`:** row KC's `count11.py` (`lab/2162-kc-sheet` at 604bd4e0) with two changes, each marked PB: it masks the PAN part in everything it prints and stops without `buyer-pan.local`; and `--holds-pan FILE...` says whether each file's decoded text holds the PAN part, in any case.
- **`.gitignore`:** the four lab-PC files above.

When the sheet was written, on a copy of this folder outside the repository with an invented PAN part:
- `gen10.py` refused the company's PAN part and a malformed line, and with the invented part wrote both files, listed their sha256 and printed the check character.
- On S13a2's captured `0053` with its GSTIN replaced by the invented one, `count12.py --diff` against the capture printed `PARTYGSTIN: '08ZZZZZ0000Z3ZO' -> '08<PAN-B>3Z…'`, `lists compared: 1` and `legs: 6 -> 6`, and nothing else. The R kind printed the same masked GSTIN.
- `--holds-pan` marked that copy and both filled requests as staying on the lab PC, and S13a2's capture and `I-0053.xml` as clear.
- Without `buyer-pan.local`, `count12.py` printed STOP and nothing else.

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count12.py a-<label>-*`. A STOP from either script stops the row there.
0. **The filled requests**, before Tally is touched. Put the PAN part the run coordinator sent in `buyer-pan.local`. Run `python3 -I gen10.py`. Record the two lines it prints by sha256 and size, and the check character. Check that `git status` lists none of the four lab-PC files. Anything else: stop.
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date. If it is before 1-Aug-2026, stop: the invoice is dated 1 Aug 2026, and a post-dated invoice is another row's variable.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to a new empty folder. Record the file's name, size and sha256.
2. **The book as found.** `s2`: `C-catalogue.xml Q-rates.xml N-0053.xml R-0010.xml`.
   - EXPECT 0 rows named `BRIDGE Customer RJ` in C and in Q, 0 rows in N-0053, and 1 row in R-0010.
   - Anything else: record it and stop before any write.
3. **The setup ledger.** `s3m`, the check, then `s3`: `L-seedB.xml`. Screenshot the window. Then `s3r`: `C-catalogue.xml Q-rates.xml M-marks.xml`.
   - EXPECT CREATED 1 and every other counter 0, counted from `a-s3-L-seedB.xml` itself.
   - C: one row named `BRIDGE Customer RJ`, with `PARENT` Sundry Debtors and `ISBILLWISEON` No.
   - Q: one row of that name, with one registration row that holds anything: `APPLICABLEFROM` 20260401, `GSTREGISTRATIONTYPE` Regular, `PLACEOFSUPPLY` Rajasthan, and `GSTIN` `08<PAN-B>3Z` with step 0's check character.
   - Any other answer: record it, send no invoice, and go to step 6. A ledger that does not read so is a failed setup, not a finding about the invoice.
4. **The invoice.** `s4m`, the check, then `s4`: `I-0053B.xml`. Screenshot the window. Then `s4r`: `R-0053.xml N-0053.xml M-marks.xml`. Then two diffs:
   - `count12.py --diff a-s2-R-0010.xml a-s4r-R-0053.xml` (against the control, as in S13a2);
   - `count12.py --diff S13A2-a-s4r-R-0053.xml a-s4r-R-0053.xml` (against S13a2's invoice).

   EXPECT CREATED 1 and every other counter 0 except `LASTVCHID`, with no `LINEERROR`. R-0053 holds one row, and N-0053 one row of Sales class.
5. **Tally's GSTR-1 screen, read only.** S13a2's step 5, word for word, with its findings:
   - **Stop rule for this step:** a prompt, a screen not described here, or a button bar that would have to be used. Stop, press nothing more, screenshot, and ask.
   - **Never press Enter on a voucher's own line,** which opens it for alteration. Never press a key that resolves, accepts or marks anything.
   - **If any Enter opens a voucher or a master screen, press nothing at all, not even Esc** (C7), and stop. Esc in this step is for report screens only.
   1. At the Gateway, press Alt+G (Go To), type `GSTR-1`. S13a2 found the List of Reports with no line highlighted: one Down put the highlight on `GSTR-1`, then Enter. Press Enter only on the line that reads `GSTR-1`. If Tally first shows a list of GST registrations, it should hold one line, `08ZZZZZ0000Z1ZQ` (Rajasthan): screenshot it and press Enter on that line. A list of more than one line is a stop. S13a2 saw no such list. Screenshot the report.
   2. The period. S13a2 found the report opened on 1-Aug-26 to 31-Aug-26, with a button bar offering `F2: Period` and no Alt+F2. If it opens on that period, press no period key. If it opens on any other, write down what it shows and the period keys its button bar offers, and stop. Screenshot the whole return summary, and write down every section's count as text.
   3. **If `0053` is uncertain** (step 4's read-back): S13a2's step 5.3, word for word. Move to Uncertain Transactions (Corrections needed), press Enter, and screenshot the list of causes. Open every cause line in turn: Enter, screenshot, Esc back to the list. Write down as text every cause line under which `0053` is listed, with its count, then every other cause line's words and count. Write any GSTIN a cause line shows in the masked form.
   4. **If `0053` is included:** move to the section that counts it (B2B is expected; write down the section's name), press Enter, screenshot the list, and write down its lines as text. Write the GSTIN on `0053`'s line as `08<PAN-B>3Z` and its check character if it is the one sent; if it is not, write "not the GSTIN sent" and nothing of it.
   5. Press Esc one screen at a time back to the Gateway. Screenshot the Gateway.
   6. `s5m`: `M-marks.xml`, then `count12.py --same-row a-s4r-M-marks.xml a-s5m-M-marks.xml`. EXPECT it passes: the look changed neither counter.
   7. `s5r`: `R-0053.xml`, then `count12.py --diff a-s4r-R-0053.xml a-s5r-R-0053.xml`. EXPECT only `lists compared` and `legs: 6 -> 6`: the look changed no field or flag.
   8. Either check failing: record it, send nothing more, and go to step 6.
6. **The restore**, if anything was created. C6, as rows 2162-S13a2 and 2162-KD did, word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml C-catalogue.xml`, and `count12.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes, and C's rows equal to `s2`'s.
7. **Close** by C10. One tally.exe owns 9001.
8. **The screen before the push**, as above: `count12.py --holds-pan` on every file the push would carry, `kept-local.txt` for each it marks.

**Stop**, with no key pressed, if Tally's window shows a dialog after a write, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. Also stop at a write whose answer is neither CREATED 1 with ERRORS 0 and EXCEPTIONS 0, nor a decline in the answer. A decline of the invoice is a result: record its counters and every `LINEERROR`, send R-0053 and N-0053 once, skip step 5, and go to step 6.

## Record
- Each answer's bytes and sha256, and `count12.py`'s lines; the two filled requests' sha256 and size from step 0.
- The three `--diff` outputs, whole.
- Step 5's screenshots, listed by name, bytes and sha256, and every section and cause line written down as text, with any GSTIN in the masked form.
- The BK file, and every prompt word for word with the key that answered it.
- The release on Tally's screen, the Gateway's date, and the screenshot after each write, listed by name, bytes and sha256.
- `kept-local.txt`, and the `--holds-pan` output of step 8.

## Expected
Estimates from row S13a2, not predictions.
- **s2:** as S13a2's s2.
- **s3r:** as in step 3.
- **s4r**, the diff against S13a2's `0053`: `PARTYGSTIN` (`08ZZZZZ0000Z3ZO` to `08<PAN-B>3Z` and the check character); the identity fields may or may not equal S13a2's, and are recorded. Anything else that differs is the row's result, the status flags above all.
- **Step 5:** B2B, one party line.

## Decides
The post refuses an invoice unless every field it wrote reads back (`invoice_readback_differences`) and the status reads included, not overridden and not post-dated (`gst_status_differences`).
- **Included Yes, uncertain No, and B2B:** item 18's first element set is stored as the build writes it, and listed as included, for a customer under another PAN. Step 5's section is item 19's B2B placement for that customer. Both are PARTIAL: a raw import, not a build, one book, one release. S13a2's same-PAN result then is not a special case of the PAN.
- **Uncertain:** the PAN part changes Tally's status, while S13a2's same-PAN customer read included. Step 5's cause text is the result. A cause that names the PAN or the GSTIN goes to the run coordinator before anything is written up.
- **`PARTYGSTIN` or `GSTREGISTRATIONTYPE` not as sent:** the read-back comparison refuses every such post. Report it.
- **s5m or s5r not unchanged:** looking at GSTR-1 changed the book. Report it before any other row uses that screen.
- **A decline:** report its `LINEERROR`, with any GSTIN masked.
- **A dialog anywhere:** the row stops (see Stop).

## Code admits the trigger
Lines are at master ae38a75c.
- **Sales is not posted today.** `LIVE_QUALIFIED_VOUCHER_TYPES` (`agent_import.rs:421`) holds Journal, Payment, Receipt and Contra. So no release sends `I-0053B`, and this row sends it raw.
- **The build admits this customer.** `classify_sales_invoice` (`agent_import_invoice.rs:681`) admits a party whose registration in force names a GSTIN that `gstin_valid` (`:288`) accepts, whose state is the place of supply, whose type is `Regular` and whose catalogue `ISBILLWISEON` is No (S13a2's sheet traces each). The build then refuses only a GSTIN byte-equal to the company's own (`invoice_party_gstin_is_company_own`, `:2357`), which this one is not, by construction (`gen10.py`).
- **What the build renders.** `render_sales_invoice_xml` (`:2664`) writes `PARTYGSTIN` when a GSTIN is in force (`:2705`). `I-0053.xml` is S13a2's rendering of that, and `I-0053B.xml` changes only the GSTIN.
- **The reads.** R is the build's read-back, compared by `invoice_readback_differences` (`:1396`) and `gst_status_differences` (`:1649`). C, Q and N are S13a2's, unchanged.

## Not measured
A customer in another state, a bill-wise registered customer, a ledger keyed on Tally's screen in place of seeded, a GSTIN under another entity code, an invoice built and posted through Bridge (row S12), and any other book or release.

Row: lab/2162-pb/
