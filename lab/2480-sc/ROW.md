# Row 2480-SC: does master's own `CHILDOF` read tell a ledger from a sibling that shares its key, and how is an altered ledger read?

Shape discovery, as rows 2346-PB and 2480-NM to 2480-SK: the requests are written for the lab, not requests the build sends, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2480 (0.9.0) and #485. Written 11 Nov 2026. Lines are at master 7a3c9e49. Confidence of everything here until the row runs: unverified.

This row is in two parts, run in one sitting. **Part B is sent only after Part A has passed in full** (step 7), and a STOP in Part B is Part B's own: Part A's answers stand.

## Why this row
Row 2480-SK (`lab/2480-sk-answers` at `f7cb2588`, PARTIAL) made pairs of ledgers whose names share master's letters-and-digits key. For an apostrophe (pair `q`) and an underscore in place of a space (pair `u`), the gateway made a second ledger beside the first. For a comma (pairs `k`, `r`) and a space alone (pair `e`), it altered the first ledger. SK read every name in the `##LedgerName` form only, and master's `CHILDOF` form for the control alone.

**Part A: master's own `CHILDOF` reads.** Master's `LedgerScopeName` (`src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65`) admits `BRIDGE Sk Ck Cm` (`qa`) and `BRIDGE Sk Dk Dm` (`ua`): letters and single spaces. So master's shipped census (`render_agent_ledger_census`, `src-tauri/src/agent_read_profiles.rs:258`) and movement and window reads (`render_agent_ledger_vouchers`, `:367`), through `VoucherScope::collection` (`:279`), write that name itself into `CHILDOF`, in a book that can hold its sibling `qb` or `ub`. No row has read that. This is the one open question of #2480 about code that ships now:
- **each name reads its own ledger alone:** master's scoped reads are measured for these pairs, on one book;
- **an answer holds the sibling's voucher, or any other voucher, or lacks its own:** a shipped read returned another ledger's data under the name the user asked for. That is a result of this row, not a STOP, and the run coordinator decides where it is written (step 7).

**Part B: the altered ledger.** SK left one state no row has read: a ledger whose `NAME` attribute is `BRIDGE Sk Ak Am` (`ka`) and whose name list holds only `BRIDGE Sk Ak, Am` (`kb`), after the gateway took `kb`'s `Create` as an alteration of `ka`. Part B reads it by each spelling. This bears on the read side (what a read returns for a row whose two spellings differ) and on the write side (master's name index reads a row's spelling beside its names, `src-tauri/src/agent_master_post.rs:96`).

## Question
- **A.** For `qa` and `ua`, does each of master's three `CHILDOF` reads (census, movement, window) hold that name's own invoice and not its sibling's, and nothing else, twice byte-equal, and byte-equal to the same read in the `##LedgerName` form? For `qa`, `qb`, `ua` and `ub`, do SK's `##LedgerName` answers come again in this sitting?
- **B.** Read by `BRIDGE Sk Ak Am` and by `BRIDGE Sk Ak, Am`, in the four `##LedgerName` forms, and by `BRIDGE Sk Ak Am` in master's three `CHILDOF` forms: does each answer hold the ledger's one invoice (or bill) and nothing else, or is it empty, twice byte-equal?

**The one variable.** Part A: the form, master's `CHILDOF` against the `##LedgerName` form SK measured, with the names and the book fixed. Part B: the spelling, for one ledger, with the forms fixed. Each part is judged on its own answers.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` (byte-equal to SK's and MW's) applies in full. Where it and this sheet differ, the stricter one holds.

- **Pinned by GUID,** as in SK: `count20.py`, `nm.py`, `st.py`, `wx.py` and `sk.py` each hold BRIDGE PILOT LAB's GUID as `BOOK` and STOP on a row with another; `sc.py` uses `sk.py`'s.
- **Its state:** ALTVCHID 68 and ALTMSTID 245, the marks SK's restore left (its `s9`, equal to its `s1` and `s2`). Other marks at s1: stop before BK, with nothing sent.
- **It holds none of the eleven ledgers of `names.tsv` and none of their numbers.** `names.tsv` is SK's, byte for byte; this row creates seven of them (below). Step 2 checks all eleven, as SK's did.
- **The set-up is SK's imports, byte for byte,** for the control `pl` and the pairs `k`, `q` and `u`: thirteen imports, each sent in SK and taken out by SK's restore. Under C3, a file that writes is sent once in an item's life: this row is its own item, and sends each at most once. SK's imports for pairs `r` and `e`, and `I-kb.xml`, are not in this folder: `r` and `e` would only repeat `k`'s alteration, and `kb` gets no invoice.
- **Two exceptions hold only with the run coordinator's word,** given when the row is queued (SK's run lead's words (a) and (c)):
  - **(a) the empty answer:** an empty answer to an `L-lo-`, `C-v-`, `Vm-v-` or `Vw-v-` request with HTTP 200 and `CONTENT-LENGTH: 0` is not a STOP. **This row asks for (a) to cover its `C-u-`, `Vm-u-` and `Vw-u-` requests of steps 7 and 8 too.** Without that word, an empty answer to one of them is a STOP.
  - **(c) the alteration rule,** for `s4kb` alone (recipe (e)). Without that word the row is not run: it is built on SK's alteration, and without (c) `s4kb`'s answer would be the STOP of the set-up.
  - SK's (b), the refused-import exception, is not asked for: any refusal here is the STOP of the set-up.
- **The date and the period:** read the Gateway's current date and period at step 1. If the date is before 1-Aug-2026, or the period is not 1-Apr-26 to 31-Mar-27: stop before BK. **Do not change Tally's current date or period.**
- **BK is this row's way back.** Taken at step 1, before anything is sent, and restored at step 9 by C6. The restore runs on every path from step 3 on that is not a STOP. After any STOP, at any step, C8 holds, and the restore waits for the run coordinator's word.
- Write down the release shown on Tally's screen.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6).

