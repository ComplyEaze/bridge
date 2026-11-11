# Row 2480-SK: does each scoped read tell a ledger name from another ledger that shares its letters-and-digits key?

Shape discovery, as rows 2346-PB, 2480-NM, 2480-NS, 2480-NC, 2480-NX, 2480-CV, 2480-MX and 2480-MW: the requests are written for the lab, not requests the build sends, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2480 (0.9.0) and #485. Written 11 Nov 2026. Lines are at master 7a3c9e49. Confidence of everything here until the row runs: unverified.

## Why this row
Every row from 2480-NS to 2480-MW read names whose letters-and-digits key (`letters_and_digits_key`, `src-tauri/crates/bridge-tally-core/src/master_binding.rs:1982`) no other ledger of the book shared. Each comment on #2480 says so, and the static-variable type the issue builds is to keep a one-stem condition for that reason: admit a name outside `LedgerScopeName`'s alphabet only when exactly one ledger of the catalogue has its key. No row has measured a book where two ledgers share a key. This row makes five such pairs and reads each name in each of the four forms of the scoped read:
- `Ledger Outstandings` with the name as its `LedgerName` static variable (rows NS to MX);
- the ledger census with `<CHILDOF>##LedgerName</CHILDOF>` and the name in the variable (row CV);
- the movement read and the window read in that form (row MW).

Either answer settles something for the build:
- **each name reads its own ledger and never its sibling's, in every form:** the maintainers may narrow the read-side condition for these marks (the write side keeps #708's `ledger_has_folded_twin` refusal whatever this row shows);
- **any read holds the sibling's voucher or bill, or lacks its own:** the condition is measured as needed for that form and mark, and the build keeps the typed refusal on evidence.

Row 2480-NM showed that Tally can take a `Create` of a name close to an existing ledger's as an alteration of that ledger. So the set-up may itself give the answer for a pair, before any read: that is a result of this row, recorded as such (step 3).

## Question
For each pair `(a, b)` below, whose names share master's key:
1. **The set-up.** Does Tally create `b` as a second ledger beside `a`, with both names as sent, each its own GUID, in the catalogue master's build reads? Or does it alter `a`, or refuse `b`?
2. **The reads.** Where both exist with one invoice each: does each of the four forms, sent for `a` and for `b`, hold that name's own invoice (or bill) and not its sibling's, twice byte-equal?

