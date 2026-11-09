# Row 2162-GY: row 2162-GF's change, with Tally's own message answered Y, to read where the company's GST setting lands with GST off, then a restore

Not for merge. Part of #2162 (ADR 0004, the 2026-10-31 checklist, item 16: the company's GST flag). Written 10 Nov 2026. Confidence of everything here until the row runs: unverified.

## Question
The ruling for item 16: a GST invoice for a company whose GST is turned off must be refused by name, so the build has to read the setting. Row 2162-GF (`lab/2162-gf-answers` at 61896c16, `RESULTS.md`) asked where that setting reads. Its run stopped at the message Tally shows before saving:

- Tally asks no "Accept ?" for this change. On Ctrl+A it shows a message box, word for word:
  - "GST-related changes in masters may impact the GST Status of vouchers in Returns."
  - "Y : Accept Master & Update GST Status of Vouchers"
  - "N : Back"
- Its prompt rule answered N, so the setting was never saved. With the field reading No on screen but not saved, the company object still read `ISGSTON` Yes. GF's s4 was never taken.

"The book is GST-on unless that message is accepted" does not settle item 16. A client's book can hold GST off, accepted by its own user, and today's build has no check for it (`Code admits the trigger`, below). Where to read the setting depends on the answer to GF's question, which is still open: does the object answer's `ISGSTON` follow a saved change?

This row asks the same question, with one difference from GF: **the message is answered Y**. Everything else is GF's.

