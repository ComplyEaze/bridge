# Row 2480-MW: do the two data-carrying ledger-scoped reads take a ledger name as a static variable? (Row 2480-MX's part B again, with its reader mended)

Shape discovery, as rows 2346-PB, 2480-NM, 2480-NS, 2480-NC, 2480-NX, 2480-CV and 2480-MX: the requests are written for the lab, not requests the build sends, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2480 (0.9.0) and #485. Written 11 Nov 2026. Lines are at master 812d39bb. Confidence of everything here until the row runs: unverified.

This row is row 2480-MX's part B, sent again on a book set up as MX's part A set it up. It has three changes from MX:
- MX's part A reads (its steps 3 and 6) are not sent: part A's results are MX's, and stand.
- Part A's set-up is kept: the same 22 imports, byte for byte, with every check MX made on them.
- **`wx.py`, part B's reader, is mended.** MX's part B stopped on a defect of that sheet's own (below), not on anything Tally did.

## Why this row
**What MX found.** In row 2480-MX's sitting, part A passed in full, as the run coordinator reports it:
- all 22 imports `CREATED ONE`;
- every id's report pair `ROWS 1`, `SAME: EQUAL`, `OUTCOME: told` and `AGAINST: EQUAL`;
- the marks unchanged by every read.

Part B ended at a STOP at step 8, id `pl`, at `k1`, its first voucher read. That STOP was `wx.py`'s own, on the answer to master's own movement read of the control, `Vm-u-pl.xml`, with no dialog. The coordinator then ordered MX's restore and close, and part B was not sent again in that sitting. MX's answers stand as its RESULTS record them, once its answers branch has passed the coordinator's gate. MX's RESULTS, not this sheet, are the record of that sitting.

**The defect, the sheet's own.** MX's `wx.py` read every `BILLALLOCATIONS.LIST` that was not empty as a bill, and STOPped on one without a `NAME`. Master's movement read asks for no bill field (`AGENT_MOVEMENT_FETCH`, `src-tauri/src/agent_read_profiles.rs:96`). Tally still lists the party line's `BILLALLOCATIONS.LIST`, holding one `AMOUNT` and nothing else. That is an ordinary answer, and master's own capture of a movement read shows it: `ledger-scoped-vouchers/movement-dn-party-month.answer.utf16le.xml` holds 21 such elements and 21 empty ones. MX's sheet ran `wx.py` on that capture only in a copy pinned to another book's GUID. The copy STOPped at the GUID check, before any bill was read, and the sheet took that STOP for the expected one. Read with its own book's GUID, MX's `wx.py` STOPs on that capture with the same message as at `k1`. So `k1` measured the reader, not the read.

**The mend.** `wx.py` reads a `BILLALLOCATIONS.LIST` that holds exactly one child, `AMOUNT`, itself with no child, as a bill with no name and no bill type (printed `<no name>`). It still skips an empty one, as MX did. It still STOPs on any other allocation without exactly one `NAME`: `AMOUNT` with any other child, a `BILLTYPE` alone, two `AMOUNT`s, or an `AMOUNT` holding an element. That is the only change; its lines are marked MW.

**What part B asks** is unchanged from MX. Row 2480-CV showed, on this book and release, that the ledger-scoped census reads one ledger when its name is the `LedgerName` static variable and `CHILDOF` holds only `##LedgerName`. The census only plans a read. The reads that carry the data are the movement read (`ledger_movement`) and the window read (the `outstandings` party detail). Both are master's `render_agent_ledger_vouchers`, which differs from the census in its `FETCH` list and its collection name. Neither has been sent in this form with an answer read. Until they have, the change the CV comment describes on #2480 cannot be built. `LedgerScopeName` admits only letters, digits and single spaces there, and every other name falls back to the whole book's window (protocol reference §11c.6; see also §11d).

## Question
On the ledgers this row's set-up creates, the names of row 2480-MX:
1. **The shape (the control only).** Does each of the two reads, with `<CHILDOF>##LedgerName</CHILDOF>` and the name in the `LedgerName` static variable, give the same bytes as master's read with the name in `CHILDOF`? Does master's read hold exactly the vouchers of the book-wide read that carry the control's ledger?
2. **For each of the ten other names,** does each read in that form hold exactly the vouchers of the book-wide read that carry that ledger, each the same in its own fields and its line on the ledger, twice byte-equal?

