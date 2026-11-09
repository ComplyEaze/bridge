# Row 683-TB: on the several-currency lab book, does a dollar ledger's currency Trial Balance row carry a composite when its balance is carried into the window with no entry in it

Not for merge. Part of #683. **Read only:** nothing is written, so no BK and no restore. Lines are at master ae38a75c.

## Question
Row 683-LM (`lab/683-lm-answers` f832eb17) found that five of this book's eight ledgers do not tie a day's movement to the next day's opening. The choice made on it is that `ledger_movement` sets aside, by name, every ledger that `trial_balance` sets aside for the same window. That is rule 1 of the choice: a ledger whose `CURRENCYNAME` is not the base master's NAME, or any of whose `TBALOPENING`, `DEBITTOTALS`, `CREDITTOTALS` or `TBALCLOSING` is a currency composite (`parse_native_trial_balance_with_currency`, `native_trial_balance/wire.rs:41`). Rules 2 and 3 (a composite entry in the window, a composite opening at `from`) come from the movement's own reads.

Rule 1 has been measured only on a window that starts at the book's start and holds the vouchers: `trial_balance` for 1 to 30 April (`lab/2071-tool-reads-answers` 33034e8f, at this book's present marks) set aside six ledgers as `mixed_currency_movement` and kept two. A ledger that carries a dollar balance into a window with no entry in it is the case rules 2 and 3 cannot see: its opening in the movement's export is a plain figure (row 683-LM's `lx2`: `BRIDGE USD Customer` `-87912000.00`). This row asks whether rule 1 sees it.

Every voucher of this book is dated 1 April 2025, and its last voucher date is 20250401. So a window from 2 April holds no entry, and every ledger's balance in it is carried in.

**The question:** in the currency Trial Balance for 20250402 to 20250430 (`tb2`), is any of `BRIDGE USD Customer`'s four values a composite? `tb.py --question` answers it in one line. `BRIDGE USD Supplier` (a dollar part of $0.00 carried in) and `BRIDGE Lab Bank` (a dollar part carried in) are recorded beside it.

**The control:** the same request for 20250401 to 20250430 (`tb1`), byte-equal to the one the build sent on this book on 29 Oct. Its sets must read as on 2 Nov. Then a "no composite" in `tb2` is about the window, not the book's state or the script. `tb1` and `tb2` differ in `SVFROMDATE` alone (P6).

Sending master's own builder raw is shape discovery on a lab book, which the owner allows. Neither request is a new shape: `trial_balance` sends this request on every several-currency book, with the caller's dates, and has sent it on this one.

## Book
`BRIDGE FOREXGROUP LAB` (synthetic), GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, TallyPrime 7.2 Silver. THE COMPANY in `LAB-COMMON.md` means this book.
- **Synthetic:** it is in master's registry, `scripts/merge-gate-synthetic-uuids.json` line 201 ("created in the lab on 28 October 2026 with invented masters and vouchers keyed by hand").
- **Not an excluded book:** its GUID is not the register lab book's (`REGISTER_LAB_GUID`, `captured_parse_record.rs:302`) and not the one the private exclusion list names.
- **Pinned by GUID.** `lm.py`'s and `tb.py`'s `BOOK` is this GUID. m0 must print one COMPANY row, `the book's: yes`, else STOP before any other send.
- **Its last state** (row 683-LM, `RESULTS.md`): V 11, M 214, m1 byte-equal to m0; Tally's date 1-Apr-2025, period 1-Apr-25 to 31-Mar-26; eleven vouchers on 1-Apr-2025. No row has written to it since.
- **Do not change Tally's current date or period.**

## Kit
Check every file against `SHA256.txt` first, and each send's request file again just before it is sent (Get-FileHash, C3). Any file that differs = STOP, and nothing is sent.