**The seven ledgers this row writes** (six `Create`s that make a ledger, and `kb`'s, which alters `ka`) (of SK's eleven; `n` is the id's place in `names.tsv`, as in SK):

| n | id | number | name | role here |
| --- | --- | --- | --- | --- |
| 1 | `pl` | 0281 | `BRIDGE Mx Plain` | the control, MX's |
| 2 | `ka` | 0292 | `BRIDGE Sk Ak Am` | pair `k`: created with its invoice; Part B's ledger |
| 3 | `kb` | 0293 | `BRIDGE Sk Ak, Am` | its `Create` must alter `ka` (step 3); no invoice; Part B's second spelling |
| 6 | `qa` | 0296 | `BRIDGE Sk Ck Cm` | pair `q`; Part A, in both forms |
| 7 | `qb` | 0297 | `BRIDGE Sk Ck' Cm` | pair `q`; Part A, `##LedgerName` form only |
| 8 | `ua` | 0298 | `BRIDGE Sk Dk Dm` | pair `u`; Part A, in both forms |
| 9 | `ub` | 0299 | `BRIDGE Sk Dk_Dm` | pair `u`; Part A, `##LedgerName` form only |

**No name with a mark is ever sent inside `CHILDOF`** (P7). Master's own rule decides which names may be: `scope-master.txt` is the output of master's `LedgerScopeName::new` on every line of `names.tsv`, by `scope-test.rs`, copied into `src-tauri/crates/bridge-tally-protocol/tests/` of a checkout of master at 7a3c9e49, run by `cargo +1.96.0 test -p bridge-tally-protocol --test sc_scope` and never committed to master. It gives `ADMITTED` for `pl`, `ka`, `rb`, `qa`, `ua`, `ea` and `eb`, and `REFUSED ledger_scope_name_character` for `kb`, `ra`, `qb` and `ub`. `gen-sc.py` pins that file's sha256 and asserts, for **every request of this folder**, that each `CHILDOF` element holds either `##LedgerName` or a name master's verdict admitted, written as itself. The only names inside `CHILDOF` are `pl`'s (SK's control files), `ka`'s, `qa`'s and `ua`'s. In every other request, every name reaches Tally only as escaped XML text: as the `LedgerName` static variable in the reads, and as master's import shapes carry a name in the imports.

## The requests
Every file but the nine new ones is SK's, byte for byte (`lab/2480-sk-sheet` at `c5f9d52f`, folder `lab/2480-sk/`). Requests are UTF-16LE with a BOM, as sent.

| file | what it is |
| --- | --- |
| `LM-<id>.xml`, `I-<id>.xml` | SK's imports: a ledger, and its invoice, for `pl`, `ka`, `qa`, `qb`, `ua`, `ub`; `LM-kb.xml` alone for `kb` |
| `N-<number>.xml`, `B-bills.xml`, `S-snap-20260401-20260801.xml`, `C-book.xml`, `G-cat.xml`, `M-marks.xml` | SK's book reads, all eleven `N-` reads among them, so that step 2 and `s9` are SK's `s2`, byte for byte |
| `Vm-u-pl.xml`, `Vm-v-pl.xml`, `Vw-u-pl.xml`, `Vw-v-pl.xml`, `L-lo-pl-20260801.xml`, `C-u-pl.xml`, `C-v-pl.xml`, `Vb-book.xml` | SK's shape-step requests for the control, and the book-wide read of 1-Aug-2026 |
| `C-v-<id>.xml`, `Vm-v-<id>.xml`, `Vw-v-<id>.xml` | SK's `##LedgerName` forms of the census, the movement read and the window read, for `ka`, `kb`, `qa`, `qb`, `ua`, `ub` |
| `L-lo-ka-20260801.xml`, `L-lo-kb-20260801.xml` | SK's `Ledger Outstandings` with the name as the `LedgerName` static variable |
| **`C-u-<id>.xml`, `Vm-u-<id>.xml`, `Vw-u-<id>.xml`**, for `ka`, `qa`, `ua` (new) | master's `CHILDOF` form of the census, the movement read and the window read: SK's `C-u-pl.xml`, `Vm-u-pl.xml` and `Vw-u-pl.xml` with `<CHILDOF>BRIDGE Mx Plain</CHILDOF>` turned into the id's name, once, and nothing else |

**Sizes.** The voucher and census answers are SK's sizes for a one-day read (the window read about 95,000 B); `Vb-book.xml`'s answer is SK's with fewer invoices. An answer above 2,000,000 B is `run.py`'s stop.

