# Row 2480-CV: does the ledger-scoped census read one ledger when its name is a static variable and `CHILDOF` holds only `##LedgerName`?

Shape discovery, as rows 2346-PB, 2480-NM, 2480-NS, 2480-NC and 2480-NX: the requests are written for the lab, not requests the build sends, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2480 (0.9.0) and #485. Written 11 Nov 2026. Lines are at master 812d39bb. Confidence of everything here until the row runs: unverified.

## Why this row
Bridge reads one ledger's vouchers with a collection of `<TYPE>Vouchers : Ledger</TYPE>` and `<CHILDOF>name</CHILDOF>` (protocol reference §11c.6). `CHILDOF` takes a formula, not a quoted literal, so the name is placed inside a formula. `LedgerScopeName` therefore admits only ASCII letters, digits and single spaces, and no operator word (`ledger_scope.rs:65`). Every other name, `Ram & Sons` among them, falls back to the whole book's window. Sending names with other marks in that position cannot widen the type: there is no literal whose end a closed set of characters could guard (see also protocol reference §11d).

Rows 2480-NS, 2480-NC and 2480-NX read twenty marks as data: each name escaped once as XML text in the `LedgerName` static variable of `Ledger Outstandings`, each read as its own ledger. This row asks whether the ledger-scoped census can take its name the same way. The name goes in the static variable, and `CHILDOF` holds the fixed text `##LedgerName`. The formula Tally evaluates is then fixed in code, and no name enters it.

**What this row changes from NX:**
1. **The reads.** In place of NX's `Ledger Outstandings` reads (`L-lo-`), this row sends master's census requests (`C-`, below). The ledger census is master's `render_agent_ledger_census`, and the book-wide census is master's `render_agent_voucher_census`.
2. **The shape step first (P6).** Before any of the ten names is sent, the control alone is read three ways once its ledger and invoice exist: master's unquoted form; master's form with the static variable added; and this row's form. The three answers must be byte-equal and must hold the control's invoice. Otherwise the row stops there (step 5).
3. **The order of the set-up.** The control is set up alone (step 4) before the shape step. The ten are set up after it (step 7).
4. **NX's imports are sent again.** The names, invoices, numbers and `REMOTEID` tails are NX's, byte for byte. NX's restore removed them (its `s9` equal to its `s1` and `s2`). Step 2 checks again that the book holds none of them. The write gate passes only `CREATED` 1, so an import that met an existing key is the STOP of the set-up. Within this row, each import is still sent at most once.

Everything else is NX's: the reads first, the exact write gate (`st.py --import`), the refusal path (`rf.py`), the empty-answer exception, the restore, and every script NX ran.

## Question
1. **The shape (control only).** Once the control ledger and its one invoice exist, does the census with `<CHILDOF>##LedgerName</CHILDOF>` and the name in the `LedgerName` static variable give the same bytes as master's census with the name in `CHILDOF`? Does it hold that invoice and nothing else?
2. **Before a ledger exists,** what does that census answer for its name? §11c.6 lists a name that matches no ledger as not established.
3. **For each of the ten names, once its ledger exists with one invoice,** does that census hold that ledger's invoice and nothing else, twice byte-equal? Is that invoice the one the book-wide census shows was created for that ledger?

