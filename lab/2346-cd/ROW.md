# Row 2346-CD: does Tally take the build's own invoice request as a sales-side Credit Note and a sales-side Debit Note, with only the type and the sides changed, and what does the build's read-back see?

Not for merge. For #2346 (0.9.0: sales-side Credit Notes and Debit Notes through `build_import_xml` and `post_import`), capture 5 of the #691 design of 26 Sep 2026: an accounting-mode Credit Note and a sales-side Debit Note, posted and read back through the verification request. Written 10 Nov 2026. Two imports, each on its own, then a restore from the row's own backup. No master is made or changed. Confidence of everything here until the row runs: unverified.

## Question
#2346 plans to build the notes on the Sales invoice core: the same request, with the type resolved by its class and the sides following from a declared reading. A Credit Note puts the party Cr and every other leg Dr; a sales-side Debit Note puts the party Dr and the rest Cr. Nothing has posted such a request to this book, or read one back through the build's read-back.

- **The requests.** Each is the build's own request that created `BP/26-27/0010` (`render_sales_invoice_xml`), changed only as `gen.py` says:
  - the type `Credit Note` or `Debit Note`, in `VCHTYPE` and `VOUCHERTYPENAME`;
  - no `VOUCHERNUMBER`, because both types number automatically on this book (row 2162-RU's T answer: `NUMBERINGMETHOD` Default, series Automatic);
  - a new REMOTEID tail;
  - for the Credit Note only, every leg's side turned over (`ISDEEMEDPOSITIVE` and the sign of `AMOUNT`).
  - `REFERENCE` and `REFERENCEDATE` stay `BP/26-27/0010` and `20260801`, where a note's original invoice is kept (§8.2k). The legs are the invoice's six, so each note is for the whole invoice.
- **What each shape is meant to become:** the note builder of #2346, as the Sales invoice core with a declared reading. The reads `R-CN1.xml` and `R-DN1.xml` are the build's read-back (`render_invoice_readback_request`) with the note type's GUID in place of `BRIDGE Sales`'s. That is what a note's verification would send if it reuses the Sales read-back.
- **What is read:** the marks (M), every voucher of August with its type and number (W), the native read of the vouchers numbered `1` (V), and the build's read-back (R) for the note and for `BP/26-27/0010`.
- **Not asked here:** an `Agst Ref` or any bill allocation (`BRIDGE Walk-in` is not bill-wise), `GSTNATUREOFRETURN` supplied, a part or tax-only note, IGST, invoice mode with items, a purchase-side note, a numbered note, and GSTR-1.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-RP's and 2464-GR's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count17.py` holds BRIDGE PILOT LAB's GUID as `BOOK`.
  - Its M kind prints whether the COMPANY row's GUID is the book's.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is sent unless s1's COMPANY row is the book's.
- **Its state,** from the lab's last notes on it: rows queued before this one each end with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245.
- **It holds no note.** Row 2162-S5's year read (`lab/2162-s5-answers` at ab3d2e77, `answers/a-s2-Y-all.xml`) returns every voucher of 1 Apr 2026 to 31 Mar 2027, of every type: 41 rows, none a Credit Note or a Debit Note. Its vouchers numbered `1` are one Receipt (20260803) and one Purchase (20260804). Rows 1342-W23 and 1342-W25 keyed a Credit Note and a Debit Note on this book by hand, and restored them away.
- **Its ledgers:** `BRIDGE Walk-in` is under Sundry Debtors and not bill-wise (`ISBILLWISEON` No, `pilot-lab-ledger-catalogue-v2-10oct.utf16le.xml`). The five other ledgers are `BP/26-27/0010`'s, as the invoice has them.
- **BK is this row's way back.** It is taken at step 1, before anything is sent, and restored at step 5 by C6. The restore runs after step 3 in every case, including a stop. After it, every read is compared whole with the reads before; any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Rows 2162-RU, 2162-SC, 2162-GY and 2467-GS read TallyPrime with tally.exe 1.1.7.2.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6). The two writes are `I-CN.xml` and `I-DN.xml`, each sent once.

**The sends** are row 2162-RP's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count17.py`, which prints STOP and exits non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count17.py` and copy its whole output into RESULTS.md.
- Close every other Bridge process first.

## Files
- **Byte-equal to row 2464-GR's file of the same name** (`lab/2464-gr-sheet`, folder `lab/2464-gr/`), which in turn names row 2162-RP's and 2162-RS's: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml` (the build's read-back for `BP/26-27/0010`), `template-import-0010.utf16le.xml` (the build's request that created `BP/26-27/0010`, sha256 `b333f0dc`, equal to `PILOT_REQUEST_SHA256` on master) and `template-full-0057.utf16le.xml` (row 2162-RS's native read by number).
- **`W-2026-08.xml`:** row 2162-KC's file of that name (`lab/2162-kc-sheet` at 604bd4e0), byte for byte: the build's number read over August 2026, every voucher of every type with its date, number, type, `ISCANCELLED` and the Sales class.
- **`gen.py`** writes, each with only the change it names asserted, and each template's sha256 checked first:
  - `I-CN.xml`: the template import as a Credit Note, with the REMOTEID tail `0d01` and each leg turned over: `BRIDGE Walk-in` No `1051.00`; `BRIDGE Svc 998313 5%` Yes `-600.40`; `BRIDGE Svc 998314 5%` Yes `-400.40`; `BRIDGE CGST 2.5%` and `BRIDGE SGST 2.5%` Yes `-25.02` each; `BRIDGE Round Off` Yes `-0.16`. The legs sum to 0.00.
  - `I-DN.xml`: the template import as a Debit Note, with the REMOTEID tail `0d02` and the invoice's legs unchanged: `BRIDGE Walk-in` Yes `-1051.00`, the rest No.
  - `gen.py` asserts that neither names `VOUCHERNUMBER`, `GSTREGISTRATION`, `CMPGSTIN`, `GSTNATUREOFRETURN` or `BILLALLOCATIONS`.
  - `V-1.xml`: the native read for the number `1`.
  - `R-CN1.xml`, `R-DN1.xml`: the build's read-back for the number `1` on the type `Credit Note` (`...-00000028`) and on `Debit Note` (`...-0000002a`), the GUIDs row 2162-RU's T answer gives.
- **No file on any `lab/` branch, and nothing on master, holds the REMOTEID tail `000000000d01` or `000000000d02`** (every file decoded, 10 Nov 2026).
- **`count17.py`:** row 2464-GR's `count16.py` with one change, marked CD.
  - The V kind also prints `REFERENCE`, `REFERENCEDATE`, `GSTNATUREOFRETURN`, `VATPARTYTRANSRETURNNUMBER`, `VATPARTYTRANSRETURNDATE`, `ISINVOICE`, `VCHENTRYMODE` and `PERSISTEDVIEW`.
  - It also prints each `LEDGERENTRIES.LIST` and `ALLLEDGERENTRIES.LIST` (how many, then each one's ledger, side and amount).
  - Tried on `0010`'s voucher cut from the `pilot-lab-vouchers-after-post` fixture and re-encoded as UTF-16LE, it prints `REFERENCE` `BP/26-27/0010`, `GSTNATUREOFRETURN` empty, `ISINVOICE` Yes, `VCHENTRYMODE` `Accounting Invoice`, and six legs in each list, as the invoice has them.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count17.py a-<label>-*`. A STOP from either script stops the row there.

**The prompt rule, at every step.** The only prompts answered are C5's and C6's. For any other prompt, message or dialog, copy it word for word and screenshot it first. Then answer No, or Esc if it offers no No. Press nothing more and send nothing. Then restore BK by C6, compare as step 5, and close (C10). That prompt is this row's result.

**The wait rule.** After each send at steps 3 and 4: if 5 minutes pass with no answer and no message, send nothing, press nothing, screenshot the whole window, and report to the run coordinator. A read sent while Tally is busy can hold its gateway. With the Gateway showing and no dialog, run C2's status check (`GET /status` 200, one tally.exe owning 9001) before the next read.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to the new empty folder `C:\lab\2346-cd\backup-1`. Row 2162-GY's step 1 is the path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `W-2026-08.xml V-1.xml R-CN1.xml R-DN1.xml R-0010.xml M-marks.xml`.
   - EXPECT W 40 rows, as row 2162-KC's s2 read (`lab/2162-kc-answers` at e3c2bd1f, RESULTS.md): none of type `Credit Note` or `Debit Note`. Any note: stop before step 3.
   - EXPECT V-1 2 rows: the Receipt numbered `1` (20260803) and the Purchase numbered `1` (20260804). A note among them: stop before step 3.
   - EXPECT R-CN1 0 rows and R-DN1 0 rows.
   - EXPECT R-0010 one row: `BRIDGE Walk-in`, included Yes, every other status flag No, 6 legs.
3. **The Credit Note.** `s3m`: `M-marks.xml`, then `count17.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass. Then no key is pressed in Tally before the send.
   - `s3`: `I-CN.xml`, sent once. Then the wait rule. Record every counter and any LINEERROR, word for word. The window after it (shot): the Gateway, no dialog. A dialog: the prompt rule.
   - `s3r`: `W-2026-08.xml V-1.xml R-CN1.xml R-0010.xml M-marks.xml`.
     - `W`: record every row s2's W does not hold, whole: its number, type, date, `ISCANCELLED`, the Sales class and its GUID.
     - `V-1`: record every row of type `Credit Note` whole: every field count17.py prints, the legs in both lists included.
     - `R-CN1`: record its rows: ledger, the three status flags and the legs.
     - `count17.py --diff a-s2-R-0010.xml a-s3r-R-0010.xml`: record the whole output, which says whether the invoice moved.
     - Record ALTVCHID and ALTMSTID against s3m's.
   - If the note was created under a number other than `1`, W shows it and V-1 and R-CN1 do not. Record it as found, and send no other request for it: a read by that number is the next row's.
   - Any answer at s3 is a result, created or not. Record it and go on to step 4. Never send `I-CN.xml` again (C3).
4. **The Debit Note.** `s4m`: `M-marks.xml`, then `count17.py --before-write a-s1-M-marks.xml a-s4m-M-marks.xml`. It must pass.
   - `s4`: `I-DN.xml`, sent once. Then the wait rule. Record every counter and any LINEERROR, word for word. The window after it (shot): the Gateway, no dialog.
   - `s4r`: `W-2026-08.xml V-1.xml R-DN1.xml R-CN1.xml R-0010.xml M-marks.xml`.
     - Record as at s3r, for the Debit Note.
     - `count17.py --diff a-s2-R-0010.xml a-s4r-R-0010.xml`: record the whole output.
     - Only if R-CN1 held one row at s3r and holds one at s4r: `count17.py --diff a-s3r-R-CN1.xml a-s4r-R-CN1.xml`, recorded whole. `--diff` refuses any other count of rows. That refusal is not a STOP, since the counts are already recorded.
   - Any answer at s4 is a result. Never send `I-DN.xml` again.
5. **Restore** BK by C6, with `restore-plan.txt` written first. Row 2162-GY's step 5 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace). Then `s5`: `M-marks.xml W-2026-08.xml V-1.xml R-CN1.xml R-DN1.xml R-0010.xml`, and compare whole:
   - `count17.py --same-row a-s1-M-marks.xml a-s5-M-marks.xml`: EXPECT it passes.
   - W 40 rows and V-1 2 rows, each answer's sha256 equal to s2's.
   - R-CN1 and R-DN1 0 rows. `count17.py --diff a-s2-R-0010.xml a-s5-R-0010.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
6. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those C5 and C6 name, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. Say in the report what the book then holds: a stop between steps 3 and 5 leaves whatever the imports did.

## Record
- Each answer's bytes and sha256, and `count17.py`'s lines.
- The `--diff`, `--before-write` and `--same-row` outputs, whole.
- The screenshots after each send, listed by name, bytes and sha256.
- As text and word for word: every prompt, message and busy screen, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2:** as the Book and step 2 say.
- **Step 3:** `CREATED` 1, numbered `1`, the legs stored as sent. This is the shape row 2027's `seed2L` created on another book and release (`lab/2027-window-read-answers` at 307427aa: `LEDGERENTRIES.LIST` under the invoice view, `CREATED` 1, where `ALLLEDGERENTRIES.LIST` answered `EXCEPTIONS` 1). It is also the shape row 1342-W23 keyed on this book (`lab/1342-w23-answers` at 67fa148a, `answers/aw23-b.xml`: `BRIDGE Walk-in` No, the rest Yes).
- **Step 4:** as step 3, for the Debit Note. Row 1342-W25 keyed one on this book (`lab/1342-w25-answers` at 4d80233a: `BRIDGE Walk-in` Yes, the rest No).
- **R-0010:** unchanged at s3r and s4r.
- **s5:** equal to s2 and s1.

## Decides
Each is PARTIAL: one import of each type, one sitting, one book, one release.
- **Each note is created, and its R row holds the six legs as sent:**
  - the Sales invoice core, with the type and the sides changed, is a note request on this release;
  - #2346's builder can reuse `render_sales_invoice_xml`'s shape;
  - the note's verification can reuse the Sales read-back with the note type's GUID.
  Record the status flags, and what Tally filled in `GSTNATUREOFRETURN` and the registration fields. The builder must say which it relies on.
- **Tally numbers the note itself:** record the number. The build cannot read a note back by a number it did not send. Before the builder is designed, it must choose: read back by a number Tally assigns (a read before the approval, which can go stale), send a number into a Manual note type (a further row), or read back by something else Tally keeps (a further shape).
- **A note is refused** (`CREATED` 0, an error or a LINEERROR): record the words. A change this row did not make is then needed, such as `GSTNATUREOFRETURN`, a number, or `ALLLEDGERENTRIES.LIST`. Each is its own row, one change at a time.
- **A note is stored other than as sent** (a side, an amount, a leg added or dropped): record it whole. The builder must not judge a note by the request's legs alone.
- **`R-0010` moves after a note:** a note naming the invoice in `REFERENCE` reached the invoice. Record which field, and report before the restore.
- **A note is created but R does not return it under its number:** the Sales read-back does not carry over by changing the type's GUID. Record what W and V hold for it.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build has no note type.** `VoucherType` (`src-tauri/src/agent_import.rs:330`) has Payment, Receipt, Journal, Contra and Sales, so no build sends `I-CN.xml` or `I-DN.xml`. Each is a raw edit of the build's captured Sales request, sent to measure the shape #2346 would build.
- **The read-back is bound to the type's GUID.** `render_invoice_readback_request` (`src-tauri/src/agent_import_invoice_wire.rs:505`, the filter at `:514`) filters on the date span, the number and `$GUID:VoucherType:$VoucherTypeName`. `read_back_sales_invoice` (`src-tauri/src/agent_import_invoice.rs:1693`) passes the observed type's GUID (`:1742`). `R-CN1.xml` and `R-DN1.xml` are that request with the note types' GUIDs.

## Not measured
- An `Agst Ref` or `New Ref` on a note, and a bill-wise customer.
- `GSTNATUREOFRETURN`, `VATPARTYTRANSRETURNNUMBER` or a registration written by the request.
- A part note, a tax-only note, an inter-state or IGST note.
- A note in invoice mode with items, and a purchase-side note.
- A note sent with a number, into an automatic or a Manual series.
- GSTR-1 for either note.
- A note built and posted through Bridge.
- Any other book or release.

Row: lab/2346-cd/
