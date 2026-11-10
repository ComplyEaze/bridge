# Row 2346-TD: does master's own bills read, at the period's last day, show the bills a note is checked against?

Shape discovery, as rows 2346-AR and 2346-PB: the notes are requests written for the lab, not requests the build renders, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2346 (0.9.0), check 4 of the capture-6 comment. Written 10 Nov 2026. Lines are at master ae38a75c. Confidence of everything here until the row runs: unverified.

## Question
The capture-6 comment's check 4: the note builder reads the customer's bills "at the end of the books, not at the note's date", the last day of the current period. Row 2346-PB (`lab/2346-pb-answers` at bc52b5ce) read the bills only at the notes' own date, 1 Aug 2026.

PB's read was also not one master can send.
- **The name was refused.** `fetch_party_bills` takes the party only as a `LedgerScopeName` (`src-tauri/src/tally/runtime_party_bills.rs:61`). That type admits ASCII letters, digits and single spaces (`src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65`, `:74`). PB's customer, `BRIDGE Walk-in BC`, is refused for its hyphen. So the `open_bills` detail refuses it as `party_name_not_admitted` before any send (`src-tauri/src/agent_party_bills.rs:28`). The correction on #2346 says so.
- **This row rebuilds PB's state with one change, the customer's name, to `BRIDGE Walkin BC`, which the type admits.** It then sends master's own requests, rendered through the type, twice:
  - **the control:** at PB's to-date, 20260801;
  - **the case:** at the period's last day, 20270331. Only `SVTODATE` differs (step 6).

**The questions:**
1. **Does master's own request read PB's state?** That is the control's `pb.py --outcome`, and its answers diffed with PB's s8 answers, the name mapped.
2. **Does `NativeLedgerSnapshotPeriod` admit the books' first day to the period's last day?** Answered from the code before the row (Files, the render test): yes, under both profiles. `ModeAgnostic` admits every day, and `EducationRestricted` admits a day 1, 2 or 31 (`src-tauri/crates/bridge-tally-protocol/src/outstandings_shared.rs:182` to `:186`). `fetch_party_bills` refuses `EducationRestricted` before that (`runtime_party_bills.rs:74` to `:76`).
3. **At 20270331, what does the read return, against the control:**
   - the same rows, each with the same `BILLCL`?
   - the same snapshot closing?
   - the same outcome?

**The one variable** in each of steps 5 and 6 is the to-date. Steps 1 to 4 rebuild the state. The rebuild is not a variable: the variable against PB is the name.

