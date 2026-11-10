# Row 2480-NS: does Tally's `Ledger Outstandings` report read a ledger whose name holds one mark, when no two names of the row share a stem?

Shape discovery, as rows 2346-PB and 2480-NM: the requests are written for the lab, not requests the build sends, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2480 (0.9.0), "What would settle it", item 1. Written 11 Nov 2026. Lines are at master 5bf79364. Confidence of everything here until the row runs: unverified.

## Why a second row
Row 2480-NM (`lab/2480-nm-sheet` at 73cb01ae, answers `lab/2480-nm-answers` at 12a84c9a; its #2480 comment) did not answer the question. It sent no `Ledger Outstandings` read.
- **One stem for every mark.** Seven of its names were `BRIDGE Nm Ax<mark>Bx`. The dot, ampersand, slash and bracket names were each answered `CREATED` 0, `ALTERED` 1, and the hyphen ledger's own name was rewritten. With letters and digits alone, all seven names are the same. `st.py --before` on NM's own state read `c8s`, with NM's names, finds the dot, ampersand, slash and opening-bracket names on the hyphen ledger's key, and the closing-bracket name in that ledger's name list. So the mark was not the only variable.
- **The stop was passed.** NM's sheet listed three import answers and sent any other to the prompt rule. Its `step3.sh` tested only `CREATED` 1, `ERRORS` 0 and `EXCEPTIONS` 0, so the first `ALTERED` answer (`s3dt`) did not stop it, and twelve imports went out after it (NM's results, deviation 1).

**What this row changes, and nothing else:**
1. **A stem of its own for each name.** No two names of this row share a key, and no name shares a key with a ledger the book holds. `gen-ns.py` asserts the first (and that no name shares a key with one of NM's). `st.py --before` checks the second on the book's own snapshot before any write, and STOPs on any hit.
2. **Reads first.** Each name's report is sent once before its ledger exists (step 3). A name Tally reads as no ledger is answered with no bytes (row 2377-L). A name answered with another ledger's bills is the finding #2480 most needs, and it is measured here with nothing written.
3. **The write gate is a program, and it is exact.** After every import, `st.py --import` on the answer. The next request is sent only on its exit 0, `IMPORT: CREATED ONE`, which means one `RESPONSE`, `CREATED` 1, `ALTERED`, `DELETED`, `COMBINED`, `IGNORED`, `ERRORS`, `CANCELLED` and `EXCEPTIONS` each 0, and no LINEERROR. Any other exit is the STOP of step 4, at the request that made it. A refusal (`ERRORS` 1) stops the row too: this row has no "go on with the next id".

The requests are NM's, with only the name, the invoice number and the `REMOTEID` tail changed (Files).

## Question
For each of the ten names master's party type refuses, one name a ledger:
1. **Before the ledger exists, what does the report answer for that name?** No bytes is expected. Any bill is another ledger's.
2. **Once it exists with one bill, does the report answer with that ledger's own bill?** Master's own request, with only the name changed, sent twice.
3. **Does that answer agree with the other two reads of the book?** The ledger snapshot's closing for the ledger (the tie master makes), and the book-wide Bills Receivable group for it.

**The one variable** in each ledger's answers is its name's mark, in names of one pattern (letters, the mark, letters) except `op`, `dd` and `sc`, whose shape is their point. The ledgers are alike but for the name: the same parent group, the same bill-wise setting, one invoice of the same amount on the same day, a bill named by the invoice's number. The control ledger, whose name the type admits, is read the same way.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (byte-equal to NM's) applies in full. Where it and this sheet differ, the stricter one holds.

