**Shape discovery: the one import of this row is authored, not rendered by any build.** It is assembled from two captured requests, the build's own invoice request and an item line from another row's request, as the maintainers allow raw lab requests for discovering a shape (ruling of 26 Oct 2026). Every byte that differs from the build's request is listed under "The request, byte by byte".

# Row 1379-IT: does Tally take the build's invoice request with its two service legs replaced by one amount-only stock-item line, and what do the build's read-back and a native read return for it?

Not for merge. For #1379 (a GST Sales item invoice), its first "Measure live first" bullet: an amount-only line on a book with one godown and no batches. Written 10 Nov 2026. One stock item keyed on Tally's screen, one import, then a restore from the row's own backup. Confidence of everything here until the row runs: unverified.

## Question
#1379's first slice posts amount-only item lines on the #1342 invoice core. Nothing has posted an item line through the XML gateway to a book with no batches, or read one back through the build's read-back.

- **What exists, and why it does not answer this:**
  - Row 2016 (`lab/2016-stock-lines-answers` at 186c39d6) reads stock lines on BRIDGE SHAPE LAB, a batch-wise book with two godowns. Its item Sales (`reqs/v-sales.xml`) carries a quantity, a rate and a batch, is exempt, and is a seed its RESULTS.md calls set-up only, never evidence of its own shape. No read-back of the build's ran on it.
  - Section 9.12 of the protocol reference holds an amount-only service line, imported through Tally's own screen with no gateway response.
- **What this row adds:** the gateway's counters for a GST Sales invoice whose sales line is an amount-only stock-item line, on a book with no batches; and what the build's read-back (`render_invoice_readback_request`) and a native read return for that invoice: where Tally keeps the line, and whether the line's ledger and amount reach the read-back's legs.
- **What the shape is meant to become:** the item-line form of #1379's builder, as a change to `render_sales_invoice_xml`. Tally's answers here inform that design. They are never a regression fixture for the build's output (Decides).
- **Not asked here:** quantity and rate, a batch or a second godown, an item whose GST details differ from the sales ledger's, an item not in the book, the tax rounding rule, more than one item line, GSTR-1.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2346-CD's and 2464-GR's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count18.py` holds BRIDGE PILOT LAB's GUID as `BOOK`.
  - Its M kind prints whether the COMPANY row's GUID is the book's, and its S kind whether the item's `BRIDGECOMPANYGUID` is.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is keyed or sent unless s1's COMPANY row is the book's.
- **Its state,** from the lab's last notes on it: rows queued before this one each end with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245.
- **Inventory is on and batches are off:** the company object fixture on master (`pilot-lab-company-object`) reads `ISINVENTORYON` Yes and `ISBATCHWISEON` No. No captured answer shows whether the book holds a stock item or how many godowns it has; s2 reads both.
- **The number `BP/26-27/0093` is unused.** No file on any of the 650 `lab/` branches or on master holds it, answers with cancelled vouchers included (every file decoded, UTF-16 with or without a BOM, 10 Nov 2026). `0089` is used by row 2360's sheet (`lab/2360-declined-sheet`) and `0091` by row 2464-GR's. s2's V read is the STOP if this census is wrong.
- **No file anywhere holds the REMOTEID tail `000000000e01` or the name `BRIDGE Item A`** (the same census).
- **Its ledgers:** `BRIDGE Walk-in` is under Sundry Debtors and not bill-wise (`pilot-lab-ledger-catalogue-v2-10oct.utf16le.xml`). `BRIDGE Svc 998313 5%`, the two tax ledgers and `BRIDGE Round Off` are `BP/26-27/0010`'s, as the invoice has them.
- **BK is this row's way back.** It is taken at step 1, before anything is keyed or sent, and restored at step 5 by C6. **The restore runs in every case once BK exists, including a stop at any step:** this sheet is the run coordinator's word for it (C8). After it, every read is compared with the reads before; any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Rows 2162-RU, 2162-SC, 2162-GY and 2467-GS read TallyPrime with tally.exe 1.1.7.2.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5), the stock item (step 3) and the restore (C6). The one write is `I-0093.xml`, sent once.

**The sends** are row 2162-RP's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count18.py`, which prints STOP and exits non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count18.py` and copy its whole output into RESULTS.md.
- Close every other Bridge process first.

