# Row 2346-PB: what does one customer's bills read show for an open bill, a settled one, a part-settled one, an over-settled one and a bill a note made?

Shape discovery, as row 2346-AR: the notes are requests written for the lab, not requests the build renders, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2346 (0.9.0), the row the capture-6 comment of 10 Nov 2026 asks for. Written 10 Nov 2026. Row 2346-AR's set-up and its three notes are sent again, byte for byte, with one second invoice and one part note added. After the set-up and after each note, this row reads the customer's bills the way the build reads them. Then it restores from its own backup. Nothing is keyed in Tally but the backup and the restore. Confidence of everything here until the row runs: unverified.

## Question
Row 2346-AR (`lab/2346-ar-answers` at 11d6ad2b) measured what Tally stores for three Credit Notes with an `Agst Ref`:
- **c1**, against the customer's open bill: stored as sent, and the bill settled;
- **c2**, the same note against that settled bill: stored as sent, with no error;
- **c3**, naming a bill the customer does not hold: stored with a `New Ref` under that name.

AR read Bills Receivable only, which lists debit bills. So it did not show what the customer then held. #2346's builder must refuse an `Agst Ref` before the post unless the bill is open and the amount fits. It reads the customer's bills through the `outstandings` tool's one-party read on master: the ledger snapshot, then the party's `Ledger Outstandings` report, tied (`TallyRuntime::fetch_party_bills`, `src-tauri/src/tally/runtime_party_bills.rs:57`).

**The question, for each state the notes leave:**
- What does the one-party read return: which rows, each with what `BILLCL`?
- Does the sum of `BILLCL` equal the snapshot's closing, so that master tells the read?
- Where does each bill show in Bills Receivable and Bills Payable?

The states:

| After | The one variable | What is not known |
| --- | --- | --- |
| **the set-up** | two open invoice bills, `BP/26-27/0099` and `BP/26-27/0101`, 1,051.00 each | (a control: two debit rows, as on row 2377-L's book) |
| **c1** | `BP/26-27/0099` settled in full | a zero row for it, or no row |
| **p, the part note** | `BP/26-27/0101` settled for half, 525.50 | its row with the rest, -525.50, or something else |
| **c2** | `BP/26-27/0099` over-settled by 1,051.00 | a credit row for it, a row elsewhere, or none; and whether the read still ties |
| **c3** | a credit `New Ref` `BP/26-27/0098` | a credit row for it, and whether the read still ties |

- **Why a second invoice.** The part note needs a bill that is still open. c1, c2 and c3 must stay AR's requests byte for byte, against the bill AR sent them to, so they cannot share it. A part note on `BP/26-27/0099` before c1 would turn c1 into an over-settlement. So the set-up adds `BP/26-27/0101`: AR's set-up invoice with only its number and REMOTEID tail changed (asserted). It is a set-up, not a variable. The part note is c1's request against `BP/26-27/0101`, with every amount halved (asserted).
- **Why one sitting:** each note's question is the state the notes before it left.
- **Not asked here:** a Debit Note; a note against a bill from an earlier year or an opening bill; a part note whose rest is then settled; two part notes; a to-date at the period's end; a book with many bills (the read's cost).

**The read is sent the way ComplyEaze Bridge sends it, with curl as the transport only.** On master ae38a75c, `fetch_party_bills` sends the ledger snapshot twice and requires the two answers to be byte-equal (`:89`). It then sends the party's `Ledger Outstandings` report twice under the same rule (`:101`). Each BR below sends those four requests in that order, after the step's write.
- **The bytes are master's builders'.** `S-snap-20260401-20260801.xml` is `render_native_ledger_snapshot_request`'s format string, and `L-lo-bc-20260801.xml` is `render_ledger_outstandings_request`'s, each filled with this row's values. `B-bills.xml` and `B-payable.xml` are `render_native_bills_request`'s, one for each `NativeBillsReportKind`. Each was checked against master's format strings at ae38a75c before this sheet was sent.
- **curl carries the bytes and keeps each answer's headers.** So the answers stand as Bridge-shaped evidence of what Tally returns to this read.
- **What is not Bridge's:**
  - The reads Bridge sends around the pairs are not sent: the read boundary, the company identity bracket before and after, and the book extent before and after. The step's M marks read stands in for part of the identity bracket (one COMPANY row, the book's GUID, its marks). Nothing stands in for the extent.
  - `pb.py --outcome` names the code master would give, from its own reading of master's code. Master's parsers, run on the answers if they become fixtures, are the run that counts.
