# Row 2346-MN: does a Manual Credit or Debit Note type keyed today keep the number sent, decline it sent again, and which class does the number read see it in?

Shape discovery: the notes are requests written for this row, not requests the build renders, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2346 (0.9.0): the measurement behind the note-numbering decision (option 2, a number sent into a Manual note type). Written 10 Nov 2026. Three voucher types keyed on screen, then six notes, each sent once, and read-only reads between them; then a restore from the row's own backup. Confidence of everything here until the row runs: unverified.

## Question
The build admits a Sales type only when its one series reads Manual with "prevent duplicates" Yes (`resolve_voucher_type`, `src-tauri/src/agent_import_invoice_wire.rs:384` to `:390`), and its number read counts Sales-class rows only (`BRIDGEVCHISSALES`, `:436` to `:445`). #2346's design applies the same rule to a note: the note is sent with a number, into a Manual note type, and its number is checked against notes of its own class (credit with credit, debit with debit).
- **Measured so far, on Sales only:** a keyed Manual Sales type with "Prevent creating duplicate Voucher Nos." Yes kept the number sent and declined it sent again under a new key, in the answer: `CREATED` 0, `EXCEPTIONS` 1, no `LINEERROR`, no dialog (row 2162-KD, `lab/2162-keyed-dup-answers` at b9886b6a; PARTIAL).
- **Measured on notes:** row 2346-CD (`lab/2346-cd-answers` at fcdd4356) sent a Credit Note and a Debit Note with no number into the predefined types, and each was created as `1`. Row 2162-KD's type read (`answers/a-s2-T-types.xml`) shows both predefined note types Automatic, `Auto Retain`, `PREVENTDUPLICATES` No.
- **The class functions are measured, but not inside the number read.** `$$IsCreditNote` and `$$IsDebitNote` answered Yes only within their own class, across all eight classes, in the class-resolving read (protocol reference, reads and dates, §8.2e, verified 2026-09-24). The number read's `COMPUTE` has only ever carried `$$IsSales`.

**This row asks, one variable a step:**
1. **Series.** What the two predefined note types' series read in this sitting (method, sub-method, "prevent duplicates").
2. **Key.** Whether a credit type and a debit type keyed Manual with "Prevent creating duplicate Voucher Nos." Yes read as the build checks, and a second credit type the same.
3. **Create.** Whether each Manual type keeps the number sent, on the credit side and on the debit side.
4. **Read.** Whether the build's read-back finds each note by its number and its type's GUID.
5. **Two resends.**
   - The same request again, byte for byte: the same key and the same number. Does Tally re-state the note, decline it, or make a second one?
   - The same number under a new key into the same type: declined in the answer, as on Sales?
6. **Class reads.** Whether the number read, with the note's class function in its `COMPUTE`, tells the classes apart:
   - a credit number with the credit class (Yes expected) and with the debit class (No);
   - a debit number with the debit class (Yes) and with the credit class (No);
   - a Sales invoice's number with each note class (No), beside the build's own Sales read (Yes).
7. **The guard per type and across types.**
   - The credit number into a second Manual credit type: does Tally's guard span types within the class?
   - The credit number into the Manual debit type: does it span classes?