The control `pl` (row 2480-MX's control, byte for byte) has a key of its own. Its reads are the shape step, sent after the set-up and before any pair's name is read. Its five reads that rows MX and MW kept answers for must give those bytes when every pair's result is BOTH; otherwise they are recorded, not required (step 6, (*)).

**The one variable** is the pair: two names that differ by marks alone, with the forms and requests fixed.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` (byte-equal to MW's) applies in full. Where it and this sheet differ, the stricter one holds.

- **Pinned by GUID.** `count20.py`, `nm.py`, `st.py`, `wx.py` and `sk.py` each hold BRIDGE PILOT LAB's GUID as `BOOK`, and each STOPs on a row with another (`wx.py` on a voucher GUID that does not begin with it; `sk.py` on a catalogue row whose company GUID is not it). `cv.py` reads census answers, which carry voucher GUIDs only; `rf.py` reads import answers. The marks read beside each import is pinned.
- **Its state:** ALTVCHID 68 and ALTMSTID 245, the marks MW's restore left (its `s9`, equal to its `s1`). Other marks at s1: stop before BK, with nothing sent.
- **It holds none of the eleven ledgers and none of the numbers below.** Step 2 checks this; a row there is a STOP. `st.py --before` also checks that no ledger of the book shares a key with any of the eleven names.
- **The control's two imports are MX's files, byte for byte** (`LM-pl.xml`, `I-pl.xml`, number `BP/26-27/0281`, `REMOTEID` tail `000000000e81`), sent in MX and in MW, and taken out by each row's restore. Under C3, a file that writes is sent once in an item's life: this row is its own item, and sends each at most once.
- **The refused-import exception (NX's, MX's and MW's), and the alteration rule** (both below) hold only with the run coordinator's word, given when the row is queued.
- **The date and the period:** read the Gateway's current date and period at step 1. If the date is before 1-Aug-2026, or the period is not 1-Apr-26 to 31-Mar-27: stop before BK. **Do not change Tally's current date or period.**
- **BK is this row's way back.** Taken at step 1, before anything is sent, and restored at step 8 by C6. The restore runs on every path from step 3 on that is not a STOP. After any STOP, at any step, C8 holds, and the restore waits for the run coordinator's word.
- Write down the release shown on Tally's screen.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6).

**The eleven ledgers** (`names.tsv`: `id`, invoice number, name). Each is created by `LM-<id>.xml` and given its one open bill by `I-<id>.xml`.

| n | id | number | name | the pair, and how its second name differs |
| --- | --- | --- | --- | --- |
| 1 | `pl` | 0281 | `BRIDGE Mx Plain` | the control: MX's, a key of its own |
| 2 | `ka` | 0292 | `BRIDGE Sk Ak Am` | pair `k`: a comma, the plain name first |
| 3 | `kb` | 0293 | `BRIDGE Sk Ak, Am` | |
| 4 | `ra` | 0294 | `BRIDGE Sk Bk, Bm` | pair `r`: a comma, the marked name first |
| 5 | `rb` | 0295 | `BRIDGE Sk Bk Bm` | |
| 6 | `qa` | 0296 | `BRIDGE Sk Ck Cm` | pair `q`: an apostrophe |
| 7 | `qb` | 0297 | `BRIDGE Sk Ck' Cm` | |
| 8 | `ua` | 0298 | `BRIDGE Sk Dk Dm` | pair `u`: an underscore in place of the space |
| 9 | `ub` | 0299 | `BRIDGE Sk Dk_Dm` | |
| 10 | `ea` | 0300 | `BRIDGE Sk EkEm` | pair `e`: a space alone |
| 11 | `eb` | 0301 | `BRIDGE Sk Ek Em` | |

**Why these pairs.** Every mark is one rows NS, NC and NX created and read under a stem of its own. No pair differs by case alone (§9.4e: a `Create` in another case altered the ledger), by a trailing line break (§9.4e measures it), or by one of the five marks row NM saw answered as an alteration against a hyphen (`.`, `&`, `/`, `(`, `)`): those would spend the set-up on answers already recorded. The comma is sent in both orders, as §9.4e asks of a twin. `rb`, `qa`, `ua`, `ka`, `ea` and `eb` hold letters and single spaces only, so master's `LedgerScopeName` admits them today; `kb`, `ra`, `qb` and `ub` it refuses.

**Master's keys, not a copy.** `keys-master.txt` is the output of master's own `letters_and_digits_key` on every name of rows NM, NS, NC, NX, MX and this row: `keys-test.rs`, copied into `src-tauri/crates/bridge-tally-core/tests/` of a checkout of master at 7a3c9e49, run by `cargo +1.96.0 test -p bridge-tally-core --test sk_keys` and never committed to master. It writes a key's non-ASCII characters (the Devanagari names of rows NM, NS and NC) as Rust's `\u{..}` escapes, one to one, so that equal keys stay equal and different keys different. `gen-sk.py` pins that file's sha256, checks that its lines are exactly those rows' names, and asserts from master's keys alone: each pair shares one key; no two pairs share one; no pair's key is any earlier row's; the control's key is MX's control's alone.

Every name reaches Tally only as escaped XML text: in the reads, as the text of the `LedgerName` static variable; in the imports, as master's import shapes carry a name. **No pair's name is ever sent inside a formula.** The only requests with a name inside `CHILDOF` are `Vm-u-pl.xml`, `Vw-u-pl.xml` and `C-u-pl.xml`, the control's, whose name master's type admits.

## The requests
All made by `gen-sk.py` (Files). Requests are UTF-16LE with a BOM, as sent.

| file | what it is |
| --- | --- |
| `LM-<id>.xml`, `I-<id>.xml`, `N-<number>.xml` | MX's control files with the name (number, `REMOTEID` tail `000000000ec2` to `000000000ecb`) turned into the id's; the control's are MX's |
| `L-lo-<id>-20260801.xml` | `Ledger Outstandings`, the name as the `LedgerName` static variable: MX's `L-lo-pl` with the name turned |
| `C-u-pl.xml`, `C-v-<id>.xml`, `C-book.xml` | row CV's census requests (window 1-Apr-2026 to 1-Aug-2026, AlterID span (0, 8192]): master's ledger census of the control with the name in `CHILDOF` and the `##LedgerName` form, each CV's with CV's control name `BRIDGE Nx Plain` turned into `BRIDGE Mx Plain`; the `##LedgerName` form for each id; and the book-wide census (CV's, byte for byte) |
| `Vm-u-pl.xml`, `Vw-u-pl.xml`, `Vm-v-<id>.xml`, `Vw-v-<id>.xml`, `Vb-book.xml` | row MW's voucher reads of the one day 1-Aug-2026: master's movement and window reads of the control, the `##LedgerName` form for each id, and the book-wide read; the control's and `Vb-book.xml` MW's, byte for byte |
| `G-cat.xml` | master's standard ledger catalogue (`standard_ledger_catalog_read`, `src-tauri/src/agent_read_profiles.rs:642`): the request the lab sent BRIDGE SHAPE LAB, which `src-tauri/src/agent_voucher_groups_tests.rs:100` pins to that builder, with the company name alone changed. It is the read the one-stem condition would be checked against: each ledger's `NAME` attribute, its `LANGUAGENAME.LIST` names and its `GUID`. It has not been sent to this book; its first send is at step 2, before any write. |
| `M-marks.xml`, `S-snap-20260401-20260801.xml`, `B-bills.xml` | NM's, byte for byte, as in every row since |

**Sizes.** `Vb-book.xml`'s answer is MW's: the book's eleven vouchers of 1-Aug-2026 and this row's eleven invoices, as MW's 22 (MW's `k7` was under the 2,000,000 B bound). The catalogue's answer is not estimated: master's capture answered about 1,200 B a ledger on a small book. An answer above 2,000,000 B is `run.py`'s stop.

## The sends and the wedge rule
`run.py` is NM's, byte for byte: one request at a time, each answer's headers kept by curl -D in `a-<label>-<file>.headers`, and it never abandons a request.
- **The wedge rule.** If any send has not answered **5 minutes** after it began: press nothing, do not end curl, send nothing more (not even `GET /status`), screenshot the whole Tally window, and report to the run coordinator (C9). Nothing is re-sent on this row's own judgement.
- **The stops.** `run.py`'s stops are failed requests, never "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 B. So are the STOPs (exit 1) of `count20.py`, `nm.py`, `st.py`, `rf.py`, `cv.py`, `wx.py` and `sk.py`. Each stops the row there, with the exceptions of step 3 (a pair's result) and the empty answer below.
- **The empty-answer exception for a `Vm-v-` or `Vw-v-` answer at step 7,** MW's, unchanged, read by `wx.py` as MW's sheet says. It holds only with the run coordinator's word given when the row is queued; without it such an answer is a STOP. It is not a STOP only if its headers file shows HTTP status 200 and `CONTENT-LENGTH: 0`. Then, before the next request: keep the headers file and the log line, screenshot the whole Tally window (a dialog: STOP), send `GET /status` once (C2c's command; not 200: STOP), and run `wx.py` on that answer. Every name read there has a ledger, so an empty answer is not expected; under the exception it is that id's result for that form, and its `JUDGE` line says `EMPTY ANSWER`. An empty answer to an `L-lo-` or `C-v-` request (`nm.py` and `cv.py` read no empty answer), at any other step, or to any other request is a STOP.
- **What the branch and any public text say about a STOP of this kind.** If a send wedges, Tally exits or stops answering, or a dialog the prompt rule names comes up, the row's RESULTS and any public comment say only "a STOP at step N, id X" (or "a STOP at step N" where no id applies: steps 1, 2, 4, 5, 8 and 9, the marks reads `k0`, `k9` and `k11`, and the book-wide reads `k8` and `k10`; a write's own marks reads, and a pair's `s7<b>`, take that write's or that `b`'s id). The request before it, the screens, the timings and anything else that would show how to cause it go to the run coordinator by the private channel. The coordinator decides what, if anything, is published later.
- Check each file's sha256 against `SHA256.txt` before its send. No file in `mx/`, `cv/`, `fx/`, `nm/`, `ns/`, `nc/` or `nx/` is ever an argument to `run.py`.
- Close no Bridge process, and call no Bridge tool.

