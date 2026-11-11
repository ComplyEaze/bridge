# Row 2346-N134: a numbered Credit Note in a keyed Manual type, read back at once and again at +5 s and +60 s (N1, N4), and the build's month number read with each note class's compute (N3)

Shape discovery, as rows 2346-CD and 2346-MN. The note is a request written for the lab, not one the build renders, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2346 (0.9.0): cells N1, N3 and N4 of the design proposal on the issue, in one sitting. Written 11 Nov 2026. Lines are at master 7a3c9e49, where this branch starts.

One voucher type is keyed on screen and one note is sent once, with read-only reads before and after it. Then the book is restored from this row's own backup. Confidence of everything here until the row runs: unverified.

## Question
Design A posts a note through the invoice core, which needs three things that hold for Sales and have not been measured together for a note:
- **N1:** a note sent with a number, into a keyed Manual, prevent-duplicates Credit Note type, keeps that number. The build's read-back, by the type's GUID and the number, returns the legs as sent and `REFERENCE` as the original invoice's number, not the note's.
- **N4:** that read-back, sent at once after the import's answer, equals the same read-back at +5 s and at +60 s, byte for byte. Row 2162-S2 measured this for a Sales invoice (#2162 item 2). No note has been measured.
- **N3:** the build's number read of one calendar month, the request master sends today (`render_invoice_window_numbers_request`, `src-tauri/src/agent_import_invoice_wire.rs:467`), tells the note apart when its `COMPUTE` carries the note class's function in place of `$$IsSales`. The note's row answers Yes to the credit class only, and every other row of the month answers No.

**Measured so far** (each PARTIAL: one synthetic book, one release, one sitting):
- Row 2346-MN (`lab/2346-mn-answers` at 7b4b9ce5) sent this note's request with another key: `CN/26-27/0001` into `BRIDGE Credit Note Manual`, `REFERENCE` `BP/26-27/0010`. It was created (`LASTVCHID` 45), and the build's read-back (`R-MC1.xml`) found it with the six legs as sent. That read-back was sent after the marks and the window's screenshot, not at once.
- Row 2346-MN's class reads were the retired one-number read (`template-n-0001.utf16le.xml`). It told credit from debit with `$$IsCreditNote` and `$$IsDebitNote` in its `COMPUTE`. The month read, which replaced it in the build (#2162 item 11), has carried only `$$IsSales`.
- Row 2162-S2 (`lab/2162-s2-answers` at 64f47017): a Sales invoice's read-back at once, at +5 s and at +60 s, was byte-identical, with included Yes, uncertain No and excluded No.

So N1 here is row 2346-MN's step 6 again, with a new key. Its new evidence is the read-back's timing (N4) and its `REFERENCE` and `REFERENCEDATE` against the original invoice. N3 is new.

**Not asked here:**
- a Debit Note (N2), and any number held in a second type;
- a `REFERENCE` the book does not hold (N5), or one dated in an earlier year (N6);
- GSTR-1's screen (N7);
- an `Agst Ref` or any bill allocation;
- a note built or posted by ComplyEaze Bridge.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2346-MN's) applies in full. Where it and this sheet differ, the stricter one holds, except for two exceptions: the screenshot's place in step 4, and the send line (below).

