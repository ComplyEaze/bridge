# Row 2162-GF: where the company's GST setting reads in the answers the Sales build already fetches, and what turning it off does to the registration the build parses

Not for merge. Part of #2162 (ADR 0004, the 2026-10-31 checklist, item 16: the company's GST flag). Written 9 Nov 2026. One company setting changed on Tally's screen, read before and after, then a restore from the row's own backup. Confidence of everything here until the row runs: unverified.

## Question
The ruling for item 16: a GST invoice for a company whose GST is turned off must be refused by name, which means the build has to read the setting. Today it does not:
- **What the invoice judge reads (code reading, master ae38a75c).** It reads the company's tax units (`render_company_registration_request`, `agent_import_invoice_wire.rs:694`) and parses them with `parse_company_registration` (`:731`, called at `agent_import_invoice.rs:2338`), which never looks at a GST setting. It also reads the company object with every field (`InvoiceCompanyHsnV1`, `INVOICE_COMPANY_FETCH = ["*"]`, `xml_read_profiles.rs:499`; sent at `agent_import_invoice.rs:2099`) and parses it with `parse_company_hsn` (`agent_import_invoice_hsn.rs:437`). That parse reads the GUID, the HSN rows, the company's GST settings rows and `ISGSTCLASSIFON` (`:466`), but not `ISGSTON`.
- **Where the setting already reads.** The captured answer to that same company object request on this book (`pilot-lab-company-object.utf16le.xml`, in the `pilot-lab` fixtures) holds `<ISGSTON TYPE="Logical">Yes</ISGSTON>`. A separate company features read (`native_company_features.rs`, which reads `ISGSTON` at `:210`) was compared with the F11 screen on other synthetic books (§12a.18). It is not part of the invoice build.

So the cheapest place for the build to read the setting is the object answer it already fetches, with no new request (P4), but only if that answer's `ISGSTON` follows the screen. This row measures that. It also measures what turning GST off does to the tax units the parser reads today, to see whether the build already refuses that book some other way.
- **The one variable:** BRIDGE PILOT LAB's company setting for GST (F11), from Yes to No. Nothing else changes.
- **What is read, before and after:** the company object (O, the build's own request), the tax units (U), the voucher types (T), the read-back of `BP/26-27/0010` (R) and the marks (M).
- **Not asked here:** GSTR-1, and anything posted while the setting is off.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2162-KC's, 2162-SC's and 2162-RU's) applies in full. Where it and this sheet differ, the stricter one holds.
- **The book is pinned by GUID.** `count15.py` holds BRIDGE PILOT LAB's GUID as `BOOK`. Its M and O kinds print whether the COMPANY row's GUID is the book's, its U kind whether each tax unit's GUID begins with it, and `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called. STOP before anything is keyed unless s1's COMPANY row is the book's.
- Its state, from the lab's last notes on it: rows queued before this one each end with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245.
- **BK is this row's way back,** taken at step 1 before anything is keyed and restored at step 5 by C6. After the restore every read is compared whole with the reads before the change, and a part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Rows 2162-M1 and 2162-B2C read TallyPrime 7.2 Silver (tally.exe 1.1.7.2).

**No Bridge build.** Raw reads, one at a time, through `run.py`. The change is made on Tally's screen, by hand; no request writes.

**The sends** are row 2162-RU's, unchanged, plus one more read. Each of these stops the run and is reported as "the request failed", never as "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`); a collection answer with no `BODY/DATA/COLLECTION`, an object answer without exactly one `BODY/DATA/TALLYMESSAGE/COMPANY`, or an answer that does not parse (`count15.py`, which prints STOP and exits non-zero). Check each file's sha256 against `SHA256.txt` before its send. Count every answer with `count15.py` and copy its whole output into RESULTS.md. Close every other Bridge process first.