**The one variable** in the shape step is the request's form, with the control's name fixed; in step 7 it is the name, with the form fixed.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` in this folder (byte-equal to MX's) applies in full. Where it and this sheet differ, the stricter one holds.

- **Pinned by GUID.** `count20.py`, `nm.py`, `st.py` and `wx.py` each hold BRIDGE PILOT LAB's GUID as `BOOK`, and each STOPs on a row with another (`wx.py` on a voucher GUID that does not begin with it). (`rf.py` reads import answers, which carry no GUID; the marks read beside each import is pinned.)
- **Its state:** ALTVCHID 68 and ALTMSTID 245, the marks MX's restore is to leave (its `s9`, equal to its `s1`), as the run coordinator reports. Other marks at s1: stop before BK, with nothing sent.
- **It holds none of the eleven ledgers and none of the numbers below.** Step 2 checks this, and a row there is a STOP. It is also the check that MX's restore took MX's set-up out.
  - The ledgers are the eleven names of `names.tsv`, all beginning `BRIDGE Mx `: MX's names, byte for byte.
  - The invoice numbers are `BP/26-27/0281` to `BP/26-27/0291`, and the `REMOTEID` tails `000000000e81` to `000000000e8b`: MX's.
- **The 22 imports are MX's files, byte for byte** (`LM-<id>.xml`, `I-<id>.xml`), by the run coordinator's direction. MX sent each once, and MX's restore is to take their effect out of the book; step 2 checks that it did. Under C3, a file that writes is sent once in an item's life: this row is its own item, and sends each at most once, never again whatever happened to its first send. If step 2 finds any of the eleven ledgers or numbers, nothing of the set-up is sent.
- **The refused-import exception (NX's and MX's, unchanged).** It holds only with the run coordinator's word, given when the row is queued. An import that step 3 (e) finds refused shows no effect, by design. That absence is the result for its id, and the row goes on. The exception covers that import alone. Every other write that a read does not show is C3's STOP. (In MX, no import was refused.)
- **The date and the period:** read the Gateway's current date and period at step 1. If the date is before 1-Aug-2026, or the period is not 1-Apr-26 to 31-Mar-27: stop before BK. **Do not change Tally's current date or period.**
- **BK is this row's way back.** It is taken at step 1, before anything is sent, and restored at step 8 by C6. The restore runs on every path from step 3 on that is not a STOP: after step 7, or after step 6 when the set-up did not pass in full. After any STOP, at any step (C7, C8 or C9, a script's, a marks compare's, or one a step names), C8 holds, and the restore waits for the run coordinator's word.
- Write down the release shown on Tally's screen.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6).

**The eleven ledgers** (`names.tsv`: `id`, invoice number, name), MX's. Each is created by `LM-<id>.xml` and given its one open bill by `I-<id>.xml`, each sent at most once.

| n | id | number | name |
| --- | --- | --- | --- |
| 1 | `pl` | 0281 | `BRIDGE Mx Plain` (the control: letters and single spaces, which `LedgerScopeName` admits) |
| 2 | `ga` | 0282 | `BRIDGE Mx Ak, Am & Co` |
| 3 | `ss` | 0283 | `BRIDGE Mx Bk & Sons (Bm), Bn` |
| 4 | `ms` | 0284 | `BRIDGE Mx M/s. Ck_Cm, Cn` |
| 5 | `qd` | 0285 | `BRIDGE Mx "Dk" Dm & Dn` |
| 6 | `pp` | 0286 | `BRIDGE Mx Ek 50% + Em` |
| 7 | `hd` | 0287 | `BRIDGE Mx Fk #2, Fm-Fn` |
| 8 | `ad` | 0288 | `BRIDGE Mx Gk @ Gm.` |
| 9 | `ca` | 0289 | `BRIDGE Mx Hk: Hm's (Hn)` |
| 10 | `xs` | 0290 | `BRIDGE Mx Jk! Jm *` |
| 11 | `al` | 0291 | `BRIDGE Mx Lk, Lm_Ln "Lp" + Lq @ Lr #4: 6% * Ls! & Lt. Lu-Lv/Lw (Lx) Ly's` |

Every mark reaches Tally only as escaped XML text: in the `Vm-v-` and `Vw-v-` requests, as the text of the `LedgerName` static variable; in the imports, as master's import shapes carry a name. **None of the ten is ever sent inside a formula.** The only requests with a name inside `CHILDOF` are `Vm-u-pl.xml`, `Vm-uv-pl.xml`, `Vw-u-pl.xml` and `Vw-uv-pl.xml`, the control's, whose name master's type already admits.

## The voucher reads
All are master's voucher reads, BRIDGE PILOT LAB, the one day 1-Aug-2026 (`SVFROMDATE` and `SVTODATE` 20260801, the formula's two dates the same), made by `gen-wx.py`: MX's files, byte for byte. **The window** starts on day 1 of August, holds 1-Aug-2026, the day of every invoice of this row, and holds no day outside that month. It is one day, not the month, because the book-wide read of the month would pass the read bound: by master's own capture of this book (`agent/audit-read-pilot-lab/pilot-lab-audit-vouchers.utf16le.xml`, 15 Oct 2026, at the marks 68 and 245), its vouchers dated in August come to more than 2,000,000 B before this row adds any.

