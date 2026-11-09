# Row 2162-IRN10: with a clean tax line, where does Tally list an invoice imported without e-invoice details?

Not for merge. Part of #2162 (item 10). Written 9 Nov 2026. Lines below are at master ae4bada2, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
Row 2162-IRN9 (`lab/2162-irn-import-answers` at 9cb3ebd0) imported Sales `4`, an invoice without e-invoice details. Tally's e-Invoice report listed it under `Uncertain Transactions (Corrections needed)` > `Applicable Tax Ledger is not selected`, beside the keyed invoice B (Sales `2`). It did not list it as waiting for an IRN. So that row could not say what Tally shows for an imported invoice without details. The tax line was in the way.

**The likely cause is the IGST ledger's tax type.** In 1378's ledger read just after the seed import (`lab/1378-irn-capture-answers`, `reads/l1.xml`), `BRIDGE IRN IGST` reads `TAXTYPE` `GST` with no `GSTDUTYHEAD`, though that read named it. So Tally may not know the ledger is an IGST ledger. 1378 keyed the customer's and the Sales ledger's fields by hand, and its notes record no change to the IGST ledger. Confidence: unverified, until this row's first read.

**This row changes that one variable, on Tally's screen.** It sets the IGST ledger's tax type to IGST and nothing else. Then:
- it reads the report: does B move out of that cause?
- only if it moved, it imports Sales `5`, row 2162-IRN9's invoice `4` with the number `5`, and reads the report again: where is `5` listed, and is it beside B?

Reads come first. If the first read shows the tax type already set, the row changes nothing (step 2).

## Book
`BRIDGE EINVOICE LAB` (synthetic), GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`, loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-BC's and 2162-IRN9's) applies in full. Where it and this sheet differ, the stricter one holds.
- **Its state,** as row 2162-IRN9's restore left it: three vouchers, all dated 3-Apr-25:
  - invoice A, Sales `1`, with the e-invoice details;
  - invoice B, Sales `2`, without them;
  - a Payment `1`.

  Marks V 3, M 215. Period 1-Apr-25 to 31-Mar-26, Tally's current date 3-Apr-2025. TallyPrime 7.2 Silver. E-invoicing is on for its Maharashtra registration from 1-Apr-25.
- **The masters,** as 1378 made them: `BRIDGE IRN Customer` (Sundry Debtors, not bill-wise, Regular, Karnataka, with an invented GSTIN), `BRIDGE IRN Sales`, and `BRIDGE IRN IGST` (Duties & Taxes).
- **No BK file from this row is on the lab PC.** Step 3 takes one before the first change (C5), and step 7 restores it after the last (C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's (the ruling of 4 Nov 2026 on unchanged marks).
- Write down the release shown on Tally's screen.

**No Bridge build.** Raw requests, one at a time, through `run.py` (row 2162-BC's, byte-equal). The build cannot send this invoice: Sales is not a qualified voucher type. The build never alters a master, and the ledger change here is a lab step on Tally's screen, not a request.

**The sends** are row 2162-IRN9's. Each of these stops the run and is reported as "the request failed", never as "nothing found":
- in `run.py`: a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes;
- in `count.py`: a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse. It prints STOP and exits non-zero.

Check each file's sha256 before its send: a committed file against `SHA256.txt`, and `I-5-sent.xml` against what `fill.py` printed at step 6. Send the write once. Screenshot the whole Tally window after the ledger change and after the write. A change, on screen or by request, is made only after its own marks read, in a `run.py` call of its own, has passed `count.py --before-write` (see Steps). Count every answer with `count.py` and copy its whole output into RESULTS.md. Close every other Bridge process first, or record each one that is left running, as row 2162-IRN9 did; none of them may take a call.

**The GSTINs stay on the lab PC,** as in row 2162-IRN9:
- `I-5.xml` carries 15 X in its one `PARTYGSTIN` element;
- at step 6, `fill.py` writes `I-5-sent.xml` from it, with the customer's GSTIN as this sitting's own ledger read holds it. It prints the sent file's bytes and sha256, never the GSTIN;
- that file, and every answer holding a GSTIN, stay on the lab PC, listed by sha256, bytes and name in `kept-local.txt`.

`count.py` prints a GSTIN as `<GSTIN shape>` and counts the shapes in every answer.

## Files
Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.
- **Byte-equal to row 2162-IRN9's file of the same name** (`lab/2162-irn-import-sheet` at 463fbea9, folder `lab/2162-irn-import/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `L-list.xml`, `V-named.xml`, `template-full-1.utf16le.xml`, `I-4.xml` and `K-customer.xml`. That row's ROW.md says where each first came from:
  - M reads the company's marks;
  - L lists every voucher of the year;
  - V is the vouchers window read of 2025-04 with `IRN`, `IRNACKNO` and `IRNACKDATE` named;
  - the template is a voucher collection with every native method and no fetch list;
  - `I-4.xml` is the invoice without details that Tally created on that row as Sales `4`;
  - `K-customer.xml` is the template with `TYPE` Ledger, filtered to `$Name = "BRIDGE IRN Customer"`.