## The request, byte by byte
`I-0093.xml` is `template-import-0010.utf16le.xml` (the build's request that created `BP/26-27/0010`, 4,754 bytes) with the four changes below and nothing else; `gen.py` asserts each. Element numbers count the decoded text split before every `<` that follows a `>`, from 1. In `template-v-sales-2016.utf16le.xml` (row 2016's `reqs/v-sales.xml`, byte for byte) they are:
`16` `<VOUCHER …>`; `17`–`23` the voucher head (`DATE`, `EFFECTIVEDATE`, `VOUCHERTYPENAME`, `PERSISTEDVIEW`, `ISINVOICE`, `PARTYLEDGERNAME`, `NARRATION`); `24`–`29` the party leg; `30` `<ALLINVENTORYENTRIES.LIST>`; `31` `STOCKITEMNAME`; `32` `ISDEEMEDPOSITIVE` No; `33` `RATE`; `34` `AMOUNT`; `35` `ACTUALQTY`; `36` `BILLEDQTY`; `37`–`44` `BATCHALLOCATIONS.LIST`; `45`–`49` `ACCOUNTINGALLOCATIONS.LIST` (`LEDGERNAME`, `ISDEEMEDPOSITIVE` No, `AMOUNT`); `50` `</ALLINVENTORYENTRIES.LIST>`.

| # | Template element | In `I-0093.xml` | Where it comes from |
| --- | --- | --- | --- |
| 1 | 16, `REMOTEID` tail `000000000777` | tail `000000000e01` | This row's own tail, unused (Book). |
| 2 | 20 `REFERENCE` and 22 `VOUCHERNUMBER`, `BP/26-27/0010` | `BP/26-27/0093` in both | The build writes the number in both (`src-tauri/src/agent_import_invoice.rs:2708`); the number is unused (Book). |
| 3 | 37 `<VCHENTRYMODE>Accounting Invoice</VCHENTRYMODE>` | absent | `v-sales.xml`'s voucher head, elements 16 to 23, has no `VCHENTRYMODE`. |
| 4 | 46 to 55, the legs `BRIDGE Svc 998313 5%` No `600.40` and `BRIDGE Svc 998314 5%` No `400.40` | 45 to 54, one item line in their place after the party leg | `v-sales.xml` elements 30, 31, 32, 34 and 45 to 50, in that order. RATE (33), ACTUALQTY (35), BILLEDQTY (36) and BATCHALLOCATIONS.LIST (37 to 44) are left out: the line is amount-only and the book has no batches. |

The values in change 4: the item `BRIDGE Item A` (keyed at step 3) in element 31; the side No in 32 and in the allocation's `ISDEEMEDPOSITIVE`, which are both the 2016 line's and the two replaced legs'; the ledger `BRIDGE Svc 998313 5%`, the first replaced leg's; and `1000.80`, the two legs' sum, in 34 and in the allocation's `AMOUNT`.

So the invoice has one sales ledger, as #1379's scope requires. The party leg (`BRIDGE Walk-in` Yes `-1051.00`), the CGST and SGST legs (No `25.02` each, 2.5% of `1000.80`) and `BRIDGE Round Off` (No `0.16`) are the template's bytes, and the legs and the line sum to 0.00. `gen.py` asserts that the request names no `VCHENTRYMODE`, `RATE`, `ACTUALQTY`, `BILLEDQTY`, batch, godown, bill allocation, `BRIDGE Svc 998314 5%` or `BP/26-27/0010`. It is 4,718 bytes.

