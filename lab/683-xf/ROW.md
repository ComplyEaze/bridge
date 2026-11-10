# Row 683-XF: on the several-currency lab book, with row 683-LD's two vouchers dated 30 April saved, does the opening export at 1 April, sent as the first read after the write, change how the Trial Balance to 1 April reads the dollar ledgers

Not for merge. Part of #683. **Two writes, behind a backup:** row 683-LD's two vouchers are keyed again on screen, as rows 683-VD and 683-RO keyed them, the reads are taken, and the book is restored. The run coordinator gives the word for that restore now, under C8, for every outcome from the BK on (step 3 says when it waits). Lines are at master ae38a75c.

## Question
The Trial Balance from 1 April 2025 to 1 April 2025 (`tb3`, request sha256 `1853d0df…0ad1a`) has given three answers on this book:
- **Rows 683-TW and 683-CD,** the book as found: 14,140 B, sha256 `1c4638e8…b7b7`. Six ledgers MIXED (each reading the dollar composite) and two PLAIN, whatever the current date.
- **Row 683-LD,** with two vouchers dated 30 April saved: 13,528 B, sha256 `a12217b8…7705`. Every ledger PLAIN, each rupee figure the same.
- **Rows 683-VD and 683-RO,** with the same two vouchers keyed again: 13,740 B, sha256 `f30cdee1…1bcb`, five times over (VD's `tb3w`, `tb3v` and `tb3w2`; RO's `tb3c` and `tb3x`). `BRIDGE Import Purchases` and `BRIDGE Lab Bank` MIXED, the other six PLAIN; every rupee figure the same.

