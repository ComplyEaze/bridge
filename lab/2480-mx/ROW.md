# Row 2480-MX: does Tally read a ledger whose name mixes measured marks, and do the two data-carrying ledger-scoped reads take a name as a static variable?

Shape discovery, as rows 2346-PB, 2480-NM, 2480-NS, 2480-NC, 2480-NX and 2480-CV: the requests are written for the lab, not requests the build sends, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2480 (0.9.0) and #485. Written 11 Nov 2026. Lines are at master 812d39bb. Confidence of everything here until the row runs: unverified.

The row has two parts in one sitting, on one book, judged apart:
- **Part A** is row 2480-NX's method with new names. Each name mixes marks that earlier rows read one a name (the `Gupta, Verma & Co` shape among them). It reads them through `Ledger Outstandings`.
- **Part B** sends the movement read and the window read of one ledger (`render_agent_ledger_vouchers`), with the name as the `LedgerName` static variable and `CHILDOF` holding only `##LedgerName`, on part A's ledgers. It compares each answer, voucher by voucher, with the book-wide window read of the same day.

**Part B starts only after part A has passed in full** (step 8). Its STOP is its own, and part A's answers stand whatever part B gives.

## Why this row
**Part A.** Rows 2480-NS, 2480-NC and 2480-NX read twenty marks as data, each name escaped once as XML text in the `LedgerName` static variable of `Ledger Outstandings`, each read as its own ledger. NS's marks were `-`, `.`, `&`, `/`, `(`, `)`, `'` and two more; NC mixed them; NX read ten more, one a name: `,` `_` `"` `+` `@` `#` `:` `%` `*` `!`. No row has sent a name that mixes NX's marks with each other or with NS's. Most real party names are such a mix (`Gupta, Verma & Co`). A static-variable type that admits a name only if each of its marks was measured alone would rest on an assumption about mixes; this part measures ten mixes and one name that holds all seventeen marks.

**Part B.** Row 2480-CV showed, on this book and release, that the ledger-scoped census reads one ledger when its name is the `LedgerName` static variable and `CHILDOF` holds only `##LedgerName`. The census only plans a read. The reads that carry the data are the movement read (`ledger_movement`) and the window read (the `outstandings` party detail). Both are master's `render_agent_ledger_vouchers`, which differs from the census in its `FETCH` list and its collection name. Neither has been sent in this form. Until they have, the change the CV comment describes on #2480 cannot be built. `LedgerScopeName` admits only letters, digits and single spaces there, and every other name falls back to the whole book's window (protocol reference §11c.6; see also §11d).

**What this row changes from NX:**
1. **The names.** Each name but the control mixes two or more of those seventeen marks. One name, `al`, holds all seventeen. Each name has a stem of its own: no two names of this row share a key, and none shares a key with one of NM's, NS's, NC's or NX's names. `gen-mx.py` asserts all of it.
2. **The numbers and tails.** The invoice numbers are `BP/26-27/0281` to `0291`, and the `REMOTEID` tails `0e81` to `0e8b`.
3. **The ids** are this row's own (table below).
4. **Part B** (steps 8 and 9) is new.

Everything else in part A is NX's: the reads first, the exact write gate (`st.py --import`), the refused-import rule (`rf.py` and the marks compare), the empty-answer exception, the restore, and every script NX ran.

## Question
**Part A**, for each of the ten names, one name a ledger:
1. **Before the ledger exists, what does the report answer for that name?** No bytes is expected. Any bill is another ledger's.
2. **Does Tally create the ledger and its bill from master's import shapes, or refuse them?**
3. **Once it exists with one bill, does the report answer with that ledger's own bill?** Master's own request, with only the name changed, sent twice.
4. **Does that answer agree with the other two reads of the book?** The ledger snapshot's closing for the ledger, and the book-wide Bills Receivable group for it.

**Part B**, on the ledgers part A created:
5. **The shape (the control only).** Does each of the two reads, with `<CHILDOF>##LedgerName</CHILDOF>` and the name in the `LedgerName` static variable, give the same bytes as master's read with the name in `CHILDOF`? Does master's read hold exactly the vouchers of the book-wide read that carry the control's ledger?
6. **For each of the ten names,** does each read in that form hold exactly the vouchers of the book-wide read that carry that ledger, each the same in its own fields and its line on the ledger, twice byte-equal?

**The one variable** in part A is the name. In part B's shape step it is the request's form, with the control's name fixed; in step 9 it is the name, with the form fixed.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (byte-equal to NX's) applies in full. Where it and this sheet differ, the stricter one holds.

