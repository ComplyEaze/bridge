# Row 2162-HS: does Tally's GSTR-1 HSN summary give each sales line of a two-line invoice to its own ledger's code?

Not for merge. Part of #2162 (ADR 0004, the 2026-10-31 checklist, item 10: several sales legs on one pair of tax heads, "the split itself"). Written 9 Nov 2026. One raw import of the build's own two-line invoice under a fresh number, with GSTR-1's HSN summary for 2026-08 read on screen before and after, then a restore from the row's own backup. Confidence of everything here until the row runs: unverified.

## Question
The build admits a Sales invoice with two sales lines on one pair of tax heads (ADR 0004 item 10; `classify_sales_invoice`, `agent_import_invoice.rs:681` at master ae38a75c). `BP/26-27/0010` is such an invoice, with 600.40 on `BRIDGE Svc 998313 5%` and 400.40 on `BRIDGE Svc 998314 5%`. Row 2162-M1 (part B, `lab/2162-m1-answers` at 1f250b86) read GSTR-1's HSN summary for 2026-08 on this book. It showed one line per code, 998313 and 998314, but each line pools every August invoice with that code, so the screen alone could not show which code each of `0010`'s lines went to.

This row makes the difference measurable. It reads the summary, adds one more invoice with known lines, and reads the summary again. The change in each code's line is that invoice's share.
- **The one variable:** one invoice added, `BP/26-27/0072`. It is the build's request that created `0010`, with only the number and the REMOTEID changed: the same date (2026-08-01), party, ledgers, figures and type.
- **What is read:** GSTR-1's HSN summary for 2026-08 before and after, as a screen read. Where the screen offers a list of vouchers under a code's line, that list is read too. Also the number read and the read-back of `0072`, the read-back of `0010`, and the marks.
- **Not asked here:** whether the build's own render of a two-line invoice equals this request (row 2162-S13 and the 0010 provenance cover that); any rate but 5%; B2B.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-KC's, 2162-SC's and 2162-RU's) applies in full. Where it and this sheet differ, the stricter one holds.
- **The book is pinned by GUID.** `count15.py` holds BRIDGE PILOT LAB's GUID as `BOOK`. Its M kind prints whether the COMPANY row's GUID is the book's, and `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called. STOP before anything is sent that writes unless s1's COMPANY row is the book's.
- Its state, from the lab's last notes on it: rows queued before this one each end with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245, and row 2162-M1's HSN summary as below.
- **BK is this row's way back,** taken at step 1 before anything is sent that writes, and restored at step 7 by C6. After the restore every read is compared whole with the reads before the import, and a compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Rows 2162-M1 and 2162-B2C read TallyPrime 7.2 Silver (tally.exe 1.1.7.2).

**No Bridge build.** Raw requests, one at a time, through `run.py`. The import is the one write; nothing is keyed that saves.

**The sends** are row 2162-RU's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a read answer with no `BODY/DATA/COLLECTION`, or an answer that does not parse (`count15.py`, which prints STOP and exits non-zero). Check each file's sha256 against `SHA256.txt` before its send. Count every answer with `count15.py` and copy its whole output into RESULTS.md. Close every other Bridge process first.