**What each sitting sent after the write** (each row's `evidence/sends-utc.txt`):
- **Row 683-LD:** `m1`, then the opening export `x1`, then `tb3`. The first read after the write was the export.
- **Row 683-VD:** `m1`, then `tb3w`. **Row 683-RO:** `m1`, then `tb3c`, then `x1`, then `tb3x`. In both, the first read after the write other than the marks was a Trial Balance, and every Trial Balance after it in the sitting was byte-equal to that first one: across about 3 minutes and two changes of the current date in row 683-VD, and across `x1` in row 683-RO.

So row 683-RO showed that `x1` sent **after** a Trial Balance does not change the next one. It did not test `x1` sent **before** the first Trial Balance, which is row 683-LD's order. This row tests that alone (P6). It repeats row 683-VD's sitting word for word, except that `x1` is sent between `m1` and the first Trial Balance, as row 683-LD sent it. That is the only change against row 683-VD. Against row 683-LD, what still differs is recorded, not changed: the reads before the write, and the keying of K1 (see Decides).

**Q1:** with `x1` the first read after the write, does `tb3f` read as row 683-LD's `tb3` (every ledger PLAIN) or as rows 683-VD's and 683-RO's (two MIXED, six PLAIN)? `tb.py --sets` answers it in one line against each.

**Repeated (P5):** `tb3` is sent once more (`tb3f2`), and must read as `tb3f`.

**Timing is recorded, not set.** Row 683-LD sent `x1` about 10 s after `m1` answered and its `tb3` about 58 s after `x1` answered, about 2 minutes after K2's save. Here the gaps are those of running each answer's counters before the next send. Row 683-VD's `tb3w2`, about 3.7 minutes after its K2 save, read as its first Trial Balance.

**The controls:**
- **Before the BK,** as rows 683-VD's and 683-RO's: `tb3a` at 1-Apr-2025 on the book as found, EXPECT byte-equal to row 683-TW's `tb3` answer; and `mv0`, EXPECT byte-equal to row 683-TW's `mvy`. The restore is checked against `mv0`.
- **`x1`,** compared with row 683-LD's `x1` answer, to which row 683-RO's `x1` was byte-equal.

Every request is master's own builder, sent raw, and each has been sent on this book before (rows 683-LM, 683-OP, 683-TW, 683-LD, 683-VD and 683-RO). No request is new and nothing is rendered for this row.

## Book
`BRIDGE FOREXGROUP LAB` (synthetic), GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, TallyPrime 7.2. THE COMPANY in `LAB-COMMON.md` means this book.
- **Synthetic:** it is in master's registry, `scripts/merge-gate-synthetic-uuids.json` lines 201 and 202 ("created in the lab on 28 October 2026 with invented masters and vouchers keyed by hand").
- **Not an excluded book:** its GUID is not the register lab book's (`REGISTER_LAB_GUID`, `captured_parse_record.rs:302`), and not the one the private exclusion list names.
- **Pinned by GUID.** The `BOOK` of `lm.py` and `tb.py` is this GUID. m0 must print one COMPANY row, `the book's: yes`, or the row STOPs before any other send.
- **Its last state:** row 683-RO (`lab/683-ro-answers` dfce44b8) restored it from its backup, then its m2 was byte-equal to its m0 and to row 683-LD's m2 (V 11, M 214), and its book-scoped window byte-equal to the one found. The Gateway then showed current date Tuesday, 1-Apr-2025, period 1-Apr-25 to 31-Mar-26, last entry 1-Apr-25. `a-ldm2-M-marks.xml` here is row 683-LD's m2; m0 checks that nothing has written to the book since.
- **Its vouchers** (row 683-TW's book-scoped answer, `a-twmvy` here): Sales 1, 2 and 3; Receipt 1, 2 and 3; Journal 1; Purchase 1 and 2; Payment 1 and 2, all dated 20250401. Rows 683-LD's, 683-VD's and 683-RO's restores each removed their Sales 4 and Receipt 4. **So Sales 4 and Receipt 4 are expected to be offered again** (see The write).
- **The current date and the period:** do not change either. Only K1's F2 on its voucher screen moves the current date (to 30-Apr-2025, as rows 1211, 683-LD, 683-VD and 683-RO recorded), and the restore's reload sets it back. This row has no F2 at the Gateway.
- **One BK, taken in this sitting before the write, and one restore after the last read** (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID must equal m0's (the ruling of 4 Nov 2026 on unchanged marks). It is also checked by the book-scoped window, which must again equal the book as found.

## Kit
Check every file against `SHA256.txt` first, and check each send's request file again just before it is sent (Get-FileHash, C3). If any file differs: STOP, and nothing is sent.

| file | what, and where its bytes come from |
|---|---|
| `M-marks.xml` | The marks request (GUID, ALTVCHID, ALTMSTID). Byte-equal to rows 683-TW's, 683-LD's, 683-CD's, 683-VD's and 683-RO's. |
| `T-tb3-20250401-20250401.xml` | Row 683-TW's `tb3` request, byte-equal (sha256 `1853d0df…0ad1a`): `render_native_trial_balance_request_with_currency` (`native_trial_balance/mod.rs:170`), from 20250401 to 20250401. Rows 683-LD, 683-CD, 683-VD and 683-RO sent the same file. Every Trial Balance of this row uses it. |
| `X-lx1-20250401.xml` | Row 683-LD's `x0` and `x1` request and row 683-RO's `x1`, byte-equal (sha256 `de2c64d4…bac1`, 1,162 B), which is row 683-OP's and row 683-LM's: `render_native_ledger_export_request` (`native_outstandings/request.rs:258`) for `from` 20250401, the opening read `ledger_movement` sends. |
| `V-mvy-20250401-20260331.xml` | Row 683-TW's `mvy` request, byte-equal: `render_agent_movement_vouchers` (`agent_read_profiles.rs:100`), book scope, 20250401 to 20260331. Here it checks the book before the BK and after the restore. |
| `a-twtb3-T-tb3-20250401-20250401.xml` | Not sent. Row 683-TW's `tb3` answer (`lab/683-tw-answers` 4f31b925, sha256 `1c4638e8…b7b7`, 14,140 B; six MIXED). Rows 683-VD's and 683-RO's `tb3a` were byte-equal to it. |
| `a-vdtb3w-T-tb3-20250401-20250401.xml` | Not sent. Row 683-VD's `tb3w` answer (`lab/683-vd-answers` db7a1313, 13,740 B, sha256 `f30cdee1…1bcb`; two MIXED, six PLAIN). Its `tb3v` and `tb3w2`, and row 683-RO's `tb3c` and `tb3x`, are byte-equal to it. |
| `a-ldtb3-T-tb3-20250401-20250401.xml` | Not sent. Row 683-LD's `tb3` answer (`lab/683-ld-answers` e38d2b82, sha256 `a12217b8…7705`, 13,528 B; all PLAIN). |
| `a-ldx1-X-lx1-20250401.xml` | Not sent. Row 683-LD's `x1` answer (sha256 `8d9ac402…ead6`, 11,556 B; 8 rows, each opening `0.00`), sent as the first read after its write, before its `tb3`. Row 683-RO's `x1` was byte-equal to it. |
| `a-twmvy-V-mvy-20250401-20260331.xml` | Not sent. Row 683-TW's `mvy` answer (sha256 `56554b6a…4df7`, 59,524 B); rows 683-LD's, 683-VD's and 683-RO's `mv0` and `mv2` are byte-equal to it. |
| `a-ldm2-M-marks.xml` | Not sent. Row 683-LD's m2, the marks after its restore (sha256 `3d75dd1a…dafba`, 3,404 B; byte-equal to its m0, to row 683-TW's m0, and to rows 683-VD's and 683-RO's m0 and m2). |
| `a-ldm1-M-marks.xml` | Not sent. Row 683-LD's m1, the marks with its two vouchers saved (sha256 `cee319cc…64d7`, 3,404 B; ALTVCHID 13, ALTMSTID 214; rows 683-VD's and 683-RO's m1 are byte-equal to it). |
| `run.py`, `LAB-COMMON.md`, `.gitattributes`, `lm.py`, `tb.py`, `cmpinfo.py` | Rows 683-VD's and 683-RO's, byte-equal. `cmpinfo.py` is row 683-LD's helper: it compares two answers byte for byte and, where they differ, says whether every difference lies inside the CMPINFO count block (C4: not rows), listing those counts. Exit 0 BYTE-EQUAL, 1 differing only inside CMPINFO, 2 differing outside it. It reads an opening export as it reads a Trial Balance (row 683-LD's `cmp-x1-x0`). |

The reference answers keep the `a-<label>-<K>-` form so that `lm.py` and `tb.py` read them by kind. Their labels (`twtb3`, `vdtb3w`, `ldtb3`, `ldx1`, `twmvy`, `ldm2`, `ldm1`) are not used for any send.

## The write
Row 683-LD's two vouchers, keyed as rows 683-LD's, 683-VD's and 683-RO's `RESULTS.md` (step 4) record them. The run lead makes them on Tally's screen, once, after the BK and the check before the write (step 4). **Type only the fields named, and leave every other field as Tally offers it (Enter only).** As row 683-LD found:
- **Type every name and amount as keystrokes, in short chunks, never as one paste.** In row 683-LD a pasted party name left the field empty and brought up a "Spelling Error" box; rows 683-VD and 683-RO typed every name in keystroke chunks and no box came up. Check the highlighted row before each Enter.
- no narration;
- a screenshot before each Ctrl+A.

Expected screens, none of them a STOP (each was seen in row 683-LD):
- the Change Voucher Date box on K1's F2;
- the Dispatch Details box and the Party Details box after K1's party, each passed by Enter on every field as offered;
- the Forex Rate of Exchange box where K1 says;
- the Bill-wise Details and Pending Bills the steps name;
- after K2's credit line, Tally offering `By` `Cash`, its ledger list with `Cash` highlighted, and the debit as offered.

If Tally adds a gain or loss line, press no key: screenshot it, write it down, and ask the run coordinator before Ctrl+A. Do not add one.

**Every STOP inside these steps means: press no key, screenshot the whole window, and report.**

**K1, F8 Sales** (from the Gateway, Vouchers, F8):
1. **Voucher No.:** write down what Tally offers. EXPECT `4`. If it offers any other number, STOP before any other key: `1`, `2` or `3` would reuse a number, and anything else means the book is not as its answers show.
2. **Date:** F2, type `30-4-2025`, Enter. Write down the date the box offered. The voucher must then show 30-Apr-25 Wednesday. If it shows anything else, or Tally answers with a message: STOP.
3. **Party A/c name:** `BRIDGE USD Customer`, typed in keystroke chunks, Enter on the one row listed. Dispatch Details and Party Details: Enter only. If a "Spelling Error" box comes up or the field stays empty: press no key, and ask. Row 683-LD cleared that box on the run lead's word by typing one `B` as a single keystroke, then `RIDGE`, ` USD` and ` Customer` as keystrokes; that is done here only on the run coordinator's word.
4. **Particulars:** `BRIDGE Export Sales`, Enter on the one row listed; the amount typed `$100`, Enter. Where the Forex Rate of Exchange box opens: Enter on the forex amount, `88` typed over the rate, Enter, the value as offered (8,800.00 expected), Enter. If no such box opens, or the line does not then show `$100.00 @ ₹ 88/$`: STOP before Ctrl+A.
5. **Bill-wise Details:** `New Ref`; Name typed `LD1`, replacing what is offered; Due Date empty; `$100.00` and `Dr` as offered. Write down each value shown.
6. **Screenshot the whole screen before Ctrl+A.** Write down the Voucher No., the date and each line as Tally shows it. Then Ctrl+A. If "Accept ?" is asked, Yes. Row 683-LD's Ctrl+A saved with no prompt; record which it was.

**K2, F6 Receipt** (from the Gateway, Vouchers, F6):
1. **Voucher No.:** EXPECT `4`, with the same STOP as K1's.
2. **Date:** if the screen does not already show 30-Apr-25, F2, type `30-4-2025`, Enter. Write down which it was. If it then shows anything else: STOP.
3. **To** `BRIDGE INR Customer`, **Credit** `2000`. If a Forex Rate of Exchange box opens: STOP, press nothing.
4. **Bill-wise Details:** `Agst Ref`, choose `R1` (the pending amount offered is expected to be 3,000.00 Dr), then the Amount typed `2000` Cr, replacing what is offered. If R1 is not in Pending Bills: STOP.
5. **By:** Tally offers `By` `Cash` (row 683-LD: "Cash was offered by Tally, not typed"). Enter on By; the ledger list opens with `Cash` highlighted: do not type it, Enter on it; **Debit** as offered (2,000.00 expected), Enter. If the highlighted row is not `Cash`: press no key, and ask.
6. **Screenshot the whole screen before Ctrl+A.** Write down every line. Then Ctrl+A, with Yes to "Accept ?" if it is asked; record which it was.

After each save, Tally shows a blank voucher screen. Screenshot its title, then leave it with Ctrl+Q (C7). Never press Esc on a voucher screen, and never press Ctrl+Q at the Gateway. After K2, screenshot the whole Tally window at the Gateway, and write down its current date and date of last entry (30-Apr-2025 and 30-Apr-25 expected), before `m1` is sent (C3).

After a STOP inside these steps, nothing of that voucher has been saved, but K1 may have been. Press no key: the run coordinator says how the screen is left. The restore then runs as step 8 says.

## Screens and prompts
The screen paths for load and close are those of rows 683-TW, 683-LD, 683-CD, 683-VD and 683-RO:
- **Load:** Select Company, the book typed into its name filter, the one row, Enter.
- **A company already loaded at the start:** C1 says STOP. If the run coordinator's word is to shut it, shut it by the K: Company label, Shut, Enter on it, with no change to it.
- **Close (C10):** the K: Company label, Shut, Enter on the book.
- **Bring Tally to the front only by its running window.** A second `tally.exe` is a STOP.

**The prompt rule.** The only dialogs answered in this row are:
- C5's: a message that only says the backup is done (screenshot it, accept once), and the backup's "Accept ?" = Yes;
- C6's two: "Accept ?" = Yes, and "Company data folder (<the book's number>) already exists ..." = Replace;
- each voucher's own "Accept ?" on Ctrl+A = Yes, if it is asked;
- the screens the write names: Change Voucher Date (F2), Dispatch Details, Party Details, the Forex Rate of Exchange box, Bill-wise Details, Pending Bills, and the ledger and party lists of K1 and K2.