**Not asked here:**
- **ComplyEaze Bridge's own transport.** No row of #2377 sent the bills read through a Bridge build or Bridge's transport: rows 2377-L, 2377-B and 2377-NB each say no build ran. So no path is shown that this row could follow. It stays unmeasured, and this row sends by curl, as PB did.
- **A voucher dated after the notes,** such as a receipt that settles a bill before the period's end. That is the reason for check 4, and a row of its own.
- Not asked either: another to-date; a Debit Note; the build's own note.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2346-PB's, 2346-AR's and 2377-NB's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count20.py` and `pb.py` each hold BRIDGE PILOT LAB's GUID as `BOOK`.
  - Each M answer prints whether the COMPANY row's GUID is the book's.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is sent unless s1's COMPANY row is the book's.
- **Its state:** every row queued before this one ends with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245. PB's restore did (its s9). Other marks at s1: stop before BK, with nothing sent.
- **It holds no `BRIDGE Walkin BC`, no Credit Note and none of the numbers below.** Step 2 checks this, and a row there is a STOP.
- **The numbers and the keys are PB's, as PB sent them:**
  - the invoices `BP/26-27/0099` and `BP/26-27/0101`;
  - the `REMOTEID` tails `000000000d10` to `000000000d15`;
  - the note named `BP/26-27/0098`.

  PB's restore took every one of them away (its s9: each N and R read 0 rows).
- **The Credit Note numbers:** Credit Note numbers automatically on this book. The four notes are expected to take `1` (c1), `2` (p), `3` (c2) and `4` (c3), in the order sent, as on PB. If a note takes another number, the R-CN read for it finds nothing. Record the number from Tally's screen (Display, the day book of 1 Aug 2026) and say so. That is not a finding about the bills.
- **The date and the period:** read the Gateway's current date and period at step 1.
  - If the date is before 1-Aug-2026, stop.
  - If the period is not 1-Apr-26 to 31-Mar-27, stop: the case's to-date is that period's last day. Rows 2346-PB and 2377-NB read 1-Apr-26 to 31-Mar-27 and Tuesday 15-Sep-2026.
  - **Do not change Tally's current date or period.**
- **BK is this row's way back.**
  - It is taken at step 1, before anything is sent, and restored at step 8 by C6.
  - The restore runs on every path from step 3 on that is not a STOP. After a STOP (C7, C8 or C9), C8 holds, and the restore waits for the run coordinator's word.
  - After it, every read of step 2 is sent again and compared whole. Any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. PB read TallyPrime with `tally.exe` 1.1.7.2, from the program's file.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6). The writes are `L-ledger.xml`, `I-0099.xml`, `I-0101.xml`, `I-CN-1.xml`, `I-CN-P.xml`, `I-CN-2.xml` and `I-CN-3.xml`, each sent at most once.

**This row's exception to C3.** C3 says a file that writes is sent once in the item's life.
- **What is new here.** Each of this row's seven writes is a new file. But each carries PB's numbers and `REMOTEID`s, which PB's sitting sent, so the same keys are sent to this book a second time.
- **Why that is the row's method, not its finding.** PB's restore removed them, and the row rebuilds PB's state.
- **It is sent only if the run coordinator said yes to this exception when queuing the row.** Without that yes, nothing is sent from step 3 on: record that and close.
- **The rest of C3 holds.** Each of this row's files is sent at most once, and never again whatever happened to its first send.

## The sends and the wedge rule
`run.py` is row 2377-NB's (as PB's), which also keeps each answer's response headers, by curl -D, in `a-<label>-<file>.headers`.
- One request at a time. `run.py` never abandons a request: curl waits for its answer.
- **The wedge rule.** If any send has not answered **5 minutes** after it began:
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).

  Nothing is re-sent on this row's own judgement. On this book, PB's snapshot answered 26,724 B and the report 3,720 B. A read-back that finds a voucher answers about 95,834 B. The case's snapshot covers the whole year and has not been sent: its time is part of the result.
- **The stops.** `run.py`'s stops are failed requests, never "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 B. So are `count20.py`'s, `pb.py`'s and `td-diff.py`'s STOPs. Each stops the row there, with one exception.
- **The one exception: an empty answer to an `L-` request.** That case is #2473's domain. The exception is row 2377-NB's and PB's, unchanged, and it needs the run coordinator's order for this row, as they had one.
  - It is recorded first as what it is: `run.py` printed "empty answer", so the request failed.
  - It is not a STOP, and its pair's second send is allowed, only if its headers file shows HTTP status 200 and `CONTENT-LENGTH: 0`. `run.py`'s printed "stop, do not resend" is then read as this exception.
  - Any other empty body (no headers file, no status line, another status or another length) ended without an answer. That is a STOP, and the request is never sent again. The rest of C3 holds.
  - Then, in this order:
    1. keep the headers file and the log line, and screenshot the whole Tally window;
    2. if a dialog is up: STOP;
    3. send `GET /status` once (C2c's command), recording its status and bytes; if it is not 200: STOP;
    4. run `pb.py` on that answer, and go on with the step. `td-diff.py` is not run on it.
  - An empty answer to any other request is a STOP, as `run.py` says.
- Check each file's sha256 against `SHA256.txt` before its send.
- Close no Bridge process, and call no Bridge tool. That is the lab's standing rule, as on PB.

**Which script reads which answer**
- **`count20.py`** (PB's, with its default customer changed, marked TD) reads `a-*-M-marks.xml`, `a-*-N-*`, `a-*-R-*`, `a-*-B-*`, and the import answers `a-*-I-*` and `a-*-L-ledger.xml`. Its B kind prints the customer's groups (`--ledger` defaults to `BRIDGE Walkin BC`).
- **`pb.py`** (PB's, with `PARTY` changed, marked TD) reads `a-*-S-snap-*` and `a-*-L-lo-*`. It takes each answer's to-date from its file name, so the case's bills are read in the window to 20270331.
- **`td-diff.py`** compares two answers and prints what differs. It records, and it never decides: it exits 0 whether they are equal or not.
  - `--map TD PB` compares an answer of this row with PB's, with PB's name turned into this row's in PB's text first.
  - `--same A B` compares two answers of this row as they are.
- `count20.py` and `pb.py` share no answer file. Give each script only its own files.

**A bills read at a to-date** (named BR below) is four labels, `fetch_party_bills`'s sends in its order (`runtime_party_bills.rs:89`, then `:101`):
1. `<step>sa`: the snapshot;
2. `<step>sb`: the same request again;
3. `<step>la`: the report. Screenshot the whole Tally window at once (shot), before the next request;
4. `<step>lb`: the same request again.

If the two snapshot answers are not byte-equal, master sends nothing more. This row still sends 3 and 4, and records that master would not have.

Then:
- `pb.py` on both S answers and both L answers;
- `pb.py --outcome <sa> <sb> <la> <lb>`, recorded whole, its OUTCOME line above all.

## Files
**Byte-equal to row 2346-PB's file of the same name** (`lab/2346-pb-sheet` at 53540665, folder `lab/2346-pb/`):
- `.gitattributes`, `LAB-COMMON.md`, `run.py` and `M-marks.xml`;
- the reads: `N-0099.xml`, `N-0101.xml`, `R-0099.xml`, `R-0101.xml`, `R-CN1.xml` to `R-CN4.xml`, `B-bills.xml`, `B-payable.xml` and `S-snap-20260401-20260801.xml`. None holds the customer's name;
- in `pb/`, the seven writes and the report request as PB sent them: `L-ledger.xml`, `I-0099.xml`, `I-0101.xml`, `I-CN-1.xml`, `I-CN-P.xml`, `I-CN-2.xml`, `I-CN-3.xml` and `L-lo-bc-20260801.xml`. They are never sent. `gen-td.py` and the render test start from them.

**Byte-equal to row 2377-NB's file** (`lab/2377-nb-sheet` at 03d7763a): `snapshot-build-request-forex-20250401-20250430.bin`. It is never sent: it is the request the build sent on another lab book on 29 Oct, which the render test checks the snapshot builder against first.

**Changed from PB's, by one line each:**
- **`pb.py`:** `PARTY` is `BRIDGE Walkin BC` (marked TD).
- **`count20.py`:** the B kind's default ledger is `BRIDGE Walkin BC` (marked TD, with its header line).

**Made by `gen-td.py`** from the files in `pb/`, each checked against PB's `SHA256.txt` first. Each is PB's request with `BRIDGE Walk-in BC` turned into `BRIDGE Walkin BC`, and nothing else. For each, it asserts:
- the old name occurs as often as expected (`L-ledger.xml` 2 times, each import 7), and every `Walk-in` in the file is part of it;
- the new name does not occur before;
- turned back, the file is PB's byte for byte.

Its output: `L-ledger.xml`, `I-0099.xml`, `I-0101.xml`, `I-CN-1.xml`, `I-CN-P.xml`, `I-CN-2.xml` and `I-CN-3.xml`. Each is 2 B shorter per occurrence than PB's, the hyphen's UTF-16 unit.

**Made by master's own builders, through the type:** `L-lo-wk-20260801.xml`, `S-snap-20260401-20270331.xml` and `L-lo-wk-20270331.xml`.
- **How they were made.** By a lab-only render test, row 2377-NB's path: `src-tauri/crates/bridge-tally-protocol/tests/lab_2346_td_render.rs` on this branch, one `#[ignore]` test, run once in a build slot, `LAB_2346_TD_OUT=<this folder> cargo test -p bridge-tally-protocol --test lab_2346_td_render -- --ignored --exact lab_2346_td_renders_the_row_requests --nocapture`. It calls `render_native_ledger_snapshot_request` with a `NativeLedgerSnapshotPeriod`, and `render_ledger_outstandings_request` with a `LedgerScopeName`, as `fetch_party_bills` does (`runtime_party_bills.rs:80` to `:82`, `:84`, `:96`). It writes each request as the build sends it, UTF-16LE with a BOM. The test is on this branch only and is never merged.
- **What it asserted first,** each passing:
  - the snapshot builder for the 29 Oct request: byte-equal to `snapshot-build-request-forex-20250401-20250430.bin`;
  - the snapshot builder at 20260801: byte-equal to PB's `S-snap-20260401-20260801.xml`. So PB's snapshot request was master's;
  - the report builder at 20260801 for `BRIDGE Walkin BC`: byte-equal to PB's `L-lo-bc-20260801.xml` with the name mapped;
  - `LedgerScopeName::new("BRIDGE Walk-in BC")` is `Err(Character)`, and `BRIDGE Walkin BC` is admitted unchanged.