**Not asked here:**
- an Agst Ref or any bill allocation (row 2346-AR's question);
- a numberless post into a Manual type with duplicates allowed;
- the GUID read-back (option 3's shape);
- renumbering of a note, a back-dated note, a type with more than one series;
- `GSTNATUREOFRETURN`, invoice mode, the return, a type created over XML.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2346-AR's and 2346-CD's) applies in full. Where it and this sheet differ, the stricter one holds, except for the one exception step 8 names.

- **When it runs:** in row 2346-AR's sitting, after AR's step 8 compare passes, as a row of its own with its own BK. If AR's compare fails, or AR stops, this row does not run in that sitting.
- **The book is pinned by GUID.** `count21.py` holds BRIDGE PILOT LAB's GUID as `BOOK`.
  - Its M kind prints whether the COMPANY row's GUID is the book's.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is sent unless s1's COMPANY row is the book's.
- **Its state:** every row queued before this one ends with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245. Other marks at s1: stop before BK, with nothing sent.
- **The types:**
  - No type named `BRIDGE Credit Note Manual`, `BRIDGE Debit Note Manual` or `BRIDGE Credit Note Manual 2` is in any file on any `lab/` branch or on master.
  - Row 2162-KD's s2 type read of the restored book holds 28 types. No type's GUID is the book's GUID followed by `000000e8`, `000000e9` or `000000ea`.
- **The predicted GUIDs.** Row 2162-KD's keyed type took the GUID ending `000000e8` (MasterID 232) on the restored book. So the three types keyed here are predicted to take `000000e8`, `000000e9` and `000000ea`, in the order keyed.
  - Each R request names its type by that predicted GUID, as the build's read-back does.
  - `count21.py`'s T kind prints which type holds each predicted GUID. Each prediction is checked at the read after its key.
  - **A prediction that fails is not a stop.** That type's R requests are not sent, and its V reads stand in their place. Record which GUID it took.
- **The numbers and keys:**
  - The numbers are `CN/26-27/0001` and `DN/26-27/0001`.
  - Every blob on every `lab/` branch (658 refs, 23,634 blobs, read as UTF-16 and as UTF-8) and on master was searched on 10 Nov 2026. None holds `CN/26-27/` or `DN/26-27/`, the REMOTEID tails `000000000e10` to `000000000e19`, or this row's type names.
  - Step 2's V reads stay a STOP in case this check is wrong.
- **The date:** read the Gateway's current date at step 1. If it is before 1-Aug-2026, stop: the notes would be post-dated, which is row 2162-S1's variable. Rows 2162-KD and 2346-CD read Tuesday 15-Sep-2026.
- **BK is this row's way back.**
  - It is taken at step 1, before anything is keyed or sent, and restored at step 13 by C6.
  - The restore runs after step 3 in every case, including a stop.
  - After it, the reads are compared with the reads before the change. Any part of that compare that is not equal stops the sitting.
- Write down the release shown on Tally's screen. Row 2162-KD read `tally.exe` 1.1.7.2.

**No Bridge build.** Raw requests, one at a time, through `run.py`. The build has no note type, so it cannot send these. The writes are three voucher types keyed on screen and the six imports `I-MC1.xml`, `I-MD1.xml`, `I-MC1-again.xml`, `I-MC1n.xml`, `I-MC2.xml` and `I-MDX.xml`, each sent at most once.

**The sends** are row 2346-AR's, unchanged.
- Each of these stops the run and is reported as "the request failed", never as "nothing found":
  - a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes (all four in `run.py`);
  - a collection answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse (`count21.py`, which prints STOP and exits non-zero).
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with `count21.py`, and copy its whole output into RESULTS.md.
- Close every other Bridge process first.

## Files
**Copied from earlier rows**
- **Byte-equal to row 2346-AR's file of the same name** (`lab/2346-ar-sheet` at fd0aef60, folder `lab/2346-ar/`), whose ROW.md says where each came from:
  - `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`;
  - `template-n-0001.utf16le.xml`, the build's number read;
  - `I-CN.xml`, row 2346-CD's Credit Note (sha256 `705a9c57`);
  - `R-CN1.xml`, the build's read-back for the number `1` on the type `Credit Note` (sha256 `448064a4`).
- **Byte-equal to row 2346-CD's file of the same name** (`lab/2346-cd-sheet` at 9c2229cc, folder `lab/2346-cd/`):
  - `I-DN.xml`, its Debit Note (sha256 `9f753e4c`);
  - `R-DN1.xml`, the build's read-back for `1` on the type `Debit Note` (sha256 `aacdf011`);
  - `V-1.xml`, every native field of the vouchers numbered `1` (sha256 `ea8a66b7`).
- **Byte-equal to row 2162-KD's file** (`lab/2162-keyed-dup-sheet` at c5b8e06e): `T-types.xml`, the build's voucher-type read (`render_voucher_types_request`, `agent_import_invoice_wire.rs:267`; sha256 `c351d6c6`, pinned by a test on master).

**`gen.py`** writes each request below, with only the change it names, each asserted to occur exactly as often as expected. Each template's sha256 is checked first. Each import is parsed, and its type, its one number right after the type's name, its six legs and its party's side are asserted.

| File | From | Changed |
| --- | --- | --- |
| `I-MC1.xml` | `I-CN.xml` | the type `BRIDGE Credit Note Manual` (in `VCHTYPE` and `VOUCHERTYPENAME`); `<VOUCHERNUMBER>CN/26-27/0001</VOUCHERNUMBER>` after `VOUCHERTYPENAME`, where the build's Sales request places its number; the tail `000000000e11` |
| `I-MC1-again.xml` | `I-MC1.xml` | nothing: byte-equal (asserted) |
| `I-MC1n.xml` | `I-MC1.xml` | the tail `000000000e12` |
| `I-MD1.xml` | `I-DN.xml` | the type `BRIDGE Debit Note Manual`; `DN/26-27/0001`, placed as in `I-MC1.xml`; the tail `000000000e13` |
| `I-MC2.xml` | `I-MC1.xml` | the type `BRIDGE Credit Note Manual 2`; the tail `000000000e14` |
| `I-MDX.xml` | `I-MD1.xml` | the number `CN/26-27/0001`; the tail `000000000e15` |
| `R-MC1.xml` | `R-CN1.xml` | the number `CN/26-27/0001`; the type GUID ending `000000e8` |
| `R-MD1.xml` | `R-DN1.xml` | `DN/26-27/0001`; the GUID ending `000000e9` |
| `R-MC2.xml` | `R-CN1.xml` | `CN/26-27/0001`; the GUID ending `000000ea` |
| `R-MDX.xml` | `R-DN1.xml` | `CN/26-27/0001`; the GUID ending `000000e9` |
| `V-CN.xml`, `V-DN.xml` | `V-1.xml` | the number `CN/26-27/0001`, and `DN/26-27/0001` |
| `N-0010s.xml` | the number read | the number `BP/26-27/0010`, which the book holds on `BRIDGE Sales`. The `COMPUTE` is unchanged, so this is the build's read |
| `N-0010c.xml`, `N-0010d.xml`, `N-CNc.xml`, `N-CNd.xml`, `N-DNc.xml`, `N-DNd.xml` | the number read | the number, and the `COMPUTE` as `BRIDGEVCHISCREDITNOTE:$$IsCreditNote:$VoucherTypeName` (c) or `BRIDGEVCHISDEBITNOTE:$$IsDebitNote:$VoucherTypeName` (d). These are the element names and the form `src-tauri/src/agent_voucher_type_class.rs:74` to `:121` writes for its class read |

The parties, figures, dates (1 Aug 2026), `REFERENCE` and legs are row 2346-CD's, unchanged. The party is `BRIDGE Walk-in`, which is not bill-wise, so no note carries a bill allocation.

**`count21.py`** is row 2346-AR's `count20.py` with three changes, each marked MN.
- **N** also prints each row's `VOUCHERTYPENAME` and every child whose tag begins `BRIDGEVCHIS`, then how many rows hold none. A row with none prints "class element absent". Tally omits the element of a function it does not answer, so that is a result of the read, never No and never 0 rows.
- **T**'s default names are the five note types above, and each series also prints `NUMBERINGSUBMETHOD`.
- **T** also prints which row, if any, holds each predicted GUID.
- Tried once, each exit 0:
  - T on row 2162-KD's s2 and s3r: 28 and 29 rows. `Credit Note` and `Debit Note` each Default, Automatic, `Auto Retain`, `PREVENTDUPLICATES` No. The GUID ending `000000e8` was held by none at s2, and by `BRIDGE Sales Keyed` at s3r.
  - N on row 2162-KD's s4r (one row, `BRIDGE Sales Keyed`, `BRIDGEVCHISSALES` Yes, 0 rows with no class element). Also on a scratch copy of it with that element removed, which printed "class element absent" and 1. That copy is not committed.
  - R on row 2346-CD's s2 (0 rows) and s3r (one note, six legs), and I on its s3.
  - `--same-row` on row 2162-KD's s1 and s9 (equal).

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each read as `python3 -I run.py <gateway URL> <label> <files>`, then `python3 -I count21.py a-<label>-*`. A STOP from either script stops the row there, and the row goes to step 13.

**Before each import** (steps 6 to 9, 11 and 12):
- `run.py URL s<n>m M-marks.xml`, then `python3 -I count21.py --before-write a-s1-M-marks.xml a-s<n>m-M-marks.xml`.
- Only if that passes, and no key was pressed in Tally since, send the import alone: `run.py URL s<n> <file>`.
- Then screenshot the whole window before the next request (C3).

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named, and a voucher type screen's own save as step 3 says.
- For any other prompt, message or dialog: copy it word for word and screenshot it first.
- Then answer No, or Esc if it offers no No. Press nothing more and send nothing.
- Then restore BK by C6, compare as step 13, and close (C10).
- That prompt is this row's result.

**The wait rule.** After each import's send: if 5 minutes pass with no answer and no message:
- send nothing, press nothing, screenshot the whole window, and report to the run coordinator;
- a read sent while Tally is busy can hold its gateway.
- With the Gateway showing and no dialog, run C2's status check (`GET /status` 200, one tally.exe owning 9001) before the next read.

**Each import's answer** is one of these:
- `CREATED` 1 with `ERRORS` 0;
- `CREATED` 0 with `ALTERED` 1 and `ERRORS` 0;
- `CREATED` 0 with `ERRORS` 1 and a LINEERROR;
- `CREATED` 0 with `EXCEPTIONS` 1.

Any other answer, or a dialog after it: the prompt rule.

1. **Load, check and back up.** If this row follows row 2346-AR in its sitting, the book is already loaded alone after AR's step 8 compare. Go on from s1 without closing.
   - Otherwise: step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down. Before 1-Aug-2026: stop.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2346-mn\backup-1` (Tally may show it as `C:\lab\2346-Mn\backup-1`). Record the file's name, size and sha256, and the time the Restore list will show.
2. **Series, before anything is keyed.** `s2`: `T-types.xml V-CN.xml V-DN.xml R-MC1.xml R-MD1.xml M-marks.xml`.
   - **The baseline:** record each predefined note type's `PARENT`, `GUID`, `NUMBERINGMETHOD`, and each series with `NAME`, `NUMBERINGMETHOD`, `NUMBERINGSUBMETHOD` and `PREVENTDUPLICATES`. EXPECT them as row 2162-KD's s2 read them.
   - EXPECT the three keyed names 0 rows each, and each predicted GUID held by none.
   - EXPECT V-CN, V-DN, R-MC1 and R-MD1 0 rows each.
   - A row in V-CN or V-DN, a keyed name already held, or a predicted GUID already held: stop before step 3, with each recorded whole.
3. **Key the credit type.** Steps 3, 4 and 5 key one type each, by this procedure, in the table's order. They are the row's only master writes. Before each, `s<n>m` and `--before-write` as before an import. Then:
   - From the Gateway: Create, then Voucher Type. Write down the menu path and keys used.
   - Set only these four fields, and leave every other field as Tally fills it:
     - Name, as the table says;
     - Select type of voucher, as the table says;
     - Method of Voucher Numbering: `Manual`;
     - Prevent creating duplicate Voucher Nos.: `Yes`. If Tally does not offer this field once Manual is chosen, stop before saving and ask.
   - Screenshot every part of the screen before saving, and write down every field it shows with its value, as text.
   - Save with Ctrl+A. Row 2162-KD's save showed no prompt. If Tally asks "Accept ?", answer Yes; that is the master's own accept (C7). Any other prompt: the prompt rule.
   - If Tally then opens a new blank Voucher Type Creation screen, screenshot its title and leave it with Ctrl+Q (C7). Row 2162-KD took two Ctrl+Q presses, each after a capture of the screen's title, back to the Gateway by way of the Master Creation list. Screenshot the Gateway.
   - Then the read the table names.

   | Step | Name | Select type of voucher | Read after it | EXPECT in T |
   | --- | --- | --- | --- | --- |
   | **3** (`s3m`, then `s3r`) | `BRIDGE Credit Note Manual` | `Credit Note` | `T-types.xml M-marks.xml` | one row, `PARENT` `Credit Note`, GUID ending `000000e8`, one series, Manual, `PREVENTDUPLICATES` Yes |
   | **4** (`s4m`, then `s4r`) | `BRIDGE Debit Note Manual` | `Debit Note` | `T-types.xml M-marks.xml` | one row, `PARENT` `Debit Note`, GUID ending `000000e9`, one series, Manual, Yes |
   | **5** (`s5m`, then `s5r`) | `BRIDGE Credit Note Manual 2` | `Credit Note` | `T-types.xml M-marks.xml` | one row, `PARENT` `Credit Note`, GUID ending `000000ea`, one series, Manual, Yes |

   - In each M, EXPECT ALTVCHID 68, and ALTMSTID above the last read. Row 2162-KD's one keyed type raised it by 2.
   - In each T, EXPECT the types read before it unchanged. Run `count21.py --type-diff` against the last earlier T and record its lines.
   - **A series that does not read Manual and Yes is a result:** the screen's setting is not the flag the build reads on a note type. Record it.
     - If it is step 3's, send no note, and go to step 13.
     - If it is step 4's, skip steps 7 and 12.
     - If it is step 5's, skip step 11.
   - **A type that took another GUID:** record the GUID, and leave out each R file that names the predicted one (step 6 onward). Its V reads stand.
4. **Key the debit type** by step 3's procedure, on its table's second row (`s4m`, then `s4r`).
5. **Key the second credit type** by step 3's procedure, on its table's third row (`s5m`, then `s5r`).
6. **Create a credit note.** `s6m`, the check, then `s6`: `I-MC1.xml`, sent once. Record every counter and any LINEERROR word for word, and the window after it (shot).
   - `s6r`: `R-MC1.xml V-CN.xml M-marks.xml`.
   - EXPECT `CREATED` 1, `LASTVCHID` 45 (row 2162-KD's first voucher on this book took MasterID 45).
   - EXPECT R-MC1 one row: `VOUCHERNUMBER` `CN/26-27/0001`, `VOUCHERTYPENAME` `BRIDGE Credit Note Manual`, the six legs as sent.
   - EXPECT V-CN one row with the same number and type, its `MASTERID` the import's `LASTVCHID` and its GUID the book's GUID followed by that MasterID in eight hex digits.
   - **Another number or type in V-CN is a result:** record both. Steps 8 and 9 still run.
   - **A decline is a result:** record its counters and every LINEERROR, send nothing more into this type, and go to step 7.
7. **Create a debit note.** `s7m`, the check, then `s7`: `I-MD1.xml`, sent once. Record as step 6.
   - `s7r`: `R-MD1.xml V-DN.xml V-CN.xml M-marks.xml`.
   - EXPECT `CREATED` 1 and R-MD1 one row with `DN/26-27/0001` and `BRIDGE Debit Note Manual`; V-DN one row; V-CN as at s6r.
8. **Resend under the same key.** `s8m`, the check, then `s8`: `I-MC1-again.xml`, sent once.
   - **This step is the one exception to C3** ("a file that writes is sent once in the item's life"). `I-MC1-again.xml` is a second file, byte-equal to `I-MC1.xml`, because the question is what Tally does with the same key and the same number.
   - It is sent only if the run coordinator said yes to this exception when queuing the row. Without that yes, skip step 8 and record it as not measured.
   - It is skipped if step 6 did not create the note.
   - Record as step 6. `s8r`: `R-MC1.xml V-CN.xml M-marks.xml`, then `count21.py --diff a-s6r-R-MC1.xml a-s8r-R-MC1.xml`.
   - Not estimated. The nearest measurement: a byte-identical re-import under the same `REMOTEID` upserted, `CREATED` 0 and `ALTERED` 1, one voucher afterwards (`docs/tally/IMPLEMENTATION_GUIDE.md` §3.3a, verified 2026-07-30; the protocol reference's §9.14 for Journal upserts). No note type has been measured.
   - Record how many V-CN rows there are, and each one's GUID, MASTERID and ALTERID against s6r's.
9. **Resend under a new key.** `s9m`, the check, then `s9`: `I-MC1n.xml`, sent once. Skipped if step 6 did not create the note.
   - Record as step 6. `s9r`: `R-MC1.xml V-CN.xml M-marks.xml`.
   - EXPECT `CREATED` 0, `EXCEPTIONS` 1, every other counter 0, no LINEERROR, no dialog, as row 2162-KD's s5 on Sales.
   - EXPECT V-CN as at s8r, and `count21.py --same-row a-s8r-M-marks.xml a-s9r-M-marks.xml` to pass. If step 8 was skipped, use s7r's marks.
   - **`CREATED` 1 is a result:** the guard did not hold on a note type. Record V-CN's rows, all of them.
10. **Class reads, read only.** `s10`: `N-0010s.xml N-0010c.xml N-0010d.xml N-CNc.xml N-CNd.xml N-DNc.xml N-DNd.xml`. EXPECT:

    | Read | Rows | Class element on each row |
    | --- | --- | --- |
    | N-0010s | one or more, each `BP/26-27/0010` | `BRIDGEVCHISSALES` Yes on the `BRIDGE Sales` row (the build's read, the control) |
    | N-0010c, N-0010d | the same rows as N-0010s | `BRIDGEVCHISCREDITNOTE` No, and `BRIDGEVCHISDEBITNOTE` No |
    | N-CNc | as many as V-CN held at s9r | `BRIDGEVCHISCREDITNOTE` Yes |
    | N-CNd | the same rows as N-CNc | `BRIDGEVCHISDEBITNOTE` No |
    | N-DNd | as many as V-DN held at s7r | `BRIDGEVCHISDEBITNOTE` Yes |
    | N-DNc | the same rows as N-DNd | `BRIDGEVCHISCREDITNOTE` No |

    - Record every row whole. A row whose class element is absent, or whose class is not Yes or No, is a result, not a stop.
    - If N-0010s's control row is not Yes, record it: the read itself failed in this sitting, and nothing in step 10 is judged.
11. **The same number into a second credit type.** `s11m`, the check, then `s11`: `I-MC2.xml`, sent once. Skipped if step 5 did not read Manual and Yes, or step 6 did not create the note.
    - Record as step 6. `s11r`: `R-MC2.xml R-MC1.xml V-CN.xml N-CNc.xml M-marks.xml`.
    - Not estimated. That is the question.
12. **The credit number into the debit type.** `s12m`, the check, then `s12`: `I-MDX.xml`, sent once. Skipped if step 4 did not read Manual and Yes, or step 6 did not create the note.
    - Record as step 6. `s12r`: `R-MDX.xml R-MD1.xml V-CN.xml N-CNc.xml N-CNd.xml M-marks.xml`.
    - Not estimated.
13. **Restore** BK by C6, with `restore-plan.txt` written first. Row 2162-KD's step 6 is the path:
    - shut the company;
    - restore BK's folder alone;
    - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (r, Replace);
    - load the company alone.

    Then `s13`: `M-marks.xml T-types.xml V-CN.xml V-DN.xml`, and compare:
    - `count21.py --same-row a-s1-M-marks.xml a-s13-M-marks.xml`: EXPECT it passes.
    - T: EXPECT its sha256 equal to s2's, and the three keyed names 0. Row 2162-KD's T after its restore was byte-equal to its s2.
    - V-CN and V-DN: 0 rows. Record each answer's sha256 against s2's.
    - **Any of these not equal stops the sitting:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
14. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule, step 3 and C6 name, or if the gateway stops answering.
- Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- Say in the report what the book then holds: which types were keyed, and which notes each import made.
- A dialog after an import makes that guard a failure inside Tally's screen, not in its answer (P7). That decides the design before any count does.

## Record
- Each answer's bytes and sha256, and `count21.py`'s lines.
- `run.py`'s log for each label.
- The `--diff`, `--type-diff`, `--before-write` and `--same-row` outputs, whole.
- Step 3's screens: every field and value as text, and the screenshots by name, bytes and sha256.
- The window after each import (shot), listed by name, bytes and sha256.
- Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions.
- **s2:** both predefined note types Automatic, `Auto Retain`, `PREVENTDUPLICATES` No, as row 2162-KD read them.
- **s3r to s5r:** each type reads as row 2162-KD's keyed Sales type did: one series, Default, Manual, Yes.
- **s6 and s7:** created, number and type as sent.
- **s9:** declined in the answer, with no dialog.
- **s10:** as the table says. §8.2e measured these functions answering only within their class in another read.
- **s8, s11 and s12:** not estimated.
- **s13:** equal to s1 and s2.

## Decides
Each is PARTIAL: one sitting, one book, one release.

**Series and keying (s2 to s5r)**
- **s2 as row 2162-KD's:** the pilot's note types are Automatic `Auto Retain` from two sittings' series reads. A first-time user's note types fail option 2's check, so the set-up and its refusal text are needed.
- **s3r and s4r read Manual and Yes:** a note type keyed so carries the flag the build reads. The resolver can admit a credit type and a debit type by the Sales rule, with the class in place of Sales.
- **Anything else:** the note path needs its own check before it is built. Report what T shows.

**Create and read (s6, s7)**
- **Stored as sent and found by the read-back:** the build's read-back, with the note type's GUID, finds a note by the number the person gave. Option 2's create path stands on both sides.
- **Another number stored:** a Manual note type does not keep the number sent. The read-back must compare the stored number before any post is called verified.

**Resends (s8, s9)**
- **s9 declined in the answer:** the guard holds on a note type within the type, in-band. Option 2's resend failure is the one P7 prefers.
- **s9 created:** the guard did not hold. Notes may not be built on the flag as their guard.
- **s8 re-stated** (`ALTERED` 1, one note, GUID unchanged): a same-key resend is safe on a note type, as §3.3a found for a voucher.
- **s8 declined:** the number guard acts before the key is matched.
- **s8 created a second note:** a same-key resend makes a duplicate on a note type. Record it whole, and the coordinator decides.

**Class reads (s10)**
- **As the table:** the number read tells the classes apart inside its `COMPUTE`. Condition 1's class read can be built per class, credit with credit and debit with debit.
- **A negative reads Yes:** the function does not discriminate in this read. The class read cannot rest on it.
- **A class element absent:** the function does not answer inside the number read's `COMPUTE`. The build would then read every such row as `invoice_number_class_unread`, which is loud. The class read needs another shape, and that is its own row.

**The guard across types (s11, s12)**
- **s11 declined:** Tally's guard spans Manual credit types in this book. The build's class read is a second guard, not the only one.
- **s11 created:** the guard is per type. Between two Manual credit types, the class read before the post is the only guard, and it is a read, so a post between the read and the import is not caught. Report V-CN's rows.
- **s12 created:** credit and debit numbers are separate series in Tally, which agrees with condition 1's per-class domain.
- **s12 declined:** Tally's guard spans classes. Condition 1 still stands, since a per-class read refuses less than Tally does. Record it.

**What these answers are for.** They inform #2346's note-numbering build. They are never a regression fixture for the build's output, because the build renders no note.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build has no note type.** `VoucherType` (`src-tauri/src/agent_import.rs:330`) has Payment, Receipt, Journal, Contra and Sales. So no build sends a note, and this row sends them raw.
- **The flag the build reads.** `resolve_voucher_type` (`src-tauri/src/agent_import_invoice_wire.rs:329`) refuses a series that is not Manual with `PREVENTDUPLICATES` Yes (`:384` to `:390`). T is its request, byte-equal.
- **The number read.** The build's request (`:436` to `:445`) computes `BRIDGEVCHISSALES` from `$$IsSales`. N-0010s is that request. The other N files change its `COMPUTE` to the class element and function `src-tauri/src/agent_voucher_type_class.rs` already sends in the class-resolving read.
- **The read-back.** R-MC1 and the other R files are the build's read-back (`render_invoice_readback_request`, `agent_import_invoice_wire.rs:505`) with the number and the type's GUID changed. The build would fill both from the batch and the resolved type.

## Not established after this row
- The uniqueness domain for tax (credit with credit, or every note together). That is a decision on unverified recall, not a measurement.
- How a note appears on the return, invoice-mode note entry, and `GSTNATUREOFRETURN`.
- A note type created over XML rather than keyed.
- A same-key or new-key resend on the debit side.
- A numberless post into a Manual note type with duplicates allowed.
- Renumbering of notes, and a type with more than one series.
- A note built and posted through ComplyEaze Bridge.
- Any other book or release.

Row: lab/2346-mn/