## Files
- **Byte-equal to row 2162-RU's file of the same name** (`lab/2162-ru-sheet` at b5418f19, folder `lab/2162-ru/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml` and `R-0010.xml` (the build's read-back for `BP/26-27/0010`).
- **Byte-equal to row 2162-KD's file of the same name** (`lab/2162-keyed-dup-sheet` at c5b8e06e), whose ROW.md says where each came from: `template-import-0010.utf16le.xml` (the build's request that created `BP/26-27/0010`) and `template-n-0001.utf16le.xml` (the build's number read).
- **`count15.py`:** byte-equal to row 2162-GF's (`lab/2162-gf-sheet`); this row uses its M, N, I and R kinds and `--diff`, `--before-write` and `--same-row`, all as row 2162-RU's `count14.py` had them.
- **`gen.py`** writes, from those files, and asserts each change occurs exactly as often as expected:
  - `I-0072.xml`: the import template with the number `BP/26-27/0072` (VOUCHERNUMBER and REFERENCE) and the REMOTEID tail `0872`. Nothing else changes.
  - `N-0072.xml`: the number read for `BP/26-27/0072`.
  - `R-0072.xml`: the read-back with the number `BP/26-27/0072`.
  - Neither the number nor the REMOTEID tail appears in any earlier lab row's folder. The files are committed as generated; run `python3 -I gen.py` only to check them, and compare the hashes it gives with `SHA256.txt`.
- Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count15.py a-<label>-*`. A STOP from either script stops the row there.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to the new empty folder `C:\lab\2162-hs\backup-1`. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0072.xml R-0010.xml M-marks.xml`.
   - EXPECT N-0072 0 rows: no voucher holds the number. One row or more: stop before step 4.
   - EXPECT R-0010 one row: `BRIDGE Walk-in`, included Yes, every other status flag No, 6 legs.
3. **The HSN summary before, read only.** Follow the screen rules below, then:
   1. At the Gateway (shot), press Alt+G (Go To) and type `GSTR-1`. Earlier rows found the List of Reports with no line highlighted: one Down put the highlight on `GSTR-1`, then Enter. Press Enter only on the line that reads `GSTR-1`. If Tally first shows a list of GST registrations, it should hold one line, `08ZZZZZ0000Z1ZQ`: screenshot it and press Enter on that line. More than one line is a stop. (shot)
   2. The period. Rows 2162-B2C and 2162-PB found the report opening on 1-Aug-26 to 31-Aug-26. If it opens on that period, press no period key. If it opens on any other, write down what it shows and the period keys its button bar offers, and stop.
   3. (shot) Write down the summary's counts: Total Vouchers, Included in Return, Not Relevant for This Return and Uncertain Transactions.
   4. Move down one line at a time to `HSN Summary - 12 (B2B - B2C Supplies)` and press Enter (shot). Row 2162-M1 found one list, headed B2C Supplies. If Tally offers a choice of lists instead, choose the one that names B2C and write down every choice it offered. Write down the whole screen as text: its title, its heading, and every line with HSN/SAC, UQC, total, rate, taxable value, each tax and tax amount, plus the Sub Total and Total lines.
   5. **The voucher list under a code, if there is one.** Move to the line for `998313` and press Enter once (shot). If it opens a list of vouchers, write down every line of it, word for word. If it opens a summary by another heading, write that down and press nothing more on it. If it opens a voucher or a master screen, follow the screen rules (press nothing, stop). Then Esc once, back to the HSN list. Do the same on the `998314` line.
   6. Press Esc one screen at a time back to the Gateway (shot). Write down each screen passed.
4. **The import.** `s4m`: `M-marks.xml`, then `count15.py --before-write a-s1-M-marks.xml a-s4m-M-marks.xml`. It must pass; then no key is pressed in Tally before the send.
   - `s4`: `I-0072.xml`, sent once. EXPECT CREATED 1 and every other counter 0, no LINEERROR. The window after it (shot): the Gateway, no dialog. Anything else: record it and stop before step 5.
   - `s4r`: `N-0072.xml R-0072.xml R-0010.xml M-marks.xml`. EXPECT N-0072 one row, Sales class; R-0072 one row, `BRIDGE Walk-in`, the 6 legs as sent, included Yes and every other status flag No; R-0010 as at s2; ALTVCHID one above s4m's.
   - `count15.py --diff a-s4r-R-0010.xml a-s4r-R-0072.xml`. Record the whole output. EXPECT only the number and identity fields to differ, and `legs: 6 -> 6`.
5. **The HSN summary after, read only.** Step 3 again, in full, with the same keys. At 3.3, write down the counts. At 3.4, write down the whole HSN list. At 3.5, open each code's line as before.
6. **The reads after the look.** `s6`: `M-marks.xml R-0072.xml`.
   - `count15.py --same-row a-s4r-M-marks.xml a-s6-M-marks.xml`: EXPECT it passes. The look changed neither mark.
   - `count15.py --diff a-s4r-R-0072.xml a-s6-R-0072.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`.
7. **Restore** BK by C6, with `restore-plan.txt` written first. Then `s7`: `M-marks.xml N-0072.xml R-0010.xml`, and compare whole:
   - `count15.py --same-row a-s1-M-marks.xml a-s7-M-marks.xml`: EXPECT it passes.
   - N-0072 0 rows, and its sha256 equal to s2's.
   - `count15.py --diff a-s2-R-0010.xml a-s7-R-0010.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`, and the sha256 equal to s2's.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
8. **Close** by C10. One tally.exe owns 9001.

**The screen rules for steps 3 and 5:** no request is sent while a report screen is open, and nothing is keyed but what the step lists. Screenshot the whole window at each point marked (shot).
- **Stop rule:** a prompt, a screen not described here, or a button bar that would have to be used other than as named. Stop, press nothing more, screenshot, and ask.
- **Never press Enter on a voucher's own line,** which opens it for alteration. Never press a key that resolves, accepts or marks anything.
- **If any Enter opens a voucher or a master screen, press nothing at all, not even Esc** (C7), and stop. Esc in these steps is for report screens only.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those C6 names, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. A stop between steps 4 and 7 leaves `0072` on the book: say so in the report.

## Record
- Each answer's bytes and sha256, and `count15.py`'s lines.
- The `--diff`, `--before-write` and `--same-row` outputs, whole.
- Steps 3 and 5's screenshots, listed by name, bytes and sha256. As text and word for word: the summary counts, the whole HSN list before and after, every voucher list opened under a code, every key pressed, and every prompt or message with the key that answered it.
- **The difference, as a table:** for each HSN line, its total, taxable value, CGST, SGST and tax before, after, and after minus before. Every line that holds anything in either reading goes in, not only 998313 and 998314.
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **Step 3:** row 2162-M1's list. One heading, B2C Supplies; nine lines. The two codes of `0010`: `998313`, total 11,137.40, 5 %, taxable 10,607.00, CGST 265.21, SGST 265.19, tax 530.40; and `998314`, total 1,366.70, 5 %, taxable 1,301.60, CGST 32.55, SGST 32.55, tax 65.10. Sub Total and Total 14,275.20 / 13,508.70 / 383.26 / 383.24 / 766.50.
- **Step 4:** stored as sent, included, 6 legs. That is what `0010` did.
- **Step 5, if each line goes to its own ledger's code:** `998313` up by taxable 600.40, CGST 15.01, SGST 15.01 and total 630.42; `998314` up by taxable 400.40, CGST 10.01, SGST 10.01 and total 420.42; every other line unchanged. The round off of 0.16 on no line. Row 2162-B2C's voucher register summed the month's invoices at 14,275.36, and row 2162-M1's HSN total was 14,275.20. The 0.16 between them equals `0010`'s round off (the only round off this sheet knows of in the month), so the HSN total is estimated to leave it out.
- **Step 3.5:** not estimated. Whether the HSN line opens a list of vouchers is not known.

## Decides
- **Each code's line rises by exactly that code's sales line** (600.40 under 998313, 400.40 under 998314): Tally gives each sales line of a two-line invoice to its own ledger's code. Item 10's split is measured, PARTIAL (one invoice, one book, one release, a screen read). Section 9.17's two-ledger bullet can say so. No build change.
- **The whole 1,000.80 lands on one code:** Tally takes one code for the whole invoice. A two-line invoice with two codes is then filed under one of them. The build must refuse sales lines whose ledgers carry different HSN/SAC codes, until a row shows a shape that files them apart. That is a switch-on blocker for the coordinator.
- **Any other difference** (lines that do not add up to the invoice, a code line changing that `0072` does not use, or a new line): record it as found. The coordinator decides from the table.
- **Step 3.5 opens a list of vouchers:** the share is then read per invoice as well, from `0010`'s and `0072`'s lines. That is stronger evidence than the difference alone, and the reference cites it first.
- **The round off shows on an HSN line:** record which. The reference then says where Tally files a round off.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build admits this invoice's shape.** `classify_sales_invoice` (`agent_import_invoice.rs:681`) takes one to three sales lines on one pair of tax heads (ADR 0004 item 10). `0010` is the two-line bill the build rendered and Tally stored (section 9.17). Each line's HSN/SAC is filled by Tally from its ledger when the voucher is saved, and Bridge writes none (section 9.16, the HSN/SAC of each sales line in the read-back).
- **Nothing in the build checks how GSTR-1 files the lines.** If they are filed under one code, the build would post such invoices without a word.

## Not measured
- An invoice with three sales lines, or two lines on one code.
- Any rate but 5%, IGST, and B2B (a registered customer's HSN list).
- Codes from a group or the company rather than the ledger.
- An invoice posted through a Bridge build.
- Any other book or release.

Row: lab/2162-hs/
