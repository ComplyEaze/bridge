# Row 2480-NM: does Tally's `Ledger Outstandings` report read a ledger whose name holds a mark that master's party type refuses?

Shape discovery, as rows 2346-AR, 2346-PB and 2346-TD: the requests are written for the lab, not requests the build sends, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2480 (0.9.0), "What would settle it", item 1. Written 10 Nov 2026. Lines are at master 56f7be77. Confidence of everything here until the row runs: unverified.

## Question
`outstandings` with the `open_bills` detail refuses a party whose name holds anything but ASCII letters, ASCII digits and single spaces, or a formula word, as `party_name_not_admitted` (#2480).
- **The refusal is master's, before any send.** `LedgerScopeName::new` (`src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65`) admits that alphabet (`:74`) and refuses the words of `FORMULA_WORDS` (`:30`). `admitted_party` (`src-tauri/src/agent_party_bills.rs:26`) maps every refusal to the one code. `render_ledger_outstandings_request` (`src-tauri/crates/bridge-tally-protocol/src/native_outstandings/party_bills.rs:39`) takes only that type.
- **The alphabet was set for another read.** The module comment (`ledger_scope.rs:15` to `:22`) says it was set for a `CHILDOF` formula. In the `Ledger Outstandings` report the name travels as the text of the `LedgerName` static variable. Whether that read needs the same rule is not established (#2480).
- **Row 2346-PB already sent one such name,** `BRIDGE Walk-in BC`, with a hyphen, by this request: its report answer (`a-s8la`, `lab/2346-pb-answers` at bc52b5ce) holds the customer's bills. So the hyphen here is a repeat in master's bytes. Every other name is new.

**The questions,** for each of the ten names master refuses, one name a ledger:
1. **Does Tally answer the report for that name with the ledger's own bill?** Master's own request, with only the name changed, sent twice.
2. **Does that answer agree with the other two reads of the same book?** The ledger snapshot's closing for the ledger (the tie master makes), and the book-wide Bills Receivable group for the same ledger.
3. **If it does not, what does it do instead?** An empty answer (row 2377-L saw a name matching no ledger answered with HTTP 200 and no bytes), another ledger's bill, or another code. Another ledger's bill is the finding that would keep a mark out for good.

**The one variable** in each ledger's answer is the name's mark. The ledgers are alike but for the name: the same parent group, the same bill-wise setting, one invoice of the same amount on the same day, a bill named by the invoice's number. A control ledger, whose name the type admits, is read the same way.

**Not asked here:** see the last section.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2346-PB's, 2346-TD's and 2377-NB's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count20.py` and `nm.py` each hold BRIDGE PILOT LAB's GUID as `BOOK`.
  - Each M answer prints whether the COMPANY row's GUID is the book's.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is sent unless s1's COMPANY row is the book's.
- **Its state:** every row queued before this one ends with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245. Rows 2346-TD and 2162-RS ran on this book on 10 Nov 2026 and each ended with its restore, as the coordinator reports. Other marks at s1: stop before BK, with nothing sent.
- **It holds none of the eleven ledgers and none of the numbers below.** Step 2 checks this, and a row there is a STOP.
  - The ledgers are the eleven names of `names.tsv`, all beginning `BRIDGE Nm `.
  - The invoice numbers are `BP/26-27/0201` to `BP/26-27/0211`.
  - The `REMOTEID` tails are `000000000e01` to `000000000e0b`. No file of rows 2346-PB, 2346-TD or 2162-RS uses one: their tails are `777`, `795`, `d01` and `d10` to `d15`.
- **No exception to C3 is needed.** Each of this row's 22 writes is a new file, and none carries a key that an earlier row sent. Each is sent at most once, and never again whatever happened to its first send.
- **The date and the period:** read the Gateway's current date and period at step 1.
  - If the date is before 1-Aug-2026, stop: the invoices are dated 1 Aug 2026.
  - If the period is not 1-Apr-26 to 31-Mar-27, stop.
  - **Do not change Tally's current date or period.**
- **BK is this row's way back.**
  - It is taken at step 1, before anything is sent, and restored at step 7 by C6.
  - The restore runs on every path from step 3 on that is not a STOP. After a STOP (C7, C8 or C9), C8 holds, and the restore waits for the run coordinator's word.
  - After it, every read of step 2 is sent again and compared whole. Any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Row 2346-PB read TallyPrime with `tally.exe` 1.1.7.2, from the program's file.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6).

**The eleven ledgers** (`names.tsv`; `id`, invoice number, name). Each is created by `LM-<id>.xml` and given its one open bill by `I-<id>.xml`, both sent once.

| id | number | name | what differs from the control |
| --- | --- | --- | --- |
| `pl` | 0201 | `BRIDGE Nm Plain` | the control: letters and single spaces, which the type admits |
| `hy` | 0202 | `BRIDGE Nm Ax-Bx` | a hyphen |
| `dt` | 0203 | `BRIDGE Nm Ax.Bx` | a dot |
| `am` | 0204 | `BRIDGE Nm Ax&Bx` | an ampersand (the request carries `&amp;`) |
| `sl` | 0205 | `BRIDGE Nm Ax/Bx` | a slash |
| `lp` | 0206 | `BRIDGE Nm Ax(Bx` | an opening bracket alone |
| `rp` | 0207 | `BRIDGE Nm Ax)Bx` | a closing bracket alone |
| `ap` | 0208 | `BRIDGE Nm Ax'Bx` | an apostrophe (the request carries `&apos;`) |
| `op` | 0209 | `BRIDGE Nm Ax and Bx` | an operator word (`and`), letters and spaces only |
| `dd` | 0210 | `BRIDGE Nm 2024-25` | digits with a dash |
| `sc` | 0211 | `BRIDGE Nm नमूना` | another script (Devanagari) |

The names are invented. The marks sit between two letters, one mark a name, so that no name holds a second mark or a mark at an edge.

## The sends and the wedge rule
`run.py` is row 2377-NB's (as PB's), which also keeps each answer's response headers, by curl -D, in `a-<label>-<file>.headers`.
- One request at a time. `run.py` never abandons a request: curl waits for its answer.
- **The wedge rule.** If any send has not answered **5 minutes** after it began:
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).

  Nothing is re-sent on this row's own judgement. On this book, PB's snapshot answered 26,724 B and its report 3,720 B. This row's snapshot holds eleven more ledgers.