- **`gen.py`** writes these from those files, and asserts every change:
  - `I-5.xml`: `I-4.xml` with the number `5` (twice) and the REMOTEID tail `0805`. Its three legs are row 2162-IRN9's: `BRIDGE IRN Customer` Dr 118.00, `BRIDGE IRN Sales` Cr 100.00 and `BRIDGE IRN IGST` Cr 18.00;
  - `F-5.xml`: the template with its filter changed to `$VoucherNumber = "5"`;
  - `K-igst.xml`, `K-sales.xml`: the template with `TYPE` Ledger, filtered to `$Name = "BRIDGE IRN IGST"` or `"BRIDGE IRN Sales"`, as `K-customer.xml` is.
- **`fill.py`:** row 2162-IRN9's, writing `I-5-sent.xml` only. Step 6, above.
- **`count.py`:** row 2162-IRN9's, with its K kind widened from the customer's ledger to any ledger by name. For each LEDGER row K prints:
  - its name, `PARENT`, `ISBILLWISEON`, `TAXTYPE`, `GSTDUTYHEAD` and how many GSTIN values it holds;
  - every non-empty leaf of the GST and tax family by path;
  - for `BRIDGE IRN IGST`, one gate line: `IGST ledger reads as a GST ledger with duty head IGST: yes` or `no`. It says yes only for `TAXTYPE` `GST` with `GSTDUTYHEAD` `IGST`, the spelling TallyPrime 7.2 stored when this row first ran (its s4r answer, `lab/2162-irn-pending-answers`).

  Its other kinds (M, L, V, F, I), its parsing, its STOP rules, `--diff`, `--before-write` and `--same-row` are unchanged.

