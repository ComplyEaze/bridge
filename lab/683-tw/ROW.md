# Row 683-TW: on the several-currency lab book, from its first day, do the currency Trial Balance's sets and the movement window's entries stay the same for a `to` other than 30 April

Not for merge. Part of #683. **Read only:** nothing is written, so no BK and no restore. Lines are at master ae38a75c.

## Question
The #683 build plan reads `ledger_movement` on a book with several Currency masters only when `from` is the book's first day. It sets a ledger aside by name when `trial_balance` sets it aside for the same window (rule 1), when an entry on it in the window is a composite (rule 2), or when its opening at `from` is a composite (rule 3). Every measurement behind it is for one window: 1 to 30 April 2025 for the Trial Balance (rows 2071-tool-reads and 683-TB) and the single day 1 April for the movement window (row 683-LM). The plan lists two of the gaps this leaves as unmeasured. This row reads them, on the same book, from the same first day, changing `to` alone (P6).

Every voucher of this book is dated 1 April 2025 (its last voucher date is 20250401), so a window from 1 April holds the same eleven vouchers whatever its `to`. This row cannot put entries on a second day without a write; that stays out (see Not in this row).

**Q1, the Trial Balance's `to`:** from 20250401, are the sets the same for `to` 20250401 (`tb3`, the voucher day alone) and `to` 20260331 (`tb4`, the book's last day) as for `to` 20250430 (`tb1`)? `tb.py --sets` answers each in one line.

**Q2, a movement window longer than the voucher day:** from 20250401 to 20260331, does the book-scoped window (`mvy`) hold the same vouchers with the same entry texts, composites included, as row 683-LM's one-day window? And do the ledger-scoped windows for a kept ledger (`BRIDGE INR Customer`, `mvly-inr`) and a set-aside one (`BRIDGE USD Customer`, `mvly-usd`) hold that ledger's entries as the book-scoped window does? `lm.py --capture` and `lm.py --ledger` answer each in one line.

**The control:** `tb1`, byte-equal to the request row 683-TB sent as its control and to the build's request of 29 Oct. Its sets must read as on 2 Nov. Then a difference in `tb3` or `tb4` is about `to`, not the book's state or the script. The movement window's control is row 683-LM's own answer of 1 April, kept here and not sent (`ref-a-mv-V-mv-20250401.xml`).

Sending master's own builders raw is shape discovery on a lab book, which the owner allows. None of the requests is a new shape: each is a request `trial_balance` or `ledger_movement` already sends, with other dates.

## Book
`BRIDGE FOREXGROUP LAB` (synthetic), GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, TallyPrime 7.2. THE COMPANY in `LAB-COMMON.md` means this book.
- **Synthetic:** it is in master's registry, `scripts/merge-gate-synthetic-uuids.json` line 201 ("created in the lab on 28 October 2026 with invented masters and vouchers keyed by hand").
- **Not an excluded book:** its GUID is not the register lab book's (`REGISTER_LAB_GUID`, `captured_parse_record.rs:302`) and not the one the private exclusion list names.
- **Pinned by GUID.** `lm.py`'s and `tb.py`'s `BOOK` is this GUID. m0 must print one COMPANY row, `the book's: yes`, else STOP before any other send.
- **Its last state** (row 683-OP, `lab/683-op-answers` fb7125ad, `RESULTS.md`): restored from its backup, then m2 byte-equal to m0, V 11, M 214; Tally's date 1-Apr-2025, period 1-Apr-25 to 31-Mar-26, last entry 1-Apr-25. No row has written to it since (queue items 43 and 44 are on another book).
- **Do not change Tally's current date or period.**

## Kit
Check every file against `SHA256.txt` first, and each send's request file again just before it is sent (Get-FileHash, C3). Any file that differs = STOP, and nothing is sent.

