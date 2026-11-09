# Row 683-LM: what `ledger_movement`'s two data requests answer on the several-currency lab book, and whether a ledger's opening ties to its day's movement

Not for merge. Part of #683. **Read only:** nothing is written, so no BK and no restore. Lines are at master ae38a75c.

## Question
On a book with several Currency masters, `ledger_movement` refuses after its currency read (`tally/runtime.rs:3159`, pinned by `a_movement_on_a_several_currency_book_is_refused_before_any_ledger`, `agent_movement_snapshot_tests.rs:660`), so its own data requests have never been answered on such a book. This row sends them raw, with bytes rendered by master's builders, and asks three things:
1. Does the movement voucher window, book-scoped (`mv`) and ledger-scoped (`mvl`), answer each entry's `AMOUNT` with the same text as the `vouchers` capture of the same day, composites included?
2. What `OPENINGBALANCE` text does the movement ledger export carry at `from` 20250401 (`lx1`) and at 20250402 (`lx2`), after a day of composite vouchers?
3. **The decisive check, per ledger:** lx1's opening base part + mv's net base movement on that ledger = lx2's opening base part? Each ledger is TIES, DIFFERS (with the gap) or NOT A FIGURE. A ledger that a revaluation moved with no voucher would DIFFER, and a movement summed from vouchers could then never tie.

Sending master's own builders raw is shape discovery on a lab book, which the owner allows. No request here is a new shape: each is one a release already sends on single-currency books. Only the book is new.

## Book
`BRIDGE FOREXGROUP LAB` (synthetic), GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, TallyPrime 7.2 Silver. THE COMPANY in `LAB-COMMON.md` means this book.
- **Synthetic:** it is in master's registry, `scripts/merge-gate-synthetic-uuids.json` line 201 ("created in the lab on 28 October 2026 with invented masters and vouchers keyed by hand").
- **Not an excluded book:** its GUID is not the register lab book's (`REGISTER_LAB_GUID`, `captured_parse_record.rs:302`) and not the one the private exclusion list names.
- **Pinned by GUID.** `lm.py`'s `BOOK` is this GUID. m0 must print one COMPANY row, `the book's: yes`, else STOP before any other send.
- **Its last state** (row 683-B, `lab/683-build-row-answers` at 7aeedb2e, `RESULTS.md`): V 11, M 214; Tally's date 1-Apr-2025, period 1-Apr-25 to 31-Mar-26; eleven vouchers on 1-Apr-2025, eight with composites. Its extent (`forex-register-book-extent`): BOOKSFROM and LASTVOUCHERDATE 20250401. No row has written to it since.
- **Do not change Tally's current date or period.**

## Kit
Check every file against `SHA256.txt` first, and each send's request file again just before it is sent (Get-FileHash, C3). Any file that differs = STOP, and nothing is sent.

| file | sha256 | what, and where its bytes come from |
|---|---|---|
| `M-marks.xml` | `5b447d03…b4b7` | The marks request (GUID, ALTVCHID, ALTMSTID). Byte-equal to `lab/683-register-answers` `marks.utf16le.xml`, which is also the build's own marks read on this book, and to row 683-B's. |
| `C-currency.xml` | `df262192…69cb` | The currency read, `render_company_currency_request` (`runtime.rs:3140`). Byte-equal to the build's own request on this book: `lab/683-register-answers` `relay/exchange-58.request`, its body (`cur-build-body.bin`). |
| `V-mv-20250401.xml` | `294ee078…e70c` | `render_agent_movement_vouchers` (`agent_read_profiles.rs:100`), book scope, 20250401 to 20250401, unspanned. |
| `V-mvl-usd-customer-20250401.xml` | `b3ee0fad…e0cb` | `render_agent_ledger_vouchers` with `LedgerVoucherFields::Movement` (`agent_read_profiles.rs:361`), `Vouchers : Ledger` `CHILDOF` `BRIDGE USD Customer`, the same day. That ledger is the dollar customer: in the `vouchers` capture its four entries, on four vouchers, are all composites. |
| `X-lx1-20250401.xml` | `de2c64d4…bac1` | `render_native_ledger_export_request` (`native_outstandings/request.rs:258`), with the period `ledger_opening_period` (`runtime.rs:2078`) builds for `from` 20250401 on this extent: 20250401 to 20250401, `ModeAgnostic` (licensed, `runtime.rs:1840`). |
| `X-lx2-20250402.xml` | `ec65454d…e040` | The same builder for `from` 20250402: 20250402 to 20250402 (the later of the last voucher date and `from`). Only the two dates differ from lx1. |
| `K-vouchers-capture-20250401.xml` | `92e0e054…9369` | Not sent. The `vouchers` answer of the same day, byte-equal to master's fixture `vouchers-forex-unspaced-20250401.utf16le.xml` and to `lab/683-v72-voucher-answers` (03940570) `raw/vouchers.response`. `lm.py --capture` compares `mv` with it. |
| `cur-build-body.bin` | `df262192…69cb` | Not sent. The body of the build's exchange 58, which the render test checks `C-currency.xml` against. |
| `run.py`, `LAB-COMMON.md`, `.gitattributes` | | Row 2162-FY14R's, byte-equal (as rows 2162-RU and 2162-GF carry them). |
| `lm.py` | | This row's counter (its header says what each kind and mode prints). |

**How the request bytes were made: a lab-only render test, not a template edit.** `src-tauri/src/agent_lab_683_lm_render_tests.rs` on this branch, one `#[ignore]` test, run once in a build slot:
`LAB_683_LM_OUT=<this folder> cargo test -p bridge --lib lab_683_lm -- --ignored`. It calls the builders above with the book's name and dates, and writes each request as the build sends it, UTF-16LE with a BOM. Before writing, it checks two renders byte for byte against bytes already sent, so the encoding and the builders are shown equal:
- the currency read against the build's own exchange 58 on this book;
- the ledger-scoped movement form against `movement-dn-party-month.request.utf16le.xml`, the fixture `the_movement_builder_renders_the_request_the_lab_sent_byte_for_byte` pins.