**Any other dialog, message box, sub-screen or prompt, at any step, is unnamed:** press no key, screenshot the whole window, copy its text exactly, and send nothing while it is up. Then ask the run coordinator (C7). It stays on the screen until the run coordinator answers.

**The path rule.** If a list or line shown is not exactly what a path above names (other words, no such line, a Spelling Error), send nothing and ask:
- **a list or box reached from the Gateway** (Select Company, the Shut list, the Backup or Restore list): Esc out of it;
- **a list on a voucher screen** (the party, ledger or Pending Bills list of K1 or K2): press no key. Never press Esc on a voucher or master screen (C7).

## The wedge rule
A read that Tally is still building keeps the gateway busy for every company, and giving up on it does not stop Tally's work. So:
- **One request at a time, by `run.py`.** It never abandons a request: curl waits for its answer.
- **The book is small:** 13 vouchers and 8 ledgers after the write. This row's largest answer is the book-scoped window, 59,524 B as found; it is read only before the BK and after the restore. The opening export is 11,556 B and the Trial Balance to 1 April at most 14,140 B. Every read of rows 683-LD, 683-VD and 683-RO answered within about a second.
- **If any send has not answered 5 minutes after it began:**
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).

  Nothing is re-sent on this row's own judgement. With the vouchers saved and the gateway stuck, the restore waits for the run coordinator's word.
