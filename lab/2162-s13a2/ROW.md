# Row 2162-S13a2: does the S13 invoice to a registered customer still read uncertain when the customer's GSTIN is not the company's own? And what cause does Tally's GSTR-1 screen give?

Question (ADR 0004, item 18 and row S13). S13 sent the build's request for a Sales invoice to a registered customer that is not bill-wise (`BP/26-27/0053`). Tally stored it as sent, but its GST status read included No and uncertain Yes. The control `BP/26-27/0010` reads included.
- S13's customer GSTIN, `08ZZZZZ0000Z1ZQ`, is the company's own. The book's tax unit reads `GSTREGNUMBER` `08ZZZZZ0000Z1ZQ`: the `pilot-lab-tax-units` fixture, with the same book GUID, 6b43e498. So S13 measured an invoice billed to the seller's own GSTIN, and its status says nothing yet about registered customers.
- **This row changes that one variable.** The customer's GSTIN becomes `08ZZZZZ0000Z3ZO`. Every other byte of every request is S13's, including the invoice's number `BP/26-27/0053` and its REMOTEID tail 0788: S13's restore left neither in the book (S13's step 6, s9).
- It then reads, on Tally's GSTR-1 screen, what Tally gives as the invoice's status and, if it is uncertain, the cause. No field carries the cause (agent_gst_status.rs:634). That step presses no key that alters anything and takes screenshots only.
- Sub-row b of S13 (the round off on the debit side) is done, and is not repeated.

Book: BRIDGE PILOT LAB (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to S13's and to `lab/1233-stock-census-slice`'s) applies in full. Where it and this sheet differ, the stricter one holds, except as stated under "The sends".
- **Its state, from the lab's last note on it** (S13's RESULTS, step 6): restored, with the COMPANY row back to ALTVCHID 68 and ALTMSTID 245 and the ledger catalogue byte-equal to S13's step 2 answer, then shut. So neither `BP/26-27/0053` nor a ledger named `BRIDGE Customer RJ` is expected in it.
- One BK before the first write and one restore after the last (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's. The bytes of the marks answer are not compared across the restore and reload, because its CMPINFO counters belong to each load (the ruling of 4 Nov 2026 on unchanged marks).
- Write down the release shown on Tally's screen. S13 read TallyPrime 7.2 Silver.

**The ledger is setup**, seeded by direct XML under the owner's seed rule, as S13 seeded it. It is never evidence of its own shape. `L-seed.xml` is S13's, with the GSTIN changed and nothing else. The GSTIN is of the synthetic family that cannot be issued, and its check character is right, so `gstin_valid` admits it. `gen.py` asserts, before it writes any file, that it is neither the company's GSTIN nor the one other GSTIN the book's ledgers held before S13 (`08ZZZZZ0000Z2ZP`, `BRIDGE Supplier RJ`'s, from S13's `a-s2-Q-rates` answer).

**No Bridge build.** Raw requests, one at a time, through `run.py` (S13's, byte-equal). The build cannot send the invoice, because Sales is not a qualified voucher type. The import is what the build would render for this invoice (see "Code admits the trigger").

**The sends.** S13's, unchanged. `run.py` sends each file with `curl.exe -sS --fail-with-body -H "Content-Type: text/xml; charset=utf-16" --data-binary @<file> -o <answer>`. Each of these stops the run and is reported as "the request failed", never as "nothing found":
- a curl exit other than 0 (run.py);
- an answer that says Unknown Request (run.py);
- an empty answer (run.py);
- an answer above the 2,000,000-byte read bound (run.py; the answer is kept and not used);
- a read answer with no `BODY/DATA/COLLECTION`, an import answer (I or L) with no counters, or an answer that does not parse (`count.py`, which prints STOP and exits non-zero, after the answer's `STATUS` and every `LINEERROR`).

Every other rule of C3 holds: check each file's sha256 against `SHA256.txt` before its send, send each write once, and screenshot the whole Tally window after each write. A write is sent only after its own marks read, in a `run.py` call of its own, has passed `count.py --before-write` (see Steps). Count every answer with `count.py` (an XML parser, never a text search), and copy its whole output into RESULTS.md. Close every other Bridge process first.

## Files

Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.
- **Byte-equal to S13's file of the same name** (`lab/2162-s13-sheet` at c4744e43, folder `lab/2162-s13/`): `.gitattributes`, `LAB-COMMON.md`, `M-marks.xml`, `C-catalogue.xml`, `Q-rates.xml`, `run.py`, the four templates, and the generated `R-0053.xml`, `R-0010.xml` and `N-0053.xml`. S13's ROW.md says where each first came from.
- **`L-seed.xml` and `I-0053.xml`:** S13's, with `08ZZZZZ0000Z1ZQ` replaced by `08ZZZZZ0000Z3ZO` once in each, and no other byte changed. With the GSTIN swapped back, each file's text equals S13's, and each has the same size.
- **`gen.py`:** S13's, with the GSTIN changed, the GSTIN assertions added, and sub-row b's files left out. Run on a fresh copy of the templates, it makes the five generated files byte for byte.
- **`count.py`:** S13's, with one change: an `L-` answer (the setup ledger's import) is counted as an import answer, with every counter and every `LINEERROR`, and stops when it holds no counters. S13's count.py had no L kind, so its seed answer had to be counted from a byte-equal copy under an I name. When this sheet was written:
  - On S13's real seed answer `a-s3-L-seed.xml` (sha256 `1b72152a…`, from `lab/2162-s13-answers` at 96f07160), it printed CREATED 1 and every other counter 0, line for line what S13's count.py printed for the I-named copy.
  - On an L answer with no counters, `STATUS` 0 and a `LINEERROR`, it printed the `LINEERROR`, then STOP, and exited 1.
  - On eight of S13's other answers (M, C, Q twice, N, R twice and I), its output was byte-equal to S13's count.py.
- **`S13-a-s4r-R-0053.xml`:** S13's captured read-back of its `0053`, byte-equal to `lab/2162-s13-answers` at 96f07160, `answers/a-s4r-R-0053.xml` (sha256 `e45fedbd…`, 94,496 bytes). It is never sent. It is the other side of the diff in step 4, so that the change from S13 can be read off directly.

## Steps

Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count.py a-<label>-*`. A STOP from either script stops the row there.

**Before each write** (steps 3 and 4): `run.py URL s<n>m M-marks.xml`, then `python3 -I count.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`. Only if that passes, and no key was pressed in Tally since, send the write alone: `run.py URL s<n> <write>`.
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date. If it is before 1-Aug-2026, stop: the invoice is dated 1 Aug 2026, and a post-dated invoice is S1's variable.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row with the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: record them and go on; step 2 decides the writes.
   - **Take BK** by C5, to a new empty folder. Record the file's name, size and sha256.
2. **The book as found.** `s2`: `C-catalogue.xml Q-rates.xml N-0053.xml R-0010.xml`.
   - EXPECT 0 rows named `BRIDGE Customer RJ` in C and in Q, 0 rows in N-0053, and 1 row in R-0010.
   - Anything else: record it and stop before any write. This book is then not as the sheet expects.
3. **The setup ledger.** `s3m`, the check, then `s3`: `L-seed.xml`. Screenshot the window. Then `s3r`: `C-catalogue.xml Q-rates.xml M-marks.xml`.
   - EXPECT CREATED 1 and every other counter 0, as S13's seed answered, counted from `a-s3-L-seed.xml` itself.
   - C: one row named `BRIDGE Customer RJ`, with `PARENT` Sundry Debtors and `ISBILLWISEON` No.
   - Q: one row of that name, with one registration row that holds anything: `APPLICABLEFROM` 20260401, `GSTREGISTRATIONTYPE` Regular, `PLACEOFSUPPLY` Rajasthan, and `GSTIN` 08ZZZZZ0000Z3ZO.
   - Any other answer: record it, send no invoice, and go to step 6. A ledger that does not read so is a failed setup, not a finding about the invoice.
4. **The invoice.** `s4m`, the check, then `s4`: `I-0053.xml`. Screenshot the window. Then `s4r`: `R-0053.xml N-0053.xml M-marks.xml`. Then two diffs:
   - `count.py --diff a-s2-R-0010.xml a-s4r-R-0053.xml` (against the control, as in S13);
   - `count.py --diff S13-a-s4r-R-0053.xml a-s4r-R-0053.xml` (against S13's invoice).

   EXPECT CREATED 1 and every other counter 0 except `LASTVCHID`, with no `LINEERROR`. R-0053 holds one row, and N-0053 one row of Sales class.
5. **Tally's GSTR-1 screen, read only.** No request is sent and nothing is keyed but what is listed here. The screen names are those of Tally's help pages. If a screen's words differ, write down the screen's words and go on only if the step is still plain.
   - **Stop rule for this step:** a prompt, a screen not described here, or a button bar that would have to be used. Stop, press nothing more, screenshot, and ask.
   - **Never press Enter on a voucher's own line,** which opens it for alteration. Never press a key that resolves, accepts or marks anything (Accept As Is or any other button the screen offers).
   1. At the Gateway, press Alt+G (Go To), type `GSTR-1`, and press Enter on the line that reads GSTR-1. Screenshot.
   2. Set the period with Alt+F2 (Period): From `1-8-2026`, To `31-8-2026`, Enter. Do not use F2, which changes Tally's current date. Screenshot the whole return summary, with every section's count.
   3. **If `0053` is uncertain** (step 4's read-back):
      - Move with the arrow keys to the line Uncertain Transactions (Corrections needed), and press Enter: the list of causes, each with its count. Screenshot.
      - For each cause line, press Enter to see its vouchers, screenshot, and press Esc. Go on until the line that lists `BP/26-27/0053` is found.
      - Write down, as text and exactly as shown, every cause line under which `0053` is listed, with its count. Then write down every other cause line's words and count.
   4. **If `0053` is included:** move to the section that counts it (B2B is expected; write down the section's name), press Enter, screenshot the list, and write down the section's name and count as text.
   5. Press Esc one screen at a time back to the Gateway. Screenshot the Gateway.
   6. `s5m`: `M-marks.xml`, then `count.py --same-row a-s4r-M-marks.xml a-s5m-M-marks.xml`. EXPECT it passes: the look changed neither counter.
   7. `s5r`: `R-0053.xml`, then `count.py --diff a-s4r-R-0053.xml a-s5r-R-0053.xml`. EXPECT only `legs: 6 -> 6`: the look changed no field or flag.
   8. Either check failing: record it, send nothing more, and go to step 6. A screen look that changes the book is a finding of its own.
6. **The restore**, if anything was created. C6, and S13's step 6 word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml C-catalogue.xml`, and `count.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes (GUID, ALTVCHID and ALTMSTID equal `s1`'s), and C's rows equal to `s2`'s.
7. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog after a write, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. Also stop at a write whose answer is neither CREATED 1 with ERRORS 0 and EXCEPTIONS 0, nor a decline in the answer. A decline of the invoice is a result: record its counters and every `LINEERROR`, send R-0053 and N-0053 once, skip step 5, and go to step 6.

## Record

- Each answer's bytes and sha256, and `count.py`'s lines.
- The three `--diff` outputs, whole.
- Step 5's screenshots, listed by name, bytes and sha256, and every cause line and section written down as text, word for word.
- The BK file, and every prompt word for word with the key that answered it.
- The release on Tally's screen, the Gateway's date, and the screenshot after each write, listed by name, bytes and sha256.

## Expected

- **s2:** none of the names or the number present, and R-0010 one row: `BRIDGE Walk-in`, Unregistered/Consumer, included Yes and every other status flag No. That is how S13 read it.
- **s3r:** as in step 3.
- **s4r**, the diff against R-0010:
  - `VOUCHERNUMBER`, `REFERENCE`, `GUID`, `MASTERID`, `ALTERID` and the voucher keys differ, as they did in S13's diff;
  - `PARTYLEDGERNAME` reads `BRIDGE Customer RJ`, `PARTYGSTIN` reads `08ZZZZZ0000Z3ZO` in place of empty, and `GSTREGISTRATIONTYPE` reads `Regular`;
  - leg 1's ledger is `BRIDGE Customer RJ`, and its sign and amount are unchanged;
  - nothing else differs except, possibly, the status flags. `STATENAME` and `PLACEOFSUPPLY` stay Rajasthan, and `ISGSTOVERRIDDEN` and `ISPOSTDATED` stay No.
- **s4r**, the diff against S13's `0053`:
  - `PARTYGSTIN` differs (`08ZZZZZ0000Z1ZQ` to `08ZZZZZ0000Z3ZO`);
  - the identity fields may or may not equal S13's, and are recorded;
  - anything else that differs is the row's result, the flags above all.

  The flags are the row's open question. They are recorded, not predicted.
- **s5m, s5r:** unchanged from s4r.
- **s9:** M's COMPANY row equal to `s1`'s (GUID, ALTVCHID 68, ALTMSTID 245), and C's rows equal to `s2`'s.

## Decides

The post refuses an invoice unless every field it wrote reads back (`invoice_readback_differences`) and the status reads included, not overridden and not post-dated (`gst_status_differences`).
- **Included Yes, uncertain No:** S13's uncertain status came from the customer's GSTIN being the company's own (one variable, one voucher, one book). Item 18's first element set is then stored as the build writes it and listed as included. That is PARTIAL: a raw import, not a build. Step 5's section is item 19's placement for a registered customer (PARTIAL: a screen read).
- **Still uncertain:** the GSTIN being the company's own is not the cause, or not the only one. Step 5's cause text decides the next sitting:
  - a cause about the party or its registration: vary the ledger by one field, its state first, copied from a ledger keyed on Tally's screen and read natively;
  - a cause about the invoice: a twin keyed on Tally's screen against the same ledger, with both read natively;
  - no cause readable: the keyed twin first.
- **`PARTYGSTIN` or `GSTREGISTRATIONTYPE` not as sent:** the read-back comparison refuses every such post. Report it.
- **s5m or s5r not unchanged:** looking at GSTR-1 changed the book. Report it before any other row uses that screen.
- **A decline:** report its `LINEERROR`.
- **A dialog anywhere:** the row stops (see Stop).

## Code admits the trigger

Lines are at master 35c8414c. `agent_import.rs`, `agent_import_invoice.rs` and `agent_import_invoice_wire.rs` are unchanged since a5463e2f, the commit S13 cited, so S13's citations stand.
- **Sales is not posted today.** `LIVE_QUALIFIED_VOUCHER_TYPES` (agent_import.rs:419-424) holds Journal, Payment, Receipt and Contra. So no release sends I-0053, and this row sends it raw.
- **The build admits this customer.** In `classify_sales_invoice` (agent_import_invoice.rs:825-887), a party whose registration in force names a GSTIN is admitted when:
  - `gstin_valid` passes (:288-319);
  - the GSTIN's state is the place of supply (:850);
  - the entry's type is `Regular` (:857);
  - the catalogue's `ISBILLWISEON` is No (:880-886).

  `08` is Rajasthan, the company's state and the template's place of supply. The admission reads the company's registration but keeps only its state (:2338-2347), so it compares the party's GSTIN with the company's neither for S13's GSTIN nor for this one.
- **What the build renders for that party.**
  - `observe` (:2618-2647) sets the party's state from the GSTIN (:2632-2635) and its registration type to `Regular` (:2636-2640).
  - `render_sales_invoice_xml` (:2655-2707) writes `PARTYGSTIN` after `CONSIGNEEMAILINGNAME` exactly when a GSTIN is in force (:2693-2697).
  - It writes no `CONSIGNEEGSTIN` and no company registration.
  - The party's name goes in the seven places the template carries it, and no New Ref is written for a party that is not bill-wise.

  That is I-0053's change set, as in S13.
- **The reads.**
  - R is `render_invoice_readback_request` (agent_import_invoice_wire.rs:505). The post compares its answer with `invoice_readback_differences` (agent_import_invoice.rs:1396; `PARTYGSTIN` and `GSTREGISTRATIONTYPE` at :1437-1444) and `gst_status_differences` (:1649; Uncertain at :1668).
  - Q is the whole-list form of `render_ledger_rates` (agent_import_invoice_wire.rs:876), pinned to the file by `the_ledger_rates_request_is_the_answered_one_with_the_hsn_fields` (agent_import_invoice_wire_tests.rs:894).
  - N is `render_invoice_number_request` (agent_import_invoice_wire.rs:429).

## Not measured

A customer in another state, a bill-wise registered customer, a ledger keyed on Tally's screen in place of seeded, an invoice built and posted through Bridge (row S12), GSTR-1's own placement of an unregistered customer's invoice on this book beyond what step 5's summary shows, and any other book or release.

Row: lab/2162-s13a2/
