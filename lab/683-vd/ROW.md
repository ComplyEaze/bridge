# Row 683-VD: on the several-currency lab book, with row 683-LD's two vouchers dated 30 April saved, does the Trial Balance to 1 April read as row 683-TW's when Tally's current date is 1 April

Not for merge. Part of #683. **Two writes, behind a backup:** row 683-LD's two vouchers are keyed again on screen, the reads are taken, and the book is restored. The run coordinator gives the word for that restore now, under C8, for every outcome from the BK on (step 3 says when it waits). Lines are at master ae38a75c.

## Question
The Trial Balance from 1 April 2025 to 1 April 2025 (`tb3`, request sha256 `1853d0df…0ad1a`) has been read three ways on this book:
- **Row 683-TW:** the book as found (every voucher dated 1 April, date of last entry 1-Apr-25), current date 1-Apr-2025. 14,140 B, six ledgers MIXED (each reading the dollar composite), two PLAIN.
- **Row 683-LD:** with two vouchers dated 30 April saved (date of last entry 30-Apr-25), current date 30-Apr-2025, moved there by the voucher's date. 13,528 B, every ledger PLAIN; each of the six read its rupee figure alone, and every rupee figure was the same.
- **Row 683-CD** (`lab/683-cd-answers` 09fa7a49): the book as found, current date moved to 30-Apr-2025 by F2 at the Gateway and back. All three answers (`tb3a` at 1-Apr-2025, `tb3d` at 30-Apr-2025, `tb3r` at 1-Apr-2025) are 14,140 B with sha256 `1c4638e8…b7b7`, byte-equal to row 683-TW's, six MIXED; m0, m1 and m2 byte-equal. So the current date alone does not change this answer.