It passed on master ae38a75c (1 passed). The test and its one `mod` line in `agent.rs` are on this branch only and are never merged.

**`lm.py` was tried** on master's captures of this book: the marks (`forex-register-marks`: one row, the book's, V 11, M 214), the currencies (`currency_masters_forex_72`: `$` and `₹`), and the `vouchers` capture (11 vouchers; `--capture` against itself: CAPTURE SAME). On copies: one composite `AMOUNT` changed gave CAPTURE DIFFERS on that voucher alone. The capture cut to the dollar customer's vouchers gave LEDGER SAME with 4, and the whole capture gave LEDGER DIFFERS. ALTVCHID moved by one gave MARKS DIFFER (exit 2). An answer with no COLLECTION gave STOP (exit 1). On invented exports: a composite opening read by its base part gave TIES, a plain figure that did not equal gave DIFFERS, and an empty or other opening gave NOT A FIGURE.

## Screens and prompts
This row opens no master, voucher or report screen, and expects no prompt. Every screen path is one an earlier row used on this lab's Tally, cited:
- **Load:** Select Company, the book typed into its name filter, the one row, Enter. Row 683-B loaded this book this way with no prompt (`RESULTS.md`, step 0).
- **A company loaded at the start** (C1 says STOP): if the run coordinator's word is to shut it, Company > Shut, as row 683-B shut the company the row before had left (`RESULTS.md`, step 0), with no change to it.
- **Close (C10):** the `K: Company` label, Shut, Enter on the book, as rows 2162-PB and 2162-B2C shut theirs (`RESULTS.md`, the close).
- **Bring Tally to the front only by its running window.** In row 683-register a desktop "open application" call started a second `tally.exe` (`lab/683-register-answers` `evidence/notes.txt`). A second one is a STOP.

**The path rule.** If a list or line shown is not exactly what a path above names (other words, no such line, a Spelling Error): stop, Esc out of the list (never from a master screen, C7), send nothing, and close by C10 on the run coordinator's word. Nothing has been written, so there is nothing to restore.

## The wedge rule
A read that Tally is still building keeps the gateway busy for every company, and giving up on it does not stop Tally's work. So:
- One request at a time, by `run.py`. It never abandons a request: curl waits for its answer.
- This book is small (11 vouchers, 8 ledgers), and the largest answer on record for it is the `vouchers` capture, 387,856 B. If any send has not answered **5 minutes** after it began: press nothing, and do not end curl. Send nothing more, not even `GET /status`. Screenshot the whole Tally window, and report to the run coordinator (C9). Nothing is re-sent on this row's own judgement.
- `run.py`'s own stops (a curl exit other than 0, Unknown Request, an empty answer, an answer above 2,000,000 B) and `lm.py`'s STOP are failed requests, never "nothing found". Each stops the row there.

