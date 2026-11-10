# Row 2162-ST: does a second Manual Sales type, keyed with "Prevent duplicates", accept an invoice number that another Sales type already holds?

Not for merge. Part of #2162. Written 10 Nov 2026. Lines are at master ae38a75c, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
After a post, the build reads an invoice back in the posted type only.
- **Before the post,** the duplicate-number read counts every Sales-class voucher of the year holding the number, whatever its type (`render_invoice_number_request`, `src-tauri/src/agent_import_invoice_wire.rs:429`; refused at `src-tauri/src/agent_import_invoice.rs:2283` to `:2299`). It runs at the build, at the re-reads around the approval and in the endpoint queue.
- **After the post,** and at every later verification, the only read is the read-back (`read_back_sales_invoice`, `agent_import_invoice.rs:1693`). Its filter adds the posted type's GUID (`agent_import_invoice_wire.rs:514`), so it never sees another type.

So nothing reads between the queue's read and the import. The answer to the question below decides whether that gap can leave a duplicate.

- Row 2162-KD found that a keyed Manual Sales type declines a number it holds itself.
- Row 2346-MN found that a keyed Manual credit note type accepted a number that another credit type held (its s11).
- For Sales across two types, nothing is measured.

**This row changes one variable: the type.** `BRIDGE Sales Keyed` is keyed on screen as row 2162-KD keyed it. Then the build's request that created `BP/26-27/0010` on `BRIDGE Sales` is sent to it under a new `REMOTEID`, with nothing else changed. The row reads:
- the counters, every `LINEERROR`, and whether a dialog appears (P7);
- the number read (class-wide): how many Sales rows hold `BP/26-27/0010`, and in which types;
- the build's read-back for `BRIDGE Sales` and `BP/26-27/0010`: how many rows it finds.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2346-AR's and 2346-MN's) applies in full. Where it and this sheet differ, the stricter one holds.
- **The book is pinned by GUID.** `count22.py` holds BRIDGE PILOT LAB's GUID as `BOOK`. Stop before anything is sent unless s1's COMPANY row is the book's.
- **Its state:** every row queued before this one ends with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245. Other marks at s1: stop before BK, with nothing sent.
- **What it holds.** Row 2346-MN's reads of the restored book (`lab/2346-mn-answers` at 7b4b9ce5) showed:
  - `BP/26-27/0010` once, on `BRIDGE Sales`, Sales class Yes (its s10, `N-0010s`);
  - `BRIDGE Sales` with one series, Default, Manual, `PREVENTDUPLICATES` Yes, and no type named `BRIDGE Sales Keyed` (its s2 T, byte-equal to row 2162-KD's s2).

  Step 2 reads these again.
- **The key.** The new `REMOTEID` ends `000000000796`. No blob on any `lab/` branch or on master holds that string (666 refs and 23,929 blobs, each read as UTF-8 and as UTF-16LE at both byte offsets).
- **The date:** read the Gateway's current date at step 1. If it is before 1-Aug-2026, stop: the invoice is dated 1 Aug 2026, and a post-dated invoice is another row's variable.
- **BK is this row's way back.** It is taken at step 1, before anything is keyed, and restored at step 5 by C6. After a STOP (C7, C8 or C9), C8 holds, and the restore waits for the run coordinator's word.
- Write down the release shown on Tally's screen. Row 2346-MN read `tally.exe` 1.1.7.2 from the program's file.

**No Bridge build.** Raw requests, one at a time, through `run.py`. The build cannot send this invoice, because Sales is not a qualified voucher type. Nothing is keyed in Tally but BK (C5), the type (step 3) and the restore (C6). The one import is `I-0010k.xml`, sent at most once.

## The sends
`run.py` is row 2346-MN's (byte-equal to row 2346-AR's).
- One request at a time, and `run.py` never abandons a request.
- **The wedge rule.** If any send has not answered **5 minutes** after it began:
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).
- **The stops** are failed requests, never "nothing found":
  - from `run.py`: a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 B;
  - from `count22.py`: any STOP.

  Each stops the row there. Nothing is re-sent on this row's own judgement.
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count22.py`, and copy its whole output into RESULTS.md.
- Close no Bridge process, and call no Bridge tool.

## Files
Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.
- **Byte-equal to row 2346-MN's file of the same name** (`lab/2346-mn-sheet` at cec5d5a8, folder `lab/2346-mn/`):
  - `.gitattributes`, `LAB-COMMON.md`, `M-marks.xml` and `run.py`;
  - `T-types.xml`: the build's voucher-type read (`render_voucher_types_request`), the read the resolver judges;
  - `N-0010s.xml`: the build's number read for `BP/26-27/0010` (`render_invoice_number_request`), with its Sales-class compute.
- **Byte-equal to row 2346-AR's file of the same name** (`lab/2346-ar-sheet` at fd0aef60, folder `lab/2346-ar/`):
  - `R-0010.xml`: the build's read-back for `BP/26-27/0010` on `BRIDGE Sales`, its type GUID ending `000000ce`;
  - `template-import-0010.utf16le.xml`: the build's request that created `BP/26-27/0010`.
- **Byte-equal to row 2162-KD's file** (`lab/2162-keyed-dup-sheet` at c5b8e06e, folder `lab/2162-keyed-dup/`): `template-full-1.utf16le.xml`.
- **`gen.py`** writes these, asserting every change:
  - `I-0010k.xml`: `template-import-0010.utf16le.xml` with the type `BRIDGE Sales Keyed` (in `VCHTYPE` and `VOUCHERTYPENAME`) and the `REMOTEID` tail `0796`. The number, party, figures and date are unchanged. Turned back, it is its source byte for byte (asserted).
  - `V-0010.xml`: every native field of the vouchers numbered `BP/26-27/0010` in 2026-27, built as row 2162-KD's `V-0061.xml`.
- **`count22.py`:** row 2346-MN's `count21.py` with two lines changed, each marked ST. Its T kind names `BRIDGE Sales` and `BRIDGE Sales Keyed`, and it predicts no type GUID.

**Tried before this sheet was sent:** `count22.py` on row 2346-MN's answers, each exit 0:
- s1 M: the book's GUID, ALTVCHID 68, ALTMSTID 245;
- s2 T: 28 rows. `BRIDGE Sales` reads one series, Default, Manual, `PREVENTDUPLICATES` Yes, with its GUID ending `000000ce`. `BRIDGE Sales Keyed` reads 0;
- s10 `N-0010s`: one row, `BP/26-27/0010` on `BRIDGE Sales`, `BRIDGEVCHISSALES` Yes.

## Steps
Run each label as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count22.py a-<label>-*`.

