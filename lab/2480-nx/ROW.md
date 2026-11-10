# Row 2480-NX: does Tally's `Ledger Outstandings` report read a ledger whose name holds one of ten marks rows 2480-NS and 2480-NC did not measure?

Shape discovery, as rows 2346-PB, 2480-NM, 2480-NS and 2480-NC: the requests are written for the lab, not requests the build sends, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2480 (0.9.0). Written 11 Nov 2026. Lines are at master 812d39bb. Confidence of everything here until the row runs: unverified.

## Why a fourth row
Rows 2480-NS (`lab/2480-ns-sheet` at 6e44a3dc) and 2480-NC (`lab/2480-nc-sheet` at 8eb5012c, answers `lab/2480-nc-answers` at 2ce5b6ea) read every name tried as its own ledger, one measured mark a name and then mixes of them. Those marks were `-`, `.`, `&`, `/`, `(`, `)`, `'`, the word `and`, digits with a dash, and a Devanagari word. A static-variable type built on that evidence still refuses every other mark. The commonest of those in real party names is the comma (`Gupta, Verma & Co`). This row measures ten of them, one a name, by NS's method.

**What this row changes from NS and NC:**
1. **The names.** Each name but the control holds exactly one of ten marks no earlier row of the #2480 series sent in a name: `,` `_` `"` `+` `@` `#` `:` `%` `*` `!`. It holds no other mark. Each of the ten is held by one name. Each name has a stem of its own: no two names of this row share a key, and none shares a key with one of NM's, NS's or NC's names. `gen-nx.py` asserts all of it.
2. **The numbers and tails.** The invoice numbers are `BP/26-27/0261` to `0271`, and the `REMOTEID` tails `0e61` to `0e6b`.
3. **The ids** are this row's own (table below).
4. **One addition to the method: a refused import is a result, not a STOP.** NS's and NC's write gate passed only `CREATED` 1 and stopped on anything else. Tally may refuse some of these marks in a ledger name. A refusal is then the answer for that name, and it changes nothing in the book. So an import that Tally refused, and that changed nothing, is recorded for its id; the rest of that id is skipped and the row goes on. Two checks decide that the import was refused: `rf.py` (new) reads the answer, and a marks read after the import must equal the marks read before it. Anything else that is not `CREATED` 1 is still the STOP of the set-up: an alteration, a merge, a deletion, a marks change, or an answer `rf.py` cannot place. Step 4 says how.