**Checked when the sheet was written:**
- `I-5.xml` differs from `I-4.xml` in the number (twice) and the REMOTEID tail only. `F-5.xml` differs from row 2162-IRN9's `F-4.xml` in the number only.
- On 1378's `reads/l1.xml`, the K kind printed five LEDGER rows. `BRIDGE IRN IGST` printed `PARENT` `Duties & Taxes`, `TAXTYPE` `GST`, `GSTDUTYHEAD` absent, and its gate line `no`.
- `fill.py` stopped on that read as captured: its GSTIN is masked as 15 X, which is not of GSTIN shape. On a copy with an invented GSTIN of state code 29 in its place, `fill.py` wrote one file, differing from `I-5.xml` in exactly 15 bytes. The copy was made only to test the script. It is not committed or sent.

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count.py a-<label>-*`. A STOP from either script stops the row there.

**Before each change** (steps 4 and 6): `run.py URL s<n>m M-marks.xml`, then `python3 -I count.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`. Only if that passes, make the change. No key is pressed in Tally between that read and the change, except, at step 4, the keys that open the ledger screen and set its one field.

**The report read** (steps 2, 5 and 6) is read only. No request is sent and nothing is keyed but what is listed here. The screen names are row 2162-IRN9's (its `evidence/s5-screens.txt`). If a screen's words differ, write down the screen's words and go on only if the step is still plain.
- **Never press Enter on a voucher's own line,** which opens it for alteration. Never press a key that resolves, accepts, marks or sends anything. Never press any of these: `Get IRN Info`, `Manually Update IRN Info`, `F10: Cancel IRN`, `Open e-Invoice Portal`, `Set GST Status`, `F5: IRN Register`, `Fetch Details Using GSTIN/UIN`, or any send or export action.
- **If any Enter opens a voucher or a master screen, press nothing at all, not even Esc** (C7), and stop. Esc in the report read is for report screens only.

1. At the Gateway, press Alt+G (Go To), type `e-Invoice`, and press Enter on the e-Invoice report line under GST Reports. A list of more than one registration is a stop.
2. If the period shown is not 1-Apr-25 to 3-Apr-25, set it with Alt+F2 (Period): From `1-4-2025`, To `3-4-2025`, Enter. Do not use F2, which changes Tally's current date. Screenshot the whole report, with every line's count.
3. Open each line that shows a count, with Enter, one at a time, to its list of causes or vouchers. Screenshot it, and write down which of `1`, `2` and `5` it names. Open a cause line the same way. Esc back, one screen at a time.
4. Write down, for each of `1`, `2` and `5`, every line and cause that names it, and the Taxable Amount, Invoice Amount and IGST columns as shown.
5. Press Esc one screen at a time back to the Gateway. Screenshot the Gateway.

Then:
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE EINVOICE LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period. If the date is not 3-Apr-2025, write it down. Go on only if it is on or after 3-Apr-2025 and within the period: the invoice is dated 3-Apr-25.
   - `s1`: `M-marks.xml L-list.xml V-named.xml K-igst.xml K-sales.xml K-customer.xml`. EXPECT:
     - M: one COMPANY row, GUID yes, ALTVCHID 3, ALTMSTID 215;
     - L: three rows, all 20250403: Sales `1`, Sales `2` and Payment `1`;
     - V: the same three. Sales `1` has `IRN` length 64, `IRNACKNO` `112233` and `IRNACKDATE` 20250403. The other two have all three present and empty;
     - K-igst: one LEDGER row named `BRIDGE IRN IGST`, `PARENT` `Duties & Taxes`, `TAXTYPE` `GST`;
     - K-sales: one LEDGER row named `BRIDGE IRN Sales`;
     - K-customer: one LEDGER row named `BRIDGE IRN Customer`, not bill-wise, holding one distinct GSTIN value.

     Anything else: record it and stop with nothing keyed. A number `5` already in L is such a case.
   - Record K-igst's gate line and every leaf it prints, and K-sales' leaves.
2. **The report before any change.** The report read above. EXPECT, from row 2162-IRN9 less its two invoices: `2` under `Uncertain Transactions (Corrections needed)` > `Applicable Tax Ledger is not selected`, and `1` under `IRN Generated` > `IRN updated without QR Code`. Then `s2m`: `M-marks.xml`, and `count.py --same-row a-s1-M-marks.xml a-s2m-M-marks.xml`. EXPECT it passes.
   - **If K-igst's gate line at step 1 was `yes`**, the tax type is not the cause. Record that, change nothing, take no BK, and go to step 8.
   - **If `2` is not under that cause**, record where it is. Change nothing, take no BK, and go to step 8.
3. **Take BK** by C5, to a new empty local folder. Record the file's name, size and sha256. Then `s3`: `M-marks.xml`, and `count.py --same-row a-s1-M-marks.xml a-s3-M-marks.xml`.
4. **Set the IGST ledger's tax type, on Tally's screen.** `s4m`, the check, then:
   - From the Gateway open the ledger for alteration: Alter > Ledger (or Alt+G, `Alter Master`, Ledger), and pick `BRIDGE IRN IGST`. Write down the path and every key used.
   - Before pressing anything on that screen, screenshot it and write down every label and its value, word for word.
   - **Change only the tax type.** Go to it with Enter or the arrows, without typing into any field you pass.
     - If `Type of Duty/Tax` is `GST` and a `Tax Type` (or GST duty head) field shows blank or another value: choose `IGST` from its list (TallyPrime 7.2 offers IGST, CGST, SGST/UTGST and Cess). Screenshot.
     - If that field shows `IGST` already, if `Type of Duty/Tax` is not `GST`, or if the screen has no such field or its list has no `IGST`: change nothing. Screenshot, leave by Ctrl+Q (C7), and go to step 7 (nothing to restore: send only `s9` and its checks).
   - Accept with Ctrl+A. An `Accept ?` prompt is answered Yes (`y`). Any other prompt, sub-screen or dialog: C7, stop with no key pressed, and report. Fields that appear only after the choice are left as they show; write each one down from the screenshot.
   - Screenshot the window. Then `s4r`: `M-marks.xml K-igst.xml`. EXPECT ALTVCHID 3, ALTMSTID one more than s1's, and K-igst's gate line `yes`. Record every leaf K-igst prints, and how it differs from s1's. If the gate line is `no`: record it and go to step 7.
5. **The report after the change.** The report read above. Write down where `1` and `2` are listed. Then `s5m`: `M-marks.xml`, and `count.py --same-row a-s4r-M-marks.xml a-s5m-M-marks.xml`. EXPECT it passes.
   - **If `2` is still under `Applicable Tax Ledger is not selected`**, the tax type alone did not clean the line. Skip step 6 and go to step 7.
6. **Invoice 5, without details.**
   - `python3 -I fill.py a-s1-K-customer.xml`. Copy the line it prints into `kept-local.txt`. A STOP: record it, send nothing, and go to step 7.
   - `s6m`, the check, then `s6`: `I-5-sent.xml`. Screenshot the window. Then `s6r`: `V-named.xml F-5.xml L-list.xml M-marks.xml`.
     - EXPECT `CREATED` 1 and every other counter 0 except `LASTVCHID`, with no `LINEERROR`. V and L then hold four rows, F one voucher.
     - **A decline is a result** (`CREATED` 0, or a `LINEERROR`): record its counters and every `LINEERROR`, send `s6r` once, skip the report and go to step 7.
     - **A dialog on Tally's screen after the send:** C7. Stop with no key pressed.
   - The report read above. Write down where `1`, `2` and `5` are listed. Then `s6p`: `M-marks.xml`, and `count.py --same-row a-s6r-M-marks.xml a-s6p-M-marks.xml`. EXPECT it passes.
7. **The restore,** if anything was changed (step 4 accepted). C6, as row 2162-IRN9's step 6 did on this book:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml L-list.xml V-named.xml K-igst.xml`, and `count.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes, with L's and V's rows equal to `s1`'s, and K-igst's leaves and gate line equal to `s1`'s. If nothing was changed, send `s9` and these checks only, with no restore.
8. **Close** by C10. One tally.exe owns 9001.

## Stop
**Stop with no key pressed** in any of these cases:
- a dialog appears that this sheet does not name;
- the gateway stops answering;
- a screen offers an action that would reach the portal.

Screenshot the window, keep the answers, and ask before anything else is pressed or sent.

Also stop at a write whose answer is neither `CREATED` 1 with `ERRORS` 0 and `EXCEPTIONS` 0, nor a decline in the answer. And at `run.py` stopping: do not resend, and go to step 7 if anything was changed.

## Record
- Each answer's bytes and sha256, and `count.py`'s lines. The answers stay on the lab PC, listed in `kept-local.txt`:
  - the K-customer and F answers, which hold the book's GSTINs;
  - `I-5-sent.xml`.

  The M, L, V, K-igst and K-sales answers and `a-s6-I-5-sent.xml` may be committed under `answers/`, if each holds no GSTIN shape (`count.py` prints the count).
- The ledger screen of step 4, before and after the change: every label and value, word for word, and the keys used.
- Each report read's screenshots, listed by name, bytes and sha256. Every report line and cause, written down as text, word for word, with its count and the voucher numbers it names.
- The BK file, and every prompt word for word with the key that answered it.
- The release on Tally's screen, the Gateway's date and period, and the screenshot after each change, listed by name, bytes and sha256.

## Expected
Estimates from 1378 and row 2162-IRN9, not predictions.
- **s1's K-igst:** the gate line `no`, as 1378's `l1.xml` read after the seed. If it is `yes`, the cause is elsewhere and step 2 ends the change.
- **Step 4:** the screen offers the tax type, and the change raises ALTMSTID by one.
- **Step 5:** not predicted. That `2` leaves `Applicable Tax Ledger is not selected` is the row's first question.
- **s6:** `CREATED` 1, as row 2162-IRN9's `4`.
- **Step 6's report:** not predicted. `5` and `2` under one line that waits for an IRN would match a keyed invoice.

## Decides
- **`2` leaves the cause at step 5, and `5` lists beside it:** the IGST ledger's tax type was the cause. An invoice imported without details is listed as a keyed one is. Write down the line's words, since they are what the build's next step names. Item 10 is then answered, PARTIAL: one book, one release.
- **`2` leaves the cause, but `5` lists elsewhere:** Tally treats an imported invoice without details differently from a keyed one. Record where. Item 10 is answered with that difference.
- **`2` does not leave the cause:** the tax type alone is not the cause. The ledger screen recorded at step 4 and K-sales' leaves name the next variable, such as the Sales ledger's GST details. Item 10 stays open.
- **s1's gate line `yes`:** the tax type is not unset on this book. Item 10 stays open, and the K leaves recorded name the next variable.
- **s2m, s5m or s6p not unchanged:** looking at the report changed the book. Report it.
- **A dialog anywhere:** the row stops (see Stop).

## Code admits the trigger
Lines are at master ae4bada2.
- **Sales is not posted today.** `LIVE_QUALIFIED_VOUCHER_TYPES` (`agent_import.rs:421`) holds Journal, Payment, Receipt and Contra. So no release sends this invoice, and this row sends it raw.
- **What the build renders.** `render_sales_invoice_xml` (`agent_import_invoice.rs:2664`) writes the header that `I-5` carries, with `PARTYGSTIN` when a GSTIN is in force (`:2705`). `I-5` is row 2162-IRN9's `I-4` with another number. It is not a render of this build for this book.
- **The variable is a master the build does not write.** No Bridge request alters a ledger. The change is made on Tally's screen, as a user would make it, and is restored afterwards.
- **The reads.** M and V are as row 2162-IRN9 sent them. L, F and the three K reads are lab reads for shape, which the build does not send.

## Not measured
- an IRN the portal issued, or any portal action;
- the IGST ledger's other fields, or the Sales ledger's GST details, changed one at a time;
- an invoice keyed by hand after the change;
- another customer or state;
- a bill-wise customer;
- an invoice built and posted through Bridge;
- any other book or release.

Row: lab/2162-irn-pending/