- **Pinned by GUID.** `count20.py`, `nm.py` and `st.py` each hold BRIDGE PILOT LAB's GUID as `BOOK`, and each STOPs on a row with another.
- **Its state:** ALTVCHID 68 and ALTMSTID 245, the marks NM's restore left (its `s9`, equal to its `s1`), if every row since has ended with its restore, as the coordinator reports. Other marks at s1: stop before BK, with nothing sent.
- **It holds none of the eleven ledgers and none of the numbers below.** Step 2 checks this, and a row there is a STOP.
  - The ledgers are the eleven names of `names.tsv`, all beginning `BRIDGE Ns `.
  - The invoice numbers are `BP/26-27/0221` to `BP/26-27/0231` (NM's were 0201 to 0211).
  - The `REMOTEID` tails are `000000000e21` to `000000000e2b` (NM's were `e01` to `e0b`; PB's, TD's and RS's are `777`, `795`, `d01` and `d10` to `d15`).
- **No exception to C3 is needed.** Each of this row's 22 imports is a new file, and none carries a key an earlier row sent. Each is sent at most once, and never again whatever happened to its first send.
- **The date and the period:** read the Gateway's current date and period at step 1. If the date is before 1-Aug-2026, or the period is not 1-Apr-26 to 31-Mar-27: stop before BK. **Do not change Tally's current date or period.**
- **BK is this row's way back.** It is taken at step 1, before anything is sent, and restored at step 8 by C6. The restore runs on every path from step 3 on that is not a STOP. After a STOP (C7, C8 or C9, or step 4's), C8 holds, and the restore waits for the run coordinator's word.
- Write down the release shown on Tally's screen.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6).

**The eleven ledgers** (`names.tsv`: `id`, invoice number, name). Each is created by `LM-<id>.xml` and given its one open bill by `I-<id>.xml`, each sent once.

| id | number | name | what differs from the control |
| --- | --- | --- | --- |
| `pl` | 0221 | `BRIDGE Ns Plain` | the control: letters and single spaces, which the type admits |
| `hy` | 0222 | `BRIDGE Ns Hk-Hm` | a hyphen |
| `dt` | 0223 | `BRIDGE Ns Dk.Dm` | a dot |
| `am` | 0224 | `BRIDGE Ns Ak&Am` | an ampersand (the requests carry `&amp;`) |
| `sl` | 0225 | `BRIDGE Ns Sk/Sm` | a slash |
| `lp` | 0226 | `BRIDGE Ns Lk(Lm` | an opening bracket alone |
| `rp` | 0227 | `BRIDGE Ns Rk)Rm` | a closing bracket alone |
| `ap` | 0228 | `BRIDGE Ns Pk'Pm` | an apostrophe (the requests carry `&apos;`) |
| `op` | 0229 | `BRIDGE Ns Ok and Om` | an operator word (`and`), letters and spaces only |
| `dd` | 0230 | `BRIDGE Ns 2023-24` | digits with a dash |
| `sc` | 0231 | `BRIDGE Ns उदाहरण` | another script (Devanagari) |

The names are invented. Each mark sits between two letters, one mark a name. Each stem's letters are its own, so no name is another's with a mark turned or dropped.