## Steps
Run each send as `python3 -I run.py http://127.0.0.1:9001 <label> <file>`, then `python3 -I lm.py a-<label>-<file>`. Write down each send's start time from its log.
0. **C2's step 0** with no company loaded: the screenshot first, one `tally.exe` owning 9001, and `GET /status` 200 (record its bytes). Load the book alone, as above. Read the Gateway's date and period, and write them down. Record the release and the licence mode.
1. **`m0`:** `M-marks.xml`. EXPECT one COMPANY row, `the book's: yes`, ALTVCHID 11, ALTMSTID 214. Any other GUID or row count: STOP. Other figures: record them and STOP before step 2, since the book changed after its last note.
2. **`cur`:** `C-currency.xml`. EXPECT two CURRENCY rows, `$` and `₹` (the input that refuses movement today). Anything else: record it and go on.
3. **`mv`:** `V-mv-20250401.xml`. Then, at once, screenshot the whole Tally window: this is the first movement read on this book (C3). A dialog = Stop. Then `python3 -I lm.py --capture a-mv-V-mv-20250401.xml K-vouchers-capture-20250401.xml`, and record its whole output. EXPECT 11 vouchers and CAPTURE SAME.
4. **`mvl`:** `V-mvl-usd-customer-20250401.xml`. Then `python3 -I lm.py --ledger a-mvl-V-mvl-usd-customer-20250401.xml a-mv-V-mv-20250401.xml "BRIDGE USD Customer"`, recorded whole. EXPECT 4 vouchers and LEDGER SAME.
5. **`lx1`:** `X-lx1-20250401.xml`. Record every ledger's OPENINGBALANCE text, as `lm.py` prints it.
6. **`lx2`:** `X-lx2-20250402.xml`. The same.
7. **The tie:** `python3 -I lm.py --tie a-lx1-X-lx1-20250401.xml a-mv-V-mv-20250401.xml a-lx2-X-lx2-20250402.xml`, recorded whole: one line per ledger, TIES, DIFFERS or NOT A FIGURE. Nothing is sent for this step.
8. **`m1`:** `M-marks.xml`, then `python3 -I lm.py --same-row a-m0-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS and m1 byte-equal to m0 (record both sha256). MARKS DIFFER: send `M-marks.xml` once more as `m1r` (P5, C8), record it, send nothing else, and STOP. The answers already recorded stand, and the report says the marks moved.
9. **Close** by C10, with `GET /status` after the last request (C11). One `tally.exe` owns 9001.

Steps 2 to 7 never stop the row on an answer that differs from EXPECT: each is a result, recorded in full. Only the STOPs named above stop it.

## Stop
Press no key, screenshot the whole window, and report before anything else is pressed or sent, if:
- a dialog or prompt appears, or a second `tally.exe` starts;
- the gateway stops answering, or a send passes the wedge rule's 5 minutes;
- `run.py` or `lm.py` stops;
- m0 is not one row, the book's, V 11, M 214;
- m1 differs from m0.

## Record
- Each answer's bytes and sha256, and each request's sha256 (as the Kit lists; any other = the Stop of C3).
- `lm.py`'s lines for every answer, and the whole output of `--capture`, `--ledger`, `--tie` and `--same-row`.
- Each send's start time and how long it took, from `log-<label>.txt`.
- The marks at m0 and m1; the release, the licence mode, Tally's date and period, and the PC's date.
- The screenshot after `mv`, by name, bytes and sha256 (screenshots are never pushed).

## Expected (estimates, not EXPECTs)
- `mv` and `mvl` carry the same `AMOUNT` text as the `vouchers` capture, composites included.
- The openings at `lx2` are composites on the ledgers whose closings were composites in the trial balance.
- At least one dollar-touched ledger DIFFERS by a revaluation, if the behaviour noted on #683 on 26 Sep holds on this book and day.

## Decides
- **`mv` and `mvl` equal the capture, and every ledger TIES:** movement can take the composite parse for its entries, as `vouchers` does (`read_composites`, `agent_voucher_parse.rs:599`), and the same parse for the opening. The currency gate at `runtime.rs:3159` can then be lifted for movement alone, with its own test on these captures. Still PARTIAL: one book, one day, one currency.
- **Any ledger DIFFERS:** a movement summed from vouchers cannot tie on a several-currency book. That ledger must be refused by name, or answered with the gap stated, never as a balance. Which of the two is put to the run coordinator before any code.
- **`mv` differs from the capture:** the named fetch list has its own `AMOUNT` layout. Its answers become fixtures, and the parse is tested on them before any adoption.
- **Any NOT A FIGURE:** that ledger refuses, and its text is recorded for #683.

## Not in this row
The bill trail, import verification and the desktop reads; a second foreign currency; a book whose base is not the rupee; a window longer than one day; any write.
