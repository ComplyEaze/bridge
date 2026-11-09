# Row 2377-NB: what Tally's Ledger Outstandings report answers for a bill-wise party with no open bills, and which of master's refusals that answer meets

Not for merge. Part of #2377. **Reads only:** nothing is written, so no BK and no restore. Lines are at master ae38a75c.

## Question
`outstandings` with a party and `detail` `open_bills` runs `fetch_party_bills` (`tally/runtime_party_bills.rs`). It reads master's `List of Ledgers` snapshot from the books' first day to the to-date (:85), then the party's `Ledger Outstandings` report (:97), each as a paired read: two sends that must be byte-equal, else `native_ledger_collection_changed` or `party_bills_drifted` (:89, :101). Then, in this order:
0. the snapshot parses whole, else its error (:113): one bad row anywhere refuses it;
1. the party is found in the snapshot, else `party_not_in_book` (:116);
2. its `ISBILLWISEON` is Yes (or True or 1), else `party_not_billwise` (:118);
3. the answer parses, else `party_bills_tally_failure` or `party_bills_answer_malformed`;
4. `PartyBillsAnswer::tie` (`native_outstandings/party_bills.rs`):
   - an empty `CLOSINGBALANCE` refuses as `party_ledger_closing_not_read` (:91);
   - else the sum of `BILLCL` must equal the closing, or `party_bills_untied` (:98);
   - only then does an answer with no row refuse as `party_bills_no_rows_not_established` (:104-107).

Before any of that, the transport refuses an answer without exactly one `Content-Type` header as `response_content_type_unsupported` (`bridge-tally-transport/src/lib.rs:1288-1295`), and one whose value is not `text/xml` with a UTF-16 or no charset (`text_encoding.rs:451`). A name no ledger carries was answered that way on another book: HTTP 200, `CONTENT-LENGTH: 0`, no `Content-Type`, no bytes, twice (`tests/fixtures/agent/party-bills/PROVENANCE.md`).

The runtime also reads the book's extent before (:78) and after (:109), and sends the snapshot and the report each twice (a paired read); this row sends each report twice and each snapshot once, and takes the books' first day from the extent fixture. Those other reads are not this row's question.

What a bill-wise party with no open bills gets is not measured: the last refusal says so in its own comment. **The question:** on a bill-wise party with no bill and no entry, does the report answer no bytes, an envelope with no `BILLFIXED`, or something else? And is the snapshot's closing for it empty, `0.00` or another figure? `pb.py --outcome` names the refusal or read master gives for the pair.

**The control:** the same party at a date after its bills (20260804), where it holds one bill. It changes only the side from the receivable answers already captured (a payable party, on another book). The case (20260731) then changes only the bills (`AGENTS.md` P6). The control's tie also gives a first reading of a payable party.

All four requests are master's own builders, rendered by a lab-only test and sent raw (shape discovery, allowed on lab books). The design note this row follows was read by the run coordinator before the sheet.

