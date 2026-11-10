# Row 683-TN: on the several-currency lab book, with row 683-NO's reads and the Trial Balance to 30 April read once before the write (in place of row 683-TA's one-day Trial Balance), do two ledgers read MIXED in the Trial Balance to 1 April after the same two vouchers are saved

Not for merge. Part of #683. **Two writes, behind a backup:** row 683-LD's two vouchers are keyed again on screen, as rows 683-VD, 683-RO, 683-XF, 683-PW, 683-XA, 683-NO and 683-TA keyed them, the reads are taken, and the book is restored. The run coordinator gives the word for that restore now, under C8, for every outcome from the BK on (step 3 says when it waits). Lines cited are at master 8a4ba410.

This row's answers are staged under `lab/683-tn-answers/`, apart from every earlier row's, so that no label is used twice. Its backup goes to a new empty folder, `bk-683tn`.

## Question
The Trial Balance from 1 April 2025 to 1 April 2025 (`tb3`, request sha256 `1853d0df…0ad1a`) has given three answers on this book:
- **Rows 683-TW and 683-CD,** the book as found: 14,140 B, sha256 `1c4638e8…b7b7`. Six ledgers MIXED (each reading the dollar composite) and two PLAIN, whatever the current date.
- **Rows 683-LD, 683-PW, 683-XA and 683-NO,** with two vouchers dated 30 April saved: 13,528 B, sha256 `a12217b8…7705`, byte-equal in all four. Every ledger PLAIN, each rupee figure the same.
- **Rows 683-VD, 683-RO, 683-XF (second sitting) and 683-TA,** with the same two vouchers keyed again: 13,740 B, sha256 `f30cdee1…1bcb`, nine times over. `BRIDGE Import Purchases` and `BRIDGE Lab Bank` MIXED, the other six PLAIN; every rupee figure the same.

What the sitting read before the write is what has differed between the two answers after it:
- **Rows 683-VD and 683-RO** (two MIXED): `m0`, the one-day Trial Balance `tb3a`, then `mv0`.
- **Row 683-XF's second sitting and row 683-TA** (`lab/683-xf2-answers` 27416b50 and `lab/683-ta-answers` 031ca50c; two MIXED): `m0`, `tb3a`, its repeat `tb3a2`, then `mv0`.
- **Rows 683-LD and 683-PW** (every ledger PLAIN): `m0`, the opening export `x0`, the Trial Balance to 30 April `tb10`, then `mv0`.
- **Row 683-XA** (every ledger PLAIN): `m0`, `x0`, then `mv0`.
- **Row 683-NO** (`lab/683-no-answers` 791db6bc; every ledger PLAIN): `m0`, then `mv0`.

Row 683-TA confirmed against row 683-NO, one variable apart (that read, sent twice), that the one-day Trial Balance read before the write keeps the two ledgers composite in the same request after it. Before the write the book has no voucher after 1 April, so the Trial Balance to 30 April answers with the one-day Trial Balance's bytes there (`a-ldtb10` and `a-twtb3` are both sha256 `1c4638e8…b7b7`). The request differs only in its end date.

This row asks whether that request, read before the write, does the same (P6): before the write, `m0`, `tb10` once, then `mv0`. It is row 683-NO with that one read added. Rows 683-LD and 683-PW read `tb10` before the write too, but after the opening export, and read plain. It repeats row 683-NO word for word otherwise, and the reads after the write are row 683-NO's (`x1`, `tb3p`, `tb3p2`, `tb1p`).

**Q1:** with `tb10` read before the write, does `tb3p` read as row 683-TA's (two MIXED, six PLAIN) or as row 683-NO's (every ledger PLAIN)? `tb.py --sets` answers it in one line against each.

**Repeated (P5):** `tb3` is sent once more (`tb3p2`), and must read as `tb3p`.