- **What it printed:** `NativeLedgerSnapshotPeriod` from 20260401 to 20270331, `ModeAgnostic`: admitted true; `EducationRestricted`: admitted true.
- **The pairs differ by the date only.** `S-snap-20260401-20270331.xml` differs from `S-snap-20260401-20260801.xml`, and `L-lo-wk-20270331.xml` from `L-lo-wk-20260801.xml`, only in `SVTODATE`'s digits. `L-lo-wk-20260801.xml` differs from PB's report request only by the hyphen.

**`pb-answers/`:** 19 of PB's answers, byte-equal to `lab/2346-pb-answers` at bc52b5ce, for `td-diff.py --map`. They are never sent:
- `a-s1-M-marks.xml`;
- the seven import answers, `a-s3-L-ledger.xml` to `a-s8-I-CN-3.xml`;
- `a-s6r-R-0101.xml`, `a-s6r-R-CN2.xml` and `a-s7r-R-CN3.xml`;
- `a-s8r-R-CN4.xml`, `a-s8r-R-CN1.xml`, `a-s8r-R-0099.xml`, `a-s8r-B-bills.xml`, `a-s8r-B-payable.xml` and `a-s8r-M-marks.xml`;
- `a-s8sa-S-snap-20260401-20260801.xml` and `a-s8la-L-lo-bc-20260801.xml`.

