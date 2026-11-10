# Row 683-LD: on the several-currency lab book, with one dollar voucher and one rupee voucher dated 30 April 2025, do the first-day reads still mark every dollar entry as a composite, and do the plain ledgers still tie

Not for merge. Part of #683. **Two writes, behind a backup:** two vouchers are keyed on screen, the reads are taken, and the book is restored. The run coordinator gives the word for that restore now, under C8, for every outcome from the BK on (step 3 says when it waits). Lines are at master ae38a75c.

## Question
The #683 build reads `ledger_movement` on a book with several Currency masters only when `from` is the book's first day. It sets aside by name every ledger that one of three rules finds:
1. `trial_balance` for the same window sets it aside (its `CURRENCYNAME` is not the base master's NAME, or a value on its row is a currency composite);
2. an entry on it in the window is a composite;
3. its opening at `from` is a composite.

It returns the other ledgers' figures as partial. Every measurement behind these rules is on this book as found, where all eleven vouchers are dated 1 April 2025, the book's first day. Row 683-TW read windows from 1 April to 1 April, 30 April and 31 March 2026, and they gave the same answers. On a book with one voucher day, though, that says nothing about entries on a later day. The build's pull request lists "entries on more than one day" as unmeasured.

This row puts entries on a second day, 30 April 2025, and changes nothing else (P6). It keys two vouchers:
- **K1, a dollar Sales:** `$100` at rate `88`, to `BRIDGE USD Customer` and `BRIDGE Export Sales`. Both ledgers are already set aside, by rules 1 and 2.
- **K2, a rupee Receipt:** `2000`, from `BRIDGE INR Customer` into `Cash`. These are the two ledgers the build keeps.

Then it reads, from the first day, the windows that reach 30 April.

**Q1, rule 2 on a later day:** in the book-scoped window (`mv1`) and in the ledger-scoped window on `BRIDGE USD Customer` (`mvlu1`), is each entry of K1 a composite? `ld.py --day … 20250430` prints `ALL COMPOSITE (2)` for K1 when it is.

**Q2, rule 1 on a later day:** do the currency Trial Balances to 30 April (`tb1`) and to 31 March 2026 (`tb4`) still set aside the same six ledgers and keep the same two? `tb.py --sets` against the book's as-found `tb1` answers this.

**Q3, the kept ledgers tie over two voucher days:** for `Cash` and `BRIDGE INR Customer`, does the sum of the window's entries (`ld.py --sum`) equal the Trial Balance's `TBALCLOSING` minus its `TBALOPENING`, in both `tb1` and `tb4`? Is K2 `ALL PLAIN (2)`? And does the ledger-scoped window on `BRIDGE INR Customer` (`mvli1`) hold the book-scoped window's entries on that ledger?

**The controls:**
- On the book as found, before the BK: the same reads (`x0`, `tb10`, `mv0`). Each request is byte-equal to one row 683-TW or 683-LM sent, and each answer is compared with that row's answer, kept here and not sent.
- After the write, two reads that end before 30 April and so must not move: the opening export at 1 April (`x1`), and the Trial Balance to 1 April (`tb3`).

Every request is master's own builder, sent raw, and each has been sent on this book before (rows 683-LM, 683-TB and 683-TW). No request is new and nothing is rendered for this row. **Not sent: a movement window ending exactly on 30 April.** The build would send that window for `to` 20250430, and it is a new render. Row 683-TW found the window to 31 March 2026 byte-equal to the one-day window, so a later `to` alone did not change the answer on this book. The windows here reach 30 April and stop on 31 March 2026.

## Book
`BRIDGE FOREXGROUP LAB` (synthetic), number 100090, GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, TallyPrime 7.2. THE COMPANY in `LAB-COMMON.md` means this book.
- **Synthetic:** it is in master's registry, `scripts/merge-gate-synthetic-uuids.json` line 201 ("created in the lab on 28 October 2026 with invented masters and vouchers keyed by hand").
- **Not an excluded book:** its GUID is not the register lab book's (`REGISTER_LAB_GUID`, `captured_parse_record.rs:302`), and not the one the private exclusion list names.
- **Pinned by GUID.** The `BOOK` of `lm.py`, `tb.py` and `ld.py` is this GUID. m0 must print one COMPANY row, `the book's: yes`, or the row STOPs before any other send.
- **Its last state** (row 683-TW, `lab/683-tw-answers` 4f31b925, `lab/683-tw-answers/RESULTS.md`): V 11, M 214, and m1 byte-equal to m0. Tally's date was 1-Apr-2025, with the period 1-Apr-25 to 31-Mar-26 and the last entry 1-Apr-25. No row has written to the book since. Row 683-OP's ledger was restored away before that.
- **Its vouchers** (row 683-LM's book-scoped answer, `a-twmvy` here, read by `ld.py`): Sales 1, 2 and 3; Receipt 1, 2 and 3; Journal 1; Purchase 1 and 2; Payment 1 and 2. All are dated 20250401, and none is cancelled or optional. ALTVCHID 11 with eleven vouchers means no voucher was ever deleted. **So Sales 4 and Receipt 4 are unused,** and the write expects Tally to offer them (see The write).
- **Tally's current date:** do not change it, or the period, except as F2 on a voucher screen does. Row 1211 recorded that keying a voucher with F2 moved Tally's current date, and that the restore's reload set it back. Step 9 records it.
- **One BK, taken in this sitting before the write, and one restore after the last read** (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID must equal m0's (the ruling of 4 Nov 2026 on unchanged marks). It is also checked by the book-scoped window, which must again equal the book as found.

## Kit
Check every file against `SHA256.txt` first, and check each send's request file again just before it is sent (Get-FileHash, C3). If any file differs: STOP, and nothing is sent.

| file | what, and where its bytes come from |
|---|---|
| `M-marks.xml` | The marks request (GUID, ALTVCHID, ALTMSTID). Byte-equal to row 683-TW's. |
| `X-lx1-20250401.xml` | Row 683-OP's, which is row 683-LM's: `render_native_ledger_export_request` (`request.rs:258`) for `from` 20250401, the opening read `ledger_movement` sends (rule 3's input). |
| `T-tb1-20250401-20250430.xml`, `T-tb3-20250401-20250401.xml`, `T-tb4-20250401-20260331.xml` | Row 683-TW's: `render_native_trial_balance_request_with_currency` (`native_trial_balance/mod.rs:170`), from 20250401 to 30 April, 1 April and 31 March 2026 (rule 1's input). |
| `V-mvy-20250401-20260331.xml` | Row 683-TW's: `render_agent_movement_vouchers` (`agent_read_profiles.rs:100`), book scope, 20250401 to 20260331 (rule 2's input). |
| `V-mvly-usd-customer-20250401-20260331.xml`, `V-mvly-inr-customer-20250401-20260331.xml` | Row 683-TW's: `render_agent_ledger_vouchers` with `LedgerVoucherFields::Movement` (`agent_read_profiles.rs:361`), `CHILDOF` `BRIDGE USD Customer` and `BRIDGE INR Customer`, the same dates. |
| `a-twm0-M-marks.xml` | Not sent. Row 683-TW's m0 answer (sha256 `3d75dd1a…dafba`), the marks this row expects at m0 and after the restore. |
| `a-twtb1-T-tb1-20250401-20250430.xml` | Not sent. Row 683-TW's `tb1` answer (sha256 `1c4638e8…b7b7`, 14,140 B). Its `tb3` and `tb4` answers are byte-equal to it, so this one file stands for all three. |
| `a-twmvy-V-mvy-20250401-20260331.xml` | Not sent. Row 683-TW's `mvy` answer (sha256 `56554b6a…4df7`, 59,524 B), byte-equal to row 683-LM's one-day window. |
| `a-twmvlyusd-V-mvly-usd-customer-20250401-20260331.xml`, `a-twmvlyinr-V-mvly-inr-customer-20250401-20260331.xml` | Not sent. Row 683-TW's `mvly-usd` and `mvly-inr` answers. |
| `a-lmlx1-X-lx1-20250401.xml` | Not sent. Row 683-LM's `lx1` answer (sha256 `898ad76b…ce3`, 11,556 B), the opening export at 1 April. |
| `run.py`, `LAB-COMMON.md`, `.gitattributes`, `lm.py`, `tb.py` | Row 683-TW's, byte-equal. |
| `ld.py` | This row's counter for a movement answer. Its header says what each mode prints and every STOP. It copies these functions unchanged, checked by script function by function: `say`, `stop`, `load` and `leaf` from `lm.py`; and the classes and master's composite check (`split_once_exact` to `is_balance_composite`, and `shape`) from `tb.py`. |

The reference answers keep the `a-<label>-<K>-` form so that `lm.py`, `tb.py` and `ld.py` read them by kind. Their labels (`twm0`, `twtb1`, `twmvy`, `twmvlyusd`, `twmvlyinr`, `lmlx1`) are not used for any send.

**`ld.py` was tried:**
- **On row 683-TW's `mvy` answer:** `--day … 20250401` gives eleven vouchers, eight `ALL COMPOSITE (2)` and three `ALL PLAIN (2)`, then `DAY 20250401: 11 vouchers, 0 on other days`. `--day … 20250430` gives `0 vouchers, 11 on other days`. `--sum` gives `Cash` `-3000.00 (2 entries on 2 vouchers)` and `BRIDGE INR Customer` the same, each equal to `tb1`'s closing minus its opening. `BRIDGE USD Customer` gives `NOT A FIGURE (4 entries not PLAIN)`, and a name with no entry gives `NO ENTRY`.
- **On row 683-TW's `mvly-inr` answer:** two vouchers, `ALL PLAIN (2)` each.
- **On copies of `mvy`:**
  - Journal 1 marked `ISCANCELLED` Yes gives `--sum Cash` `NOT A FIGURE (1 vouchers not ISCANCELLED No and ISOPTIONAL No: …)`, naming it; `--day` prints each voucher's two flags.
  - Receipt 3's DATE made 20250430 gives that voucher alone on the day, `ALL COMPOSITE (2)`, with `10 on other days`. Receipt 2's made 20250430 gives `ALL PLAIN (2)`.
  - Journal 1's Cash amount made a composite, empty, or with a leading space gives `--sum Cash` `NOT A FIGURE (1 entries not PLAIN)` each time.
  - Each of these is a STOP (exit 1): STATUS `0`; a LINEERROR added; one voucher twice; a GUID one digit off the book's; a voucher with no DATE; no VOUCHER row; the answer cut in half (it does not parse); an answer whose name is not a movement answer's.

Master's own parsers are run on the answers when they become fixtures, and that run is the one that counts.

## The write
The run lead makes it on Tally's screen, once, after the BK and the check before the write (step 4). **Type only the fields named, and leave every other field as Tally offers it (Enter only).** This is the keying convention of `lab/2071-forex-reads` steps 4 and 7 (its K10, K11 and K9), by which this book's vouchers were made:
- accounting invoice for Sales, and double entry for Receipt;
- Party Details screens are passed by Enter with the party's defaults;
- no narration;
- a screenshot before each Ctrl+A.

Expected screens, none of them a STOP:
- the Forex Rate of Exchange box where K1 says;
- an empty Inventory Allocations box for the sales ledger, closed by Enter on the blank Name of Item;
- the Bill-wise Details the steps name.

If Tally adds a gain or loss line, press no key: screenshot it, write it down, and ask the run coordinator before Ctrl+A. Do not add one.

**Every STOP inside these steps means: press no key, screenshot the whole window, and report.**

**K1, F8 Sales:**
1. **Voucher No.:** write down what Tally offers. EXPECT `4`. If it offers any other number, STOP before any other key: `1`, `2` or `3` would reuse a number, and anything else means the book is not as its answers show.
2. **Date:** F2, type `30-4-2025`, Enter. The voucher's date must then show 30-Apr-2025. If it shows anything else, or Tally answers with a message: STOP.
3. **Party A/c name:** `BRIDGE USD Customer`, Enter on the one row listed. Party Details: Enter only.
4. **Particulars:** `BRIDGE Export Sales`; the amount typed `$100`. Where the Forex Rate of Exchange box opens, key `88`, then Enter. If no such box opens, or the line does not then show a dollar amount: STOP before Ctrl+A.
5. **Bill-wise Details:** type of reference `New Ref`; Name typed `LD1`, replacing what is offered; `$100.00 Dr` as offered. Write down each value shown.
6. **Screenshot the whole screen before Ctrl+A.** Write down the Voucher No., the date and each line as Tally shows it. Then Ctrl+A, and Yes to "Accept ?".

**K2, F6 Receipt** (from the Gateway, Vouchers, F6):
1. **Voucher No.:** EXPECT `4`, with the same STOP as K1's.
2. **Date:** if the screen does not already show 30-Apr-2025, F2, type `30-4-2025`, Enter. Write down which it was. If it then shows anything else: STOP.
3. **To** `BRIDGE INR Customer`, **Credit** `2000`. If a Forex Rate of Exchange box opens: STOP, press nothing.
4. **Bill-wise Details:** `Agst Ref`, choose `R1` (the pending amount offered is expected to be 3,000.00 Dr), then the Amount typed `2000` Cr, replacing what is offered. If R1 is not in Pending Bills: STOP.
5. **By** `Cash`, **Debit** as offered (2,000.00 expected).
6. **Screenshot the whole screen before Ctrl+A.** Write down every line. Then Ctrl+A, and Yes to "Accept ?".

After each save, Tally shows a blank voucher screen. Screenshot its title, then leave it with Ctrl+Q (C7). Never press Esc on a voucher screen, and never press Ctrl+Q at the Gateway. After K2, screenshot the whole Tally window at the Gateway before `m1` is sent (C3).

After a STOP inside these steps, nothing of that voucher has been saved, but K1 may have been. Press no key: the run coordinator says how the screen is left. The restore then runs as step 8 says.

## Screens and prompts
The screen paths for load and close are those of rows 683-TW and 683-OP:
- **Load:** Select Company, the book typed into its name filter, the one row, Enter.
- **A company already loaded at the start:** C1 says STOP. If the run coordinator's word is to shut it, shut it by Company > Shut, with no change to it.
- **Close (C10):** the `K: Company` label, Shut, Enter on the book.
- **Bring Tally to the front only by its running window.** A second `tally.exe` is a STOP.

**The prompt rule.** The only dialogs answered in this row are:
- C5's: a message that only says the backup is done (screenshot it, accept once);
- C6's two: "Accept ?" = Yes, and "Company data folder (100090) already exists ..." = Replace;
- each voucher's own "Accept ?" on Ctrl+A = Yes;
- the screens the write names: the date (F2), the Forex Rate of Exchange box, Inventory Allocations, Party Details and Bill-wise Details.

**Any other dialog, message box, sub-screen or prompt, at any step, is unnamed:** press no key, screenshot the whole window, copy its text exactly, and send nothing while it is up. Then ask the run coordinator (C7). It stays on the screen until the run coordinator answers.

**The path rule.** If a list or line shown is not exactly what a path above names (other words, no such line, a Spelling Error), send nothing and ask:
- **a list reached from the Gateway** (Select Company, the Shut list, the Backup or Restore list): Esc out of the list;
- **a list on a voucher screen** (the party, ledger or Pending Bills list of K1 or K2): press no key. Never press Esc on a voucher or master screen (C7).

## The wedge rule
A read that Tally is still building keeps the gateway busy for every company, and giving up on it does not stop Tally's work. So:
- **One request at a time, by `run.py`.** It never abandons a request: curl waits for its answer.
- **The book is small:** 13 vouchers and 8 ledgers after the write. Its answers on record are under 60,000 B (row 683-TW's `mvy`, 59,524 B, is the largest). Two vouchers add about 10,000 B to a book-scoped window.
- **If any send has not answered 5 minutes after it began:**
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).

  Nothing is re-sent on this row's own judgement. With the vouchers saved and the gateway stuck, the restore waits for the run coordinator's word.
- **Failed requests:** `run.py`'s own stops (a curl exit other than 0, Unknown Request, an empty answer, an answer above 2,000,000 B), and the STOP of `lm.py`, `tb.py` or `ld.py`. Each is a failed request, never "nothing found".

## Steps
Run each send as `python3 -I run.py http://127.0.0.1:9001 <label> <file>`, and write down each send's start time from its log. **Each label is used once.** A repeated send takes a new label (`m1r`), never an earlier one's. The answer of send `<label>` is `a-<label>-<request file>`.

0. **C2's step 0, with no company loaded:**
   - the screenshot first;
   - one `tally.exe` owning 9001;
   - `GET /status` 200 (record its bytes).

   Load the book alone, as above. Read the Gateway's date and period, and write them down. Record the release and the licence mode, as row 683-TW recorded them.
1. **`m0`:** `M-marks.xml`, then `python3 -I lm.py --same-row a-twm0-M-marks.xml a-m0-M-marks.xml`. EXPECT SAME MARKS: one COMPANY row, the book's, with ALTVCHID 11 and ALTMSTID 214.
   - Any other GUID or row count: STOP.
   - MARKS DIFFER: record it and STOP before step 2. The book changed after its last note, and its voucher numbers are then not known.
2. **The book as found** (the controls), one send at a time:
   - **`x0`:** `X-lx1-20250401.xml`. Record whether its answer is byte-equal to `a-lmlx1-X-lx1-20250401.xml` (sha256). Then `python3 -I lm.py a-x0-X-lx1-20250401.xml`. EXPECT 8 rows, each opening `0.00`.
   - **`tb10`:** `T-tb1-20250401-20250430.xml`. Then `python3 -I tb.py --sets a-twtb1-T-tb1-20250401-20250430.xml a-tb10-T-tb1-20250401-20250430.xml`. EXPECT SETS SAME.
   - **`mv0`:** `V-mvy-20250401-20260331.xml`. Then `python3 -I lm.py --capture a-mv0-V-mvy-20250401-20260331.xml a-twmvy-V-mvy-20250401-20260331.xml`. EXPECT CAPTURE SAME, with `vouchers: movement 11, capture 11`.
   - If any of these is not as EXPECT: record it, and STOP before the BK. The book, or the script, reads differently from the rows before, and a write on it would decide nothing.
3. **BK**, by C5, to a new empty local folder named `bk-683ld`. Record the file's name, size and sha256, and the time the Restore list will show. Then **`mb`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m0-M-marks.xml a-mb-M-marks.xml`. Record its line. A difference is a result to record (C5), not a STOP.

   **The restore's word, given now (C8).** C8 restores a backup after a STOP only on the run coordinator's word. For this row that word is given in advance: from here on, step 8 runs after the last read, after any STOP at any later step, and after a voucher saved wrongly. Nothing else is sent, keyed or backed up after a STOP: only the restore and step 8's state reads, `m2` and `mv2`.

   The word waits, and the run coordinator is asked first, in three cases: an unnamed dialog still up, a send past the wedge rule's 5 minutes, or Tally closed (C9). Each waits for the word on how to clear it, and then the restore runs.
4. **The write.**
   - First **`mc`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-mb-M-marks.xml a-mc-M-marks.xml`. Go on only on SAME MARKS, and only with no key pressed in Tally since `mb`. Otherwise go to step 8.
   - Then K1 and K2, by **The write** above.
5. **`m1`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-mc-M-marks.xml a-m1-M-marks.xml`. EXPECT MARKS DIFFER (exit 2), with ALTVCHID above `mc`'s (13 is estimated). Record ALTMSTID.
   - Anything else: record it, and go on with step 6. The reads are still taken, and the restore puts the book back.
6. **The book with the two vouchers.** One send at a time, and run each answer's counters before the next send:
   - **`x1`:** `X-lx1-20250401.xml`. Record whether it is byte-equal to `a-x0-…`. Then `python3 -I lm.py a-x1-X-lx1-20250401.xml`.
   - **`tb3`:** `T-tb3-20250401-20250401.xml`. Record whether it is byte-equal to `a-tb10-…`. Then `python3 -I tb.py --sets a-tb10-T-tb1-20250401-20250430.xml a-tb3-T-tb3-20250401-20250401.xml`.
   - **`tb1`:** `T-tb1-20250401-20250430.xml`. Then, at once, screenshot the whole Tally window: this is the first Trial Balance on this book with entries on two days. A dialog = Stop. Then `python3 -I tb.py a-tb1-T-tb1-20250401-20250430.xml` and `python3 -I tb.py --sets a-tb10-T-tb1-20250401-20250430.xml a-tb1-T-tb1-20250401-20250430.xml`, each recorded whole.
   - **`tb4`:** `T-tb4-20250401-20260331.xml`. Then `python3 -I tb.py a-tb4-T-tb4-20250401-20260331.xml` and `python3 -I tb.py --sets a-tb10-T-tb1-20250401-20250430.xml a-tb4-T-tb4-20250401-20260331.xml`, each recorded whole.
   - **`mv1`:** `V-mvy-20250401-20260331.xml`. Then, at once, screenshot the whole Tally window. A dialog = Stop. Then, each recorded whole:
     - `python3 -I ld.py --day a-mv1-V-mvy-20250401-20260331.xml 20250430`;
     - `python3 -I ld.py --sum a-mv1-V-mvy-20250401-20260331.xml Cash`;
     - `python3 -I ld.py --sum a-mv1-V-mvy-20250401-20260331.xml "BRIDGE INR Customer"`;
     - `python3 -I lm.py --capture a-mv1-V-mvy-20250401-20260331.xml a-mv0-V-mvy-20250401-20260331.xml`.
   - **`mvlu1`:** `V-mvly-usd-customer-20250401-20260331.xml`. Then `python3 -I lm.py --ledger a-mvlu1-V-mvly-usd-customer-20250401-20260331.xml a-mv1-V-mvy-20250401-20260331.xml "BRIDGE USD Customer"` and `python3 -I ld.py --day a-mvlu1-V-mvly-usd-customer-20250401-20260331.xml 20250430`, each recorded whole.
   - **`mvli1`:** `V-mvly-inr-customer-20250401-20260331.xml`. Then `python3 -I lm.py --ledger a-mvli1-V-mvly-inr-customer-20250401-20260331.xml a-mv1-V-mvy-20250401-20260331.xml "BRIDGE INR Customer"` and `python3 -I ld.py --day a-mvli1-V-mvly-inr-customer-20250401-20260331.xml 20250430`, each recorded whole.
7. **`m1r`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m1-M-marks.xml a-m1r-M-marks.xml`. EXPECT SAME MARKS, since the reads wrote nothing. On MARKS DIFFER, record it, send nothing else but the restore, and go to step 8.
8. **The restore,** from the BK of step 3, by C6:
   - check the backup file's sha256 against the one recorded, and write `restore-plan.txt`;
   - shut the company;
   - restore the BK's folder alone;
   - answer only "Accept ?" (Yes) and "Company data folder (100090) already exists ..." (Replace);
   - load the company alone, with the screenshots C6 names.

   Then:
   - **`m2`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m0-M-marks.xml a-m2-M-marks.xml`. EXPECT SAME MARKS.
   - **`mv2`:** `V-mvy-20250401-20260331.xml`, and `python3 -I lm.py --capture a-mv2-V-mvy-20250401-20260331.xml a-mv0-V-mvy-20250401-20260331.xml`. EXPECT CAPTURE SAME, with 11 and 11. Record whether `a-mv2` is byte-equal to `a-mv0`.
   - MARKS DIFFER or CAPTURE DIFFERS: STOP, and report "the restore of bk-683ld did not return the book to m0". Nothing more is sent or keyed.
9. **Tally's date:** read the Gateway's current date and period, and write them down. EXPECT 1-Apr-2025 and 1-Apr-25 to 31-Mar-26, as at step 0. If either differs, press nothing to change it: record it and report it. The run coordinator decides.
10. **Close** by C10, with `GET /status` after the last request (C11). One `tally.exe` must own 9001.

Steps 2 and 6 never stop the row on an answer that differs from an estimate. Each such answer is a result and is recorded in full. Only the STOPs named stop the row.

## Stop
Press no key, screenshot the whole window, and report before anything else is pressed or sent, if:
- an unnamed dialog appears (the prompt rule), or a second `tally.exe` starts;
- the gateway stops answering, or a send passes the wedge rule's 5 minutes;
- `run.py`, `lm.py`, `tb.py` or `ld.py` stops;
- m0 is not SAME MARKS with row 683-TW's m0;
- a control in step 2 is not as EXPECT;
- a step of the write says STOP;
- m2 or mv2 is not as EXPECT after the restore.

From step 3 on, a STOP is followed by the restore (step 8) on the word step 3 gives, except in the three cases step 3 names.

## Record
- Each answer's bytes and sha256, and each request's sha256 (as `SHA256.txt` lists; any other is the Stop of C3).
- `lm.py`'s lines for every marks read, and the whole output of each `lm.py`, `tb.py` and `ld.py` call.
- Each send's start time and how long it took, from `log-<label>.txt`.
- **The write:** every value written down in its steps, word for word as the screen shows it:
  - each Voucher No. offered and the date shown;
  - each line and amount;
  - the rate box's title and fields;
  - every field of each Bill-wise line, with the pending amount R1 showed;
  - every prompt, with the key that answered it.
- The BK file (name, folder, bytes, sha256, and the Restore list's time), `restore-plan.txt`, the data-folder listings before and after the restore, and every prompt of the restore word for word.
- The marks at m0, mb, mc, m1, m1r and m2.
- The release, the licence mode, and Tally's date and period at steps 0 and 9, with the PC's date.
- Every screenshot by name, bytes and sha256 (screenshots are never pushed).

**Staging the evidence**, as for row 683-TW's:
- **The start-up check:** a hit is allowed only where it lies inside the amount `$1000000.00`. List each hit with 12 characters either side. A hit anywhere else waits for the run coordinator.
- **The check for 10 or more digits:** it sets aside this book's registered GUID matched whole, and nothing else.

## Expected (estimates, not EXPECTs)
- **The controls** read as rows 683-LM and 683-TW read them.
- **`x1`** is byte-equal to `x0`, and **`tb3`** to `tb10`: no read that ends on 1 April sees a voucher dated 30 April.
- **`mv1`:** thirteen vouchers. `lm.py --capture` against `mv0` gives eleven SAME and the two new GUIDs "in the movement window only", so CAPTURE DIFFERS for that reason alone. `ld.py --day … 20250430` gives Sales `'4'` `ALL COMPOSITE (2)` and Receipt `'4'` `ALL PLAIN (2)`, then `DAY 20250430: 2 vouchers, 11 on other days`.
- **The sums:** `ld.py --sum` gives `Cash` `-5000.00 (3 entries on 3 vouchers)` and `BRIDGE INR Customer` `-1000.00 (3 entries on 3 vouchers)`.
- **`tb1` and `tb4`:** SETS SAME against `tb10`. `Cash` is PLAIN with `TBALOPENING` `0.00`, `DEBITTOTALS` `-5000.00` and `TBALCLOSING` `-5000.00`. `BRIDGE INR Customer` is PLAIN with `0.00`, `-5000.00`, `4000.00` and `-1000.00`. The six others are MIXED, with new composite texts.
- **`mvlu1`:** LEDGER SAME, five vouchers, K1 `ALL COMPOSITE (2)`. **`mvli1`:** LEDGER SAME, three vouchers, K2 `ALL PLAIN (2)`.
- **K1's figure:** `$100` at `88` is 8,800.00 rupees, Dr on the party.
- **Not estimated:** whether a dollar entry dated after the book's first day is sent as a composite in a window from the first day. Row 683-TB found a carried dollar balance plain from the second day, but that is a balance, not an entry. That is Q1.

## Decides
For the build, by the lines step 6 records:
- **Q1, K1 `ALL COMPOSITE (2)` in `mv1` and in `mvlu1`:** rule 2 sees a dollar entry on a later day in a window from the first day, on this book and release: PARTIAL. "Entries on more than one day" leaves Stays unmeasured for this book. The captures may become a fixture of a later-day window.
- **Q1, K1 `NOT ONE CLASS`, or `ALL PLAIN`:** a dollar entry on a later day can reach the window as a plain rupee figure. Rule 2 does not see it there, and the build's admission would have to stop at the book's first voucher day, not its first day. Record the texts; the choice goes to #683 first, and nothing is built on it here.
- **Q2, SETS SAME for `tb1` and `tb4`:** rule 1's sets for a first-day `from` do not move with a later-day entry, on this book: PARTIAL.
- **Q2, SETS DIFFER:** record which ledgers moved and how. If a kept ledger was set aside, rule 1 widened, which is safe but is recorded. If a set-aside ledger was kept, the build waits.
- **Q3, each `--sum` equal to `TBALCLOSING` minus `TBALOPENING` for that ledger in `tb1` and in `tb4`, K2 `ALL PLAIN (2)`, and `mvli1` LEDGER SAME:** the kept ledgers tie over two voucher days: PARTIAL.
- **Q3, a `--sum` that differs, or `NOT A FIGURE`:** record the entries. The build's tie for a kept ledger is measured on one voucher day only, and the build waits.
- **`x1` or `tb3` not byte-equal to its control:** a read ending on 1 April changed with a voucher dated later. Record the difference; the answers decide nothing until it is explained.

Whichever way it falls, master's own parsers parse the answers again when they become fixtures, before any code relies on them.

## Not in this row
- `ledger_movement` itself, or any Bridge build. The window plan's split of a long window, and a window ending exactly on 30 April, are not sent (see Question).
- A dollar entry on a ledger the build keeps (on `Cash`, say). That would change the sets as well as the date, so it is a second variable and gets its own row.
- A typed bill allocation in the movement window. The window's fetch names no allocation field, so its answers carry each allocation amount-only. The `vouchers` capture of 1 April already holds them typed.
- A `from` after the book's first day (row 683-TB), a second foreign currency, a base other than the rupee, and another release.