- **An empty answer to the report** is row 2377-NB's case, which is #2473's domain.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (v3, byte-equal to rows 2346-AR's and 2377-NB's) applies in full. Where it and this sheet differ, the stricter one holds.

- **The book is pinned by GUID.** `count20.py` and `pb.py` each hold BRIDGE PILOT LAB's GUID as `BOOK`.
  - Each M answer prints whether the COMPANY row's GUID is the book's.
  - `--before-write` and `--same-row` STOP on any other GUID, whatever the company is called.
  - STOP before anything is sent unless s1's COMPANY row is the book's.
- **Its state:** every row queued before this one ends with a restore, which leaves the marks at ALTVCHID 68 and ALTMSTID 245. Other marks at s1: stop before BK, with nothing sent.
- **It holds no `BRIDGE Walk-in BC`, no Credit Note and no bill-wise customer.** Row 2346-AR made `BRIDGE Walk-in BC` and its notes, and restored them away: its s8 read every one of them as absent, byte-equal to its s2. Step 2's reads stay a STOP, in case that is wrong.
- **The numbers:**
  - AR's numbers, `BP/26-27/0099` and `BP/26-27/0098`, and its tails `000000000d10` to `000000000d13`, are reused as AR sent them. The book holds none of them after AR's restore.
  - Every blob on every `lab/` branch (663 refs, 23,803 blobs, read as UTF-16 and as UTF-8) and on master was searched. None holds `26-27/0100` to `26-27/0103` or the tails `000000000d14` to `000000000d19`. This row takes `BP/26-27/0101` and the tails `000000000d14` and `000000000d15`.
- **The Credit Note numbers:** Credit Note numbers automatically on this book. The four notes are expected to take `1` (c1), `2` (p), `3` (c2) and `4` (c3), in the order sent. If a note takes another number, the R-CN read for it finds nothing. Record the number from Tally's screen (Display, the day book of 1 Aug 2026) and say so. That is not a finding about the bills.
- **The date:** read the Gateway's current date at step 1. If it is before 1-Aug-2026, stop: the vouchers would be post-dated, which is row 2162-S1's variable. Rows 2346-AR and 2377-NB read Tuesday 15-Sep-2026.
- **The to-date of every bills read is 20260801,** the vouchers' date, as AR's Bills Receivable read used. The snapshot reads from the books' first day, 20260401 (as `pb.py`'s `BOOKS_FROM` and row 2377-NB's snapshot read).
- **BK is this row's way back.**
  - It is taken at step 1, before anything is sent, and restored at step 9 by C6.
  - The restore runs on every path from step 3 on that is not a STOP. After a STOP (C7, C8 or C9), C8 holds, and the restore waits for the run coordinator's word.
  - After it, every read is compared whole with the reads before the change. Any part of that compare that is not equal stops the row.
- Write down the release shown on Tally's screen. Rows 2346-AR and 2377-NB read TallyPrime with `tally.exe` 1.1.7.2, from the program's file.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6). The writes are `L-ledger.xml`, `I-0099.xml`, `I-0101.xml`, `I-CN-1.xml`, `I-CN-P.xml`, `I-CN-2.xml` and `I-CN-3.xml`, each sent at most once.