## The sends and the wedge rule
`run.py` is SK's (NM's), byte for byte: one request at a time, each answer's headers kept by curl -D in `a-<label>-<file>.headers`, and it never abandons a request.
- **The wedge rule.** If any send has not answered **5 minutes** after it began: press nothing, do not end curl, send nothing more (not even `GET /status`), screenshot the whole Tally window, and report to the run coordinator (C9). Nothing is re-sent on this row's own judgement.
- **The stops.** `run.py`'s stops are failed requests, never "nothing found": a curl exit other than 0, an Unknown Request, an empty answer, or an answer above 2,000,000 B. `run.py` stops (exit 1) on every empty answer. For a request that exception (a) covers, at step 7 or 8, that exit is not yet a STOP: the rule below decides it. At steps 1 to 6 and 9, and for any other request, it is a STOP. So are the STOPs (exit 1) of `count20.py`, `nm.py`, `st.py`, `cv.py`, `wx.py`, `sk.py` and `sc.py`. Each stops the row there, but for the exceptions above.
- **An empty answer under exception (a),** for a request (a) covers, at step 7 or 8: it is not a STOP only if its headers file shows HTTP status 200 and `CONTENT-LENGTH: 0` (`wx.py` decides this: it reads an absent answer file as no bytes, so it does not matter whether curl wrote a 0-byte file). Then, before the next request: keep the headers file and the log line, screenshot the whole Tally window (a dialog: STOP), send `GET /status` once (C2c's command; not 200: STOP), and run `wx.py` on that answer, whatever its form. `nm.py` and `cv.py` are never given an empty answer, and no line that would give them one is run (step 7 names them). Its `JUDGE` or `JUDGE-B` line says `EMPTY ANSWER`, and that is its result. The lab may carry this by a wrapper, as SK's run did (`scripts/er.sh` on `lab/2480-sk-answers` at `f7cb2588`), with `C-u-`, `Vm-u-` and `Vw-u-` added to its forms and an absent answer file read as 0 bytes, as `wx.py` reads it. Without the wrapper, each step is done by hand, in this order. Every name read has a ledger, so an empty answer is not expected.
- **What the branch and any public text say about a STOP of this kind.** If a send wedges, Tally exits or stops answering, or a dialog the prompt rule names comes up, the row's RESULTS and any public comment say only "a STOP at step N, id X" (or "a STOP at step N" where no id applies: steps 1, 2, 4, 5 and 9, the marks reads and the book-wide reads; a write's own marks reads, and an `s7` read, take that write's or that pair's id; in step 8 the id is the spelling, `ka` or `kb`). The request before it, the screens, the timings and anything else that would show how to cause it go to the run coordinator by the private channel. The coordinator decides what, if anything, is published later.
- Check each file's sha256 against `SHA256.txt` before its send. No file in `sk/` is ever an argument to `run.py`.
- Close no Bridge process, and call no Bridge tool.

**Which script reads which answer**
- **`count20.py`**, **`nm.py`**, **`st.py`**, **`rf.py`**, **`wx.py`**, **`cv.py`** and **`sk.py`**: SK's, byte for byte, each with the modes its header names.
- **`sc.py`** (new): `--altered-state` and `--judge-b`, each whole in its header. It parses no answer itself: it loads `sk.py` and, through it, the readers above. `--judge-b` checks for an empty answer first, by `wx.py`'s rule, so no reader is given one. It prints no `VOUCHERKEY` and no GUID.

## Files
**SK's, byte for byte** (sha256 and size as SK's `SHA256.txt` lists them; that file is kept as `sk/SHA256.txt` and pinned by `gen-sc.py`): `.gitattributes`, `LAB-COMMON.md`, `names.tsv`, `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py`, `wx.py`, `cv.py`, `sk.py`, and every request of the table above but the nine new ones.

**New for this row:** this file, `gen-sc.py` and its nine files, `sc.py`, `scope-test.rs`, `scope-master.txt`, `SHA256.txt`.

**`gen-sc.py`** checks every other file against `sk/SHA256.txt` and `scope-master.txt` against its pin before it writes anything; writes the nine files, each swap's count asserted and each swapped back to its source byte for byte; and checks every request of the folder, `CHILDOF` by `CHILDOF`, against master's verdicts (above). It prints one line a request.

