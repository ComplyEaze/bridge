# Row 2162-KD: on a Sales voucher type keyed on screen today with Manual numbering and Prevent duplicates Yes, what does Tally store for a fresh invoice number, and does it decline the same number sent again in its answer?

Not for merge. Part of #2162 (ADR 0004, checklist item 12: the Sales type's own prevent-duplicates flag). Written 9 Nov 2026. Lines below are at master ae38a75c, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
The build admits a Sales type only when its one numbering series reads Manual with `PREVENTDUPLICATES` Yes (`resolve_voucher_type`, `agent_import_invoice_wire.rs:383-391`). Behind the build's own number read, that flag is what stops a number from being used twice when no invoice stands as sent.

Row 2162-S4 (3 Nov 2026) measured the flag once, on `BRIDGE Sales`: a type keyed for the rehearsal, weeks before that row. It declined a number already held in its answer (`CREATED` 0, `EXCEPTIONS` 1, no `LINEERROR`) with no dialog. Confidence PARTIAL: one type, one sitting.

**This row changes one variable: the type.** A second Sales type, `BRIDGE Sales Keyed`, is keyed on screen in this sitting with the same two settings, and the same invoice is sent to it twice. The row reads:
- the type as the build reads it: does the screen's "Prevent duplicates" land in the series' `PREVENTDUPLICATES`, which the build checks?
- for the fresh number: every native field of what Tally stored, above all `VOUCHERNUMBER` and `VOUCHERTYPENAME`. Tally kept the number sent, or replaced it with one of its own;
- for the same number sent again: the counters, every `LINEERROR`, and whether a dialog appears (P7).

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2162-BC's) applies in full. Where it and this sheet differ, the stricter one holds.
- Its state, from the lab's last notes on it: each row since S13a2 restored it to ALTVCHID 68 and ALTMSTID 245, or only read it. So neither `BP/26-27/0061` nor a type named `BRIDGE Sales Keyed` is expected in it.
- One BK before the first write and one restore after the last (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID equal step 1's (the ruling of 4 Nov 2026 on unchanged marks).
- Write down the release shown on Tally's screen. S4 and S13a2 read TallyPrime 7.2 Silver.

**No Bridge build.** Raw requests, one at a time, through `run.py` (row 2162-BC's, byte-equal). The build cannot send this invoice, because Sales is not a qualified voucher type.

**The sends** are row 2162-BC's, unchanged. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count6.py`, which prints STOP and exits non-zero).

## Files
Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.
- **Byte-equal to row 2162-BC's file of the same name** (`lab/2162-bc-sheet`, folder `lab/2162-bc/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `template-import-0010.utf16le.xml`, `template-n-0001.utf16le.xml` and `template-full-1.utf16le.xml`. Row 2162-BC's ROW.md says where each first came from.
- **`T-types.xml`:** byte-equal to `r08-voucher-types.xml` (`lab/1342-capture-requests`). It is `render_voucher_types_request` for `BRIDGE PILOT LAB` (`agent_import_invoice_wire.rs:267`), the read the build makes before it admits a type. A test pins its sha256, `c351d6c6…` (`agent_import_post_e2e_tests.rs:7509`). Its committed answer, `pilot-lab-voucher-types.utf16le.xml`, shows `BRIDGE Sales` with `NUMBERINGMETHOD` None on top and one series, Default, Manual, `PREVENTDUPLICATES` Yes.
- **`gen.py`** writes, from those files, and asserts every change:
  - `I-0061.xml`: the build's request that created `BP/26-27/0010`, with the number `BP/26-27/0061` (in `VOUCHERNUMBER` and `REFERENCE`), the type `BRIDGE Sales Keyed` (in `VCHTYPE` and `VOUCHERTYPENAME`) and the REMOTEID tail `0792`. Party, figures and date unchanged;
  - `I-0061b.xml`: `I-0061.xml` with the REMOTEID tail `0793`, nothing else. The two files differ in one byte;
  - `N-0061.xml`: the number read with the number `BP/26-27/0061`;
  - `V-0061.xml`: every native field of the vouchers numbered `BP/26-27/0061` in 2026-27, built as row 2162-BC's V files.
- **`count6.py`:** row 2162-BC's `count5.py` with one kind added, T: how many `VOUCHERTYPE` rows, how many carry each of the names `BRIDGE Sales` and `BRIDGE Sales Keyed`, and for each such row its `PARENT`, `GUID`, `NUMBERINGMETHOD` and `ISACTIVE`, then every `VOUCHERNUMBERSERIES.LIST` with its `NAME`, `NUMBERINGMETHOD` and `PREVENTDUPLICATES`.

  When the sheet was written, on the committed answer `pilot-lab-voucher-types.utf16le.xml` (264,678 bytes) as a T answer, it printed 25 rows; `BRIDGE Sales` 1, with `NUMBERINGMETHOD` 'None' and one series 'Default', 'Manual', 'Yes'; `BRIDGE Sales Keyed` 0.

**Checked when the sheet was written:** `I-0061.xml` names `BRIDGE Sales Keyed` twice and holds no other voucher type name; `BP/26-27/0061` twice; one REMOTEID. `I-0061b.xml` equals it but for the REMOTEID's last digit.

## Steps
Run each step as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count6.py a-<label>-*`. A STOP from either script stops the row there.

**Before each write sent as a request** (steps 4 and 5): `run.py URL s<n>m M-marks.xml`, then `python3 -I count6.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`. Only if that passes, and no key was pressed in Tally since, send the write alone: `run.py URL s<n> <write>`.
1. **Load and check.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date. If it is before 1-Aug-2026, stop: the invoice is dated 1 Aug 2026, and a post-dated invoice is another row's variable.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row with the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to a new empty folder. Record the file's name, size and sha256.
2. **The book as found.** `s2`: `T-types.xml N-0061.xml V-0061.xml`.
   - EXPECT in T `BRIDGE Sales` 1 row (Default, Manual, Yes) and `BRIDGE Sales Keyed` 0; 0 rows in N and in V.
   - Anything else: record it and stop before any write.
3. **Key the voucher type on screen.** This is the row's one master write. `s3m`, the check, as before a request.
   - From the Gateway: Create, then Voucher Type. Write down the menu path and keys used.
   - Set only these four fields, and leave every other field as Tally fills it:
     - Name: `BRIDGE Sales Keyed`;
     - Select type of voucher: `Sales`;
     - Method of voucher numbering: `Manual`;
     - Prevent duplicates: `Yes`. If Tally does not offer this field once Manual is chosen, stop before saving and ask.
   - Screenshot every part of the screen before saving, and write down every field it shows with its value, as text.
   - Save with Ctrl+A. If Tally asks "Accept ?", answer Yes; that is the master's own accept (C7). Any other prompt: stop, press nothing, and ask.
   - If Tally then opens a new blank Voucher Type Creation screen, screenshot its title and leave it with Ctrl+Q (C7). Screenshot the Gateway.
   - `s3r`: `T-types.xml M-marks.xml`.
   - EXPECT in T one row named `BRIDGE Sales Keyed`, `PARENT` 'Sales', with one series, Manual, `PREVENTDUPLICATES` Yes; `BRIDGE Sales` as in `s2`. In M ALTVCHID as in `s1` and ALTMSTID above it.
   - Any other answer, above all a series that does not read Manual and Yes: record it, send no invoice, and go to step 6. That is a result: the screen's setting is not the flag the build reads.
4. **The fresh number.** `s4m`, the check, then `s4`: `I-0061.xml`. Screenshot the window. Then `s4r`: `N-0061.xml V-0061.xml M-marks.xml`.
   - EXPECT CREATED 1 and every other counter 0 except `LASTVCHID`, with no `LINEERROR`; N one row, `BP/26-27/0061`, `BRIDGEVCHISSALES` Yes; V one row with `VOUCHERNUMBER` `BP/26-27/0061` and `VOUCHERTYPENAME` `BRIDGE Sales Keyed`.
   - A V row with any other number or type is a result: record both, and still go on to step 5. Tally then stored something other than what was sent.
   - A decline is a result: record its counters and every `LINEERROR`, send N and V once, and go to step 6.
5. **The same number sent again.** `s5m`, the check, then `s5`: `I-0061b.xml`. Screenshot the window. Then `s5r`: `N-0061.xml V-0061.xml M-marks.xml`.
   - EXPECT, as S4's b1 read on `BRIDGE Sales`: CREATED 0, EXCEPTIONS 1, every other counter 0, no `LINEERROR` text; N and V one row each, the voucher of step 4; M equal to `s4r`'s COMPANY row (`count6.py --same-row a-s4r-M-marks.xml a-s5r-M-marks.xml`).
   - CREATED 1 is a result: the type keyed today did not decline the number. Record V's rows, both of them, and go to step 6.
6. **The restore**, if anything was created. C6, and row 2162-BC's step 6 word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `s9`: `M-marks.xml T-types.xml`, and `count6.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`. EXPECT it passes, and in T `BRIDGE Sales Keyed` 0.
7. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog after a write, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. A dialog after step 5 would make the guard a failure inside Tally's screen rather than one in its answer (P7), and that decides the design before any count does. Also stop at a write whose answer is neither CREATED 1 with ERRORS 0 and EXCEPTIONS 0, nor a decline in the answer.

## Record
- Each answer's bytes and sha256, and `count6.py`'s lines.
- Step 3's screens: every field and value as text, the screenshots by name, bytes and sha256, and every prompt word for word with the key that answered it.
- The screenshot after each write, by name, bytes and sha256.
- The BK file, and every prompt of the restore word for word with the key that answered it.
- The release on Tally's screen and the Gateway's date.

## Expected
Estimates from earlier rows, not predictions.
- **s3r:** `BRIDGE Sales Keyed` reads as `BRIDGE Sales` does: `NUMBERINGMETHOD` None on top, one series Default, Manual, Yes (the committed answer; §9.16 says the rehearsal's keyed type read so).
- **s4:** created, number and type as sent (S4's seed, `BP/26-27/0051`, was stored as sent on `BRIDGE Sales`).
- **s5:** declined in the answer, with no dialog (S4's b1).

## Decides
- **s3r reads Manual and Yes, s4 stores the number sent, s5 declines in the answer:** the flag holds on a second Sales type, keyed in this sitting, in-band. With S4, item 12's first sub-item rests on two types and two sittings of one book and one release: PARTIAL still, a stronger PARTIAL.
- **s3r reads anything else:** keying "Prevent duplicates" on screen does not produce the flag the build reads, or produces it in another place. The build's check then needs its own row before Sales is switched on. Report what T shows.
- **s4 stores another number or type:** a Manual type does not keep the number sent. The switch-on then needs the read-back to compare the stored number before any post is called verified. Report both numbers.
- **s5 creates a voucher:** the flag did not hold on this type. Sales may not be switched on with the flag as its guard. Report V's two rows.
- **A dialog after any write:** the row stops (see Stop).

## Code admits the trigger
Lines are at master ae38a75c.
- **Sales is not posted today.** `LIVE_QUALIFIED_VOUCHER_TYPES` (`agent_import.rs:421`) holds Journal, Payment, Receipt and Contra. So no release sends this invoice, and this row sends it raw.
- **The flag the build reads.** `resolve_voucher_type` (`agent_import_invoice_wire.rs:329`) refuses a Sales type unless its one series reads Manual with `PREVENTDUPLICATES` Yes (`:383-391`). The build calls it before any number read (`agent_import_invoice.rs:2280`). T is its request, byte-equal.
- **What the build sends.** `I-0061` is the build's request for `0010` with the number, type and REMOTEID changed. The same request with another type name is what the build would send to a type named so.
- **The reads.** N is the number read with the e-invoice words. V is a lab read of every native field, the kind row 2162-IRN sent as its F read; no build sends it.

## Not measured
A number held by another Sales type sent to this one; a number that differs by case or a leading zero (S4 measured both on `BRIDGE Sales`); a type with more than one series; a book with no invoice; an invoice built and posted through Bridge (row S12); and any other book or release.

Row: lab/2162-keyed-dup/