| file | what, and where its bytes come from |
|---|---|
| `M-marks.xml` | The marks request (GUID, ALTVCHID, ALTMSTID). Byte-equal to rows 683-LM's, 683-TB's and 683-OP's. |
| `T-tb1-20250401-20250430.xml` | `render_native_trial_balance_request_with_currency` (`native_trial_balance/mod.rs:170`), the period `trial_balance` builds (`tally/runtime_trial_balance.rs:391`, with the profile `observe_read_boundary` gives, `tally/runtime.rs:2052`: `ModeAgnostic` on a licensed gateway, as the render test sets it), 20250401 to 20250430. Byte-equal to row 683-TB's and 683-OP's `tb1`. |
| `T-tb3-20250401-20250401.xml` | The same builder for 20250401 to 20250401. Only `SVTODATE` differs from `tb1`. |
| `T-tb4-20250401-20260331.xml` | The same builder for 20250401 to 20260331. Only `SVTODATE` differs from `tb1`. |
| `V-mvy-20250401-20260331.xml` | `render_agent_movement_vouchers` (`agent_read_profiles.rs:100`), book scope, 20250401 to 20260331. Only the dates differ from row 683-LM's `V-mv-20250401.xml`. |
| `V-mvly-inr-customer-20250401-20260331.xml` | `render_agent_ledger_vouchers` with `LedgerVoucherFields::Movement` (`agent_read_profiles.rs:361`), `CHILDOF` `BRIDGE INR Customer`, the same dates. That ledger is kept (PLAIN) in `tb1`; in row 683-LM's window it has entries on two vouchers. |
| `V-mvly-usd-customer-20250401-20260331.xml` | The same, `CHILDOF` `BRIDGE USD Customer`. Only the dates differ from row 683-LM's `V-mvl-usd-customer-20250401.xml`. That ledger is set aside (MIXED) in `tb1`; its four entries in row 683-LM's window are composites. |
| `ref-a-mv-V-mv-20250401.xml` | Not sent. Row 683-LM's answer to its book-scoped window of 1 April, byte-equal to `lab/683-lm-answers` f832eb17 `answers/a-mv-V-mv-20250401.xml` (sha256 `56554b6a…4df7`, 59,524 B). `lm.py --capture` compares `mvy` with it. |
| `run.py`, `LAB-COMMON.md`, `.gitattributes`, `lm.py` | Rows 683-LM's, 683-TB's and 683-OP's, byte-equal. |
| `tb.py` | Row 683-TB's, byte-equal. Its `--question` mode is not used here, so its question ledger does not matter. |

**How the request bytes were made: a lab-only render test, not a template edit.** `src-tauri/src/agent_lab_683_tw_render_tests.rs` on this branch, one `#[ignore]` test, run once in a build slot:
`LAB_683_TW_OUT=<this folder> cargo test -p bridge --lib lab_683_tw -- --ignored --exact agent::lab_683_tw_render_tests::lab_683_tw_renders_the_row_requests`. It calls the builders above with the book's name and dates, and writes each request as the build sends it, UTF-16LE with a BOM. Before writing, it checks three renders byte for byte against requests already sent on this book, so the encoding and the builders are shown equal:
- the 1 to 30 April Trial Balance against `T-tb1-20250401-20250430.xml`;
- the book-scoped 1 April window against row 683-LM's `V-mv-20250401.xml`;
- the ledger-scoped 1 April window against row 683-LM's `V-mvl-usd-customer-20250401.xml`.

The test and its one `mod` line in `agent.rs` are on this branch only and are never merged.

