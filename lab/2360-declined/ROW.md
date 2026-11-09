# Row 2360-L: after Tally declines a whole Sales import in its answer, does any voucher of it appear later?

Not for merge. Part of #2360 (ADR 0004; the owner's ruling of 5 Nov 2026 on #1709). Written 9 Nov 2026. Lines below are at master ae38a75c, where this branch starts. Confidence of everything here until the row runs: unverified.

## Question
A stop release may end the unconfirmed-invoice guard for a batch only if a received whole rejection is final: nothing of the batch lands in the book later. One read after the answer has shown nothing there, and neither mark moved:
- protocol reference §9.2, PARTIAL, one run each, Payment vouchers;
- §9.16, PARTIAL, one rehearsal, one declined Sales import.

No row has read the book again minutes later. This row does that: it sends one Sales invoice that Tally declines whole, then reads the voucher mark and that day's vouchers at once and at +1, +5 and +15 minutes. Two causes of rejection, three sittings on separate days, Tally closed and opened again between sittings.

**One variable per run.** Within a sitting, runs A and B differ only in the cause:
- **A:** the invoice debits a ledger the book does not hold (§9.2).
- **B:** the invoice reuses a number the book holds, on a Manual series that prevents duplicates (§9.14).

Date, ledgers otherwise, amounts and envelope are the same in both. Across sittings, each file differs from the last only in its fresh REMOTEID, and in run A its fresh number, because a write file is sent once in the row's life (C3).

