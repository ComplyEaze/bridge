# Row 683-OP: on the several-currency lab book, when a ledger's own opening balance is keyed in dollars on its master, do master's reads see a composite at the book's first day

Not for merge. Part of #683. **One write, behind a backup:** one ledger is created on screen, read, and then the book is restored. Lines are at master ae38a75c.

## Question
The plan agreed on #683 after row 683-TB (`lab/683-tb-answers` 154a0ca3): `ledger_movement` keeps refusing a book with several Currency masters for any `from` after the book's first day (`company_several_currency_masters`, `tally/runtime.rs:3155-3162`). When `from` is the book's first day, it sets aside by name every ledger that one of three rules finds:
1. `trial_balance` sets it aside for the same window (its `CURRENCYNAME` is not the base master's NAME, or a value on its row is a currency composite);
2. an entry on it in the window is a composite;
3. its opening at `from` is a composite.

It returns the other ledgers' figures as partial.

What is left: on the book's first day, a dollar balance can enter a ledger with no voucher, as the ledger master's own opening balance. If Tally sends that opening as a plain rupee figure in every read, no rule sees it. This row keys one such opening and reads it three ways:
- **the ledger master read** (`s`): master's `List of Ledgers` snapshot, which `outstandings` (`tally/runtime.rs:4228`) and the party bills read (`tally/runtime_party_bills.rs:85`) send. It fetches `CURRENCYNAME`, `OPENINGBALANCE` and `CLOSINGBALANCE`;
- **the movement export's opening at the book's first day** (`x`): rule 3's input, `ledger_movement`'s own opening read (`tally/runtime.rs:3172`);
- **the currency Trial Balance from the book's first day** (`t`): rule 1's input.

**The question:** after `BRIDGE USD Opening` is created with an opening of `$500` keyed on its master, is that ledger's opening a composite in `t` (`tb.py --question`) and in `x` (`op.py --question`)? `s` is recorded beside them.

**The controls:** the same three reads on the book as found, before the backup (`s0`, `x0`, `t0`). `t0` is byte-equal in its request to row 683-TB's `tb1` and must read as it did. Then the only change between the two rounds is the one ledger (P6). `op.py --others` and `tb.py --sets` show whether any other ledger's row changed.

All three requests are master's own builders, sent raw. None is a new shape: each has been sent on this book before (29 Oct, and rows 683-LM and 683-TB).

## Book
`BRIDGE FOREXGROUP LAB` (synthetic), number 100090, GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, TallyPrime 7.2 Silver. THE COMPANY in `LAB-COMMON.md` means this book.
- **Synthetic:** it is in master's registry, `scripts/merge-gate-synthetic-uuids.json` line 201 ("created in the lab on 28 October 2026 with invented masters and vouchers keyed by hand").
- **Not an excluded book:** its GUID is not the register lab book's (`REGISTER_LAB_GUID`, `captured_parse_record.rs:302`) and not the one the private exclusion list names.
- **Pinned by GUID.** `lm.py`'s, `tb.py`'s and `op.py`'s `BOOK` is this GUID. m0 must print one COMPANY row, `the book's: yes`, else STOP before any other send.
- **Its last state** (row 683-TB, `RESULTS.md`): V 11, M 214, m1 byte-equal to m0; Tally's date 1-Apr-2025, period 1-Apr-25 to 31-Mar-26; eleven vouchers on 1-Apr-2025, eight ledgers, every opening 0.00. No row has written to it since.
- **Do not change Tally's current date or period.**
- **One BK, taken in this sitting before the write, and one restore after the last read** (C5, C6). The restore is checked by the COMPANY row of the marks read: its GUID, ALTVCHID and ALTMSTID must equal m0's (the ruling of 4 Nov 2026 on unchanged marks).

## Kit
Check every file against `SHA256.txt` first, and each send's request file again just before it is sent (Get-FileHash, C3). Any file that differs = STOP, and nothing is sent.

| file | what, and where its bytes come from |
|---|---|
| `M-marks.xml` | The marks request (GUID, ALTVCHID, ALTMSTID). Byte-equal to rows 683-LM's and 683-TB's. |
| `S-ls-20250401-20250430.xml` | `render_native_ledger_snapshot_request` (`native_outstandings/request.rs:169`) for this book, 20250401 to 20250430. Byte-equal to `lab/2071-forex-rerun` `requests/ledger-snapshot.utf16le.xml`, the request the build sent on this book on 29 Oct, UTF-16LE with a BOM. Its text is master's format string (`request.rs:203`) filled with the book's name and those dates, byte for byte (checked by script). `git diff af4a0dbf ae38a75c` on `request.rs` moves the `parent_filter_parts(None)` call from inside the renderer to its caller; it gives two empty strings either way. |
| `X-lx1-20250401.xml` | Row 683-LM's, byte-equal: `render_native_ledger_export_request` (`request.rs:258`), with the period `ledger_opening_period` (`runtime.rs:2078`) builds for `from` 20250401: 20250401 to 20250401. |
| `T-tb1-20250401-20250430.xml` | Row 683-TB's, byte-equal: `render_native_trial_balance_request_with_currency` (`native_trial_balance/mod.rs:170`), 20250401 to 20250430, byte-equal to the build's request of 29 Oct. |
| `run.py`, `LAB-COMMON.md`, `.gitattributes`, `lm.py` | Row 683-LM's, byte-equal. `lm.py` is used here for the marks only. |
| `tb.py` | Row 683-TB's, with two lines changed, and its header comment (line 2): `QUESTION_LEDGER` is `BRIDGE USD Opening`, and `WATCH` is that ledger, `BRIDGE USD Customer` and `Profit & Loss A/c`. Nothing else differs. |
| `op.py` | This row's counter for `S` and `X` answers. Its header says what each class and mode prints, and every STOP. Its composite check and classes are tb.py's functions, copied unchanged (checked by script, function by function). |

**`op.py` and `tb.py` were tried:**
- **On row 683-LM's `lx1` and `lx2` answers and the 29 Oct snapshot answer** (`lab/2071-forex-rerun-answers` `raw/ledger-snapshot.response`): every row read, with the classes those answers show (each `lx1` opening PLAIN `0.00`; `Profit & Loss A/c`'s `lx2` opening COMPOSITE; four of the snapshot's five closings COMPOSITE).
- **On copies of those answers**, with one row renamed `BRIDGE USD Opening`:
  - `X`: a plain opening, NO COMPOSITE; `Profit & Loss A/c`'s composite opening, COMPOSITE; an empty opening, EMPTY; an opening with a leading space, OTHER;
  - `S`: a composite closing, COMPOSITE; its `CURRENCYNAME` made `$`, FOREIGN; plain values, NO COMPOSITE;
  - `--others` with a row added: OTHERS SAME; with a row added and another row's opening changed: OTHERS DIFFER, naming it; `--same` of an answer against itself: ROWS SAME;
  - each a STOP (exit 1): a GUID one digit off the book's, a `BRIDGECOMPANYGUID` not the book's, an opening with no TYPE, an opening removed, STATUS `0`, a closing twice, two names that differ only in case, a comment inside an amount, two answers of different kinds.
- **`tb.py`, on row 683-TB's `tb1` answer** and on copies with a `BRIDGE USD Opening` row added: no row, NOT ANSWERED (no row); a composite opening and closing with empty movement, MIXED and COMPOSITE (TBALOPENING, TBALCLOSING); a plain opening and closing, PLAIN and NO COMPOSITE. `--sets` against `tb1` named that ledger alone.

Neither script compares the opening with the figure keyed: a plain `0.00` opening prints NO COMPOSITE too. Decides reads the figure on the QUESTION line for that reason.

Master's own parsers are run on the answers when they become fixtures, and that run is the one that counts.

## The write
Made by the run lead on Tally's screen, once, after the BK and the before-write check (step 4). **Type only the fields named; every other field is left as Tally offers it (Enter only).** This is the keying convention of `lab/2071-forex-reads` step 4, by which this book's ledgers were made.

1. From the Gateway: Create, then Ledger. Ledger Creation opens. Capture it.
2. **Name:** type `BRIDGE USD Opening` exactly, in this case and with no space after it.
3. **Under:** type `Sundry Debtors`, and Enter on the one row listed.
4. **Maintain balances bill-by-bill:** as offered. Write down what it shows (Yes is expected, as `BRIDGE INR Customer` and `BRIDGE USD Supplier` showed it).
5. **Currency of Ledger,** if the screen shows it: as offered. Write down what it shows. If it offers anything other than `₹`: STOP before the opening, press nothing.
6. **Opening Balance:** type `$500`, then Enter.
   - **Where a box asking for a rate of exchange opens, key `88`, then Enter.** Every rate keyed on this book is a whole number.
   - **If no such box opens,** write down exactly what the field then shows, and go on.
   - **If the field does not show a dollar amount** (it clears, beeps, shows `500` or a rupee amount, or Tally shows an error line): STOP before Ctrl+A, press nothing, screenshot. That is a result: on this release a dollar opening cannot be keyed on a base-currency ledger's master.
   - **Dr or Cr:** as offered. Write it down (Dr is expected).
7. **Bill-wise Breakup,** where bill-by-bill is Yes and it opens: one line.
   - The fields in the order Tally offers them: a type of reference, if asked, `New Ref`; the bill's name typed `OPBILL` (replacing what is offered); every other field as offered (Enter). Write down each value shown, the date and the amount above all.
   - A rate asked inside the line: `88`.
   - Once the line holds the whole opening, an empty line may be offered: Enter once on its first field. The breakup closes and the master shows again.
8. Every other field, including any GST, PAN, address or bank field: Enter only. If Enter opens a sub-screen this list does not name: STOP, press nothing.
9. **Screenshot the whole screen before Ctrl+A,** with the opening as Tally shows it. Write that text down exactly.
10. **Save with Ctrl+A.** If Tally asks "Accept ?", answer Yes. Screenshot what follows. A blank Ledger Creation screen: capture its title, leave it with Ctrl+Q (C7). A list: Esc out of the list to the Gateway (never Esc from a master screen, never Ctrl+Q at the Gateway).

After a STOP inside these steps nothing has been saved. Press no key: the run coordinator says how the screen is left, and whether the restore runs.

## Screens and prompts
The screen paths for load and close are row 683-TB's:
- **Load:** Select Company, the book typed into its name filter, the one row, Enter.
- **A company loaded at the start:** C1 says STOP. If the run coordinator's word is to shut it, shut it by Company > Shut, with no change to it.
- **Close (C10):** the `K: Company` label, Shut, Enter on the book.
- **Bring Tally to the front only by its running window.** A second `tally.exe` is a STOP.

**The prompt rule.** The only dialogs answered in this row are:
- C5's: a message that only says the backup is done (screenshot, accept once);
- C6's two: "Accept ?" = Yes, and "Company data folder (100090) already exists ..." = Replace;
- the ledger's own "Accept ?" on Ctrl+A = Yes;
- the screens the write names: the rate of exchange box and the Bill-wise Breakup.

**Any other dialog, message box, sub-screen or prompt, at any step, is unnamed:** press no key, screenshot the whole window, copy its text exactly, send nothing while it is up, and ask the run coordinator (C7). It stays on the screen until the run coordinator answers.

**The path rule.** If a list or line shown is not exactly what a path above names (other words, no such line, a Spelling Error), stop and Esc out of the list (never from a master screen), send nothing, and ask. A list on the way to the write that differs means nothing has been saved.

## The wedge rule
A read that Tally is still building keeps the gateway busy for every company, and giving up on it does not stop Tally's work. So:
- One request at a time, by `run.py`. It never abandons a request: curl waits for its answer.
- This book is small (11 vouchers, 9 ledgers after the write). Its answers on record are under 15,000 B (row 683-TB's `tb1`, 14,140 B, is the largest). If any send has not answered **5 minutes** after it began:
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).

  Nothing is re-sent on this row's own judgement. With the ledger saved and the gateway stuck, the restore waits for the run coordinator's word.
- `run.py`'s own stops (a curl exit other than 0, Unknown Request, an empty answer, an answer above 2,000,000 B) and `lm.py`'s, `tb.py`'s or `op.py`'s STOP are failed requests, never "nothing found". Each stops the row there, and a saved ledger is then restored only on the run coordinator's word.

## Steps
Run each send as `python3 -I run.py http://127.0.0.1:9001 <label> <file>`. Write down each send's start time from its log. **Labels are used once each.** A repeated send takes a new label (`m1r`, `s1r`), never an earlier one's.
0. **C2's step 0** with no company loaded:
   - the screenshot first;
   - one `tally.exe` owning 9001;
   - `GET /status` 200 (record its bytes).

   If no `tally.exe` is running: STOP and report (C2 a); the run coordinator says how Tally is opened.

   Load the book alone, as above. Read the Gateway's date and period, and write them down. Record the release and the licence mode.
1. **`m0`:** `M-marks.xml`, then `python3 -I lm.py a-m0-M-marks.xml`. EXPECT one COMPANY row, `the book's: yes`, ALTVCHID 11, ALTMSTID 214.
   - Any other GUID or row count: STOP.
   - Other figures: record them and STOP before step 2, since the book changed after its last note.
2. **The book as found** (the controls), one send at a time:
   - `s0`: `S-ls-20250401-20250430.xml`, then `python3 -I op.py a-s0-S-ls-20250401-20250430.xml`. EXPECT every row's `CURRENCYNAME` `₹` and `OPENINGBALANCE` `0.00`, and no row named `BRIDGE USD Opening`. The row count is recorded, not an EXPECT: this read has not been sent on the book's present state (on 29 Oct, with four vouchers, it gave 5 rows), and `x0` gives 8.
   - `x0`: `X-lx1-20250401.xml`, then `python3 -I op.py a-x0-X-lx1-20250401.xml`. EXPECT the 8 rows and openings of row 683-LM's `lx1`: each `0.00`, PLAIN.
   - `t0`: `T-tb1-20250401-20250430.xml`, then `python3 -I tb.py a-t0-T-tb1-20250401-20250430.xml`. EXPECT row 683-TB's `tb1`: SET FOREIGN (0); SET MIXED (6) `BRIDGE Export Sales`, `BRIDGE Import Purchases`, `BRIDGE Lab Bank`, `BRIDGE USD Customer`, `BRIDGE USD Supplier`, `Profit & Loss A/c`; SET PLAIN (2) `BRIDGE INR Customer`, `Cash`; SET UNREAD (0).
   - Any of these not as EXPECT: record it, and STOP before the BK. The book or the script reads differently from the rows before, and a write on it would decide nothing.
3. **BK** by C5, to a new empty local folder named `bk-683op`. Record the file's name, size and sha256, and the time the Restore list will show. Then **`mb`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m0-M-marks.xml a-mb-M-marks.xml`. Record its line. A difference is a result to record (C5), not a STOP.
4. **The write.**
   - First **`mc`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-mb-M-marks.xml a-mc-M-marks.xml`. Only on SAME MARKS, and with no key pressed in Tally since `mb`, go on.
   - Then the ledger, by **The write** above.
5. **`m1`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-mc-M-marks.xml a-m1-M-marks.xml`. EXPECT MARKS DIFFER (exit 2), with ALTVCHID as `mc`'s and ALTMSTID above it.
   - Anything else: record it, and go on with step 6. The reads are still taken, and the restore puts the book back.
6. **The book with the ledger,** one send at a time, each answer's counters run before the next send:
   - `s1`: `S-ls-20250401-20250430.xml`. Then `python3 -I op.py --question a-s1-S-ls-20250401-20250430.xml` and `python3 -I op.py --others a-s0-S-ls-20250401-20250430.xml a-s1-S-ls-20250401-20250430.xml`, each recorded whole.
   - `x1`: `X-lx1-20250401.xml`. Then `python3 -I op.py --question a-x1-X-lx1-20250401.xml` and `python3 -I op.py --others a-x0-X-lx1-20250401.xml a-x1-X-lx1-20250401.xml`, each recorded whole.
   - `t1`: `T-tb1-20250401-20250430.xml`. Then, at once, screenshot the whole Tally window: this is the first Trial Balance on this book with an opening balance in it. A dialog = Stop. Then `python3 -I tb.py --question a-t1-T-tb1-20250401-20250430.xml` and `python3 -I tb.py --sets a-t0-T-tb1-20250401-20250430.xml a-t1-T-tb1-20250401-20250430.xml`, each recorded whole, the QUESTION line above all.
   - The last line of each `--question` call is the row's answer for that read. None of them stops the row unless it is a STOP.
7. **`m1r`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m1-M-marks.xml a-m1r-M-marks.xml`. EXPECT SAME MARKS: the reads wrote nothing. On MARKS DIFFER, record it, send nothing else but the restore, and go to step 8.
8. **The restore,** from the BK of step 3, by C6:
   - check the backup file's sha256 against the one recorded, and write `restore-plan.txt`;
   - shut the company;
   - restore the BK's folder alone;
   - answer only "Accept ?" (Yes) and "Company data folder (100090) already exists ..." (Replace);
   - load the company alone, with the screenshots C6 names.

   Then **`m2`:** `M-marks.xml`, and `python3 -I lm.py --same-row a-m0-M-marks.xml a-m2-M-marks.xml`. EXPECT SAME MARKS. Then **`s2`:** `S-ls-20250401-20250430.xml`, and `python3 -I op.py --same a-s0-S-ls-20250401-20250430.xml a-s2-S-ls-20250401-20250430.xml`. EXPECT ROWS SAME, with 8 rows and none named `BRIDGE USD Opening`.
   - MARKS DIFFER or ROWS DIFFER: STOP, and report "the restore of bk-683op did not return the book to m0". Nothing more is sent or keyed.
9. **Close** by C10, with `GET /status` after the last request (C11). One `tally.exe` owns 9001.

Steps 2's sends and step 6 never stop the row on an answer that differs from a QUESTION estimate: each is a result, recorded in full. Only the STOPs named stop it.

## Stop
Press no key, screenshot the whole window, and report before anything else is pressed or sent, if:
- an unnamed dialog appears (the prompt rule), or a second `tally.exe` starts;
- the gateway stops answering, or a send passes the wedge rule's 5 minutes;
- `run.py`, `lm.py`, `tb.py` or `op.py` stops;
- m0 is not one row, the book's, V 11, M 214;
- a control in step 2 is not as EXPECT;
- a step of the write says STOP;
- m2 or s2 is not as EXPECT after the restore.

## Record
- Each answer's bytes and sha256, and each request's sha256 (as the Kit lists; any other = the Stop of C3).
- `lm.py`'s lines for every marks read, and the whole output of each `op.py` and `tb.py` call.
- Each send's start time and how long it took, from `log-<label>.txt`.
- **The write:** every value written down in its steps, word for word as the screen shows it: the bill-by-bill field, the currency field, the opening as shown after the rate, the rate box's title and fields, every field of the Bill-wise Breakup line, Dr or Cr, and every prompt with the key that answered it.
- The BK file (name, folder, bytes, sha256, the Restore list's time), `restore-plan.txt`, the data-folder listings before and after the restore, and every prompt of the restore word for word.
- The marks at m0, mb, mc, m1, m1r and m2; the release, the licence mode, Tally's date and period, and the PC's date.
- Every screenshot by name, bytes and sha256 (screenshots are never pushed).

**Staging the evidence**, as rows 683-LM's and 683-TB's:
- **The start-up check:** a hit is allowed only where it lies inside the amount `$1000000.00`. List each hit with 12 characters either side. A hit anywhere else waits for the run coordinator.
- **The check for 10 or more digits:** it sets aside this book's registered GUID matched whole, and nothing else.

## Expected (estimates, not EXPECTs)
- The controls read as rows 683-LM and 683-TB read them.
- After the write: one row more in `s1` and `x1` than in `s0` and `x0`, and a `BRIDGE USD Opening` row in `t1`; the other eight rows unchanged (OTHERS SAME; in `--sets`, only the new ledger listed).
- Tally may show a difference in opening balances, since the new opening has no other side. Row 683-LM's `lx2` showed no row for its own opening gap, so none is expected in these reads. If one appears without the book's GUID, `op.py` or `tb.py` stops on it: record the row, and the run coordinator decides whether the counter is run again on a copy without it.
- The opening Tally keeps for `$500` at `88` is 44,000.00 rupees, Dr, which XML writes negative.
- Whether that opening is sent as a composite or a plain rupee figure, in each read, is not estimated: that is the question. Row 683-TB found a carried dollar balance plain from the second day, and row 683-LM found every opening at the first day plain `0.00`, but no opening on this book has held dollars before.

## Decides
For the build, by the last line of `tb.py --question` on `t1` (rule 1) and of `op.py --question` on `x1` (rule 3):
- **`t1` COMPOSITE or FOREIGN:** rule 1 sees a dollar opening keyed on a master at the book's first day, on this book and release: PARTIAL. The classified path at `from` equal to the book's first day stands as agreed (partial: one book, one release).
- **`t1` NO COMPOSITE, `x1` COMPOSITE:** rule 1 misses it and rule 3 sees it. The classified path stands, with rule 3 as the one that catches this case, and a test built on these answers.
- **`t1` NO COMPOSITE and `x1` NO COMPOSITE, each with the opening PLAIN and not zero (`-44000.00` expected):** no rule sees it. The residual is real: the classified path needs another check before it ships, or `ledger_movement` keeps its refusal at the first day too. `s1`'s answer says whether the ledger master read could be that check. The choice goes to #683 first; nothing is built on it here.
- **NO COMPOSITE with the opening `0.00`, in `t1` or `x1`:** the opening keyed was not read there (or not saved). The scripts print NO COMPOSITE for any plain opening, zero included, so the figure on the QUESTION line is read too. Record it; the decision waits.
- **NOT ANSWERED, EMPTY or OTHER in `t1` or `x1`,** or `t1` with UNREAD rows: record it; the decision waits, and nothing is built on it.
- **No dollar opening could be keyed** (the write's step 6 STOP): on this release the case arises only on a ledger whose own currency is not the base, which rule 1 sets aside by its `CURRENCYNAME`. That ledger's opening is another variable and gets its own row.
- **OTHERS DIFFER in `s1` or `x1`, or `--sets` naming another ledger:** the write changed more than one ledger. Record it; the answers decide nothing until that is explained.

Whichever way it falls, the answers are parsed again by master's own parsers when they become fixtures, before any code relies on them.

## Not in this row
- `ledger_movement` itself, or any Bridge build;
- a ledger whose own currency is not the base;
- an opening on an existing ledger, or one that holds vouchers too;
- a `from` after the book's first day;
- a second rate, a second foreign currency, or a book whose base is not the rupee;
- another release.