| file | what, and where its bytes come from |
|---|---|
| `M-marks.xml` | The marks request (GUID, ALTVCHID, ALTMSTID). Byte-equal to row 683-LM's, and to `lab/2071-forex-rerun` `requests/marks.utf16le.xml`. |
| `T-tb1-20250401-20250430.xml` | `render_native_trial_balance_request_with_currency` (`native_trial_balance/mod.rs:170`), the period `trial_balance` builds (`tally/runtime_trial_balance.rs:391`, `ModeAgnostic` on a licensed gateway), 20250401 to 20250430. Byte-equal to `tb-build-request-20250401-20250430.bin`. |
| `T-tb2-20250402-20250430.xml` | The same builder for 20250402 to 20250430. Only `SVFROMDATE` differs from `tb1`. |
| `tb-build-request-20250401-20250430.bin` | Not sent. The request the build sent on this book on 29 Oct (`lab/2071-forex-rerun` 0d7eb0bf, `requests/tb-currency.utf16le.xml`, made by the build's own builder). The render test checks `tb1` against it. |
| `run.py`, `LAB-COMMON.md`, `.gitattributes`, `lm.py` | Row 683-LM's, byte-equal. `lm.py` is used here for the marks only. |
| `tb.py` | This row's counter for a Trial Balance answer (its header says what each class, verdict and mode prints). |

**How the request bytes were made: a lab-only render test, not a template edit.** `src-tauri/crates/bridge-tally-protocol/tests/lab_683_tb_render.rs` on this branch, one `#[ignore]` test, run once in a build slot:
`LAB_683_TB_OUT=<this folder> cargo test -p bridge-tally-protocol --test lab_683_tb_render -- --ignored --exact lab_683_tb_renders_the_row_requests`. It calls the builder with the book's name and the dates, and writes each request as the build sends it, UTF-16LE with a BOM. Before writing, it checks the 1 to 30 April render byte for byte against the request of 29 Oct. That shows the encoding and the builder equal, and the builder's text has not changed since that build (`git diff af4a0dbf ae38a75c` on `native_trial_balance/mod.rs` touches no renderer). The test is on this branch only and is never merged.

**`tb.py` mirrors master's parse, rule for rule:** its composite check is `is_currency_composite` and `is_unspaced_currency_composite` (`currency_composite.rs`) written out in Python, each value is classed on its text as sent (never trimmed), and it stops where `parse_envelope` and `parse_native_trial_balance_with_currency` (`native_trial_balance/wire.rs`) refuse the whole answer. Its header lists every STOP.

**`tb.py` was tried:**
- **On master's fixture `trial_balance_currency_forex_live`** (7.1, another book, with `BOOK` and `BASE` set to that book's GUID and `I₹` in a scratch copy): FOREIGN 3, MIXED 3, PLAIN 4. These are the same ledgers that master's parse record for that fixture sets aside and keeps (`tests/parse_record/trial_balance_currency_forex_live.utf16le.xml.parse_native_trial_balance_with_currency.txt`).
- **On this book's 29 Oct answer** (`lab/2071-forex-rerun-answers` 0d7eb0bf, `raw/tb-currency.response`, four vouchers then): MIXED `BRIDGE Export Sales`, `BRIDGE Lab Bank`, `BRIDGE USD Customer` and `Profit & Loss A/c`, and PLAIN `Cash`. `--question` gave COMPOSITE (DEBITTOTALS, CREDITTOTALS, TBALCLOSING).
- **On copies of that answer**, the question ledger's row unless named:
  - its three composites made plain: NO COMPOSITE;
  - then one of them written `87 008.00`, or with Arabic-Indic digits in its rate, or a 7.2 composite with one decimal, or a value with a leading space: OTHER, UNREAD, and NOT ANSWERED (OTHER in that field);
  - a 7.1 composite with three spaces before `=`, a 7.1 composite with no decimals and a rate of `87.5`, or a symbol `US.$`: COMPOSITE;
  - one composite made plain, the others kept: still COMPOSITE on those;
  - its `CURRENCYNAME` made `$`: FOREIGN ('$'), and `--sets` against the original gave SETS DIFFER naming it, while the original against itself gave SETS SAME;
  - `Cash`'s debit written `-1,000.00`: UNREAD, with the line that master refuses the answer;
  - the row renamed: NOT ANSWERED (no row);
  - its GUID's book part in capitals: read as the book's, as master compares it;
  - each a STOP (exit 1): its GUID one digit off the book's, `Cash` given its GUID in capitals, `Cash` renamed to it in capitals, its `CURRENCYNAME` or `TBALOPENING` removed, STATUS `0`, a LINEERROR added, every LEDGER row removed, no COLLECTION, an unclosed tag, bytes that are not UTF-16LE.

