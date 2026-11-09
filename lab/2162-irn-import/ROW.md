# Row 2162-IRN9: can an import set an invoice's e-invoice details, and does Tally then treat the invoice as e-invoiced?

Not for merge. Part of #2162 (item 9, moved from #1378 item 3). Written 9 Nov 2026. Lines below are at master 10f38816, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
Tally keeps an invoice's e-invoice details as three plain voucher elements, `IRN`, `IRNACKNO` and `IRNACKDATE`. It took them when they were keyed by hand, and then listed that invoice under `IRN Generated` in its e-Invoice report, with no offer to generate one (`lab/1378-irn-capture-answers` at 183ae58c). Whether an import can set them is not measured. Neither is what the report shows for an invoice imported with them.

**This row changes that one variable.** Two Sales invoices, sent one at a time to one book:
- `3`: the invoice as the build renders it for this customer, with the three elements added;
- `4`: the same invoice without them.

Nothing else differs between the two requests except the number and the REMOTEID. After each import the row reads what Tally stored: the vouchers window read with the three elements named, and every native field of that voucher. Then it reads Tally's e-Invoice report on screen, read only, for where each invoice is listed.

## Book
`BRIDGE EINVOICE LAB` (synthetic), GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`, loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2162-BC's) applies in full. Where it and this sheet differ, the stricter one holds.
- **Its state,** from `lab/2162-irn-read-answers` (4 Nov 2026), whose restore left it so: three vouchers, all dated 3-Apr-25:
  - invoice A, Sales `1`, with the e-invoice details;
  - invoice B, Sales `2`, without them;
  - a Payment `1`.

  Marks V 3, M 215. Period 1-Apr-25 to 31-Mar-26, Tally's current date 3-Apr-2025. TallyPrime 7.2 Silver. E-invoicing is on for its Maharashtra registration from 1-Apr-25 (1378, answer 1).
- **The masters the invoices name**, as 1378 made them:
  - `BRIDGE IRN Customer`: Sundry Debtors, not bill-wise, Regular, Karnataka, with an invented GSTIN;
  - `BRIDGE IRN Sales`;
  - `BRIDGE IRN IGST`: Integrated Tax, with no rate.

  The report listed B under `Applicable Tax Ledger is not selected` because of that IGST ledger, and A under `IRN Generated` (1378, answer 6).
- **No BK file is on the lab PC** for this book. Step 1 takes one before the first write (C5), and step 6 restores it after the last (C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's (the ruling of 4 Nov 2026 on unchanged marks).
- Write down the release shown on Tally's screen.

**No Bridge build.** Raw requests, one at a time, through `run.py` (row 2162-BC's, byte-equal). The build cannot send these invoices: Sales is not a qualified voucher type, and no builder writes the three elements.

**The sends** are row 2162-BC's. Each of these stops the run and is reported as "the request failed", never as "nothing found":
- in `run.py`: a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes;
- in `count.py`: a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse. It prints STOP and exits non-zero.

Check each file's sha256 before its send: a committed file against `SHA256.txt`, and each `-sent` file against what `fill.py` printed at step 2. Send each write once. Screenshot the whole Tally window after each write. A write is sent only after its own marks read, in a `run.py` call of its own, has passed `count.py --before-write`. Count every answer with `count.py` and copy its whole output into RESULTS.md. Close every other Bridge process first, or record each one that is left running, as row 2162-IRN did; none of them may take a call.

**The GSTINs stay on the lab PC.** The book's GSTINs are invented and are kept local (as in 1378). So:
- `I-3.xml` and `I-4.xml` carry 15 X in their one `PARTYGSTIN` element;
- at step 2, `fill.py` writes `I-3-sent.xml` and `I-4-sent.xml` from them, with the customer's GSTIN as this sitting's own ledger read holds it. It prints each sent file's bytes and sha256, never the GSTIN;
- those two files, and every answer holding a GSTIN, stay on the lab PC, listed by sha256, bytes and name in `kept-local.txt`.

`count.py` prints a GSTIN as `<GSTIN shape>` and counts the shapes in every answer.

## Files
Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.
- **Byte-equal to row 2162-BC's file of the same name** (`lab/2162-bc-sheet`, folder `lab/2162-bc/`): `.gitattributes`, `LAB-COMMON.md`, `run.py` and `template-import-0010.utf16le.xml`. The last is the build's request that created `BP/26-27/0010` on `BRIDGE PILOT LAB` (row 2162-BC's ROW.md says where it first came from).
- **Byte-equal to row 2162-IRN's file of the same name** (`lab/2162-irn-read-sheet`, folder `lab/2162-irn-read/`): `M-marks.xml`, `L-list.xml`, `V-named.xml` and `template-full-1.utf16le.xml`. That row's ROW.md says where each first came from:
  - M reads the company's marks;
  - L lists every voucher of the year;
  - V is the vouchers window read of 2025-04 with `IRN`, `IRNACKNO` and `IRNACKDATE` named;
  - the template is a voucher collection with every native method and no fetch list.
- **`gen.py`** writes these from those files, and asserts every change:
  - `I-4.xml`: `template-import-0010` with:
    - the company `BRIDGE EINVOICE LAB`, the voucher type `Sales`, the date 20250403 (three times), the number `4` (twice), and the REMOTEID tail `0804`;
    - the party `BRIDGE IRN Customer` (six times) and its state `Karnataka` (three times);
    - the registration type `Regular`;
    - a `PARTYGSTIN` element after `CONSIGNEEMAILINGNAME`, where `render_sales_invoice_xml` writes it for a customer with a GSTIN in force;
    - its six legs replaced by three: `BRIDGE IRN Customer` Dr 118.00, `BRIDGE IRN Sales` Cr 100.00 and `BRIDGE IRN IGST` Cr 18.00.
  - `I-3.xml`: `I-4.xml` with the number `3`, the REMOTEID tail `0803`, and these three added after `ISINVOICE`:
    - `IRN`: 64 invented hex characters, not A's;
    - `IRNACKNO` `445566`;
    - `IRNACKDATE` 20250403.
  - `F-3.xml`, `F-4.xml`: the template with its filter changed to `$VoucherNumber = "3"` or `"4"`;
  - `K-customer.xml`: the template with `TYPE` Ledger, filtered to `$Name = "BRIDGE IRN Customer"`, as row 2162-BC's K reads.
- **`fill.py`:** step 2, above.
- **`count.py`:** row 2162-BC's `count5.py` with this row's kinds:
  - M: whether the GUID is the book's, ALTVCHID and ALTMSTID;
  - L: each row's number, date, type and MASTERID;
  - V: each row's number, type and date, then each of the three elements with its count, its `TYPE`, and the IRN's length;
  - F: each voucher's number, type and size; its GST header fields; and every direct child of the e-invoice family;
  - K: the customer's ledger row, and how many GSTIN values it holds;
  - I: every counter and every `LINEERROR`;
  - `--diff`: two F answers, field by field.

  Its parsing, STOP rules, `--before-write` and `--same-row` are `count5.py`'s.

**Checked when the sheet was written:**
- `I-3.xml` and `I-4.xml` differ in the REMOTEID, the number (twice) and the three added elements only.
- On 1378's captured answers, `count.py` printed these:
  - marks: GUID yes, M 215;
  - list: 2 rows;
  - named read: A with IRN length 64 and `IRNACKNO` 112233, and B with all three empty;
  - A's and B's full reads, `full-a.xml` and `full-b.xml`: 37 e-invoice family elements each. Their `--diff` gave 1378's nine differences.
- On row 2162-BC's `a-s3r-K-customer.xml`, `fill.py` stopped twice:
  - as captured: no ledger of that name;
  - with the ledger renamed in a copy: state code not 29.

  On a copy with the code also changed, it wrote two files, each differing from its source in exactly 15 bytes. These copies were made only to test the script. None of them is committed or sent.

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count.py a-<label>-*`. A STOP from either script stops the row there.