Everything else is NS's and NC's: the reads first, the exact write gate (`st.py --import`, byte-equal to NS's), the empty-answer exception, the restore, and every other script.

## Question
For each of the ten names master's party type refuses, one name a ledger:
1. **Before the ledger exists, what does the report answer for that name?** No bytes is expected. Any bill is another ledger's.
2. **Does Tally create the ledger and its bill from master's import shapes, or refuse them?**
3. **Once it exists with one bill, does the report answer with that ledger's own bill?** Master's own request, with only the name changed, sent twice.
4. **Does that answer agree with the other two reads of the book?** The ledger snapshot's closing for the ledger (the tie master makes), and the book-wide Bills Receivable group for it.

**The one variable** in each ledger's answers is its name, and in each name, its one mark. The ledgers are alike but for the name: the same parent group, the same bill-wise setting, one invoice of the same amount on the same day, a bill named by the invoice's number. The control ledger, whose name the type admits, is read the same way.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (byte-equal to NC's, NS's and NM's) applies in full. Where it and this sheet differ, the stricter one holds.

- **Pinned by GUID.** `count20.py`, `nm.py` and `st.py` each hold BRIDGE PILOT LAB's GUID as `BOOK`, and each STOPs on a row with another. (`rf.py` reads import answers, which carry no GUID; the marks read beside each import is pinned.)
- **Its state:** ALTVCHID 68 and ALTMSTID 245, the marks NC's restore left (its `s9`, equal to its `s1`), if every row since has ended with its restore, as the coordinator reports. Other marks at s1: stop before BK, with nothing sent.
- **It holds none of the eleven ledgers and none of the numbers below.** Step 2 checks this, and a row there is a STOP.
  - The ledgers are the eleven names of `names.tsv`, all beginning `BRIDGE Nx `.
  - The invoice numbers are `BP/26-27/0261` to `BP/26-27/0271` (NC's were 0241 to 0251, NS's 0221 to 0231, NM's 0201 to 0211).
  - The `REMOTEID` tails are `000000000e61` to `000000000e6b` (NC's were `e41` to `e4b`, NS's `e21` to `e2b`, NM's `e01` to `e0b`; PB's, TD's and RS's are `777`, `795`, `d01` and `d10` to `d15`).
- **Each import is new and sent once.** Each of this row's 22 imports is a new file, and none carries a key an earlier row sent. Each is sent at most once, and never again whatever happened to its first send.
- **One exception to C3, for this row only, given with the run coordinator's word when the row was asked for.** C3 says that if a read does not show a write's effect, the row STOPs. Here, an import that step 4 (e) finds refused shows no effect, by design: that absence is the result for its id, and the row goes on. The exception covers that import alone. Every other write that a read does not show is C3's STOP.
- **The date and the period:** read the Gateway's current date and period at step 1. If the date is before 1-Aug-2026, or the period is not 1-Apr-26 to 31-Mar-27: stop before BK. **Do not change Tally's current date or period.**
- **BK is this row's way back.** It is taken at step 1, before anything is sent, and restored at step 8 by C6. The restore runs on every path from step 3 on that is not a STOP. After a STOP (C7, C8 or C9, or step 4's), C8 holds, and the restore waits for the run coordinator's word.
- Write down the release shown on Tally's screen.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6).

**The eleven ledgers** (`names.tsv`: `id`, invoice number, name). Each is created by `LM-<id>.xml` and given its one open bill by `I-<id>.xml`, each sent at most once.

| id | number | name | its one mark |
| --- | --- | --- | --- |
| `pl` | 0261 | `BRIDGE Nx Plain` | none: the control, letters and single spaces, which the type admits |
| `cm` | 0262 | `BRIDGE Nx Ak, Am` | `,` after a word, before a space |
| `us` | 0263 | `BRIDGE Nx Bk_Bm` | `_` between two words, no space |
| `dq` | 0264 | `BRIDGE Nx "Ck" Cm` | `"` as a pair round a word, sent escaped once as XML text (`&quot;`) |
| `ps` | 0265 | `BRIDGE Nx Dk + Dm` | `+` between spaces |
| `at` | 0266 | `BRIDGE Nx Ek @ Em` | `@` between spaces |
| `hs` | 0267 | `BRIDGE Nx Fk #12` | `#` before digits |
| `cl` | 0268 | `BRIDGE Nx Gk: Gm` | `:` after a word, before a space |
| `pc` | 0269 | `BRIDGE Nx Hk 15% Hm` | `%` after digits |
| `st` | 0270 | `BRIDGE Nx Jk * Jm` | `*` between spaces |
| `ex` | 0271 | `BRIDGE Nx Kk!` | `!` at the end |

The names are invented. Each stem's letters are its own, so no name is another's with a mark turned or dropped. No name holds a double space or a space at an edge. Every mark reaches Tally only as escaped XML text: in the report request, as the text of the `LedgerName` static variable; in the imports, as master's import shapes carry a name. None is sent inside a formula.

## The sends and the wedge rule
`run.py` is NM's, byte for byte: one request at a time, each answer's headers kept by curl -D in `a-<label>-<file>.headers`, and it never abandons a request.
- **The wedge rule.** If any send has not answered **5 minutes** after it began (NM's bound, kept on purpose though it is tighter than LAB-COMMON's): press nothing, do not end curl, send nothing more (not even `GET /status`), screenshot the whole Tally window, and report to the run coordinator (C9). Nothing is re-sent on this row's own judgement. On this book NC's reads each answered within about 1 s, and the reads with a capture within about 5.6 s.
- **The stops.** `run.py`'s stops are failed requests, never "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 B. So are the STOPs of `count20.py`, `nm.py`, `st.py` and `rf.py`. Each stops the row there, with two exceptions: the empty `L-lo-` answer below, and the refused import of step 4.
- **The empty-answer exception for an `L-lo-` request.** It is looser than C3, which makes every empty body a STOP, so it holds only with the run coordinator's word given when the row is queued, as NM's, NS's and NC's were. Without that word, an empty `L-lo-` answer is a STOP like any other.
  - A name that matches no ledger is answered with no bytes (row 2377-L, and all eleven of NS's and of NC's step 3). At step 3 that is the expected answer for every name.
  - It is recorded first as what it is: `run.py` printed "empty answer", so the request failed.
  - It is not a STOP, and the step goes on, only if its headers file shows HTTP status 200 and `CONTENT-LENGTH: 0`. `run.py`'s "stop, do not resend" is then read as this exception.
  - Any other empty body (no headers file, no status line, another status or another length) ended without an answer: a STOP, and the request is never sent again.
  - Then, in this order: keep the headers file and the log line, and screenshot the whole Tally window; if a dialog is up: STOP; send `GET /status` once (C2c's command), recording its status and bytes, and if it is not 200: STOP; run `nm.py` on that answer, and go on with the step.
  - Each `GET /status` this sends is a request of the row: it is logged with the rest, and there may be up to 33 of them (11 at step 3, 22 at step 6).
  - At step 6, an empty `r<n>a` under this exception is followed by its `r<n>b`, as for any other answer.
  - An empty answer to any other request, an `LM-` or `I-` import included, is a STOP, as `run.py` says.
- **What the branch and any public text say about a STOP of this kind.** If a send wedges (the wedge rule), Tally exits or stops answering, or a dialog the prompt rule names comes up, the row's RESULTS and any public comment on it say only "a STOP at step N, id X" (or "a STOP at step N" where no id applies, as at steps 1, 2, 7, 8 and 9). The request that preceded it, the screens, the timings and anything else that would show how to cause it go to the run coordinator by the private channel, not to the branch. The coordinator decides what, if anything, is published later.
- Check each file's sha256 against `SHA256.txt` before its send. No file in `nm/`, `ns/`, `nc/` or `pb/` is ever an argument to `run.py`: they are sources for `gen-nx.py`, and `pb/L-ledger.xml` is a ledger create.
- Close no Bridge process, and call no Bridge tool.

**Which script reads which answer**
- **`count20.py`** (NM's and PB's, byte-equal) reads `a-*-M-marks.xml` and `a-*-N-*`. Its `--before-write` and `--same-row` are the marks gate and the marks compare. Give it no `L-lo-` answer.
- **`nm.py`** (NM's, byte-equal; it takes the eleven names from this folder's `names.tsv`) reads `a-*-M-marks.xml`, `a-*-S-snap-*`, `a-*-B-bills.xml` and `a-*-L-lo-<id>-*`, with its modes `--outcome`, `--against` and `--same`, whole in its header.
- **`st.py`** (NS's, byte-equal; it too takes the names from `names.tsv`) reads the import answers `a-*-LM-*` and `a-*-I-*` (`--import`) and the snapshot answers (`--before`, `--names`). Its modes are whole in its header.
- **`rf.py`** (new for this row) reads an import answer that `st.py --import` did not pass, and prints `IMPORT: REFUSED` (exit 0) or `IMPORT: NOT A REFUSAL` (exit 1), each with the counters and every `LINEERROR` whole. Its rule is whole in its header.

**A ledger read** (LR below) is the report's two sends for one ledger, `fetch_party_bills`'s second pair (`runtime_party_bills.rs:57`): `r<n>a` and `r<n>b`, the same file twice. The snapshot's pair is sent once for all ledgers, in step 5, before any LR.

## Files
**Byte-equal to NC's file of the same name** (`lab/2480-nc-sheet` at 8eb5012c, folder `lab/2480-nc/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count20.py`, `nm.py`, `st.py`, `M-marks.xml`, `B-bills.xml`, `S-snap-20260401-20260801.xml`, the two templates (`template-import-0010.utf16le.xml`, `template-n-0001.utf16le.xml`), `pb/` (PB's `L-ledger.xml`, `I-0099.xml`, `N-0099.xml` and `L-lo-bc-20260801.xml`, never sent), `nm/` (NM's `names.tsv` and its eleven `L-lo-<id>-20260801.xml`, never sent) and `ns/` (NS's `names.tsv`, never sent). Each is NS's file too, as NC's ROW.md lists.

**NC's, kept in `nc/`, never sent:** NC's `names.tsv`.

**Written for this row:** `names.tsv`, `gen-nx.py`, `rf.py` and this file.

**Made by `gen-nx.py`** (44 files), which is NC's `gen-nc.py` with the lines marked NX changed. Each source file's sha256 is checked first, and it asserts that the same functions give PB's own files byte for byte, and NM's eleven `L-lo` files from NM's `L-lo-pl`.
- `LM-<id>.xml`, `I-<id>.xml` and `N-<number>.xml`: as NC made them, with this row's name, number and tail.
- `L-lo-<id>-20260801.xml`: NM's `L-lo-pl` with its one escaped name turned into this row's escaped name, and nothing else. So each request is master's builder template with only the name changed, as NM's render test checked.
- It also asserts:
  - each name but the control holds exactly one of the ten marks (the quote as one pair), no other mark and nothing outside ASCII, and each of the ten marks is held by one name;
  - no two of this row's names share a key, and none shares a key or a number with NM's, NS's or NC's;
  - each name occurs in its files as often as NC's did.

`SHA256.txt` lists every file with its size, in NS's and NC's form (`sha256  size  path`, which `shasum -c` does not read). Requests are UTF-16LE with a BOM, as sent.

**Tried before this sheet was sent:**
- `gen-nx.py`: its 44 files, each check passing. Read back by an XML parser: each `L-lo` file's `LedgerName` is its name alone, each `LM` file's `NAME` attribute and element are its name with `ACTION` `Create`, each `I` file carries its name, number and tail, and each `N` file its number.
- `st.py`, on NC's own answers (`lab/2480-nc-answers` at 2ce5b6ea), with this row's names:
  - `--before`: `FOLD: CLEAR` on NC's `s2` snapshot (18 ledgers) and on its `s6sa` snapshot (29 ledgers, NC's eleven included);
  - `--names`: `NAMES: EQUAL` on `s6sa`;
  - `--import`: `CREATED ONE` on `a-s4pl-LM-pl.xml`.
- `rf.py`:
  - on real answers: `NOT A REFUSAL` (exit 1) on NM's `ALTERED` 1 answers (`a-s3am-LM-am.xml`, `a-s3dt-LM-dt.xml`) and on a `CREATED` 1 answer; a STOP (exit 1) on a marks answer;
  - on answer shapes written by hand for the test, not kept: `REFUSED` on all counters with `ERRORS` 1 and a `LINEERROR`, and on all counters with `EXCEPTIONS` 1; `NOT A REFUSAL` on `IGNORED` 1 with `ERRORS` 1, on `ALTERED` 1 with `ERRORS` 1, on a `LINEERROR` with every counter 0, on counters missing, and on a counter given twice; a STOP on a lone `LINEERROR` with no `RESPONSE`.
  - **Its `REFUSED` path has not met a real refusal.** No earlier row captured one. An answer it does not place is `NOT A REFUSAL`, which is a STOP, so it errs toward stopping.
- **Not tried:** any send to Tally.

## Steps
Run each label as `python3 -I run.py http://127.0.0.1:9001 <label> <files>`, the Gateway on 9001 as C2 finds it (written `URL` below). (The lab PC has no `python3` command; there, `python -I` with Python 3 is the same command, as NS and NC ran it.) The prompt rule, the wait rule and the path rule are NM's.

**Labels are not step numbers.** Step 1 sends `s1`, step 2 `s2`, step 3 `p1` to `p12`, step 4 `s4<id>` and `s5<id>` (with `m` after each for its marks read, and `x` after each for the marks read after a refusal), step 5 `s6…`, step 6 `r<n>a` and `r<n>b`, step 7 `m1`, step 8 `s9`.

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named. For any other prompt, message or dialog: copy it word for word and screenshot it first; then answer No, or Esc if it offers no No; press nothing more and send nothing. That prompt is this row's result, and C8 holds: the restore waits for the run coordinator's word. What the branch says about it is the STOP rule above.

**Each write, the same way, by hand, one label a command** (the write recipe, used only in step 4):
- (a) `run.py URL <label>m M-marks.xml`, then `python3 -I count20.py --before-write a-s1-M-marks.xml a-<label>m-M-marks.xml`.
- (b) Only if that passes, and no key was pressed in Tally since: `run.py URL <label> <file>`, the import alone.
- (c) Screenshot the whole window before the next request (C3). A dialog: the prompt rule.
- (d) `python3 -I st.py --import a-<label>-<file>`, recorded whole. **On exit 0 (`CREATED ONE`), go on with the next write.**
- (e) **Only if `st.py` gave exit 1 and `run.py` in (b) gave exit 0:** `python3 -I rf.py a-<label>-<file>`, recorded whole.
  - On exit 1 (`NOT A REFUSAL`, or a STOP of `rf.py`): the STOP of the set-up.
  - On exit 0 (`REFUSED`): `run.py URL <label>x M-marks.xml`, then `python3 -I count20.py --same-row a-<label>m-M-marks.xml a-<label>x-M-marks.xml`. On SAME MARKS, that import is **refused** for its id: record it, and skip the rest of that id. On anything else: the STOP of the set-up, with nothing more sent.
- On any exit other than 0 of `run.py` in (b), (e) is not run: the STOP of the set-up.

The set-up (step 4) is not run as a loop. If a script sends any of it, the script ends at the first exit other than 0 of `run.py`, `count20.py` or `rf.py`, and at the first exit 1 of `st.py` that (e) does not turn into a refusal, and sends nothing after it. Nothing else decides whether an import answer is as expected.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down (Book).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2480-nx\backup-1` (Tally may show it as `C:\lab\2480-Nx\backup-1`). NC's step 1 is the path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0261.xml N-0262.xml N-0263.xml N-0264.xml N-0265.xml N-0266.xml N-0267.xml N-0268.xml N-0269.xml N-0270.xml N-0271.xml B-bills.xml S-snap-20260401-20260801.xml M-marks.xml`.
   - EXPECT each N read 0 rows (`count20.py`), and `nm.py` on B-bills no group for any of the eleven names, and on the snapshot "no row" for each. A row or a group: stop before step 3, each recorded whole.
   - `st.py --before a-s2-S-snap-20260401-20260801.xml`: EXPECT `FOLD: CLEAR`. A STOP: stop before step 3, recorded whole.
   - `st.py --names a-s2-S-snap-20260401-20260801.xml`, recorded whole.
3. **Reads first: each name before its ledger exists.** For each id in the order of the table, `p<n>`: `L-lo-<id>-20260801.xml`, sent once (`p1` for `pl` to `p11` for `ex`, the numbers of step 6's table).
   - Screenshot the whole Tally window after each, at once, before the next request: each is a request the lab has not been sent. A dialog: the prompt rule.
   - `nm.py a-p<n>-L-lo-<id>-20260801.xml`, recorded whole. EXPECT `ANSWER: NO BYTES`, under the exception above.
   - **An answer with bytes** is that name's result, the most important kind this row can give: record it whole, with `nm.py --against <id> a-p<n>-L-lo-<id>-20260801.xml a-s2-B-bills.xml a-s2-S-snap-20260401-20260801.xml`. That id's ledger and invoice are never sent; step 4 skips it. The row goes on.
   - **If the control `pl` is answered with bytes,** record it whole and STOP after `p12`: no other name can be judged against it.
   - Then `p12`: `M-marks.xml`, and `count20.py --same-row a-s1-M-marks.xml a-p12-M-marks.xml`. EXPECT SAME MARKS. On MARKS DIFFER: send `M-marks.xml` once more as `p12r` and record it; send nothing else, and STOP.
4. **The set-up,** one ledger at a time in the order of the table, each id's ledger and then its invoice, each write as above. The labels are `s4<id>` for `LM-<id>.xml` and `s5<id>` for `I-<id>.xml`, with `s4<id>m` and `s5<id>m` for the marks reads before them, and `s4<id>x` or `s5<id>x` for the marks read after a refusal.
   - **A refused ledger** (`s4<id>` refused): its invoice is never sent, and that id is not read at step 6. The next id starts at (a).
   - **A refused invoice** (`s5<id>` refused): the ledger exists with no bill. That id is not read at step 6, and step 5 records the ledger as it stands.
   - **The STOP of the set-up.** It is one of these, on any answer, a ledger's or an invoice's, the control's or a mark's:
     - an exit 1 of `st.py --import` that (e) does not turn into a refusal;
     - a refusal whose marks after it differ from those before it;
     - an exit other than 0 of `run.py` on an `LM-` or `I-` file (a curl failure, an empty answer, Unknown Request or an answer above the bound), after which what the import did is not known and the file is never sent again;
     - a dialog after an import;
     - a refusal of the control's ledger or invoice: without the control, no other name can be judged.

     At a STOP: send nothing more, not even the invoice of that id; screenshot the whole window; record the answer, if there is one, whole; and report to the run coordinator. C8 holds. The book then holds each ledger and invoice created before it, and possibly one that answer altered: say which in the report. For a dialog, or a send with no answer, the branch says only what the STOP rule of "The sends" allows, and the rest goes to the coordinator.
   - **Why the marks read after a refusal is also the check of the book.** `count20.py --same-row` compares the book's GUID, ALTVCHID and ALTMSTID. If the book was no longer loaded, or another company was, that read does not pass, so an import that failed for that reason is a STOP, not a refusal.
5. **The book-wide reads and the state.** `s6m`: `M-marks.xml`. Then `s6sa`, `s6sb`: `S-snap-20260401-20260801.xml`, the same file twice; and `s6ba`, `s6bb`: `B-bills.xml`, the same file twice. Neither request is a new shape (NM's files, byte for byte); a dialog at any time is still the prompt rule.
   - `nm.py` on `a-s6sa-S-snap-20260401-20260801.xml` and `a-s6ba-B-bills.xml`, each recorded whole.
   - `nm.py --same` on each pair, recorded.
   - `st.py --names a-s6sa-S-snap-20260401-20260801.xml`, recorded whole. EXPECT `NAMES: EQUAL`, each created ledger's name list its own name. A ledger whose name list is not its NAME attribute is a set-up result, recorded whole. Step 6 still runs: it only reads.
   - **It is the intended state** if S holds a row for each ledger created at step 4 and none for a refused one, each `ISBILLWISEON` Yes, and B one group for each id whose invoice was created, `BP/26-27/<its number>`. Each closing and bill is the invoice's amount, as NC's were. If not, step 6 is still sent for the ledgers that exist with a bill, and the report says the state is not the intended one.
   - If the two snapshot answers are not byte-equal, master would send nothing more for any ledger; this row still sends step 6, and records that master would not have.
6. **The ledger reads.** For each id in the order of the table that has a ledger and a bill, LR with the labels below. An id has a ledger and a bill when `st.py --import` gave exit 0 on both its files at step 4 and `nm.py` on `a-s6sa-…` shows a row for its name. Any other id is not read here, and the report says why (refused at its ledger, refused at its invoice, or skipped at step 3). The request is `L-lo-<id>-20260801.xml`, the file step 3 sent.

   | n | id | labels | n | id | labels |
   | --- | --- | --- | --- | --- | --- |
   | 1 | `pl` | `r1a`, `r1b` | 7 | `hs` | `r7a`, `r7b` |
   | 2 | `cm` | `r2a`, `r2b` | 8 | `cl` | `r8a`, `r8b` |
   | 3 | `us` | `r3a`, `r3b` | 9 | `pc` | `r9a`, `r9b` |
   | 4 | `dq` | `r4a`, `r4b` | 10 | `st` | `r10a`, `r10b` |
   | 5 | `ps` | `r5a`, `r5b` | 11 | `ex` | `r11a`, `r11b` |
   | 6 | `at` | `r6a`, `r6b` | | | |

   - Screenshot the whole Tally window after each `r<n>a`, before the next request. A dialog: the prompt rule.
   - Then, for each id, recorded whole: `nm.py a-r<n>a-L-lo-<id>-20260801.xml`; `nm.py --same a-r<n>a-… a-r<n>b-…`; `nm.py --outcome <id> a-s6sa-S-snap-20260401-20260801.xml a-s6sb-S-snap-20260401-20260801.xml a-r<n>a-L-lo-<id>-20260801.xml a-r<n>b-L-lo-<id>-20260801.xml`; `nm.py --against <id> a-r<n>a-L-lo-<id>-20260801.xml a-s6ba-B-bills.xml a-s6sa-S-snap-20260401-20260801.xml`.
   - A ledger's answer never stops the row: each is a result, recorded in full. Only the STOPs named stop it.
7. **`m1`:** `M-marks.xml`, then `count20.py --same-row a-s6m-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS: steps 5 and 6 only read. On MARKS DIFFER: send `M-marks.xml` once more as `m1r` and record it; send nothing else, and STOP. The answers already recorded stand.
8. **Restore** BK by C6, with `restore-plan.txt` written first. NC's step 8 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace).
   - Then `s9`: every file of s2, in s2's order, and compare whole: `count20.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml` passes; each N read 0 rows; B-bills no group of the eleven names; the snapshot "no row" for each; each answer's sha256 equal to s2's.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The coordinator decides.
9. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering. Screenshot the window, send it and the answers to the run coordinator, and ask before anything else is pressed or sent. Say in the report to the coordinator what the book then holds. What the branch says about it is the STOP rule of "The sends": only "a STOP at step N", with the id where one applies.

## Record
- Each answer's bytes and sha256, its headers file whole, and each script's lines, whole.
- `run.py`'s log for each label, and how long each report read took.
- Every `--before-write`, `--same-row`, `--import`, `--before`, `--names`, `--outcome`, `--against` and `--same` output, and every `rf.py` output, whole.
- The window after each import and after each shot named in steps 3 and 6, listed by name, bytes and sha256.
- Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC), except where the STOP rule above sends the detail to the coordinator instead.
- The BK file and the restore, as C5 and C6 ask.
- The release on Tally's screen, and the Gateway's date and period.
- For each ledger: its step 3 answer; whether Tally created it, refused it, or refused its invoice, with the `LINEERROR` text if any; its name list at step 5; and its three reads (the report pair, the snapshot row, the Bills Receivable group).

**Staging the evidence:** the check for 10 or more digits sets aside this book's registered GUID matched whole, and nothing else. Any other hit waits for the run coordinator.

## Expected
Estimates, not predictions.
- **s2:** as the Book and step 2 say; `FOLD: CLEAR`; `NAMES: EQUAL`.
- **Step 3:** `ANSWER: NO BYTES` (HTTP 200, `CONTENT-LENGTH: 0`) for each name, as NS and NC saw for all their names.
- **Step 4:** `IMPORT: CREATED ONE` for the control. For each of the ten marks, either `CREATED ONE` or a refusal; no earlier row of the #2480 series carried any of them in a name, so neither is predicted. The quote (`dq`) is the likeliest to be refused.
- **Step 5:** for each created ledger, S one row, bill-wise Yes; B one group for each created invoice; `NAMES: EQUAL`.
- **The control `pl`:** `ANSWER: ROWS 1`, the bill `BP/26-27/0261`; `OUTCOME: told`; `AGAINST: EQUAL`.
- **The other ten, if created:** as the control, if these marks act as NS's did. No stronger estimate.
- **s9:** equal to s2 and s1.

## Decides
Each is PARTIAL: one book, one release, one sitting, master's request template sent by curl, without the reads Bridge sends around it, and without the build's own parse.

**For each of the ten names,** one of these. The control is the baseline: if it is not `told` with `AGAINST: EQUAL`, no other name is judged until it is explained.
- **Step 3 answered with bytes:** Tally reads that name, before its ledger exists, as some other ledger. That mark is not admitted, and the coordinator decides what it means for the type.
- **Refused at its ledger:** Tally will not hold that name through master's ledger import. The report read was not tried, so the row says nothing about whether the report could read such a ledger. A ledger keyed by hand might still hold the name, and this row does not measure that.
- **Refused at its invoice:** the ledger exists but master's invoice shape would not post to it. Recorded; the read was not tried.
- **`told`, `AGAINST: EQUAL`, the one bill the ledger's own, the two sends equal, and NO BYTES at step 3:** on this book and release, Tally reads that name in the `LedgerName` static variable as that ledger and as no other. With NS and NC, that is evidence for a type that admits that mark. It says nothing about the formula read (`CHILDOF`).
- **NO BYTES at step 6,** for an id with a ledger and a bill as step 6 defines it: Tally matched no ledger for that name, so the mark is not admitted.
- **Rows that are another ledger's at step 6:** as step 3's, the most important result.
- **`party_bills_untied`, `party_bills_answer_malformed`, `party_bills_tally_failure` or another code:** master's parse would refuse that read for that name, so admitting the mark would not make it work as it is.
- **`SAME: DIFFER`:** record both; master would refuse as `party_bills_drifted`.
- **The STOP of step 4 at that name's import:** what Tally did with that name as a new ledger; no name after it is measured, and the coordinator decides whether a later row continues.
- **A name list not its own at step 5:** an import answered `CREATED` 1 still changed a ledger's name; recorded, and a finding for any writer as much as for #2480.

**If all ten are read as their own:** the static-variable type has evidence for these ten marks too, on one book and release. **If some are not, or are refused:** the type admits only the marks that were read, and the report names the rest.

**What these answers are for.** They inform the static-variable read's own type on #2480. They become fixtures only if the manifest admits them.

## Code admits the trigger
Lines are at master 812d39bb.
- **The build refuses these names before any send.** `LedgerScopeName::new` (`src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65`) refuses any character outside ASCII letters, digits and a single space (`:74`) and the words of `FORMULA_WORDS` (`:30`). `admitted_party` (`src-tauri/src/agent_party_bills.rs:26`) maps each refusal to `party_name_not_admitted`. So no build reaches this row's reads, and this row sends by curl.
- **The bills read is master's.** `fetch_party_bills` (`src-tauri/src/tally/runtime_party_bills.rs:57`) reads the snapshot and then the report, each sent twice, and ties them. `render_ledger_outstandings_request` (`src-tauri/crates/bridge-tally-protocol/src/native_outstandings/party_bills.rs:40`) takes the party as the text of the `LedgerName` static variable, escaped once as XML text.
- **The key `st.py` checks is master's collision key, read in Python:** `letters_and_digits_key` (`src-tauri/crates/bridge-tally-core/src/master_binding.rs:1982`), which the ledger-create path asks at admission (`admit_free_name`, `src-tauri/src/agent_master_post.rs:338`).

## Not in this row
- ComplyEaze Bridge's own transport, or any Bridge build;
- the `CHILDOF` formula read and `ledger_movement`'s ledger-scoped voucher read (a later row, if the coordinator queues one);
- names that share a stem with another ledger, and what Tally matches a created name on;
- a mark not in this table (`<`, `>`, `;`, `=`, `$`, `?`, `[`, `]`, `{`, `}`, `|`, `\`, `~`, `` ` `` and others), a mark beside another of this row's, a mark mixed with NS's marks, a double space, a space at an edge, a name of the longest admitted length, a name that differs from another only by case;
- a ledger keyed by hand, for a name the import refuses;
- a ledger with more than one bill, a settled or part-settled bill, a note or a voucher after the invoice;
- another to-date, book or release; a book with GST on; Educational mode.