## Files
- **Byte-equal to row 2162-RU's file of the same name** (`lab/2162-ru-sheet` at b5418f19, folder `lab/2162-ru/`), whose ROW.md says where each came from: `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml`, `U-taxunits.xml` and `T-types.xml`.
- **`O-object.xml`:** the build's company object request for this book, byte-equal to `pilot-lab-company-object.request.utf16le.xml` in the `pilot-lab` fixtures (750 bytes, sha256 beginning `72d8cd92`). A test asserts that the build's `InvoiceCompanyHsnV1` request renders it character for character (`invoice_company_hsn_is_the_company_object_request_the_lab_answered`, `xml_read_profiles_tests.rs:239`). It is not a new shape.
- **`count15.py`:** row 2162-RU's `count14.py` with two changes, each marked GF:
  - a kind O, which prints the one COMPANY row of an object answer: its GUID and whether it is the book's, `ISGSTON`, `ISGSTCLASSIFON`, `GSTAPPLICABLEDATE`, and how many `CMPGSTDETAILS.LIST` and `HSNDETAILS.LIST` it holds;
  - `--object-diff A B`, which prints every leaf of that COMPANY row that differs between two O answers, at most 200 lines.
  - Tried on the `pilot-lab-company-object` fixture, it prints the book's GUID, `ISGSTON` Yes, `ISGSTCLASSIFON` No, one row of each list. On a copy with `ISGSTON` set to No, `--object-diff` printed that one leaf; on the answer against itself, none.
- Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its size.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count15.py a-<label>-*`. A STOP from either script stops the row there.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID. Record ALTVCHID and ALTMSTID (68 and 245 expected) and go on.
   - **Take BK** by C5, to the new empty folder `C:\lab\2162-gf\backup-1`. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `O-object.xml U-taxunits.xml T-types.xml R-0010.xml M-marks.xml`.
   - EXPECT O one COMPANY row, the book's GUID, `ISGSTON` Yes, `ISGSTCLASSIFON` No (as the fixture). Any other: record it and stop before step 3.
   - EXPECT U as row 2162-SC's s2 expects: `Default Tax Unit`, and `Rajasthan Registration` with `08ZZZZZ0000Z1ZQ` and one dated row `20260401`, Rajasthan, Regular, not inactive.
   - EXPECT R-0010 one row: `BRIDGE Walk-in`, included Yes and every other status flag No. Record T's lines.
3. **The change, on Tally's screen.** Before it, `s3m`: `M-marks.xml`, then `count15.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass; then no key is pressed in Tally but those below.
   - **The path rule.** Every screen below either cites the earlier row whose answers used that exact path, or is marked NEW PATH. If the line or screen shown is not exactly what the step names: stop, Esc out (a list or prompt, never the features screen itself, C7), restore BK by C6 and check as step 5, then Quit (C10).
   - **The prompt rule** (the coordinator's, after row 2162-SC's stop). If Tally shows any prompt or message on turning GST off, at 3.3 or at 3.5, whether a warning or a notice about data already entered: copy it word for word and screenshot it first, then answer No, or Esc if it offers no No. Press nothing more on the features screen. Then stop and restore: leave any screen still open by Ctrl+Q (C7), restore and compare by step 5, and close. That prompt is this row's result.
   1. At the Gateway (shot), press F11 (Features), for the screen `Company Features Alteration`. Row 1342-W14 opened it this way on this book (its notes, step 2.5: Taxation, GST Yes; quit unchanged by Ctrl+Q). If the button bar names another key, or the title differs: the path rule. Write down the screen's title and every field it shows with its value, as text, before anything is changed (shot).
   2. Find the field whose label names Goods and Services Tax (GST) and reads Yes. Exactly one such field, or stop: press nothing more, and leave only as C7 says, on the coordinator's word.
   3. **NEW PATH: no earlier row has turned GST off on an existing book.** Row 1231 (its cost-centre answers) changed one other field on this screen, `Enable Cost Centres`, on another book and accepted it by Ctrl+A; that is the nearest path seen. Move to the field with Enter or the arrow keys only (never Esc on this screen, C7). Set it to No by typing `n` or by the item its list offers. Zoom the field and check that it reads No before going on.
   4. **Touch no other field.** If setting it opens another field, a sub-screen or a list, write down every line it shows, press nothing more, screenshot, and stop.
   5. Accept with Ctrl+A (shot after). An `Accept ?` prompt is answered Yes. **Any other prompt or message** (for example about GST details or vouchers already entered): the prompt rule.
   6. Write down every screen passed until the Gateway shows (shot), and whether its GST lines or menus changed.