**The prompt rule, at every step.** Only these prompts are answered, each with the key named: those step 3 names, C5's and C6's.
- For any other prompt, message or dialog, copy it word for word and screenshot it first.
- Then answer No, or Esc if it offers no No. Press nothing more and send nothing.
- Then restore BK by C6, compare as step 5, and close (C10). That prompt is this row's result.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1).
   - Read the Gateway's current date and period, and write them down. Before 1-Aug-2026: stop.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2162-st\backup-1`. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `T-types.xml N-0010s.xml V-0010.xml R-0010.xml M-marks.xml`.
   - EXPECT:
     - in T, `BRIDGE Sales` as above, and `BRIDGE Sales Keyed` 0;
     - in N, one row, `BP/26-27/0010`, `BRIDGE Sales`, `BRIDGEVCHISSALES` Yes;
     - in V, one row, `BRIDGE Sales`;
     - in R, one row.
   - Anything else: record it and stop before step 3.
3. **Key the voucher type on screen.** Row 2162-KD's step 3, word for word, with the same name, `BRIDGE Sales Keyed`.
   - `s3m`: `M-marks.xml`, then `count22.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass.
   - From the Gateway: Create, then Voucher Type. Set only these four fields:
     - Name `BRIDGE Sales Keyed`;
     - Select type of voucher `Sales`;
     - Method of voucher numbering `Manual`;
     - Prevent duplicates `Yes`. If Tally does not offer it once Manual is chosen, stop before saving and ask.
   - Screenshot the screen before saving, and write down every field with its value, as text.
   - Save with Ctrl+A. If Tally asks "Accept ?", answer Yes. If Tally then opens a new blank Voucher Type Creation screen, leave it with Ctrl+Q.
   - `s3r`: `T-types.xml M-marks.xml`. EXPECT:
     - in T, one row named `BRIDGE Sales Keyed`, `PARENT` `Sales`, with one series, Manual, `PREVENTDUPLICATES` Yes. Record its GUID;
     - `BRIDGE Sales` as at s2;
     - in M, ALTVCHID 68 and ALTMSTID above 245.
   - Any other answer, above all a series that is not Manual and Yes: record it, send no invoice, and go to step 5.
4. **The number another Sales type holds.**
   - `s4m`: `M-marks.xml`, then `--before-write` against s1. It must pass.
   - `s4`: `I-0010k.xml`, sent once. Screenshot the window at once.
   - `s4r`: `N-0010s.xml V-0010.xml R-0010.xml M-marks.xml`.
   - **Its answer** is one of these:
     - **created:** `CREATED` 1, `ERRORS` 0, `EXCEPTIONS` 0, no `LINEERROR`. EXPECT N two rows, one on each type, each `BRIDGEVCHISSALES` Yes; V two rows; R one row, the `BRIDGE Sales` invoice as at s2.
     - **declined in the answer:** `CREATED` 0, `EXCEPTIONS` 1 or `ERRORS` 1. Record every counter and every `LINEERROR` word for word. EXPECT N, V and R as at s2, and `count22.py --same-row a-s3r-M-marks.xml a-s4r-M-marks.xml` to pass.
   - Any other answer, or a dialog: the prompt rule.
5. **Restore** BK by C6, with `restore-plan.txt` written first. Row 2346-MN's step 13 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace).
   - Then `s9`: every file of s2, in s2's order, and compare whole:
     - `count22.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`: EXPECT it passes;
     - T `BRIDGE Sales Keyed` 0;
     - each answer's sha256 against s2's: EXPECT each equal.
   - **Any part not equal stops the row:** send nothing more, and report before the close.
6. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog after a write, or if the gateway stops answering. Screenshot the window, send it and the answers to the run coordinator, and ask before anything else is pressed or sent.

## Record
- Each answer's bytes and sha256, and `count22.py`'s output, whole.
- `run.py`'s log for each label.
- Step 3's screen: every field and value as text, and the screenshots by name, bytes and sha256.
- The window after the import, by name, bytes and sha256.
- Every prompt, word for word, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s3r:** `BRIDGE Sales Keyed` reads as row 2162-KD's s3r: Manual, `PREVENTDUPLICATES` Yes.
- **s4:** created. Row 2346-MN's s11 created the same number in a second credit type. Nothing measured says Sales types differ.

## Decides
Each is PARTIAL: one book, one release, one sitting.
- **Created, with N two rows and R one:**
  - Tally's duplicate guard is per type on Sales too.
  - The build's read after the post does not see the second invoice, because it reads the posted type only.
  - So an invoice whose number another Sales type takes between the queue's read and the import would read `posted_verified`.
  - #2162 needs the number read sent again after the post. `posted_verified` would then need exactly one Sales-class row, the posted invoice.
- **Declined in the answer:**
  - Tally's guard spans Sales types on this book, and the import refuses the case the read after the post misses.
  - The read after the post is then a defence, not the only guard. Whether to add it stays the maintainers' call.
- **Created, but N or R not as expected:** record every row whole. The coordinator decides.
- **A dialog after the import:** the row stops (see Stop). A dialog is the failure P7 rules out, and it decides the design before any count does.

**What these answers are for.** They inform #2162's read after the post. They are never a regression fixture for a request the build renders, because the build posts no invoice into a second type.

## Code admits the trigger
Lines are at master ae38a75c.
- **Sales is not posted today.** `LIVE_QUALIFIED_VOUCHER_TYPES` (`src-tauri/src/agent_import.rs:421`) holds Journal, Payment, Receipt and Contra. So no release sends this invoice, and this row sends it raw.
- **The resolver admits a second Manual Sales type.** `resolve_voucher_type` (`agent_import_invoice_wire.rs:329`) checks the named type's own series (`:384` to `:390`). It does not ask whether another Sales type holds the number.
- **The number read is class-wide, and the read-back is per type.** See the Question. N and R are those two requests, byte-equal.
- **What the build sends.** `I-0010k` is the build's request for `0010` with the type and `REMOTEID` changed. The same request with another type name is what the build would send to a type named so.

## Not measured
- Two keyed types each sent a fresh number.
- A number held by a Sales type that is not Manual.
- A case or leading-zero variant of the number.
- An invoice built and posted through ComplyEaze Bridge.
- Any other book or release.

Row: lab/2162-st/