**Tried before this sheet was sent**, each as stated:
- `gen-td.py`: its seven files, each check passing.
- `pb.py --outcome`, on PB's s8 answers with the name mapped and renamed to this row's labels and the 20270331 to-date (headers files copied): OUTCOME told, ROWS 3, sum 1576.50 equals closing 1576.50.
- `count20.py`'s B kind, on PB's s8r Bills Receivable and Bills Payable with the name mapped: 1 group for `BRIDGE Walkin BC` (`BP/26-27/0101` -525.50), and 2 (`BP/26-27/0099` and `BP/26-27/0098`, 1051.00 each).
- `td-diff.py`:
  - `--map` of PB's s8 snapshot and report, with the name mapped, against `pb-answers/`: EQUAL (2 and 0 names mapped);
  - the same with one amount changed: DIFFER, 2 lines, both shown;
  - with every `0` changed to `1`: DIFFER, 276 lines, 40 shown and the count of the rest;
  - `--same` of a report with itself: EQUAL;
  - an empty file and a missing file: STOP, exit 1;
  - `--map` with a file that already holds this row's name: STOP, exit 1.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each label as `python3 -I run.py <gateway URL> <label> <files>`. The prompt rule, the wait rule and the import answers are PB's.

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named.
- For any other prompt, message or dialog: copy it word for word and screenshot it first.
- Then answer No, or Esc if it offers no No. Press nothing more and send nothing.
- Then restore BK by C6, compare as step 8, and close (C10).
- That prompt is this row's result.