**Where this row departs from the issue's row.**
- The book is BRIDGE PILOT LAB as earlier rows left it, not a new one. Its `BRIDGE Sales` type already has one series, Default, Manual, with `PREVENTDUPLICATES` Yes (rows 2162-KD and 2162-FY14). The voucher it holds, `BP/26-27/0010` dated 20260801, stands for the issue's `LR-1`. So no voucher type, ledger or voucher is made for this row.
- **Run C** (cause A sent while another read is still being answered) is **not in this row**. A read that keeps the gateway busy long enough needs a book of tens of thousands of vouchers, as #1713's rows used. BRIDGE PILOT LAB is small, and running C on another book would change a second variable. Run C stays owed, as its own row.
- **The waits** (+1, +5 and +15 minutes) are counted from the import's start time in its log, when the send began, not from its answer.
- **A repeat (P5) repeats the read, not the run.** A write file is sent once in the row's life (C3), so a read that shows a change is sent once more; the write is never sent again.
- **The release** is the lab's TallyPrime 7.2 Silver, recorded at each sitting. §9.2 and §9.14 were seen on 7.1 Silver.
- **Restarting Tally between sittings** departs from C2's "Never 'open application' on Tally", at step 6 only, under the owner's word of 28 Oct 2026 that the lab restarts Tally. Step 6 says how.
- **`run.py`** sends by `curl.exe` with `--fail-with-body` and records curl's exit code and the answer's bytes, as row 2162-FY14R did. It keeps no headers file (C3's `-D`), so CONTENT-LENGTH is not recorded, and prints no status line (`-w`), so the status is not recorded. A status of 400 or above is a non-zero exit, which stops the run; another status other than 200 is not caught by curl, and its answer is caught only if it is empty, an Unknown Request, or does not parse as the read or import `count10.py` expects. It does not hash the request; the steps do that before every send (C3).

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2162-FY14R's) applies in full. Where it and this sheet differ, the stricter one holds.
- **Pinned by GUID.** The book is the company whose GUID is `BOOK` in `count10.py` (it begins `6b43e498`). Each sitting's first marks read must print `GUID is the book's: yes`. Any other GUID, or more than one COMPANY row, = STOP before any write, whatever name the company shows. `--before-write`, `--same-row` and `--same-mark` stop on any other GUID too.
- Its state when last read (row 2162-FY14, `lab/2162-fy14-answers` at 35bceb81):
  - restored to ALTVCHID 68 and ALTMSTID 245;
  - current period 1-Apr-26 to 31-Mar-27, and the Gateway's current date 15-Sep-2026;
  - `BP/26-27/0010` held once, dated 20260801.

  Later rows may have moved the marks. Another value is recorded, not a STOP.
- **One BK and one restore in every sitting** (C5, C6): the BK before the sitting's first write, and the restore after its last, whatever the runs did. The restore is checked by the marks read's COMPANY row: its GUID, ALTVCHID and ALTMSTID equal the sitting's first read (the ruling of 4 Nov 2026 on unchanged marks).
- **Do not change Tally's current date or period.**

**Screens.**
- Tally's menus are opened by clicking their labels with the mouse, never by Alt+Y or any other menu key.
- This row opens no master or voucher-type screen. If one opens, it is left with Ctrl+Q, only after a screenshot shows its title, and never with Escape (C7).
- The Backup and Restore screens are reached by clicking the Data menu's label, then Backup or Restore, and are used as C5 and C6 say.

**No Bridge build and no approval window.** Raw requests, one at a time, through `run.py` (row 2162-FY14R's, byte-equal). Each of these stops the run and is reported as "the request failed", never as "nothing found":
- `run.py` stops on a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 bytes.
- `count10.py` prints STOP and exits 1 on a read answer with no `BODY/DATA/COLLECTION`, an import answer with no counters, or an answer that does not parse.

## Files
Requests are UTF-16LE with a BOM, as sent. `SHA256.txt` lists every file with its sha256.
- **Byte-equal to row 2162-FY14R's file of the same name** (`lab/2162-fy14r-sheet`, folder `lab/2162-fy14r/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml` (the marks read), `T-types.xml` (the build's voucher-type read), `template-import-0010.utf16le.xml` (the build's request that created `BP/26-27/0010`) and `template-n-0001.utf16le.xml` (the build's invoice-number read as row 2162-KD captured it; master's now also fetches three IRN fields).
- **`gen.py`** writes the other nine files from the two templates, asserting each change. It says what each is:
  - `N-day-20260801.xml`: every voucher dated 20260801, by the number read's own FETCH and COMPUTE, with the number clause taken out. A new shape, so it gets a screenshot after its first send in each sitting (C3).
  - `I-A1.xml` to `I-A3.xml` (run A, sitting 1 to 3) and `I-A9.xml` (spare): the party `BRIDGE Walk-in` renamed `BRIDGE Absent 2360` in its seven places, with fresh numbers `BP/26-27/0081` to `0083` (spare `0089`).
  - `I-B1.xml` to `I-B3.xml` and `I-B9.xml` (spare): the template with only its REMOTEID changed, so each reuses `BP/26-27/0010` on its own date.
  - No REMOTEID here is the held voucher's: one equal to it would alter that voucher, not be declined (§9.14).
  - Before the first sitting, run `python3 -I gen.py`. Before each sitting, check every file against `SHA256.txt`, and before every send check that send's request files again (Get-FileHash), as C3 asks. Any file that differs = STOP, and nothing is sent.
- **`count10.py`:** row 2162-FY14R's `count9.py` with three modes added, each marked 2360:
  - `--whole I`: prints WHOLE REJECTION only if the seven counters of §9.2 and COMBINED (C4) are each present once, EXCEPTIONS is 1 and the other seven are 0, as in row 2162-FY14's declined answer. Otherwise it prints NOT A WHOLE REJECTION and exits 2.
  - `--same-day A B`: compares two day reads row by row. Both must be answers of `N-day-20260801.xml` by name, and A must hold `BP/26-27/0010` dated 20260801 once, else STOP. It prints SAME DAY, or DAY DIFFERS and exits 3.
  - `--same-mark S M`: compares the GUID, ALTVCHID and ALTMSTID. It prints SAME MARK, or MARK MOVED and exits 4.

  Exit 3 and 4 are results, not STOPs: the steps say what follows. When the sheet was written, these were tested on row 2162-FY14's committed answers, kept on no branch:
  - `--whole`: a declined import printed WHOLE REJECTION; a created one, and copies with CANCELLED or COMBINED removed or with IGNORED, ERRORS, CANCELLED or COMBINED set to 1, each exit 2.
  - `--same-mark`: equal marks printed SAME MARK; ALTVCHID 68 against 69, and ALTMSTID 245 against 246, each exit 4.
  - `--same-day` (on number reads renamed as day reads): two equal reads printed SAME DAY; 1 row against 0 exit 3; a first read without `0010` STOP and exit 1; a file not named as a day answer STOP and exit 1.
  - A copy with its COLLECTION removed printed STOP and exit 1.

## Steps
Run each send as `python3 -I run.py http://127.0.0.1:9001 <label> <files>`, then `python3 -I count10.py a-<label>-*`. A STOP from either script stops the row there. Labels carry no hyphen.

In sitting k (k = 1, 2, 3), every label begins `p<k>`. Between the sends of a run, nothing else is sent and no key is pressed in Tally.

1. **Load and check.**
   - C2's step 0, with no company loaded. In sittings 2 and 3, write down tally.exe's start time first: it must be later than the previous sitting's close. A company that loaded by itself when Tally started is shut first, by C9. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Write down the release and licence mode, the current period and the Gateway's current date. If the date is not 15-Sep-2026, record it and go on unless it is on or after 1-Apr-2027.
   - `p<k>s1`: `M-marks.xml T-types.xml N-day-20260801.xml`. Then screenshot the whole Tally window.
     - EXPECT one COMPANY row and `GUID is the book's: yes`, else STOP.
     - EXPECT in T `BRIDGE Sales` 1 row: one series, Default, Manual, `PREVENTDUPLICATES` Yes. Anything else: record it, send no write, and go to step 6: no BK has been taken and nothing was written, so there is nothing to restore. Run B would then not be declined by duplicate number. A sitting ended here is not counted, and the coordinator says whether it is held again, with the same k, on a later day.
     - In N-day: record every row. EXPECT `BP/26-27/0010` dated 20260801 among them, and no `0081` to `0089`. Anything else: record it, send no write, and go to step 6, as above.
   - **Take BK** by C5, to a new empty folder. Record the file's name, size and sha256. Then `p<k>bk`: `M-marks.xml`, and record both marks. C5 lets a backup move the marks (a result, not a STOP), so from here the sitting's marks are compared with `p<k>bk`, and its day reads with `p<k>s1`; `--same-mark a-p<k>s1-M-marks.xml a-p<k>bk-M-marks.xml` is run and recorded only.
2. **Run A.**
   1. `p<k>a0`: `M-marks.xml N-day-20260801.xml`, then `python3 -I count10.py --before-write a-p<k>s1-M-marks.xml a-p<k>a0-M-marks.xml`, `--same-mark a-p<k>bk-M-marks.xml a-p<k>a0-M-marks.xml` and `--same-day a-p<k>s1-N-day-20260801.xml a-p<k>a0-N-day-20260801.xml`. Nothing has been written in this sitting yet, so MARK MOVED or DAY DIFFERS here = STOP.
   2. Only if that passes, and no key was pressed in Tally since: `p<k>a`: `I-A<k>.xml` alone. Screenshot the whole Tally window at once. A dialog = the Stop below.
   3. `python3 -I count10.py --whole a-p<k>a-I-A<k>.xml`, and record every counter and every LINEERROR. NOT A WHOLE REJECTION: send nothing more, and report to the run coordinator with the answer. The restore (step 5) is made only on the coordinator's word.
   4. `p<k>a1`, at once: `M-marks.xml N-day-20260801.xml`.
   5. `p<k>a2`, `p<k>a3`, `p<k>a4`: the same two files, sent at +1, +5 and +15 minutes after the import's start time in `log-p<k>a.txt`, each within one minute after its time. A read sent later is still sent, and its time recorded.
   6. After each of `a1` to `a4`, run `--same-mark a-p<k>bk-M-marks.xml a-p<k>a<n>-M-marks.xml` and `--same-day a-p<k>s1-N-day-20260801.xml a-p<k>a<n>-N-day-20260801.xml`. Every read of the sitting is compared with these two, never with another run read, so a read that differed once and not on its repeat is never a baseline.
      - MARK MOVED or DAY DIFFERS: send the same two reads once more at once, as `p<k>a<n>r`, and run both checks on them (P5).
      - If the repeat shows it too, that is a late voucher: record every row and both marks, send nothing more, and report. The spare `I-A9.xml` is sent only on the coordinator's word.
      - If the repeat does not show it, record both reads and go on.
3. **Run B.** As step 2, with `b` in place of `a` in every label and `I-B<k>.xml` (spare `I-B9.xml`). Its three checks at `b0` are against the same two reads as at `a0`, but there MARK MOVED or DAY DIFFERS is a reading of run A, 15 minutes or more after its write: it is handled as in step 2.6 (one repeat as `p<k>b0r`; confirmed, it is a late voucher, and B is not sent). EXPECT, as at row 2162-FY14's s5, CREATED 0, EXCEPTIONS 1 and no LINEERROR.
4. **The sitting's last read**, after B's +15 minutes: `p<k>s8`: `M-marks.xml N-day-20260801.xml`, `--same-mark a-p<k>bk-M-marks.xml a-p<k>s8-M-marks.xml` and `--same-day a-p<k>s1-N-day-20260801.xml a-p<k>s8-N-day-20260801.xml`. EXPECT SAME MARK and SAME DAY. Either one not so: one repeat as `p<k>s8r`, as in step 2.6; confirmed, it is a late voucher.
5. **The restore**, whatever steps 2 to 4 did, except after a STOP, NOT A WHOLE REJECTION, a late voucher (at any of a run's reads, at `b0` or at `s8`) or a dialog, where it waits for the coordinator's word (C8), and step 6 follows only on that word. C6, and row 2162-FY14R's step 6 word for word:
   - shut the company;
   - restore BK's folder alone;
   - answer only "Accept ?" (y) and "Company data folder (<company number>) already exists ..." (Replace, r);
   - load the company alone.

   Then `p<k>s9`: `M-marks.xml T-types.xml N-day-20260801.xml`, `count10.py --same-row a-p<k>s1-M-marks.xml a-p<k>s9-M-marks.xml` and `--same-day a-p<k>s1-N-day-20260801.xml a-p<k>s9-N-day-20260801.xml`. EXPECT both pass, and T as at `p<k>s1`. Anything else = STOP (C8): record it, and report.
6. **Close** by C10. One tally.exe owns 9001. In sittings 1 and 2, then close Tally by its own Quit, clicked by its label: write down the label and the time, and check that tasklist shows no tally.exe. Nothing is ended by tasklist or Task Manager. If Tally does not close within two minutes: press nothing more, screenshot, and report. Open Tally again only at the next sitting, on a later calendar day, and only after tasklist shows no tally.exe: start it once from its desktop shortcut, write down the time, and take C2's step 0 in full before anything else. That start is this row's one departure from C2's "Never 'open application' on Tally" (see above).

**Stop**, with no key pressed, if Tally's window shows a dialog after a write, or if the gateway stops answering. Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.

## Record
- Each answer's bytes and sha256, `count10.py`'s lines, and each `log-<label>.txt`.
- The minutes between each import's start and each of its reads, from the logs.
- The screenshot after each write and after each sitting's first N-day read, by name, bytes and sha256. Screenshots are never pushed.
- The BK file of each sitting, and every prompt of each restore word for word, with the key that answered it.
- Per sitting: the release, the licence mode, the current period, the Gateway's date, tally.exe's start time, and how Tally was closed.
- The result with its confidence marker (P6): verified, partial or unverified, and why.

## Expected
Estimates, not predictions.
- **A and B:** each declined whole in its answer, A with a LINEERROR naming the absent ledger, B with none (§9.2, §9.14).
- **Every read:** SAME MARK and SAME DAY.

## Pass, fail and what each decides
- **Pass:** in every read of every run of every sitting, `b0` and `s8` included, SAME MARK and SAME DAY. That is evidence for causes A and B only, on this book and release, at PARTIAL (P6). It does not settle #2360: the issue stays open until run C has run, and whether the change it describes is built is decided then.
- **Fail:** a late voucher confirmed by its repeat, at any of a run's reads, at `b0` or at `s8`. That closes #2360 as not safe, and the stated stop stays.
- **NOT A WHOLE REJECTION:** not a result about late vouchers. Record the answer: it says how Tally answers that cause on this book.
- **A dialog after any write:** the row stops (see Stop).

## Code admits the trigger
Lines are at master ae38a75c.
- **The guard.** `unsettled_invoice_twin` (`src-tauri/src/agent_import_ledger.rs:1111`) counts a voucher with a new invoice's date, type and entries as an unconfirmed earlier invoice. Only `verify_import` ends that.
- **The release.** `acknowledge_post_review` with `doubt` `invoice_stop` (`src-tauri/src/agent_import_ack.rs:1031`) lifts the stop, not the guard.
- **What the build sends.** Its invoice body is the template at `src-tauri/src/agent_import_invoice.rs:2708`. `template-import-0010` is that request, sent by the build and committed by row 2162-KD. Its number read is `render_invoice_number_request` (`src-tauri/src/agent_import_invoice_wire.rs:429`).

## Not measured
- run C;
- a batch of more than one invoice;
- a partly created batch or a lost answer (out of scope in #2360);
- any cause other than A and B;
- a wait longer than 15 minutes;
- an invoice built and posted through Bridge;
- Gold, any other book, and any other release.

Row: lab/2360-declined/