## Book
`BRIDGE PILOT LAB` (synthetic), GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, TallyPrime 7.2 Silver, loaded alone. THE COMPANY in `LAB-COMMON.md` means this book.
- **Synthetic and registered:** master's `scripts/merge-gate-synthetic-uuids.json` names its GUID. It is not the register lab book (`REGISTER_LAB_GUID`, `captured_parse_record.rs:302`).
- **Pinned by GUID.** `pb.py`'s `BOOK` is this GUID. m0 must print one COMPANY row, `the book's: yes`, else STOP before any other send. Every snapshot row must carry it as `BRIDGECOMPANYGUID`, else STOP.
- **Its state at rest:** ALTVCHID 68 and ALTMSTID 245. The rows that wrote to it from 13 Oct, `lab/2162-s1-answers` to `lab/2162-sc-answers`, each restored it to those marks (`RESULTS.md` each); later rows are not checked here, and m0 checks it. Its books begin 20260401 (`tests/fixtures/agent/pilot-lab-register/pilot-lab-register-book-extent.utf16le.xml`).
- **The party:** `BRIDGE Supplier RJ`, Sundry Creditors, opening 0.00, bill-wise Yes, the book's one bill-wise ledger in every PILOT ledger read in master's fixtures. Its bills (`audit-read-pilot-lab`, `pilot-lab-audit-vouchers`, 15 Oct): Purchase 1 makes New Ref `SUP/0001` for 1050.00 and Purchase 2 is an Agst Ref to it for 1050.00, both dated 20260804. Its closing after them is 2100.00 (`pilot-lab-register-ledgers-paired`). So from 20260401 to 20260803 it has no bill and nothing in its ledger.
- **Do not change Tally's current date or period.**

## Kit
Check every file against `SHA256.txt` first, and each send's request file again just before it is sent (Get-FileHash, C3). Any file that differs = STOP, and nothing is sent.

| file | what, and where its bytes come from |
|---|---|
| `M-marks.xml` | The marks request for this book (GUID, ALTVCHID, ALTMSTID). Byte-equal to row 2162-S9a's (`lab/2162-s9a-sheet` e558c9f8). |
| `S-snap-20260401-20260804.xml` | `render_native_ledger_snapshot_request` (`native_outstandings/request.rs:169`), 20260401 to 20260804, the period `fetch_party_bills` builds (`runtime_party_bills.rs:81`, `ModeAgnostic` on a licensed gateway). |
| `L-lo-rj-20260804.xml` | `render_ledger_outstandings_request` (`native_outstandings/party_bills.rs:39`) for `BRIDGE Supplier RJ` to 20260804. |
| `S-snap-20260401-20260731.xml` | The snapshot builder for 20260401 to 20260731. Only `SVTODATE` differs from the 0804 one. |
| `L-lo-rj-20260731.xml` | The report builder to 20260731. Only `SVTODATE` differs from the 0804 one. |
| `snapshot-build-request-forex-20250401-20250430.bin` | Not sent. The snapshot request the build sent on another lab book on 29 Oct (`lab/2071-forex-rerun` `requests/ledger-snapshot.utf16le.xml`). The render test checks the snapshot builder's render against it first. |
| `run.py` | Row 683-LM's, with one change: curl's `-D` keeps each answer's response headers in `a-<label>-<file>.headers` (C3). Its stops are unchanged. |
| `LAB-COMMON.md`, `.gitattributes` | Row 683-LM's, byte-equal (v3). |
| `pb.py` | This row's counter for marks, snapshot and report answers. Its header says what each mode prints, and every STOP. |

**How the request bytes were made: a lab-only render test, not a template edit.** `src-tauri/crates/bridge-tally-protocol/tests/lab_2377_nb_render.rs` on this branch, one `#[ignore]` test, run once in a build slot:
`LAB_2377_NB_OUT=<this folder> cargo test -p bridge-tally-protocol --test lab_2377_nb_render -- --ignored --exact lab_2377_nb_renders_the_row_requests`.
- It calls the two builders with the book's name, the party and the dates, and writes each request as the build sends it, UTF-16LE with a BOM.
- Before writing, it renders the snapshot builder for the 29 Oct request and checks it byte for byte. That shows the encoding and the builder equal to what the build sends.
- The test is on this branch only and is never merged.

**`pb.py` follows master's report grammar** (`parse_ledger_outstandings_answer`, with the date windows of `native_outstandings/date.rs`: a bill dated after the to-date is malformed), the transport's header checks, the paired read and the runtime's order of refusals. Its limits:
- it reads only the party's row of a snapshot (and every row's GUID), where master refuses the whole snapshot on any row it cannot read;
- it does not check the snapshot's own pair or headers (the snapshot is sent once);
- it finds a `STATUS` by a text search, where master stops at the first XML error;
- it stops on a character XML forbids in a report answer, where master's reading of one is not checked;
- a `Content-Encoding` header, or a closing or bill-wise flag in a form it does not read, prints `NOT READ BY THIS SCRIPT`. Master's own parsers are run on the answers when they become fixtures, and that run is the one that counts.