## The sends and the wedge rule
`run.py` is NM's, byte for byte: one request at a time, each answer's headers kept by curl -D in `a-<label>-<file>.headers`, and it never abandons a request.
- **The wedge rule.** If any send has not answered **5 minutes** after it began (NM's bound, kept on purpose though it is tighter than LAB-COMMON's): press nothing, do not end curl, send nothing more (not even `GET /status`), screenshot the whole Tally window, and report to the run coordinator (C9). Nothing is re-sent on this row's own judgement. On this book NM's reads each answered within about 4.2 s.
- **The stops.** `run.py`'s stops are failed requests, never "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 B. So are the STOPs of `count20.py`, `nm.py` and `st.py`. Each stops the row there, with one exception.
- **The one exception: an empty answer to an `L-lo-` request.** It is looser than C3, which makes every empty body a STOP, so it holds only with the run coordinator's word given when the row is queued, as NM's was. Without that word, an empty `L-lo-` answer is a STOP like any other.
  - A name that matches no ledger is answered with no bytes (row 2377-L). At step 3 that is the expected answer for every name.
  - It is recorded first as what it is: `run.py` printed "empty answer", so the request failed.
  - It is not a STOP, and the step goes on, only if its headers file shows HTTP status 200 and `CONTENT-LENGTH: 0`. `run.py`'s "stop, do not resend" is then read as this exception.
  - Any other empty body (no headers file, no status line, another status or another length) ended without an answer: a STOP, and the request is never sent again.
  - Then, in this order: keep the headers file and the log line, and screenshot the whole Tally window; if a dialog is up: STOP; send `GET /status` once (C2c's command), recording its status and bytes, and if it is not 200: STOP; run `nm.py` on that answer, and go on with the step.
  - Each `GET /status` this sends is a request of the row: it is logged with the rest, and there may be up to 22 of them (11 at step 3, 11 at step 6).
  - At step 6, an empty `r<n>a` under this exception is followed by its `r<n>b`, as for any other answer.
  - An empty answer to any other request, an `LM-` or `I-` import included, is a STOP, as `run.py` says.
- Check each file's sha256 against `SHA256.txt` before its send. No file in `nm/` or `pb/` is ever an argument to `run.py`: they are sources for `gen-ns.py`, and `pb/L-ledger.xml` is a ledger create.
- Close no Bridge process, and call no Bridge tool.

**Which script reads which answer**
- **`count20.py`** (NM's and PB's, byte-equal) reads `a-*-M-marks.xml` and `a-*-N-*`. Its `--before-write` and `--same-row` are the marks gate and the marks compare. Give it no `L-lo-` answer.
- **`nm.py`** (NM's, byte-equal; it takes the eleven names from this folder's `names.tsv`) reads `a-*-M-marks.xml`, `a-*-S-snap-*`, `a-*-B-bills.xml` and `a-*-L-lo-<id>-*`, with its modes `--outcome`, `--against` and `--same`, whole in its header.
- **`st.py`** (new) reads the import answers `a-*-LM-*` and `a-*-I-*` (`--import`) and the snapshot answers (`--before`, `--names`). Its modes are whole in its header.

**A ledger read** (LR below) is the report's two sends for one ledger, `fetch_party_bills`'s second pair (`runtime_party_bills.rs:57`): `r<n>a` and `r<n>b`, the same file twice. The snapshot's pair is sent once for all eleven, in step 5, before any LR.

## Files
**Byte-equal to NM's file of the same name** (`lab/2480-nm-sheet` at 73cb01ae, folder `lab/2480-nm/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count20.py`, `nm.py`, `M-marks.xml`, `B-bills.xml`, `S-snap-20260401-20260801.xml`, the two templates (`template-import-0010.utf16le.xml`, `template-n-0001.utf16le.xml`), and `pb/` (PB's `L-ledger.xml`, `I-0099.xml`, `N-0099.xml` and `L-lo-bc-20260801.xml`, never sent).

**NM's, kept in `nm/`, never sent:** NM's `names.tsv` and its eleven `L-lo-<id>-20260801.xml`, which NM's render test made from master's builder template.

**Written for this row:** `names.tsv`, `st.py`, `gen-ns.py` and this file.

**Made by `gen-ns.py`** (44 files), which is NM's `gen-nm.py` with the lines marked NS changed. Each source file's sha256 is checked first, and it asserts that the same functions give PB's own files byte for byte.
- `LM-<id>.xml`, `I-<id>.xml` and `N-<number>.xml`: as NM made them, with this row's name, number and tail.
- `L-lo-<id>-20260801.xml`: NM's `L-lo-pl` with its one escaped name turned into this row's escaped name, and nothing else. It first asserts that the same turn gives each of NM's eleven `L-lo` files from NM's `L-lo-pl`, byte for byte. So each request is master's builder template with only the name changed, as NM's render test checked (its ROW.md, "What it asserted first"). No build was needed.
- It also asserts: no two of this row's names share a key, none shares a key or a number with NM's, and each name occurs in its files as often as NM's did.