- **Failed requests:** `run.py`'s own stops (a curl exit other than 0, Unknown Request, an empty answer, an answer above 2,000,000 B), and the STOP of `lm.py` or `tb.py`. Each is a failed request, never "nothing found".

## Steps
Run each send as `python3 -I run.py http://127.0.0.1:9001 <label> <file>`, and write down each send's start time from its log. **Each label is used once.** A repeated send takes a new label (as `tb3f2` repeats `tb3f`), never an earlier one's. The answer of send `<label>` is `a-<label>-<request file>`.

0. **C2's step 0, with no company loaded:**
   - the screenshot first;
   - one `tally.exe` owning 9001;
   - `GET /status` 200 (record its bytes).

   Load the book alone, as above. Read the Gateway's current date, period and date of last entry, and write them down. Record the release and the licence mode, as row 683-VD recorded them, and the PC's date.
   - EXPECT current date Tuesday, 1-Apr-2025, period 1-Apr-25 to 31-Mar-26, last entry 1-Apr-25.
   - If any differs: record it and STOP before any send. Press nothing to change it. The run coordinator decides.
1. **`m0`:** `M-marks.xml`, then `python3 -I lm.py --same-row a-ldm2-M-marks.xml a-m0-M-marks.xml`. EXPECT SAME MARKS: one COMPANY row, the book's, with ALTVCHID 11 and ALTMSTID 214.
   - Any other GUID or row count: STOP.
   - MARKS DIFFER: record it and STOP before step 2. The book changed after its last note, and its voucher numbers are then not known.