`SHA256.txt` lists every file with its size (`sha256  size  path`, SK's form).

**Tried before this sheet was sent:**
- `gen-sc.py`, run in this folder: its nine files; every SK file equal to SK's; every request's `CHILDOF` line as above. In copies: asked to write the `CHILDOF` form for `qb`, it stops at its check of master's verdicts, and with that check taken out, at its check of every request (`CHILDOF holds qb, which master's LedgerScopeName refuses`), writing nothing either time; a file that is neither SK's nor this row's, and a changed `scope-master.txt`, each stop it before anything is written.
- `scope-test.rs` on master at 7a3c9e49: the verdicts above.
- **`sc.py`**, on SK's answers (`lab/2480-sk-answers` at `f7cb2588`), read with the sheet's readers:
  - `--altered-state` on SK's `s6ga` and `s7qb` catalogue answers: `AS SK RECORDED`, exit 0; on SK's `s2` (before the set-up): `NOT AS SK RECORDED`, exit 3; on a voucher answer: its STOP.
  - `--judge-b` on SK's answers: on a read of `qa` or of the control, judged as `ka`: `OWN NO, OTHERS 1`, exit 3; on the book-wide read: `OWN YES, OTHERS 18`, exit 3; on a marks answer: its STOP. In a copy whose `names.tsv` gave `ka` the control's number and name: on SK's control reads `k1`, `k3`, `k5` and `k6`, `OWN YES, OTHERS 0`, exit 0, for each form.
  - `--judge-b` on a 0-byte answer with an HTTP 200, `CONTENT-LENGTH: 0` headers file: `EMPTY ANSWER`, exit 3; with another `CONTENT-LENGTH`, or with no headers file: its STOP.
  - `sk.py --judge` on SK's master-form answers of the control (`k1`, `k3`, `k6`): `OWN YES, SIBLING none, OTHERS 0`.
  - `sc.py --judge-b ka outstandings` on a copy of SK's `k5` answer with its one bill row repeated (`ka` given the control's number and name): its STOP, `the invoice is held 2 times, not once`, exit 1. Step 8's check before `b1a`, on SK's `k8`: `JUDGE-B ka movement: OWN YES, OTHERS 18`, exit 3.
- SK's book-wide read `k8` showed `ka`'s invoice on the ledger `BRIDGE Sk Ak Am` after the alteration, which is how `sc.py --judge-b` finds it.
- **Not tried:** any send to Tally; the nine new requests, which no book has been sent; any read of a ledger whose `NAME` attribute and name list differ.

## Steps
Run each label as `python3 -I run.py http://127.0.0.1:9001 <label> <files>`, the Gateway on 9001 as C2 finds it (written `URL` below). (The lab PC has no `python3` command; there, `python -I` with Python 3 is the same command.) The prompt rule, the wait rule and the path rule are SK's (NM's).

| step | labels |
| --- | --- |
| 1 | `s1` |
| 2 | `s2` |
| 3 | `s4<id>`, `s5<id>`, with `m` after each for its marks read, and `x` for the marks read after the alteration; `s7<b>` for a pair's catalogue read |
| 4 | `s6…` |
| 5 | `m1` |
| 6 | `k0` to `k9` |
| 7 | Part A: `c<n>a`, `c<n>b`, `q<n>a`, `q<n>b`, `w<n>a`, `w<n>b`, and for `qa` and `ua` `cu<n>a`, `cu<n>b`, `qu<n>a`, `qu<n>b`, `wu<n>a`, `wu<n>b`; then `a10`, `a11` |
| 8 | Part B: `b1a` to `b11b`; then `b12`, `b13` |
| 9 | `s9` |
| repeats | `m1r`, `k0r`, `k9r`, `a11r`, `b13r`: a marks read sent once more after MARKS DIFFER |

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named. For any other prompt, message or dialog: copy it word for word and screenshot it first; then answer No, or Esc if it offers no No; press nothing more and send nothing. That prompt is this row's result, and C8 holds. What the branch says about it is the STOP rule above.

**Each write, the same way, by hand, one label a command** (SK's recipe, without (f)):
- (a) `run.py URL <label>m M-marks.xml`, then `python3 -I count20.py --before-write a-s1-M-marks.xml a-<label>m-M-marks.xml`.
- (b) Only if that passes, and no key was pressed in Tally since: `run.py URL <label> <file>`, the import alone.
- (c) Screenshot the whole window before the next request (C3). A dialog: the prompt rule.
- (d) `python3 -I st.py --import a-<label>-<file>`, recorded whole. **On exit 0 (`CREATED ONE`), go on with the next write.** For `s4kb`, exit 0 is the STOP of the set-up: this row is built on SK's alteration.
- (e) **The alteration rule, for `s4kb` alone, only if `st.py` gave exit 1 and `run.py` in (b) gave exit 0:** `python3 -I sk.py --altered a-s4kb-LM-kb.xml`, recorded whole. On exit 0 (`ALTERED ONE`): `run.py URL s4kbx M-marks.xml`, then `count20.py --before-write a-s1-M-marks.xml a-s4kbx-M-marks.xml` and `count20.py --same-row a-s4kbm-M-marks.xml a-s4kbx-M-marks.xml`, each recorded whole. If `--before-write` passes, go on to `s7kb` (step 3). The `--same-row` line is a record, not a check (SK saw the master mark move by one). Anything else: the STOP of the set-up.
- On any exit other than 0 of `run.py` in (b): the STOP of the set-up.

The set-up is not run as a loop that decides for itself. If a script sends it, it ends at the first exit other than 0 of `run.py` or `count20.py`, and at the first exit of `st.py`, `sk.py` or `sc.py` that the recipe does not name as the next step.

### The set-up

1. **Load, check and back up.**
   - Step 0 of C2, with no company loaded. Load BRIDGE PILOT LAB alone from Select Company (C1). Any prompt: stop, press nothing, and ask.
   - Read the Gateway's current date and period, and write them down (Book).
   - `s1`: `M-marks.xml`. EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245. Other marks: stop here, before BK.
   - **Take BK** by C5, to the new empty folder `C:\lab\2480-sc\backup-1` (Tally may show it as `C:\lab\2480-Sc\backup-1`). SK's step 1 is the path. Record the file's name, size and sha256, and the time the Restore list will show.
2. **The book before.** `s2`: `N-0281.xml N-0292.xml N-0293.xml N-0294.xml N-0295.xml N-0296.xml N-0297.xml N-0298.xml N-0299.xml N-0300.xml N-0301.xml B-bills.xml S-snap-20260401-20260801.xml C-book.xml G-cat.xml M-marks.xml`: SK's `s2`, in its order.
   - EXPECT each N read 0 rows (`count20.py`), `nm.py` on B-bills no group for any of the eleven names, and on the snapshot "no row" for each. A row or a group: stop before step 3, each recorded whole.
   - `st.py --before a-s2-S-snap-20260401-20260801.xml`: EXPECT `FOLD: CLEAR`. A STOP: stop before step 3.
   - `st.py --names a-s2-S-snap-20260401-20260801.xml`, recorded whole.
   - `cv.py a-s2-C-book.xml`: EXPECT `ANSWER: VOUCHERS <n>`, the book before this row's invoices. Anything else: stop before step 3.
   - `sk.py --names a-s2-G-cat.xml`, recorded whole. EXPECT "no row" for each of the eleven. A row, or a STOP of `sk.py`: stop before step 3.
   - Each of these answers' sha256 is written down: `s9` is compared with them.
3. **The set-up,** in this order, each write by the recipe. **Every answer must be the one named; any other is the STOP of the set-up.**
   - **The control:** `s4pl` (`LM-pl.xml`), then `s5pl` (`I-pl.xml`): `CREATED ONE` each.
   - **Pair `k`:** `s4ka` (`LM-ka.xml`), `s5ka` (`I-ka.xml`): `CREATED ONE` each. `s4kb` (`LM-kb.xml`): **ALTERED** by recipe (e). Then `s7kb`: `G-cat.xml`, then `python3 -I sc.py --altered-state a-s7kb-G-cat.xml`, recorded whole: EXPECT `ALTERED STATE k: AS SK RECORDED` (exit 0). SK read no catalogue at this point: its later `s7qb`, `s6ga` and `s6gb` answers show this state, and `sc.py --altered-state` checks only pair `k`'s two names, which no later write of the set-up names. `I-kb.xml` is not in this folder and is never sent.
   - **Pair `q`, then pair `u`,** each `(a, b)`: `s4<a>` (`LM-<a>.xml`), `s5<a>` (`I-<a>.xml`), `s4<b>` (`LM-<b>.xml`): `CREATED ONE` each. Then `s7<b>`: `G-cat.xml`, then `python3 -I sk.py --pair <a> <b> a-s7<b>-G-cat.xml`, recorded whole: EXPECT `PAIR <a> <b>: BOTH` (exit 0). Then `s5<b>` (`I-<b>.xml`): `CREATED ONE`.
   - **The STOP of the set-up.** On any answer this step does not name, an exit 1 of any script, an exit other than 0 of `run.py` on an import or on `G-cat.xml`, or a dialog: send nothing more; screenshot the whole window; record the answer, if there is one, whole; and report to the run coordinator. C8 holds. Say in the report what the book then holds.
4. **The book-wide reads and the state.** `s6m`: `M-marks.xml`. Then `s6sa`, `s6sb`: `S-snap-20260401-20260801.xml`; `s6ba`, `s6bb`: `B-bills.xml`; `s6ga`, `s6gb`: `G-cat.xml`; `s6ka`, `s6kb`: `C-book.xml`; each the same file twice.
   - `nm.py` on `a-s6sa-…` and `a-s6ba-…`; `nm.py --same` on each of the four pairs; `st.py --names a-s6sa-S-snap-20260401-20260801.xml`; `sk.py --names a-s6ga-G-cat.xml`; `sc.py --altered-state a-s6ga-G-cat.xml`; `cv.py a-s6ka-C-book.xml`. Each recorded whole.
   - **INVOICED** is `pl,ka,qa,qb,ua,ub`. EXPECT, at `s6ga`: `NAMES` `EQUAL` for `pl`, `qa`, `qb`, `ua` and `ub`; for pair `k`, `NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])` and `NAMES kb: no row`, as SK's `s6ga` gave; and `ALTERED STATE k: AS SK RECORDED`; each pair of sends `EQUAL`; for each id of INVOICED, in the snapshot a row, bill-wise Yes, closing the invoice's amount, and in B-bills one group, `BP/26-27/<its number>`, as SK's step 4 found them, `ka`'s under `ka`'s name; and "no row" and no group for `kb`. Anything else: the STOP of the set-up.
5. **`m1`:** `M-marks.xml`, then `count20.py --same-row a-s6m-M-marks.xml a-m1-M-marks.xml`. EXPECT SAME MARKS. On MARKS DIFFER: send `M-marks.xml` once more as `m1r`, record it, send nothing else, and STOP.

### The reads

6. **The set-up whole, then the shape step, the control alone,** SK's step 6 in its sends and checks. Two things differ from SK's, each named below: the `REF` lines are never required (the note under the table), and READ is not used (this row reads the ids step 7 names).
   - **Before `k0`,** write down that each holds, with the line of the notes that shows it: no STOP at steps 1 to 5; every send answered; `m1` SAME MARKS against `s6m`; every `--before-write` passed; `pl` in its intended state (`CREATED ONE` at `s4pl` and `s5pl`, `NAMES pl: EQUAL` at `s6ga`). If any does not hold, nothing of steps 6 to 8 is sent: the row goes to step 9.
   - **Then SK's table, `k0` to `k9`,** one send at a time, each checked before the next, with `<INVOICED>` written `pl,ka,qa,qb,ua,ub`. After each send, screenshot the whole Tally window (a dialog: the prompt rule).

     | label | file | check, run before the next send | it must give |
     | --- | --- | --- | --- |
     | `k0` | `M-marks.xml` | `count20.py --same-row a-m1-M-marks.xml a-k0-M-marks.xml` | SAME MARKS |
     | `k1` | `Vm-u-pl.xml` | `wx.py a-k1-Vm-u-pl.xml`; `sk.py --ref a-k1-Vm-u-pl.xml` | `ANSWER: VOUCHERS 1`; the `REF` line recorded (*) |
     | `k2` | `Vm-v-pl.xml` | `wx.py --same a-k1-Vm-u-pl.xml a-k2-Vm-v-pl.xml`; `sk.py --ref a-k2-Vm-v-pl.xml` | `SAME: EQUAL`; recorded (*) |
     | `k3` | `Vw-u-pl.xml` | `wx.py a-k3-Vw-u-pl.xml`; `sk.py --ref a-k3-Vw-u-pl.xml` | `ANSWER: VOUCHERS 1`; recorded (*) |
     | `k4` | `Vw-v-pl.xml` | `wx.py --same a-k3-Vw-u-pl.xml a-k4-Vw-v-pl.xml`; `sk.py --ref a-k4-Vw-v-pl.xml` | `SAME: EQUAL`; recorded (*) |
     | `k5` | `L-lo-pl-20260801.xml` | `nm.py a-k5-L-lo-pl-20260801.xml`; `sk.py --ref a-k5-L-lo-pl-20260801.xml` | `ANSWER: ROWS 1 (0 On Account)`; recorded (*) |
     | `k6` | `C-u-pl.xml` | `cv.py a-k6-C-u-pl.xml` | `ANSWER: VOUCHERS 1` |
     | `k7` | `C-v-pl.xml` | `cv.py --same a-k6-C-u-pl.xml a-k7-C-v-pl.xml` | `SAME: EQUAL` |
     | `k8` | `Vb-book.xml` | `wx.py a-k8-Vb-book.xml`; `wx.py --match pl movement a-k1-Vm-u-pl.xml a-k8-Vb-book.xml`; `wx.py --match pl window a-k3-Vw-u-pl.xml a-k8-Vb-book.xml`; `cv.py --own pl a-k6-C-u-pl.xml a-s2-C-book.xml a-s6ka-C-book.xml pl,ka,qa,qb,ua,ub` | `ANSWER: VOUCHERS <n>`; `MATCH pl movement: EQUAL`; `MATCH pl window: EQUAL`; `OWN: YES` |
     | `k9` | `M-marks.xml` | `count20.py --same-row a-k0-M-marks.xml a-k9-M-marks.xml` | SAME MARKS |

     (*) The kept bytes are MX's and MW's, read with eleven of MX's ledgers in the book; this book holds six new ledgers, so an answer's company-information counts differ, as SK's did with eight. The `REF` lines are recorded and never required.
   - **The shape step passes only if every check gave what it names**, every send answered, and no dialog came up. Anything else is the STOP of the shape step, "a STOP at step 6" (with the id `pl` for `k1` to `k7`): nothing more is sent, not a marks read beyond C8's. On MARKS DIFFER at `k0` or `k9` alone, `M-marks.xml` is sent once more as `k0r` or `k9r` and recorded first. A STOP of a reader on an answer `run.py` kept, with no dialog, is reported with the reader's line whole and the shape of the element it stopped on (tags, no values), for the coordinator to judge, as SK's step 6 says.
7. **Part A: the pair names, in both forms. Only after step 6 has passed.** For each id of `qa`, `qb`, `ua`, `ub`, in that order (`n` = 6, 7, 8, 9), each send once, **each answer read by its reader before the next request**, and a screenshot of the whole Tally window after each `a` send, before its reader runs (a dialog: the prompt rule):
   - **the `##LedgerName` form,** SK's requests: `c<n>a`, `c<n>b`: `C-v-<id>.xml`, then `cv.py` on each and `cv.py --same` on the two; `q<n>a`, `q<n>b`: `Vm-v-<id>.xml`, then `wx.py` on each and `wx.py --same` on the two; `w<n>a`, `w<n>b`: `Vw-v-<id>.xml`, the same;
   - **for `qa` and `ua` only, master's `CHILDOF` form:** `cu<n>a`, `cu<n>b`: `C-u-<id>.xml`, then `cv.py` on each and `cv.py --same` on the two; `qu<n>a`, `qu<n>b`: `Vm-u-<id>.xml`, then `wx.py` on each and `wx.py --same`; `wu<n>a`, `wu<n>b`: `Vw-u-<id>.xml`, the same;
   - then, before the next id, each one line, recorded whole:
     - `cv.py --own <id> a-c<n>a-C-v-<id>.xml a-s2-C-book.xml a-s6ka-C-book.xml pl,ka,qa,qb,ua,ub`;
     - `wx.py --match <id> movement a-q<n>a-Vm-v-<id>.xml a-k8-Vb-book.xml` and `wx.py --match <id> window a-w<n>a-Vw-v-<id>.xml a-k8-Vb-book.xml`;
     - `sk.py --judge <id> census a-c<n>a-C-v-<id>.xml a-k8-Vb-book.xml`, and the same with `movement a-q<n>a-Vm-v-<id>.xml` and `window a-w<n>a-Vw-v-<id>.xml`;
     - **for `qa` and `ua`, the same three kinds of line on the `CHILDOF` answers** (`a-cu<n>a-C-u-<id>.xml`, `a-qu<n>a-Vm-u-<id>.xml`, `a-wu<n>a-Vw-u-<id>.xml`), and the form against the form, byte for byte: `cv.py --same a-c<n>a-C-v-<id>.xml a-cu<n>a-C-u-<id>.xml`, `wx.py --same a-q<n>a-Vm-v-<id>.xml a-qu<n>a-Vm-u-<id>.xml`, `wx.py --same a-w<n>a-Vw-v-<id>.xml a-wu<n>a-Vw-u-<id>.xml`.
   - After the last id: `a10`: `Vb-book.xml`, then `wx.py a-a10-Vb-book.xml` and `wx.py --same a-k8-Vb-book.xml a-a10-Vb-book.xml`; then `a11`: `M-marks.xml`, then `count20.py --same-row a-k9-M-marks.xml a-a11-M-marks.xml`. On MARKS DIFFER: send `M-marks.xml` once more as `a11r`, record it, send nothing else, and STOP.
   - **An empty answer under (a), at step 7,** is read by `wx.py` alone, as in step 8, and its pair by `wx.py --same` in place of `cv.py --same`. For an empty `C-` answer, no `cv.py`, `cv.py --own`, `cv.py --same` against the other form, or `sk.py --judge … census` line is run on it (`cv.py` would STOP, and `sk.py` would print `NO BYTES`); RESULTS records `JUDGE <id> census: EMPTY ANSWER` in their place, from the `wx.py` line. For an empty `V` answer, `sk.py --judge` prints `EMPTY ANSWER` itself, and the `wx.py --match` line on it is not run; the form-against-form line is `wx.py --same`, as for any answer.
   - **An answer a reader reads never stops the row,** whatever it holds: a `SIBLING YES`, an `OWN NO`, an `OTHERS` above 0, a `DIFFER`, a `STATUS`, an empty answer under (a), and every exit 3 are results, recorded in full. Only an exit 1 of a reader, a dialog and a send with no answer are STOPs: "a STOP at step 7, id X". Nothing more is sent, C8 holds, and the answers before it stand. Part B is then not sent.
   - **A `CHILDOF` answer that is not clean.** If any `JUDGE` line on a `CHILDOF` answer (`cu`, `qu` or `wu`) is other than `OWN YES, SIBLING NO, OTHERS 0`, or one of them is empty, the row goes on, and **RESULTS states it in one line, the first line of Part A's section**: `CHILDOF FORM: <id> <form>: <the JUDGE line>`, one such line each. **The operator tells the run coordinator at once, as soon as the line is read, before any comment is drafted,** and the row goes on; the hand-back says so again in its first line. It would be a defect of a shipped read; the coordinator decides where it is written, and nothing more is said of it in public text until then.
   - **Part A has passed in full** only if: no STOP at steps 6 and 7; every send answered; no dialog; `a10` `SAME: EQUAL` with `k8`; and `a11` SAME MARKS. Write it down, with the lines that show it. If Part A has not passed in full, Part B is not sent, and the row goes to step 9 (or, after a STOP, waits under C8).
8. **Part B: the altered ledger, by each spelling. Only after Part A has passed in full.**
   - **Before `b1a`:** `python3 -I sc.py --judge-b ka movement a-k8-Vb-book.xml a-k8-Vb-book.xml`, recorded whole. It must print `JUDGE-B ka movement: OWN YES, OTHERS <n>`: `ka`'s invoice found in `k8`, as SK's `k8` showed it. Any other line: Part B is not sent, and the row goes to step 9.
   - Then each send once, in this order, each answer read before the next request, a screenshot after each `a` send:

   | labels | file | spelling | form |
   | --- | --- | --- | --- |
   | `b1a`, `b1b` | `L-lo-ka-20260801.xml` | `ka` | outstandings |
   | `b2a`, `b2b` | `C-v-ka.xml` | `ka` | census |
   | `b3a`, `b3b` | `Vm-v-ka.xml` | `ka` | movement |
   | `b4a`, `b4b` | `Vw-v-ka.xml` | `ka` | window |
   | `b5a`, `b5b` | `C-u-ka.xml` | `ka`, master's `CHILDOF` form | census |
   | `b6a`, `b6b` | `Vm-u-ka.xml` | `ka`, master's `CHILDOF` form | movement |
   | `b7a`, `b7b` | `Vw-u-ka.xml` | `ka`, master's `CHILDOF` form | window |
   | `b8a`, `b8b` | `L-lo-kb-20260801.xml` | `kb` | outstandings |
   | `b9a`, `b9b` | `C-v-kb.xml` | `kb` | census |
   | `b10a`, `b10b` | `Vm-v-kb.xml` | `kb` | movement |
   | `b11a`, `b11b` | `Vw-v-kb.xml` | `kb` | window |

   - **Each answer's reader:** `nm.py` for an `L-lo-` answer, `cv.py` for a `C-` answer, `wx.py` for a `V` answer; **an empty answer under (a) is read by `wx.py` alone**, whatever its form. Then `wx.py --same` on each `a` and `b` (a byte compare, which reads two empty answers as `EQUAL`).
   - Then each one line, recorded whole:
     - `sc.py --judge-b <spelling> <form> a-<label>a-<file> a-k8-Vb-book.xml` for each row of the table: **one `JUDGE-B` line a spelling a form; an empty answer is its own line, `EMPTY ANSWER`, for that spelling and form**;
     - the spellings against each other, byte for byte: `wx.py --same a-b1a-… a-b8a-…`, `a-b2a-… a-b9a-…`, `a-b3a-… a-b10a-…`, `a-b4a-… a-b11a-…`;
     - the forms against each other for `ka`, byte for byte: `wx.py --same a-b2a-… a-b5a-…`, `a-b3a-… a-b6a-…`, `a-b4a-… a-b7a-…`.
   - After the last: `b12`: `Vb-book.xml`, then `wx.py a-b12-Vb-book.xml` and `wx.py --same a-k8-Vb-book.xml a-b12-Vb-book.xml`; then `b13`: `M-marks.xml`, then `count20.py --same-row a-a11-M-marks.xml a-b13-M-marks.xml`. On MARKS DIFFER: send `M-marks.xml` once more as `b13r`, record it, send nothing else, and STOP. If `b12` is not equal to `k8`, record both: the report says the book-wide read changed during step 8.
   - **No `kb` name is sent in `CHILDOF`:** master refuses it (`scope-master.txt`), and `gen-sc.py` asserts no request holds it there.
   - **A reader's answer never stops the row** (as in step 7). Only an exit 1 of a reader, a dialog and a send with no answer are **Part B's STOP**, "a STOP at step 8, id `ka`" or "`kb`": nothing more is sent, C8 holds, and every answer before it, all of Part A's included, stands.
9. **Restore** BK by C6, with `restore-plan.txt` written first. SK's step 8 is the path, with its two prompts ("Accept ?" y; the folder-exists question, R Replace). List the data folder before and after, as SK did.
   - Then `s9`: every file of s2, in s2's order, and compare whole: `count20.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml` passes; each N read 0 rows; B-bills holds no group of the eleven names; the snapshot shows "no row" for each; `sk.py --names a-s9-G-cat.xml` "no row" for each; each answer's sha256 equals s2's.
   - **Any of these not equal stops the row:** send nothing more, and report before the close. The coordinator decides.
10. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering. Screenshot the window, send it and the answers to the run coordinator, and ask before anything else is pressed or sent. What the branch says about it is the STOP rule of "The sends".

## Record
- **RESULTS has two sections, Part A and Part B, each with its own confidence line,** deviations and "not measured". Part A's section holds the set-up (steps 1 to 5), the shape step (6) and step 7, and begins with the `CHILDOF FORM:` lines if there are any (step 7). Part B's section holds step 8, or says that Part B was not sent and why. The way back (step 9) and the close follow both.
- For the set-up: each import's counters whole, `sk.py --altered` and the two marks lines for `s4kb`, `sc.py --altered-state` at `s7kb` and `s6ga`, `sk.py --pair` at `s7qb` and `s7ub`, and step 4's checks line by line.
- Each answer's bytes and sha256, its headers file whole, and each script's lines, whole. `run.py`'s log for each label, and how long each read took, except for the send of a STOP the STOP rule names.
- The window after each import and after each shot named in steps 6 to 8, by name, bytes and sha256. Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- **For a STOP the STOP rule of "The sends" names,** none of that send's detail goes on the branch: all of it goes to the run coordinator by the private channel, and the branch says only "a STOP at step N, id X".
- The BK file and the restore, as C5 and C6 ask, with the data folder listed before and after. The release on Tally's screen, and the Gateway's date and period.
- For each id of Part A: its pairs of reads and their `--same` lines, its `--own` and two `--match` lines, its `JUDGE` lines, and for `qa` and `ua` the same for the `CHILDOF` form and the three form-against-form lines.
- For Part B: each pair of reads and its `--same` line, every `JUDGE-B` line, and the seven spelling-against-spelling and form-against-form lines.

**Staging the evidence:** SK's rule. The check for 10 or more digits sets aside this book's registered GUID matched whole, and nothing else. Any other hit waits for the run coordinator. The voucher answers hold each voucher's `VOUCHERKEY`, a run of more than ten digits; the catalogue answer holds each ledger's GUID. They are Tally's own, in the answers only. No script prints a `VOUCHERKEY`. `cv.py` prints the voucher GUIDs of a census answer, each this book's GUID with a suffix; `sk.py` and `sc.py` print no GUID. The RESULTS quote neither.

## Expected
Estimates, not predictions.
- **Steps 1 to 5:** SK's answers for the same imports: `CREATED ONE` for each but `s4kb`, `ALTERED ONE` there, `BOTH` for `q` and `u`.
- **Step 6:** as SK's, with the `REF` lines differing in the company-information counts.
- **Step 7:** the `##LedgerName` answers as SK's for the same names. For the `CHILDOF` form, no estimate: no row has read a name in `CHILDOF` in a book that holds a ledger sharing its key.
- **Step 8:** no estimate. No row has read a ledger whose `NAME` attribute and name list differ.
- **s9:** equal to s2 and s1.

## Decides
Each is PARTIAL: one book, one release, one sitting, master's request templates sent by curl, without the reads Bridge sends around them, and without the build's own parse.

**Part A,** for `qa` and `ua`, for each `CHILDOF` form, by its `JUDGE` line, its pair `SAME: EQUAL`, its `--own` or `--match` line and its form-against-form line:
- **`OWN YES, SIBLING NO, OTHERS 0`, the pair equal, its own line `EQUAL` or `YES`, and the form-against-form line `EQUAL`:** on this book and release, master's shipped `CHILDOF` read told the name from a sibling that shares its key by an apostrophe or an underscore.
- **`SIBLING YES`, `OTHERS` above 0, `OWN NO`, or an empty answer:** master's shipped read, given a name it admits, did not read that ledger alone. The coordinator decides where that is written (step 7).
- **The form-against-form line `DIFFER` with a clean `JUDGE` line:** recorded; the two forms answered differently with the same vouchers, and the coordinator decides what that means.
- For `qb` and `ub`, and for `qa` and `ua` in the `##LedgerName` form: SK's reads again, in another sitting.

**Part B,** for each spelling and form, by its `JUDGE-B` line:
- **`OWN YES, OTHERS 0`:** that read, by that spelling, reached the altered ledger.
- **`EMPTY ANSWER`,** or `OWN NO`: that read, by that spelling, did not reach it.
- **`OWN YES, OTHERS` above 0:** that read reached it and other vouchers too. Recorded; the coordinator decides what it means.
- **A `STATUS`, another reader kind, or `NOT JUDGED`:** recorded; it decides nothing for that spelling and form, and the coordinator decides.
- Together, whether a ledger altered through a `Create` is read by its `NAME` attribute, by the name its name list holds, by both or by neither, in each form.

It says nothing of marks not paired, of a comma or a space between two coexisting ledgers (the gateway made none), of pairs made on Tally's screens, or of the build's own parse.

**What these answers are for.** Part A informs whether master's scoped reads need the one-stem refusal now, not only in the #2480 build. Part B informs the one-stem condition's catalogue check (`NAME` attribute against name list) and the write side's reading of a row's spelling. They become fixtures only if the manifest admits them.

## Code admits the trigger
Lines are at master 7a3c9e49.
- **Master sends a name in `CHILDOF` when `LedgerScopeName` admits it.** `LedgerScopeName::new` (`src-tauri/crates/bridge-tally-protocol/src/ledger_scope.rs:65`) admits only ASCII letters, digits and a single space (`:74`), and refuses the words of `FORMULA_WORDS` (`:30`). `VoucherScope::collection` (`src-tauri/src/agent_read_profiles.rs:279`) writes that name itself into `CHILDOF`; its users are the census (`:258`) and `render_agent_ledger_vouchers` (`:367`). `admitted_party` (`src-tauri/src/agent_party_bills.rs:26`) and `scope_name_of` (`src-tauri/src/agent_movement.rs:99`) are where a user's name meets that type.
- **`qa`, `ua` and `ka` pass it; `qb`, `ub` and `kb` do not** (`scope-master.txt`). So master's shipped reads can send `qa` and `ua` in a book that holds `qb` and `ub`, as Part A does, and `ka` for the altered ledger, as Part B does.
- **The `##LedgerName` form is sent by no build for any name;** it is the form #2480 plans. Part A and Part B send it as SK did.

## Not in this row
- ComplyEaze Bridge's own transport, or any Bridge build;
- any name with a mark inside `CHILDOF`;
- pairs `r` and `e`, a pair by case, a trailing line break, an alias, or any mark other than SK's; three or more ledgers sharing one key; a pair made on Tally's own screens;
- the reads over more than one day, with another AlterID span, or on a book of real size; a ledger with more than one voucher or bill, or a settled bill;
- a voucher posted after the alteration naming either spelling;
- another to-date, book or release; a book with GST on; Educational mode.