## Screens and prompts
This row opens no master, voucher or report screen, and expects no prompt. The screen paths are row 683-LM's, which loaded and closed this book that way with no prompt (`lab/683-lm-answers` `RESULTS.md`, steps 0 and 9):
- **Load:** Select Company, the book typed into its name filter, the one row, Enter.
- **A company loaded at the start:** C1 says STOP. If the run coordinator's word is to shut it, shut it by Company > Shut, as row 683-LM shut the start-up company, with no change to it.
- **Close (C10):** the `K: Company` label, Shut, Enter on the book.
- **Bring Tally to the front only by its running window.** A second `tally.exe` is a STOP.

**The path rule.** If a list or line shown is not exactly what a path above names (other words, no such line, a Spelling Error), then:
1. stop, and Esc out of the list (never from a master screen, C7);
2. send nothing;
3. close by C10 on the run coordinator's word.

Nothing has been written, so there is nothing to restore.

## The wedge rule
A read that Tally is still building keeps the gateway busy for every company, and giving up on it does not stop Tally's work. So:
- One request at a time, by `run.py`. It never abandons a request: curl waits for its answer.
- This book is small (11 vouchers, 8 ledgers). Its Trial Balance answer on record is 9,880 B (29 Oct). If any send has not answered **5 minutes** after it began:
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).

  Nothing is re-sent on this row's own judgement.
- `run.py`'s own stops (a curl exit other than 0, Unknown Request, an empty answer, an answer above 2,000,000 B) and `lm.py`'s or `tb.py`'s STOP are failed requests, never "nothing found". Each stops the row there.

## Steps
Run each send as `python3 -I run.py http://127.0.0.1:9001 <label> <file>`. Write down each send's start time from its log.
0. **C2's step 0** with no company loaded:
   - the screenshot first;
   - one `tally.exe` owning 9001;
   - `GET /status` 200 (record its bytes).

   Load the book alone, as above. Read the Gateway's date and period, and write them down. Record the release and the licence mode.
1. **`m0`:** `M-marks.xml`, then `python3 -I lm.py a-m0-M-marks.xml`. EXPECT one COMPANY row, `the book's: yes`, ALTVCHID 11, ALTMSTID 214.
   - Any other GUID or row count: STOP.
   - Other figures: record them and STOP before step 2, since the book changed after its last note.
2. **`tb1`:** `T-tb1-20250401-20250430.xml`, then `python3 -I tb.py a-tb1-T-tb1-20250401-20250430.xml`, recorded whole. EXPECT, as on 2 Nov:
   - SET FOREIGN (0);
   - SET MIXED (6): `BRIDGE Export Sales`, `BRIDGE Import Purchases`, `BRIDGE Lab Bank`, `BRIDGE USD Customer`, `BRIDGE USD Supplier`, `Profit & Loss A/c`;
   - SET PLAIN (2): `BRIDGE INR Customer`, `Cash`;
   - SET UNREAD (0).