4. **The book with GST off.** `s4`: `M-marks.xml O-object.xml U-taxunits.xml T-types.xml R-0010.xml`. Then, each recorded whole:
   - `count15.py --object-diff a-s2-O-object.xml a-s4-O-object.xml`;
   - `count15.py --unit-diff a-s2-U-taxunits.xml a-s4-U-taxunits.xml`;
   - `count15.py --type-diff a-s2-T-types.xml a-s4-T-types.xml`;
   - `count15.py --diff a-s2-R-0010.xml a-s4-R-0010.xml`.
   - Record ALTMSTID and ALTVCHID against s3m's. Any read that now fails is a result: record it as the request failed, with its words, and go on to step 5.
5. **Restore** BK by C6, with `restore-plan.txt` written first. Then `s5`: `M-marks.xml O-object.xml U-taxunits.xml T-types.xml R-0010.xml`, and compare whole:
   - `count15.py --same-row a-s1-M-marks.xml a-s5-M-marks.xml`: EXPECT it passes.
   - `--object-diff`, `--unit-diff` and `--type-diff` of s2 against s5: EXPECT 0 leaves differ in each.
   - `count15.py --diff a-s2-R-0010.xml a-s5-R-0010.xml`: EXPECT only `lists compared` and `legs: 6 -> 6`.
   - Record each answer's sha256 against s2's. **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
6. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those step 3's prompt rule, 3.5 and C6 name, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent. A stop between steps 3 and 5 leaves the book with GST off: say so in the report.

## Record
- Each answer's bytes and sha256, and `count15.py`'s lines.
- The `--object-diff`, `--unit-diff`, `--type-diff`, `--diff`, `--before-write` and `--same-row` outputs, whole.
- Step 3's screenshots, listed by name, bytes and sha256. As text and word for word: the features screen's fields before the change, every key pressed, and every prompt or message with the key that answered it.
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2:** O, U, R as the fixtures and row 2162-SC's s2 read.
- **Step 3:** the setting takes No. Whether Tally warns that the book holds GST details or GST vouchers is not known; a warning is a result.
- **s4:** O's `ISGSTON` reads No, and few other leaves move. U unchanged: the registration stays on the books with GST off. R-0010 unchanged.
- **s5:** equal to s2 and s1.

## Decides
- **O's `ISGSTON` reads No at s4 and Yes at s2 and s5:** the setting reads in the answer the build already fetches. The build then reads `ISGSTON` from the company object beside `ISGSTCLASSIFON`, in `parse_company_hsn` or in the judge that calls it, and refuses an invoice by name unless it reads exactly Yes. An absent or empty value also refuses (P3). No new request is needed. Its tests run on the s2 and s4 answers once they are pushed as captured fixtures (P1).
- **O's `ISGSTON` does not move, or is absent:** the object answer does not carry the setting. The next row reads the company features collection (`render_company_features_request`), whose `ISGSTON` followed the screen on other books (§12a.18). The build then needs that read added to the invoice judge.
- **U is unchanged at s4:** today's build would parse a valid registration on a company with GST off and admit the invoice, so the refusal above closes a real gap (code reading plus this answer).
- **U changes at s4** (a unit removed, a dated row added or marked inactive): record exactly what moved. If the parser already refuses the s4 answer, the refusal it gives is recorded as the build's present behaviour, by code reading, and a test on the captured answer pins it. The named refusal is still added, so the reason given is GST being off, not a side effect.
- **T or R-0010 changes at s4:** turning GST off reaches the voucher types or a posted voucher. Record which fields, and report before the restore.
- **Tally refuses the change, or asks before it:** record the words. The coordinator decides whether that answer is pressed in a later sitting.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build reads the company object and the tax units this row changes**, on every invoice it builds: `agent_import_invoice.rs:2099` (the object request) and `:2338` (the tax units, parsed by `parse_company_registration`, `agent_import_invoice_wire.rs:731`). The object is parsed by `parse_company_hsn` (`agent_import_invoice_hsn.rs:437`).
- **Neither parse reads the GST setting.** `parse_company_registration` reads the tax units only. `parse_company_hsn` reads `ISGSTCLASSIFON` (`:466`) and not `ISGSTON`. If the tax units survive the change, no code path refuses the invoice for GST being off.

## Not measured
- Turning GST on again from No on a book that never had it.
- A company whose GST was never on.
- The company features collection's `ISGSTON` on this book (read only if O does not carry the setting; see Decides).
- GSTR-1, and any invoice built or posted while GST is off: no Bridge build runs.
- Any other book or release.

Row: lab/2162-gf/
