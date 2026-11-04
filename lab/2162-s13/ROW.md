# Row 2162-S13: does Tally store, as sent, a Sales invoice to a registered customer that is not bill-wise, and one with its round off on the debit side? And does each read back clean?

Question (ADR 0004, the Sales qualification checklist of 31 Oct 2026, item 18 and row S13). The 6 October amendment owes two element sets before Sales is switched on, because the rehearsal never posted them: a registered customer that is not bill-wise, and a round off on the debit side. 0.8.0 posts Sales only to customers that are not bill-wise, so a registered one of them is the ordinary B2B bill of that release. Every earlier invoice row of #2162 sent the bill to an unregistered customer (`BRIDGE Walk-in`, or S3's bill-wise copy of it), with a round off on the credit side. Each sub-row changes one of those two things against the build's own request for `BP/26-27/0010`, and reads the stored invoice back with the read-back the post sends.
- **a:** the party is a registered customer that is not bill-wise, `BRIDGE Customer RJ`, with GSTIN `08ZZZZZ0000Z1ZQ` from 1 Apr 2026.
- **b:** the party is `BRIDGE Walk-in`, as in the template, and the round off is a debit of 0.84 in place of a credit of 0.16, so that the bill closes at 1,050.00.

Book: BRIDGE PILOT LAB (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to `lab/1233-stock-census-slice`'s) applies in full. Where it and this sheet differ, the stricter one holds, except as stated under "The sends".
- **Its state, from the lab's last note on it** (S5's RESULTS, step 6): restored, with marks byte-identical to S5's step 1 (ALTVCHID 68, ALTMSTID 245) and 41 vouchers in 2026-27, then shut. No row since has used this book. So neither `BP/26-27/0053` nor `BP/26-27/0054` is expected in it, and no ledger named `BRIDGE Customer RJ`.
- One BK before the first write and one restore after the last (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's. The bytes of the marks answer are not compared across the restore and reload, because its CMPINFO counters belong to each load (the ruling of 4 Nov 2026 on unchanged marks).
- Write down the release shown on Tally's screen. The earlier rows of #2162 read TallyPrime 7.2 Silver.

**The ledger is setup**, seeded by direct XML under the owner's seed rule, as S3a seeded its bill-wise ledger. It is never evidence of its own shape. `L-seed.xml` is S3a's `L-ledger.xml` with the name, `ISBILLWISEON` No, and one `LEDGSTREGDETAILS.LIST` holding `APPLICABLEFROM`, `GSTREGISTRATIONTYPE`, `PLACEOFSUPPLY` and `GSTIN`. Those are the four children of the one row that `BRIDGE Supplier RJ`, a registered party in this book, carries in the pilot lab's rates answer (`pilot-lab-ledger-rates-whole-10oct.utf16le.xml`). It writes no `PARTYGSTIN`: that row holds none. The GSTIN is of the synthetic family that cannot be issued. Its check character is right, so `gstin_valid` admits it.

**No Bridge build.** Raw requests, one at a time, through `run.py` (S5's, with two stops added; see "The sends"). The build cannot send either invoice: Sales is not a qualified voucher type. The two imports are what the build would render for these two invoices (see "Code admits the trigger"), so the answers transfer to the post that qualifies Sales.

**The sends.** `run.py` sends each file with `curl.exe -sS --fail-with-body -H "Content-Type: text/xml; charset=utf-16" --data-binary @<file> -o <answer>`. Rows S2 to S6 used that line. It writes no headers file and prints no status. So, in place of C3's checks on those two, each of these stops the run and is reported as "the request failed", never as "nothing found":
- a curl exit other than 0 (run.py);
- an answer that says Unknown Request (run.py);
- an empty answer (run.py);
- an answer above the 2,000,000-byte read bound (run.py; the answer is kept and not used);
- a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count.py`, which prints STOP and exits non-zero, after the answer's `STATUS` and every `LINEERROR`).

Every other rule of C3 holds: check each file's sha256 against `SHA256.txt` before its send, send each write once, and screenshot the whole Tally window after each write. A write is sent only after its own marks read, in a `run.py` call of its own, has passed `count.py --before-write` (see Steps). Count every answer with `count.py` (an XML parser, never a text search), and copy its whole output into RESULTS.md. Close every other Bridge process first.

## Files

Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.
- `M-marks.xml`: the company high-water read. `lab/2162-s5-sheet:lab/2162-s5/M-marks.xml`, byte-equal.
- `C-catalogue.xml`: the ledger catalogue with each ledger's parent and `ISBILLWISEON`. `lab/1342-capture-requests:lab/1342-capture-requests/r13-ledger-catalogue-v2.xml`, byte-equal. It is the request behind `pilot-lab-ledger-catalogue-v2.utf16le.xml` (the pilot-lab PROVENANCE, sha256 `28a47c26…`).
- `Q-rates.xml`: the ledger rates and registration read, over 1 Apr 2026 to 1 Aug 2026, every ledger. It is `pilot-lab-ledger-rates-hsn.request.utf16le.xml` on master, byte-equal. A master test pins it to the code's render (see "Code admits the trigger").
- `L-seed.xml`: the setup ledger (above).
- `I-0053.xml` (sub-row a, REMOTEID ending 0788) and `I-0054.xml` (sub-row b, 0789).
- `R-0053.xml`, `R-0054.xml`, and `R-0010.xml` (the control, `BP/26-27/0010`, already in the book): the read-back the post sends.
- `N-0053.xml`, `N-0054.xml`: the number read as the build renders it today, with the e-invoice words.
- `gen.py` makes every `L-`, `I-`, `R-` and `N-` file again from four templates, and asserts each change and how often it occurs:
  - `template-ledger.utf16le.xml`, S3a's `L-ledger.xml`, byte-equal;
  - `template-import-0010.utf16le.xml` and `template-r-0010.utf16le.xml`, S4's, byte-equal. The second is also byte-equal to `pilot-lab-invoice-readback-post-dated.request.utf16le.xml` on master;
  - `template-n-0001.utf16le.xml`, S5's, byte-equal, with S5's change to its FETCH.

  The changes against the import template, and no others:
  - **I-0053:** the number in `REFERENCE` and `VOUCHERNUMBER`, the REMOTEID, the party in its seven places, `<PARTYGSTIN>08ZZZZZ0000Z1ZQ</PARTYGSTIN>` after `CONSIGNEEMAILINGNAME`, and `GSTREGISTRATIONTYPE` `Regular` in place of `Unregistered/Consumer`.
  - **I-0054:** the number, the REMOTEID, the party leg's `AMOUNT` `-1050.00` in place of `-1051.00`, and the round-off leg's `ISDEEMEDPOSITIVE` `Yes` and `AMOUNT` `-0.84` in place of `No` and `0.16`.
- `run.py`: S5's (`lab/2162-s5-sheet`), with the empty-answer and size-bound stops added. It still sends with S5's curl line.
- `count.py`, for each answer, after its `STATUS` and every `LINEERROR`:
  - **M:** each COMPANY row's GUID, with ALTVCHID and ALTMSTID;
  - **C:** the LEDGER rows, and the named ledger's `PARENT` and `ISBILLWISEON`;
  - **Q:** the LEDGER rows, and the named ledger's `PARENT`, `PARTYGSTIN`, `STATENAME` and every registration row that holds anything, with each child;
  - **N:** every VOUCHER row of the collection, split into those with a `VOUCHERNUMBER` and those without, and each number with its class;
  - **I:** every counter and every `LINEERROR`;
  - **R:** the VOUCHER rows, each with the read-back's header fields and each leg's ledger, sign and amount.

  `count.py --before-write S1 M` stops unless M holds exactly one COMPANY row, with the GUID of `s1`'s one row. `count.py --same-row S1 M` also stops unless ALTVCHID and ALTMSTID equal `s1`'s.

  `count.py --diff A B` prints every field and leg that differs between two read-backs of one row each. When the sheet was written, it was run on S4's captured answers and on two pilot-lab fixtures, and its lines matched those files read by hand: `BRIDGE Supplier RJ`'s one registration row, `BRIDGE Walk-in`'s `ISBILLWISEON` No, and S4's 0051 against 051 differing only in number, reference, GUID, MASTERID, ALTERID and the two voucher keys. Its stops were tried on an answer with no collection, `STATUS` 0 and a `LINEERROR` (every read kind stops) and on an import answer with no counters (stops), against S4's real empty number read (`STATUS` 1, 0 rows, no stop). `--before-write` and `--same-row` were tried on S4's marks answers. `run.py`'s five stops were tried with a stand-in for curl.

## Steps

Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count.py a-<label>-*`. A STOP from either script stops the row there.

**Before each write** (steps 3, 4 and 5): `run.py URL s<n>m M-marks.xml`, then `python3 -I count.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`. Only if that passes, and no key was pressed in Tally since, send the write alone: `run.py URL s<n> <write>`.
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date. If it is before 1-Aug-2026, stop: the invoices are dated 1 Aug 2026, and a post-dated invoice is S1's variable.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row with the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: record them and go on; step 2 decides the writes.
   - **Take BK** by C5, to a new empty folder. Record the file's name, size and sha256.
2. **The book as found.** `s2`: `C-catalogue.xml Q-rates.xml N-0053.xml N-0054.xml R-0010.xml`.
   - EXPECT 0 rows named `BRIDGE Customer RJ` in C and in Q, 0 rows in each N, and 1 row in R-0010.
   - Anything else: record it and stop before any write. This book is then not as the sheet expects.
3. **The setup ledger.** `s3m`, the check, then `s3`: `L-seed.xml`. Screenshot the window. Then `s3r`: `C-catalogue.xml Q-rates.xml M-marks.xml`.
   - EXPECT CREATED 1 and every other counter 0, as S3a's ledger create answered.
   - C: one row named `BRIDGE Customer RJ`, with `PARENT` Sundry Debtors and `ISBILLWISEON` No.
   - Q: one row of that name, with one registration row that holds anything: `APPLICABLEFROM` 20260401, `GSTREGISTRATIONTYPE` Regular, `PLACEOFSUPPLY` Rajasthan, and `GSTIN` 08ZZZZZ0000Z1ZQ.
   - Any other answer: record it, send no invoice, and go to step 6. A ledger that does not read so is a failed setup, not a finding about the invoice.
4. **Sub-row a, the registered customer.** `s4m`, the check, then `s4`: `I-0053.xml`. Screenshot the window. Then `s4r`: `R-0053.xml N-0053.xml M-marks.xml`, and `count.py --diff a-s2-R-0010.xml a-s4r-R-0053.xml`.
   - EXPECT CREATED 1 and every other counter 0 except `LASTVCHID`, with no `LINEERROR`.
   - R-0053 holds one row, and N-0053 one row of Sales class.
5. **Sub-row b, the round off on the debit side.** `s5m`, the check, then `s5`: `I-0054.xml`. Screenshot the window. Then `s5r`: `R-0054.xml N-0054.xml R-0010.xml M-marks.xml`, and `count.py --diff a-s2-R-0010.xml a-s5r-R-0054.xml`.
   - EXPECT as in step 4, and R-0010 with the same fields as `s2`'s.
6. **The restore**, if anything was created. C6, and S5's step 6 word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml C-catalogue.xml`, and `count.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes (GUID, ALTVCHID and ALTMSTID equal `s1`'s), and C's rows equal to `s2`'s.
7. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog after a write, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. Also stop at a write whose answer is neither CREATED 1 with ERRORS 0 and EXCEPTIONS 0, nor a decline in the answer. A decline of either invoice is a result: record its counters and every `LINEERROR`, send that sub-row's R and N once, and go on to the next step.

## Record

- Each answer's bytes and sha256, and `count.py`'s lines.
- The two `--diff` outputs whole.
- The BK file, and every prompt word for word with the key that answered it.
- The release on Tally's screen, the Gateway's date, and the screenshot after each write, listed by name, bytes and sha256.

## Expected

- **s2:** none of the names or numbers present, and R-0010 one row: `BRIDGE Walk-in`, Unregistered/Consumer, included Yes and every other status flag No. That is how S4 and S5 read the same template's invoices.
- **s3r:** as in step 3.
- **s4r**, the diff against R-0010:
  - `VOUCHERNUMBER`, `REFERENCE`, `GUID`, `MASTERID`, `ALTERID` and the voucher keys differ, as they did in S4's diff;
  - `PARTYLEDGERNAME` reads `BRIDGE Customer RJ`, `PARTYGSTIN` reads `08ZZZZZ0000Z1ZQ` in place of empty, and `GSTREGISTRATIONTYPE` reads `Regular`;
  - leg 1's ledger is `BRIDGE Customer RJ`, and its sign and amount are unchanged;
  - nothing else differs. `STATENAME` and `PLACEOFSUPPLY` stay Rajasthan, and the four status flags and `ISPOSTDATED` stay as R-0010's.

  The flags of a registered customer's invoice are the row's open question. They are recorded, not predicted beyond this line.
- **s5r**, the diff against R-0010:
  - the same identity fields differ;
  - leg 1's amount reads `-1050.00`;
  - leg 6 reads `BRIDGE Round Off`, with `ISDEEMEDPOSITIVE` Yes and amount `-0.84`;
  - nothing else differs, the flags included. R-0010 equals `s2`'s.
- **s9:** M's COMPANY row equal to `s1`'s (GUID, ALTVCHID 68, ALTMSTID 245), and C's rows equal to `s2`'s.

## Decides

The post reads an invoice back and refuses it unless every field it wrote comes back (`invoice_readback_differences`) and the status flags read included, not overridden and not post-dated (`gst_status_differences`). So each sub-row decides whether that check passes for its element set.
- **a as expected, flags included:** the first element set of item 18 is stored as the build writes it, and its read-back is clean. Recorded PARTIAL: one book, one release, a raw import and not a build.
- **a with the flags uncertain or excluded:** the post would end every bill to a registered customer in a stop. Report it. Item 18 stays owed, and whether the cause is the seeded ledger or the invoice is the next row's question.
- **a with `PARTYGSTIN` or `GSTREGISTRATIONTYPE` not as sent:** the read-back comparison refuses every such post. Report it. The render or the comparison changes before Sales is switched on.
- **b as expected:** the second element set of item 18 is stored as sent, and reads back clean. PARTIAL, as above.
- **b with leg 6 changed or missing,** or the party's amount not 1,050.00: Tally does not store a debit round off as the build writes it. Report it. The build then refuses a debit round off until it is understood.
- **A decline of either:** report its `LINEERROR`. That element set cannot be posted in this form.
- **A dialog anywhere:** the row stops (see Stop).

## Code admits the trigger

Lines are at master a5463e2f.
- **Sales is not posted today.** `LIVE_QUALIFIED_VOUCHER_TYPES` (agent_import.rs:419-424) holds Journal, Payment, Receipt and Contra. So no release sends I-0053 or I-0054, and this row sends them raw.
- **The build admits a registered customer that is not bill-wise.** In `classify_sales_invoice` (agent_import_invoice.rs:825-887), a party whose registration in force names a GSTIN is admitted when:
  - `gstin_valid` passes (:288-319);
  - the GSTIN's state is the place of supply (:850);
  - the entry's type is `Regular` (:857);
  - the catalogue's `ISBILLWISEON` is No (:880-886).

  The GSTIN comes from the dated registration rows through `gstin_evidence` (:1824-1844), fed by the ledger read the build already makes (:2241-2245). The bill-wise flag comes from the catalogue (:2264). The sheet's ledger meets each test: `08` is Rajasthan, the company's state and the template's place of supply.
- **What the build renders for that party.** `observe` (:2618-2647) sets the party's state from the GSTIN and its registration type to `Regular`. `render_sales_invoice_xml` (:2655-2707) writes `PARTYGSTIN` after `CONSIGNEEMAILINGNAME` exactly when a GSTIN is in force (:2693-2697). The party's name goes in the seven places the template carries it. The party is not bill-wise, so no New Ref is written (:2682). That is I-0053's change set.
- **The build admits a round off on the debit side.**
  - The shape check leaves the round-off leg out of the debit count, so a debit round off is not a second party (:661-670).
  - Its ledger must sit under Indirect Expenses or Indirect Incomes, with one empty GST row and zero tax fields (:709-758). `BRIDGE Round Off` is the ledger those rules were measured on, and the template's.
  - A debit lowers what the customer owes (:987-990), and must be under 1.00 (:994). The party's amount must close (:997).
  - The sheet's figures: 1,000.80 taxable, plus 25.02 and 25.02 of tax, less 0.84, is 1,050.00.
  - The render writes a debit leg with `ISDEEMEDPOSITIVE` Yes and a signed negative amount (:2666-2680; `tally_positive`, agent_import.rs:458-465). That is I-0054's change set.
- **The reads.**
  - R is `render_invoice_readback_request` (agent_import_invoice_wire.rs:505), and the post compares its answer with `invoice_readback_differences` (agent_import_invoice.rs:1396; `PARTYGSTIN` and `GSTREGISTRATIONTYPE` at :1437-1444) and `gst_status_differences` (:1649).
  - Q is the whole-list form of `render_ledger_rates` (agent_import_invoice_wire.rs:876), pinned to the file by `the_ledger_rates_request_is_the_answered_one_with_the_hsn_fields` (agent_import_invoice_wire_tests.rs:894). The build sends it restricted to the named ledgers' GUIDs, and #1331 found each ledger's row under that restriction byte-identical to its row under a parent filter (agent_import_invoice_wire.rs:855-862). The whole-list form is sent here because the new ledger's GUID is not known before step 3.
  - N is `render_invoice_number_request` (agent_import_invoice_wire.rs:429), as in S5.

## Not measured

Tally's GSTR-1 view of either invoice (row S14), a customer in another state, a bill-wise registered customer, a ledger keyed on Tally's screen in place of seeded, an invoice built and posted through Bridge (row S12), and any other book or release.

Row: lab/2162-s13/