- **The one variable:** BRIDGE PILOT LAB's company setting for GST (F11), from Yes to No, saved through Tally's message by Y. Nothing else changes.
- **What is read, before and after:** the company object (O, the build's own request), the tax units (U), the voucher types (T), the read-back of `BP/26-27/0010` (R) and the marks (M).
- **Not asked here:**
  - GSTR-1, and anything posted while the setting is off;
  - what Y would do on a book with more GST vouchers. This book holds the ones it holds.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-KC's, 2162-SC's, 2162-RU's and 2162-GF's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count15.py` holds BRIDGE PILOT LAB's GUID as `BOOK`.
  - Its M and O kinds print whether the COMPANY row's GUID is the book's, and its U kind whether each tax unit's GUID begins with it.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is keyed unless s1's COMPANY row is the book's.
- **Its state,** from the lab's last notes on it: rows queued before this one each end with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245.
- **BK is this row's way back.** It is taken at step 1, before anything is keyed, and restored at step 5 by C6.
  - Y writes to the book's vouchers as well as its features. So the restore is not optional: it runs after step 4 in every case, including a stop.
  - After the restore, every read is compared whole with the reads before the change. Any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Rows 2162-M1 and 2162-B2C read TallyPrime 7.2 Silver; rows 2162-RU and 2162-GF read TallyPrime Silver with tally.exe 1.1.7.2.

**No Bridge build.** Raw reads, one at a time, through `run.py`. The change is made on Tally's screen, by hand; no request writes.

**The sends** are row 2162-GF's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, an object answer without exactly one `BODY/DATA/TALLYMESSAGE/COMPANY`, or an answer that does not parse (`count15.py`, which prints STOP and exits non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count15.py` and copy its whole output into RESULTS.md.
- Close every other Bridge process first.

## Files
Byte-equal to row 2162-GF's file of the same name (`lab/2162-gf-sheet` at 09bfc8d4, folder `lab/2162-gf/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count15.py`, `M-marks.xml`, `O-object.xml`, `R-0010.xml`, `U-taxunits.xml` and `T-types.xml`.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count15.py a-<label>-*`. A STOP from either script stops the row there.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to the new empty folder `C:\lab\2162-gy\backup-1`. Row 2162-GF's step 1 is the path: the cursor opens on Company Name, Backspace to Backup Data Path, Specify Path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `O-object.xml U-taxunits.xml T-types.xml R-0010.xml M-marks.xml`.
   - EXPECT each answer's sha256 equal to row 2162-GF's s2 answer of the same request (`lab/2162-gf-answers` at 61896c16):
     - M, T and R-0010: `answers/a-s2-*` in that branch;
     - O and U: GF's `kept-local.txt`, which lists their size and sha256. The files stay on the lab PC under the same rules (O names the company number; U holds a GSTIN shape).
   - Record any that is not equal, with `--object-diff`, `--unit-diff`, `--type-diff` or `--diff` against GF's file, and stop before step 3.
3. **The change, on Tally's screen.** Before it, `s3m`: `M-marks.xml`, then `count15.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass. After that, no key is pressed in Tally but those below.
   - **The path rule.** Every screen below either cites the earlier row whose answers used that exact path, or is marked NEW PATH. If the line or screen shown is not exactly what the step names: stop, then Esc out (a list or prompt, never the features screen itself, C7). Restore BK by C6, check as step 5, then Quit (C10).
   - **The prompt rule.** Step 3.5 names the one message this row answers Y. For any other prompt or message, at any step: copy it word for word and screenshot it first, then answer No, or Esc if it offers no No. Press nothing more. Then leave any screen still open by Ctrl+Q (C7), restore and compare by step 5, and close. That prompt is this row's result.
   1. At the Gateway (shot), press F11 (Features), for the screen `Company Features  Alteration`.
      - Rows 1342-W14 and 2162-GF opened it this way on this book (GF's `evidence/s3-screens.txt`, step 3.1).
      - If the button bar names another key, or the title differs: the path rule.
      - Write down every field the screen shows with its value, as text, before anything is changed (shot). EXPECT the fields GF recorded at `s3-features-before`.
   2. Find the field `Enable Goods and Services Tax (GST)`, reading Yes. Exactly one such field, or stop: press nothing more, and leave only as C7 says, on the coordinator's word.
   3. Row 2162-GF's step 3.3 is the path:
      - Down 13 times from Maintain Accounts, captured after 7, 5 and 1. Each field passed keeps its value.
      - `n`, then Enter. Zoom the field: EXPECT it reads No, its sub-line `Set/Alter Company GST Rate and Other Details` dimmed and reading No, and the cursor on Enable Tax Deducted at Source (TDS).
      - Never Esc on this screen (C7).
   4. **Touch no other field.** If setting it opens another field, a sub-screen or a list, write down every line it shows, press nothing more, screenshot, and stop.
   5. **Ctrl+A.** Row 2162-GF's step 3.5 is the path to the message box.
      - EXPECT the message box GF recorded, word for word (the three lines in Question). Screenshot it and zoom it before any key.
      - If the words differ in any way: the prompt rule (N, then restore).
      - If they are the same: **NEW PATH: no earlier row has answered Y.** Press `y` once.
   6. **After `y`.**
      - Press nothing while Tally shows a progress bar, a count or a busy window.
      - Write down every screen and message it shows, with the time of each (UTC), until the Gateway shows (shot).
      - Any further prompt or message: the prompt rule.
      - **The wait rule.** If 5 minutes pass after `y` with no Gateway and no further message: send nothing, press nothing, screenshot the whole window, and report to the run coordinator. A read sent while Tally is busy can hold its gateway.
      - With the Gateway showing, write down whether its GST lines or menus changed, then go on to C2's status check (`GET /status` 200, one tally.exe owning 9001) before any read.
4. **The book with GST off.** `s4`: `M-marks.xml O-object.xml U-taxunits.xml T-types.xml R-0010.xml`. Then, each recorded whole:
   - `count15.py --object-diff a-s2-O-object.xml a-s4-O-object.xml`;
   - `count15.py --unit-diff a-s2-U-taxunits.xml a-s4-U-taxunits.xml`;
   - `count15.py --type-diff a-s2-T-types.xml a-s4-T-types.xml`;
   - `count15.py --diff a-s2-R-0010.xml a-s4-R-0010.xml`.
   - Record ALTMSTID and ALTVCHID against s3m's.
   - Any read that now fails is a result: record it as the request failed, with its words, and go on to step 5.
5. **Restore** BK by C6, with `restore-plan.txt` written first. Row 2162-GF's step 5 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace). Then `s5`: `M-marks.xml O-object.xml U-taxunits.xml T-types.xml R-0010.xml`, and compare whole:
   - `count15.py --same-row a-s1-M-marks.xml a-s5-M-marks.xml`: EXPECT it passes.
   - `--object-diff`, `--unit-diff` and `--type-diff` of s2 against s5: EXPECT 0 leaves differ in each.
   - `count15.py --diff a-s2-R-0010.xml a-s5-R-0010.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`.
   - Record each answer's sha256 against s2's. **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
6. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those that step 3's prompt rule, 3.5 and C6 name, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. A stop between step 3.5's `y` and step 5 leaves the book with GST off and its vouchers' GST status updated: say so in the report.

## Record
- Each answer's bytes and sha256, and `count15.py`'s lines.
- The `--object-diff`, `--unit-diff`, `--type-diff`, `--diff`, `--before-write` and `--same-row` outputs, whole.
- Step 3's screenshots, listed by name, bytes and sha256.
- As text and word for word: the features screen's fields before the change, every key pressed, and every prompt, message and busy screen, with the key that answered it and its time.
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2:** each sha256 equal to row 2162-GF's s2.
- **Step 3.5:** GF's message box, word for word.
- **Step 3.6:** the Gateway after a short pause, with no other message.
- **s4:**
  - O's `ISGSTON` reads No.
  - O may also carry the 28 elements GF's s3q held after an unsaved change (`RESULTS.md`, "s3q"). They are recorded, not judged.
  - U unchanged: the registration stays on the books with GST off.
  - R-0010's GST status flags (`VCHGSTSTATUSISINCLUDED`, `VCHGSTSTATUSISUNCERTAIN`, `VCHGSTSTATUSISEXCLUDED`) may move. That is what Y names.
- **s5:** equal to s2 and s1.

## Decides
Row 2162-GF's Decides, applied to s4, and one more for R-0010.

- **O's `ISGSTON` reads No at s4, and Yes at s2 and s5:** the setting reads in the answer the build already fetches.
  - The build then reads `ISGSTON` from the company object beside `ISGSTCLASSIFON`, in `parse_company_hsn` or in the judge that calls it.
  - It refuses an invoice by name unless the value reads exactly Yes. An absent or empty value also refuses (P3).
  - No new request is needed. Its tests run on the s2 and s4 answers as captured fixtures (P1). Under the lab's present rules the object answer stays on the lab PC, because it names the company number. Before the build's tests, the coordinator decides how the s2 and s4 copies reach the fixtures: whole, as `pilot-lab-company-object` was, or with that one value substituted.
- **O's `ISGSTON` does not move, or is absent:** the object answer does not carry the setting.
  - The next row reads the company features collection (`render_company_features_request`), whose `ISGSTON` followed the screen on other books (§12a.18).
  - The build then needs that read added to the invoice judge.
- **U is unchanged at s4:** today's build would parse a valid registration on a company with GST off and admit the invoice. So the refusal above closes a real gap (code reading plus this answer).
- **U changes at s4** (a unit removed, a dated row added or marked inactive): record exactly what moved.
  - If the parser already refuses the s4 answer, the refusal it gives is recorded as the build's present behaviour, by code reading, and a test on the captured answer pins it.
  - The named refusal is still added, so the reason given is GST being off, not a side effect.
- **T changes at s4:** turning GST off reaches the voucher types. Record which fields.
- **R-0010 changes at s4:** record which fields moved. A posted invoice's GST status moving with the company setting is a fact for the read side (the GST status the registers read), not for item 16. It goes in RESULTS' follow-ups.
- **Tally shows anything at 3.5 or 3.6 that this sheet does not name:** record the words. The coordinator decides whether that answer is pressed in a later sitting.

## Code admits the trigger
Lines are at master ae38a75c, as row 2162-GF read them.
- **The build reads the company object and the tax units this row changes, on every invoice it builds.**
  - The object request is sent at `agent_import_invoice.rs:2099` and parsed by `parse_company_hsn` (`agent_import_invoice_hsn.rs:437`).
  - The tax units are read at `:2338` and parsed by `parse_company_registration` (`agent_import_invoice_wire.rs:731`).
- **Neither parse reads the GST setting.** `parse_company_registration` reads the tax units only. `parse_company_hsn` reads `ISGSTCLASSIFON` (`:466`) and not `ISGSTON`. If the tax units survive the change, no code path refuses the invoice for GST being off.

## Not measured
- Turning GST on again from No on a book that never had it.
- A company whose GST was never on.
- The company features collection's `ISGSTON` on this book (read only if O does not carry the setting; see Decides).
- GSTR-1, and any invoice built or posted while GST is off: no Bridge build runs.
- Y on a book with vouchers of other GST kinds, and Y's effect on a voucher this book does not hold.
- Any other book or release.

Row: lab/2162-gy/