**`pb.py` was tried:**
- **On master's captures:** `Lab Party P1` at 31 July 2026 with its snapshot (`party-bills/`, with `BOOK`, `PARTY` and `BOOKS_FROM` set to that book's in a scratch copy): ROWS 5 (1 On Account), sum -37000.00, OUTCOME told. The unknown-name answer (0 bytes) with its headers: NO BYTES, OUTCOME `response_content_type_unsupported`. PILOT's `pilot-lab-register-ledgers-paired`: 18 rows, `BRIDGE Supplier RJ` bill-wise Yes, closing 2100.00.
- **On answers made for the purpose**, against that PILOT snapshot or a copy:
  - one bill `SUP/0001` of 2100.00: told; against a `0.00` closing: `party_bills_untied`; the same bill at a to-date of 20260731: `party_bills_answer_malformed` (`bill_date_invalid`), as also for a month `Foo`, 31 February and a four-digit bill year; a due date of `1-Dec-2108`: told; two `BILLOP`: told;
  - the two answers not byte-equal: `party_bills_drifted`; a charset `utf-8`: `declared_encoding_mismatch`; `text/html`: `response_content_type_unsupported`; a `Content-Encoding`: NOT READ BY THIS SCRIPT;
  - an envelope with no row: `party_bills_untied` against 2100.00; `party_ledger_closing_not_read` against an empty closing; `party_bills_no_rows_not_established` against `0.00`;
  - the party renamed in the snapshot: `party_not_in_book`; its bill-wise No: `party_not_billwise`; `TRUE`: told; `maybe`: NOT READ BY THIS SCRIPT;
  - a STATUS: `party_bills_tally_failure`; a self-closed element, a scalar before any `BILLFIXED`, an unknown element, a `BILLCL` twice or empty, an unclosed envelope: `party_bills_answer_malformed`, each naming its rule;
  - 0 bytes, or no file written, with a `Content-Type`: `party_bills_answer_malformed` (an empty text is an unclosed envelope);
  - an answer without its headers file: STOP.

## Screens and prompts
This row opens no master, voucher or report screen. Only the Gateway and Select Company are used.
- **Load:** Select Company, the book typed into its name filter, the one row, Enter after a screenshot shows the highlighted row reads `BRIDGE PILOT LAB`.
- **A company loaded at the start:** C1 says STOP. If the run coordinator's word is to shut it, shut it by Company > Shut, with no change to it.
- **Close (C10):** the `K: Company` label, Shut, Enter on the book.
- **Bring Tally to the front only by its running window.** A second `tally.exe` is a STOP.

**The prompt rule.** No dialog is expected in this row. Any dialog, message box or prompt, at any step: press no key, screenshot the whole window, copy its text exactly, send nothing while it is up, and ask the run coordinator (C7).

**The path rule.** If a list or line shown is not exactly what a path above names (other words, no such line, a Spelling Error), stop, Esc out of the list, send nothing, and ask. Nothing has been written, so there is nothing to restore.

## The wedge rule
A read that Tally is still building keeps the gateway busy for every company, and giving up on it does not stop Tally's work. So:
- One request at a time, by `run.py`. It never abandons a request: curl waits for its answer.
- This book is small (68 vouchers' worth of marks, 18 ledgers). Its ledger snapshot on record is 25,518 B; the report answers on record are under 6,000 B. If any send has not answered **5 minutes** after it began:
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).

  Nothing is re-sent on this row's own judgement.