**`lm.py` and `tb.py` are unchanged** from rows 683-LM and 683-TB, where each was tried (see those sheets). `lm.py --capture` is used here with two movement answers: it compares, voucher by voucher by GUID, each voucher's direct entries (LEDGERNAME and AMOUNT text, in order), which is all it reads; its words "movement" and "capture" name the first and second file. Run on row 683-LM's answer against itself it gives `vouchers: movement 11, capture 11` and CAPTURE SAME.

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
- This book is small (11 vouchers, 8 ledgers). Its answers on record: a Trial Balance of 14,140 B (row 683-TB's `tb1`), a book-scoped window of 59,524 B and a ledger-scoped one of 23,720 B (row 683-LM). A longer `to` adds no voucher. If any send has not answered **5 minutes** after it began:
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

   Load the book alone, as above. Read the Gateway's date and period, and write them down. Record the release and the licence mode: the Gateway's title (a licensed gateway shows no Educational mode) and the `tally.exe` file version, as row 683-LM recorded them.
1. **`m0`:** `M-marks.xml`, then `python3 -I lm.py a-m0-M-marks.xml`. EXPECT one COMPANY row, `the book's: yes`, ALTVCHID 11, ALTMSTID 214.
   - Any other GUID or row count: STOP.
   - Other figures: record them and STOP before step 2, since the book changed after its last note.
2. **`tb1`:** `T-tb1-20250401-20250430.xml`, then `python3 -I tb.py a-tb1-T-tb1-20250401-20250430.xml`, recorded whole. EXPECT, as on 2 Nov:
   - SET FOREIGN (0);
   - SET MIXED (6): `BRIDGE Export Sales`, `BRIDGE Import Purchases`, `BRIDGE Lab Bank`, `BRIDGE USD Customer`, `BRIDGE USD Supplier`, `Profit & Loss A/c`;
   - SET PLAIN (2): `BRIDGE INR Customer`, `Cash`;
   - SET UNREAD (0).
3. **`tb3`:** `T-tb3-20250401-20250401.xml`. Then, at once, screenshot the whole Tally window: this is the first Trial Balance on this book to a `to` other than 30 April. A dialog = Stop. Then `python3 -I tb.py a-tb3-T-tb3-20250401-20250401.xml`, recorded whole.
4. **`tb4`:** `T-tb4-20250401-20260331.xml`, then `python3 -I tb.py a-tb4-T-tb4-20250401-20260331.xml`, recorded whole.
5. **The sets:** `python3 -I tb.py --sets a-tb1-T-tb1-20250401-20250430.xml a-tb3-T-tb3-20250401-20250401.xml`, then the same with `a-tb4-T-tb4-20250401-20260331.xml` in place of `tb3`, each recorded whole. Nothing is sent for this step.
6. **`mvy`:** `V-mvy-20250401-20260331.xml`. Then, at once, screenshot the whole Tally window: this is the first movement window on this book longer than one day. A dialog = Stop. Then `python3 -I lm.py --capture a-mvy-V-mvy-20250401-20260331.xml ref-a-mv-V-mv-20250401.xml`, recorded whole. `--capture` compares each voucher's ledger names and amount texts only, so then `python3 -I lm.py a-mvy-V-mvy-20250401-20260331.xml`, recorded whole: each voucher with its DATE and each entry's ISDEEMEDPOSITIVE.
7. **`mvly-inr`:** `V-mvly-inr-customer-20250401-20260331.xml`. Then `python3 -I lm.py --ledger a-mvly-inr-V-mvly-inr-customer-20250401-20260331.xml a-mvy-V-mvy-20250401-20260331.xml "BRIDGE INR Customer"`, recorded whole.
8. **`mvly-usd`:** `V-mvly-usd-customer-20250401-20260331.xml`. Then `python3 -I lm.py --ledger a-mvly-usd-V-mvly-usd-customer-20250401-20260331.xml a-mvy-V-mvy-20250401-20260331.xml "BRIDGE USD Customer"`, recorded whole.
9. **`m1`:** `M-marks.xml`, then `python3 -I lm.py --same-row a-m0-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS, and m1 byte-equal to m0 (record both sha256). On MARKS DIFFER:
   - send `M-marks.xml` once more as `m1r` (P5, C8) and record it;
   - send nothing else, and STOP.

   The answers already recorded stand, and the report says the marks moved.
10. **Close** by C10, with `GET /status` after the last request (C11). One `tally.exe` owns 9001.

Steps 2 to 8 never stop the row on an answer that differs from EXPECT or from the estimates: each is a result, recorded in full. Only the STOPs named above stop it.

## Stop
Press no key, screenshot the whole window, and report before anything else is pressed or sent, if:
- a dialog or prompt appears, or a second `tally.exe` starts;
- the gateway stops answering, or a send passes the wedge rule's 5 minutes;
- `run.py`, `lm.py` or `tb.py` stops. Their STOPs are listed in their headers and in rows 683-LM's and 683-TB's sheets: an answer that does not decode or parse, carries a LINEERROR, ERROR or DOCTYPE, has no BODY/DATA/COLLECTION, or (for `tb.py`) has a row master's Trial Balance parse would refuse whole;
- m0 is not one row, the book's, V 11, M 214;
- m1 differs from m0.

## Record
- Each answer's bytes and sha256, and each request's sha256 (as `SHA256.txt` lists; any other = the Stop of C3).
- `lm.py`'s lines for m0 and m1, and the whole output of each `tb.py` and `lm.py` call.
- Whether `a-mvy` is byte-equal to `ref-a-mv-V-mv-20250401.xml`, and whether `a-mvly-usd` is byte-equal to row 683-LM's `a-mvl` answer (sha256 `bff19938…9443`, 23,720 B). Either way it is recorded, not judged.
- Each send's start time and how long it took, from `log-<label>.txt`.
- The marks at m0 and m1; the release, the licence mode, Tally's date and period, and the PC's date.
- The screenshots after `tb3` and `mvy`, by name, bytes and sha256 (screenshots are never pushed).

**Staging the evidence**, as row 683-LM's:
- **The start-up check:** a hit is allowed only where it lies inside the amount `$1000000.00`. List each hit with 12 characters either side.
- **The check for 10 or more digits:** it sets aside this book's registered GUID matched whole, and nothing else.

## Expected (estimates, not EXPECTs)
- `tb1` reads as on 2 Nov.
- `tb3` and `tb4` give the same sets as `tb1` (SETS SAME), since every voucher is on 1 April and every window here starts at the book's first day. A Trial Balance to 31 March 2026 may still carry other text in a row, for example in Profit & Loss A/c; that is the question.
- `mvy`: 11 vouchers and CAPTURE SAME. `mvly-inr`: 2 vouchers and LEDGER SAME. `mvly-usd`: 4 vouchers and LEDGER SAME.

## Decides
- **Q1, SETS SAME for both `tb3` and `tb4`:** rule 1's set for a first-day `from` does not move with `to` on this book and release: PARTIAL. The build's replay tests may then use `tb3` and `tb4` beside `tb1`, and the plan's "a `to` other than 30 April" leaves Stays unmeasured, for this book.
- **Q1, SETS DIFFER for either:** the plan's rule 1 is measured for 1 to 30 April only. Record which ledgers moved and how; the build waits until the coordinator has read it.
- **Q1, any UNREAD:** master's parse would refuse that Trial Balance whole, so `trial_balance` would refuse it today. Record the text for #683; the build's rule 1 then refuses that window, as `trial_balance` does.
- **Q2, CAPTURE SAME and both LEDGER SAME:** a window to the book's last day carries the same entries, with the same composites, as the voucher day alone, book-scoped and ledger-scoped: PARTIAL, on a book whose vouchers sit on one day. It says nothing about entries on later days.
- **Q2, CAPTURE DIFFERS or LEDGER DIFFERS:** record the vouchers and texts that differ; rule 2 is then measured for the one-day window only, and the build waits.
- **`tb1` not as EXPECT:** the book or the script reads differently from 2 Nov. `tb3` and `tb4` are recorded, but they decide nothing until that is explained.

Whichever way it falls, the answers are parsed again by master's own parsers when they become fixtures, before any code relies on them.

## Not in this row
- **Entries on more than one day.** Every voucher on this book is on 1 April, and adding one is a write. A window holding entries on two days for a kept ledger stays unmeasured.
- **A composite bill allocation inside a plain entry.** Row 683-LM's window answer holds ten typed bill allocations: eight composites, each inside an entry whose own amount is a composite, and two plain ones inside plain entries. So this book has no such voucher, and reading one needs a write. Until one is captured, the build keeps refusing that case whole.
- `ledger_movement` itself, and how its window plan splits a long window into requests (this row sends each window as one request).
- A `from` after the book's first day (row 683-TB), a second rate after the first day, a second foreign currency, a base other than the rupee, another release, and any write.