## Files
- **Byte-equal to row 2346-CD's file of the same name** (`lab/2346-cd-sheet`, folder `lab/2346-cd/`), which in turn names row 2464-GR's and 2162-RP's: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml` (the build's read-back for `BP/26-27/0010`), `W-2026-08.xml` (row 2162-KC's number read over August 2026), `template-import-0010.utf16le.xml` (sha256 `b333f0dc`, equal to `PILOT_REQUEST_SHA256` on master) and `template-full-0057.utf16le.xml` (row 2162-RS's native read by number).
- **`template-v-sales-2016.utf16le.xml`:** `lab/2016-stock-lines-answers` at 186c39d6, `reqs/v-sales.xml`, byte for byte (sha256 `c749d38a`, as that branch's SHA256.txt lists it). It is never sent.
- **`template-stock-items.utf16le.xml` and `template-godowns.utf16le.xml`:** master's `masters_stock_items_request.utf16le.xml` (sha256 `c90cfcc2`) and `masters_godowns_request.utf16le.xml` (sha256 `a2a63164`), byte for byte. They are the build's stock-item and godown master reads on BRIDGE SHAPE LAB (`native_masters_tests.rs:61` and `:70` compare the build's rendering with them).
- **`gen.py`** writes, each change asserted, each template's sha256 checked first:
  - `I-0093.xml`, as above;
  - `V-0093.xml`: the native read for the number `BP/26-27/0093`;
  - `R-0093.xml`: the build's read-back for `BP/26-27/0093` on `BRIDGE Sales`;
  - `S-items.xml` and `G-godowns.xml`: the two master reads with the company `BRIDGE PILOT LAB` in place of `BRIDGE SHAPE LAB`, which is what the build renders for this book.
- **`count18.py`:** row 2346-CD's `count17.py`, with four changes, marked IT.
  - The V and R kinds also print the row's item lines: each `ALLINVENTORYENTRIES.LIST` and `INVENTORYENTRIES.LIST`, and each `INVENTORYALLOCATIONS.LIST` a leg holds, with the item, side, rate, amount, quantities, how many batch allocations, and each accounting allocation.
  - A kind S: the stock-item rows, those named `BRIDGE Item A` (or `--item`), and each one's parent, GUID, MASTERID, ALTERID, company pin and every HSN and GST leaf (at most 200 lines).
  - A kind G: every GODOWN row, with its name, parent and GUID.
  - count17's V kind named a local list `legs`, the name of the function the R kind calls, so every R answer failed. The list is renamed here, and in row 2346-CD's own sheet by a forward commit.
- **What `count18.py` was tried on:**
  - A captured read-back answer (`lab/2162-bc-answers`, `answers/a-s2-R-0010.xml`): one row, six legs.
  - `0010`'s voucher cut from the `pilot-lab-vouchers-after-post` fixture: six legs in each list, and one `ALLINVENTORYENTRIES.LIST` with no item.
  - Master's `stock_movement_credit_note_vouchers_shape_lab_live.utf16le.xml`: no `ALLINVENTORYENTRIES.LIST`, and one item line held under the `Sales - Exempt` leg as `INVENTORYALLOCATIONS.LIST` (`Hydrochloric Acid 33pc`, `30.00/Kgs`, `1.000 Kgs`, one batch allocation).
  - `masters_stock_items_shape_lab_item_created_live.utf16le.xml`, with `--item`: one named row and its HSN and GST leaves.
  - `masters_godowns_shape_lab_live.utf16le.xml`: two godowns.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count18.py a-<label>-*`. A STOP from either script stops the row there, and step 5 still runs once BK exists.

**The prompt rule, at every step.** The only prompts answered are C5's, C6's and step 3's "Accept ?". For any other prompt, message or dialog, copy it word for word and screenshot it first. Then answer No, or Esc if it offers no No. Press nothing more and send nothing. Then restore BK by C6, compare as step 5, and close (C10). That prompt is this row's result.

**The wait rule.** After the send at step 4, and after step 3's accept: if 5 minutes pass with no answer and no message, send nothing, press nothing, screenshot the whole window, and report to the run coordinator. A read sent while Tally is busy can hold its gateway. With the Gateway showing and no dialog, run C2's status check (`GET /status` 200, one tally.exe owning 9001) before the next read.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to the new empty folder `C:\lab\1379-it\backup-1`. Row 2162-GY's step 1 is the path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `W-2026-08.xml V-0093.xml R-0093.xml R-0010.xml S-items.xml G-godowns.xml M-marks.xml`.
   - EXPECT V-0093 0 rows. **Any row: stop before step 3.** The number is then held, and the census was wrong.
   - EXPECT W 40 rows, as row 2162-KC's s2 read (`lab/2162-kc-answers` at e3c2bd1f), none numbered `BP/26-27/0093`.
   - EXPECT R-0093 0 rows, and R-0010 one row: `BRIDGE Walk-in`, included Yes, every other status flag No, 6 legs.
   - S: record how many STOCKITEM rows. EXPECT none named `BRIDGE Item A`. **One named so: stop before step 3.**
   - G: record every GODOWN row. **More than one godown: stop before step 3.** This row is for a book with one.
3. **Key the stock item**, on Tally's screen, by the path row 1381-M keyed one on 7.2 (`lab/1381-stock-item-create-answers` at d8948467, `notes.txt`, step 1d and R1b). Step 0 of C2 first.
   - Gateway > Create > Stock Item. Read each row before Enter.
   - Screenshot the Stock Item Creation screen as it opens. Write down every field and its value, word for word. Row 1381-M's opened with: Under Primary, Units Not Applicable, GST applicability Applicable, HSN/SAC Details and GST Rate Details each As per Company/Stock Group, Type of Supply Goods, Rate of Duty empty.
   - Type the name `BRIDGE Item A`. Keep Under `Primary` and Units `Not Applicable` as offered.
   - Set Type of Supply to `Services`, read on the screen before Enter. This is so the item does not disagree with `BRIDGE Svc 998313 5%`, a ledger for SAC 998313, a services code. An item that disagrees is #1379's fourth bullet, not this row.
   - Leave every other field as offered. An opening quantity, rate or value stays empty. A field this list does not name: keep it as offered and write it down.
   - Screenshot the whole screen before the accept. Then accept. "Accept ?": y.
   - Tally shows a blank Stock Item Creation. One Ctrl+Q, after reading the title (C7): back to the masters menu. Escape (a menu, not a master screen): the Gateway. Screenshot it: no dialog.
   - Then the wait rule. `s3r`: `S-items.xml G-godowns.xml M-marks.xml`.
     - EXPECT S one row named `BRIDGE Item A`, its company the book's. Record every line count18.py prints for it: PARENT, GUID, MASTERID, ALTERID and each HSN and GST leaf.
     - G: EXPECT the same rows as s2's.
     - Record ALTMSTID against s1's.
     - **No row named `BRIDGE Item A`, or more than one: stop.** Send nothing, and go to step 5.
   - **RESULTS records the item's master fields:** every field from the two screenshots, and s3r's S lines.
4. **The import.** `s4m`: `M-marks.xml`, then `count18.py --before-write a-s1-M-marks.xml a-s4m-M-marks.xml`. It must pass. Then no key is pressed in Tally before the send.
   - `s4`: `I-0093.xml`, sent once. Then the wait rule. Record every counter and any LINEERROR, word for word. The window after it (shot): the Gateway, no dialog. A dialog: the prompt rule.
   - `s4r`: `W-2026-08.xml V-0093.xml R-0093.xml R-0010.xml S-items.xml M-marks.xml`.
     - `W`: record every row s2's W does not hold, whole: its number, type, date, `ISCANCELLED`, the Sales class and its GUID.
     - `V-0093`: record each row whole: every field count18.py prints, both leg lists, and every item line in each of its three forms.
     - `R-0093`: record its rows whole: the header fields, every leg, and every item line, in whichever form it returns.
     - `count18.py --diff a-s2-R-0010.xml a-s4r-R-0010.xml`: record the whole output. It says whether the invoice moved.
     - S: record the item's lines again (an import may alter a master it names). Record ALTVCHID and ALTMSTID against s4m's.
   - Any answer at s4 is a result, created or not. Record it and go on to step 5. Never send `I-0093.xml` again (C3).
5. **Restore** BK by C6, in every case once BK exists, with `restore-plan.txt` written first. Row 2162-GY's step 5 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace). Then `s5`: `M-marks.xml W-2026-08.xml V-0093.xml R-0093.xml R-0010.xml S-items.xml G-godowns.xml`, and compare:
   - `count18.py --same-row a-s1-M-marks.xml a-s5-M-marks.xml`: EXPECT it passes.
   - W 40 rows, V-0093 0 rows and R-0093 0 rows, each answer's sha256 equal to s2's.
   - `count18.py --diff a-s2-R-0010.xml a-s5-R-0010.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`.
   - S: as many STOCKITEM rows as s2's, none named `BRIDGE Item A`. G: the same rows as s2's.
     - Record each sha256, but equal bytes are not required for S and G. Row 1381-M's master reads came back from a restore with equal rows and different bytes.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
6. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those C5, C6 and step 3 name, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. Then step 5, as the Book says. Say in the report what the book held before the restore: a stop after step 3 leaves the item, and one after step 4 whatever the import did.

## Record
- Each answer's bytes and sha256, and `count18.py`'s lines.
- The `--diff`, `--before-write` and `--same-row` outputs, whole.
- The screenshots at step 3 and after the send, listed by name, bytes and sha256.
- As text and word for word: every field of the Stock Item Creation screen as opened and as accepted; every prompt, message and busy screen, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2:** as the Book and step 2 say. One godown, `Main Location`, as Tally creates for a book with inventory on.
- **Step 3:** one item created, with its HSN and GST lists deferring (As per Company/Stock Group). Row 1381-M's keyed item read so.
- **Step 4:** `CREATED` 1, under `BP/26-27/0093`.
- **V:** the line stored either as an `ALLINVENTORYENTRIES.LIST` or under the `BRIDGE Svc 998313 5%` leg as an `INVENTORYALLOCATIONS.LIST`. A captured read of an item Credit Note on BRIDGE SHAPE LAB holds it the second way (Files).
- **R:** five legs, with `BRIDGE Svc 998313 5%` No `1000.80` among them, if Tally keeps the line as that leg's allocation.
- **R-0010:** unchanged at s4r.
- **s5:** equal to s2 and s1.

## Decides
Each is PARTIAL: one import, one sitting, one book, one release.

**What this row's answers are for.** Tally's answers here may inform #1379's design: the line's element, the read-back's fetch, and what Tally fills. **They are never a regression fixture for the build's output**, because the request is authored, not rendered (the first line of this sheet). No test may assert that a build renders `I-0093.xml`'s bytes, or replay its answers as the answer to a build's request. A later row does that: **row 1379-IT2**, the build's own rendered item request, sent once the #1379 builder exists, on this book, with the same reads.

- **Created, and R returns the line's ledger and amount among its legs:**
  - the build's read-back can verify an amount-only line with its present fetch;
  - #1379's builder may render the line in this row's form (change 4) and let the verify step compare the leg.
  - Record whether Tally filled `VCHENTRYMODE`, and with what, and the three GST status flags.
- **Created, but R's legs lack the line's amount:** the read-back must fetch the item list it is stored in (V says which) before #1379 can verify a line. That fetch is a new shape: a candidate builder, its own row.
- **Refused** (`CREATED` 0, an error or a LINEERROR): record the words. One change this row did not make may be needed, each its own row, one at a time. Examples: `VCHENTRYMODE` `Item Invoice`, a unit on the item, `RATE` and a quantity, or a godown allocation.
- **Stored other than sent** (a side, an amount, a ledger, a leg or line added or dropped): record it whole. The builder must not judge an item invoice by the request's lines alone.
- **The item master moved at s4r:** an import naming an item altered it. Record the leaves, and report before the restore.
- **What Tally filled for the line's GST** (V's status flags, and any HSN or rate it shows on the line): record as found. It is acceptance 3's question, read here only for an item that defers to the company or group while its sales ledger names SAC 998313.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build has no item line.** `render_sales_invoice_xml` (`src-tauri/src/agent_import_invoice.rs:2664`) writes every line as a `LEDGERENTRIES.LIST` (`:2686`). No line of that file names an inventory list or a stock item. No build sends `I-0093.xml`: it is a raw edit, sent to measure the shape #1379 would build.
- **The read-back fetches legs only.** `render_invoice_readback_request` (`src-tauri/src/agent_import_invoice_wire.rs:505`, the filter at `:514`) fetches `ALLLEDGERENTRIES.*` and names no inventory list. `R-0093.xml` is that request for this row's number.
- **The master reads are the build's.** `S-items.xml` and `G-godowns.xml` are the bytes `NativeMasterKind::StockItems` and `NativeMasterKind::Godowns` render (`src-tauri/crates/bridge-tally-protocol/src/native_masters_tests.rs:70` and `:61`), with this book's name.

## Not measured
- Quantity and rate, a unit, a batch, a second godown.
- An item whose GST details differ from the sales ledger's, or that names its own HSN, SAC or rate.
- An item not in the book.
- More than one item line, or an item line beside a sales ledger line.
- IGST, cess, the tax rounding rule.
- GSTR-1.
- An item invoice built and posted through Bridge (row 1379-IT2).
- Any other book or release.

Row: lab/1379-it/