## The sends and the wedge rule
`run.py` is row 2377-NB's, which also keeps each answer's response headers, by curl -D, in `a-<label>-<file>.headers`.
- One request at a time. `run.py` never abandons a request: curl waits for its answer.
- **The wedge rule.** If any send has not answered **5 minutes** after it began:
  - press nothing, and do not end curl;
  - send nothing more, not even `GET /status`;
  - screenshot the whole Tally window, and report to the run coordinator (C9).

  Nothing is re-sent on this row's own judgement. On this book the snapshot answered 25,518 B and the report under 6,000 B. A read-back that finds a voucher answers about 95,834 B.
- **The stops.** `run.py`'s stops are failed requests, never "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 B. So are `count20.py`'s and `pb.py`'s STOPs. Each stops the row there, with one exception.
- **The one exception: an empty answer to an `L-` request.** That case is #2473's domain. The exception is row 2377-NB's, unchanged, and it needs the run coordinator's order for this row, as NB had one. Row 2377-NB found a bill-wise party with no bills answered with no bytes. Here the customer holds a bill at every read, but a settled bill may not count.
  - It is recorded first as what it is: `run.py` printed "empty answer", so the request failed.
  - It is not a STOP, and its pair's second send is allowed, only if its headers file shows HTTP status 200 and `CONTENT-LENGTH: 0`. `run.py`'s printed "stop, do not resend" is then read as this exception.
  - Any other empty body (no headers file, no status line, another status or another length) ended without an answer. That is a STOP, and the request is never sent again. The rest of C3 holds.
  - Then, in this order:
    1. keep the headers file and the log line, and screenshot the whole Tally window;
    2. if a dialog is up: STOP;
    3. send `GET /status` once (C2c's command), recording its status and bytes; if it is not 200: STOP;
    4. run `pb.py` on that answer, and go on with the step.
  - An empty answer to any other request is a STOP, as `run.py` says.
- Check each file's sha256 against `SHA256.txt` before its send.
- Count every answer with the script its kind names below, and copy its whole output into RESULTS.md.
- Close no Bridge process, and call no Bridge tool. That is the lab's standing rule, as in rows 2346-AR and 2377-NB.

**Which script reads which answer**
- **`count20.py`** (row 2346-AR's, unchanged) reads `a-*-M-marks.xml`, `a-*-N-*`, `a-*-R-*`, `a-*-B-*` (Bills Receivable and Bills Payable alike, by the flat grammar of reference §12a.1), and the import answers `a-*-I-*` and `a-*-L-ledger.xml`. Its B kind prints the customer's groups (`--ledger` defaults to `BRIDGE Walk-in BC`).
- **`pb.py`** (row 2377-NB's with two changes, marked PB) reads `a-*-S-snap-*` and `a-*-L-lo-*`. For each step it also prints `pb.py --outcome SA SB LA LB`: the code master gives for the snapshot sent twice and then the report sent twice, in master's order.
- They share no answer file. Give each script only its own files.

**A read of the bills** (named BR below) is five labels, in this order. The first four are `fetch_party_bills`'s sends, in its order:
1. `<step>sa`: `S-snap-20260401-20260801.xml`;
2. `<step>sb`: `S-snap-20260401-20260801.xml`, the same request again;
3. `<step>la`: `L-lo-bc-20260801.xml`;
4. `<step>lb`: `L-lo-bc-20260801.xml`, the same request again;
5. `<step>r`: `B-bills.xml B-payable.xml M-marks.xml`, plus the step's N or R reads.

If the two snapshot answers are not byte-equal, master sends nothing more. This row still sends 3 to 5, and records that master would not have.

Then:
- `count20.py` on the `r` label's M, N, R and B answers;
- `pb.py` on both S answers and both L answers;
- `pb.py --outcome a-<step>sa-S-snap-20260401-20260801.xml a-<step>sb-S-snap-20260401-20260801.xml a-<step>la-L-lo-bc-20260801.xml a-<step>lb-L-lo-bc-20260801.xml`.

Record the window after `<step>la` (shot): the first read of this report for this party in each state.

## Files
**Copied from earlier rows**
- **Byte-equal to row 2346-AR's file of the same name** (`lab/2346-ar-sheet` at fd0aef60, folder `lab/2346-ar/`):
  - `.gitattributes`, `LAB-COMMON.md`, `M-marks.xml` and `count20.py`;
  - the set-up: `L-ledger.xml` and `I-0099.xml`;
  - the notes: `I-CN-1.xml` (c1), `I-CN-2.xml` (c2) and `I-CN-3.xml` (c3);
  - the reads: `N-0099.xml`, `R-0099.xml`, `R-CN1.xml`, `R-CN2.xml`, `R-CN3.xml` and `B-bills.xml` (the outstandings tool's Bills Receivable read, `render_native_bills_request`, from 1 Apr 2026 to 1 Aug 2026);
  - the templates `gen.py` starts from: `template-import-0010.utf16le.xml`, `template-n-0001.utf16le.xml`, `R-0010.xml` and `I-CN.xml`.
- **Byte-equal to row 2377-NB's file** (`lab/2377-nb-sheet` at 03d7763a, folder `lab/2377-nb/`):
  - `run.py`;
  - `template-nb-L-lo-rj-20260804.xml` and `template-nb-S-snap-20260401-20260804.xml`, NB's `L-lo-rj-20260804.xml` and `S-snap-20260401-20260804.xml`, renamed.
- **`pb.py`:** row 2377-NB's, with each changed line marked PB:
  - `PARTY` is `BRIDGE Walk-in BC`;
  - `--outcome` takes the snapshot's two answers before the report's two, as master sends them (`runtime_party_bills.rs:89`, then `:101`). It checks the snapshot answers' headers as it checks the report's. A snapshot pair that is not byte-equal gives `native_ledger_collection_changed` (`src-tauri/src/tally/connection.rs:1270`).

**`gen.py`** first asserts that row 2346-AR's own functions, with AR's values, give each AR file above byte for byte. It then writes each new request with only the change it names, each change asserted to occur exactly as often as expected:
- **`I-0101.xml` (set-up):** `I-0099.xml` with the number `BP/26-27/0101` (twice, and as its New Ref's name) and the tail `000000000d14`.
- **`I-CN-P.xml` (p):** `I-CN-1.xml` with:
  - the tail `000000000d15`;
  - `REFERENCE` `BP/26-27/0101`;
  - the Agst Ref named `BP/26-27/0101`;
  - every `AMOUNT` halved: the party leg and its Agst Ref 525.50; the two income legs -300.20 and -200.20; each tax leg -12.51; the round off -0.08.
  - The halves are exact, and the six legs still sum to zero. The party leg is still the sum of the others, and each tax leg is still 2.5% of the income legs' 500.40. Each is asserted.
  - Turned back, it is `I-CN-1.xml` byte for byte (asserted).
- **`N-0101.xml`, `R-0101.xml`:** the number read and the build's read-back for `BP/26-27/0101`.
- **`R-CN4.xml`:** `R-CN1.xml` with the number `4`.
- **`L-lo-bc-20260801.xml`:** NB's report request with the party `BRIDGE Walk-in BC` and the to-date `20260801`.
- **`S-snap-20260401-20260801.xml`:** NB's snapshot request with the to-date `20260801`.
- **`B-payable.xml`:** `B-bills.xml` with the report `Bills Payable`, the same builder's other kind (`NativeBillsReportKind::Payable`).

**Tried before this sheet was sent**, each exit 0:
- `pb.py`, on row 2377-NB's answers:
  - m0, and `--same-row` m0 m1: SAME MARKS;
  - s1, where `BRIDGE Walk-in BC` has no row;
  - l1a, one row;
  - l2a, NO BYTES;
  - `--outcome` s1 s1' l1a l1b, where s1' is s1's answer and headers file copied: `party_not_in_book`, as the party is not in NB's book. The script before the change gives the same for l1a l1b s1;
  - `--outcome` s1 s2 l1a l1b: `native_ledger_collection_changed`;
  - `--outcome` with the old order, l1a l1b s1: STOP, exit 1, as meant.
- `pb.py` on master's capture `ledger-outstandings-p1-20260331.utf16le.xml` (with l1a's headers file beside it): ROWS 6 (1 On Account). The two credit bills read with a positive `BILLCL`, and the sum is -26000.00.
- `count20.py`'s B kind on master's capture `native-outstandings-lab-bills-payable.utf16le.xml`: 3 groups, 0 for `BRIDGE Walk-in BC`.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

## Steps
Run each label as `python3 -I run.py <gateway URL> <label> <files>`. The prompt rule, the wait rule and the import answers are row 2346-AR's.

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named.
- For any other prompt, message or dialog: copy it word for word and screenshot it first.
- Then answer No, or Esc if it offers no No. Press nothing more and send nothing.
- Then restore BK by C6, compare as step 9, and close (C10).
- That prompt is this row's result.

**Each import's answer** is one of these:
- `CREATED` 1 with `ERRORS` 0;
- `CREATED` 0 with `ERRORS` 1 and a LINEERROR;
- `CREATED` 0 with `EXCEPTIONS` 1.

Any other answer, or a dialog after it: the prompt rule. Never send an import file twice (C3).

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down. Before 1-Aug-2026: stop.
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2346-pb\backup-1`. Row 2346-AR's step 1 is the path.
     - Tally may show the folder as `C:\lab\2346-Pb\backup-1`.
     - Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0099.xml N-0101.xml R-0099.xml R-0101.xml R-CN1.xml R-CN2.xml R-CN3.xml R-CN4.xml B-bills.xml B-payable.xml S-snap-20260401-20260801.xml M-marks.xml`.
   - EXPECT each N and R read 0 rows. A row in any: stop before step 3, with each row recorded whole.
   - EXPECT B-bills and B-payable 0 groups for `BRIDGE Walk-in BC`, and the snapshot no row for it. Record each one's groups or rows in all, and its bytes.
   - The report is not sent here: no ledger carries the name yet, and a name no ledger carries is answered with no bytes (row 2377-L).
3. **The set-up.**
   - `s3m`: `M-marks.xml`, then `count20.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. It must pass.
   - `s3`: `L-ledger.xml`, sent once. EXPECT `CREATED` 1, `ERRORS` 0.
   - `s4`: `I-0099.xml`, sent once. EXPECT `CREATED` 1, `ERRORS` 0.
   - `s4b`: `I-0101.xml`, sent once. EXPECT `CREATED` 1, `ERRORS` 0.
   - Any other answer to any of the three: send nothing more, and go to step 9.
   - **BR at s4**, with `N-0099.xml N-0101.xml R-0099.xml R-0101.xml` in `s4r`.
   - **The set-up holds** only if all of these do, as row 2346-AR's s4r read for the first invoice:
     - each N one row;
     - each R one row whose `BRIDGE Walk-in BC` leg holds one bill with `NAME` its own number, `New Ref`, `-1051.00`;
     - B-bills two groups for `BRIDGE Walk-in BC`: `BP/26-27/0099` and `BP/26-27/0101`, each `1-Aug-26`, `BILLCL` `-1051.00`.
   - If the set-up does not hold: no note is sent. Record the answers whole and go to step 9.
   - The report's answer and the outcome are recorded, whatever they are. They do not decide whether the row goes on.
4. **c1: row 2346-AR's note against the open bill `BP/26-27/0099`.**
   - `s5m`: `M-marks.xml`, then `--before-write` against s1. It must pass.
   - `s5`: `I-CN-1.xml`, sent once. Record every counter and any LINEERROR word for word. Shot the window after it.
   - **BR at s5**, with `R-CN1.xml R-0099.xml R-0101.xml` in `s5r`.
   - EXPECT, as AR's c1: R-CN1 one row, Credit Note `1`, its `BRIDGE Walk-in BC` leg one bill `BP/26-27/0099`, `Agst Ref`, `1051.00`.
   - `count20.py --diff a-s4r-R-0099.xml a-s5r-R-0099.xml` and the same for R-0101: record whether either invoice moved.
5. **p: the part note against `BP/26-27/0101`.**
   - `s6m`: `M-marks.xml`, then `--before-write` against s1.
   - `s6`: `I-CN-P.xml`, sent once. Record as step 4.
   - **BR at s6**, with `R-CN2.xml R-CN1.xml R-0101.xml` in `s6r`.
   - **p's answer:** R-CN2's row and every bill on its `BRIDGE Walk-in BC` leg. If p took a number other than `2`, the Book's note numbers bullet applies.
   - `--diff` of R-CN1 and of R-0101 against their s5r answers.
   - If p was not created, c2 and c3 may take `2` and `3`. Then also send `R-CN2.xml` in `s7r` and `R-CN3.xml` in `s8r`, and record which one holds each.
6. **c2: row 2346-AR's note again, against the settled `BP/26-27/0099`.**
   - `s7m`: `M-marks.xml`, then `--before-write` against s1.
   - `s7`: `I-CN-2.xml`, sent once. Record as step 4.
   - **BR at s7**, with `R-CN3.xml R-CN1.xml R-0099.xml` in `s7r`.
   - EXPECT, as AR's c2: R-CN3 one row whose leg holds `BP/26-27/0099`, `Agst Ref`, `1051.00`.
   - `--diff` of R-CN1 and of R-0099 against their last earlier answers.
7. **c3: row 2346-AR's note naming `BP/26-27/0098`.**
   - `s8m`: `M-marks.xml`, then `--before-write` against s1.
   - `s8`: `I-CN-3.xml`, sent once. Record as step 4.
   - **BR at s8**, with `R-CN4.xml R-CN1.xml R-0099.xml` in `s8r`.
   - EXPECT, as AR's c3: R-CN4 one row whose leg holds `BP/26-27/0098`, `New Ref`, `1051.00`.
   - `--diff` of R-CN1 and of R-0099 against their last earlier answers.
8. *(Reserved: nothing between c3 and the restore.)*
9. **Restore** BK by C6, with `restore-plan.txt` written first. Row 2346-AR's step 8 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace).
   - Then `s9`: every file of s2, in s2's order, and compare whole:
     - `count20.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml`: EXPECT it passes.
     - Each N and R read: 0 rows. B-bills and B-payable: 0 groups for `BRIDGE Walk-in BC`. The snapshot: no row for it.
     - Record each answer's sha256 against s2's; EXPECT each equal.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The book is then not as it was, and the coordinator decides.
10. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering.
- Screenshot the window, send the screenshot and the answers to the run coordinator, and ask before anything else is pressed or sent.
- Say in the report what the book then holds: from step 3 the ledger and the invoices, and from each note whatever it did.

## Record
- Each answer's bytes and sha256, its headers file whole, and each script's lines.
- `run.py`'s log for each label: each send's start time, file and bytes.
- The `--diff`, `--before-write`, `--same-row` and `--outcome` outputs, whole.
- The window after each import and after each `<step>la` (shot), listed by name, bytes and sha256.
- Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen and the Gateway's date and period.

## Expected
Estimates, not predictions. Debit amounts are negative, as the report and the snapshot sign them.
- **s2:** as the Book and step 2 say.
- **s4:** the report's rows `BP/26-27/0099` and `BP/26-27/0101`, -1051.00 each. The snapshot's closing -2102.00. OUTCOME told.
- **s5:** `BP/26-27/0101` -1051.00, and for `BP/26-27/0099` a zero row or none. The closing -1051.00.
- **s6:** `BP/26-27/0101` -525.50. The closing -525.50.
- **s7:** `BP/26-27/0099` as a credit row, 1051.00, in the report and in Bills Payable (reference §12a.1 and §12a.14: bills scope by the sign of their balance, on other books). The closing 525.50.
- **s8:** a further credit row `BP/26-27/0098`, 1051.00. The closing 1576.50.
- **s9:** equal to s2 and s1.

## Decides
Each is PARTIAL: one book, one release, one sitting, the read sent in master's bytes and order by curl, without the reads around it.

**For each state, the outcome `pb.py --outcome` names:**
- **told:** master's one-party read reads this state. Its rows are what the note builder's refusals are judged on (the capture-6 comment's checks 1 to 3).
- **`party_bills_untied`:** the report leaves out or misstates a bill that the closing holds. Record which bill, from Bills Receivable, Bills Payable and the read-backs. Master refuses every customer in that state, which is safe but stops notes to them. The coordinator decides whether the read needs a change.
- **`native_ledger_collection_changed`:** the snapshot answered the same request differently, with nothing sent between. Record both answers whole. Nothing is sent again on this row's judgement.
- **`response_content_type_unsupported` or NO BYTES:** row 2377-NB's empty answer, in a state with an open bill. #2473 is open for that case.
- **Any other code:** record it whole.

**For each kind of bill:**
- **s5, the settled bill:**
  - **no row:** a settled bill is absent from the read. The refusal of a bill not in the read (check 1) also refuses it.
  - **a zero row:** check 2 (open as a debit) refuses it.
  - Either way, the builder refuses c2 before the post.
- **s6, the part note:** the rest shown as the bill's `BILLCL` is what check 3 (the amount fits) is measured against. **Any other figure** decides that check 3 cannot be built on this read as it is.
- **s7 and s8, the credit bills:**
  - **credit rows that tie:** master tells customers holding a credit bill, and check 2 refuses an `Agst Ref` to one.
  - **otherwise:** as `party_bills_untied`, above.

**What these answers are for.** They may inform #2346's builder and the bills read it calls. They become fixtures only if the manifest admits them. They are never a regression fixture for a request the build renders, because the build renders no note.

## Code admits the trigger
Lines are at master ae38a75c.
- **The build has no note type.** `VoucherType` (`src-tauri/src/agent_import.rs:330`) has Payment, Receipt, Journal, Contra and Sales, so no build sends a note.
- **The build refuses a bill-wise customer** before any write (`invoice_party_bill_wise_not_supported`, `src-tauri/src/agent_import_invoice.rs:883`). So no build reaches this row's set-up either.
- **The bills read exists.** The `outstandings` tool's `open_bills` detail (`src-tauri/src/agent_outstandings.rs:13`) calls `TallyRuntime::fetch_party_bills` (`src-tauri/src/tally/runtime_party_bills.rs:57`). That reads the snapshot and then the report, each sent twice (`:89`, `:101`), and ties them (`PartyBillsAnswer::tie`, `src-tauri/crates/bridge-tally-protocol/src/native_outstandings/party_bills.rs:87`). This row sends the same two requests, each twice, in that order, with its own party and to-date.

## Not measured
- The reads ComplyEaze Bridge sends around the pairs: the read boundary, the identity bracket and the book extent.
- Bridge's own transport and parsers on these answers. An empty report answer is #2473's domain.
- A to-date at the period's end, or after a later-dated settlement.
- A Debit Note, a purchase-side note, and a note against an opening bill or one from an earlier year.
- A second part note, or the rest of a part-settled bill settled.
- The read's cost on a book with many bills.
- Any other book or release.

Row: lab/2346-pb/