**The one variable** of the shape step is the request's form, with the control's name fixed. In step 9 it is the name, with the form fixed. The ledgers are alike but for the name, as in NX.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (byte-equal to NX's) applies in full. Where it and this sheet differ, the stricter one holds.

- **Pinned by GUID.** `count20.py`, `nm.py` and `st.py` each hold BRIDGE PILOT LAB's GUID as `BOOK`, and each STOPs on a row with another. (`rf.py` and `cv.py` read answers that carry no GUID of the company. The marks read beside each import, and the shape and ledger reads, are pinned by those marks reads.)
- **Its state:** ALTVCHID 68 and ALTMSTID 245, the marks NX's restore left (its `s9`, equal to its `s1`), if every row since has ended with its restore, as the coordinator reports. Other marks at s1: stop before BK, with nothing sent.
- **It holds none of the eleven ledgers and none of the numbers below.** Step 2 checks this, and a row there is a STOP. The ledgers are the eleven names of `names.tsv`, all beginning `BRIDGE Nx `. The invoice numbers are `BP/26-27/0261` to `BP/26-27/0271`, and the `REMOTEID` tails `000000000e61` to `000000000e6b`, as NX sent them.
- **Each import is sent at most once in this row,** and never again whatever happened to its first send.
- **NX's exception to C3 holds here, for the same reason.** Given with the run coordinator's word when the row is queued: an import that the write recipe's (e) finds refused shows no effect, and that absence is the result for its id. Every other write that a read does not show is C3's STOP.
- **The date and the period:** read the Gateway's current date and period at step 1. If the date is before 1-Aug-2026, or the period is not 1-Apr-26 to 31-Mar-27: stop before BK. **Do not change Tally's current date or period.**
- **BK is this row's way back.** It is taken at step 1, before anything is sent, and restored at step 11 by C6. The restore runs on every path from step 3 on that is not a STOP. After any STOP, at any step (C7, C8 or C9, a script's, a marks compare's, or one a step names), C8 holds, and the restore waits for the run coordinator's word.
- Write down the release shown on Tally's screen.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6).

**The eleven ledgers** (`names.tsv`: `id`, invoice number, name), NX's. Each is created by `LM-<id>.xml` and given its one open bill by `I-<id>.xml`, each sent at most once.

| id | number | name | its one mark |
| --- | --- | --- | --- |
| `pl` | 0261 | `BRIDGE Nx Plain` | none: the control, which `LedgerScopeName` admits |
| `cm` | 0262 | `BRIDGE Nx Ak, Am` | `,` |
| `us` | 0263 | `BRIDGE Nx Bk_Bm` | `_` |
| `dq` | 0264 | `BRIDGE Nx "Ck" Cm` | `"` as a pair round a word, sent escaped once as XML text (`&quot;`) |
| `ps` | 0265 | `BRIDGE Nx Dk + Dm` | `+` |
| `at` | 0266 | `BRIDGE Nx Ek @ Em` | `@` |
| `hs` | 0267 | `BRIDGE Nx Fk #12` | `#` |
| `cl` | 0268 | `BRIDGE Nx Gk: Gm` | `:` |
| `pc` | 0269 | `BRIDGE Nx Hk 15% Hm` | `%` |
| `st` | 0270 | `BRIDGE Nx Jk * Jm` | `*` |
| `ex` | 0271 | `BRIDGE Nx Kk!` | `!` |

**Where each name goes.** In `C-v-<id>.xml`, the name is the text of the `LedgerName` static variable, escaped once as XML text. It is the same text NX's `L-lo-<id>` request carried, byte for byte. `CHILDOF` holds `##LedgerName` and nothing else. In the imports, the name is carried as master's import shapes carry it. **The only request with a name inside `CHILDOF` is `C-u-pl.xml` (and `C-uv-pl.xml`), the control's,** whose name master's type already admits. None of the ten is ever sent inside a formula.

## The requests
All are census requests (`FETCH` `GUID,ALTERID,DATE`), BRIDGE PILOT LAB, the window 1-Apr-2026 to 1-Aug-2026, and the AlterID span (0, 8192], made by `gen-cv.py`:

| file | what it is |
| --- | --- |
| `C-book.xml` | master's book-wide census (`render_agent_voucher_census`) |
| `C-u-pl.xml` | master's ledger census (`render_agent_ledger_census`) of the control: `<CHILDOF>BRIDGE Nx Plain</CHILDOF>` |
| `C-uv-pl.xml` | `C-u-pl.xml` with `<LedgerName>BRIDGE Nx Plain</LedgerName>` after `SVCURRENTCOMPANY`, and nothing else changed |
| `C-v-<id>.xml` | `C-uv-pl.xml` with `<CHILDOF>##LedgerName</CHILDOF>` and the name of `<id>` in `LedgerName`, for each of the eleven |

## The sends and the wedge rule
`run.py` is NM's, byte for byte: one request at a time, each answer's headers kept by curl -D in `a-<label>-<file>.headers`, and it never abandons a request.
- **The wedge rule.** If any send has not answered **5 minutes** after it began (NM's bound): press nothing, do not end curl, send nothing more (not even `GET /status`), screenshot the whole Tally window, and report to the run coordinator (C9). Nothing is re-sent on this row's own judgement. On this book NX's reads each answered within about 3 s.
- **The stops.** `run.py`'s stops are failed requests, never "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 B. So are the STOPs of `count20.py`, `nm.py`, `st.py`, `rf.py` and `cv.py`. Each stops the row there, with two exceptions: the empty answer below, and the refused import of the write recipe.
- **The empty-answer exception for a `C-u-` or `C-v-` answer at steps 3, 6 and 9.** It holds only with the run coordinator's word given when the row is queued, as NX's did. Without that word, an empty answer is a STOP like any other.
  - A census of a name that matches no ledger has not been observed. It may be an empty collection, or no bytes, as `Ledger Outstandings` gave for every such name in NM, NS, NC and NX.
  - It is recorded first as what it is: `run.py` printed "empty answer", so the request failed.
  - It is not a STOP, and the step goes on, only if its headers file shows HTTP status 200 and `CONTENT-LENGTH: 0`. `run.py`'s "stop, do not resend" is then read as this exception.
  - Any other empty body (no headers file, no status line, another status or another length) ended without an answer: a STOP, and the request is never sent again.
  - Then, in this order: keep the headers file and the log line, and screenshot the whole Tally window; if a dialog is up: STOP; send `GET /status` once (C2c's command), recording its status and bytes, and if it is not 200: STOP; run `cv.py` on that answer, and go on with the step.
  - Each `GET /status` this sends is a request of the row: it is logged with the rest, and there may be up to 34 of them (2 at step 3, 10 at step 6, 22 at step 9).
  - An empty answer at step 5, to `C-book.xml` at any step, or to an import is a STOP, as `run.py` says.
- **What the branch and any public text say about a STOP of this kind.** If a send wedges (the wedge rule), Tally exits or stops answering, or a dialog the prompt rule names comes up, the row's RESULTS and any public comment on it say only "a STOP at step N, id X" (or "a STOP at step N" where no id applies: steps 1, 2, 8, 10, 11 and 12, and the marks reads `p3` and `p14`; a write's own marks reads take that write's id). The request that preceded it, the screens, the timings and anything else that would show how to cause it go to the run coordinator by the private channel, not to the branch. The coordinator decides what, if anything, is published later.
- Check each file's sha256 against `SHA256.txt` before its send. No file in `nx/` or `fx/` is ever an argument to `run.py`: they are sources for `gen-cv.py`.
- Close no Bridge process, and call no Bridge tool.

**Which script reads which answer**
- **`count20.py`** (NX's, byte-equal) reads `a-*-M-marks.xml` and `a-*-N-*`. Its `--before-write` and `--same-row` are the marks gate and the marks compare.
- **`nm.py`** (NX's, byte-equal; it takes the eleven names from this folder's `names.tsv`) reads `a-*-S-snap-*` and `a-*-B-bills.xml`, with `--same` for a pair.
- **`st.py`** (NX's, byte-equal) reads the import answers (`--import`) and the snapshot answers (`--before`, `--names`).
- **`rf.py`** (NX's, byte-equal) reads an import answer that `st.py --import` did not pass. Its rule is whole in its header.
- **`cv.py`** (new for this row) reads every `C-` answer: `ANSWER: NO BYTES`, `ANSWER: STATUS <s>` or `ANSWER: VOUCHERS <n>` with each voucher's GUID, AlterID and date. `--same` compares two answers' bytes. `--own` judges whether a ledger census holds exactly its own ledger's invoice, against the book-wide census before and after the set-up. Its rules are whole in its header.

## Files
**Byte-equal to NX's file of the same name** (`lab/2480-nx-sheet` at ea84bcdf, folder `lab/2480-nx/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py`, `M-marks.xml`, `B-bills.xml`, `S-snap-20260401-20260801.xml`, `names.tsv`, and the 33 imports and number reads `LM-<id>.xml`, `I-<id>.xml` and `N-<number>.xml`.

**NX's, kept in `nx/`, never sent:** NX's `SHA256.txt`, and its eleven `L-lo-<id>-20260801.xml`, the source of each escaped name.

**Master's, kept in `fx/`, never sent:** two census requests the lab sent, which master keeps as captures (lines at 812d39bb):
- `census-dn-party-year-span1.request.utf16le.xml`, master's ledger census, which `agent_voucher_window_tests.rs:8829` pins to `render_agent_ledger_census` byte for byte;
- `pilot-lab-audit-census-spanned.request.utf16le.xml`, the book-wide census the lab sent to this book for #2145.

**Written for this row:** `gen-cv.py`, `cv.py` and this file.

**Made by `gen-cv.py`** (14 files): the requests above. It first checks every source's sha256: the two captures, NX's `SHA256.txt`, and each of NX's files here against it. It then asserts:
- each capture, carved into a template with five holes (company, dates, span end, collection, scope), is given back byte for byte;
- the two captures give one template, and the ledger template with the book scope gives the book capture byte for byte;
- each escaped name, read back by an XML parser, is its `names.tsv` name;
- each request, read back by an XML parser, has one `SVCURRENTCOMPANY` (the book), its `CHILDOF` and `LedgerName` as the table says, and the collection type master gives;
- `C-uv-pl` differs from `C-u-pl` by the variable alone, `C-v-pl` from `C-uv-pl` by the `CHILDOF` text alone, and each `C-v-<id>` from `C-v-pl` by the escaped name alone.

`SHA256.txt` lists every file with its size, in NX's form (`sha256  size  path`, which `shasum -c` does not read). Requests are UTF-16LE with a BOM, as sent.

**Tried before this sheet was sent:**
- `gen-cv.py`: its 14 files, each check passing.
- `cv.py`:
  - on master's captured census answers (`ledger-scoped-vouchers/`): `VOUCHERS 82` on `census-dn-sales-day-span1`, and `VOUCHERS 0` on the empty collection `census-dn-sales-day-span2`;
  - on answer shapes written by hand for the test, not kept:
    - `NO BYTES` on a 0-byte file;
    - `STATUS 0` with its `LINEERROR`;
    - a STOP on a missing file, on text that is not XML, on two collections, on a non-voucher element, on a voucher with no AlterID, on a `LINEERROR` in an answer whose `STATUS` is 1, and on a GUID listed twice;
    - `--same` `EQUAL` and `DIFFER`;
    - `--own`:
      - `YES` on the id's own voucher;
      - `NO` on another id's voucher, on two vouchers, on no bytes and on a `STATUS 0`;
      - `NOT JUDGED` when a voucher of the before census is missing after, when the new vouchers do not match the created invoices in number, and on an after census with no bytes;
      - a STOP when the created list is out of order or lacks the id.
- **Not tried:** any send to Tally.

## Steps
Run each label as `python3 -I run.py http://127.0.0.1:9001 <label> <files>`, the Gateway on 9001 as C2 finds it (written `URL` below). (The lab PC has no `python3` command; there, `python -I` with Python 3 is the same command, as NX ran it.) The prompt rule, the wait rule and the path rule are NM's.

**Labels are not step numbers.**

| step | labels |
| --- | --- |
| 1 | `s1` |
| 2 | `s2` |
| 3 | `p1`, `p2`, then `p3` |
| 4 | `s4pl`, `s5pl`, with `m` after each for its marks read, and `x` for the marks read after a refusal |
| 5 | `c0` to `c5` |
| 6 | `p4` to `p13`, then `p14` |
| 7 | `s4<id>`, `s5<id>`, as at step 4 |
| 8 | `s6…` |
| 9 | `r<n>a`, `r<n>b` |
| 10 | `m1` |
| 11 | `s9` |
| repeats | `p3r`, `p14r`, `m1r`: a marks read sent once more after MARKS DIFFER, as steps 3, 6 and 10 say |

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named. For any other prompt, message or dialog:
- copy it word for word and screenshot it first;
- then answer No, or Esc if it offers no No;
- press nothing more and send nothing.

That prompt is this row's result, and C8 holds: the restore waits for the run coordinator's word. What the branch says about it is the STOP rule above.

**Each write, the same way, by hand, one label a command** (NX's write recipe, used only in steps 4 and 7):
- (a) `run.py URL <label>m M-marks.xml`, then `python3 -I count20.py --before-write a-s1-M-marks.xml a-<label>m-M-marks.xml`.
- (b) Only if that passes, and no key was pressed in Tally since: `run.py URL <label> <file>`, the import alone.
- (c) Screenshot the whole window before the next request (C3). A dialog: the prompt rule.
- (d) `python3 -I st.py --import a-<label>-<file>`, recorded whole. **On exit 0 (`CREATED ONE`), go on with the next write.**
- (e) **Only if `st.py` gave exit 1 and `run.py` in (b) gave exit 0:** `python3 -I rf.py a-<label>-<file>`, recorded whole.
  - On exit 1 (`NOT A REFUSAL`, or a STOP of `rf.py`): the STOP of the set-up.
  - On exit 0 (`REFUSED`): `run.py URL <label>x M-marks.xml`, then `python3 -I count20.py --same-row a-<label>m-M-marks.xml a-<label>x-M-marks.xml`. On SAME MARKS, that import is **refused** for its id: record it, and skip the rest of that id. On anything else: the STOP of the set-up, with nothing more sent.
- On any exit other than 0 of `run.py` in (b), (e) is not run: the STOP of the set-up.

The set-up is not run as a loop. If a script sends any of it, the script ends at the first exit other than 0 of `run.py`, `count20.py` or `rf.py`, and at the first exit 1 of `st.py` that (e) does not turn into a refusal, and sends nothing after it. Nothing else decides whether an import answer is as expected.

**The STOP of the set-up** (steps 4 and 7) is one of these, on any answer, a ledger's or an invoice's:
- an exit 1 of `st.py --import` that (e) does not turn into a refusal;
- a refusal whose marks after it differ from those before it;
- an exit other than 0 of `run.py` on an `LM-` or `I-` file;
- a dialog after an import;
- at step 4, any refusal: without the control, no other name can be judged.

At a STOP: send nothing more; screenshot the whole window; record the answer, if there is one, whole; and report to the run coordinator. C8 holds. Say in the report what the book then holds. For a dialog, or a send with no answer, the branch says only what the STOP rule of "The sends" allows.

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down (Book).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2480-cv\backup-1` (Tally may show it as `C:\lab\2480-Cv\backup-1`). NX's step 1 is the path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0261.xml N-0262.xml N-0263.xml N-0264.xml N-0265.xml N-0266.xml N-0267.xml N-0268.xml N-0269.xml N-0270.xml N-0271.xml B-bills.xml S-snap-20260401-20260801.xml C-book.xml M-marks.xml`.
   - EXPECT each N read 0 rows (`count20.py`), `nm.py` on B-bills no group for any of the eleven names, and on the snapshot "no row" for each. A row or a group: stop before step 3, each recorded whole.
   - `st.py --before a-s2-S-snap-20260401-20260801.xml`: EXPECT `FOLD: CLEAR`. A STOP: stop before step 3, recorded whole.
   - `cv.py a-s2-C-book.xml`, recorded whole. EXPECT `ANSWER: VOUCHERS <n>`, which step 5 and step 9 take as the book before this row's invoices (the N reads above show none of them is in the book). Anything else (no bytes, a `STATUS`, a STOP of `cv.py`): stop before step 3, recorded whole. The book-wide census is a request this book has answered (#2145, with a narrower window and span), so this read is not the row's question.
3. **Reads first: the control before its ledger exists.**
   - `p1`: `C-u-pl.xml`, then screenshot the whole Tally window at once. A dialog: the prompt rule.
   - `p2`: `C-v-pl.xml`, then screenshot the whole Tally window at once. A dialog: the prompt rule. **`p2` is the first send of the `##LedgerName` form, and it carries the control's name, before any of the ten is sent.**
   - `cv.py` on each, recorded whole, and `cv.py --same a-p1-C-u-pl.xml a-p2-C-v-pl.xml`, recorded. EXPECT `ANSWER: VOUCHERS 0` or `ANSWER: NO BYTES` (under the exception) for each. Either kind is "no voucher": if the two differ in kind or bytes, that is recorded as a result and the step goes on.
   - **Anything else stops the row after `p3`,** before the control's ledger is sent: a voucher in either answer, a `STATUS`, or a STOP of `cv.py`. Record it whole. The control can then not be judged, so no other name can be.
   - `p3`: `M-marks.xml`, and `count20.py --same-row a-s1-M-marks.xml a-p3-M-marks.xml`. EXPECT SAME MARKS. On MARKS DIFFER: send `M-marks.xml` once more as `p3r` and record it; send nothing else, and STOP.
4. **The control's set-up.** `s4pl` (`LM-pl.xml`), then `s5pl` (`I-pl.xml`), each write as above, with `s4plm` and `s5plm` before them. A refusal of either is the STOP of the set-up.
5. **The shape step, the control alone, before any of the ten.** All of it is one control's reads; none of the ten is sent until it passes.
   - `c0`: `M-marks.xml`.
   - `c1`: `C-u-pl.xml`; `c2`: `C-uv-pl.xml`; `c3`: `C-v-pl.xml`. Screenshot the whole Tally window after each, at once, before the next request. A dialog: the prompt rule.
   - `c4`: `C-book.xml`.
   - `c5`: `M-marks.xml`.
   - Then, each recorded whole:
     - `cv.py` on `a-c1-…`, `a-c2-…` and `a-c3-…`;
     - `cv.py --same a-c1-C-u-pl.xml a-c3-C-v-pl.xml`;
     - `cv.py --same a-c1-C-u-pl.xml a-c2-C-uv-pl.xml`;
     - `cv.py --own pl a-c1-C-u-pl.xml a-s2-C-book.xml a-c4-C-book.xml pl`;
     - `count20.py --same-row a-c0-M-marks.xml a-c5-M-marks.xml`.
   - **The shape step passes only if all of these hold:**
     - `a-c1` is `ANSWER: VOUCHERS 1`, and `--own` gives `OWN: YES` (master's form reads the control's own invoice);
     - `a-c3` is byte-equal to `a-c1` (`SAME: EQUAL`), so the `##LedgerName` form gave master's bytes;
     - `a-c2` is byte-equal to `a-c1`, so the added variable changed nothing in master's form;
     - SAME MARKS from `c0` to `c5`;
     - every send answered, with no dialog.
   - **Anything else is the STOP of the shape step, before the ten:** a difference, a `STATUS`, no bytes, `OWN` other than YES, a STOP of a script, a dialog, or a send with no answer. Nothing more is sent: not the ten names, not their ledgers, not a marks read beyond C8's. Record each answer whole, screenshot the window, and report to the run coordinator. C8 holds, and the restore waits for the coordinator's word. A dialog or a send with no answer is, in public, only "a STOP at step 5, id pl".
6. **Reads first: the ten before their ledgers exist. Only after step 5 has passed.** (`p4` follows `c5`, not `p3`.) For each id but `pl` in the order of the table, `p<n>`: `C-v-<id>.xml`, sent once (`p4` for `cm` to `p13` for `ex`).
   - Screenshot the whole Tally window after each, at once, before the next request. A dialog: the prompt rule.
   - `cv.py a-p<n>-C-v-<id>.xml` and `cv.py --same a-p2-C-v-pl.xml a-p<n>-C-v-<id>.xml`, each recorded whole. EXPECT `ANSWER: VOUCHERS 0` or `ANSWER: NO BYTES`. Either kind is "no voucher". A kind or bytes other than `p2`'s is recorded as that name's result, and the step goes on. `SAME: EQUAL` here says only that the two answers are alike, nothing about a ledger.
   - **An answer with a voucher** is that name's result, the most important kind this row can give. Tally scoped the census to some other ledger, or to none. Record it whole, with `cv.py --own pl a-p<n>-C-v-<id>.xml a-s2-C-book.xml a-c4-C-book.xml pl`, which names whether the voucher is the control's. That id's ledger and invoice are never sent, and step 7 skips it. The row goes on with the next id: a voucher for one name does not stop the reads of the others, since each name is sent alone and changes nothing.
   - **A `STATUS` answer** is that name's result too: record it whole. The id is still set up at step 7 and read at step 9.
   - `p14`: `M-marks.xml`, and `count20.py --same-row a-c5-M-marks.xml a-p14-M-marks.xml`. EXPECT SAME MARKS. On MARKS DIFFER: send `M-marks.xml` once more as `p14r` and record it; send nothing else, and STOP.
7. **The set-up of the ten,** one ledger at a time in the order of the table, each id's ledger and then its invoice, each write as above (`s4<id>`, `s5<id>`, with `m` and `x` reads). Skip an id step 6 answered with a voucher.
   - **A refused ledger** (`s4<id>` refused): its invoice is never sent, and that id is not read at step 9. The next id starts at (a).
   - **A refused invoice** (`s5<id>` refused): the ledger exists with no invoice. That id is not read at step 9, and step 8 records the ledger as it stands.
   - **Why the marks read after a refusal is also the check of the book.** `count20.py --same-row` compares the book's GUID, ALTVCHID and ALTMSTID. If the book was no longer loaded, or another company was, that read does not pass, so an import that failed for that reason is a STOP, not a refusal.
8. **The book-wide reads and the state.** `s6m`: `M-marks.xml`. Then `s6sa`, `s6sb`: `S-snap-20260401-20260801.xml`, the same file twice; `s6ba`, `s6bb`: `B-bills.xml`, the same file twice; and `s6ka`, `s6kb`: `C-book.xml`, the same file twice.
   - Each recorded whole:
     - `nm.py` on `a-s6sa-S-snap-20260401-20260801.xml` and on `a-s6ba-B-bills.xml`;
     - `nm.py --same` on the two snapshot and the two bills answers;
     - `st.py --names a-s6sa-S-snap-20260401-20260801.xml`;
     - `cv.py a-s6ka-C-book.xml`;
     - `cv.py --same a-s6ka-C-book.xml a-s6kb-C-book.xml`.
   - **It is the intended state** if all of these hold:
     - S holds a row for each ledger created and none for a refused one, each `ISBILLWISEON` Yes;
     - B holds one group for each id whose invoice was created, `BP/26-27/<its number>`;
     - the book-wide census holds every voucher of `a-s2-C-book.xml` and one new voucher, dated 20260801, for each created invoice.

     If not, step 9 is still sent for the ledgers that exist with an invoice, and the report says the state is not the intended one. That holds for answers the scripts read. A STOP of a script at this step (an answer it cannot read) is a STOP like any other, and step 9 is not sent.
   - **CREATED**, for step 9, is the list of ids whose `LM-` and `I-` imports both gave `CREATED ONE`, in table order, `pl` first, comma-separated. Write it down here.
9. **The ledger reads.** For each id of CREATED, in the order of the table, `r<n>a` and `r<n>b`: `C-v-<id>.xml` twice, the file steps 3 and 6 sent.

   | n | id | labels | n | id | labels |
   | --- | --- | --- | --- | --- | --- |
   | 1 | `pl` | `r1a`, `r1b` | 7 | `hs` | `r7a`, `r7b` |
   | 2 | `cm` | `r2a`, `r2b` | 8 | `cl` | `r8a`, `r8b` |
   | 3 | `us` | `r3a`, `r3b` | 9 | `pc` | `r9a`, `r9b` |
   | 4 | `dq` | `r4a`, `r4b` | 10 | `st` | `r10a`, `r10b` |
   | 5 | `ps` | `r5a`, `r5b` | 11 | `ex` | `r11a`, `r11b` |
   | 6 | `at` | `r6a`, `r6b` | | | |

   - Screenshot the whole Tally window after each `r<n>a`, before the next request. A dialog: the prompt rule.
   - Then, for each id, recorded whole:
     - `cv.py a-r<n>a-C-v-<id>.xml`;
     - `cv.py --same a-r<n>a-C-v-<id>.xml a-r<n>b-C-v-<id>.xml`;
     - `cv.py --own <id> a-r<n>a-C-v-<id>.xml a-s2-C-book.xml a-s6ka-C-book.xml <CREATED>`.
   - A ledger's answer that `cv.py` reads never stops the row, whatever it holds: each is a result, recorded in full. A STOP of `cv.py` (an answer it cannot read), a dialog and a send with no answer are STOPs, as everywhere.
10. **`m1`:** `M-marks.xml`, then `count20.py --same-row a-s6m-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS: steps 8 and 9 only read. On MARKS DIFFER: send `M-marks.xml` once more as `m1r` and record it; send nothing else, and STOP. The answers already recorded stand.
11. **Restore** BK by C6, with `restore-plan.txt` written first. NX's step 8 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace). List the data folder before and after, as NX did.
    - Then `s9`: every file of s2, in s2's order, and compare whole:
      - `count20.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml` passes;
      - each N read 0 rows;
      - B-bills holds no group of the eleven names;
      - the snapshot shows "no row" for each;
      - each answer's sha256 equals s2's, `C-book.xml`'s included.
    - **Any of these not equal stops the row:** send nothing more, and report before the close. The coordinator decides.
12. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering. Screenshot the window, send it and the answers to the run coordinator, and ask before anything else is pressed or sent. Say in the report to the coordinator what the book then holds. What the branch says about it is the STOP rule of "The sends": only "a STOP at step N", with the id where one applies.

## Record
- Each answer's bytes and sha256, its headers file whole, and each script's lines, whole.
- `run.py`'s log for each label, and how long each census took, except for the send of a STOP the STOP rule names.
- Every `--before-write`, `--same-row`, `--import`, `--before`, `--names`, `--same` and `--own` output, and every `cv.py` and `rf.py` output, whole.
- The window after each import, and after each shot named in steps 3, 5, 6 and 9, listed by name, bytes and sha256.
- Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- **For a STOP the STOP rule of "The sends" names** (a wedge, an exit of Tally, a dialog), none of that send's detail goes on the branch: not its screenshots, headers file, log line, timing, or the request that preceded it. All of that goes to the run coordinator by the private channel, and the branch says only "a STOP at step N, id X".
- The BK file and the restore, as C5 and C6 ask, with the data folder listed before and after.
- The release on Tally's screen, and the Gateway's date and period.
- CREATED, as step 8 writes it.
- For each ledger:
  - its step 3 or step 6 answer;
  - whether Tally created it, refused it, or refused its invoice, with the `LINEERROR` text if any;
  - its name list at step 8;
  - its step 9 pair and `--own` result.

**Staging the evidence:** the check for 10 or more digits sets aside this book's registered GUID matched whole, and nothing else. Any other hit waits for the run coordinator.

## Expected
Estimates, not predictions.
- **s2:** as the Book and step 2 say; `FOLD: CLEAR`; the book-wide census answered with this book's vouchers.
- **Step 3:** no voucher for either form. Whether a census of a name that matches no ledger is an empty collection or no bytes has not been observed (§11c.6).
- **Step 4:** `CREATED ONE` for both, as in NX.
- **Step 5:**
  - `c1` holds the control's invoice alone (§11c.6 read two plain names this way on another book);
  - `c2` equal to `c1`;
  - `c3` equal to `c1` only if Tally evaluates `##LedgerName` in a collection's `CHILDOF` from the request's static variables. That has not been observed, so neither outcome is predicted.
- **Steps 6 and 9:** as the control, if the name in the variable acts as it did for `Ledger Outstandings` in NX. No stronger estimate.
- **Step 7:** `CREATED ONE` for each, as in NX.
- **s9:** equal to s2 and s1.

## Decides
Each is PARTIAL: one book, one release, one sitting, master's request templates sent by curl, without the reads Bridge sends around them, and without the build's own parse.

**The shape step:**
- **It passes:** on this book and release, `<CHILDOF>##LedgerName</CHILDOF>` with the name in the `LedgerName` static variable gave master's bytes for the control. The ten are then measured.
- **It does not pass:** the `##LedgerName` form is not master's census on this book and release, and nothing in this row measures the ten through it. Record what differed. The coordinator decides whether a later row measures it another way (for example a variable the request declares in its own TDL). `LedgerScopeName` stays as it is.

**For each of the ten names,** one of these:
- **A voucher at step 6:** Tally scoped the census, before the ledger existed, to some other ledger's vouchers. That form is not admitted for that mark. This is the most important result.
- **Refused at its ledger or at its invoice:** as in NX. NX created all ten, so a refusal here would differ from NX, and the coordinator decides what it means.
- **`OWN: YES`, both sends equal, and no voucher at step 6:** on this book and release, the census with the name in the static variable reads that ledger alone. With NS, NC and NX, that is evidence for a ledger-scoped read that takes its name as data, in place of `LedgerScopeName`.
- **`OWN: NO` or a `STATUS` at step 9:** the form does not read that name as its ledger, so that mark is not admitted to it.
- **`SAME: DIFFER` at step 9:** record both; master would not plan on an answer that changes between two sends.

**If the shape step passes and all ten are `OWN: YES`:** the ledger-scoped census can take its name as data, for these ten marks and the control, on one book and release. A later change could then let `ledger_movement` and the `outstandings` party detail reach the scoped read for every name the static-variable type admits. The movement read (`render_agent_ledger_vouchers`) differs from the census in its `FETCH` list. This row does not send it, and a later row would.

**What these answers are for.** They inform #485's scoped read and #2480's type. They become fixtures only if the manifest admits them.

## Code admits the trigger
Lines are at master 812d39bb.
- **The build never sends the `##LedgerName` form.** `VoucherScope::collection` (`src-tauri/src/agent_read_profiles.rs:279`) writes the name itself into `CHILDOF`, escaped once (`:286`). It takes only a `LedgerScopeName`. `render_agent_ledger_census` (`:258`) and `render_agent_ledger_vouchers` (`:367`) are its two users. So this row's `C-v-` requests are lab requests, sent by curl.
- **The build refuses the ten names before any scoped send.** `LedgerScopeName::new` (`src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65`) refuses any character outside ASCII letters, digits and a single space (`:74`) and the words of `FORMULA_WORDS` (`:30`). `scope_name_of` (`src-tauri/src/agent_movement.rs:99`) and the party detail's admission (`src-tauri/src/agent_party_bills.rs:27`) map a refusal to the whole book's window.
- **The variable is the one master already fills for a party.** `render_ledger_outstandings_request` (`src-tauri/crates/bridge-tally-protocol/src/native_outstandings/party_bills.rs:40`) sends the party as the text of the `LedgerName` static variable (`:49`). Rows NS, NC and NX read twenty marks through it.

## Not in this row
- ComplyEaze Bridge's own transport, or any Bridge build;
- the movement read's and the window read's `FETCH` lists, ledger-scoped, by `##LedgerName` (a later row, if this one passes);
- any of the ten names inside `CHILDOF` itself (the formula position), which this row never sends;
- NS's and NC's names, and marks mixed in one name;
- names that share a stem with another ledger;
- a ledger with more than one voucher, or a voucher with two lines on the ledger;
- a census over more than one span, or a book of real size;
- another to-date, book or release; a book with GST on; Educational mode.