- **Pinned by GUID.** `count20.py`, `nm.py`, `st.py` and `wx.py` each hold BRIDGE PILOT LAB's GUID as `BOOK`, and each STOPs on a row with another (`wx.py` on a voucher GUID that does not begin with it). (`rf.py` reads import answers, which carry no GUID; the marks read beside each import is pinned.)
- **Its state:** ALTVCHID 68 and ALTMSTID 245, the marks CV's restore left (its `s9`, equal to its `s1`), if every row since has ended with its restore, as the coordinator reports. Other marks at s1: stop before BK, with nothing sent.
- **It holds none of the eleven ledgers and none of the numbers below.** Step 2 checks this, and a row there is a STOP.
  - The ledgers are the eleven names of `names.tsv`, all beginning `BRIDGE Mx `.
  - The invoice numbers are `BP/26-27/0281` to `BP/26-27/0291` (NX's and CV's were 0261 to 0271, NC's 0241 to 0251, NS's 0221 to 0231, NM's 0201 to 0211).
  - The `REMOTEID` tails are `000000000e81` to `000000000e8b` (NX's and CV's were `e61` to `e6b`, NC's `e41` to `e4b`, NS's `e21` to `e2b`, NM's `e01` to `e0b`; PB's, TD's and RS's are `777`, `795`, `d01` and `d10` to `d15`).
- **Each import is new and sent once.** Each of this row's 22 imports is a new file, and none carries a key an earlier row sent. Each is sent at most once, and never again whatever happened to its first send.
- **NX's exception to C3 holds here, for the same reason.** It holds only with the run coordinator's word, given when the row is queued. An import that step 4 (e) finds refused shows no effect, by design. That absence is the result for its id, and the row goes on. The exception covers that import alone. Every other write that a read does not show is C3's STOP.
- **The date and the period:** read the Gateway's current date and period at step 1. If the date is before 1-Aug-2026, or the period is not 1-Apr-26 to 31-Mar-27: stop before BK. **Do not change Tally's current date or period.**
- **BK is this row's way back.** It is taken at step 1, before anything is sent, and restored at step 10 by C6. The restore runs on every path from step 3 on that is not a STOP: after step 9, or after step 8 when part A did not pass in full. After any STOP, at any step, of part A or part B (C7, C8 or C9, a script's, a marks compare's, or one a step names), C8 holds, and the restore waits for the run coordinator's word.
- Write down the release shown on Tally's screen.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6).

**The eleven ledgers** (`names.tsv`: `id`, invoice number, name). Each is created by `LM-<id>.xml` and given its one open bill by `I-<id>.xml`, each sent at most once.

| n | id | number | name | its marks |
| --- | --- | --- | --- | --- |
| 1 | `pl` | 0281 | `BRIDGE Mx Plain` | none: the control, letters and single spaces, which `LedgerScopeName` admits |
| 2 | `ga` | 0282 | `BRIDGE Mx Ak, Am & Co` | `,` and `&`: the `Gupta, Verma & Co` shape |
| 3 | `ss` | 0283 | `BRIDGE Mx Bk & Sons (Bm), Bn` | `&`, a bracket pair and `,` |
| 4 | `ms` | 0284 | `BRIDGE Mx M/s. Ck_Cm, Cn` | `/`, `.`, `_` and `,` |
| 5 | `qd` | 0285 | `BRIDGE Mx "Dk" Dm & Dn` | `"` as a pair round a word, and `&` |
| 6 | `pp` | 0286 | `BRIDGE Mx Ek 50% + Em` | `%` after digits, and `+` |
| 7 | `hd` | 0287 | `BRIDGE Mx Fk #2, Fm-Fn` | `#` before a digit, `,` and `-` |
| 8 | `ad` | 0288 | `BRIDGE Mx Gk @ Gm.` | `@`, and `.` at the end |
| 9 | `ca` | 0289 | `BRIDGE Mx Hk: Hm's (Hn)` | `:`, `'` and a bracket pair |
| 10 | `xs` | 0290 | `BRIDGE Mx Jk! Jm *` | `!` and `*` at the end |
| 11 | `al` | 0291 | `BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly's` | all seventeen |

The names are invented. Each stem's letters are its own, so no name is another's with a mark turned or dropped. No name holds a double space or a space at an edge. Every mark reaches Tally only as escaped XML text: in the report and in part B's `Vm-v-` and `Vw-v-` requests, as the text of the `LedgerName` static variable; in the imports, as master's import shapes carry a name. **None of the ten is ever sent inside a formula.** The only requests with a name inside `CHILDOF` are `Vm-u-pl.xml`, `Vm-uv-pl.xml`, `Vw-u-pl.xml` and `Vw-uv-pl.xml`, the control's, whose name master's type already admits.

## Part B's requests
All are master's voucher reads, BRIDGE PILOT LAB, the one day 1-Aug-2026 (`SVFROMDATE` and `SVTODATE` 20260801, the formula's two dates the same), made by `gen-wx.py`. **The window** starts on day 1 of August, holds 1-Aug-2026, the day of every invoice of this row, and holds no day outside that month. It is one day, not the month, because the book-wide read of the month would pass the read bound: by master's own capture of this book (`agent/audit-read-pilot-lab/pilot-lab-audit-vouchers.utf16le.xml`, 15 Oct 2026, at the marks 68 and 245), its vouchers dated in August come to more than 2,000,000 B before this row adds any.