`SHA256.txt` lists every file with its size. Requests are UTF-16LE with a BOM, as sent.

**Tried before this sheet was sent:**
- `gen-ns.py`: its 44 files, each check passing.
- `st.py`, on NM's own answers (`lab/2480-nm-answers` at 12a84c9a), with this row's names unless said otherwise:
  - `--import`: `CREATED ONE` (exit 0) on `a-s3pl-LM-pl.xml`, `a-s4pl-I-pl.xml` and `a-s4sc-I-sc.xml`; `STOP (CREATED is not 1; ALTERED is not 0)` (exit 1) on `a-s3dt-LM-dt.xml` and `a-s3am-LM-am.xml`; a STOP on a marks answer, which holds no `RESPONSE`.
  - `--before`: `FOLD: CLEAR` on NM's `s2` snapshot (18 ledgers) and on its `c8s` snapshot (24 ledgers). With NM's names in place of this row's, on `c8s`: a STOP naming all eleven; the dot, ampersand, slash and opening-bracket names by the hyphen ledger's key, and the rest as held by the book.
  - `--names`: with NM's names on `c8s`, `NAMES: DIFFER`, the hyphen ledger's NAME attribute against the name list `['BRIDGE Nm Ax)Bx']`; on `s2`, `NAMES: EQUAL`.
- **Not tried:** any send to Tally.