2. **The book as found** (the controls, as row 683-VD's step 2), one send at a time:
   - **`tb3a`:** `T-tb3-20250401-20250401.xml`. Record whether its answer is byte-equal to `a-twtb3-T-tb3-20250401-20250401.xml` (sha256). Then `python3 -I tb.py --sets a-twtb3-T-tb3-20250401-20250401.xml a-tb3a-T-tb3-20250401-20250401.xml`. EXPECT byte-equal, and SETS SAME.
   - **`mv0`:** `V-mvy-20250401-20260331.xml`. Then `python3 -I lm.py --capture a-mv0-V-mvy-20250401-20260331.xml a-twmvy-V-mvy-20250401-20260331.xml`. EXPECT CAPTURE SAME, with `vouchers: movement 11, capture 11`. Record whether `a-mv0` is byte-equal to `a-twmvy`.
   - If either is not as EXPECT: record it, and STOP before the BK. The book, or the script, reads differently from the rows before, and a write on it would decide nothing.
   - **No opening export is sent before the write.** Rows 683-VD and 683-RO sent none there either. Row 683-LD's `x0` and Trial Balance to 30 April before its write are not repeated (see Not in this row).
3. **BK**, by C5, to a new empty local folder named `bk-683xf`. Record the file's name, size and sha256, and the time the Restore list will show. Then **`mb`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m0-M-marks.xml a-mb-M-marks.xml`. Record its line. A difference is a result to record (C5), not a STOP.

   **The restore's word, given now (C8).** C8 restores a backup after a STOP only on the run coordinator's word. For this row that word is given in advance: from here on, step 8 runs after the last read, after any STOP at any later step, and after a voucher saved wrongly. Nothing else is sent, keyed or backed up after a STOP: only the restore and step 8's state reads, `m2` and `mv2`.

   The word waits, and the run coordinator is asked first, in three cases: an unnamed dialog still up, a send past the wedge rule's 5 minutes, or Tally closed (C9). Each waits for the word on how to clear it, and then the restore runs.
4. **The write.**
   - First **`mc`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-mb-M-marks.xml a-mc-M-marks.xml`. Go on only on SAME MARKS, and only with no key pressed in Tally since `mb`. Otherwise go to step 8.
   - Then K1 and K2, by **The write** above.
5. **`m1`:** `M-marks.xml`, then:
   - `python3 -I lm.py --same-row a-mc-M-marks.xml a-m1-M-marks.xml`. EXPECT MARKS DIFFER (exit 2), with ALTVCHID 13 and ALTMSTID 214.
   - `python3 -I lm.py --same-row a-ldm1-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS: the write moved the marks as rows 683-LD's, 683-VD's and 683-RO's did.
   - Anything else: record it, and go on with step 6. The reads are still taken, and the restore puts the book back.
6. **The opening export, then the Trial Balance to 1 April twice,** one send at a time, with the current date at 30-Apr-2025 as the write left it. **`x1` is the first send after `m1`.** Run each answer's counters before the next send, and send nothing else in between: no marks read, no `GET /status`, and no key in Tally. A failed request here (`run.py`'s own stop, or a STOP of `lm.py` or `tb.py`, which exits 1) ends step 6: send nothing more, and go to step 8 on the word step 3 gives. A ledger that `tb.py` lists under SET UNREAD is not such a STOP: it is a result, and the next send goes ahead.
   - **`x1`** (the one variable, in row 683-LD's place): `X-lx1-20250401.xml`. Then `python3 -I cmpinfo.py a-ldx1-X-lx1-20250401.xml a-x1-X-lx1-20250401.xml` and `python3 -I lm.py a-x1-X-lx1-20250401.xml`, each recorded whole. EXPECT 8 rows, each opening `0.00`.
   - **`tb3f`** (the question): `T-tb3-20250401-20250401.xml`. Then, at once, screenshot the whole Tally window. A dialog is the prompt rule's Stop: press no key, and the restore waits for the run coordinator's word (the first of step 3's three cases), not the word given in advance. Then, each recorded whole:
     - `python3 -I cmpinfo.py a-ldtb3-T-tb3-20250401-20250401.xml a-tb3f-T-tb3-20250401-20250401.xml`;
     - `python3 -I cmpinfo.py a-vdtb3w-T-tb3-20250401-20250401.xml a-tb3f-T-tb3-20250401-20250401.xml`;
     - `python3 -I tb.py a-tb3f-T-tb3-20250401-20250401.xml`;
     - `python3 -I tb.py --sets a-ldtb3-T-tb3-20250401-20250401.xml a-tb3f-T-tb3-20250401-20250401.xml`;
     - `python3 -I tb.py --sets a-vdtb3w-T-tb3-20250401-20250401.xml a-tb3f-T-tb3-20250401-20250401.xml`.
   - **`tb3f2`** (the repeat, sent whatever `tb3f` read): `T-tb3-20250401-20250401.xml`. Then `python3 -I cmpinfo.py a-tb3f-T-tb3-20250401-20250401.xml a-tb3f2-T-tb3-20250401-20250401.xml` and `python3 -I tb.py --sets a-tb3f-T-tb3-20250401-20250401.xml a-tb3f2-T-tb3-20250401-20250401.xml`, each recorded whole.
7. **`m1r`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m1-M-marks.xml a-m1r-M-marks.xml`. EXPECT SAME MARKS, since the reads wrote nothing. On MARKS DIFFER, record it, send nothing else but the restore, and go to step 8.
8. **The restore,** from the BK of step 3, by C6:
   - check the backup file's sha256 against the one recorded, and write `restore-plan.txt`;
   - shut the company;
   - restore the BK's folder alone;
   - answer only "Accept ?" (Yes) and "Company data folder (<the book's number>) already exists ..." (Replace);
   - load the company alone, with the screenshots C6 names.

   Then:
   - **`m2`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m0-M-marks.xml a-m2-M-marks.xml`. EXPECT SAME MARKS.
   - **`mv2`:** `V-mvy-20250401-20260331.xml`, and `python3 -I lm.py --capture a-mv2-V-mvy-20250401-20260331.xml a-mv0-V-mvy-20250401-20260331.xml`. EXPECT CAPTURE SAME, with 11 and 11. Record whether `a-mv2` is byte-equal to `a-mv0`.
   - MARKS DIFFER or CAPTURE DIFFERS: STOP, and report "the restore of bk-683xf did not return the book to m0". Nothing more is sent or keyed.
9. **Tally's date:** read the Gateway's current date, period and date of last entry, and write them down. EXPECT 1-Apr-2025, 1-Apr-25 to 31-Mar-26 and 1-Apr-25, as at step 0 (rows 683-LD's, 683-VD's and 683-RO's restores left them so). If any differs, press nothing to change it: record it and report it before step 10. The run coordinator says what is done first; step 10 runs on that word.
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

From step 3 on, a STOP is followed by the restore (step 8) on the word step 3 gives, except in the three cases step 3 names. If the BK itself fails or is not recorded, nothing has been written yet: STOP there, with no restore, and close by C10 on the run coordinator's word.

## Record
- Each answer's bytes and sha256, and each request's sha256 (as `SHA256.txt` lists; any other is the Stop of C3).
- `lm.py`'s lines for every marks read, and the whole output of each `lm.py`, `tb.py` and `cmpinfo.py` call.
- Whether `tb3a` is byte-equal to `a-twtb3`, and each `cmpinfo.py` line. Either way it is recorded, not judged. Where two Trial Balances differ, also list the fields that differ, row by row, as row 683-LD's `evidence/tb3-fields-vs-683tw.txt` did.
- **Each send's start time and how long it took, from `log-<label>.txt`, and the gap in seconds between the end of each send and the start of the next.** Row 683-LD sent its `x1` about 10 s after `m1` answered and its `tb3` about 58 s after `x1` answered; those gaps are recorded here, not set. Record also the time from K2's save to `tb3f`'s send (about 2 minutes in row 683-LD).
- **The write:** every value written down in its steps, word for word as the screen shows it:
  - each Voucher No. offered and the date shown;
  - each line and amount;
  - the rate box's title and fields;
  - every field of each Bill-wise line, with the pending amount R1 showed;
  - every prompt, with the key that answered it, and each save that asked none.
- Tally's current date, period and date of last entry at steps 0, 4 (after K2) and 9, with the date K1's F2 box offered.
- The BK file (name, folder, bytes, sha256, and the Restore list's time), `restore-plan.txt`, the data-folder listings before and after the restore, and every prompt of the restore word for word.
- The marks at m0, mb, mc, m1, m1r and m2.
- The release, the licence mode, and the PC's date.
- Every screenshot by name, bytes and sha256 (screenshots are never pushed).

**Staging the evidence**, as for rows 683-LM's, 683-TW's, 683-LD's, 683-CD's, 683-VD's and 683-RO's:
- **The start-up check:** a hit is allowed only where it lies inside the amount `$1000000.00`. List each hit with 12 characters either side. A hit anywhere else waits for the run coordinator.
- **The check for 10 or more digits:** it sets aside this book's registered GUID matched whole, and nothing else.

## Expected (estimates, not EXPECTs)
- **The controls:** `tb3a` byte-equal to `a-twtb3`, and `mv0` to `a-twmvy`.
- **The write:** Sales No. 4 and Receipt No. 4, as rows 683-LD's, 683-VD's and 683-RO's; m1 SAME MARKS with `a-ldm1`.
- **`x1`:** byte-equal to `a-ldx1`, as row 683-RO's was; 8 rows, each `0.00`.
- **`tb3f`:** not known. That is the question.
- **`tb3f2`:** byte-equal to `tb3f`, as every repeated Trial Balance in a sitting on this book has been.
- **The restore:** m2 and mv2 as m0 and mv0, and the Gateway at 1-Apr-2025 after the reload, as rows 683-LD's, 683-VD's and 683-RO's.

## Decides
Below, **as** an answer means SETS SAME against it **and** `cmpinfo.py` exit 0 or 1 against it. Anything else is **not as** that answer; an answer with SETS SAME but exit 2 against both references is "neither". **The `x1` case below overrides the others:** if `x1` is not as row 683-LD's, that line decides, whatever `tb3f` reads. "Row 683-VD's" means `a-vdtb3w`, which rows 683-VD and 683-RO read five times.

- **`tb3f` as row 683-LD's (every ledger PLAIN), `tb3f2` as `tb3f`:** with these vouchers saved, the opening export sent before the first Trial Balance after the write turns the two MIXED ledgers plain, while sent after one (row 683-RO) it changes nothing. So on this book and release the classification is fixed by what the sitting reads first after the write, and stays fixed: PARTIAL. The #683 build (#2470 at 7d532165) sends its opening read at `from` first and its Trial Balance after it (`agent_movement.rs:160-167`, then `:206`), the same order of kinds. Its Trial Balance is over the movement window with the base-currency scope, not this row's one-day `tb3`, so whether the build's own reads move the same way is not measured here. If they do, only the build's check of the Trial Balance against the composite openings and entries (`ledger_currency_scope_inconclusive`) stands between a plain answer and a dollar ledger read as plain. That goes to #683 first; nothing is built on it here.
- **`tb3f` as row 683-VD's (two MIXED), `tb3f2` as `tb3f`:** the opening export does not explain row 683-LD's all-plain `tb3` in either order. What still differs from row 683-LD is recorded, and none of it is assumed to be the cause: the reads before the write (row 683-LD's `x0` and its Trial Balance to 30 April, where this row sends `tb3a` as rows 683-VD and 683-RO did), and the keying of K1 (row 683-LD's met a "Spelling Error" box and cleared it; rows 683-VD's and 683-RO's met none, and every m1 since is byte-equal to row 683-LD's). Each candidate is its own row.
- **`tb3f` neither** (some other mix of MIXED and PLAIN, other text, or any UNREAD), with `tb3f2` as `tb3f`: the export first moved the answer, but not to row 683-LD's. Record both whole; they decide nothing until the run coordinator has read them.
- **`tb3f2` not as `tb3f`:** the Trial Balance did not repeat within the sitting, for the first time on this book. Record both whole; `tb3f` decides nothing until that is explained (P5).
- **`x1` not as row 683-LD's `x1`** (`cmpinfo.py` exit 2 against `a-ldx1`, or any row count or opening other than 8 rows of `0.00`): the variable was not the one row 683-LD sent. `tb3f` is recorded, but decides nothing about row 683-LD's answer until that is explained.
- **`tb3a` not as row 683-TW's:** the book or the sitting reads differently from the rows before; the row stopped before the BK (step 2).

Whichever way it falls, any answer that becomes a fixture is parsed again by master's own parser first, before any code relies on it.

## Not in this row
- Any Trial Balance other than the one to 1 April, any movement window beyond the book-scoped checks, and any opening export other than row 683-LD's at 1 April.
- Row 683-LD's reads before the write (`x0`, and its Trial Balance to 30 April `tb10`). Each would be a second variable here.
- A Trial Balance before `x1` after the write: row 683-RO sent that order.
- Any current date other than the 30-Apr-2025 the write leaves, a voucher on any day other than 30 April, and any change to the period.
- A wait set between sends; the gaps are recorded only.
- `trial_balance` or `ledger_movement` through Bridge, another book, and another release.