**Before each write** (steps 3 and 4): `run.py URL s<n>m M-marks.xml`, then `python3 -I count.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`. Only if that passes, and no key was pressed in Tally since, send the write alone: `run.py URL s<n> <write>`.
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE EINVOICE LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period. If the date is not 3-Apr-2025, write it down. Go on only if it is on or after 3-Apr-2025 and within the period: the invoices are dated 3-Apr-25.
   - `s1`: `M-marks.xml L-list.xml V-named.xml K-customer.xml`. EXPECT:
     - M: one COMPANY row, GUID yes, ALTVCHID 3, ALTMSTID 215;
     - L: three rows, all 20250403: Sales `1`, Sales `2` and Payment `1`;
     - V: the same three. Sales `1` has `IRN` length 64, `IRNACKNO` `112233` and `IRNACKDATE` 20250403. The other two have all three present and empty;
     - K: one LEDGER row named `BRIDGE IRN Customer`, not bill-wise, holding one distinct GSTIN value.

     Anything else: record it and stop with nothing keyed. A number `3` or `4` already in L is such a case.
   - **Take BK** by C5, to a new empty local folder. Record the file's name, size and sha256.
2. **The sent files.** `python3 -I fill.py a-s1-K-customer.xml`. Copy the two lines it prints into `kept-local.txt`. A STOP: record it, send nothing, and go to step 6.
3. **Invoice 3, with the details.** `s3m`, the check, then `s3`: `I-3-sent.xml`. Screenshot the window. Then `s3r`: `V-named.xml F-3.xml L-list.xml M-marks.xml`.
   - EXPECT `CREATED` 1 and every other counter 0 except `LASTVCHID`, with no `LINEERROR`. V then holds four rows, F one voucher, L four rows.
   - **A decline is a result** (`CREATED` 0, or a `LINEERROR`): record its counters and every `LINEERROR`, send `s3r` once, and go on to step 4.
   - **A dialog on Tally's screen after the send:** C7. Stop with no key pressed.