**Which script reads which answer**
- **`count20.py`**, **`nm.py`**, **`st.py`**, **`rf.py`** and **`wx.py`**: MW's, byte for byte, each with the modes its header names. `nm.py` and `st.py` take the eleven names from this folder's `names.tsv`; `wx.py --match` too.
- **`cv.py`**: CV's, byte for byte. `--own ID ANSWER BEFORE AFTER INVOICED` takes, for its last argument, the ids whose invoice was created (INVOICED, step 4), as its header says.
- **`sk.py`** (new): `--pair`, `--names`, `--altered`, `--ref` and `--judge`, each whole in its header. It parses no voucher, bill or census answer itself: it loads the readers above and calls their own parse. It never prints a `VOUCHERKEY` or a GUID. `--judge` prints one line a name a form: `OWN`, `SIBLING` (`YES` or `NO`; `none` for the control; `NOT IN THE BOOK` where the sibling's invoice is not in the book-wide read, so nothing can be matched) and `OTHERS`, matched by `VOUCHERKEY` (movement, window), by GUID (census) and by bill number (outstandings). Exit 0 for `OWN YES`, `SIBLING NO` or `none`, `OTHERS 0`; exit 3 for any other line, a result and not a STOP; exit 1, its STOP.

## Files
**Byte-equal to MW's file of the same name** (`lab/2480-mw-sheet` at a7bf1de3, folder `lab/2480-mw/`): `.gitattributes`, `LAB-COMMON.md`, `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py`, `wx.py`, `M-marks.xml`, `B-bills.xml`, `S-snap-20260401-20260801.xml`; and in `mx/` (sources, never sent) `LM-pl.xml`, `I-pl.xml`, `N-0281.xml`, `L-lo-pl-20260801.xml`, `Vb-book.xml`, `Vm-u-pl.xml`, `Vm-v-pl.xml`, `Vw-u-pl.xml`, `Vw-v-pl.xml`, and MW's `names.tsv` (MX's names). `nm/`, `ns/`, `nc/`, `nx/`: those rows' `names.tsv`, as MW keeps them.

**Byte-equal to CV's** (`lab/2480-cv-sheet` at 9de0dee1, folder `lab/2480-cv/`): `cv.py`; and in `cv/` (sources, never sent) `C-book.xml`, `C-u-pl.xml`, `C-v-pl.xml`.

**Master's** (at 7a3c9e49), in `fx/`, never sent: `shape-lab-standard-ledger-catalogue.request.utf16le.xml`, byte-equal to `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/` of that name (its PROVENANCE lists the same sha256).

**New for this row:** this file, `names.tsv`, `gen-sk.py` and its 83 files, `sk.py`, `keys-test.rs`, `keys-master.txt`, `SHA256.txt`.

**`gen-sk.py`** checks the sha256 of every source against the sheet it came from (above) before it writes anything. It writes the control's files as MX's and MW's, byte for byte, and every other request from the control's by swapping the name (and, for an invoice, its number and `REMOTEID` tail) and nothing else. Each swap's count is asserted, and swapped back, each file gives its source byte for byte. MX's invoice checks (`gen-mx.py`'s `check_invoice` and `check_ledger`) are run on every import. It asserts the names as the Book says, from master's keys.

`SHA256.txt` lists every file with its size (`sha256  size  path`, MW's form).

**Tried before this sheet was sent:**
- `gen-sk.py`, run in this folder: its 83 files, the control's byte-equal to MX's and MW's, every check passing.
- **`sk.py`**, on answers rows MX and MW kept, each read with the book whose GUID it holds, in a copy whose `names.tsv` gave this row's ids to MX's names (so `ka` was MX's `ga`, `kb` MX's `ss`):
  - `--judge` movement and window on MW's own reads of `ga`, `ss`, `pl` and `al`: `OWN YES`, `SIBLING NO` (or `none`), `OTHERS 0`, exit 0; on MW's read of `ss` judged as `ka`: `OWN NO, SIBLING YES`, exit 3; on MW's book-wide read judged as `ka`: `OWN YES, SIBLING YES, OTHERS 20`, exit 3; against MW's control read as the book: `NOT JUDGED`, exit 3; on a marks answer: a STOP of `wx.py`.
  - `--judge` outstandings on MX's `L-lo` answers: the same three outcomes. `--judge` census on CV's control census answer: `OWN YES, SIBLING none`. That last match shows that voucher GUIDs repeat across sittings when a restore takes the book back to the same marks: a match means something only between answers of one sitting, which is how step 7 uses it.
  - `--ref` on MW's `k3` and `k6` and MX's `r1a`: `EQUAL`; on another request's answer: its STOP.
  - `--altered` on row NM's answers: `ALTERED ONE` on two of its five alterations; `NOT ALTERED ONE` (exit 3) on its `CREATED` 1 answer.
  - `--pair` and `--names` on master's catalogue capture (`shape-lab-standard-ledger-catalogue.utf16le.xml`), in a copy whose `BOOK` was that capture's own book and whose names were that book's ledgers: `BOTH`, exit 0; with one name changed to a name the book does not hold, `no row` and `NOT BOTH`, exit 3; ids out of order, its STOP. In the copy pinned to this book, the capture is a STOP (a company GUID not the book's). No row of that capture has a name list other than its `NAME`, so the `DIFFER` path met no real answer.
- **Not tried:** any send to Tally; `G-cat.xml` against this book; `sk.py --judge` census against a book-wide read of the same sitting (no row has sent both).

## Steps
Run each label as `python3 -I run.py http://127.0.0.1:9001 <label> <files>`, the Gateway on 9001 as C2 finds it (written `URL` below). (The lab PC has no `python3` command; there, `python -I` with Python 3 is the same command.) The prompt rule, the wait rule and the path rule are NM's. `n` is an id's place in the table.

| step | labels |
| --- | --- |
| 1 | `s1` |
| 2 | `s2` |
| 3 | `s4<id>`, `s5<id>`, with `m` after each for its marks read, and `x` for the marks read after a refusal or an alteration; `s7<b>` for a pair's catalogue read |
| 4 | `s6…` |
| 5 | `m1` |
| 6 | `k0` to `k9` |
| 7 | `r<n>a`, `r<n>b`, `c<n>a`, `c<n>b`, `q<n>a`, `q<n>b`, `w<n>a`, `w<n>b`, then `k10`, `k11` |
| 8 | `s9` |
| repeats | `m1r`, `k0r`, `k9r`, `k11r`: a marks read sent once more after MARKS DIFFER |

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named. For any other prompt, message or dialog: copy it word for word and screenshot it first; then answer No, or Esc if it offers no No; press nothing more and send nothing. That prompt is this row's result, and C8 holds. What the branch says about it is the STOP rule above.

**Each write, the same way, by hand, one label a command** (MW's write recipe, with (e) and (f) added; used only in step 3):
- (a) `run.py URL <label>m M-marks.xml`, then `python3 -I count20.py --before-write a-s1-M-marks.xml a-<label>m-M-marks.xml`.
- (b) Only if that passes, and no key was pressed in Tally since: `run.py URL <label> <file>`, the import alone.
- (c) Screenshot the whole window before the next request (C3). A dialog: the prompt rule.
- (d) `python3 -I st.py --import a-<label>-<file>`, recorded whole. **On exit 0 (`CREATED ONE`), go on with the next write of the recipe.**
- (e) **The alteration rule, only for a pair's second ledger (`s4<b>`), only if `st.py` gave exit 1 and `run.py` in (b) gave exit 0:** `python3 -I sk.py --altered a-s4<b>-LM-<b>.xml`, recorded whole. On exit 0 (`ALTERED ONE`: `CREATED` 0, `ALTERED` 1, every other counter 0, no line error): `run.py URL s4<b>x M-marks.xml`, then `count20.py --before-write a-s1-M-marks.xml a-s4<b>x-M-marks.xml` (the book is still the one loaded) and `count20.py --same-row a-s4<b>m-M-marks.xml a-s4<b>x-M-marks.xml`, each recorded whole. If `--before-write` passes, the pair's result is **ALTERED**: Tally took the two names as one ledger on import. The `--same-row` line is a record, not a check (row NM saw each alteration move the master mark by one). `b`'s invoice is not sent, neither name of the pair is read, and the pair is out of steps 6 and 7. If `--before-write` does not pass: the STOP of the set-up. On exit 3 of `sk.py`, go on to (f).
- (f) **The refused-import exception, for any import of a pair** (not the control's), only if `st.py` gave exit 1, `run.py` gave exit 0, and (e) did not apply or gave exit 3: `python3 -I rf.py a-<label>-<file>`, recorded whole. On exit 0 (`REFUSED`): `run.py URL <label>x M-marks.xml`, then `count20.py --same-row a-<label>m-M-marks.xml a-<label>x-M-marks.xml`. On SAME MARKS, that import is **REFUSED**, the pair's result: no later import of the pair is sent, and the pair is out of steps 6 and 7. On anything else, or on exit 1 of `rf.py`: the STOP of the set-up.
- On any exit other than 0 of `run.py` in (b): the STOP of the set-up; (e) and (f) are not run.

The set-up is not run as a loop that decides for itself. If a script sends it, it ends at the first exit other than 0 of `run.py` or `count20.py`, and at the first exit of `st.py`, `sk.py` or `rf.py` that the recipe does not name as the next step.

### The set-up

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down (Book).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2480-sk\backup-1` (Tally may show it as `C:\lab\2480-Sk\backup-1`). MW's step 1 is the path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0281.xml N-0292.xml N-0293.xml N-0294.xml N-0295.xml N-0296.xml N-0297.xml N-0298.xml N-0299.xml N-0300.xml N-0301.xml B-bills.xml S-snap-20260401-20260801.xml C-book.xml G-cat.xml M-marks.xml`.
   - EXPECT each N read 0 rows (`count20.py`), `nm.py` on B-bills no group for any of the eleven names, and on the snapshot "no row" for each. A row or a group: stop before step 3, each recorded whole.
   - `st.py --before a-s2-S-snap-20260401-20260801.xml`: EXPECT `FOLD: CLEAR`. A STOP: stop before step 3.
   - `st.py --names a-s2-S-snap-20260401-20260801.xml`, recorded whole.
   - `cv.py a-s2-C-book.xml`: EXPECT `ANSWER: VOUCHERS <n>`, the book before this row's invoices. Anything else: stop before step 3.
   - `sk.py --names a-s2-G-cat.xml`, recorded whole. EXPECT "no row" for each of the eleven. A row, or a STOP of `sk.py`: stop before step 3. **`G-cat.xml` is a request this book has not been sent**; a dialog is the prompt rule.
3. **The set-up,** in the order of the table, each write by the recipe:
   - **The control:** `s4pl` (`LM-pl.xml`), then `s5pl` (`I-pl.xml`). A refusal or an alteration of either is the STOP of the set-up: without the control, no pair can be judged.
   - **Then each pair `(a, b)`**, `k`, `r`, `q`, `u`, `e` in that order, four writes and one read:
     1. `s4<a>` (`LM-<a>.xml`), then `s5<a>` (`I-<a>.xml`);
     2. `s4<b>` (`LM-<b>.xml`);
     3. **Only if `s4<b>` gave `CREATED ONE`:** `s7<b>`: `G-cat.xml`, then `python3 -I sk.py --pair <a> <b> a-s7<b>-G-cat.xml`, recorded whole. On exit 0 (`PAIR <a> <b>: BOTH`: each name as sent in one row of its own, its name list that name alone, two GUIDs), go on. On exit 3 (`NOT BOTH`), the pair's result is **CREATED, NOT BOTH**, recorded whole: for example, `a`'s own name changed under `b`'s `CREATED` 1, as row NM saw a name change under an alteration. `b`'s invoice is not sent, the pair is not read, and this is not a STOP;
     4. **Only after `BOTH`:** `s5<b>` (`I-<b>.xml`).
   - **The pair's result** is one of: **BOTH** (all four imports `CREATED ONE` and `BOTH`); **ALTERED** (recipe (e)); **REFUSED** at a label (recipe (f)); **CREATED, NOT BOTH** (3 above). Write it down before the next pair starts.
   - **The STOP of the set-up.** On any answer that the recipe does not name: an exit 1 of `st.py` that (e) and (f) do not turn into a pair's result; an alteration of anything but a pair's second ledger; a refusal whose marks after it differ; an exit other than 0 of `run.py` on an import or on `G-cat.xml`; a STOP of `sk.py`; a dialog. At a STOP: send nothing more; screenshot the whole window; record the answer, if there is one, whole; and report to the run coordinator. C8 holds. Say in the report what the book then holds.
4. **The book-wide reads and the state.** `s6m`: `M-marks.xml`. Then `s6sa`, `s6sb`: `S-snap-20260401-20260801.xml`; `s6ba`, `s6bb`: `B-bills.xml`; `s6ga`, `s6gb`: `G-cat.xml`; `s6ka`, `s6kb`: `C-book.xml`; each the same file twice.
   - `nm.py` on `a-s6sa-…` and `a-s6ba-…`; `nm.py --same` on each of the four pairs; `st.py --names a-s6sa-S-snap-20260401-20260801.xml`; `sk.py --names a-s6ga-G-cat.xml`; `cv.py a-s6ka-C-book.xml`. Each recorded whole.
   - **INVOICED** is the list of ids whose `I-` import gave `CREATED ONE`, in table order. **READ** is `pl`, then both ids of each pair whose result is BOTH and whose two `sk.py --names` lines at `s6ga` are `EQUAL`, in table order. Write both down. A pair whose result was BOTH but whose `s6ga` lines are not both `EQUAL` is out of READ, recorded as **BOTH, CHANGED BY STEP 4**.
   - **The intended state** for each id of INVOICED: S a row, bill-wise Yes, closing the invoice's amount; B one group, `BP/26-27/<its number>`. If not, the report says so for that id; an id of READ not in that state is a set-up result, recorded, and still read.
5. **`m1`:** `M-marks.xml`, then `count20.py --same-row a-s6m-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS. On MARKS DIFFER: send `M-marks.xml` once more as `m1r`, record it, send nothing else, and STOP.

### The reads

6. **The set-up whole, then the shape step, the control alone, before any pair's name is read.**
   - **Before `k0`,** write down that each holds, with the line of the notes that shows it: no STOP at steps 1 to 5; every send answered; `m1` SAME MARKS against `s6m`; every `--before-write` passed; `pl` in READ, in the intended state. If any does not hold, nothing of steps 6 and 7 is sent: the row goes to step 8. A pair's result other than BOTH is not a failure of the set-up.
   - **Then the sends, one at a time, each once, each checked before the next.** After each send, screenshot the whole Tally window (a dialog: the prompt rule), then run the checks named for it, recorded whole. **The next request is sent only if each check gave what it names.**

     | label | file | check, run before the next send | it must give |
     | --- | --- | --- | --- |
     | `k0` | `M-marks.xml` | `count20.py --same-row a-m1-M-marks.xml a-k0-M-marks.xml` | SAME MARKS |
     | `k1` | `Vm-u-pl.xml` | `wx.py a-k1-Vm-u-pl.xml`; `sk.py --ref a-k1-Vm-u-pl.xml` | `ANSWER: VOUCHERS 1`; `REF … EQUAL` (*) |
     | `k2` | `Vm-v-pl.xml` | `wx.py --same a-k1-Vm-u-pl.xml a-k2-Vm-v-pl.xml`; `sk.py --ref a-k2-Vm-v-pl.xml` | `SAME: EQUAL`; `REF … EQUAL` (*) |
     | `k3` | `Vw-u-pl.xml` | `wx.py a-k3-Vw-u-pl.xml`; `sk.py --ref a-k3-Vw-u-pl.xml` | `ANSWER: VOUCHERS 1`; `REF … EQUAL` (*) |
     | `k4` | `Vw-v-pl.xml` | `wx.py --same a-k3-Vw-u-pl.xml a-k4-Vw-v-pl.xml`; `sk.py --ref a-k4-Vw-v-pl.xml` | `SAME: EQUAL`; `REF … EQUAL` (*) |
     | `k5` | `L-lo-pl-20260801.xml` | `nm.py a-k5-L-lo-pl-20260801.xml`; `sk.py --ref a-k5-L-lo-pl-20260801.xml` | `ANSWER: ROWS 1 (0 On Account)`; `REF … EQUAL` (*) |
     | `k6` | `C-u-pl.xml` | `cv.py a-k6-C-u-pl.xml` | `ANSWER: VOUCHERS 1` |
     | `k7` | `C-v-pl.xml` | `cv.py --same a-k6-C-u-pl.xml a-k7-C-v-pl.xml` | `SAME: EQUAL` |
     | `k8` | `Vb-book.xml` | `wx.py a-k8-Vb-book.xml`; `wx.py --match pl movement a-k1-Vm-u-pl.xml a-k8-Vb-book.xml`; `wx.py --match pl window a-k3-Vw-u-pl.xml a-k8-Vb-book.xml`; `cv.py --own pl a-k6-C-u-pl.xml a-s2-C-book.xml a-s6ka-C-book.xml <INVOICED>` | `ANSWER: VOUCHERS <n>`; `MATCH pl movement: EQUAL`; `MATCH pl window: EQUAL`; `OWN: YES` |
     | `k9` | `M-marks.xml` | `count20.py --same-row a-k0-M-marks.xml a-k9-M-marks.xml` | SAME MARKS |

     (*) **The kept bytes.** `sk.py --ref` compares the answer with the one rows MX and MW kept for the same request: MW's `k1`, `k3`, `k4` and `k6`, and MX's `r1a`. Those answers were read with all eleven of MX's ledgers in the book, and an answer's header block counts objects of the book (row CV saw empty census answers differ only there). So **`REF … EQUAL` is required when every pair's result is BOTH**, the book then holding eleven new ledgers and eleven new invoices, as MX's and MW's did. When a pair's result is not BOTH, the book holds fewer, the `--ref` lines are run and recorded but are not required, and the shape step's gate is the rest of the table. `k6` to `k8` have no kept answer: no row has read the census of `BRIDGE Mx Plain`, and the book-wide reads hold this row's invoices. They are checked by the table alone.
   - **The shape step passes only if every check gave what it names**, every send answered, and no dialog came up. Anything else is the STOP of the shape step, "a STOP at step 6", with the id `pl` where the send was one of `k1` to `k7`: nothing more is sent, not a marks read beyond C8's. On MARKS DIFFER at `k0` or `k9` alone, `M-marks.xml` is sent once more as `k0r` or `k9r` and recorded first. A STOP of a reader on an answer `run.py` kept, with no dialog, is reported with the reader's line whole and the shape of the element it stopped on (tags, no values), for the coordinator to judge, as MW's step 6 says.
7. **The pair reads. Only after step 6 has passed.** For each id of READ in table order, `pl` first, eight sends, each once, in this order, **each read by its reader before the next request**:
   - `r<n>a`, `r<n>b`: `L-lo-<id>-20260801.xml`, then `nm.py` on each answer, and `nm.py --same` on the two;
   - `c<n>a`, `c<n>b`: `C-v-<id>.xml`, then `cv.py` on each, and `cv.py --same` on the two;
   - `q<n>a`, `q<n>b`: `Vm-v-<id>.xml`, then `wx.py` on each, and `wx.py --same` on the two;
   - `w<n>a`, `w<n>b`: `Vw-v-<id>.xml`, then `wx.py` on each, and `wx.py --same` on the two;
   - then, before the next id, each one line, recorded whole:
     - `nm.py --against <id> a-r<n>a-L-lo-<id>-20260801.xml a-s6ba-B-bills.xml a-s6sa-S-snap-20260401-20260801.xml`;
     - `cv.py --own <id> a-c<n>a-C-v-<id>.xml a-s2-C-book.xml a-s6ka-C-book.xml <INVOICED>`;
     - `wx.py --match <id> movement a-q<n>a-Vm-v-<id>.xml a-k8-Vb-book.xml` and `wx.py --match <id> window a-w<n>a-Vw-v-<id>.xml a-k8-Vb-book.xml`;
     - `sk.py --judge <id> outstandings a-r<n>a-L-lo-<id>-20260801.xml a-k8-Vb-book.xml`, and the same with `census a-c<n>a-C-v-<id>.xml`, `movement a-q<n>a-Vm-v-<id>.xml` and `window a-w<n>a-Vw-v-<id>.xml`: **the pair's judgement, one line a name a form.**

   Screenshot the whole Tally window after each `a` send, before its reader runs. A dialog: the prompt rule.
   - After the last id: `k10`: `Vb-book.xml`, then `wx.py a-k10-Vb-book.xml` and `wx.py --same a-k8-Vb-book.xml a-k10-Vb-book.xml`; then `k11`: `M-marks.xml`, then `count20.py --same-row a-k9-M-marks.xml a-k11-M-marks.xml`. EXPECT `SAME: EQUAL` and SAME MARKS. On MARKS DIFFER: send `M-marks.xml` once more as `k11r`, record it, send nothing else, and STOP. If `k10` is not equal to `k8`, record both: the report says the book-wide read changed during step 7.
   - **An answer a reader reads never stops the row,** whatever it holds: a `SIBLING YES`, an `OWN NO`, a `DIFFER`, a `STATUS`, an empty answer under the exception and every exit 3 are results, recorded in full. Only an exit 1 of a reader, a dialog and a send with no answer are STOPs: "a STOP at step 7, id X". Nothing more is sent, C8 holds, and the answers before it stand.
8. **Restore** BK by C6, with `restore-plan.txt` written first. MW's step 8 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace). List the data folder before and after, as MW did.
   - Then `s9`: every file of s2, in s2's order, and compare whole: `count20.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml` passes; each N read 0 rows; B-bills holds no group of the eleven names; the snapshot shows "no row" for each; `sk.py --names a-s9-G-cat.xml` "no row" for each; each answer's sha256 equals s2's.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The coordinator decides.
9. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering. Screenshot the window, send it and the answers to the run coordinator, and ask before anything else is pressed or sent. What the branch says about it is the STOP rule of "The sends".

## Record
- **RESULTS has a section for the set-up and one for the reads,** each with its own confidence line, deviations and "not measured".
- For each pair: its result at step 3, with each answer's counters whole, the `sk.py --pair` output, and, for an alteration, both marks lines.
- Each answer's bytes and sha256, its headers file whole, and each script's lines, whole. `run.py`'s log for each label, and how long each read took, except for the send of a STOP the STOP rule names.
- The window after each import and after each shot named in steps 6 and 7, by name, bytes and sha256. Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- **For a STOP the STOP rule of "The sends" names,** none of that send's detail goes on the branch: all of it goes to the run coordinator by the private channel, and the branch says only "a STOP at step N, id X".
- The BK file and the restore, as C5 and C6 ask, with the data folder listed before and after. The release on Tally's screen, and the Gateway's date and period.
- INVOICED and READ, as step 4 writes them, and step 6's check of the set-up, line by line.
- For each id of READ: its four pairs of reads, their `--same` lines, its `--against`, `--own` and two `--match` lines, and its four `JUDGE` lines.

**Staging the evidence:** the check for 10 or more digits sets aside this book's registered GUID matched whole, and nothing else. Any other hit waits for the run coordinator. The voucher answers hold each voucher's `VOUCHERKEY`, a run of more than ten digits, as CV's and MW's did; the catalogue answer holds each ledger's GUID. They are Tally's own, in the answers only. No script prints a `VOUCHERKEY`. `cv.py` prints the voucher GUIDs of a census answer, as in row CV, each this book's GUID with a suffix; `sk.py` prints no GUID. The RESULTS quote neither.

## Expected
Estimates, not predictions.
- **s2:** as the Book and step 2 say; `FOLD: CLEAR`; the catalogue answered with this book's ledgers, none of the eleven.
- **Step 3:** `CREATED ONE` for the control's two imports and each pair's first two, as for every such name since NS. For each second ledger, no estimate: §9.4b measured a space accepted where a master carries a hyphen when a voucher names a ledger, and row NM saw five marks answered as an alteration against a hyphen. Whether a comma, an apostrophe, an underscore or a space alone is a second ledger or an alteration of the first has not been measured. Each outcome is a result.
- **Step 6:** the control's answers equal to the kept ones when every pair is BOTH, as MW's `k1` equalled MX's.
- **Step 7:** for each name of READ, no estimate. §9.4e measured that, where twins coexist, an import binds the exact name; no read has been measured against twins.
- **s9:** equal to s2 and s1.

## Decides
Each is PARTIAL: one book, one release, one sitting, master's request templates sent by curl, without the reads Bridge sends around them, and without the build's own parse.

**For each pair, at step 3:**
- **BOTH:** on this book and release, a gateway `Create` makes a second ledger whose name shares the first's key by that mark, both names kept as sent. The reads are then measured.
- **ALTERED:** Tally took that name as the first ledger's on import, as NM saw for its five marks. No two ledgers can share a key by that mark through the gateway, so the read-side condition has nothing to guard for it; the write side's refusal is confirmed as needed.
- **REFUSED:** Tally refused the import; recorded with its line error, and the coordinator decides what it means.
- **CREATED, NOT BOTH:** Tally counted a creation but the catalogue does not hold both names as sent: the most important set-up result, recorded whole for the coordinator.

**For each name of READ, for each form,** by its `JUDGE` line with the pair `SAME: EQUAL` and its own form's line (`--against`, `--own` or `--match`) beside it:
- **`OWN YES, SIBLING NO, OTHERS 0`, the pair equal, its own line `EQUAL` or `YES`:** on this book and release, that form told the name from its sibling.
- **`SIBLING YES`:** that form read the sibling's voucher or bill with or for this name. For that form and mark, the read needs the one-stem refusal: a name whose key another ledger shares must not reach it.
- **`OWN NO`:** that form did not read the name's own ledger. With `SIBLING YES`, it read the wrong one: the most important read result.
- **`NOT JUDGED`, `SIBLING NOT IN THE BOOK`, an empty answer or a `STATUS`:** recorded; the coordinator decides.

**If every pair is BOTH and every name of READ gives the first line in all four forms:** on one book and release, the scoped reads with the name as data tell these five marks from a sibling that shares the key. The maintainers may then narrow the read-side one-stem condition for these marks, on #2480 and #485. The write side keeps #708's refusal. The choice is the maintainers'.

**What these answers are for.** They inform the static-variable read's type on #2480 and the scoped reads of #485. They become fixtures only if the manifest admits them.

## Code admits the trigger
Lines are at master 7a3c9e49.
- **The build sends the `##LedgerName` form for no name, and no name of a pair but by curl here.** `LedgerScopeName::new` (`src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65`) admits only ASCII letters, digits and a single space (`:74`), and refuses the words of `FORMULA_WORDS` (`:30`). `VoucherScope::collection` (`src-tauri/src/agent_read_profiles.rs:279`) writes that name itself into `CHILDOF`; its users are the census (`:258`) and `render_agent_ledger_vouchers` (`:367`). `admitted_party` (`src-tauri/src/agent_party_bills.rs:26`) and `scope_name_of` (`src-tauri/src/agent_movement.rs:99`) map a refusal to `party_name_not_admitted` and to the whole book's window.
- **Master reaches six of these names today, in its own form.** `ka`, `rb`, `qa`, `ua`, `ea` and `eb` pass `LedgerScopeName`, so master's own scoped reads would send each in `CHILDOF` in a book that holds its sibling. This row sends master's form for the control alone; the `##LedgerName` form is the one it measures for every name.
- **The catalogue the one-stem condition would read.** `standard_ledger_catalog_read` (`src-tauri/src/agent_read_profiles.rs:642`) is the request `read_resolvable_ledgers` sends (`src-tauri/src/agent_import.rs:3030`) before `resolve_catalogue_ledger_or_refuse` (`src-tauri/src/agent_ledger_candidates.rs:173`) resolves a typed party. `G-cat.xml` is that request.

## Not in this row
- ComplyEaze Bridge's own transport, or any Bridge build;
- master's `CHILDOF` form for a pair's name (above), and any name inside `CHILDOF` but the control's;
- pairs by case, a trailing line break, an alias, or the five marks NM measured; three or more ledgers sharing one key; a pair made on Tally's own screens;
- the reads over more than one day, with another AlterID span, or on a book of real size; a ledger with more than one voucher or bill, or a settled bill;
- another to-date, book or release; a book with GST on; Educational mode.