| file | what it is |
| --- | --- |
| `Vb-book.xml` | master's book-wide window read (`render_agent_vouchers`, field list `AGENT_VOUCHER_FETCH`): the request the lab sent this book for item 697-S0A, with its window alone changed. The ground truth the ledger-scoped reads are compared with. |
| `Vm-u-pl.xml` | master's movement read of the control (`render_agent_ledger_vouchers`, `LedgerVoucherFields::Movement`): `<CHILDOF>BRIDGE Mx Plain</CHILDOF>` |
| `Vm-uv-pl.xml` | `Vm-u-pl.xml` with `<LedgerName>BRIDGE Mx Plain</LedgerName>` after `SVCURRENTCOMPANY`, and nothing else changed |
| `Vm-v-<id>.xml` | `Vm-uv-pl.xml` with `<CHILDOF>##LedgerName</CHILDOF>` and the name of `<id>` in `LedgerName`, for each of the eleven |
| `Vw-u-pl.xml`, `Vw-uv-pl.xml`, `Vw-v-<id>.xml` | the same three forms of the window read (`LedgerVoucherFields::Window`, the `outstandings` party detail's), which differ from the `Vm-` file of the same name in the `FETCH` element alone |

**Its size, estimated from that capture.** The book's eleven vouchers of 1-Aug-2026 come to 806,774 B of answer in that capture. `BP/26-27/0010`, the voucher MX's invoice template copies, comes to 90,564 B. So `Vb-book.xml`'s answer at step 6 is estimated at about 1,800,000 B (806,774 B and eleven times 90,564 B), under the bound. This is an estimate, not a measurement: MX did not reach that send. An answer above 2,000,000 B is `run.py`'s stop: kept and not used, it is the STOP of step 6, and nothing more of the step is sent.

## The sends and the wedge rule
`run.py` is NM's, byte for byte: one request at a time, each answer's headers kept by curl -D in `a-<label>-<file>.headers`, and it never abandons a request.
- **The wedge rule.** If any send has not answered **5 minutes** after it began (NM's bound): press nothing, do not end curl, send nothing more (not even `GET /status`), screenshot the whole Tally window, and report to the run coordinator (C9). Nothing is re-sent on this row's own judgement.
- **The stops.** `run.py`'s stops are failed requests, never "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 B. So are the STOPs of `count20.py`, `nm.py`, `st.py`, `rf.py` and `wx.py`. Each stops the row there, with two exceptions: the refused import of step 3, and the empty answer below.
- **The empty-answer exception for a `Vm-v-` or `Vw-v-` answer at step 7.** MX's, unchanged. It holds only with the run coordinator's word given when the row is queued; without it such an answer is a STOP.
  - Every name read there has a ledger, so an empty answer is not expected; under the exception it is that id's result, recorded as `wx.py` reads it (`EMPTY ANSWER`), and the step goes on.
  - It is recorded first as what it is: `run.py` printed "empty answer", so the request failed. It is not a STOP only if its headers file shows HTTP status 200 and `CONTENT-LENGTH: 0`. `run.py`'s "stop, do not resend" is then read as this exception. `wx.py` tells it apart from a failed send by the headers file in the same way, and any other empty body (no headers file, no status line, another status or another length) is a STOP, and that request is never sent again.
  - Then, in this order, before the next request: keep the headers file and the log line, and screenshot the whole Tally window; if a dialog is up: STOP; send `GET /status` once (C2c's command), recording its status and bytes, and if it is not 200: STOP; run `wx.py` on that answer, and go on with the step.
  - Each `GET /status` this sends is a request of the row: it is logged with the rest, and there may be up to 44 of them.
  - An empty answer at step 6, to `Vb-book.xml` at any step, or to any other request, an `LM-` or `I-` import included, is a STOP, as `run.py` says.
- **What the branch and any public text say about a STOP of this kind.** If a send wedges (the wedge rule), Tally exits or stops answering, or a dialog the prompt rule names comes up, the row's RESULTS and any public comment on it say only "a STOP at step N, id X" (or "a STOP at step N" where no id applies: steps 1, 2, 4, 5, 8 and 9, the marks reads `k0`, `k8` and `k10`, and the book-wide reads `k7` and `k9`; a write's own marks reads take that write's id). The request that preceded it, the screens, the timings and anything else that would show how to cause it go to the run coordinator by the private channel, not to the branch. The coordinator decides what, if anything, is published later.
- Check each file's sha256 against `SHA256.txt` before its send. No file in `nm/`, `ns/`, `nc/`, `nx/`, `pb/` or `fx/` is ever an argument to `run.py`: they are sources for `gen-mx.py` and `gen-wx.py`, and `pb/L-ledger.xml` is a ledger create. **No `L-lo-` file is sent in this row**: they are `gen-mx.py`'s, kept so that its output is MX's whole.
- Close no Bridge process, and call no Bridge tool.

**Which script reads which answer**
- **`count20.py`** (MX's, byte-equal) reads `a-*-M-marks.xml` and `a-*-N-*`. `--before-write` checks one COMPANY row with the book's GUID; `--same-row` also compares ALTVCHID and ALTMSTID (SAME MARKS). Give it no `V` answer.
- **`nm.py`** (MX's, byte-equal; it takes the eleven names from this folder's `names.tsv`) reads `a-*-M-marks.xml`, `a-*-S-snap-*` and `a-*-B-bills.xml`, with its modes whole in its header. Give it no `V` answer.
- **`st.py`** (MX's, byte-equal; it too takes the names from `names.tsv`) reads the import answers `a-*-LM-*` and `a-*-I-*` (`--import`) and the snapshot answers (`--before`, `--names`). Its modes are whole in its header.
- **`rf.py`** (MX's, byte-equal) reads an import answer that `st.py --import` did not pass. Its rule is whole in its header.
- **`wx.py`** (MX's, with the one change marked MW) reads every `V` answer:
  - `ANSWER: EMPTY ANSWER`, `ANSWER: STATUS <s>` or `ANSWER: VOUCHERS <n>`, with one line a voucher;
  - `--same` compares two answers' bytes;
  - `--match ID FIELDS SCOPED BOOK` prints one line for one id. It says whether the ledger-scoped answer holds exactly the vouchers of the book-wide answer that carry that ledger, matched by `VOUCHERKEY`, each the same in its own fields and in its lines on the ledger. For the window read, that includes each line's bill allocations, an amount-only one as `<no name>`.

  It never prints a `VOUCHERKEY`'s value. `--match` exits 0 on `EQUAL` and 3 on any other line: exit 3 is a result, not a STOP. Exit 1 is its STOP, in every mode. Its rules are whole in its header.

## Files
**Byte-equal to MX's file of the same name** (`lab/2480-mx-sheet` at 080692b7, folder `lab/2480-mx/`), every file but `wx.py`, this file and `SHA256.txt`:
- `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py`, `M-marks.xml`, `B-bills.xml`, `S-snap-20260401-20260801.xml`, `names.tsv`;
- the two templates (`template-import-0010.utf16le.xml`, `template-n-0001.utf16le.xml`);
- `gen-mx.py` and its 44 files (`LM-<id>.xml`, `I-<id>.xml`, `N-<number>.xml`, `L-lo-<id>-20260801.xml`);
- `gen-wx.py` and its 27 files (`Vb-book.xml`, `Vm-…`, `Vw-…`);
- `pb/`, `nm/`, `ns/`, `nc/`, `nx/` and `fx/`, never sent.

What each generator asserts is in MX's ROW.md and in its own header. Run `gen-mx.py` first, then `gen-wx.py`; each checks its sources' sha256 first.

**Changed for this row:** `wx.py` (MX's, with the lines marked MW) and this file.

`SHA256.txt` lists every file with its size, in MX's form (`sha256  size  path`, which `shasum -c` does not read). Requests are UTF-16LE with a BOM, as sent.

**Tried before this sheet was sent:**
- `gen-mx.py` and `gen-wx.py`, run in a copy: their 44 and 27 files, byte-equal to this folder's, each check passing.
- **MX's `wx.py` and this row's**, each in a copy whose `BOOK` was the capture's own book GUID, on master's three answers to the reads this row sends:
  - `ledger-scoped-vouchers/movement-dn-party-month.answer`, master's movement read: MX's STOPs, `a bill allocation without one NAME`, as at MX's `k1`; this row's reads `ANSWER: VOUCHERS 21`.
  - `ledger-filtered-allocations/billtrail-ledger-p1-fy-entries.answer`, master's window read of one ledger: both read `ANSWER: VOUCHERS 7`.
  - `agent/audit-read-pilot-lab/pilot-lab-audit-vouchers`, master's book-wide read of this book: both STOP on one voucher dated 20-Aug-2026, outside step 6's day, which has a line with no ledger name. Cut by an XML parser to its eleven vouchers of 1-Aug-2026, it reads `ANSWER: VOUCHERS 11`. So a voucher of 1-Aug-2026 with such a line would make `k7` a STOP of step 6.
- **A cross-read of two answers of one book**, in a copy of this row's `wx.py` with the one-invoice checks of `--match` turned off: `billtrail-ledger-p1-fy-entries.answer` against `ledger-filtered-allocations/billtrail-window-fy.answer`, that book's book-wide window read. `MATCH: EQUAL` for both field lists. With the ledger-scoped answer's named bill cut to an amount-only one: movement `EQUAL` (bills are not compared), window `DIFFER`, the bill allocations.
- **The mend's edges,** on master's movement answer with one allocation cut by hand: `AMOUNT` and `BILLTYPE` with no `NAME`, a `BILLTYPE` alone, two `AMOUNT`s, `AMOUNT` and `BILLDATE`, and an `AMOUNT` holding an element each STOP (`a bill allocation without one NAME`); `NAME` and `AMOUNT` reads as a bill.
- **MX's tests again:** each answer file MX's `wx.py` was tried on, read by MX's and by this row's `wx.py` with the same `BOOK`: the same output, line for line, for all 25.
- **Not tried:** any send to Tally; the answers of MX's sitting, which are not yet on a branch.

## Steps
Run each label as `python3 -I run.py http://127.0.0.1:9001 <label> <files>`, the Gateway on 9001 as C2 finds it (written `URL` below). (The lab PC has no `python3` command; there, `python -I` with Python 3 is the same command, as NX, CV and MX ran it.) The prompt rule, the wait rule and the path rule are NM's.

**Labels are not step numbers.** They are MX's, so that this row's answers sit beside MX's under the same names.

| step | labels |
| --- | --- |
| 1 | `s1` |
| 2 | `s2` |
| 3 | `s4<id>`, `s5<id>`, with `m` after each for its marks read, and `x` for the marks read after a refusal |
| 4 | `s6…` |
| 5 | `m1` |
| 6 | `k0` to `k8` |
| 7 | `q<n>a`, `q<n>b`, `w<n>a`, `w<n>b`, then `k9`, `k10` |
| 8 | `s9` |
| repeats | `m1r`, `k0r`, `k8r`, `k10r`: a marks read sent once more after MARKS DIFFER, as steps 5, 6 and 7 say |

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named. For any other prompt, message or dialog:
- copy it word for word and screenshot it first;
- then answer No, or Esc if it offers no No;
- press nothing more and send nothing.

That prompt is this row's result, and C8 holds: the restore waits for the run coordinator's word. What the branch says about it is the STOP rule above.

**Each write, the same way, by hand, one label a command** (MX's write recipe, used only in step 3):
- (a) `run.py URL <label>m M-marks.xml`, then `python3 -I count20.py --before-write a-s1-M-marks.xml a-<label>m-M-marks.xml`.
- (b) Only if that passes, and no key was pressed in Tally since: `run.py URL <label> <file>`, the import alone.
- (c) Screenshot the whole window before the next request (C3). A dialog: the prompt rule.
- (d) `python3 -I st.py --import a-<label>-<file>`, recorded whole. **On exit 0 (`CREATED ONE`), go on with the next write.**
- (e) **Only if `st.py` gave exit 1 and `run.py` in (b) gave exit 0:** `python3 -I rf.py a-<label>-<file>`, recorded whole.
  - On exit 1 (`NOT A REFUSAL`, or a STOP of `rf.py`): the STOP of the set-up.
  - On exit 0 (`REFUSED`): `run.py URL <label>x M-marks.xml`, then `python3 -I count20.py --same-row a-<label>m-M-marks.xml a-<label>x-M-marks.xml`. On SAME MARKS, that import is **refused** for its id: record it, and skip the rest of that id. On anything else: the STOP of the set-up, with nothing more sent.
- On any exit other than 0 of `run.py` in (b), (e) is not run: the STOP of the set-up.

The set-up (step 3) is not run as a loop. If a script sends any of it, the script ends at the first exit other than 0 of `run.py`, `count20.py` or `rf.py`, and at the first exit 1 of `st.py` that (e) does not turn into a refusal, and sends nothing after it. Nothing else decides whether an import answer is as expected.

### The set-up

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down (Book).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2480-mw\backup-1` (Tally may show it as `C:\lab\2480-Mw\backup-1`). MX's step 1 is the path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0281.xml N-0282.xml N-0283.xml N-0284.xml N-0285.xml N-0286.xml N-0287.xml N-0288.xml N-0289.xml N-0290.xml N-0291.xml B-bills.xml S-snap-20260401-20260801.xml M-marks.xml`.
   - EXPECT each N read 0 rows (`count20.py`), and `nm.py` on B-bills no group for any of the eleven names, and on the snapshot "no row" for each. A row or a group: stop before step 3, each recorded whole. (It would mean MX's set-up is still in the book.)
   - `st.py --before a-s2-S-snap-20260401-20260801.xml`: EXPECT `FOLD: CLEAR`. A STOP: stop before step 3, recorded whole.
   - `st.py --names a-s2-S-snap-20260401-20260801.xml`, recorded whole.
3. **The set-up,** MX's step 4: one ledger at a time in the order of the table, each id's ledger and then its invoice, each write as above. The labels are `s4<id>` for `LM-<id>.xml` and `s5<id>` for `I-<id>.xml`, with `s4<id>m` and `s5<id>m` for the marks reads before them, and `s4<id>x` or `s5<id>x` for the marks read after a refusal.
   - **A refused ledger** (`s4<id>` refused): its invoice is never sent, and that id is out of steps 6 and 7, recorded as such. The next id starts at (a).
   - **A refused invoice** (`s5<id>` refused): the ledger exists with no bill. That id is out of steps 6 and 7, recorded as such, and step 4 records the ledger as it stands.
   - **The STOP of the set-up.** It is one of these, on any answer, a ledger's or an invoice's, the control's or another name's:
     - an exit 1 of `st.py --import` that (e) does not turn into a refusal;
     - a refusal whose marks after it differ from those before it;
     - an exit other than 0 of `run.py` on an `LM-` or `I-` file (a curl failure, an empty answer, Unknown Request or an answer above the bound), after which what the import did is not known and the file is never sent again;
     - a dialog after an import;
     - a refusal of the control's ledger or invoice: without the control, no other name can be judged.

     At a STOP: send nothing more, not even the invoice of that id; screenshot the whole window; record the answer, if there is one, whole; and report to the run coordinator. C8 holds. The book then holds each ledger and invoice created before it, and possibly one that answer altered: say which in the report. For a dialog, or a send with no answer, the branch says only what the STOP rule of "The sends" allows, and the rest goes to the coordinator.
   - **Why the marks read after a refusal is also the check of the book.** `count20.py --same-row` compares the book's GUID, ALTVCHID and ALTMSTID. If the book was no longer loaded, or another company was, that read does not pass, so an import that failed for that reason is a STOP, not a refusal.
4. **The book-wide reads and the state,** MX's step 5. `s6m`: `M-marks.xml`. Then `s6sa`, `s6sb`: `S-snap-20260401-20260801.xml`, the same file twice; and `s6ba`, `s6bb`: `B-bills.xml`, the same file twice. Neither request is a new shape (NM's files, byte for byte); a dialog at any time is still the prompt rule.
   - `nm.py` on `a-s6sa-S-snap-20260401-20260801.xml` and `a-s6ba-B-bills.xml`, each recorded whole.
   - `nm.py --same` on each pair, recorded.
   - `st.py --names a-s6sa-S-snap-20260401-20260801.xml`, recorded whole. EXPECT `NAMES: EQUAL`, each created ledger's name list its own name. A ledger whose name list is not its NAME attribute is a set-up result, recorded whole.
   - **It is the intended state** if S holds a row for each ledger created at step 3 and none for a refused one, each `ISBILLWISEON` Yes, and B one group for each id whose invoice was created, `BP/26-27/<its number>`. Each closing and bill is the invoice's amount, as MX's were. If not, the report says the state is not the intended one.
   - **CREATED** is the list of ids whose `LM-` and `I-` imports both gave `CREATED ONE`, in table order, `pl` first. Write it down here. It is the list steps 6 and 7 take.
5. **`m1`:** `M-marks.xml`, then `count20.py --same-row a-s6m-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS: step 4 only read. On MARKS DIFFER: send `M-marks.xml` once more as `m1r` and record it; send nothing else, and STOP.

### The voucher reads

6. **The set-up whole, then the shape step, the control alone.**
   - **The voucher reads start only after the set-up has passed in full.** Before `k0`, write down that each of these holds, with the line of the notes that shows it:
     - no STOP at any step from 1 to 5;
     - every send of steps 1 to 5 answered: each ended with an answer `run.py` kept;
     - the marks unchanged by every read: `m1` gave SAME MARKS against `s6m`, and no `m1r` was sent (`count20.py --same-row` compares GUID, ALTVCHID and ALTMSTID);
     - the book the same before every write and after every refusal: every `--before-write` passed (`count20.py --before-write` checks one COMPANY row with the book's GUID, not the marks), and every refusal's `--same-row` gave SAME MARKS;
     - the control `pl` is in CREATED, and step 4 found it in the intended state: its S row, bill-wise Yes, its closing the invoice's amount, and one B group, `BP/26-27/0281`, `nm.py --same` `EQUAL` on both pairs.

     If any does not hold, nothing of steps 6 and 7 is sent: the row goes to step 8, and the report says which did not hold. (A STOP is already C8's, and then the restore waits for the coordinator.) A step 4 result that is not the intended state for an id other than `pl` (a name list not its own, a state other than the intended one) is a set-up result, not a failure of the set-up: steps 6 and 7 still run, and the report says so.
   - **The ids read** are CREATED, in table order: an id refused at its ledger or its invoice is not read, and the report names it with its reason. MX's part A result for each id stands beside this row's, in this row's RESULTS.
   - **Then the sends, one at a time, each once, each checked before the next.** After each send, at once and before the next request: screenshot the whole Tally window (each `V` request but `Vm-u-pl.xml` and `Vb-book.xml` is a request this book has not been sent; a dialog: the prompt rule), then run the check named for it, recorded whole. **The next request is sent only if that check gave what it names.** Anything else is the STOP of the shape step (below), and nothing more is sent.

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
   - **Anything else is the STOP of the shape step:** a check that did not give what it names (a `STATUS`, an empty answer, `SAME: DIFFER`, a `MATCH` line other than `EQUAL`, MARKS DIFFER), a STOP of a script, an answer above the bound, a dialog, or a send with no answer. It is "a STOP at step 6", with the id `pl` where the send was one of `k1` to `k6`. Nothing more is sent: not the rest of the step, not the ten names, not a marks read beyond C8's. On MARKS DIFFER at `k0` or `k8` alone, `M-marks.xml` is sent once more as `k0r` or `k8r` and recorded first. Record each answer whole, screenshot the window, and report to the run coordinator. C8 holds, and the restore waits for the coordinator's word. The set-up's answers stand as recorded.
   - **If the STOP is a STOP of `wx.py`** on an answer `run.py` kept, with no dialog, the report gives `wx.py`'s line whole and the shape of the element it stopped on (its tag and its children's tags, no values). It is then the coordinator's to judge whether it is the reader's defect, as MX's was, or Tally's answer.
7. **The ledger reads. Only after step 6 has passed.** For each id of CREATED in table order, `n` the table's, four sends, each once, in this order, **each read by `wx.py` before the next request**:
   - `q<n>a`: `Vm-v-<id>.xml`, then `wx.py a-q<n>a-Vm-v-<id>.xml`;
   - `q<n>b`, the same file `Vm-v-<id>.xml`, then `wx.py a-q<n>b-Vm-v-<id>.xml` and `wx.py --same a-q<n>a-Vm-v-<id>.xml a-q<n>b-Vm-v-<id>.xml`;
   - `w<n>a`: `Vw-v-<id>.xml`, then `wx.py a-w<n>a-Vw-v-<id>.xml`;
   - `w<n>b`, the same file `Vw-v-<id>.xml`, then `wx.py a-w<n>b-Vw-v-<id>.xml` and `wx.py --same a-w<n>a-Vw-v-<id>.xml a-w<n>b-Vw-v-<id>.xml`;
   - then, before the next id: `wx.py --match <id> movement a-q<n>a-Vm-v-<id>.xml a-k7-Vb-book.xml` and `wx.py --match <id> window a-w<n>a-Vw-v-<id>.xml a-k7-Vb-book.xml`, one line each.

   Screenshot the whole Tally window after each `q<n>a` and `w<n>a`, before its `wx.py` read. A dialog: the prompt rule. An `EMPTY ANSWER` under the exception takes the exception's own steps (keep, screenshot, one `GET /status`, then `wx.py`) right after that send, before the next request; an empty `q<n>a` or `w<n>a` is followed by its `b` send, as for any other answer.
   - After the last id: `k9`: `Vb-book.xml`, the file `k7` sent, then `wx.py a-k9-Vb-book.xml` and `wx.py --same a-k7-Vb-book.xml a-k9-Vb-book.xml`; then `k10`: `M-marks.xml`, then `count20.py --same-row a-k8-M-marks.xml a-k10-M-marks.xml`. EXPECT `SAME: EQUAL` and SAME MARKS: step 7 only reads. On MARKS DIFFER: send `M-marks.xml` once more as `k10r` and record it; send nothing else, and STOP. If `k9` is not equal to `k7`, record both: every `--match` line was judged against `k7`, and the report says that the book-wide read changed during step 7.
   - **An answer `wx.py` reads never stops the row,** whatever it holds: each is a result, recorded in full. That includes an `EMPTY ANSWER` under the exception, a `STATUS`, `SAME: DIFFER`, and every `MATCH` line. `wx.py --match` exits 3 on a line other than `EQUAL`: that is a result, not a STOP. Only its exit 1 is a STOP. A STOP of `wx.py` (an answer it cannot read, or a failed send), a dialog and a send with no answer are STOPs, as everywhere, and the `wx.py` read after each send finds it before the next request. Each is "a STOP at step 7, id X". Nothing more is sent, C8 holds, and the answers before it stand as recorded. A STOP of `wx.py` is reported as step 6 says.
8. **Restore** BK by C6, with `restore-plan.txt` written first. MX's step 10 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace). List the data folder before and after, as MX did.
   - Then `s9`: every file of s2, in s2's order, and compare whole:
     - `count20.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml` passes;
     - each N read 0 rows;
     - B-bills holds no group of the eleven names;
     - the snapshot shows "no row" for each;
     - each answer's sha256 equals s2's.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The coordinator decides.
9. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering. Screenshot the window, send it and the answers to the run coordinator, and ask before anything else is pressed or sent. Say in the report to the coordinator what the book then holds. What the branch says about it is the STOP rule of "The sends": only "a STOP at step N", with the id where one applies.

## Record
- **RESULTS has a section for the set-up and one for the voucher reads,** each with its own confidence line, its own deviations and its own "not measured". For each id, the voucher reads' section sets MX's part A result beside this row's.
- Each answer's bytes and sha256, its headers file whole, and each script's lines, whole.
- `run.py`'s log for each label, and how long each read took, except for the send of a STOP the STOP rule names.
- Every `--before-write`, `--same-row`, `--import`, `--before`, `--names`, `--same` and `--match` output, and every `rf.py` and `wx.py` output, whole.
- The window after each import, and after each shot named in steps 6 and 7, listed by name, bytes and sha256.
- Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- **For a STOP the STOP rule of "The sends" names** (a wedge, an exit of Tally, a dialog), none of that send's detail goes on the branch: not its screenshots, headers file, log line, timing, or the request that preceded it. All of that goes to the run coordinator by the private channel, and the branch says only "a STOP at step N, id X".
- The BK file and the restore, as C5 and C6 ask, with the data folder listed before and after.
- The release on Tally's screen, and the Gateway's date and period.
- CREATED, as step 4 writes it, and step 6's check of the set-up, line by line.
- For each ledger: whether Tally created it, refused it, or refused its invoice, with the `LINEERROR` text if any; its name list and its state at step 4; its two pairs of voucher reads, their `--same` lines and their two `MATCH` lines, or why it was not read.

**Staging the evidence:** the check for 10 or more digits sets aside this book's registered GUID matched whole, and nothing else. Any other hit waits for the run coordinator. The voucher answers hold each voucher's `VOUCHERKEY`, a run of more than ten digits, as CV's census answers did. They are Tally's own, in the answers only. No script prints them, and the RESULTS never quote them.

## Expected
Estimates, not predictions.
- **s2:** as the Book and step 2 say; `FOLD: CLEAR`; `NAMES: EQUAL`.
- **Step 3:** `IMPORT: CREATED ONE` for each of the 22, as in MX.
- **Step 4:** for each ledger, S one row, bill-wise Yes; B one group; `NAMES: EQUAL`, as in MX.
- **Step 6:**
  - `a-k1` and `a-k4` each `ANSWER: VOUCHERS 1`, the control's invoice; `a-k1`'s party line with an amount-only bill allocation, as master's movement capture shows; `a-k4`'s with a named bill, as master's window capture shows;
  - `a-k7`, the book-wide read: about 1,800,000 B (above), 22 vouchers, each dated 20260801;
  - `k1` and `k4` `EQUAL` against it (§11c.6 read plain names this way on two other books);
  - `k2` equal to `k1` and `k5` to `k4`, as CV's census with the variable added;
  - `k3` equal to `k1`, and `k6` to `k4`, if the voucher reads evaluate `##LedgerName` in `CHILDOF` as CV's census did. Each differs from that census in its `FETCH` list and collection name only, but that has not been observed, so neither outcome is predicted.
- **Step 7:** as the control, if each name in the variable acts as it did for the census in CV and for the report in MX. No stronger estimate.
- **s9:** equal to s2 and s1.

## Decides
Each is PARTIAL: one book, one release, one sitting, master's request templates sent by curl, without the reads Bridge sends around them, and without the build's own parse.

**The set-up** decides nothing new: it is MX's, and MX's part A results stand. A refusal or a state other than MX's here is recorded beside MX's, and the coordinator decides what it means.

**The shape step:**
- **It passes:** on this book and release, each data-carrying ledger-scoped read in the `##LedgerName` form gave master's bytes for the control, and master's form held what the book-wide read of the day holds for that ledger. The ten are then measured.
- **It does not pass:** that form is not master's read on this book and release, for the read that differed; nothing in this row measures the ten through it. Record what differed. `LedgerScopeName` stays as it is for those reads. (A STOP of `wx.py` decides nothing about Tally until the coordinator has judged it, as step 6 says.)

**For each name of CREATED,** for each of the two reads, one of these:
- **`MATCH: EQUAL`, the pair `SAME: EQUAL`:** on this book and release, that read in the `##LedgerName` form read that ledger's vouchers as the book-wide read shows them. With CV and MX, that is evidence that the read can take that name as data.
- **`MATCH: DIFFER`:** the read does not give that ledger's vouchers as the book shows them; the line names what differs. The form is not admitted for that name in that read.
- **`EMPTY ANSWER` or `STATUS`:** the form does not read that name's ledger; not admitted.
- **`NOT JUDGED`:** the book-wide read did not show that id's invoice as the set-up made it; recorded, and the coordinator decides.
- **`SAME: DIFFER`:** record both; master would not plan on an answer that changes between two sends.

**If the shape step passes and every name of CREATED is `EQUAL` in both reads:** `ledger_movement` and the `outstandings` party detail could send their ledger-scoped reads with the name as the static variable, for every name the static-variable type admits, on the evidence of one book and release. The build of that change is #2480's and #485's, and the choice is the maintainers'.

**What these answers are for.** They inform the static-variable read's own type on #2480 and the scoped reads of #485. They become fixtures only if the manifest admits them.

## Code admits the trigger
Lines are at master 812d39bb.
- **The build refuses these names before any send.** `LedgerScopeName::new` (`src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65`) refuses any character outside ASCII letters, digits and a single space (`:74`) and the words of `FORMULA_WORDS` (`:30`). `admitted_party` (`src-tauri/src/agent_party_bills.rs:26`) maps each refusal to `party_name_not_admitted`, and `scope_name_of` (`src-tauri/src/agent_movement.rs:99`) maps it to the whole book's window. So no build reaches this row's reads for the ten, and this row sends by curl.
- **The build never sends the `##LedgerName` form.** `VoucherScope::collection` (`src-tauri/src/agent_read_profiles.rs:279`) writes the name itself into `CHILDOF`, escaped once (`:286`). Its users are the census (`:258`) and `render_agent_ledger_vouchers` (`:367`), whose two field lists are `LedgerVoucherFields` (`:340`).
- **The movement read asks for no bill field.** `AGENT_MOVEMENT_FETCH` (`src-tauri/src/agent_read_profiles.rs:96`) names no `BILLALLOCATIONS` field, and Tally's answer still lists the party line's allocation with its amount alone (master's movement capture above). That is the shape MX's reader did not admit.

## Not in this row
- ComplyEaze Bridge's own transport, or any Bridge build;
- MX's part A reads (`Ledger Outstandings`), whose results are MX's;
- any of the ten names inside `CHILDOF` itself (the formula position), which this row never sends;
- the reads over more than one day, with an AlterID span, or on a book of real size; the census in this form for these names (CV's names were one mark each);
- a ledger with more than one voucher or bill, a settled or part-settled bill, a voucher with two lines on the ledger, a note or a voucher after the invoice;
- another to-date, book or release; a book with GST on; Educational mode.