**Q2 (after the question's reads):** does `tb1p`, the Trial Balance to 30 April, read as row 683-LD's `tb1` and rows 683-PW's, 683-XA's, 683-NO's and 683-TA's `tb1p` (six MIXED, two PLAIN, 14,140 B, sha256 `47916021…d237f`)? Here the same request was read before the write.

**Timing is recorded, not set.** Row 683-NO sent `mv0` 1.4 s after `m0` answered; after the write, `m1` 15.4 s after K2's save, `x1` 2.7 s after `m1` answered and `tb3p` 2.5 s after `x1` answered, 22 s after K2's save. Row 683-TA's were 81.3 s, 1.7 s, 1.9 s and 86 s, after a pause of the operator's tools with nothing sent. Here the gaps are those of running each answer's counters before the next send.

**The controls, before the BK, on the book as found:**
- **`tb10`,** EXPECT the content of row 683-LD's `tb10` (`a-ldtb10`): `tb.py --sets` SETS SAME and `rows.py` CONTENT SAME. Row 683-PW's `tb10` was byte-equal to it.
- **`mv0`,** EXPECT `lm.py --capture` CAPTURE SAME with row 683-TW's `mvy`. The restore is checked against `mv0`. Rows 683-PW's, 683-XA's, 683-NO's and 683-TA's `mv0` were byte-equal to it.

Byte equality with the reference is recorded, and is not an EXPECT.
- **After the write, `x1`** is compared with row 683-LD's `x1` answer by content (`rows.py`), with byte equality recorded, as in rows 683-XF (second sitting), 683-PW, 683-XA, 683-NO and 683-TA, where it was byte-equal. Here it is the first opening export of the sitting.

## Element order
Row 683-XF's first sitting (`lab/683-xf-answers` at 9eeeb1a7) read a one-day Trial Balance whose rows held every value of row 683-TW's, with `TBALOPENING` written before `TBALCLOSING` instead of after it, under a Tally start of its own. Its second sitting, under another Tally start, read row 683-TW's order in all four of its Trial Balances. Master's Trial Balance parse reads each child of a ledger row by its name and sets each field once (`native_trial_balance/wire.rs:307` `parse_row`, `TBALOPENING` at `:347`, `TBALCLOSING` at `:362`, `set_once` in `scalar.rs:37`), so that order changes nothing it returns. This row asks how the dollar ledgers read, which is a question of values, so every EXPECT is by content, and byte equality and element order are recorded for every Trial Balance and opening export. In Bridge, the paired read compares its two answers byte for byte (`tally/connection.rs:3193`), so an order that changed between the two halves of a pair would be refused as drift. This row reads one request twice in a row only after the write (`tb3p`, `tb3p2`), and the Trial Balance to 30 April once before the write (`tb10`) and once after it (`tb1p`); whether the order holds is recorded.

Every request is master's own builder, sent raw, and each has been sent on this book before (rows 683-LM, 683-OP, 683-TW, 683-LD, 683-VD, 683-RO and 683-XF). No request is new and nothing is rendered for this row.

## Book
`BRIDGE FOREXGROUP LAB` (synthetic), GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, TallyPrime 7.2. THE COMPANY in `LAB-COMMON.md` means this book.
- **Synthetic:** it is in master's registry, `scripts/merge-gate-synthetic-uuids.json` ("created in the lab on 28 October 2026 with invented masters and vouchers keyed by hand").
- **Not an excluded book:** its GUID is not the register lab book's (`REGISTER_LAB_GUID`, `captured_parse_record.rs:302`), and not the one the private exclusion list names.
- **Pinned by GUID.** The `BOOK` of `lm.py` and `tb.py` is this GUID. m0 must print one COMPANY row, `the book's: yes`, or the row STOPs before any other send.
- **Its last state:** row 683-TA (`lab/683-ta-answers` 031ca50c) restored it from its backup, then its m2 was SAME MARKS with its m0 and with row 683-LD's m2 (V 11, M 214), and its book-scoped window byte-equal to its `mv0`. Nothing has been loaded or sent on this book since. `a-ldm2-M-marks.xml` here is row 683-LD's m2; m0 checks that nothing has written to the book since.
- **Its vouchers** (row 683-TW's book-scoped answer, `a-twmvy` here): Sales 1, 2 and 3; Receipt 1, 2 and 3; Journal 1; Purchase 1 and 2; Payment 1 and 2, all dated 20250401. Every earlier restore removed its Sales 4 and Receipt 4. **So Sales 4 and Receipt 4 are expected to be offered again** (see The write).
- **The current date and the period:** do not change either. Only K1's F2 on its voucher screen moves the current date (to 30-Apr-2025, as rows 1211, 683-LD, 683-VD, 683-RO and 683-XF recorded), and the restore's reload sets it back. This row has no F2 at the Gateway.
- **One BK, taken in this sitting before the write, and one restore after the last read** (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID must equal m0's (the ruling of 4 Nov 2026 on unchanged marks). It is also checked by the book-scoped window, which must again equal the book as found.

## Kit
Check every file against `SHA256.txt` first, and check each send's request file again just before it is sent (Get-FileHash, C3). If any file differs: STOP, and nothing is sent.

| file | what, and where its bytes come from |
|---|---|
| `M-marks.xml` | The marks request (GUID, ALTVCHID, ALTMSTID). Byte-equal to rows 683-TW's, 683-LD's, 683-CD's, 683-VD's, 683-RO's and 683-XF's. |
| `T-tb3-20250401-20250401.xml` | Row 683-TW's `tb3` request, byte-equal (sha256 `1853d0df…0ad1a`): `render_native_trial_balance_request_with_currency` (`native_trial_balance/mod.rs:170`), from 20250401 to 20250401. Rows 683-LD, 683-CD, 683-VD, 683-RO, 683-XF, 683-PW, 683-XA, 683-NO and 683-TA sent the same file. `tb3p` and `tb3p2` use it. |
| `T-tb1-20250401-20250430.xml` | Row 683-TW's `tb1` request, byte-equal (sha256 `d3ca2b38…a1076e`): the same builder, from 20250401 to 20250430. Rows 683-LD and 683-PW sent it as `tb10` before the write and after it (`tb1`, `tb1p`). Here `tb10` and `tb1p` use it. |
| `X-lx1-20250401.xml` | Row 683-LD's `x0` and `x1` request, byte-equal (sha256 `de2c64d4…bac1`, 1,162 B), which is row 683-OP's and row 683-LM's: `render_native_ledger_export_request` (`native_outstandings/request.rs:258`) for `from` 20250401, the opening read `ledger_movement` sends. Only `x1` uses it here. |
| `V-mvy-20250401-20260331.xml` | Row 683-TW's `mvy` request, byte-equal: `render_agent_movement_vouchers` (`agent_read_profiles.rs:100`), book scope, 20250401 to 20260331. Here it checks the book before the BK and after the restore. |
| `a-ldx0-X-lx1-20250401.xml` | Not sent, and used by no command here (no `x0` is sent); kept so that the kit stays the earlier rows'. Row 683-LD's `x0` answer (`lab/683-ld-answers` e38d2b82, sha256 `898ad76b…ce3`, 11,556 B; 8 rows, each opening `0.00`), byte-equal to row 683-LM's `lx1` and to rows 683-PW's and 683-XA's `x0`. |
| `a-ldtb10-T-tb1-20250401-20250430.xml` | Not sent. Row 683-LD's `tb10` answer (sha256 `1c4638e8…b7b7`, 14,140 B; six MIXED, two PLAIN), byte-equal to row 683-TW's `tb1` and `tb3` and to row 683-PW's `tb10`. Here it is the reference for `tb10` at step 2, by content, and `tb1p`'s recorded reference for the book before the write. |
| `a-ldx1-X-lx1-20250401.xml` | Not sent. Row 683-LD's `x1` answer (sha256 `8d9ac402…ead6`, 11,556 B; 8 rows, each opening `0.00`; it differs from `a-ldx0` only inside the CMPINFO count block). Rows 683-RO's and 683-XF's `x1` were byte-equal to it. |
| `a-ldtb3-T-tb3-20250401-20250401.xml` | Not sent. Row 683-LD's `tb3` answer (sha256 `a12217b8…7705`, 13,528 B; all PLAIN). |
| `a-vdtb3w-T-tb3-20250401-20250401.xml` | Not sent. Row 683-VD's `tb3w` answer (`lab/683-vd-answers` db7a1313, 13,740 B, sha256 `f30cdee1…1bcb`; two MIXED, six PLAIN). Rows 683-VD's, 683-RO's and 683-XF's other five are byte-equal to it. |
| `a-twtb3-T-tb3-20250401-20250401.xml` | Not sent. Row 683-TW's `tb3` answer (`lab/683-tw-answers` 4f31b925, sha256 `1c4638e8…b7b7`, 14,140 B; six MIXED). Here it is the reference for the element order of `tb3p`: `rows.py` against it prints the order pair when they differ. |
| `a-ldtb1-T-tb1-20250401-20250430.xml` | Not sent. Row 683-LD's `tb1` answer, read after its write and its `tb3` (sha256 `47916021…d237f`, 14,140 B; six MIXED, two PLAIN, SETS SAME with its `tb10`). |
| `a-twmvy-V-mvy-20250401-20260331.xml` | Not sent. Row 683-TW's `mvy` answer (sha256 `56554b6a…4df7`, 59,524 B); every later row's `mv0` and `mv2` are byte-equal to it. |
| `a-ldm2-M-marks.xml` | Not sent. Row 683-LD's m2, the marks after its restore (sha256 `3d75dd1a…dafba`, 3,404 B; byte-equal to its m0, to row 683-TW's m0, and to every later row's m0 and m2). |
| `a-ldm1-M-marks.xml` | Not sent. Row 683-LD's m1, the marks with its two vouchers saved (sha256 `cee319cc…64d7`, 3,404 B; ALTVCHID 13, ALTMSTID 214; rows 683-VD's, 683-RO's and 683-XF's m1 are byte-equal to it). |
| `run.py`, `LAB-COMMON.md`, `.gitattributes`, `lm.py`, `tb.py`, `cmpinfo.py`, `rows.py` | Rows 683-XF's second sitting's and 683-PW's, byte-equal. `cmpinfo.py` compares two answers byte for byte and, where they differ, says whether every difference lies inside the CMPINFO count block, listing those counts (exit 0 BYTE-EQUAL, 1 differing only inside CMPINFO, 2 differing outside it). `rows.py` compares two answers of one request by content: each LEDGER row by its GUID, every element below it as a set, the row order and the element order reported apart (exit 0 CONTENT SAME with both orders SAME; 3 CONTENT SAME with an order that DIFFERS; 2 CONTENT DIFFER; 1 STOP). Both read a Trial Balance and an opening export alike. |
| `prep-c6.sh` | C6's prep before the restore, row 683-PW's, byte-equal (it ran there at step 8 with exit 0; its header comment names row 683-PW's own folder as the example). It is row 683-XF's second sitting's with one change: the BK folder is its second argument (here `bash prep-c6.sh 8 bk-683tn`), checked to be `bk-683` followed by lower-case letters or digits, and every line that names the folder uses it. Row 683-XF's copy wrote its own folder in the script. The two lab paths, written into row 683-XF's copy on the lab PC, now come from `LAB_DIR` and `TALLY_DATA`, given on the command line at step 8, so this file runs as listed in `SHA256.txt`. It reads the BK's line in `evidence/bk.txt` in the form step 3 gives, and stops on any other. It sends nothing to 9001 and presses no key. |

The reference answers keep the `a-<label>-<K>-` form so that `lm.py` and `tb.py` read them by kind. Their labels (`ldx0`, `ldtb10`, `ldx1`, `ldtb3`, `vdtb3w`, `twtb3`, `ldtb1`, `twmvy`, `ldm2`, `ldm1`) are not used for any send.

**The kit was checked before this sheet was sent:** every kit file but this one is byte-equal to rows 683-PW's, 683-XA's, 683-NO's and 683-TA's (`lab/683-pw-sheet` ad26232a, `lab/683-xa-sheet` 1859472a, `lab/683-no-sheet` d56f8d67, `lab/683-ta-sheet` cf9d799b), every reference answer's sha256 equals its source (`lab/683-xf-sheet` 3b543335, `lab/683-ld-sheet` c8db1931, `lab/683-ld-answers` e38d2b82), and `prep-c6.sh` refuses a missing folder argument, one that is not `bk-683` followed by lower-case letters or digits, and unset lab paths, before it reads anything. It does not tell `bk-683tn` from an earlier row's `bk-683` folder by name: the BK's sha256 against `evidence/bk.txt` is what catches a wrong backup.

## The write
Row 683-LD's two vouchers, keyed as rows 683-LD's, 683-VD's, 683-RO's and 683-XF's `RESULTS.md` (step 4) record them. The run lead makes them on Tally's screen, once, after the BK and the check before the write (step 4). **Type only the fields named, and leave every other field as Tally offers it (Enter only).** As row 683-LD found:
- **Type every name and amount as keystrokes, in short chunks, never as one paste.** In row 683-LD a pasted party name left the field empty and brought up a "Spelling Error" box; rows 683-VD, 683-RO and 683-XF typed every name in keystroke chunks and no box came up. Check the highlighted row before each Enter.
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
The screen paths for load and close are those of rows 683-TW, 683-LD, 683-CD, 683-VD, 683-RO and 683-XF:
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
- **The book is small:** 13 vouchers and 8 ledgers after the write. This row's largest answer is the book-scoped window, 59,524 B as found; it is read only before the BK and after the restore. The opening export is 11,556 B and each Trial Balance at most 14,140 B. Every read of rows 683-LD, 683-VD, 683-RO and 683-XF answered within about a second, row 683-LD's `tb10` and `tb1` included.
- **If any send has not answered 5 minutes after it began:**
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).

  Nothing is re-sent on this row's own judgement. With the vouchers saved and the gateway stuck, the restore waits for the run coordinator's word.
- **Failed requests:** `run.py`'s own stops (a curl exit other than 0, Unknown Request, an empty answer, an answer above 2,000,000 B), and the STOP of `lm.py`, `tb.py` or `rows.py`. Each is a failed request, never "nothing found".

## Steps
Run each send as `python3 -I run.py http://127.0.0.1:9001 <label> <file>`, and write down each send's start time from its log. **Each label is used once.** A repeated send takes a new label (as `tb3p2` repeats `tb3p`), never an earlier one's. The answer of send `<label>` is `a-<label>-<request file>`.

0. **C2's step 0, with no company loaded:**
   - the screenshot first;
   - one `tally.exe` owning 9001;
   - `GET /status` 200 (record its bytes).

   Load the book alone, as above. Read the Gateway's current date, period and date of last entry, and write them down. Record the release and the licence mode, as row 683-XF recorded them, and the PC's date.
   - EXPECT current date Tuesday, 1-Apr-2025, period 1-Apr-25 to 31-Mar-26, last entry 1-Apr-25.
   - If any differs: record it and STOP before any send. Press nothing to change it. The run coordinator decides.
1. **`m0`:** `M-marks.xml`, then `python3 -I lm.py --same-row a-ldm2-M-marks.xml a-m0-M-marks.xml`. EXPECT SAME MARKS: one COMPANY row, the book's, with ALTVCHID 11 and ALTMSTID 214.
   - Any other GUID or row count: STOP.
   - MARKS DIFFER: record it and STOP before step 2. The book changed after its last note, and its voucher numbers are then not known.
2. **The book as found** (the controls, as row 683-PW's step 2 less `x0`), one send at a time, `tb10` straight after `m0`'s counters and `mv0` straight after `tb10`'s:
   - **`tb10`:** `T-tb1-20250401-20250430.xml`. Then, each recorded whole:
     - `python3 -I tb.py --sets a-ldtb10-T-tb1-20250401-20250430.xml a-tb10-T-tb1-20250401-20250430.xml`. EXPECT SETS SAME.
     - `python3 -I rows.py a-ldtb10-T-tb1-20250401-20250430.xml a-tb10-T-tb1-20250401-20250430.xml`. EXPECT CONTENT SAME (exit 0 or 3). Its ELEMENT ORDER lines say which order this sitting's Tally writes.
     - `python3 -I cmpinfo.py a-ldtb10-T-tb1-20250401-20250430.xml a-tb10-T-tb1-20250401-20250430.xml`. Recorded, not EXPECTed.
   - **`mv0`:** `V-mvy-20250401-20260331.xml`. Then `python3 -I lm.py --capture a-mv0-V-mvy-20250401-20260331.xml a-twmvy-V-mvy-20250401-20260331.xml`. EXPECT CAPTURE SAME, with `vouchers: movement 11, capture 11`. Record whether `a-mv0` is byte-equal to `a-twmvy`, and that is not an EXPECT.
   - If any EXPECT here fails (SETS DIFFER, CONTENT DIFFER, CAPTURE DIFFERS, or a STOP of `tb.py`, `rows.py` or `lm.py`): record it, and STOP before the BK. The book, or the script, reads differently from the rows before, and a write on it would decide nothing. An answer that is not byte-equal to a reference, with its content the same, is not such a failure: it is recorded and the row goes on.
   - **No opening export and no one-day Trial Balance are sent before the write.** `tb10` is this row's one variable against row 683-NO (see Not in this row).
3. **BK**, by C5, to a new empty local folder named `bk-683tn`. Record the file's name, size and sha256, and the time the Restore list will show, as one line of `evidence/bk.txt` in the form row 683-XF's second sitting used, which `prep-c6.sh` reads: `TDBK1800_<n>.001 <size> B sha256 <64 hex> mtime <time> (the Restore list will show <date> (<time>))`. Then **`mb`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m0-M-marks.xml a-mb-M-marks.xml`. Record its line. A difference is a result to record (C5), not a STOP.

   **The restore's word, given now (C8).** C8 restores a backup after a STOP only on the run coordinator's word. For this row that word is given in advance: from here on, step 8 runs after the last read, after any STOP at any later step, and after a voucher saved wrongly. Nothing else is sent, keyed or backed up after a STOP: only the restore and step 8's state reads, `m2` and `mv2`.

   The word waits, and the run coordinator is asked first, in three cases: an unnamed dialog still up, a send past the wedge rule's 5 minutes, or Tally closed (C9). Each waits for the word on how to clear it, and then the restore runs.
4. **The write.**
   - First **`mc`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-mb-M-marks.xml a-mc-M-marks.xml`. Go on only on SAME MARKS, and only with no key pressed in Tally since `mb`. Otherwise go to step 8.
   - Then K1 and K2, by **The write** above.
5. **`m1`:** `M-marks.xml`, then:
   - `python3 -I lm.py --same-row a-mc-M-marks.xml a-m1-M-marks.xml`. EXPECT MARKS DIFFER (exit 2), with ALTVCHID 13 and ALTMSTID 214.
   - `python3 -I lm.py --same-row a-ldm1-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS: the write moved the marks as rows 683-LD's, 683-VD's, 683-RO's and 683-XF's did.
   - Anything else: record it, and go on with step 6. The reads are still taken, and the restore puts the book back.
6. **The opening export, then the Trial Balance to 1 April twice, then the Trial Balance to 30 April,** one send at a time, with the current date at 30-Apr-2025 as the write left it. **`x1` is the first send after `m1`,** as in rows 683-LD and 683-XF. Run each answer's counters before the next send, and send nothing else in between: no marks read, no `GET /status`, and no key in Tally. A failed request here (`run.py`'s own stop, or a STOP of `lm.py`, `tb.py` or `rows.py`, each of which exits 1) ends step 6: send nothing more, and go to step 8 on the word step 3 gives. A ledger that `tb.py` lists under SET UNREAD is not such a STOP: it is a result, and the next send goes ahead.
   - **`x1`:** `X-lx1-20250401.xml`. Then `python3 -I rows.py a-ldx1-X-lx1-20250401.xml a-x1-X-lx1-20250401.xml`, `python3 -I cmpinfo.py a-ldx1-X-lx1-20250401.xml a-x1-X-lx1-20250401.xml` and `python3 -I lm.py a-x1-X-lx1-20250401.xml`, each recorded whole. EXPECT 8 rows, each opening `0.00`, and CONTENT SAME with `a-ldx1` (exit 0 or 3); byte equality is recorded.
   - **`tb3p`** (the question): `T-tb3-20250401-20250401.xml`. Then, at once, screenshot the whole Tally window. A dialog is the prompt rule's Stop: press no key, and the restore waits for the run coordinator's word (the first of step 3's three cases), not the word given in advance. Then, each recorded whole:
     - `python3 -I rows.py a-ldtb3-T-tb3-20250401-20250401.xml a-tb3p-T-tb3-20250401-20250401.xml`;
     - `python3 -I rows.py a-vdtb3w-T-tb3-20250401-20250401.xml a-tb3p-T-tb3-20250401-20250401.xml`;
     - `python3 -I rows.py a-twtb3-T-tb3-20250401-20250401.xml a-tb3p-T-tb3-20250401-20250401.xml` (its ELEMENT ORDER lines say which order this answer has);
     - `python3 -I cmpinfo.py a-ldtb3-T-tb3-20250401-20250401.xml a-tb3p-T-tb3-20250401-20250401.xml`;
     - `python3 -I cmpinfo.py a-vdtb3w-T-tb3-20250401-20250401.xml a-tb3p-T-tb3-20250401-20250401.xml`;
     - `python3 -I tb.py a-tb3p-T-tb3-20250401-20250401.xml`;
     - `python3 -I tb.py --sets a-ldtb3-T-tb3-20250401-20250401.xml a-tb3p-T-tb3-20250401-20250401.xml`;
     - `python3 -I tb.py --sets a-vdtb3w-T-tb3-20250401-20250401.xml a-tb3p-T-tb3-20250401-20250401.xml`.
   - **`tb3p2`** (the repeat, sent whatever `tb3p` read): `T-tb3-20250401-20250401.xml`. Then `python3 -I rows.py a-tb3p-T-tb3-20250401-20250401.xml a-tb3p2-T-tb3-20250401-20250401.xml`, `python3 -I cmpinfo.py a-tb3p-T-tb3-20250401-20250401.xml a-tb3p2-T-tb3-20250401-20250401.xml` and `python3 -I tb.py --sets a-tb3p-T-tb3-20250401-20250401.xml a-tb3p2-T-tb3-20250401-20250401.xml`, each recorded whole.
   - **`tb1p`** (Q2, after the question's reads, sent whatever they read): `T-tb1-20250401-20250430.xml`. Then, at once, screenshot the whole Tally window; a dialog is handled as for `tb3p`. Then, each recorded whole:
     - `python3 -I tb.py a-tb1p-T-tb1-20250401-20250430.xml`;
     - `python3 -I tb.py --sets a-ldtb1-T-tb1-20250401-20250430.xml a-tb1p-T-tb1-20250401-20250430.xml`;
     - `python3 -I rows.py a-ldtb1-T-tb1-20250401-20250430.xml a-tb1p-T-tb1-20250401-20250430.xml`;
     - `python3 -I cmpinfo.py a-ldtb1-T-tb1-20250401-20250430.xml a-tb1p-T-tb1-20250401-20250430.xml`;
     - `python3 -I tb.py --sets a-ldtb10-T-tb1-20250401-20250430.xml a-tb1p-T-tb1-20250401-20250430.xml`;
     - `python3 -I rows.py a-tb10-T-tb1-20250401-20250430.xml a-tb1p-T-tb1-20250401-20250430.xml` (the same request before and after the write; recorded, not EXPECTed).
7. **`m1r`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m1-M-marks.xml a-m1r-M-marks.xml`. EXPECT SAME MARKS, since the reads wrote nothing. On MARKS DIFFER, record it, send nothing else but the restore, and go to step 8.
8. **The restore,** from the BK of step 3, by C6:
   - check the backup file's sha256 against the one recorded, and write `restore-plan.txt` with the kit's `prep-c6.sh`: `LAB_DIR=<the row's lab folder> TALLY_DATA=<Tally data folder> bash prep-c6.sh 8 bk-683tn`, with the two paths written in full. Any exit other than 0 is a STOP before the restore, and the restore then waits for the run coordinator's word (a fourth case beside step 3's three);
   - shut the company;
   - restore the BK's folder alone;
   - answer only "Accept ?" (Yes) and "Company data folder (<the book's number>) already exists ..." (Replace);
   - load the company alone, with the screenshots C6 names.

   Then:
   - **`m2`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m0-M-marks.xml a-m2-M-marks.xml`. EXPECT SAME MARKS.
   - **`mv2`:** `V-mvy-20250401-20260331.xml`, and `python3 -I lm.py --capture a-mv2-V-mvy-20250401-20260331.xml a-mv0-V-mvy-20250401-20260331.xml`. EXPECT CAPTURE SAME, with 11 and 11. Record whether `a-mv2` is byte-equal to `a-mv0`.
   - MARKS DIFFER or CAPTURE DIFFERS: STOP, and report "the restore of bk-683tn did not return the book to m0". Nothing more is sent or keyed.
9. **Tally's date:** read the Gateway's current date, period and date of last entry, and write them down. EXPECT 1-Apr-2025, 1-Apr-25 to 31-Mar-26 and 1-Apr-25, as at step 0 (every earlier restore left them so). If any differs, press nothing to change it: record it and report it before step 10. The run coordinator says what is done first; step 10 runs on that word.
10. **Close** by C10, with `GET /status` after the last request (C11). One `tally.exe` must own 9001.

Steps 2 and 6 never stop the row on an answer that differs from an estimate. Each such answer is a result and is recorded in full. Only the STOPs named stop the row.

## Stop
Press no key, screenshot the whole window, and report before anything else is pressed or sent, if:
- an unnamed dialog appears (the prompt rule), or a second `tally.exe` starts;
- the gateway stops answering, or a send passes the wedge rule's 5 minutes;
- `run.py`, `lm.py`, `tb.py`, `rows.py` or `prep-c6.sh` stops;
- step 0's date, period or last entry is not as EXPECT;
- m0 is not SAME MARKS with `a-ldm2`;
- a control in step 2 is not as EXPECT;
- a step of the write says STOP;
- the path rule applies;
- m2 or mv2 is not as EXPECT after the restore.

From step 3 on, a STOP is followed by the restore (step 8) on the word step 3 gives, except in the three cases step 3 names and a STOP of `prep-c6.sh` at step 8 itself: then the book stays as written, nothing is sent, and the restore waits for the run coordinator's word. If the BK itself fails or is not recorded, nothing has been written yet: STOP there, with no restore, and close by C10 on the run coordinator's word.

## Record
- Each answer's bytes and sha256, and each request's sha256 (as `SHA256.txt` lists; any other is the Stop of C3).
- `lm.py`'s lines for every marks read, and the whole output of each `lm.py`, `tb.py`, `rows.py`, `cmpinfo.py` and `prep-c6.sh` call.
- Whether each of `tb10` and `mv0` is byte-equal to its reference, and each `cmpinfo.py` line. Either way it is recorded, not judged.
- **The element order** of every Trial Balance and opening export answer, from `rows.py`'s ELEMENT ORDER lines, and whether it held within the sitting (`tb10` against `tb1p`, `tb3p` against `tb3p2`, and `tb1p` against its reference).
- **When Tally was last started** before step 0, and whether it was started for this sitting, with what ran on this PC's Tally since row 683-TA. Where two Trial Balances differ, also list the fields that differ, row by row, as row 683-LD's `evidence/tb3-fields-vs-683tw.txt` did.
- **Each send's start time and how long it took, from `log-<label>.txt`, and the gap in seconds between the end of each send and the start of the next,** for the reads before the write (`m0` to `tb10` to `mv0`) as for those after it. Rows 683-NO's and 683-TA's gaps are given under Question; they are recorded here, not set. Record also the time from K2's save to `m1` and to `tb3p`'s send (to `tb3p`: about 2 minutes in row 683-LD, 37 s in rows 683-XF's second sitting and 683-PW, 22 s in rows 683-XA and 683-NO, 86 s in row 683-TA).
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

**Staging the evidence**, as for rows 683-LM's, 683-TW's, 683-LD's, 683-CD's, 683-VD's, 683-RO's and 683-XF's:
- **The start-up check:** a hit is allowed only where it lies inside the amount `$1000000.00`. List each hit with 12 characters either side. A hit anywhere else waits for the run coordinator.
- **The check for 10 or more digits:** it sets aside this book's registered GUID matched whole, and nothing else.

## Expected (estimates, not EXPECTs)
- **The controls:** `tb10` with the content of `a-ldtb10` (byte-equal in rows 683-LD and 683-PW), and `mv0` with that of `a-twmvy` (byte-equal in rows 683-PW, 683-XA, 683-NO and 683-TA).
- **The write:** Sales No. 4 and Receipt No. 4, as every earlier row; m1 SAME MARKS with `a-ldm1`.
- **`x1`:** the content of `a-ldx1`, as rows 683-RO's, 683-XF's, 683-PW's, 683-XA's, 683-NO's and 683-TA's were (byte-equal there); 8 rows, each `0.00`.
- **`tb3p`:** not known. That is the question.
- **`tb3p2`:** the content of `tb3p`, and byte-equal to it, as every repeated Trial Balance in a sitting on this book has been.
- **`tb1p`:** the content of `a-ldtb1` (six MIXED), as row 683-LD's `tb1` and rows 683-PW's, 683-XA's, 683-NO's and 683-TA's `tb1p` were, after an all-plain `tb3` and (row 683-TA) after one with two MIXED.
- **The restore:** m2 and mv2 as m0 and mv0, and the Gateway at 1-Apr-2025 after the reload, as every earlier row.

## Decides
Below, **as** an answer means SETS SAME against it **and** `rows.py` CONTENT SAME against it (exit 0 or 3). Byte equality and element order are recorded beside each verdict and decide nothing here (see Element order). Anything else is **not as** that answer; an answer with SETS SAME but CONTENT DIFFER against both references is "neither". **The `x1` case below overrides the others:** if `x1` is not as row 683-LD's, that line decides, whatever `tb3p` reads. "Row 683-TA's" means `a-vdtb3w`, which rows 683-VD, 683-RO, 683-XF and 683-TA read nine times, byte-equal; "row 683-NO's" means `a-ldtb3`, which rows 683-LD, 683-PW, 683-XA and 683-NO read, byte-equal.

- **`tb3p` as row 683-TA's (two MIXED), `tb3p2` as `tb3p`:** against row 683-NO (one read apart), a Trial Balance read before the write keeps the two ledgers composite after it whether it ends on 1 April or on 30 April, so the effect is not tied to the one request. Rows 683-LD and 683-PW read `tb10` before the write too, after the opening export, and read plain: whether the opening export read first undoes it is then its own row. PARTIAL: one sitting, one book, one release, compared across sittings.
- **`tb3p` as row 683-NO's (every ledger PLAIN), `tb3p2` as `tb3p`:** against row 683-NO (one read apart), the Trial Balance to 30 April read before the write does not keep the two ledgers composite, while the one-day Trial Balance does (row 683-TA): the effect is tied to the one-day request read before the write, not to any Trial Balance, though both answered with the same bytes there. This agrees with rows 683-LD and 683-PW. PARTIAL, as above.
- **`tb3p` neither** (some other mix of MIXED and PLAIN, other text, or any UNREAD), with `tb3p2` as `tb3p`: the answer is neither earlier one. Record both whole; they decide nothing until the run coordinator has read them.
- **`tb3p2` not as `tb3p`:** the Trial Balance did not repeat within the sitting, for the first time on this book. Record both whole; `tb3p` decides nothing until that is explained (P5).
- **`x1` not as row 683-LD's `x1`** (`rows.py` CONTENT DIFFER against `a-ldx1`, or any row count or opening other than 8 rows of `0.00`): the reads after the write were not those of the rows before. `tb3p` is recorded, but decides nothing until that is explained.
- **`tb10` or `mv0` not as row 683-LD's and row 683-TW's** (by content): the book or the sitting reads differently from the rows before; the row stopped before the BK (step 2) and decides nothing about the question. Not byte-equal with the same content is not this case.
- **`tb1p` (Q2), whatever `tb3p` read:** as row 683-LD's `tb1` (six MIXED): the wider window kept its marks after the write, as in rows 683-LD, 683-PW, 683-XA, 683-NO and 683-TA. Not as it: record it whole, with each field that differs, row by row; it goes to #683 beside `tb3p` and decides nothing alone.
- **Element order:** recorded whichever way the rest falls. If it differs within this sitting (`tb3p` against `tb3p2`, or the order of `tb10` against that of `tb1p`), Tally's order is not fixed within one Tally start, which bears on the paired read (`tally/connection.rs:3193`); if it holds within the sitting, that is recorded beside the earlier sittings. Nothing in this row decides what sets the order.

Whichever way it falls, any answer that becomes a fixture is parsed again by master's own parser first, before any code relies on it.

## Not in this row
- Any read before the write but `m0`, `tb10` and `mv0`: the opening export `x0` (rows 683-LD, 683-PW and 683-XA) and the one-day Trial Balance `tb3a` (rows 683-VD, 683-RO, 683-XF and 683-TA). `tb10` sent twice is not varied here.
- Row 683-LD's reads after its `tb3` other than the Trial Balance to 30 April: its `tb4`, `mv1`, `mvlu1` and `mvli1`.
- K1 keyed as row 683-LD's was, with the "Spelling Error" box, and a wait set between `x1` and `tb3p`: each is its own row.
- What sets the element order of a Trial Balance row: it is recorded, not varied. A Tally restart between reads is not made here.
- Any current date other than the 30-Apr-2025 the write leaves, a voucher on any day other than 30 April, and any change to the period.
- `trial_balance` or `ledger_movement` through Bridge, another book, and another release.