4. **Invoice 4, without them.** `s4m`, the check, then `s4`: `I-4-sent.xml`. Screenshot the window. Then `s4r`: `V-named.xml F-4.xml L-list.xml M-marks.xml`.
   - EXPECT as step 3, with one more row in V and L.
   - A decline is a result here too: record it. If both invoices were declined, nothing was created: skip step 5 and go to step 6.
   - Then the diff: `count.py --diff a-s3r-F-3.xml a-s4r-F-4.xml`, when both F answers hold one voucher.
5. **Tally's e-Invoice report, read only.** No request is sent and nothing is keyed but what is listed here. The screen names are 1378's. If a screen's words differ, write down the screen's words and go on only if the step is still plain.
   - **Never press Enter on a voucher's own line,** which opens it for alteration. Never press a key that resolves, accepts, marks or sends anything. Never press any of these: `Get IRN Info`, `Manually Update IRN Info`, `F10: Cancel IRN`, `Open e-Invoice Portal`, `Set GST Status`, `Fetch Details Using GSTIN/UIN`, or any send or export action.
   - **If any Enter opens a voucher or a master screen, press nothing at all, not even Esc** (C7), and stop. Esc in this step is for report screens only.
   1. At the Gateway, press Alt+G (Go To), type `e-Invoice`, and press Enter on the e-Invoice report line under GST Reports. A list of more than one registration is a stop.
   2. Set the period with Alt+F2 (Period): From `1-4-2025`, To `3-4-2025`, Enter. Do not use F2, which changes Tally's current date. Screenshot the whole report, with every line's count.
   3. Open each line that shows a count, with Enter, one at a time, to its list of causes or vouchers. Screenshot it, and write down which of `1` to `4` it names. Open a cause line the same way. Esc back, one screen at a time. 1378 found that `IRN Generated` opens a register offering `F10: Cancel IRN`, `Manually Update IRN Info` and `Get IRN Info`: read it, press none of them, and Esc back.
   4. Write down, for each of `1` to `4`, every line and cause that names it.
   5. Press Esc one screen at a time back to the Gateway. Screenshot the Gateway.
   6. `s5m`: `M-marks.xml`, then `count.py --same-row a-s4r-M-marks.xml a-s5m-M-marks.xml`. EXPECT it passes. A failure: record it and go to step 6.
6. **The restore**, if anything was created. If nothing was, send only `s9` below and its check. C6, as row 2162-IRN's step 5 did on this book:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml L-list.xml V-named.xml`, and `count.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes, with L's and V's rows equal to `s1`'s.
7. **Close** by C10. One tally.exe owns 9001.

## Stop
**Stop with no key pressed** in any of these cases:
- a dialog appears that this sheet does not name;
- the gateway stops answering;
- a screen offers an action that would reach the portal.

Screenshot the window, keep the answers, and ask before anything else is pressed or sent.

Also stop at a write whose answer is neither `CREATED` 1 with `ERRORS` 0 and `EXCEPTIONS` 0, nor a decline in the answer. And at `run.py` stopping: do not resend, and go to step 6 if anything was created.

