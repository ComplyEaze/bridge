# Row 2162-BC: does the buyer and consignee block change what Tally stores, its GST status or its GSTR-1 placement, for a registered and an unregistered customer? And what mailing name does each customer's ledger hold?

Not for merge. Part of #2162 (ADR 0004, item 9 and row S5 of the Sales qualification checklist; the lab's `2162-s5` branches are another row, the number read). Written 9 Nov 2026. Lines below are at master e24f1244, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
The build writes six elements in every invoice's header, the buyer and consignee block: `BASICBUYERNAME`, `PARTYMAILINGNAME` and `CONSIGNEEMAILINGNAME` (each the party ledger's name, because its mailing name is not read), `CONSIGNEESTATENAME` (the party's state), `CONSIGNEECOUNTRYNAME` India and `VATDEALERTYPE` Regular. At the rehearsal the hand-keyed invoices carried them and the two posted ones did not (protocol reference, Voucher writes, section 9.16, "Posted beside keyed"). Every import since has carried them. So nothing yet shows whether they change anything.
- **This row changes that one variable.** Four invoices, sent one at a time to one book:
  - `0053`, the registered customer `BRIDGE Customer RJ`, with the block: S13a2's request, byte-equal (S13a2 stored it as sent and read it included Yes, uncertain No);
  - `0054`, the same invoice without the six elements;
  - `0055`, the unregistered `BRIDGE Walk-in`, with the block: the build's request for `0010` with the number and REMOTEID changed;
  - `0056`, the same invoice without the six elements.
- After each, it reads what Tally stored: the build's read-back, and every native field of that voucher.
- It reads each customer's ledger natively, for its mailing name.
- Then Tally's GSTR-1 screen for August 2026, read only, for where each invoice is counted.

`COUNTRYOFRESIDENCE` stays in all four: the rehearsal's posted invoices are not known to have lacked it, so it is not part of the variable.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to S13a2's) applies in full. Where it and this sheet differ, the stricter one holds.
- Its state, from the lab's last notes on it: S13a2 restored it to ALTVCHID 68 and ALTMSTID 245, and rows S7 and M1 only read it. So neither `BP/26-27/0053` to `0056` nor a ledger named `BRIDGE Customer RJ` is expected in it.
- One BK before the first write and one restore after the last (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's (the ruling of 4 Nov 2026 on unchanged marks).
- The company's own registration is in force: its tax unit reads `GSTREGNUMBER` `08ZZZZZ0000Z1ZQ` (the `pilot-lab-tax-units` fixture), a synthetic GSTIN whose check character is right.
- Write down the release shown on Tally's screen. S13a2 read TallyPrime 7.2 Silver.

**The ledger is setup**, seeded by direct XML under the owner's seed rule: S13a2's `L-seed.xml`, byte-equal. It is never evidence of its own shape.

**No Bridge build.** Raw requests, one at a time, through `run.py` (S13a2's, byte-equal). The build cannot send these invoices, because Sales is not a qualified voucher type.

**The sends** are S13a2's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count5.py`, which prints STOP and exits non-zero). Check each file's sha256 against `SHA256.txt` before its send, send each write once, and screenshot the whole Tally window after each write. A write is sent only after its own marks read, in a `run.py` call of its own, has passed `count5.py --before-write`. Count every answer with `count5.py` and copy its whole output into RESULTS.md. Close every other Bridge process first.

## Files
Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.
- **Byte-equal to S13a2's file of the same name** (`lab/2162-s13a2-sheet` at 6e157faf, folder `lab/2162-s13a2/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `gen.py`, `M-marks.xml`, `C-catalogue.xml`, `Q-rates.xml`, the four templates, `L-seed.xml`, `I-0053.xml`, `R-0053.xml`, `R-0010.xml` and `N-0053.xml`. S13a2's ROW.md says where each first came from, and its `gen.py` makes the last five again from the templates.
- **`template-full-1.utf16le.xml`:** row 2162-IRN's, byte-equal (`lab/2162-irn-read-sheet`): a voucher collection with every native method and no fetch list.
- **`gen5.py`** writes, from those files, and asserts every change:
  - `I-0054.xml`: `I-0053.xml` with the number `BP/26-27/0054` (twice), the REMOTEID tail `0789`, and the six elements removed;
  - `I-0055.xml`: `template-import-0010.utf16le.xml` (the build's request that created `0010`) with the number `BP/26-27/0055` and the REMOTEID tail `0790`;
  - `I-0056.xml`: `I-0055.xml` with the number `BP/26-27/0056`, the REMOTEID tail `0791`, and the six elements removed;
  - `R-0054.xml` to `R-0056.xml` and `N-0054.xml` to `N-0056.xml`: `R-0010.xml` and `N-0053.xml` with the number changed;
  - `V-0053.xml` to `V-0056.xml`: `template-full-1` for `BRIDGE PILOT LAB` over 1 Apr 2026 to 31 Mar 2027, filtered to `$VoucherNumber = "BP/26-27/<n>"`;
  - `K-walkin.xml`, `K-customer.xml`: the same read with `TYPE` Ledger, filtered to `$Name = "<the ledger>"`.
- **`count5.py`:** S13a2's `count.py` with two kinds added:
  - V: each voucher's number, type, party, the six elements, `COUNTRYOFRESIDENCE`, `PARTYGSTIN`, `GSTREGISTRATIONTYPE` and the three status flags, then its number of direct children;
  - K: each ledger's name, `MAILINGNAME`, every `LEDMAILINGDETAILS.LIST` with each child, `STATENAME`, `COUNTRYNAME` and `PARTYGSTIN`.

  When the sheet was written, on S13's captured read-back of its `0053` (94,496 bytes), its R output was byte-equal to S13a2's count.py. As a V answer, the same file printed one row, with the six elements `<absent>` (the read-back does not fetch them). On S13a2's `a-s3r-C-catalogue` answer as a K answer, it printed 19 LEDGER rows by name.

**Checked when the sheet was written:**
- `I-0054.xml` and `I-0056.xml` each hold none of the six elements and one `COUNTRYOFRESIDENCE`. `I-0053.xml` and `I-0055.xml` hold each of the seven once. `PARTYGSTIN` is in `0053` and `0054` only.
- With the six elements put back, and the number and REMOTEID set back, `I-0054` is `I-0053` and `I-0056` is `I-0055`, by gen5.py's construction (each change is asserted).

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count5.py a-<label>-*`. A STOP from either script stops the row there.

**Before each write** (steps 3 and 4): `run.py URL s<n>m M-marks.xml`, then `python3 -I count5.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`. Only if that passes, and no key was pressed in Tally since, send the write alone: `run.py URL s<n> <write>`.
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date. If it is before 1-Aug-2026, stop: the invoices are dated 1 Aug 2026, and a post-dated invoice is another row's variable.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row with the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to a new empty folder. Record the file's name, size and sha256.
2. **The book as found.** `s2`: `C-catalogue.xml Q-rates.xml N-0053.xml N-0054.xml N-0055.xml N-0056.xml R-0010.xml K-walkin.xml`.
   - EXPECT 0 rows named `BRIDGE Customer RJ` in C and in Q, 0 rows in each N, 1 row in R-0010, and 1 LEDGER row in K-walkin.
   - Anything else: record it and stop before any write.
3. **The setup ledger.** `s3m`, the check, then `s3`: `L-seed.xml`. Screenshot the window. Then `s3r`: `C-catalogue.xml Q-rates.xml K-customer.xml M-marks.xml`.
   - EXPECT CREATED 1 and every other counter 0; in C one row named `BRIDGE Customer RJ`, `PARENT` Sundry Debtors, `ISBILLWISEON` No; in Q one registration row: 20260401, Regular, Rajasthan, `08ZZZZZ0000Z3ZO`; in K-customer 1 LEDGER row. As S13a2 read it.
   - Any other answer: record it, send no invoice, and go to step 6.
4. **The four invoices**, in this order: `0053`, `0054`, `0055`, `0056`. For each `<n>`, with the sub-step label `s4a` to `s4d`:
   - `s4am` (and so on), the check, then `s4a`: `I-<n>.xml`. Screenshot the window.
   - Then `s4ar`: `R-<n>.xml N-<n>.xml V-<n>.xml M-marks.xml`.
   - EXPECT CREATED 1 and every other counter 0 except `LASTVCHID`, with no `LINEERROR`; R one row; N one row of Sales class; V one row.
   - A decline is a result: record its counters and every `LINEERROR`, send R, N and V once, and go on to the next invoice.

   Then the two diffs: `count5.py --diff a-s4ar-R-0053.xml a-s4br-R-0054.xml` and `count5.py --diff a-s4cr-R-0055.xml a-s4dr-R-0056.xml`.
5. **Tally's GSTR-1 screen, read only.** No request is sent and nothing is keyed but what is listed here. The screen names are those of S13a2's RESULTS. If a screen's words differ, write down the screen's words and go on only if the step is still plain.
   - **Never press Enter on a voucher's own line,** which opens it for alteration. Never press a key that resolves, accepts or marks anything (Accept As Is or any other button the screen offers).
   - **If any Enter opens a voucher or a master screen, press nothing at all, not even Esc** (C7), and stop. Esc in this step is for report screens only.
   1. At the Gateway, press Alt+G (Go To), type `GSTR-1`, and press Enter on the line that reads GSTR-1. S13a2 found it opened directly on GST Registration `08ZZZZZ0000Z1ZQ`. A list of more than one registration is a stop.
   2. Set the period with Alt+F2 (Period): From `1-8-2026`, To `31-8-2026`, Enter. Do not use F2, which changes Tally's current date. Screenshot the whole return summary, with every section's count. S13a2 read B2B 1 voucher and B2C (Small) 27 before these four.
   3. Open B2B Invoices with Enter: screenshot, write down every line, and Esc back. Open B2C (Small) Invoices with Enter: screenshot, write down every line, and Esc back.
   4. If an Uncertain Transactions line shows a count, open it with Enter: the list of causes. Open every cause line in turn, screenshot its vouchers, and Esc back, as S13a2's step 5 did. Write down, for each of `0053` to `0056`, every section or cause that names it.
   5. Press Esc one screen at a time back to the Gateway. Screenshot the Gateway.
   6. `s5m`: `M-marks.xml`, then `count5.py --same-row a-s4dr-M-marks.xml a-s5m-M-marks.xml`. EXPECT it passes.
   7. `s5r`: `R-0053.xml R-0054.xml R-0055.xml R-0056.xml`, then `count5.py --diff` of each against its step 4 answer. EXPECT only `legs: 6 -> 6` for each.
   8. Either check failing: record it, send nothing more, and go to step 6.
6. **The restore**, if anything was created. C6, and S13a2's step 6 word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml C-catalogue.xml`, and `count5.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes, and C's rows equal to `s2`'s.
7. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog after a write, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. Also stop at a write whose answer is neither CREATED 1 with ERRORS 0 and EXCEPTIONS 0, nor a decline in the answer.

## Record
- Each answer's bytes and sha256, and `count5.py`'s lines.
- The two `--diff` outputs of step 4 and the four of step 5, whole.
- Step 5's screenshots, listed by name, bytes and sha256, and every section and cause line written down as text, word for word.
- The BK file, and every prompt word for word with the key that answered it.
- The release on Tally's screen, the Gateway's date, and the screenshot after each write, listed by name, bytes and sha256.

## Expected
Estimates from earlier rows, not predictions.
- **s4a (0053):** as S13a2's s4: stored as sent, included Yes, uncertain No.
- **s4c (0055):** as `0010` reads (S13a2's s2): included Yes and every other flag No.
- **The V reads:** in `0053` and `0055` the six elements as sent (as the 9 Oct import stored them, section 9.16, "Stored as sent").
- **The V reads of `0054` and `0056`, the diffs and the GSTR-1 placement:** not predicted. They are the row's result. Whether Tally fills any of the six on its own for an invoice that omits them is the first thing to read.
- **K:** not predicted. The seed ledger and `BRIDGE Walk-in` may or may not hold a `MAILINGNAME` or a `LEDMAILINGDETAILS.LIST`.

## Decides
- **Status and placement the same with and without the block, for each customer:** the block does not change Tally's GST view on this book. The build may then keep writing it, and the read-back need not compare the six fields to judge GST status. Item 9's "gap accepted" holds for status. PARTIAL: one book, one invoice of each kind.
- **Either differs:** the block is part of what decides an invoice's GST view. The read-back must then read and compare the six fields, and the build may not drop them. Report which flag or section moved.
- **What V shows for an invoice without the block:** if Tally fills the six itself, record from where, the ledger or the company. If it leaves them empty, an invoice the build posts without them reads differently from a keyed one. That is how the rehearsal's posts read.
- **K, the mailing name:** if a customer's ledger holds a mailing name other than its name, the build's three names are wrong for that customer. A read of the mailing name is then owed before Sales qualifies. If neither ledger holds one, the ledger name is what Tally itself shows for these two. That is partial justification: two synthetic ledgers.
- **s5m or s5r not unchanged:** looking at GSTR-1 changed the book. Report it.
- **A dialog anywhere:** the row stops (see Stop).

## Code admits the trigger
Lines are at master e24f1244.
- **Sales is not posted today.** `LIVE_QUALIFIED_VOUCHER_TYPES` (`agent_import.rs:421`) holds Journal, Payment, Receipt and Contra. So no release sends any of the four, and this row sends them raw.
- **What the build renders.** `render_sales_invoice_xml` (`agent_import_invoice.rs:2664`) writes the six elements in every invoice. The three names are the party ledger's name and the consignee's state is the party's (its doc comment at `:2658-2663`). It writes `PARTYGSTIN` only when a GSTIN is in force. `I-0053` and `I-0055` are what it renders for these two customers, as S13a2's ROW.md showed for `0053` and section 9.16 for `0010`.
- **The variable.** `I-0054` and `I-0056` are what no build renders: the same requests without the six. They are sent to measure what the block does, not as a shape the build will send. A change to drop the block would be its own reviewed builder, sent to a lab book before a release sends it (P7).
- **The reads.**
  - R is `render_invoice_readback_request` (`agent_import_invoice_wire.rs:505`), which does not fetch the six.
  - N is the number read with the e-invoice words.
  - V and K are lab reads of every native field, the kind row 2162-IRN sent as its F read. No build sends them.

## Not measured
A bill-wise customer, a customer in another state, a customer whose ledger holds a mailing name other than its name (unless K finds one), the six elements varied one at a time, an invoice built and posted through Bridge (row S12), and any other book or release.

Row: lab/2162-bc/