- **The book is pinned by GUID.** `count22.py` holds BRIDGE PILOT LAB's GUID as `BOOK`. Its M kind prints whether the COMPANY row's GUID is the book's. `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called. STOP before anything is sent unless s1's COMPANY row is the book's.
- **Its state:** row 2346-MN's restore left the marks at ALTVCHID 68 and ALTMSTID 245, and every row since has ended with a restore. Other marks at s1: stop before BK, with nothing sent.
- **The type:** no type named `BRIDGE Credit Note Manual` is on the restored book (row 2346-MN's s13 T read it 0). This row keys it again.
- **The predicted GUID.** Row 2346-MN's first key on this restored book took the GUID ending `000000e8` (MasterID 232), so this row's key is predicted to take it again.
  - `R-MC1.xml` names that GUID. `R-NCa.xml` names `000000ea`, which row 2346-MN's second key took.
  - Step 3's T read says which GUID the type took. The R file that names it is this row's read-back, `R` below.
  - **If it took neither:** that is not a stop. N4 runs on `V-CN.xml` in place of R, and the record says the build's read-back was not sent. Record the GUID the type took.
- **The number and key.**
  - The number is `CN/26-27/0001`, which the restored book does not hold: row 2346-MN's s13 V-CN read 0 rows. Step 2's V-CN read stays a STOP in case that is wrong.
  - The REMOTEID tail is `000000000e31`. On 11 Nov 2026 every blob on every `lab/` branch and on master (720 refs, 36,988 blobs, read as UTF-8 and UTF-16LE) was searched, and none holds `000000000e3`.
- **The date:** read the Gateway's current date at step 1. If it is before 1-Aug-2026, stop: the note would be post-dated, which is row 2162-S1's variable. Rows 2162-KD and 2346-CD read Tuesday 15-Sep-2026.
- **BK is this row's way back.**
  - It is taken at step 1, before anything is keyed or sent, and restored at step 6 by C6.
  - The restore runs after step 3 in every case, including a stop.
  - After it, the reads are compared with the reads before the change. Any part of that compare that is not equal stops the sitting.
- Write down the release shown on Tally's screen. Rows 2162-KD and 2346-MN read `tally.exe` 1.1.7.2.

**No Bridge build.** Raw requests, one at a time, through `run.py`, and the one import through `timed.py`. The build has no note type, so it cannot send these. The writes are one voucher type keyed on screen and the import `I-NC1.xml`, sent once.

**The sends:**
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py` and `timed.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count22.py`, which prints STOP and exits non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count22.py`, and copy its whole output into RESULTS.md.
- Leave any Bridge process on the lab PC running (C2: nothing in an item ends a process), and send nothing through it.
- **The send line is the second exception.** `run.py` and `timed.py` send with row 2346-MN's line (`curl.exe -sS --fail-with-body`, the header `Content-Type: text/xml; charset=utf-16`), not C3's (`-w "%{http_code} %{size_download}"` and `-D <headers file>`). So neither script checks the status 200 or the size against CONTENT-LENGTH: `--fail-with-body` fails on a status of 400 or above, and the count stops on an answer that does not parse. Rows 2346-MN and 2162-S2 ran this line. It needs the run coordinator's yes before the sitting, recorded in RESULTS.md's deviations.

## Files
**Byte-equal to row 2346-MN's file of the same name** (`lab/2346-mn-sheet` at cec5d5a8, folder `lab/2346-mn/`), whose ROW.md says where each came from:
- `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`;
- `T-types.xml`, the build's voucher-type read (sha256 `c351d6c6`, pinned by a test on master);
- `R-MC1.xml`, the build's read-back for `CN/26-27/0001` on the type GUID ending `000000e8` (sha256 `eccb528f`);
- `V-CN.xml`, every native field of the vouchers numbered `CN/26-27/0001` (sha256 `f113b399`);
- `template-I-MC1.utf16le.xml` is row 2346-MN's `I-MC1.xml` (sha256 `bcec0c46`), renamed so it is never sent. It was sent once in row 2346-MN.

**Byte-equal to row 2162-M1's file** (`lab/2162-m1-sheet` at eaf2a4fc): `W-08s.xml` is its `A-Y0-2026-08.xml`, the build's number read of August 2026 on this book (sha256 `b226ad52`). A test on master (`the_lab_month_reads_are_the_sent_ones_and_the_derived_august_drops_one_row`, `src-tauri/src/agent_import_tests.rs:2909`) checks that the build renders it byte for byte, against the sha256 `lab_number_months()` pins for August (`:2833`).

**`gen.py`** writes each request below, with only the change it names, each asserted to occur exactly as often as expected. Each template's sha256 is checked first. The import is parsed, and its type, its one number, its `REFERENCE` and `REFERENCEDATE`, its six legs, its party's side and the balance of its legs are asserted. Run twice, it writes the same bytes.

| File | From | Changed |
| --- | --- | --- |
| `I-NC1.xml` | `template-I-MC1.utf16le.xml` | the REMOTEID tail `000000000e31` in place of `000000000e11`, and nothing else |
| `R-NCa.xml` | `R-MC1.xml` | the type GUID ending `000000ea` in place of `000000e8` |
| `W-08c.xml` | `W-08s.xml` | the `COMPUTE` as `BRIDGEVCHISCREDITNOTE:$$IsCreditNote:$VoucherTypeName` |
| `W-08d.xml` | `W-08s.xml` | the `COMPUTE` as `BRIDGEVCHISDEBITNOTE:$$IsDebitNote:$VoucherTypeName` |

The two `COMPUTE` lines are the element names and the form `src-tauri/src/agent_voucher_type_class.rs:119` to `:124` writes for its class read, as row 2346-MN's N files carried them.

The note's party, figures, date (1 Aug 2026), `REFERENCE` (`BP/26-27/0010`, a Sales invoice the book holds, dated 1 Aug 2026) and legs are row 2346-MN's, unchanged. The party is `BRIDGE Walk-in`, which is not bill-wise, so the note carries no bill allocation.

**`timed.py`** sends the import once, then R at once, at +5 s and at +60 s, each counted from the moment curl returned the import's answer. It is row 2162-S2's `run.py` (`lab/2162-s2-sheet` at a0d4c53b), with row 2346-MN's send line and stops.
- A stop on the import ends the run, and the reads are not sent.
- **The import's answer is judged by C4 before any read:** every counter present once, `CREATED` 1, `ALTERED`, `DELETED`, `COMBINED`, `IGNORED`, `ERRORS`, `CANCELLED` and `EXCEPTIONS` 0, and no `LINEERROR`. Anything else stops it with the counters printed, and no read is sent.
- **It runs once.** It refuses to start, and sends nothing, if this label's import answer or log is already in the folder.
- **The +60 s read waits for the screenshot.** After the +5 s read it asks for Enter, pressed once the window's screenshot is taken and shows no dialog. The +60 s read goes at +60 s, or at the Enter if that is later, and the Enter's time goes into the times file.
- It rewrites `times-<label>.txt` after every send, and keeps it on Ctrl+C. It writes `log-<label>.txt`.
- With `--pause`, it waits for Enter after the import's answer and counts the three reads from that Enter, with no later wait. This row does not use it (step 4).
- Tried on 11 Nov 2026 against a stand-in for `curl.exe` that answers the import with row 2346-MN's captured import answer (`a-s6-I-MC1.xml` at 7b4b9ce5) and each read with its own request:
  - clean, with Enter given: the reads were sent at +0.001 s, +5.001 s and +60.001 s, and the Enter's line was written;
  - that answer with `CREATED` 0 and `EXCEPTIONS` 1, with a `LINEERROR` added, or with `IGNORED` removed: each stopped after the import, with no read sent;
  - a second run in the same folder: refused, nothing sent;
  - SIGINT, and no Enter to read, at the prompt: stopped with the two reads sent, and the times file held both and the interrupt.

**`count22.py`** is row 2346-MN's `count21.py` with two changes, each marked N134. Its header also says that count21's header below it is kept as it was.
- **W** (the build's month read) prints every child of the row whose tag begins `BRIDGEVCHIS`, in place of `BRIDGEVCHISSALES` alone, and then how many rows hold none. A row with none prints "class element absent". Tally omits the element of a function it does not answer, so that is a result of the read, never No and never 0 rows. `--guid` prints the same way.
- **`--w-diff A B`**, on two W answers of the same `COMPUTE`: each answer's row count, then, by GUID, each row in one answer only and each row whose printed fields differ, at most 300 such rows. Its last line is always `rows in one only or differing: <n>`. A GUID on two rows of one answer stops, and so does an answer with no VOUCHER row: August holds rows, so an empty answer is a read that failed, never a match.
- Tried once, each exit 0:
  - W on row 2162-M1's `a-a05a-A-Y0-2026-08.xml`: 40 rows, 0 with no class element.
  - W on a scratch copy of it with the element renamed to the credit class and one row's element removed: 1 "class element absent". That copy is not committed.
  - `--w-diff` of that answer with itself: 0 rows in one only or differing. `--guid` for `BP/26-27/0010`'s GUID: 1 row.
  - `--w-diff` of that answer against a copy with its `COLLECTION` emptied: STOP, exit 1.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count22.py a-<label>-*`.

**A STOP**, from any script or any line below, stops the row there, by C8: nothing further is sent, keyed, backed up or restored, and the run coordinator is told at once.
- Say what the book then holds: before step 3, nothing was changed; from step 3, whether the type was keyed; from step 4, whether the import was sent.
- Step 6's restore after a STOP is made only on the run coordinator's word (C8). A STOP before step 3 needs no restore.
- Steps 3 and 4 also name results that are not STOPs ("go to step 6"); those go on to step 6 without a word.

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named, and the voucher type screen's own save as step 3 says.
- Any other prompt, message or dialog is C7's: screenshot it and copy it word for word. Press NO key and send NO request while it is up. Report it at once; it stays on the screen until the run coordinator answers.
- That prompt is this row's result, and a STOP.

1. **Load, check and back up.** Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down. Before 1-Aug-2026: stop.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2346-n134\backup-1`. Record the file's name, size and sha256, and the time the Restore list will show.
2. **Before anything is keyed.** `s2`: `T-types.xml V-CN.xml R-MC1.xml R-NCa.xml W-08s.xml W-08c.xml W-08d.xml M-marks.xml`.
   - EXPECT T 28 rows, `BRIDGE Credit Note Manual` 0 rows, and neither predicted GUID held. EXPECT T's sha256 equal to row 2346-MN's s2 and s13 T answers.
   - EXPECT V-CN, R-MC1 and R-NCa 0 rows each.
   - **M-marks:** EXPECT ALTVCHID 68 and ALTMSTID 245, as s1.
   - **W-08s:** EXPECT 40 rows, each with its class element, as row 2162-M1's a05a answer. Run `count22.py --w-diff` against that answer, copied in from `lab/2162-m1-answers` at 1f250b86 as `a-m1-W-08s.xml`, and EXPECT its last line to read `rows in one only or differing: 0`.
   - A W answer with no row: STOP. The read failed; August holds 40.
   - **W-08c and W-08d:** EXPECT the same 40 rows, every one answering No to its class, and 0 with no class element.
   - A row in V-CN, the type name already held, or a predicted GUID already held: stop before step 3, with each recorded whole.
   - A W row with no class element, or a note class answering Yes: a result, not a stop. Record it whole. Steps 3 and 4 still run, and step 5 is judged against s2.
3. **Key the credit type.** This is the row's only master write. First `s3m`: `M-marks.xml`, then `python3 -I count22.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. Then, as row 2346-MN's step 3:
   - From the Gateway: Create, then Voucher Type. Write down the menu path and keys used.
   - Set only these four fields, and leave every other field as Tally fills it:
     - Name: `BRIDGE Credit Note Manual`;
     - Select type of voucher: `Credit Note`;
     - Method of Voucher Numbering: `Manual`;
     - Prevent creating duplicate Voucher Nos.: `Yes`. If Tally does not offer this field once Manual is chosen, stop before saving and ask.
   - Screenshot every part of the screen before saving, and write down every field it shows with its value, as text.
   - Save with Ctrl+A. If Tally asks "Accept ?", answer Yes; that is the master's own accept (C7). Any other prompt: the prompt rule.
   - If Tally then opens a new blank Voucher Type Creation screen, screenshot its title and leave it with Ctrl+Q (C7). Row 2346-MN left it by two Ctrl+Q presses, each after a capture of the screen's title, back to the Gateway by way of the Master Creation list. Screenshot the Gateway.
   - `s3r`: `T-types.xml M-marks.xml`. EXPECT one `BRIDGE Credit Note Manual` row: `PARENT` `Credit Note`, the GUID ending `000000e8`, one series `Default`, Manual, `PREVENTDUPLICATES` Yes. EXPECT ALTVCHID 68 and ALTMSTID 247. Run `count22.py --type-diff a-s2-T-types.xml a-s3r-T-types.xml` and record its lines.
   - **A series that does not read Manual and Yes is a result:** send no note, and go to step 6.
   - **The type's GUID** decides R for steps 4 and 5: `R-MC1.xml` for `000000e8`, `R-NCa.xml` for `000000ea`, and `V-CN.xml` for any other.
4. **Create the note, and read it back at once, at +5 s and at +60 s (N1, N4).**
   - `s4m`: `M-marks.xml`, then `count22.py --before-write a-s1-M-marks.xml a-s4m-M-marks.xml`. EXPECT ALTVCHID 68 and ALTMSTID 247, as s3r. Only if that passes, the marks are those, and no key was pressed in Tally since, go on.
   - **This step is the one exception to C3's screenshot** ("after every write ... take a screenshot of the whole Tally window BEFORE the next request"). The reads at once and at +5 s are the measurement, so they go before the screenshot. The +60 s read goes only after it.
     - **Granted:** the run coordinator said yes to this exception on 11 Nov 2026, before any write ("coordinator's yes, 11 Nov"). Record it in RESULTS.md's deviations in those words.
     - Run `python3 -I timed.py <gateway URL> s4 I-NC1.xml <R>` once. When it asks for Enter after its +5 s read, take the screenshot straight away, and press Enter only if it shows no dialog. The +60 s read is sent then, or at +60 s if that is later.
     - `timed.py` is run once in this row: it refuses a second run. Any later read goes through `run.py`.
     - `--pause` is not used in this row. It stays in `timed.py` for a row without this yes: it waits for Enter after the import, so the screenshot comes before the first read.
     - **A dialog in the screenshot:** press Ctrl+C in the window `timed.py` runs in, not Enter, then the prompt rule. The +60 s read is not sent. `times-s4.txt` keeps every send made, and the record says the two reads were sent before the screenshot showed the dialog.
   - **The wait rule.** If 5 minutes pass after the import's send with no answer and no message: send nothing, press nothing, screenshot the whole window, and report to the run coordinator. A read sent while Tally is busy can hold its gateway.
   - Record the import's every counter and any LINEERROR, word for word, and `times-s4.txt` whole.
   - EXPECT `CREATED` 1, `ERRORS` 0, `EXCEPTIONS` 0, `LASTVCHID` 45 (row 2346-MN's s6).
   - **N1. EXPECT `a-s4t00-<R>` to hold one row:**
     - `VOUCHERNUMBER` `CN/26-27/0001`, `VOUCHERTYPENAME` `BRIDGE Credit Note Manual`, `DATE` 20260801;
     - `REFERENCE` `BP/26-27/0010` and `REFERENCEDATE` 20260801, as sent: the original invoice, not the note's number;
     - the six legs as sent: `BRIDGE Walk-in` 1051.00 on the credit side, and the five others as `I-NC1.xml` holds them;
     - the GUID the book's GUID followed by `0000002d`.
   - **N4.** Run `count22.py --diff a-s4t00-<R> a-s4t60-<R>` and `--diff a-s4t05-<R> a-s4t60-<R>`, and record each answer's sha256. EXPECT the three byte-equal, each reading `VCHGSTSTATUSISINCLUDED` Yes, `VCHGSTSTATUSISUNCERTAIN` No, `VCHGSTSTATUSISEXCLUDED` No, `ISPOSTDATED` No, `ISCANCELLED` No and `ISOPTIONAL` No. Row 2346-CD's note read as included.
   - **An empty first read** means the filter did not find the note. Record the number and type the later reads and V-CN show. That is not a finding about the timing.
   - `s4r`: `V-CN.xml M-marks.xml`. EXPECT V-CN one row with the same number and type, `MASTERID` 45 and `REFERENCE` `BP/26-27/0010`.
   - **A decline is a result:** `timed.py` stops after the import without a read when the counters are not CREATED 1 and the rest 0, or a LINEERROR is present. Record its counters and every LINEERROR, send nothing more, and go to step 6.
   - **If R is `V-CN.xml`** (the type took neither predicted GUID): its answer holds no legs and no `ISPOSTDATED`, `ISCANCELLED` or `ISOPTIONAL`, so N1's legs and N4's flags are recorded as not judged.
5. **The month read with each class (N3), read only.** `s5`: `W-08s.xml W-08c.xml W-08d.xml`. Then `count22.py --w-diff a-s2-W-08<x>.xml a-s5-W-08<x>.xml` for each of s, c and d, and `count22.py --guid <the note's GUID> a-s5-W-08s.xml a-s5-W-08c.xml a-s5-W-08d.xml`, with the GUID written whole: `6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d`, or the GUID s4's read-back gave if it differs. EXPECT:

   | Read | Rows | `--w-diff` against s2 | The note's row |
   | --- | --- | --- | --- |
   | W-08s, the build's read | 41 | one row, only in s5: the note's GUID | `BRIDGEVCHISSALES` No |
   | W-08c | 41 | the same one row | `BRIDGEVCHISCREDITNOTE` Yes |
   | W-08d | 41 | the same one row | `BRIDGEVCHISDEBITNOTE` No |

   - In each, every other row reads as at s2, so `--w-diff` prints no other row, and its last line reads `rows in one only or differing: 1`.
   - Record every line whole. A row with no class element, or a class that is not Yes or No, is a result, not a stop.
   - If W-08s's 40 earlier rows do not read as at s2, record it: the read itself moved in this sitting, and nothing in step 5 is judged.
6. **Restore** BK by C6, with `restore-plan.txt` written first. Row 2346-MN's step 13 is the path:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (r, Replace);
   - load the company alone.

   Then `s6`: `M-marks.xml T-types.xml V-CN.xml W-08s.xml`, and compare:
   - `count22.py --same-row a-s1-M-marks.xml a-s6-M-marks.xml`: EXPECT it passes.
   - T: EXPECT its sha256 equal to s2's, and `BRIDGE Credit Note Manual` 0 rows.
   - V-CN: 0 rows. W-08s: `--w-diff` against s2 ends `rows in one only or differing: 0`, and its sha256 equals s2's.
   - **Any of these not equal stops the sitting:** send nothing more, and report before the close. The book is then not as it was, and the run coordinator decides.
7. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule, step 3 and C6 name, or if the gateway stops answering.
- Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- Say in the report what the book then holds: whether the type was keyed, and whether the note was made.
- A dialog after the import makes it a failure inside Tally's screen, not in its answer (P7). That decides the design before any count does.

## The independent reader
The answers are read twice. The second reading is done by someone who did not run the sitting and did not write this sheet. They work only from the pushed answers branch, before any result is posted on #2346:
1. Check each answer's sha256 and size against the notes.
2. Run `count22.py` again on every answer, and each `--diff`, `--w-diff`, `--guid`, `--type-diff`, `--before-write` and `--same-row` line the steps name. Compare each output with RESULTS.md line by line.
3. From `times-s4.txt`, check that the first read was sent within 1 s of the import's answer, the second at 5 s or later and the third at 60 s or later. Check that its Enter line falls after the second read's return and before the third read's send, and that the screenshot's time is before the Enter.
4. Check each EXPECT line against the answers, not against RESULTS.md's wording.

They write `READER.md` on the answers branch, with each check, whether it held, and every line that differs. A result is reported only once `READER.md` says every check held, or names each difference and how RESULTS.md answers it.

## Record
- Each answer's bytes and sha256, and `count22.py`'s lines.
- `run.py`'s and `timed.py`'s logs for each label, and `times-s4.txt`.
- The `--diff`, `--w-diff`, `--guid`, `--type-diff`, `--before-write` and `--same-row` outputs, whole.
- Step 3's screens: every field and value as text, and the screenshots by name, bytes and sha256.
- The window after the import (shot), by name, bytes and sha256, and when it was taken against `times-s4.txt`.
- Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.
- Step 4's exception, as granted: "coordinator's yes, 11 Nov".

## Expected
Estimates, not predictions.
- **s2:** as row 2346-MN's s2 for T and V-CN, and as row 2162-M1's a05a for W-08s. W-08c and W-08d No on every row.
- **s3r:** as row 2346-MN's s3r: the GUID ending `000000e8`, one series, Default, Manual, Yes.
- **s4:** created, as row 2346-MN's s6. The three read-backs byte-equal, as row 2162-S2's were for Sales.
- **s5:** as the table. Row 2346-MN measured these functions answering only within their class in the one-number read.
- **s6:** equal to s1 and s2.

## Decides
Each is PARTIAL: one sitting, one book, one release.

**N1 (s4's first read)**
- **One row, with `REFERENCE` and `REFERENCEDATE` as sent:** the build's read-back finds a note by its number in its type, and can compare the original invoice, not the note's own number. Design A's read-back comparison can be built on this request unchanged.
- **`REFERENCE` stored otherwise:** the note path cannot reuse the read-back's comparison of `REFERENCE`. Record what Tally stored.

**N4 (the three reads)**
- **Byte-equal:** the read-back straight after a note's post reads what Tally keeps, as for Sales. The note path reads without a wait.
- **The first differs from the third:** the note path must wait or re-read. Whether the second equals the third shows how long. The fields that differed go to the run coordinator before any code.

**N3 (s5)**
- **As the table:** the build's month read tells a credit note apart inside its `COMPUTE`, with no other change. Design A's number reads can carry the note class's compute, chosen in code from the class (`InvoiceClass`).
- **A note class answers Yes on a row of another class:** the function does not discriminate in this read, and the number reads cannot rest on it.
- **A class element absent:** the function does not answer inside the month read's `COMPUTE`. The build would then read each such row as `invoice_number_class_unread`, which is loud. The class read needs another shape, and that is its own row.

**What these answers are for.** They inform #2346's Design A. They are never a regression fixture for the build's output, because the build renders no note. Once a candidate builder renders these requests, its tests may pin them against this row's request files.

## Code admits the trigger
Lines are at master 7a3c9e49.
- **The build has no note type.** `VoucherType` (`src-tauri/src/agent_import.rs:331`) has Payment, Receipt, Journal, Contra and Sales. So no build sends a note, and this row sends it raw.
- **The flag the build reads.** `resolve_voucher_type` (`src-tauri/src/agent_import_invoice_wire.rs:334`) refuses a type whose one series is not Manual with prevent-duplicates Yes (the match at `:390` to `:398`). T is its request, byte-equal.
- **The month read.** `render_invoice_window_numbers_request` (`:467`) computes `BRIDGEVCHISSALES` from `$$IsSales` (`:480`). The build sends it for each month of the invoice's year (`src-tauri/src/agent_import_invoice.rs:2223`), for the month of the number's control invoice when that is dated outside the year (`:2273`), and again for each month after a post (`:1821`). W-08s is that request for August 2026. W-08c and W-08d change only its `COMPUTE`.
- **The read-back.** R-MC1 and R-NCa are the build's read-back (`render_invoice_readback_request`, `agent_import_invoice_wire.rs:701`), with the number and the type's GUID as the build would fill them from the batch and the resolved type. The post's verification sends it straight after the import (`verify_import_after_current_dispatch`, `src-tauri/src/agent_import.rs:2469`, calling `read_back_sales_invoice` at `:2694`), after the marks, masters and window reads, with no wait in between.

## Not established after this row
- A Debit Note (N2), a `REFERENCE` the book does not hold (N5), an earlier-year `REFERENCEDATE` (N6) and GSTR-1's screen (N7).
- Whether the uniqueness domain for tax is the note's own class. That is a decision on unverified recall, not a measurement.
- How a note appears on the return, invoice-mode note entry, and `GSTNATUREOFRETURN`.
- A note type created over XML rather than keyed.
- A note built and posted through ComplyEaze Bridge.
- Any other book or release, and the multi-user edition.

Row: lab/2346-n134/