**Each import's answer** is one of these:
- `CREATED` 1 with `ERRORS` 0;
- `CREATED` 0 with `ERRORS` 1 and a LINEERROR;
- `CREATED` 0 with `EXCEPTIONS` 1.

Any other answer, or a dialog after it: the prompt rule. Never send an import file twice (C3).

**Each write, the same way:**
- `run.py URL <step>m M-marks.xml`, then `python3 -I count20.py --before-write a-s1-M-marks.xml a-<step>m-M-marks.xml`.
- Only if that passes, and no key was pressed in Tally since, send the import alone: `run.py URL <step> <file>`.
- Then screenshot the whole window before the next request (C3).
- Then `count20.py` on the answer, and `python3 -I td-diff.py --map a-<step>-<file> pb-answers/a-<step>-<file>`, each recorded whole.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down (the Book's date-and-period bullet).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK. Then `td-diff.py --map a-s1-M-marks.xml pb-answers/a-s1-M-marks.xml`, recorded.
   - **Take BK** by C5, to the new empty folder `C:\lab\2346-td\backup-1`. Row 2346-PB's step 1 is the path.
     - Tally may show the folder as `C:\lab\2346-Td\backup-1`.
     - Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0099.xml N-0101.xml R-0099.xml R-0101.xml R-CN1.xml R-CN2.xml R-CN3.xml R-CN4.xml B-bills.xml B-payable.xml S-snap-20260401-20260801.xml S-snap-20260401-20270331.xml M-marks.xml`.
   - EXPECT each N and R read 0 rows. A row in any: stop before step 3, with each row recorded whole.
   - EXPECT B-bills and B-payable 0 groups for `BRIDGE Walkin BC`, and each snapshot no row for it. A row or group: stop before step 3. Record each one's groups or rows in all, and its bytes.
   - The report is not sent here: no ledger carries the name yet, and a name no ledger carries is answered with no bytes (row 2377-L).
3. **The rebuild,** each write as above. Each EXPECT is `CREATED` 1, `ERRORS` 0, as PB's:
   - `s3`: `L-ledger.xml`, the bill-wise customer;
   - `s4`: `I-0099.xml`;
   - `s4b`: `I-0101.xml`;
   - `s5`: `I-CN-1.xml` (c1);
   - `s6`: `I-CN-P.xml` (p);
   - `s7`: `I-CN-2.xml` (c2);
   - `s8`: `I-CN-3.xml` (c3).

   The marks labels are `s3m`, `s4m`, `s4bm`, `s5m`, `s6m`, `s7m` and `s8m`. Any answer but `CREATED` 1, `ERRORS` 0: send no further import, record it whole, and go to step 8.
4. **The state.** `s8r`: `R-CN4.xml R-CN1.xml R-CN2.xml R-CN3.xml R-0099.xml R-0101.xml B-bills.xml B-payable.xml M-marks.xml`, then `count20.py` on each.
   - **It is PB's state** if all of these hold, as PB measured after c3:
     - R-CN1 to R-CN4 one row each, Credit Notes `1` to `4`;
     - R-0099 and R-0101 one row each;
     - B-bills one group for `BRIDGE Walkin BC`, `BP/26-27/0101` -525.50;
     - B-payable two, `BP/26-27/0099` and `BP/26-27/0098`, 1051.00 each.
   - If it does not hold, steps 5 and 6 are still sent, since they only read and each compares within this sitting. The report says the state is not PB's.
   - `td-diff.py --map`, each recorded whole:
     - `a-s8r-R-CN4.xml`, `a-s8r-R-CN1.xml`, `a-s8r-R-0099.xml`, `a-s8r-B-bills.xml`, `a-s8r-B-payable.xml` and `a-s8r-M-marks.xml`, each against `pb-answers/` of the same name;
     - `a-s8r-R-CN2.xml` against `pb-answers/a-s6r-R-CN2.xml`;
     - `a-s8r-R-CN3.xml` against `pb-answers/a-s7r-R-CN3.xml`;
     - `a-s8r-R-0101.xml` against `pb-answers/a-s6r-R-0101.xml`.
5. **The control: master's request at 20260801.** BR with labels `c1sa`, `c1sb` (`S-snap-20260401-20260801.xml`), `c1la`, `c1lb` (`L-lo-wk-20260801.xml`).
   - This is the first time master's own report request is sent for this customer. The shot after `c1la` is taken at once; a dialog = Stop.
   - `td-diff.py --map a-c1sa-S-snap-20260401-20260801.xml pb-answers/a-s8sa-S-snap-20260401-20260801.xml`, recorded.
   - `td-diff.py --map a-c1la-L-lo-wk-20260801.xml pb-answers/a-s8la-L-lo-bc-20260801.xml`, recorded.
6. **The case: the same requests at the period's last day, 20270331.** BR with labels `t1sa`, `t1sb` (`S-snap-20260401-20270331.xml`), `t1la`, `t1lb` (`L-lo-wk-20270331.xml`).
   - This is the first time this book is read at a to-date after Tally's current date. The shot after `t1sa`, and after `t1la`, is taken at once; a dialog = Stop.
   - `td-diff.py --same a-t1sa-S-snap-20260401-20270331.xml a-c1sa-S-snap-20260401-20260801.xml`, recorded.
   - `td-diff.py --same a-t1la-L-lo-wk-20270331.xml a-c1la-L-lo-wk-20260801.xml`, recorded.
7. **`m1`:** `M-marks.xml`, then `count20.py --same-row a-s8r-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS: steps 5 and 6 only read. On MARKS DIFFER:
   - send `M-marks.xml` once more as `m1r` (`AGENTS.md` P5, C8) and record it;
   - send nothing else, and STOP.

   The answers already recorded stand.
8. **Restore** BK by C6, with `restore-plan.txt` written first. Row 2346-PB's step 9 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace).
   - Then `s9`: every file of s2, in s2's order, and compare whole:
     - `count20.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`: EXPECT it passes.
     - Each N and R read: 0 rows. B-bills and B-payable: 0 groups for `BRIDGE Walkin BC`. Each snapshot: no row for it.
     - Record each answer's sha256 against s2's; EXPECT each equal.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
9. **Close** by C10. One tally.exe owns 9001.

Steps 5 and 6 never stop the row on an answer that differs from an estimate: each is a result, recorded in full. Only the STOPs named stop it.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering.
- Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- Say in the report what the book then holds: from step 3, the ledger, and each invoice and note that was created.

## Record
- Each answer's bytes and sha256, its headers file whole, and each script's lines, whole.
- `run.py`'s log for each label: each send's start time, file and bytes, and how long each BR send took.
- The `--before-write`, `--same-row`, `--outcome` and `td-diff.py` outputs, whole.
- The window after each import and after each shot named in steps 5 and 6, listed by name, bytes and sha256.
- Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen, and the Gateway's date and period.

**Staging the evidence:** the check for 10 or more digits sets aside this book's registered GUID matched whole, and nothing else. Any other hit waits for the run coordinator.

## Expected
Estimates, not predictions. Debit amounts are negative, as the report and the snapshot sign them.
- **s2:** as the Book and step 2 say.
- **Steps 3 and 4:** as PB's, with the name the only difference. Each import's answer would then be EQUAL to PB's under `--map`, and each read-back EQUAL or differing only where the name changes a length or an order.
- **The control:** rows `BP/26-27/0099` 1051.00, `BP/26-27/0101` -525.50 and `BP/26-27/0098` 1051.00, the snapshot's closing 1576.50, OUTCOME told, and both diffs EQUAL against PB's s8 answers with the name mapped.
- **The case:** the same three rows with the same `BILLCL`, and the same closing, 1576.50: the customer has no voucher but the set-up's and the notes, all dated 1 Aug 2026. `BILLOVERDUE` and the report's other fields are not estimated at a to-date after Tally's current date.
- **s9:** equal to s2 and s1.

## Decides
Each is PARTIAL: one book, one release, one sitting, master's requests sent by curl, without the reads Bridge sends around them.

**Settled from the code before the row** (the render test): `NativeLedgerSnapshotPeriod` admits 20260401 to 20270331 under both profiles. So master's own types do not refuse check 4's to-date on this book.

**The control (step 5):**
- **told, and both diffs EQUAL:** master's own request, through `LedgerScopeName`, reads PB's state. PB's findings on a settled, a part-settled and an over-settled bill then stand for a request master sends.
- **told, with a diff that differs:** record which lines. The case is judged against this sitting's control, and the coordinator decides what the difference means for PB's findings.
- **any other OUTCOME:** record it whole. The case is still sent and recorded, and decides nothing until the control is explained.

**The case (step 6), against the control:**
- **told, the same rows with the same `BILLCL`, and the same closing:** on this book, the read at the period's last day shows the same bills a note is checked against when nothing is dated after it. So the builder can take its to-date as check 4 says, and checks 1 to 3 are judged on the same rows. A later voucher is still not measured.
- **told, with a row added, missing, or with another `BILLCL`:** record which, from the report, Bills Receivable and Bills Payable as read in step 4. Then the to-date changes what checks 1 to 3 see, and the decision waits.
- **told, with the same rows and another closing:** not possible for a tied read; record it as a fault in `pb.py`.
- **`party_bills_untied`, `party_bills_answer_malformed` (a bill or due date outside the window, for example), or another code:** record it whole. Check 4 cannot be built on this read at that to-date as it is, and the coordinator decides whether the read needs a change.
- **`response_content_type_unsupported` or NO BYTES:** row 2377-NB's empty answer, in a state with open bills. That is #2473's case, recorded under the one exception.

**What these answers are for.** They may inform #2346's builder and the bills read it calls. They become fixtures only if the manifest admits them. They are never a regression fixture for a request the build renders, because the build renders no note.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build has no note type.** `VoucherType` (`src-tauri/src/agent_import.rs:330`) has Payment, Receipt, Journal, Contra and Sales, so no build sends a note.
- **The build refuses a bill-wise customer** before any write (`invoice_party_bill_wise_not_supported`, `src-tauri/src/agent_import_invoice.rs:883`). So no build reaches this row's set-up either.
- **The bills read is master's.** The `outstandings` tool's `open_bills` detail (`src-tauri/src/agent_outstandings.rs:13`) admits the party through `LedgerScopeName` (`src-tauri/src/agent_party_bills.rs:78`), then calls `TallyRuntime::fetch_party_bills` (`src-tauri/src/tally/runtime_party_bills.rs:57`). That function:
  - builds the period from the books' first day to the to-date (`:80` to `:82`);
  - reads the snapshot and then the report, each sent twice (`:89`, `:101`);
  - ties them (`PartyBillsAnswer::tie`, `src-tauri/crates/bridge-tally-protocol/src/native_outstandings/party_bills.rs:87`).

  The tool takes its to-date from `as_of`, else the host's date (`agent_party_bills.rs:47` to `:55`). This row sends the same two requests, each twice, in that order, with this customer and the two to-dates, rendered by those builders.

## Not in this row
- ComplyEaze Bridge's own transport, or any Bridge build (above);
- a voucher dated after the notes;
- another to-date, another book or release;
- a customer whose name the type refuses: that is the alphabet's own lab run (`ledger_scope.rs:20` to `:22`).