- `run.py`'s stops are failed requests, never "nothing found". Each stops the row there, with one exception, below.
- **The one exception: an empty answer to an `L-` request.** It is a possible answer to this row's question, so it is recorded as what it is first: `run.py` printed "empty answer", the request failed. This row narrows C3 for `L-` requests only, by the run coordinator's order for this row ("record no bytes as a failed request first, then classify it, never as 0 rows"): an empty body is not a STOP, and its pair's second send is allowed, only if its headers file shows HTTP status 200 and `CONTENT-LENGTH: 0`. Such an answer was received, so it is not C3's request "that ended without one". `run.py`'s printed "stop, do not resend" is then read as this exception. Any other empty body (no headers file, no status line, another status or length) ended without an answer: STOP, never sent again. The rest of C3 holds. Then, in this order:
  1. keep the headers file and the log line, and screenshot the whole Tally window;
  2. if a dialog is up: STOP;
  3. send `GET /status` once (C2c's command), recording its status and bytes; if it is not 200: STOP;
  4. run `pb.py` on that answer and go on with the step, which sends the same request once more.

  An empty answer to an `M-` or `S-` request is a STOP, as `run.py` says. Curl exit other than 0, Unknown Request and an answer above 2,000,000 B are STOPs for every request.

## Steps
Run each send as `python3 -I run.py http://127.0.0.1:9001 <label> <file>`. Write down each send's start time from its log. **Labels are used once each.**
0. **C2's step 0** with no company loaded:
   - the screenshot first;
   - one `tally.exe` owning 9001;
   - `GET /status` 200 (record its bytes).

   Load the book alone, as above. Read the Gateway's date and period, and write them down. Record the release and the licence mode.
1. **`m0`:** `M-marks.xml`, then `python3 -I pb.py a-m0-M-marks.xml`. EXPECT one COMPANY row, `the book's: yes`, ALTVCHID 68, ALTMSTID 245.
   - Any other GUID or row count: STOP.
   - Other figures: record them and STOP before step 2, since the book changed after its last note.
2. **The control, at 20260804:**
   - `s1`: `S-snap-20260401-20260804.xml`, then `python3 -I pb.py a-s1-S-snap-20260401-20260804.xml`. Record the party's row whole.
   - `l1a`: `L-lo-rj-20260804.xml`. Screenshot the whole Tally window (the first time this report is sent on this book; a dialog = Stop), then `python3 -I pb.py a-l1a-L-lo-rj-20260804.xml`.
   - `l1b`: `L-lo-rj-20260804.xml` once more, then `pb.py` on it. Record whether `l1a` and `l1b` are byte-equal (their sha256), as master's paired read requires.
   - `python3 -I pb.py --outcome a-l1a-L-lo-rj-20260804.xml a-l1b-L-lo-rj-20260804.xml a-s1-S-snap-20260401-20260804.xml`, recorded whole, its OUTCOME line above all.
3. **The case, at 20260731:**
   - `s2`: `S-snap-20260401-20260731.xml`, then `pb.py` on it. Record the party's row whole, its `CLOSINGBALANCE` text above all: empty, `0.00`, or other.
   - `l2a`: `L-lo-rj-20260731.xml`. At once, before anything else, screenshot the whole Tally window: this is the first time this report is asked for a party with no bill. A dialog = Stop. Then `pb.py` on it (after the exception's steps, if it was empty).
   - `l2b`: `L-lo-rj-20260731.xml` once more, then `pb.py` on it. Record whether `l2a` and `l2b` are byte-equal.
   - `python3 -I pb.py --outcome a-l2a-L-lo-rj-20260731.xml a-l2b-L-lo-rj-20260731.xml a-s2-S-snap-20260401-20260731.xml`, recorded whole, its OUTCOME line above all.
4. **`m1`:** `M-marks.xml`, then `python3 -I pb.py --same-row a-m0-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS. On MARKS DIFFER:
   - send `M-marks.xml` once more as `m1r` (`AGENTS.md` P5, C8) and record it;
   - send nothing else, and STOP.

   The answers already recorded stand, and the report says the marks moved.
5. **Close** by C10, with `GET /status` after the last request (C11). One `tally.exe` owns 9001.

Steps 2 and 3 never stop the row on an answer that differs from an estimate: each is a result, recorded in full. Only the STOPs named stop it.

## Stop
Press no key, screenshot the whole window, and report before anything else is pressed or sent, if:
- any dialog or prompt appears, or a second `tally.exe` starts;
- the gateway stops answering, or a send passes the wedge rule's 5 minutes;
- `run.py` stops, outside the one exception, or `pb.py` stops;
- m0 is not one row, the book's, V 68, M 245;
- m1 differs from m0.

## Record
- Each answer's bytes and sha256, its headers file whole (`RESPSTATUS`, `CONTENT-LENGTH` and `Content-Type` among them), and each request's sha256 (as the Kit lists; any other = the Stop of C3).
- The whole output of each `pb.py` call.
- Each send's start time and how long it took, from `log-<label>.txt`, and every `GET /status` with its status and bytes.
- The marks at m0 and m1; the release, the licence mode, Tally's date and period, and the PC's date.
- The screenshots after `l1a` and `l2a`, by name, bytes and sha256 (screenshots are never pushed).

**Staging the evidence:** the check for 10 or more digits sets aside this book's registered GUID matched whole, and nothing else. Any other hit waits for the run coordinator.

## Expected (estimates, not EXPECTs)
- **The control:** the snapshot gives the party's closing as `2100.00`, and the report one row, `SUP/0001` dated 3 or 4 August 2026 (the vouchers carry a reference date of 20260803), with `BILLCL` `2100.00`, so OUTCOME told. A payable party's sign in either read has not been captured: a negated or differing pair would be `party_bills_untied`, recorded as the control's result.
- **The case:** not estimated. On both books in the lab's earlier answers, a ledger with a zero opening and no entry has an empty `CLOSINGBALANCE` in the snapshot (`Cash` in every BILLTRAIL snapshot; `BRIDGE Output CGST XML` in PILOT's `pilot-lab-register-ledgers-paired`). If that holds here, the outcome is `party_ledger_closing_not_read`, unless the answer is refused before the tie (its headers, the pair, or its ANSWER line). The report's own answer is still the row's first finding.

## Decides
For the build, by the case's OUTCOME and the `ANSWER` lines of `l2a` and `l2b`, each PARTIAL (one book, one release), and each made in its own pull request. The first three need `l2a` and `l2b` byte-equal:
- **An envelope with no row, and a closing `0.00`** (OUTCOME `party_bills_no_rows_not_established`): the no-rows refusal can become an empty answer told as "no open bills", with these answers as fixtures.
- **An envelope with no row, and an empty closing** (OUTCOME `party_ledger_closing_not_read`): the build needs a rule for an empty closing on a bill-wise party with no row, and the outstandings read's treatment of an empty closing as zero (`compute.rs:435-440`) needs a measured basis. A settled bill, whose closing may be a written zero, is then the next route to the no-rows refusal that this design names. That needs a voucher behind a backup, and is a later row of its own.
- **No bytes, twice** (both ANSWER lines NO BYTES, and OUTCOME `response_content_type_unsupported`): the report answers a party with no bills as it answers an unknown name. The no-rows refusal stays; the next step says the snapshot found the party and its bill-wise flag, which only the snapshot can tell.
- **No bytes once and an envelope once,** or `l2a` and `l2b` not byte-equal: master refuses that pair. The OUTCOME is `response_content_type_unsupported` when the empty answer has no `Content-Type` (as the unknown name's did), else `party_bills_drifted`. Record both; the decision waits.
- **Rows in the case:** the party holds a bill before 20260804, against the captured vouchers. Record it; the decision waits until that is explained.
- **Any other OUTCOME in the case** (`party_bills_tally_failure`, `party_bills_answer_malformed` on an envelope or on an empty text that carries a `Content-Type`, `party_bills_untied`, `party_not_in_book`, `party_not_billwise`, another transport code, or NOT READ BY THIS SCRIPT): record it whole; the decision waits.
- **The control not told:** the case's tie decides nothing about a payable party's sign. Its ANSWER lines still stand as the report's answer for a party with no bill.

Whichever way it falls, the answers are parsed again by master's own parsers when they become fixtures, before any code relies on them.

## Not in this row
- any write, including the settled-bill case;
- the `outstandings` tool itself, or any Bridge build;
- a receivable party with no bills;
- a party with bill-wise off, or a foreign-currency party;
- another book or release.