| file | what it is |
| --- | --- |
| `Vb-book.xml` | master's book-wide window read (`render_agent_vouchers`, field list `AGENT_VOUCHER_FETCH`): the request the lab sent this book for item 697-S0A, with its window alone changed. The ground truth part B compares with. |
| `Vm-u-pl.xml` | master's movement read of the control (`render_agent_ledger_vouchers`, `LedgerVoucherFields::Movement`): `<CHILDOF>BRIDGE Mx Plain</CHILDOF>` |
| `Vm-uv-pl.xml` | `Vm-u-pl.xml` with `<LedgerName>BRIDGE Mx Plain</LedgerName>` after `SVCURRENTCOMPANY`, and nothing else changed |
| `Vm-v-<id>.xml` | `Vm-uv-pl.xml` with `<CHILDOF>##LedgerName</CHILDOF>` and the name of `<id>` in `LedgerName`, for each of the eleven |
| `Vw-u-pl.xml`, `Vw-uv-pl.xml`, `Vw-v-<id>.xml` | the same three forms of the window read (`LedgerVoucherFields::Window`, the `outstandings` party detail's), which differ from the `Vm-` file of the same name in the `FETCH` element alone |

**Its size, estimated from that capture.** The book's eleven vouchers of 1-Aug-2026 come to 806,774 B of answer in that capture. `BP/26-27/0010`, the voucher NX's invoice template copies, comes to 90,564 B. So `Vb-book.xml`'s answer at step 8 is estimated at about 1,800,000 B (806,774 B and eleven times 90,564 B), under the bound. This is an estimate, not a measurement. An answer above 2,000,000 B is `run.py`'s stop: kept and not used, it is the STOP of step 8, and part B sends nothing more.

## The sends and the wedge rule
`run.py` is NM's, byte for byte: one request at a time, each answer's headers kept by curl -D in `a-<label>-<file>.headers`, and it never abandons a request.
- **The wedge rule.** If any send has not answered **5 minutes** after it began (NM's bound): press nothing, do not end curl, send nothing more (not even `GET /status`), screenshot the whole Tally window, and report to the run coordinator (C9). Nothing is re-sent on this row's own judgement. On this book NX's report reads answered within about 1 s, and CV's census reads with a capture within 15.1 s.
- **The stops.** `run.py`'s stops are failed requests, never "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 B. So are the STOPs of `count20.py`, `nm.py`, `st.py`, `rf.py` and `wx.py`. Each stops the row there, with three exceptions: the empty `L-lo-` answer below, the empty part B answer below, and the refused import of step 4.
- **The empty-answer exception for an `L-lo-` request (part A).** NX's, unchanged. It holds only with the run coordinator's word given when the row is queued. Without that word, an empty `L-lo-` answer is a STOP like any other.
  - A name that matches no ledger is answered with no bytes (row 2377-L, and all eleven of NS's, NC's and NX's step 3). At step 3 that is the expected answer for every name.
  - It is recorded first as what it is: `run.py` printed "empty answer", so the request failed.
  - It is not a STOP, and the step goes on, only if its headers file shows HTTP status 200 and `CONTENT-LENGTH: 0`. `run.py`'s "stop, do not resend" is then read as this exception.
  - Any other empty body (no headers file, no status line, another status or another length) ended without an answer: a STOP, and the request is never sent again.
  - Then, in this order: keep the headers file and the log line, and screenshot the whole Tally window; if a dialog is up: STOP; send `GET /status` once (C2c's command), recording its status and bytes, and if it is not 200: STOP; run `nm.py` on that answer, and go on with the step.
  - Each `GET /status` this sends is a request of the row: it is logged with the rest, and there may be up to 33 of them (11 at step 3, 22 at step 6).
  - At step 6, an empty `r<n>a` under this exception is followed by its `r<n>b`, as for any other answer.
- **The empty-answer exception for a `Vm-v-` or `Vw-v-` answer at step 9 (part B).** The same rule, with the same condition: it holds only with the run coordinator's word given when the row is queued, and without it such an answer is part B's STOP. Every name read there has a ledger, so an empty answer is not expected; under the exception it is that id's result, recorded as `wx.py` reads it (`EMPTY ANSWER`), and the step goes on. `wx.py` tells it apart from a failed send by the headers file, as above, and a failed send is a STOP. The `GET /status` after each is part B's; there may be up to 44 of them. An empty answer at step 8, to `Vb-book.xml` at any step, or to any other request of either part, an `LM-` or `I-` import included, is a STOP, as `run.py` says.
- **What the branch and any public text say about a STOP of this kind.** If a send wedges (the wedge rule), Tally exits or stops answering, or a dialog the prompt rule names comes up, the row's RESULTS and any public comment on it say only "a STOP at step N, id X" (or "a STOP at step N" where no id applies: steps 1, 2, 5, 7, 10 and 11, the marks reads `p12`, `k0`, `k8` and `k10`, and the book-wide reads `k7` and `k9`; a write's own marks reads take that write's id). The request that preceded it, the screens, the timings and anything else that would show how to cause it go to the run coordinator by the private channel, not to the branch. The coordinator decides what, if anything, is published later.
- Check each file's sha256 against `SHA256.txt` before its send. No file in `nm/`, `ns/`, `nc/`, `nx/`, `pb/` or `fx/` is ever an argument to `run.py`: they are sources for `gen-mx.py` and `gen-wx.py`, and `pb/L-ledger.xml` is a ledger create.
- Close no Bridge process, and call no Bridge tool.

**Which script reads which answer**
- **`count20.py`** (NX's, byte-equal) reads `a-*-M-marks.xml` and `a-*-N-*`. Its `--before-write` and `--same-row` are the marks gate and the marks compare. Give it no `L-lo-` or `V` answer.
- **`nm.py`** (NX's, byte-equal; it takes the eleven names from this folder's `names.tsv`) reads `a-*-M-marks.xml`, `a-*-S-snap-*`, `a-*-B-bills.xml` and `a-*-L-lo-<id>-*`, with its modes `--outcome`, `--against` and `--same`, whole in its header. Give it no `V` answer.
- **`st.py`** (NX's, byte-equal; it too takes the names from `names.tsv`) reads the import answers `a-*-LM-*` and `a-*-I-*` (`--import`) and the snapshot answers (`--before`, `--names`). Its modes are whole in its header.
- **`rf.py`** (NX's, byte-equal) reads an import answer that `st.py --import` did not pass. Its rule is whole in its header.
- **`wx.py`** (new for this row) reads every `V` answer:
  - `ANSWER: EMPTY ANSWER`, `ANSWER: STATUS <s>` or `ANSWER: VOUCHERS <n>`, with one line a voucher;
  - `--same` compares two answers' bytes;
  - `--match ID FIELDS SCOPED BOOK` prints one line for one id. It says whether the ledger-scoped answer holds exactly the vouchers of the book-wide answer that carry that ledger, matched by `VOUCHERKEY`, each the same in its own fields and in its lines on the ledger. For the window read, that includes each line's bill allocations.

  It never prints a `VOUCHERKEY`'s value. `--match` exits 0 on `EQUAL` and 3 on any other line: exit 3 is a result, not a STOP. Exit 1 is its STOP, in every mode. Its rules are whole in its header.

**A ledger read** (LR below) is the report's two sends for one ledger, `fetch_party_bills`'s second pair (`runtime_party_bills.rs:57`): `r<n>a` and `r<n>b`, the same file twice. The snapshot's pair is sent once for all ledgers, in step 5, before any LR.

## Files
**Byte-equal to NX's file of the same name** (`lab/2480-nx-sheet` at ea84bcdf, folder `lab/2480-nx/`):
- `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py`, `M-marks.xml`, `B-bills.xml`, `S-snap-20260401-20260801.xml`;
- the two templates (`template-import-0010.utf16le.xml`, `template-n-0001.utf16le.xml`);
- `pb/`, `nm/`, `ns/` and `nc/`, never sent.

**NX's, kept in `nx/`, never sent:** NX's `names.tsv`.

**Master's, kept in `fx/`, never sent:** three voucher reads the lab sent, which master keeps as captures (lines at 812d39bb):
- `movement-dn-party-month.request.utf16le.xml`, the movement read of one ledger, which `agent_voucher_window_tests.rs:8842` pins to `render_agent_ledger_vouchers` byte for byte;
- `billtrail-ledger-p1-fy-entries.request.utf16le.xml`, the window read of one ledger, which `agent_voucher_window_tests.rs:9510` pins to the party detail's `ledger_part_read` byte for byte;
- `pilot-lab-audit-vouchers.request.utf16le.xml`, the book-wide window read the lab sent to this book for item 697-S0A.

**Written for this row:** `names.tsv`, `gen-mx.py`, `gen-wx.py`, `wx.py` and this file.

**Made by `gen-mx.py`** (44 files), which is NX's `gen-nx.py` with the lines marked MX changed. Each source file's sha256 is checked first, and it asserts that the same functions give PB's own files byte for byte, and NM's eleven `L-lo` files from NM's `L-lo-pl`.
- `LM-<id>.xml`, `I-<id>.xml` and `N-<number>.xml`: as NX made them, with this row's name, number and tail.
- `L-lo-<id>-20260801.xml`: NM's `L-lo-pl` with its one escaped name turned into this row's escaped name, and nothing else.
- It also asserts:
  - the control holds no mark;
  - each other name holds two or more of the seventeen marks, nothing else but ASCII letters, digits and single spaces, and the quote only as a pair;
  - `al` holds all seventeen, every mark is held by two or more names, and some name mixes an NS mark with an NX mark;
  - no two of this row's names share a key, and none shares a key or a number with NM's, NS's, NC's or NX's;
  - each name occurs in its files as often as NX's did.

**Made by `gen-wx.py`** (27 files): part B's requests above. Run it after `gen-mx.py`. It first checks the three captures' sha256. It then asserts:
- each capture, carved into a template with six holes (company, from, to, collection, scope, field list), is given back byte for byte;
- the three captures give one template, so the three differ in those six values alone;
- each capture's field list is master's constant for it (`agent_read_profiles.rs:48`, `:80` and `:96`);
- each escaped name is the one in this row's `L-lo-<id>` file and reads back as its `names.tsv` name;
- each request, read back by an XML parser, has one `SVCURRENTCOMPANY` (the book), the one day's dates, its `CHILDOF`, `LedgerName`, `FETCH`, `TYPE` and `ID` as the table says;
- `-uv-pl` differs from `-u-pl` by the variable alone, `-v-pl` from `-uv-pl` by the `CHILDOF` text alone, each `-v-<id>` from `-v-pl` by the escaped name alone, and each `Vw-` file from its `Vm-` file by the `FETCH` element alone;
- `Vb-book.xml` is the 697-S0A capture with its two dates changed and nothing else.

`SHA256.txt` lists every file with its size, in NX's form (`sha256  size  path`, which `shasum -c` does not read). Requests are UTF-16LE with a BOM, as sent.

**Tried before this sheet was sent:**
- `gen-mx.py` and `gen-wx.py`: their 44 and 27 files, each check passing.
- `st.py`, on NX's own answers (`lab/2480-nx-answers` at ea691186), with this row's names:
  - `--before`: `FOLD: CLEAR` on NX's `s2` snapshot (18 ledgers) and on its `s6sa` snapshot (29 ledgers, NX's eleven included);
  - `--names`: `NAMES: EQUAL` on `s6sa`;
  - `--import`: `CREATED ONE` on `a-s4pl-LM-pl.xml`.
- `nm.py` on NX's `s2` snapshot and bills: "no row" and 0 groups for each of this row's names.
- `wx.py`, in a copy whose `BOOK` was the other book's GUID:
  - on master's captured answers: `VOUCHERS 7` on `billtrail-window-fy`;
  - on answer shapes cut by hand from those captures for the test, not kept:
    - `--match` `EQUAL` for both field lists;
    - `DIFFER` on a missing voucher, an extra voucher (and an extra one counted as another id's), a changed line amount, a changed narration, a changed AlterID, and (window only) a changed bill name;
    - `EMPTY ANSWER` with HTTP 200 and length 0, with and without a file written;
    - `STATUS 0`;
    - `NOT JUDGED` when the book-wide answer held two vouchers on the ledger, a voucher with another number, an empty answer or a `STATUS`;
    - a STOP on no headers file, on HTTP 500, on a length other than 0, on a missing file, on a voucher listed twice, on a `LINEERROR` under `STATUS` 1, on a non-voucher element, on a `VOUCHERKEY` that is not a whole number, on a GUID that is not the book's, and on a field named twice;
    - `--same` `EQUAL` and `DIFFER`, two empty answers `EQUAL`.
  - The copy as committed STOPs on master's captures: on the bill-trail capture because its GUIDs are another book's; on this book's audit capture because one voucher there, dated 20-Aug-2026, outside step 8's day, has a line with no ledger name, which `wx.py` refuses wherever it is. Cut by an XML parser to its eleven vouchers of 1-Aug-2026, that capture reads as `ANSWER: VOUCHERS 11`. So a voucher of 1-Aug-2026 with such a line would make `k7` a STOP of step 8.
- **Not tried:** any send to Tally.

## Steps
Run each label as `python3 -I run.py http://127.0.0.1:9001 <label> <files>`, the Gateway on 9001 as C2 finds it (written `URL` below). (The lab PC has no `python3` command; there, `python -I` with Python 3 is the same command, as NX and CV ran it.) The prompt rule, the wait rule and the path rule are NM's.

**Labels are not step numbers.**

| step | part | labels |
| --- | --- | --- |
| 1 | | `s1` |
| 2 | | `s2` |
| 3 | A | `p1` to `p11`, then `p12` |
| 4 | A | `s4<id>`, `s5<id>`, with `m` after each for its marks read, and `x` for the marks read after a refusal |
| 5 | A | `s6…` |
| 6 | A | `r<n>a`, `r<n>b` |
| 7 | A | `m1` |
| 8 | B | `k0` to `k8` |
| 9 | B | `q<n>a`, `q<n>b`, `w<n>a`, `w<n>b`, then `k9`, `k10` |
| 10 | | `s9` |
| repeats | | `p12r`, `m1r`, `k0r`, `k8r`, `k10r`: a marks read sent once more after MARKS DIFFER, as steps 3, 7, 8 and 9 say |

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named. For any other prompt, message or dialog:
- copy it word for word and screenshot it first;
- then answer No, or Esc if it offers no No;
- press nothing more and send nothing.

That prompt is this row's result, and C8 holds: the restore waits for the run coordinator's word. What the branch says about it is the STOP rule above.

**Each write, the same way, by hand, one label a command** (NX's write recipe, used only in step 4):
- (a) `run.py URL <label>m M-marks.xml`, then `python3 -I count20.py --before-write a-s1-M-marks.xml a-<label>m-M-marks.xml`.
- (b) Only if that passes, and no key was pressed in Tally since: `run.py URL <label> <file>`, the import alone.
- (c) Screenshot the whole window before the next request (C3). A dialog: the prompt rule.
- (d) `python3 -I st.py --import a-<label>-<file>`, recorded whole. **On exit 0 (`CREATED ONE`), go on with the next write.**
- (e) **Only if `st.py` gave exit 1 and `run.py` in (b) gave exit 0:** `python3 -I rf.py a-<label>-<file>`, recorded whole.
  - On exit 1 (`NOT A REFUSAL`, or a STOP of `rf.py`): the STOP of the set-up.
  - On exit 0 (`REFUSED`): `run.py URL <label>x M-marks.xml`, then `python3 -I count20.py --same-row a-<label>m-M-marks.xml a-<label>x-M-marks.xml`. On SAME MARKS, that import is **refused** for its id: record it, and skip the rest of that id. On anything else: the STOP of the set-up, with nothing more sent.
- On any exit other than 0 of `run.py` in (b), (e) is not run: the STOP of the set-up.

The set-up (step 4) is not run as a loop. If a script sends any of it, the script ends at the first exit other than 0 of `run.py`, `count20.py` or `rf.py`, and at the first exit 1 of `st.py` that (e) does not turn into a refusal, and sends nothing after it. Nothing else decides whether an import answer is as expected.

### Part A

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down (Book).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2480-mx\backup-1` (Tally may show it as `C:\lab\2480-Mx\backup-1`). NX's step 1 is the path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0281.xml N-0282.xml N-0283.xml N-0284.xml N-0285.xml N-0286.xml N-0287.xml N-0288.xml N-0289.xml N-0290.xml N-0291.xml B-bills.xml S-snap-20260401-20260801.xml M-marks.xml`.
   - EXPECT each N read 0 rows (`count20.py`), and `nm.py` on B-bills no group for any of the eleven names, and on the snapshot "no row" for each. A row or a group: stop before step 3, each recorded whole.
   - `st.py --before a-s2-S-snap-20260401-20260801.xml`: EXPECT `FOLD: CLEAR`. A STOP: stop before step 3, recorded whole.
   - `st.py --names a-s2-S-snap-20260401-20260801.xml`, recorded whole.
3. **Reads first: each name before its ledger exists.** For each id in the order of the table, `p<n>`: `L-lo-<id>-20260801.xml`, sent once (`p1` for `pl` to `p11` for `al`, the table's `n`).
   - Screenshot the whole Tally window after each, at once, before the next request: each is a request the lab has not been sent. A dialog: the prompt rule.
   - `nm.py a-p<n>-L-lo-<id>-20260801.xml`, recorded whole. EXPECT `ANSWER: NO BYTES`, under the exception above.
   - **An answer with bytes** is that name's result, the most important kind this part can give: record it whole, with `nm.py --against <id> a-p<n>-L-lo-<id>-20260801.xml a-s2-B-bills.xml a-s2-S-snap-20260401-20260801.xml`. That id's ledger and invoice are never sent; step 4 skips it, and so does part B. The row goes on.
   - **If the control `pl` is answered with bytes,** record it whole and STOP after `p12`: no other name can be judged against it.
   - Then `p12`: `M-marks.xml`, and `count20.py --same-row a-s1-M-marks.xml a-p12-M-marks.xml`. EXPECT SAME MARKS. On MARKS DIFFER: send `M-marks.xml` once more as `p12r` and record it; send nothing else, and STOP.
4. **The set-up,** one ledger at a time in the order of the table, each id's ledger and then its invoice, each write as above. The labels are `s4<id>` for `LM-<id>.xml` and `s5<id>` for `I-<id>.xml`, with `s4<id>m` and `s5<id>m` for the marks reads before them, and `s4<id>x` or `s5<id>x` for the marks read after a refusal.
   - **A refused ledger** (`s4<id>` refused): its invoice is never sent, that id is not read at step 6, and it is out of part B, recorded as such. The next id starts at (a).
   - **A refused invoice** (`s5<id>` refused): the ledger exists with no bill. That id is not read at step 6, it is out of part B, recorded as such, and step 5 records the ledger as it stands.
   - **The STOP of the set-up.** It is one of these, on any answer, a ledger's or an invoice's, the control's or a mixed name's:
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
   - **It is the intended state** if S holds a row for each ledger created at step 4 and none for a refused one, each `ISBILLWISEON` Yes, and B one group for each id whose invoice was created, `BP/26-27/<its number>`. Each closing and bill is the invoice's amount, as NX's were. If not, step 6 is still sent for the ledgers that exist with a bill, and the report says the state is not the intended one.
   - If the two snapshot answers are not byte-equal, master would send nothing more for any ledger; this row still sends step 6, and records that master would not have.
   - **CREATED** is the list of ids whose `LM-` and `I-` imports both gave `CREATED ONE`, in table order, `pl` first. Write it down here. It is the list step 6 reads and part B takes.
6. **The ledger reads.** For each id of CREATED, LR with the labels `r<n>a` and `r<n>b`, `n` the table's (`r1a` and `r1b` for `pl` to `r11a` and `r11b` for `al`). Any other id is not read here, and the report says why (refused at its ledger, refused at its invoice, or skipped at step 3). The request is `L-lo-<id>-20260801.xml`, the file step 3 sent.
   - Screenshot the whole Tally window after each `r<n>a`, before the next request. A dialog: the prompt rule.
   - Then, for each id, recorded whole: `nm.py a-r<n>a-L-lo-<id>-20260801.xml`; `nm.py --same a-r<n>a-… a-r<n>b-…`; `nm.py --outcome <id> a-s6sa-S-snap-20260401-20260801.xml a-s6sb-S-snap-20260401-20260801.xml a-r<n>a-L-lo-<id>-20260801.xml a-r<n>b-L-lo-<id>-20260801.xml`; `nm.py --against <id> a-r<n>a-L-lo-<id>-20260801.xml a-s6ba-B-bills.xml a-s6sa-S-snap-20260401-20260801.xml`.
   - A ledger's answer never stops the row: each is a result, recorded in full. Only the STOPs named stop it.
7. **`m1`:** `M-marks.xml`, then `count20.py --same-row a-s6m-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS: steps 5 and 6 only read. On MARKS DIFFER: send `M-marks.xml` once more as `m1r` and record it; send nothing else, and STOP. The answers already recorded stand.

### Part B

8. **Part A whole, then the shape step, the control alone.**
   - **Part B starts only after part A has passed in full.** Before `k0`, write down that each of these holds, with the line of the notes that shows it:
     - no STOP at any step from 1 to 7;
     - every send of part A answered: each ended with an answer `run.py` kept, or with an empty `L-lo-` answer the exception admits;
     - the marks unchanged by every read: `p12` gave SAME MARKS against `s1`, and `m1` against `s6m`, and no `p12r` or `m1r` was sent (`count20.py --same-row` compares GUID, ALTVCHID and ALTMSTID);
     - the book the same before every write and after every refusal: every `--before-write` passed (`count20.py --before-write` checks one COMPANY row with the book's GUID, not the marks), and every refusal's `--same-row` gave SAME MARKS;
     - the control `pl` is in CREATED, and its step 6 reads gave the outcome `told` with `AGAINST: EQUAL`.

     If any does not hold, part B is not sent, nothing of it: the row goes to step 10, and the report says which did not hold. (A STOP is already C8's, and then the restore waits for the coordinator.) A step 5 result that is not the intended state (a name list not its own, a state other than the intended one) is a part A result, not a failure of part A: part B still runs, and the report says so in both sections.
   - **B's ids** are CREATED, in table order: an id refused at its ledger or its invoice, or skipped at step 3, is not in part B, and the report names it with its reason. A part A result for an id in CREATED (no bytes at step 6, another ledger's rows, `DIFFER`, a code) does not take it out: part B reads it, and part B's section sets the two results side by side.
   - **Then the sends, one at a time, each once, each checked before the next.** After each send, at once and before the next request: screenshot the whole Tally window (each `V` request but `Vb-book.xml` is a request this book has not been sent; a dialog: the prompt rule), then run the check named for it, recorded whole. **The next request is sent only if that check gave what it names.** Anything else is the STOP of the shape step (below), and nothing more is sent.

     | label | file | check, run before the next send | it must give |
     | --- | --- | --- | --- |
     | `k0` | `M-marks.xml` | `count20.py --same-row a-m1-M-marks.xml a-k0-M-marks.xml` | SAME MARKS |
     | `k1` | `Vm-u-pl.xml` | `wx.py a-k1-Vm-u-pl.xml` | `ANSWER: VOUCHERS <n>` |
     | `k2` | `Vm-uv-pl.xml` | `wx.py --same a-k1-Vm-u-pl.xml a-k2-Vm-uv-pl.xml` | `SAME: EQUAL` |
     | `k3` | `Vm-v-pl.xml` | `wx.py --same a-k1-Vm-u-pl.xml a-k3-Vm-v-pl.xml` | `SAME: EQUAL` |
     | `k4` | `Vw-u-pl.xml` | `wx.py a-k4-Vw-u-pl.xml` | `ANSWER: VOUCHERS <n>` |
     | `k5` | `Vw-uv-pl.xml` | `wx.py --same a-k4-Vw-u-pl.xml a-k5-Vw-uv-pl.xml` | `SAME: EQUAL` |
     | `k6` | `Vw-v-pl.xml` | `wx.py --same a-k4-Vw-u-pl.xml a-k6-Vw-v-pl.xml` | `SAME: EQUAL` |
     | `k7` | `Vb-book.xml` | `wx.py a-k7-Vb-book.xml`, then `wx.py --match pl movement a-k1-Vm-u-pl.xml a-k7-Vb-book.xml` and `wx.py --match pl window a-k4-Vw-u-pl.xml a-k7-Vb-book.xml` | `ANSWER: VOUCHERS <n>`, then `MATCH pl movement: EQUAL` and `MATCH pl window: EQUAL` |
     | `k8` | `M-marks.xml` | `count20.py --same-row a-k0-M-marks.xml a-k8-M-marks.xml` | SAME MARKS |

     Once `k8` has passed, also run `wx.py` on each of `a-k2`, `a-k3`, `a-k5` and `a-k6`, for the record (each is byte-equal to `a-k1` or `a-k4` by then).
   - **The shape step passes only if every check above gave what it names**, every send answered, and no dialog came up. Then, on this book and release: master's two reads hold the control's one invoice as the book-wide read of the day shows it (`k1`, `k4` against `k7`); the added variable changed nothing in master's form (`k2`, `k5`); the `##LedgerName` form gave master's bytes for each read (`k3`, `k6`); and these reads left the marks as they were (`k0` to `k8`).
   - **Anything else is the STOP of the shape step, part B's own:** a check that did not give what it names (a `STATUS`, an empty answer, `SAME: DIFFER`, a `MATCH` line other than `EQUAL`, MARKS DIFFER), a STOP of a script, an answer above the bound, a dialog, or a send with no answer. It is "a STOP at step 8", with the id `pl` where the send was one of `k1` to `k6`. Nothing more is sent: not the rest of the step, not the ten names, not a marks read beyond C8's. On MARKS DIFFER at `k0` or `k8` alone, `M-marks.xml` is sent once more as `k0r` or `k8r` and recorded first. Record each answer whole, screenshot the window, and report to the run coordinator. C8 holds, and the restore waits for the coordinator's word. Part A's answers stand as recorded.
9. **The ledger reads of part B. Only after step 8 has passed.** For each id of B's ids in table order, `n` the table's, four sends, each once, in this order, **each read by `wx.py` before the next request**:
   - `q<n>a`, then `wx.py a-q<n>a-Vm-v-<id>.xml`;
   - `q<n>b`, the same file `Vm-v-<id>.xml`, then `wx.py a-q<n>b-Vm-v-<id>.xml` and `wx.py --same a-q<n>a-Vm-v-<id>.xml a-q<n>b-Vm-v-<id>.xml`;
   - `w<n>a`: `Vw-v-<id>.xml`, then `wx.py a-w<n>a-Vw-v-<id>.xml`;
   - `w<n>b`, the same file `Vw-v-<id>.xml`, then `wx.py a-w<n>b-Vw-v-<id>.xml` and `wx.py --same a-w<n>a-Vw-v-<id>.xml a-w<n>b-Vw-v-<id>.xml`;
   - then, before the next id: `wx.py --match <id> movement a-q<n>a-Vm-v-<id>.xml a-k7-Vb-book.xml` and `wx.py --match <id> window a-w<n>a-Vw-v-<id>.xml a-k7-Vb-book.xml`, one line each.

   Screenshot the whole Tally window after each `q<n>a` and `w<n>a`, before its `wx.py` read. A dialog: the prompt rule. An `EMPTY ANSWER` under the exception takes the exception's own steps (keep, screenshot, one `GET /status`, then `wx.py`) right after that send, before the next request, as at step 6; an empty `q<n>a` or `w<n>a` is followed by its `b` send, as for any other answer.
   - After the last id: `k9`: `Vb-book.xml`, the file `k7` sent, then `wx.py a-k9-Vb-book.xml` and `wx.py --same a-k7-Vb-book.xml a-k9-Vb-book.xml`; then `k10`: `M-marks.xml`, then `count20.py --same-row a-k8-M-marks.xml a-k10-M-marks.xml`. EXPECT `SAME: EQUAL` and SAME MARKS: step 9 only reads. On MARKS DIFFER: send `M-marks.xml` once more as `k10r` and record it; send nothing else, and STOP. If `k9` is not equal to `k7`, record both: every `--match` line was judged against `k7`, and the report says that the book-wide read changed during part B.
   - **An answer `wx.py` reads never stops the row,** whatever it holds: each is a result, recorded in full. That includes an `EMPTY ANSWER` under the exception, a `STATUS`, `SAME: DIFFER`, and every `MATCH` line. `wx.py --match` exits 3 on a line other than `EQUAL`: that is a result, not a STOP. Only its exit 1 is a STOP. A STOP of `wx.py` (an answer it cannot read, or a failed send), a dialog and a send with no answer are part B's STOPs, as everywhere, and the `wx.py` read after each send finds it before the next request. Each is "a STOP at step 9, id X". Nothing more is sent, C8 holds, and part A's answers, and part B's answers before it, stand as recorded.
10. **Restore** BK by C6, with `restore-plan.txt` written first. NX's step 8 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace). List the data folder before and after, as NX and CV did.
    - Then `s9`: every file of s2, in s2's order, and compare whole:
      - `count20.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml` passes;
      - each N read 0 rows;
      - B-bills holds no group of the eleven names;
      - the snapshot shows "no row" for each;
      - each answer's sha256 equals s2's.
    - **Any of these not equal stops the row:** send nothing more, and report before the close. The coordinator decides.
11. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering. Screenshot the window, send it and the answers to the run coordinator, and ask before anything else is pressed or sent. Say in the report to the coordinator what the book then holds. What the branch says about it is the STOP rule of "The sends": only "a STOP at step N", with the id where one applies.

## Record
- **RESULTS keeps part A and part B as two sections,** each with its own confidence line, its own deviations and its own "not measured". A STOP of part B is written in part B's section; part A's section is written from part A's answers alone. Where one id's two results are set side by side, that is in part B's section.
- Each answer's bytes and sha256, its headers file whole, and each script's lines, whole.
- `run.py`'s log for each label, and how long each read took, except for the send of a STOP the STOP rule names.
- Every `--before-write`, `--same-row`, `--import`, `--before`, `--names`, `--outcome`, `--against`, `--same` and `--match` output, and every `rf.py` and `wx.py` output, whole.
- The window after each import, and after each shot named in steps 3, 6, 8 and 9, listed by name, bytes and sha256.
- Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- **For a STOP the STOP rule of "The sends" names** (a wedge, an exit of Tally, a dialog), none of that send's detail goes on the branch: not its screenshots, headers file, log line, timing, or the request that preceded it. All of that goes to the run coordinator by the private channel, and the branch says only "a STOP at step N, id X".
- The BK file and the restore, as C5 and C6 ask, with the data folder listed before and after.
- The release on Tally's screen, and the Gateway's date and period.
- CREATED, as step 5 writes it, and step 8's check of part A, line by line.
- For each ledger:
  - its step 3 answer;
  - whether Tally created it, refused it, or refused its invoice, with the `LINEERROR` text if any;
  - its name list at step 5;
  - its three part A reads (the report pair, the snapshot row, the Bills Receivable group);
  - its part B reads: the two pairs, their `--same` lines and their two `MATCH` lines, or why it was not read.

**Staging the evidence:** the check for 10 or more digits sets aside this book's registered GUID matched whole, and nothing else. Any other hit waits for the run coordinator. Part B's answers hold each voucher's `VOUCHERKEY`, a run of more than ten digits, as CV's census answers did. They are Tally's own, in the answers only. No script prints them, and the RESULTS never quote them.

## Expected
Estimates, not predictions.
- **s2:** as the Book and step 2 say; `FOLD: CLEAR`; `NAMES: EQUAL`.
- **Step 3:** `ANSWER: NO BYTES` (HTTP 200, `CONTENT-LENGTH: 0`) for each name, as NS, NC and NX saw for all their names.
- **Step 4:** `IMPORT: CREATED ONE` for the control. For each mixed name, `CREATED ONE` if a mix acts as its marks did alone in NS, NC and NX, where every name was created. No mix has been sent, so a refusal is not ruled out.
- **Step 5:** for each created ledger, S one row, bill-wise Yes; B one group for each created invoice; `NAMES: EQUAL`.
- **The control `pl` at step 6:** `ANSWER: ROWS 1`, the bill `BP/26-27/0281`; `OUTCOME: told`; `AGAINST: EQUAL`.
- **The other ten at step 6, if created:** as the control. No stronger estimate.
- **Step 8:**
  - `a-k7`, the book-wide read: about 1,800,000 B (above), 22 vouchers, each dated 20260801;
  - `k1` and `k4` `EQUAL` against it (§11c.6 read plain names this way on two other books);
  - `k2` equal to `k1` and `k5` to `k4`, as CV's census with the variable added;
  - `k3` equal to `k1`, and `k6` to `k4`, if the voucher reads evaluate `##LedgerName` in `CHILDOF` as CV's census did. Each differs from that census in its `FETCH` list and collection name only, but that has not been observed, so neither outcome is predicted.
- **Step 9:** as the control, if each name in the variable acts as it did for the census in CV. No stronger estimate.
- **s9:** equal to s2 and s1.

## Decides
Each is PARTIAL: one book, one release, one sitting, master's request templates sent by curl, without the reads Bridge sends around them, and without the build's own parse. Part A and part B are judged apart: a result in one never changes the other's.

**Part A, for each of the ten names,** one of these. The control is the baseline: if it is not `told` with `AGAINST: EQUAL`, no other name is judged until it is explained, and part B is not sent.
- **Step 3 answered with bytes:** Tally reads that name, before its ledger exists, as some other ledger. That mix is not admitted, and the coordinator decides what it means for the type.
- **Refused at its ledger:** Tally will not hold that name through master's ledger import. The report read was not tried, so the row says nothing about whether the report could read such a ledger.
- **Refused at its invoice:** the ledger exists but master's invoice shape would not post to it. Recorded; the read was not tried.
- **`told`, `AGAINST: EQUAL`, the one bill the ledger's own, the two sends equal, and NO BYTES at step 3:** on this book and release, Tally reads that name in the `LedgerName` static variable as that ledger and as no other. With NS, NC and NX, that is evidence that a mix of measured marks reads as its marks did alone, for that mix.
- **NO BYTES at step 6,** for an id of CREATED: Tally matched no ledger for that name, so the mix is not admitted.
- **Rows that are another ledger's at step 6:** as step 3's, the most important result.
- **`party_bills_untied`, `party_bills_answer_malformed`, `party_bills_tally_failure` or another code:** master's parse would refuse that read for that name, so admitting the mix would not make it work as it is.
- **`SAME: DIFFER`:** record both; master would refuse as `party_bills_drifted`.
- **The STOP of step 4 at that name's import:** what Tally did with that name as a new ledger; no name after it is measured, and the coordinator decides whether a later row continues.
- **A name list not its own at step 5:** an import answered `CREATED` 1 still changed a ledger's name; recorded, and a finding for any writer as much as for #2480.

**If all ten are read as their own:** a static-variable type that admits a name whose every mark was measured has evidence for these mixes too, on one book and release. **If some are not, or are refused:** the type admits only what was read, and the report names the rest.

**Part B's shape step:**
- **It passes:** on this book and release, each data-carrying ledger-scoped read in the `##LedgerName` form gave master's bytes for the control, and master's form held what the book-wide read of the day holds for that ledger. The ten are then measured.
- **It does not pass:** that form is not master's read on this book and release, for the read that differed; nothing in part B measures the ten through it. Record what differed. `LedgerScopeName` stays as it is for those reads.

**Part B, for each name of B's ids,** for each of the two reads, one of these:
- **`MATCH: EQUAL`, the pair `SAME: EQUAL`:** on this book and release, that read in the `##LedgerName` form read that ledger's vouchers as the book-wide read shows them. With CV, that is evidence that the read can take that name as data.
- **`MATCH: DIFFER`:** the read does not give that ledger's vouchers as the book shows them; the line names what differs. The form is not admitted for that name in that read.
- **`EMPTY ANSWER` or `STATUS`:** the form does not read that name's ledger; not admitted.
- **`NOT JUDGED`:** the book-wide read did not show that id's invoice as part A made it; recorded, and the coordinator decides.
- **`SAME: DIFFER`:** record both; master would not plan on an answer that changes between two sends.

**If part B's shape step passes and every name of B's ids is `EQUAL` in both reads:** `ledger_movement` and the `outstandings` party detail could send their ledger-scoped reads with the name as the static variable, for every name the static-variable type admits, on the evidence of one book and release. The build of that change is #2480's and #485's, and the choice is the maintainers'.

**What these answers are for.** They inform the static-variable read's own type on #2480 and the scoped reads of #485. They become fixtures only if the manifest admits them.

## Code admits the trigger
Lines are at master 812d39bb.
- **The build refuses these names before any send.** `LedgerScopeName::new` (`src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65`) refuses any character outside ASCII letters, digits and a single space (`:74`) and the words of `FORMULA_WORDS` (`:30`). `admitted_party` (`src-tauri/src/agent_party_bills.rs:26`) maps each refusal to `party_name_not_admitted`, and `scope_name_of` (`src-tauri/src/agent_movement.rs:99`) maps it to the whole book's window. So no build reaches this row's reads, and this row sends by curl.
- **The build never sends the `##LedgerName` form.** `VoucherScope::collection` (`src-tauri/src/agent_read_profiles.rs:279`) writes the name itself into `CHILDOF`, escaped once (`:286`). Its users are the census (`:258`) and `render_agent_ledger_vouchers` (`:367`), whose two field lists are `LedgerVoucherFields` (`:340`).
- **The bills read is master's.** `fetch_party_bills` (`src-tauri/src/tally/runtime_party_bills.rs:57`) reads the snapshot and then the report, each sent twice, and ties them. `render_ledger_outstandings_request` (`src-tauri/crates/bridge-tally-protocol/src/native_outstandings/party_bills.rs:40`) takes the party as the text of the `LedgerName` static variable (`:49`), escaped once as XML text.
- **The key `st.py` checks is master's collision key, read in Python:** `letters_and_digits_key` (`src-tauri/crates/bridge-tally-core/src/master_binding.rs:1982`), which the ledger-create path asks at admission (`src-tauri/src/agent_master_post.rs:338`).

## Not in this row
- ComplyEaze Bridge's own transport, or any Bridge build;
- any of the ten names inside `CHILDOF` itself (the formula position), which this row never sends;
- a mark not in the seventeen (`<`, `>`, `;`, `=`, `$`, `?`, `[`, `]`, `{`, `}`, `|`, `\`, `~`, `` ` `` and others), the word `and` and the Devanagari name of NS beside these marks, a double space, a space at an edge, a name of the longest admitted length, a name that differs from another only by case;
- names that share a stem with another ledger, and what Tally matches a created name on;
- a ledger keyed by hand, for a name the import refuses;
- a ledger with more than one voucher or bill, a settled or part-settled bill, a voucher with two lines on the ledger, a note or a voucher after the invoice;
- part B's reads over more than one day, with an AlterID span, or on a book of real size; the census in this form for these names (CV's names were one mark each);
- another to-date, book or release; a book with GST on; Educational mode.