So row 683-LD's change needs the vouchers dated 30 April (and the date of last entry they move), with or without its current date of 30 April. This row separates those two (P6). It keys row 683-LD's two vouchers again, word for word, and sends `tb3` with the current date at 30 April (row 683-LD's), then at 1 April (the question), then at 30 April again. Against row 683-LD's `tb3`, the one thing `tb3v` changes is the current date.

**Q1:** with the two vouchers saved and the current date set back to 1-Apr-2025, does `tb3v` read six MIXED (as rows 683-TW and 683-CD) or every ledger PLAIN (as row 683-LD)? `tb.py --sets` answers it in one line against each.

**The controls:**
- **Before the BK:** `tb3a` at 1-Apr-2025 on the book as found. EXPECT byte-equal to row 683-TW's `tb3` answer (row 683-CD read the same bytes). And `mv0`, the book-scoped window, EXPECT byte-equal to row 683-TW's `mvy`; the restore is checked against it.
- **After the write, at 30-Apr-2025:** `tb3w`, row 683-LD's condition repeated. EXPECT byte-equal to row 683-LD's `tb3` answer. Then a difference in `tb3v` is about the current date, not the vouchers as keyed this time.
- **After `tb3v`:** the date back to 30-Apr-2025, and `tb3w2`. EXPECT byte-equal to `tb3w`. A `tb3w2` that differs means the date's return did not return the answer, and `tb3v` then decides nothing until that is explained (P5).

Every request is master's own builder, sent raw, and each has been sent on this book before (rows 683-TW, 683-LD and 683-CD). No request is new and nothing is rendered for this row.

## Book
`BRIDGE FOREXGROUP LAB` (synthetic), GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, TallyPrime 7.2. THE COMPANY in `LAB-COMMON.md` means this book.
- **Synthetic:** it is in master's registry, `scripts/merge-gate-synthetic-uuids.json` line 201 ("created in the lab on 28 October 2026 with invented masters and vouchers keyed by hand").
- **Not an excluded book:** its GUID is not the register lab book's (`REGISTER_LAB_GUID`, `captured_parse_record.rs:302`), and not the one the private exclusion list names.
- **Pinned by GUID.** The `BOOK` of `lm.py` and `tb.py` is this GUID. m0 must print one COMPANY row, `the book's: yes`, or the row STOPs before any other send.
- **Its last state:** row 683-LD restored it from its backup, then m2 byte-equal to m0 (V 11, M 214) and the book-scoped window byte-equal to the one found; the Gateway then showed current date Tuesday, 1-Apr-2025, period 1-Apr-25 to 31-Mar-26, last entry 1-Apr-25. Row 683-CD wrote nothing, by its marks, and set the date back to 1-Apr-2025. `a-ldm2-M-marks.xml` here is row 683-LD's m2; m0 checks that nothing has written to the book since.
- **Its vouchers** (row 683-TW's book-scoped answer, `a-twmvy` here): Sales 1, 2 and 3; Receipt 1, 2 and 3; Journal 1; Purchase 1 and 2; Payment 1 and 2, all dated 20250401. Row 683-LD's restore removed its Sales 4 and Receipt 4. **So Sales 4 and Receipt 4 are expected to be offered again** (see The write).
- **The period:** do not change it. The current date is changed only where a step says: by F2 on K1's voucher screen (it moves the current date, as rows 1211 and 683-LD recorded) and by F2 at the Gateway (steps 6b and 6c).
- **One BK, taken in this sitting before the write, and one restore after the last read** (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID must equal m0's (the ruling of 4 Nov 2026 on unchanged marks). It is also checked by the book-scoped window, which must again equal the book as found.

## Kit
Check every file against `SHA256.txt` first, and check each send's request file again just before it is sent (Get-FileHash, C3). If any file differs: STOP, and nothing is sent.

| file | what, and where its bytes come from |
|---|---|
| `M-marks.xml` | The marks request (GUID, ALTVCHID, ALTMSTID). Byte-equal to rows 683-TW's, 683-LD's and 683-CD's. |
| `T-tb3-20250401-20250401.xml` | Row 683-TW's `tb3` request, byte-equal (sha256 `1853d0df…0ad1a`): `render_native_trial_balance_request_with_currency` (`native_trial_balance/mod.rs:170`), from 20250401 to 20250401. Rows 683-LD and 683-CD sent the same file. Every Trial Balance of this row uses it. |
| `V-mvy-20250401-20260331.xml` | Row 683-TW's `mvy` request, byte-equal: `render_agent_movement_vouchers` (`agent_read_profiles.rs:100`), book scope, 20250401 to 20260331. Row 683-LD sent it as `mv0`, `mv1` and `mv2`. Here it checks the book before the BK and after the restore. |
| `a-twtb3-T-tb3-20250401-20250401.xml` | Not sent. Row 683-TW's `tb3` answer (`lab/683-tw-answers` 4f31b925, sha256 `1c4638e8…b7b7`, 14,140 B; six MIXED). |
| `a-ldtb3-T-tb3-20250401-20250401.xml` | Not sent. Row 683-LD's `tb3` answer (`lab/683-ld-answers` e38d2b82, sha256 `a12217b8…7705`, 13,528 B; all PLAIN). |
| `a-twmvy-V-mvy-20250401-20260331.xml` | Not sent. Row 683-TW's `mvy` answer (sha256 `56554b6a…4df7`, 59,524 B); row 683-LD's `mv0` and `mv2` are byte-equal to it. |
| `a-ldm2-M-marks.xml` | Not sent. Row 683-LD's m2, the marks after its restore (sha256 `3d75dd1a…dafba`, 3,404 B; byte-equal to its m0 and to row 683-TW's m0). |
| `a-ldm1-M-marks.xml` | Not sent. Row 683-LD's m1, the marks with its two vouchers saved (sha256 `cee319cc…64d7`, 3,404 B; ALTVCHID 13, ALTMSTID 214). |
| `run.py`, `LAB-COMMON.md`, `.gitattributes`, `lm.py`, `tb.py` | Rows 683-TW's, 683-LD's and 683-CD's, byte-equal. |
| `cmpinfo.py` | Row 683-LD's helper (`lab/683-ld-answers` `scripts/cmpinfo.py`), byte-equal. It compares two answers byte for byte and, where they differ, says whether every difference lies inside the CMPINFO count block (C4: not rows), listing those counts. Exit 0 BYTE-EQUAL, 1 differing only inside CMPINFO, 2 differing outside it. Row 683-LD found its `tb3` carried CMPINFO counts after its vouchers were saved (LEDGER 6, CURRENCY 2, TAXUNIT 2, VOUCHERNUMBERSERIES 2, VOUCHER 3), where row 683-CD's answers carried 0, so a Trial Balance after the write can differ from another in those lines alone. |

The reference answers keep the `a-<label>-<K>-` form so that `lm.py` and `tb.py` read them by kind. Their labels (`twtb3`, `ldtb3`, `twmvy`, `ldm2`, `ldm1`) are not used for any send.

## The write
Row 683-LD's two vouchers, keyed as its `RESULTS.md` (step 4) records them. The run lead makes them on Tally's screen, once, after the BK and the check before the write (step 4). **Type only the fields named, and leave every other field as Tally offers it (Enter only).** As row 683-LD found:
- **Type every name and amount as keystrokes, in short chunks, never as one paste.** In row 683-LD a pasted party name left the field empty and brought up a "Spelling Error" box. Check the highlighted row before each Enter.
- no narration;
- a screenshot before each Ctrl+A.

Expected screens, none of them a STOP (each was seen in row 683-LD):
- the Change Voucher Date box on K1's F2;
- the Dispatch Details box and the Party Details box after K1's party, each passed by Enter on every field as offered;
- the Forex Rate of Exchange box where K1 says;
- the Bill-wise Details and Pending Bills the steps name;
- after K2's credit line, Tally offering `By` `Cash` with the debit as offered.

If Tally adds a gain or loss line, press no key: screenshot it, write it down, and ask the run coordinator before Ctrl+A. Do not add one.

**Every STOP inside these steps means: press no key, screenshot the whole window, and report.**

**K1, F8 Sales** (from the Gateway, Vouchers, F8):
1. **Voucher No.:** write down what Tally offers. EXPECT `4`. If it offers any other number, STOP before any other key: `1`, `2` or `3` would reuse a number, and anything else means the book is not as its answers show.
2. **Date:** F2, type `30-4-2025`, Enter. Write down the date the box offered. The voucher must then show 30-Apr-25 Wednesday. If it shows anything else, or Tally answers with a message: STOP.
3. **Party A/c name:** `BRIDGE USD Customer`, Enter on the one row listed. Dispatch Details and Party Details: Enter only.
4. **Particulars:** `BRIDGE Export Sales`, Enter on the one row listed; the amount typed `$100`, Enter. Where the Forex Rate of Exchange box opens: Enter on the forex amount, `88` typed over the rate, Enter, the value as offered (8,800.00 expected), Enter. If no such box opens, or the line does not then show `$100.00 @ ₹ 88/$`: STOP before Ctrl+A.
5. **Bill-wise Details:** `New Ref`; Name typed `LD1`, replacing what is offered; Due Date empty; `$100.00` and `Dr` as offered. Write down each value shown.
6. **Screenshot the whole screen before Ctrl+A.** Write down the Voucher No., the date and each line as Tally shows it. Then Ctrl+A. If "Accept ?" is asked, Yes. Row 683-LD's Ctrl+A saved with no prompt; record which it was.

**K2, F6 Receipt** (from the Gateway, Vouchers, F6):
1. **Voucher No.:** EXPECT `4`, with the same STOP as K1's.
2. **Date:** if the screen does not already show 30-Apr-25, F2, type `30-4-2025`, Enter. Write down which it was. If it then shows anything else: STOP.
3. **To** `BRIDGE INR Customer`, **Credit** `2000`. If a Forex Rate of Exchange box opens: STOP, press nothing.
4. **Bill-wise Details:** `Agst Ref`, choose `R1` (the pending amount offered is expected to be 3,000.00 Dr), then the Amount typed `2000` Cr, replacing what is offered. If R1 is not in Pending Bills: STOP.
5. **By** `Cash`, **Debit** as offered (2,000.00 expected).
6. **Screenshot the whole screen before Ctrl+A.** Write down every line. Then Ctrl+A, with Yes to "Accept ?" if it is asked; record which it was.

After each save, Tally shows a blank voucher screen. Screenshot its title, then leave it with Ctrl+Q (C7). Never press Esc on a voucher screen, and never press Ctrl+Q at the Gateway. After K2, screenshot the whole Tally window at the Gateway, and write down its current date and date of last entry (30-Apr-2025 and 30-Apr-25 expected), before `m1` is sent (C3).

After a STOP inside these steps, nothing of that voucher has been saved, but K1 may have been. Press no key: the run coordinator says how the screen is left. The restore then runs as step 8 says.

## The current date at the Gateway
F2 at the Gateway opens **Change Current Date** (rows 1039, 2015, 2016 and 683-CD used it; rows 2015, 2016 and 683-CD read the marks unchanged after it). At the Gateway, with no report open:
1. F2. EXPECT a box titled `Change Current Date` with one field, `Current Date` (row 683-CD's words). Screenshot it and record the date it offers: in row 683-CD it offered the day after the current date, and the date typed replaced it.
2. Type the date as `d-m-yyyy`, Enter.
3. EXPECT the Gateway again, showing the new current date with its weekday. Screenshot it, and write down the period and the date of last entry, which must read as before (1-Apr-25 to 31-Mar-26, last entry 30-Apr-25).

If what opens is not that box, or the box does not take the date (it stays, or a message comes up), the path rule applies: Esc once to leave the box only, send nothing, record the Gateway's current date, and go to step 8 on the restore's word (the box is not a voucher or master screen).

## Screens and prompts
The screen paths for load and close are those of rows 683-TW, 683-LD and 683-CD:
- **Load:** Select Company, the book typed into its name filter, the one row, Enter.
- **A company already loaded at the start:** C1 says STOP. If the run coordinator's word is to shut it, shut it by the K: Company label, Shut, Enter on it, with no change to it.
- **Close (C10):** the K: Company label, Shut, Enter on the book.
- **Bring Tally to the front only by its running window.** A second `tally.exe` is a STOP.

**The prompt rule.** The only dialogs answered in this row are:
- C5's: a message that only says the backup is done (screenshot it, accept once), and the backup's "Accept ?" = Yes;
- C6's two: "Accept ?" = Yes, and "Company data folder (<the book's number>) already exists ..." = Replace;
- each voucher's own "Accept ?" on Ctrl+A = Yes, if it is asked;
- the screens the write names: Change Voucher Date (F2), Dispatch Details, Party Details, the Forex Rate of Exchange box, Bill-wise Details and Pending Bills;
- the Gateway's Change Current Date box.

**Any other dialog, message box, sub-screen or prompt, at any step, is unnamed:** press no key, screenshot the whole window, copy its text exactly, and send nothing while it is up. Then ask the run coordinator (C7). It stays on the screen until the run coordinator answers.

**The path rule.** If a list or line shown is not exactly what a path above names (other words, no such line, a Spelling Error), send nothing and ask:
- **a list or box reached from the Gateway** (Select Company, the Shut list, the Backup or Restore list, Change Current Date): Esc out of it;
- **a list on a voucher screen** (the party, ledger or Pending Bills list of K1 or K2): press no key. Never press Esc on a voucher or master screen (C7).

## The wedge rule
A read that Tally is still building keeps the gateway busy for every company, and giving up on it does not stop Tally's work. So:
- **One request at a time, by `run.py`.** It never abandons a request: curl waits for its answer.
- **The book is small:** 13 vouchers and 8 ledgers after the write. This row's largest answer on record is the book-scoped window, 59,524 B. Every read of row 683-LD answered within about a second.
- **If any send has not answered 5 minutes after it began:**
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).

  Nothing is re-sent on this row's own judgement. With the vouchers saved and the gateway stuck, the restore waits for the run coordinator's word.
- **Failed requests:** `run.py`'s own stops (a curl exit other than 0, Unknown Request, an empty answer, an answer above 2,000,000 B), and the STOP of `lm.py` or `tb.py`. Each is a failed request, never "nothing found".

## Steps
Run each send as `python3 -I run.py http://127.0.0.1:9001 <label> <file>`, and write down each send's start time from its log. **Each label is used once.** A repeated send takes a new label (`m1r`), never an earlier one's. The answer of send `<label>` is `a-<label>-<request file>`.

0. **C2's step 0, with no company loaded:**
   - the screenshot first;
   - one `tally.exe` owning 9001;
   - `GET /status` 200 (record its bytes).

   Load the book alone, as above. Read the Gateway's current date, period and date of last entry, and write them down. Record the release and the licence mode, as row 683-LD recorded them, and the PC's date.
   - EXPECT current date Tuesday, 1-Apr-2025, period 1-Apr-25 to 31-Mar-26, last entry 1-Apr-25.
   - If any differs: record it and STOP before any send. Press nothing to change it. The run coordinator decides.
1. **`m0`:** `M-marks.xml`, then `python3 -I lm.py --same-row a-ldm2-M-marks.xml a-m0-M-marks.xml`. EXPECT SAME MARKS: one COMPANY row, the book's, with ALTVCHID 11 and ALTMSTID 214.
   - Any other GUID or row count: STOP.
   - MARKS DIFFER: record it and STOP before step 2. The book changed after its last note, and its voucher numbers are then not known.
2. **The book as found** (the controls), one send at a time:
   - **`tb3a`:** `T-tb3-20250401-20250401.xml`. Record whether its answer is byte-equal to `a-twtb3-T-tb3-20250401-20250401.xml` (sha256). Then `python3 -I tb.py a-tb3a-T-tb3-20250401-20250401.xml`, recorded whole, and `python3 -I tb.py --sets a-twtb3-T-tb3-20250401-20250401.xml a-tb3a-T-tb3-20250401-20250401.xml`. EXPECT byte-equal, and SETS SAME.
   - **`mv0`:** `V-mvy-20250401-20260331.xml`. Then `python3 -I lm.py --capture a-mv0-V-mvy-20250401-20260331.xml a-twmvy-V-mvy-20250401-20260331.xml`. EXPECT CAPTURE SAME, with `vouchers: movement 11, capture 11`. Record whether `a-mv0` is byte-equal to `a-twmvy`.
   - If either is not as EXPECT: record it, and STOP before the BK. The book, or the script, reads differently from the rows before, and a write on it would decide nothing.
3. **BK**, by C5, to a new empty local folder named `bk-683vd`. Record the file's name, size and sha256, and the time the Restore list will show. Then **`mb`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m0-M-marks.xml a-mb-M-marks.xml`. Record its line. A difference is a result to record (C5), not a STOP.

   **The restore's word, given now (C8).** C8 restores a backup after a STOP only on the run coordinator's word. For this row that word is given in advance: from here on, step 8 runs after the last read, after any STOP at any later step, and after a voucher saved wrongly. Nothing else is sent, keyed or backed up after a STOP: only the restore and step 8's state reads, `m2` and `mv2`.

   The word waits, and the run coordinator is asked first, in three cases: an unnamed dialog still up, a send past the wedge rule's 5 minutes, or Tally closed (C9). Each waits for the word on how to clear it, and then the restore runs.
4. **The write.**
   - First **`mc`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-mb-M-marks.xml a-mc-M-marks.xml`. Go on only on SAME MARKS, and only with no key pressed in Tally since `mb`. Otherwise go to step 8.
   - Then K1 and K2, by **The write** above.
5. **`m1`:** `M-marks.xml`, then:
   - `python3 -I lm.py --same-row a-mc-M-marks.xml a-m1-M-marks.xml`. EXPECT MARKS DIFFER (exit 2), with ALTVCHID 13 and ALTMSTID 214.
   - `python3 -I lm.py --same-row a-ldm1-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS: the write moved the marks as row 683-LD's did.
   - Anything else: record it, and go on with step 6. The reads are still taken, and the restore puts the book back.
6. **The Trial Balance to 1 April, three times,** one send at a time, each answer's counters run before the next step:
   - **`tb3w`** (current date 30-Apr-2025, as the write left it; row 683-LD's condition): `T-tb3-20250401-20250401.xml`. Then `python3 -I cmpinfo.py a-ldtb3-T-tb3-20250401-20250401.xml a-tb3w-T-tb3-20250401-20250401.xml`, `python3 -I tb.py a-tb3w-T-tb3-20250401-20250401.xml`, each recorded whole, and `python3 -I tb.py --sets a-ldtb3-T-tb3-20250401-20250401.xml a-tb3w-T-tb3-20250401-20250401.xml`.
   - **6b. The date to 1 April:** by the Gateway path above, F2, `1-4-2025`, Enter. EXPECT Tuesday, 1-Apr-2025, with the last entry still 30-Apr-25. Then **`md1`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m1-M-marks.xml a-md1-M-marks.xml`. EXPECT SAME MARKS.
   - **`tb3v`** (current date 1-Apr-2025, the question): `T-tb3-20250401-20250401.xml`. Then, at once, screenshot the whole Tally window: this is the first Trial Balance on this book sent with entries after its `to` and the current date at its `to`. A dialog = Stop. Then, each recorded whole:
     - `python3 -I cmpinfo.py a-tb3w-T-tb3-20250401-20250401.xml a-tb3v-T-tb3-20250401-20250401.xml`;
     - `python3 -I cmpinfo.py a-tb3a-T-tb3-20250401-20250401.xml a-tb3v-T-tb3-20250401-20250401.xml`;
     - `python3 -I tb.py a-tb3v-T-tb3-20250401-20250401.xml`;
     - `python3 -I tb.py --sets a-tb3w-T-tb3-20250401-20250401.xml a-tb3v-T-tb3-20250401-20250401.xml`;
     - `python3 -I tb.py --sets a-tb3a-T-tb3-20250401-20250401.xml a-tb3v-T-tb3-20250401-20250401.xml`.
   - **6c. The date back to 30 April:** by the Gateway path above, F2, `30-4-2025`, Enter. EXPECT Wednesday, 30-Apr-2025, with the last entry 30-Apr-25. Then **`md2`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m1-M-marks.xml a-md2-M-marks.xml`. EXPECT SAME MARKS.
   - **`tb3w2`** (current date 30-Apr-2025 again): `T-tb3-20250401-20250401.xml`. Then `python3 -I cmpinfo.py a-tb3w-T-tb3-20250401-20250401.xml a-tb3w2-T-tb3-20250401-20250401.xml` and `python3 -I tb.py --sets a-tb3w-T-tb3-20250401-20250401.xml a-tb3w2-T-tb3-20250401-20250401.xml`, each recorded whole.
   - `md1` or `md2` MARKS DIFFER: record it, send nothing else but the restore, and go to step 8.
7. **`m1r`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m1-M-marks.xml a-m1r-M-marks.xml`. EXPECT SAME MARKS, since the reads and the date wrote nothing. On MARKS DIFFER, record it, send nothing else but the restore, and go to step 8.
8. **The restore,** from the BK of step 3, by C6:
   - check the backup file's sha256 against the one recorded, and write `restore-plan.txt`;
   - shut the company;
   - restore the BK's folder alone;
   - answer only "Accept ?" (Yes) and "Company data folder (<the book's number>) already exists ..." (Replace);
   - load the company alone, with the screenshots C6 names.

   Then:
   - **`m2`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m0-M-marks.xml a-m2-M-marks.xml`. EXPECT SAME MARKS.
   - **`mv2`:** `V-mvy-20250401-20260331.xml`, and `python3 -I lm.py --capture a-mv2-V-mvy-20250401-20260331.xml a-mv0-V-mvy-20250401-20260331.xml`. EXPECT CAPTURE SAME, with 11 and 11. Record whether `a-mv2` is byte-equal to `a-mv0`.
   - MARKS DIFFER or CAPTURE DIFFERS: STOP, and report "the restore of bk-683vd did not return the book to m0". Nothing more is sent or keyed.
9. **Tally's date:** read the Gateway's current date, period and date of last entry, and write them down. EXPECT 1-Apr-2025, 1-Apr-25 to 31-Mar-26 and 1-Apr-25, as at step 0 (row 683-LD's restore left them so). If any differs, press nothing to change it: record it and report it. The run coordinator decides.
10. **Close** by C10, with `GET /status` after the last request (C11). One `tally.exe` must own 9001.

Steps 2 and 6 never stop the row on an answer that differs from an estimate. Each such answer is a result and is recorded in full. Only the STOPs named stop the row.

## Stop
Press no key, screenshot the whole window, and report before anything else is pressed or sent, if:
- an unnamed dialog appears (the prompt rule), or a second `tally.exe` starts;
- the gateway stops answering, or a send passes the wedge rule's 5 minutes;
- `run.py`, `lm.py` or `tb.py` stops;
- step 0's date, period or last entry is not as EXPECT;
- m0 is not SAME MARKS with `a-ldm2`;
- a control in step 2 is not as EXPECT;
- a step of the write says STOP;
- the path rule applies;
- m2 or mv2 is not as EXPECT after the restore.

From step 3 on, a STOP is followed by the restore (step 8) on the word step 3 gives, except in the three cases step 3 names.

## Record
- Each answer's bytes and sha256, and each request's sha256 (as `SHA256.txt` lists; any other is the Stop of C3).
- `lm.py`'s lines for every marks read, and the whole output of each `lm.py` and `tb.py` call.
- Whether `tb3a` is byte-equal to `a-twtb3`, and each `cmpinfo.py` line (`tb3w` against `a-ldtb3`, `tb3v` against `tb3w` and `tb3a`, `tb3w2` against `tb3w`). Either way it is recorded, not judged. Where two Trial Balances differ, also list the fields that differ, row by row, as row 683-LD's `evidence/tb3-fields-vs-683tw.txt` did.
- Each send's start time and how long it took, from `log-<label>.txt`.
- **The write:** every value written down in its steps, word for word as the screen shows it:
  - each Voucher No. offered and the date shown;
  - each line and amount;
  - the rate box's title and fields;
  - every field of each Bill-wise line, with the pending amount R1 showed;
  - every prompt, with the key that answered it, and each save that asked none.
- Tally's current date, period and date of last entry at steps 0, 4 (after K2), 6b, 6c and 9, with the date each F2 box offered.
- The BK file (name, folder, bytes, sha256, and the Restore list's time), `restore-plan.txt`, the data-folder listings before and after the restore, and every prompt of the restore word for word.
- The marks at m0, mb, mc, m1, md1, md2, m1r and m2.
- The release, the licence mode, and the PC's date.
- Every screenshot by name, bytes and sha256 (screenshots are never pushed).

**Staging the evidence**, as for rows 683-LM's, 683-TW's, 683-LD's and 683-CD's:
- **The start-up check:** a hit is allowed only where it lies inside the amount `$1000000.00`. List each hit with 12 characters either side. A hit anywhere else waits for the run coordinator.
- **The check for 10 or more digits:** it sets aside this book's registered GUID matched whole, and nothing else.

## Expected (estimates, not EXPECTs)
- **The controls:** `tb3a` byte-equal to `a-twtb3`, and `mv0` to `a-twmvy`.
- **The write:** Sales No. 4 and Receipt No. 4, as row 683-LD's; m1 SAME MARKS with `a-ldm1`.
- **`tb3w`:** every ledger PLAIN, and byte-equal to `a-ldtb3` or differing from it only inside CMPINFO.
- **`tb3v`:** not known. That is the question.
- **`tb3w2`:** byte-equal to `tb3w`, or differing only inside CMPINFO.
- **The restore:** m2 and mv2 as m0 and mv0, and the Gateway at 1-Apr-2025 after the reload, as row 683-LD's.

## Decides
Below, **as row 683-LD's** means SETS SAME against `a-ldtb3` with `cmpinfo.py` exit 0 or 1, and **as `tb3w`** means the same against `tb3w`.

- **`tb3w` as row 683-LD's, `tb3v` six MIXED (SETS DIFFER against `tb3w`, SETS SAME against `tb3a`), `tb3w2` as `tb3w`:** with entries after `to`, the current date decides how the Trial Balance to 1 April reads the dollar ledgers. With it at `to` they read as composites; with it at the later voucher date they read as plain rupee figures. Row 683-CD found the current date alone changes nothing without such entries. So the plain answer needs both, on this book and release: PARTIAL. Record whether `tb3v` is byte-equal to `tb3a`; where it is not, record which fields differ.
- **`tb3w` as row 683-LD's, `tb3v` every ledger PLAIN (SETS SAME against `tb3w`), `tb3w2` as `tb3w`:** the vouchers dated 30 April (and the date of last entry they move) change this answer whatever the current date, on this book and release: PARTIAL. The current date is not needed.
- **`tb3v` neither:** some ledgers MIXED and some PLAIN, other text, or any UNREAD. Record it whole; it decides nothing until the run coordinator has read it.
- **`tb3w2` not as `tb3w`:** setting the date back did not return the answer. `tb3v` decides nothing until that is explained.
- **`tb3w` not as row 683-LD's:** row 683-LD's answer did not repeat with the same vouchers and date. `tb3v` and `tb3w2` are recorded, but decide nothing until that is explained.
- **`tb3a` not as row 683-TW's:** the book or the sitting reads differently from the rows before; the row stopped before the BK (step 2).

Whichever way it falls, any answer that becomes a fixture is parsed again by master's own parser first, before any code relies on it.

## Not in this row
- Any Trial Balance other than the one to 1 April, any movement window beyond the book-scoped checks, and any opening export.
- Any current date other than 1 April and 30 April 2025, a voucher on any day other than 30 April, and any change to the period.
- The vouchers without the date of last entry they move: the two cannot be separated by keying.
- `trial_balance` or `ledger_movement` through Bridge, another book, and another release.