3. **`tb2`:** `T-tb2-20250402-20250430.xml`. Then, at once, screenshot the whole Tally window: this is the first Trial Balance on this book from a date after its start. A dialog = Stop. Then `python3 -I tb.py --question a-tb2-T-tb2-20250402-20250430.xml`, recorded whole, its last line above all.
4. **The sets:** `python3 -I tb.py --sets a-tb1-T-tb1-20250401-20250430.xml a-tb2-T-tb2-20250402-20250430.xml`, recorded whole. Nothing is sent for this step.
5. **`m1`:** `M-marks.xml`, then `python3 -I lm.py --same-row a-m0-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS, and m1 byte-equal to m0 (record both sha256). On MARKS DIFFER:
   - send `M-marks.xml` once more as `m1r` (P5, C8) and record it;
   - send nothing else, and STOP.

   The answers already recorded stand, and the report says the marks moved.
6. **Close** by C10, with `GET /status` after the last request (C11). One `tally.exe` owns 9001.

Steps 2 to 4 never stop the row on an answer that differs from EXPECT: each is a result, recorded in full. Only the STOPs named above stop it.

## Stop
Press no key, screenshot the whole window, and report before anything else is pressed or sent, if:
- a dialog or prompt appears, or a second `tally.exe` starts;
- the gateway stops answering, or a send passes the wedge rule's 5 minutes;
- `run.py`, `lm.py` or `tb.py` stops (`tb.py`: an answer that does not decode or parse, carries a DOCTYPE, LINEERROR or ERROR, has a STATUS other than one `1`, not one COLLECTION or no LEDGER row, a row without NAME, CURRENCYNAME or one of the four values, a GUID that is not the book's, two rows with one GUID or one name in any case, two of one child element in a row, or no row in the base currency);
- m0 is not one row, the book's, V 11, M 214;
- m1 differs from m0.

## Record
- Each answer's bytes and sha256, and each request's sha256 (as the Kit lists; any other = the Stop of C3).
- `lm.py`'s lines for m0 and m1, and the whole output of each `tb.py` call.
- Each send's start time and how long it took, from `log-<label>.txt`.
- The marks at m0 and m1; the release, the licence mode, Tally's date and period, and the PC's date.
- The screenshot after `tb2`, by name, bytes and sha256 (screenshots are never pushed).

**Staging the evidence**, as row 683-LM's:
- **The start-up check:** a hit is allowed only where it lies inside the amount `$1000000.00`. List each hit with 12 characters either side.
- **The check for 10 or more digits:** it sets aside this book's registered GUID matched whole, and nothing else.

## Expected (estimates, not EXPECTs)
- `tb1` reads as on 2 Nov.
- In `tb2` no ledger has a debit or a credit: `DEBITTOTALS` and `CREDITTOTALS` are empty on every row.
- `BRIDGE USD Customer`'s `TBALOPENING` and `TBALCLOSING` in `tb2` are either composites, as its closing is in `tb1`, or plain figures, as its opening is in the movement's export at 20250402. Neither is estimated over the other: that is the question.

## Decides
- **QUESTION: COMPOSITE:** rule 1 sees a dollar balance carried into a window with no entry in it, on this book, release and window: PARTIAL. A build may then lift the several-currency refusal for a `from` after the book's start, with these answers as fixtures.
- **QUESTION: NO COMPOSITE** (its currency the base and all four values plain or empty): rule 1 misses that ledger. The build keeps refusing a several-currency book for a `from` after the book's start.
- **QUESTION: NOT ANSWERED:** the answer has no `BRIDGE USD Customer` row, or a value on it is neither a plain figure nor a composite. Record it; the decision waits, and nothing is built on it.
- **QUESTION: FOREIGN:** `trial_balance` sets the ledger aside by its currency, so rule 1 still sees it. But every ledger read `₹` on 2 Nov, so it decides nothing until that is explained.
- **Any UNREAD in `tb2`:** master's parse would refuse that Trial Balance whole. Record the text for #683.
- **`tb1` not as EXPECT:** the book or the script reads differently from 2 Nov. `tb2` is recorded, but it decides nothing until that is explained.
- **`BRIDGE USD Supplier` and `BRIDGE Lab Bank`:** recorded, and they decide nothing alone. USD Supplier's dollar part is $0.00, so a plain row there is not a miss.

Whichever way it falls, the answers are parsed again by master's own `parse_native_trial_balance_with_currency` when they become fixtures, before any code relies on them.

## Not in this row
- `ledger_movement` itself;
- a window that holds entries as well as a carried balance;
- a second rate after the book's start;
- a second foreign currency, or a book whose base is not the rupee;
- another release;
- any write.