## Steps
Run each label as `python3 -I run.py http://127.0.0.1:9001 <label> <files>`, the Gateway on 9001 as C2 finds it (written `URL` below). (Row 2480-NM's PC has no `python3` command; there, `python -I` with Python 3 is the same command.) The prompt rule, the wait rule and the path rule are NM's.

**Labels are not step numbers.** Step 1 sends `s1`, step 2 `s2`, step 3 `p1` to `p12`, step 4 `s4<id>` and `s5<id>` (with `m` after each for its marks read), step 5 `s6…`, step 6 `r<n>a` and `r<n>b`, step 7 `m1`, step 8 `s9`.

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named. For any other prompt, message or dialog: copy it word for word and screenshot it first; then answer No, or Esc if it offers no No; press nothing more and send nothing. That prompt is this row's result, and C8 holds: the restore waits for the run coordinator's word.

**Each write, the same way, by hand, one label a command** (the write recipe, used only in step 4):
- (a) `run.py URL <label>m M-marks.xml`, then `python3 -I count20.py --before-write a-s1-M-marks.xml a-<label>m-M-marks.xml`.
- (b) Only if that passes, and no key was pressed in Tally since: `run.py URL <label> <file>`, the import alone.
- (c) Screenshot the whole window before the next request (C3).
- (d) `python3 -I st.py --import a-<label>-<file>`, recorded whole. **Only on exit 0 is the next request of the row sent.** On any other exit of `st.py`, or any exit other than 0 of `run.py` in (b): the STOP of the set-up (step 4).

The set-up (step 4) is not run as a loop. If a script sends any of it, the script ends at the first exit other than 0 of `run.py` or `st.py` and sends nothing after it. Nothing else decides whether an import answer is as expected.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down (Book).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2480-ns\backup-1` (Tally may show it as `C:\lab\2480-Ns\backup-1`). NM's step 1 is the path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0221.xml N-0222.xml N-0223.xml N-0224.xml N-0225.xml N-0226.xml N-0227.xml N-0228.xml N-0229.xml N-0230.xml N-0231.xml B-bills.xml S-snap-20260401-20260801.xml M-marks.xml`.
   - EXPECT each N read 0 rows (`count20.py`), and `nm.py` on B-bills no group for any of the eleven names, and on the snapshot "no row" for each. A row or a group: stop before step 3, each recorded whole.
   - `st.py --before a-s2-S-snap-20260401-20260801.xml`: EXPECT `FOLD: CLEAR`. A STOP: stop before step 3, recorded whole.
   - `st.py --names a-s2-S-snap-20260401-20260801.xml`, recorded whole.
3. **Reads first: each name before its ledger exists.** For each id in the order of the table, `p<n>`: `L-lo-<id>-20260801.xml`, sent once (`p1` for `pl` to `p11` for `sc`, the numbers of step 6's table).
   - Screenshot the whole Tally window after each, at once, before the next request: each is a request the lab has not been sent. A dialog: the prompt rule.
   - `nm.py a-p<n>-L-lo-<id>-20260801.xml`, recorded whole. EXPECT `ANSWER: NO BYTES`, under the exception above.
   - **An answer with bytes** is that name's result, the most important kind this row can give: record it whole, with `nm.py --against <id> a-p<n>-L-lo-<id>-20260801.xml a-s2-B-bills.xml a-s2-S-snap-20260401-20260801.xml`. That id's ledger and invoice are never sent; step 4 skips it. The row goes on.
   - Then `p12`: `M-marks.xml`, and `count20.py --same-row a-s1-M-marks.xml a-p12-M-marks.xml`. EXPECT SAME MARKS. On MARKS DIFFER: send `M-marks.xml` once more as `p12r` and record it; send nothing else, and STOP.
4. **The set-up,** one ledger at a time in the order of the table, each id's ledger and then its invoice, each write as above. The labels are `s4<id>` for `LM-<id>.xml` and `s5<id>` for `I-<id>.xml`, with `s4<id>m` and `s5<id>m` for the marks reads before them.
   - **The STOP of the set-up.** The first `st.py --import` exit other than 0, on any answer, a ledger's or an invoice's, the control's or a mark's; or an exit other than 0 of `run.py` on an `LM-` or `I-` file (a curl failure, an empty answer, Unknown Request or an answer above the bound), after which what the import did is not known and the file is never sent again: send nothing more, not even the invoice of that id; screenshot the whole window; record the answer whole; and report to the run coordinator. C8 holds. The book then holds each ledger and invoice created before it, and possibly a ledger that answer altered: say which in the report.
   - This is the point at which row 2480-NM went on. Nothing in this row goes on after it.
5. **The book-wide reads and the state.** `s6m`: `M-marks.xml`. Then `s6sa`, `s6sb`: `S-snap-20260401-20260801.xml`, the same file twice; and `s6ba`, `s6bb`: `B-bills.xml`, the same file twice. Neither request is a new shape (NM's files, byte for byte); a dialog at any time is still the prompt rule.
   - `nm.py` on `a-s6sa-S-snap-20260401-20260801.xml` and `a-s6ba-B-bills.xml`, each recorded whole.
   - `nm.py --same` on each pair, recorded.
   - `st.py --names a-s6sa-S-snap-20260401-20260801.xml`, recorded whole. EXPECT `NAMES: EQUAL`, each created ledger's name list its own name. A ledger whose name list is not its NAME attribute is a set-up result, recorded whole. Step 6 still runs: it only reads.
   - **It is the intended state** if S holds a row for each created ledger, `ISBILLWISEON` Yes, `CLOSINGBALANCE` -1051.00, and B one group for each, `BP/26-27/<its number>`, -1051.00. If not, step 6 is still sent for the ledgers that exist, and the report says the state is not the intended one.
   - If the two snapshot answers are not byte-equal, master would send nothing more for any ledger; this row still sends step 6, and records that master would not have.
6. **The ledger reads.** For each id in the order of the table that has a ledger and a bill, LR with the labels below. An id has a ledger and a bill when `st.py --import` gave exit 0 on both its files at step 4 and `nm.py` on `a-s6sa-…` shows a row for its name. Any other id is not read here, and the report says why. The request is `L-lo-<id>-20260801.xml`, the file step 3 sent.

   | n | id | labels | n | id | labels |
   | --- | --- | --- | --- | --- | --- |
   | 1 | `pl` | `r1a`, `r1b` | 7 | `rp` | `r7a`, `r7b` |
   | 2 | `hy` | `r2a`, `r2b` | 8 | `ap` | `r8a`, `r8b` |
   | 3 | `dt` | `r3a`, `r3b` | 9 | `op` | `r9a`, `r9b` |
   | 4 | `am` | `r4a`, `r4b` | 10 | `dd` | `r10a`, `r10b` |
   | 5 | `sl` | `r5a`, `r5b` | 11 | `sc` | `r11a`, `r11b` |
   | 6 | `lp` | `r6a`, `r6b` | | | |

   - Screenshot the whole Tally window after each `r<n>a`, before the next request. A dialog: the prompt rule.
   - Then, for each id, recorded whole: `nm.py a-r<n>a-L-lo-<id>-20260801.xml`; `nm.py --same a-r<n>a-… a-r<n>b-…`; `nm.py --outcome <id> a-s6sa-S-snap-20260401-20260801.xml a-s6sb-S-snap-20260401-20260801.xml a-r<n>a-L-lo-<id>-20260801.xml a-r<n>b-L-lo-<id>-20260801.xml`; `nm.py --against <id> a-r<n>a-L-lo-<id>-20260801.xml a-s6ba-B-bills.xml a-s6sa-S-snap-20260401-20260801.xml`.
   - A ledger's answer never stops the row: each is a result, recorded in full. Only the STOPs named stop it.
7. **`m1`:** `M-marks.xml`, then `count20.py --same-row a-s6m-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS: steps 5 and 6 only read. On MARKS DIFFER: send `M-marks.xml` once more as `m1r` and record it; send nothing else, and STOP. The answers already recorded stand.
8. **Restore** BK by C6, with `restore-plan.txt` written first. NM's step 7 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace).
   - Then `s9`: every file of s2, in s2's order, and compare whole: `count20.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml` passes; each N read 0 rows; B-bills no group of the eleven names; the snapshot "no row" for each; each answer's sha256 equal to s2's.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The coordinator decides.
9. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering. Screenshot the window, send it and the answers to the run coordinator, and ask before anything else is pressed or sent. Say in the report what the book then holds.

## Record
- Each answer's bytes and sha256, its headers file whole, and each script's lines, whole.
- `run.py`'s log for each label, and how long each report read took.
- Every `--before-write`, `--same-row`, `--import`, `--before`, `--names`, `--outcome`, `--against` and `--same` output, whole.
- The window after each import and after each shot named in steps 3 and 6, listed by name, bytes and sha256.
- Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen, and the Gateway's date and period.
- For each ledger: its step 3 answer; whether Tally created it and whether the invoice made its bill; its name list at step 5; and its three reads (the report pair, the snapshot row, the Bills Receivable group).

**Staging the evidence:** the check for 10 or more digits sets aside this book's registered GUID matched whole, and nothing else. Any other hit waits for the run coordinator.

## Expected
Estimates, not predictions. Debit amounts are negative, as the report and the snapshot sign them.
- **s2:** as the Book and step 2 say; `FOLD: CLEAR`; `NAMES: EQUAL`.
- **Step 3:** `ANSWER: NO BYTES` (HTTP 200, `CONTENT-LENGTH: 0`) for each name, as row 2377-L saw for a name matching no ledger. No estimate beyond that: no request of the lab has carried nine of these names' marks in this report.
- **Step 4:** `IMPORT: CREATED ONE` for the control and for the five names NM created (hyphen, apostrophe, operator word, digits, Devanagari). For the five NM saw altered, no estimate: whether a stem of their own is enough is what step 4 measures.
- **Step 5:** S one row each, -1051.00, Yes; B one group each, `BP/26-27/<number>`, -1051.00; `NAMES: EQUAL`.
- **The control `pl`:** `ANSWER: ROWS 1`, the bill `BP/26-27/0221`, -1051.00; `OUTCOME: told`; `AGAINST: EQUAL`.
- **The hyphen `hy`:** as the control, since row 2346-PB's report for a hyphen name was answered with its bills.
- **The other nine:** no estimate.
- **s9:** equal to s2 and s1.

## Decides
Each is PARTIAL: one book, one release, one sitting, master's request template sent by curl, without the reads Bridge sends around it, and without the build's own parse.

**For each of the ten names,** one of these. The control is the baseline: if it is not `told` with `AGAINST: EQUAL`, no other name is judged until it is explained.
- **Step 3 answered with bytes:** Tally reads that name, before its ledger exists, as some other ledger. That mark is not admitted, whatever else is measured, and the coordinator decides what it means for the alphabet.
- **`told`, `AGAINST: EQUAL`, the one bill the ledger's own, the two sends equal, and NO BYTES at step 3:** on this book and release, Tally reads that name in the `LedgerName` static variable as that ledger and as no other. This is the evidence #2480 asks for, for that one mark. It says nothing about the formula read (`CHILDOF`).
- **NO BYTES at step 6,** for an id with a ledger and a bill as step 6 defines it: Tally matched no ledger for that name; the build keeps refusing it. The row cannot tell whether the mark or its escape is the cause.
- **Rows that are another ledger's at step 6:** as step 3's, the most important result.
- **`party_bills_untied`, `party_bills_answer_malformed`, `party_bills_tally_failure` or another code:** master's parse would refuse that read for that name, so widening the alphabet would not make it work as it is.
- **`SAME: DIFFER`:** record both; master would refuse as `party_bills_drifted`.
- **The STOP of step 4 at that name's import:** what Tally did with that name as a new ledger, beside these stems; no name after it is measured, and the coordinator decides whether a later row continues.
- **A name list not its own at step 5:** an import answered `CREATED` 1 still changed a ledger's name; recorded, and a finding for any writer as much as for #2480.

**What these answers are for.** They may inform #2480's change: widening the alphabet for both reads, or giving the static-variable read a type of its own with the measured alphabet. They become fixtures only if the manifest admits them.

## Code admits the trigger
Lines are at master 5bf79364.
- **The build refuses these names before any send.** `LedgerScopeName::new` (`src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65`) refuses any character outside ASCII letters, digits and a single space (`:74`) and the words of `FORMULA_WORDS` (`:30`). `admitted_party` (`src-tauri/src/agent_party_bills.rs:26`) maps each refusal to `party_name_not_admitted`. So no build reaches this row's reads, and this row sends by curl.
- **The bills read is master's.** `fetch_party_bills` (`src-tauri/src/tally/runtime_party_bills.rs:57`) reads the snapshot and then the report, each sent twice, and ties them. `render_ledger_outstandings_request` (`src-tauri/crates/bridge-tally-protocol/src/native_outstandings/party_bills.rs:39`) takes the party as the text of the `LedgerName` static variable, escaped once as XML text.
- **The key `st.py` checks is master's collision key, read in Python:** `letters_and_digits_key` (`src-tauri/crates/bridge-tally-core/src/master_binding.rs:1982`), which the ledger-create path asks at admission (`admit_free_name`, `src-tauri/src/agent_master_post.rs:338`).

## Not in this row
- ComplyEaze Bridge's own transport, or any Bridge build;
- the `CHILDOF` formula read and `ledger_movement`'s ledger-scoped voucher read ("the formula read stays as it is unless the same row covers it", #2480);
- why NM's five names folded onto the hyphen ledger: what Tally matches a created name on, and the same marks against one stem (that is NM's question turned round, and its own row);
- a mark not in the table, a name with two or more marks, a mark beside a space or at an edge, another operator word, a name of the longest admitted length, a name that differs from another only by case;
- a ledger with more than one bill, a settled or part-settled bill, a note or a voucher after the invoice;
- another to-date, book or release; a book with GST on; Educational mode.