## Record
- Each answer's bytes and sha256, and `count.py`'s lines. The answers stay on the lab PC, listed in `kept-local.txt`:
  - the K and F answers, which hold the book's GSTINs;
  - the two `-sent` files.

  `a-s3-I-3-sent.xml`, `a-s4-I-4-sent.xml` and the s1, s3r, s4r and s9 answers of M, L and V may be committed under `answers/`, if each holds no GSTIN shape (`count.py` prints the count).
- The `--diff` output of step 4, whole.
- Step 5's screenshots, listed by name, bytes and sha256. Every report line and cause, written down as text, word for word, with its count and the voucher numbers it names.
- The BK file, and every prompt word for word with the key that answered it.
- The release on Tally's screen, the Gateway's date and period, and the screenshot after each write, listed by name, bytes and sha256.

## Expected
Estimates from 1378 and row 2162-IRN, not predictions.
- **s3, s4:** each `CREATED` 1. The build's shape created four invoices on `BRIDGE PILOT LAB` with no `LINEERROR` (row 2162-BC). Whether Tally takes the three elements by import is the row's question, so a decline of `3` alone is a result.
- **s3r's V:** not predicted. That `3` carries the three as sent is the first thing to read.
- **s4r's V:** `4` with all three present and empty, as B.
- **F:** the IRN family of `3` against A's, which `count.py` printed as above. Whether the import sets any element A's keying did not, such as `IRNIRPSOURCE`, `ISEINVOVERRIDDEN` or `IRNQRCODE`, is not predicted.
- **The report:** not predicted.
  - `3` under `IRN Generated` beside A would match a keyed invoice.
  - `4` beside B, under `Applicable Tax Ledger is not selected`, would match the tax line's effect that 1378 saw.

## Decides
- **`3` carries the three as sent, and the report lists it under `IRN Generated`:** an import can carry an invoice's e-invoice details, and Tally then treats the invoice as e-invoiced. Item 9 is then buildable: the build may accept the three from the user and write them, through a builder of its own, reviewed and sent to a lab book first (P7). PARTIAL: one book, one release, invented details.
- **`3` carries them, but the report lists it elsewhere:** the elements are stored, but Tally does not treat the invoice as e-invoiced. Record where it lists it. Item 9 then closes as not possible by import, and the build's next step tells the user to enter the details in Tally.
- **`3` is created without them, or declined:** an import cannot set them. Item 9 closes as not possible, and the next step says to enter them in Tally. Record every `LINEERROR` word for word.
- **`4` and the report:** where an invoice imported without details is listed is item 10's first reading. With this book's IGST ledger, it may list for the tax line, as B did. If so, it is not item 10's answer: a clean tax line is the variable to change next.
- **s5m not unchanged:** looking at the report changed the book. Report it.
- **A dialog anywhere:** the row stops (see Stop).

## Code admits the trigger
Lines are at master 10f38816.
- **Sales is not posted today.** `LIVE_QUALIFIED_VOUCHER_TYPES` (`agent_import.rs:421`) holds Journal, Payment, Receipt and Contra. So no release sends either invoice, and this row sends them raw.
- **What the build renders.** `render_sales_invoice_xml` (`agent_import_invoice.rs:2664`) writes the header that `I-4` carries, with `PARTYGSTIN` when a GSTIN is in force (`:2705`). `I-4` is that header for this customer, built from the request the build rendered for `0010`, with this book's names in place of the pilot book's. It is not a render of this build for this book.
- **The variable.** `I-3` is a candidate shape: `I-4` with three elements added. No builder writes them: `InvoiceDetail` (`agent_import_invoice.rs:41`) has no field for them. A builder that writes them is its own reviewed change, sent to a lab book before a release sends it (P7). This row is that first sending.
- **The reads.**
  - M is `render_agent_company_high_water`, as 1378 sent it.
  - L, F and K are lab reads for shape, which the build does not send.
  - V is the vouchers window read with the three elements named, which 1378 sent. The build's `vouchers` read with `include_e_invoice` names the same three (#2296). Whether V is byte-equal to that read's request was not checked.

## Not measured
- an IRN the portal issued, or any portal action;
- a QR code;
- a clean tax line;
- another customer or state;
- a bill-wise customer;
- an invoice built and posted through Bridge;
- any other book or release.

Row: lab/2162-irn-import/