- **The stops.** `run.py`'s stops are failed requests, never "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 B. So are `count20.py`'s and `nm.py`'s STOPs. Each stops the row there, with one exception.
- **The one exception: an empty answer to an `L-lo-` request.**
  - **Why it is this row's own case.** A name that matches no ledger is answered with no bytes (row 2377-L), so a name Tally does not read as the ledger's may be answered that way. That is a result of this row, and it needs the run coordinator's order when the row is queued.
  - It is recorded first as what it is: `run.py` printed "empty answer", so the request failed.
  - It is not a STOP, and its pair's second send is allowed, only if its headers file shows HTTP status 200 and `CONTENT-LENGTH: 0`. `run.py`'s printed "stop, do not resend" is then read as this exception.
  - Any other empty body (no headers file, no status line, another status or another length) ended without an answer. That is a STOP, and the request is never sent again. The rest of C3 holds.
  - Then, in this order:
    1. keep the headers file and the log line, and screenshot the whole Tally window;
    2. if a dialog is up: STOP;
    3. send `GET /status` once (C2c's command), recording its status and bytes; if it is not 200: STOP;
    4. run `nm.py` on that answer, and go on with the step.
  - An empty answer to any other request, an `LM-` or `I-` import included, is a STOP, as `run.py` says.
- Check each file's sha256 against `SHA256.txt` before its send.
- Close no Bridge process, and call no Bridge tool. That is the lab's standing rule, as on PB.

**Which script reads which answer**
- **`count20.py`** (PB's, byte-equal) reads `a-*-M-marks.xml`, `a-*-N-*`, and the import answers `a-*-LM-*` and `a-*-I-*`. It is read by its first letter after the label, so give it no `L-lo-` answer. Its `--before-write` and `--same-row` are the write gate and the marks compare.
- **`nm.py`** (PB's `pb.py` with the party replaced by `names.tsv`, a Bills Receivable kind and two compare modes added) reads `a-*-M-marks.xml`, `a-*-S-snap-*`, `a-*-B-bills.xml` and `a-*-L-lo-<id>-*`. Its modes, whole in its header:
  - `nm.py <answers>`: what each answer holds. For S, a row or "no row" for each of the eleven names; for B, the groups of each name and the count of any other party's; for L, the `ANSWER:` line.
  - `nm.py --outcome <id> SA SB LA LB`: what master's later steps would make of the answers for that ledger, as one `OUTCOME:` line. Master would not send these names, so this is the answer's verdict, not a path master follows.
  - `nm.py --against <id> LA B S`: that ledger's L rows against its group in B and its closing in S, `AGAINST: EQUAL` or `DIFFER`.
  - `nm.py --same A B`: sizes and sha256, `SAME: EQUAL` or `DIFFER`. It records and never decides.
- `count20.py` and `nm.py` share only the M answers. Give each script only its own files.

**A ledger read** (named LR below) is the report's two sends for one ledger, `fetch_party_bills`'s second pair (`runtime_party_bills.rs:101`): `r<n>a` and `r<n>b`, the same file twice. The snapshot's pair is sent once for all eleven, in step 4, before any report: it is the same request for every ledger.

If the two report answers of a ledger are not byte-equal, master would send nothing more for it. This row still records both.

## Files
**Byte-equal to row 2346-PB's file of the same name** (`lab/2346-pb-sheet` at 53540665, folder `lab/2346-pb/`):
- `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count20.py` and `M-marks.xml`;
- the reads: `B-bills.xml` (the Bills Receivable report to 20260801) and `S-snap-20260401-20260801.xml` (master's snapshot builder's request);
- the two templates, `template-import-0010.utf16le.xml` (the request the build sent for BP/26-27/0010) and `template-n-0001.utf16le.xml` (the number read);
- in `pb/`, PB's `L-ledger.xml`, `I-0099.xml`, `N-0099.xml` and `L-lo-bc-20260801.xml`. They are never sent here: `gen-nm.py` and the render test start from them.

**Byte-equal to row 2377-NB's file** (`lab/2377-nb-sheet`): `snapshot-build-request-forex-20250401-20250430.bin`. It is never sent: it is the request the build sent on another lab book on 29 Oct, which the render test checks the snapshot builder against first.

**Written for this row:**
- `names.tsv`: the eleven ids, numbers and names, UTF-8, read by `gen-nm.py`, `nm.py` and the render test;
- `nm.py`, as above.

**Made by `gen-nm.py`** (33 files) from the files above, each template checked against PB's `SHA256.txt` first:
- `LM-<id>.xml`: PB's `L-ledger.xml` with its customer name (twice: the `NAME` attribute and element) turned into the name, escaped as master's `escape_text` escapes it, and nothing else;
- `I-<id>.xml`: the invoice of PB's `I-0099.xml`, built by PB's function (its `gen.py`) from the template, with the name (7 times), the number (3 times) and the `REMOTEID` tail turned, and nothing else; its bill is `New Ref`, -1051.00, named by the number;
- `N-<number>.xml`: the number read with the number turned.

  For each, it asserts that the old text occurs as often as expected, that the file parses and its fields hold the name and number, that the legs balance, and that the invoice turned back is PB's `I-0099.xml` byte for byte. It first asserts that the same functions, with PB's values, give PB's `L-ledger.xml`, `I-0099.xml` and `N-0099.xml` byte for byte.

**Made by master's builder's template, by a render test:** the eleven `L-lo-<id>-20260801.xml`.
- **How they were made.** By a lab-only test, row 2377-NB's path: `src-tauri/crates/bridge-tally-protocol/tests/lab_2480_nm_render.rs` on this branch, one `#[ignore]` test, run once in a build slot with a target directory of its own, `LAB_2480_NM_OUT=<this folder> cargo test -p bridge-tally-protocol --test lab_2480_nm_render -- --ignored --exact lab_2480_nm_renders_the_row_requests --nocapture`. The test is on this branch only and is never merged.
- **Why not the builder.** `render_ledger_outstandings_request` takes a `LedgerScopeName`, and the type refuses ten of the eleven names. So the test builds the request from a copy of the builder's template with the party escaped by the same `escape_text`, a `&str` in place of the type. The builder and the type are not changed.
- **What it asserted first,** each passing:
  - the source file of the builder holds the copied template once, character for character;
  - the snapshot builder for the 29 Oct request: byte-equal to `snapshot-build-request-forex-20250401-20250430.bin`;
  - the snapshot builder at 20260801: byte-equal to PB's `S-snap-20260401-20260801.xml`;
  - `LedgerScopeName::new("BRIDGE Walk-in BC")` is `Err(Character)`, and the copy's request for that name is byte-equal to PB's `pb/L-lo-bc-20260801.xml`, the file PB sent to Tally;
  - for the control, the copy's request is byte-equal to master's own `render_ledger_outstandings_request` for the admitted name.
- **What it checked for each name:** the type's verdict, which is as the table says: the control admitted, `op` refused as a formula word (`ledger_scope_name_formula_word`), every other refused as a character (`ledger_scope_name_character`); the escaped name occurs once; and the request is the control's with the name turned, and nothing else. It printed each id with the verdict and the file's size: 820 B for the control and for the hyphen, dot, slash, bracket and script names, 824 B for the digits name, 828 B for the ampersand and the operator word, 830 B for the apostrophe.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

**Tried before this sheet was sent**, each as stated:
- `gen-nm.py`: its 33 files, each check passing.
- The render test: passed once, as above.
- `nm.py`, on answers made for the test from row 2346-PB's real s8 answers (kept in the TD sheet's `pb-answers/`) with the customer turned into this row's names, one of them with an ampersand in the snapshot and the bills, and headers files written for them (the test inputs are not fixtures and are not kept):
  - S: a row for the two names present, "no row" for nine; the ampersand name read as `&`;
  - B: the groups of the two names;
  - L: `ANSWER: ROWS 3` for PB's three-row report, and `ANSWER: NO BYTES` for an empty answer with a headers file of `CONTENT-LENGTH: 0`;
  - `--outcome`: `told` (sum 1576.50 equals closing 1576.50) for the control, and `party_bills_untied` where the rows and the closing were made to differ;
  - `--against`: `EQUAL` for a one-row answer made to match its group and closing, `DIFFER` for the empty answer;
  - `--same`: `EQUAL` for a file with itself, `DIFFER` for two files.
- **Not tried:** any send to Tally. Whether Tally accepts each name for a ledger is not known; an import that refuses one is a result (step 3).

## Steps
Run each label as `python3 -I run.py <gateway URL> <label> <files>`. The prompt rule, the wait rule and the path rule are PB's and TD's.

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named.
- For any other prompt, message or dialog: copy it word for word and screenshot it first.
- Then answer No, or Esc if it offers no No. Press nothing more and send nothing.
- Then restore BK by C6, compare as step 7, and close (C10).
- That prompt is this row's result.

**Each import's answer** is one of these:
- `CREATED` 1 with `ERRORS` 0;
- `CREATED` 0 with `ERRORS` 1 and a LINEERROR;
- `CREATED` 0 with `EXCEPTIONS` 1.

Any other answer, or a dialog after it: the prompt rule. Never send an import file twice (C3). What each answer means for the step is in step 3.

**Each write, the same way:**
- `run.py URL <label>m M-marks.xml`, then `python3 -I count20.py --before-write a-s1-M-marks.xml a-<label>m-M-marks.xml`.
- Only if that passes, and no key was pressed in Tally since, send the import alone: `run.py URL <label> <file>`.
- Then screenshot the whole window before the next request (C3).
- Then `count20.py` on the answer, recorded whole.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down (the Book's date-and-period bullet).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2480-nm\backup-1`. Row 2346-PB's step 1 is the path.
     - Tally may show the folder as `C:\lab\2480-Nm\backup-1`.
     - Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0201.xml N-0202.xml N-0203.xml N-0204.xml N-0205.xml N-0206.xml N-0207.xml N-0208.xml N-0209.xml N-0210.xml N-0211.xml B-bills.xml S-snap-20260401-20260801.xml M-marks.xml`.
   - EXPECT each N read 0 rows (`count20.py`). A row in any: stop before step 3, with each row recorded whole.
   - EXPECT `nm.py` on B-bills: no group for any of the eleven names; and on the snapshot: "no row" for each. A row or group: stop before step 3. Record the groups of any other party and the snapshot's row count, and each answer's bytes.
3. **The set-up,** one ledger at a time in the order of the table, each id's ledger and then its invoice, each write as above. The labels are `s3<id>` for `LM-<id>.xml` and `s4<id>` for `I-<id>.xml`, with `s3<id>m` and `s4<id>m` for the marks reads before them. Each EXPECT is `CREATED` 1, `ERRORS` 0.
   - **A ledger's answer that is not that,** with no dialog: it is that name's result. Record it whole (the LINEERROR or the exception text, word for word), send that id's invoice never, and go on with the next id. This is the only case in which the row goes on after an import that is not as expected. A dialog after it: the prompt rule.
   - **The control `pl`:** if its ledger or its invoice is not as expected, the baseline is lost: send no further import, and go to step 7.
   - **An invoice's answer that is not `CREATED` 1, `ERRORS` 0,** for any id: send no further import, record it whole, and go to step 7.
   - A ledger whose invoice was not sent has no bill; step 5 sends nothing for it, and the report records it as not created.
4. **The book-wide reads and the state.** `s5m`: `M-marks.xml` (the marks after the last write). Then:
   - `s5sa`, `s5sb`: `S-snap-20260401-20260801.xml`, the same file twice;
   - `s5ba`, `s5bb`: `B-bills.xml`, the same file twice.

   Neither request is a new shape (PB's files, byte for byte), so C3's screenshot rule for a new shape does not apply here; a dialog on the screen at any time is still the prompt rule.
   - `nm.py` on `a-s5sa-S-snap-20260401-20260801.xml` and `a-s5ba-B-bills.xml`, each recorded whole.
   - `nm.py --same a-s5sa-S-snap-20260401-20260801.xml a-s5sb-S-snap-20260401-20260801.xml` and `nm.py --same a-s5ba-B-bills.xml a-s5bb-B-bills.xml`, each recorded.
   - **It is the intended state** if: S holds a row for each created ledger, `ISBILLWISEON` Yes, `CLOSINGBALANCE` -1051.00; B holds one group for each, `BP/26-27/<its number>`, -1051.00. If not, step 5 is still sent for the ledgers that exist, since it only reads, and the report says the state is not the intended one.
   - If the two snapshot answers are not byte-equal, master would send nothing more for any ledger; this row still sends step 5, and records that master would not have.
5. **The ledger reads.** For each id in the order of the table that has a ledger and a bill, LR with the labels below. The request is `L-lo-<id>-20260801.xml`.

   | n | id | labels | n | id | labels |
   | --- | --- | --- | --- | --- | --- |
   | 1 | `pl` | `r1a`, `r1b` | 7 | `rp` | `r7a`, `r7b` |
   | 2 | `hy` | `r2a`, `r2b` | 8 | `ap` | `r8a`, `r8b` |
   | 3 | `dt` | `r3a`, `r3b` | 9 | `op` | `r9a`, `r9b` |
   | 4 | `am` | `r4a`, `r4b` | 10 | `dd` | `r10a`, `r10b` |
   | 5 | `sl` | `r5a`, `r5b` | 11 | `sc` | `r11a`, `r11b` |
   | 6 | `lp` | `r6a`, `r6b` | | | |

   - **A shot after each first send** (`r<n>a`): screenshot the whole Tally window at once, before the next request. Each of these requests is a file the lab has not been sent, and each carries a name no request of the lab has carried. A dialog = Stop.
   - Then, for each id, recorded whole:
     - `nm.py a-r<n>a-L-lo-<id>-20260801.xml`;
     - `nm.py --same a-r<n>a-L-lo-<id>-20260801.xml a-r<n>b-L-lo-<id>-20260801.xml`;
     - `nm.py --outcome <id> a-s5sa-S-snap-20260401-20260801.xml a-s5sb-S-snap-20260401-20260801.xml a-r<n>a-L-lo-<id>-20260801.xml a-r<n>b-L-lo-<id>-20260801.xml`;
     - `nm.py --against <id> a-r<n>a-L-lo-<id>-20260801.xml a-s5ba-B-bills.xml a-s5sa-S-snap-20260401-20260801.xml`.
   - A ledger's answer never stops the row: each is a result, recorded in full. Only the STOPs named stop it.
6. **`m1`:** `M-marks.xml`, then `count20.py --same-row a-s5m-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS: steps 4 and 5 only read. On MARKS DIFFER:
   - send `M-marks.xml` once more as `m1r` (`AGENTS.md` P5, C8) and record it;
   - send nothing else, and STOP.

   The answers already recorded stand.
7. **Restore** BK by C6, with `restore-plan.txt` written first. Row 2346-PB's step 9 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace).
   - Then `s9`: every file of s2, in s2's order, and compare whole:
     - `count20.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`: EXPECT it passes.
     - Each N read: 0 rows. `nm.py` on B-bills: no group of the eleven names. On the snapshot: "no row" for each.
     - Record each answer's sha256 against s2's; EXPECT each equal.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
8. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering.
- Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- Say in the report what the book then holds: from step 3, each ledger and each invoice that was created.

## Record
- Each answer's bytes and sha256, its headers file whole, and each script's lines, whole.
- `run.py`'s log for each label: each send's start time, file and bytes, and how long each ledger read took.
- The `--before-write`, `--same-row`, `--outcome`, `--against` and `--same` outputs, whole.
- The window after each import and after each shot named in step 5, listed by name, bytes and sha256.
- Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen, and the Gateway's date and period.
- For each ledger: whether Tally created it, whether the invoice made its bill, and the three reads of it (the report pair, the snapshot row, the Bills Receivable group).

**Staging the evidence:** the check for 10 or more digits sets aside this book's registered GUID matched whole, and nothing else. Any other hit waits for the run coordinator.

## Expected
Estimates, not predictions. Debit amounts are negative, as the report and the snapshot sign them.
- **s2:** as the Book and step 2 say.
- **Step 3:** each answer `CREATED` 1, `ERRORS` 0. No estimate for the ten marks: whether Tally accepts each as a ledger's name is part of the row.
- **Step 4:** S one row each, -1051.00, Yes; B one group each, `BP/26-27/<number>`, -1051.00, `BILLOVERDUE` not estimated.
- **The control `pl`:** `ANSWER: ROWS 1`, the bill `BP/26-27/0201`, -1051.00; `OUTCOME: told`; `AGAINST: EQUAL`.
- **The hyphen `hy`:** as the control, since PB's report for a hyphen name was answered with its bills.
- **The other nine:** no estimate. The row is built because none of them is known.
- **s9:** equal to s2 and s1.

## Decides
Each is PARTIAL: one book, one release, one sitting, master's request template sent by curl, without the reads Bridge sends around it, and without the build's own parse (that run is the one that counts when an answer becomes a fixture).

**For each of the ten names,** one of these. The control's answer is the baseline: if it is not `told` with `AGAINST: EQUAL`, no other name is judged until it is explained.
- **`told`, `AGAINST: EQUAL`, the one bill the ledger's own, the two sends equal:** on this book and release, Tally reads that name, in the `LedgerName` static variable, as the ledger, and the report agrees with the snapshot and the book-wide read. This is the evidence #2480 asks for, for that one mark. It does not say the formula read (`CHILDOF`) takes the mark.
- **NO BYTES (HTTP 200, length 0):** Tally matched no ledger for that name, as it does for an absent name (row 2377-L). That mark is not read by this request as it is sent; the build keeps refusing it. The row cannot tell whether the mark or its escape is the cause.
- **Rows that are another ledger's, or a bill that is not this ledger's own,** with or without a tie: the most important result. Record it whole. That mark is not admitted, whatever its other answers show, and the coordinator decides what it means for the alphabet.
- **`party_bills_untied`, `party_bills_answer_malformed`, `party_bills_tally_failure` or another code:** record it whole. Master's parse would refuse that read for that name, so widening the alphabet would not make it work as it is.
- **The two sends not equal (`SAME: DIFFER`):** record both; master would refuse as `party_bills_drifted`.
- **The ledger's import refused (step 3):** Tally does not take that name as a ledger's on this release; the mark is out for both reads, and no report was sent for it.
- **The operator word `op` and the digits `dd`:** each reads like any other name; a refused answer there says the filter is wider than the marks.

**What these answers are for.** They may inform #2480's change: widening the alphabet for both reads, or giving the static-variable read a type of its own with the measured alphabet, and the marks its change list names. They become fixtures only if the manifest admits them. A fixture of one of them pins the request only once the build renders it, which today it does not.

## Code admits the trigger
Lines are at master 56f7be77.
- **The build refuses these names before any send.** `LedgerScopeName::new` (`ledger_scope.rs:65`) refuses any character outside ASCII letters, digits and a single space (`:74`) and the words of `FORMULA_WORDS` (`:30`). `admitted_party` (`src-tauri/src/agent_party_bills.rs:26`) maps each refusal to `party_name_not_admitted`. So no build reaches this row's reads, and this row sends by curl.
- **The bills read is master's.** `fetch_party_bills` (`src-tauri/src/tally/runtime_party_bills.rs:57`) reads the snapshot and then the report, each sent twice (`:89`, `:101`), and ties them. This row sends the same snapshot and the same report request, with the name the only difference, which the type would not take.
- **The rendering is master's template.** `render_ledger_outstandings_request` (`party_bills.rs:39`) takes the party as the text of the `LedgerName` static variable, escaped once as XML text. The render test checks the copy against it.
- **`ledger_movement` falls back for the same names** (`src-tauri/src/agent_movement.rs:105`), so it still answers there, more slowly. This row does not read vouchers.

## Not in this row
- ComplyEaze Bridge's own transport, or any Bridge build;
- the `CHILDOF` formula read, `ledger_movement`'s ledger-scoped voucher read, whose alphabet was set for it (`ledger_scope.rs:15` to `:22`): "the formula read stays as it is unless the same row covers it" (#2480);
- a mark not in the table: a quote, a comma, an underscore, a colon, an angle bracket, an asterisk, a percent sign, an at sign, a hash, a dollar sign, an exclamation mark, a plus, an equals sign, a semicolon, a question mark, a backslash, a bar, a tilde, a caret, a square or curly bracket, a control character or a line break, another script than Devanagari;
- a name with two or more marks or a mark beside a space or at an edge, such as a name with a trailing dot, an ampersand between spaces, a balanced pair of brackets or a leading prefix with a slash; each is its own row, once the single marks are known;
- another operator word than `and` (`FORMULA_WORDS` holds fourteen more), a name of the longest admitted length, and a name that differs from another ledger's only by case;
- a ledger with more than one bill, a settled or part-settled bill, a note or a voucher after the invoice (rows 2346-PB and 2346-TD);
- another to-date, book or release; a book with GST on; Educational mode.
