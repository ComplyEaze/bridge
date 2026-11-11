# Row 2480-SV: which ledger does a voucher land on when it names a ledger whose key a sibling shares, or the altered ledger's other spelling?

Shape discovery, as rows 2346-PB and 2480-NM to 2480-SC: the requests are written for the lab, not requests the build sends, which the maintainers allow in the lab (ruling of 26 Oct 2026). Not for merge. For #2480 (0.9.0) and #485. Written 11 Nov 2026. Lines are at master 7a3c9e49. Confidence of everything here until the row runs: unverified.

## Why this row
Rows 2480-SK and 2480-SC (`lab/2480-sk-answers` at `f7cb2588`, `lab/2480-sc-answers` at `f8cd7d14`, each PARTIAL) measured the read side for three marks on one book:
- for an apostrophe (pair `q`) and an underscore in place of a space (pair `u`), the gateway made a second ledger beside the first;
- for a comma (pair `k`), it took the second `Create` as an alteration of the first ledger. That ledger's `NAME` attribute stayed `BRIDGE Sk Ak Am` (`ka`), and its name list now holds only `BRIDGE Sk Ak, Am` (`kb`);
- every scoped read, in every form, read each name as its own ledger, and read the altered ledger by both spellings.

No row has sent a voucher naming one of these ledgers. That is the question here, and it is about code that ships now (see "Code admits the trigger", the design note). Master's voucher post admits a voucher naming `qb` in a book that holds `qa`, or `ub` beside `ua`, and sends it. Its folded-twin refusal does not fold an apostrophe away or an underscore to a space. Each answer is useful:
- **each voucher lands on the ledger it names:** master's post is right to admit those names, on this book and release;
- **a voucher lands on its sibling, or on any ledger but its own:** a shipped post would send a voucher to a ledger the user did not name. The row ends there (step 7), and the run coordinator decides where that is written;
- **an import is refused with a line error:** that id's result, recorded (step 7), not a STOP.

`kb` is different: master would refuse that voucher before any send (the design note). The row still sends `J-kb.xml`, as a measure of the raw import shape, not of anything Bridge sends.

## Question
For each of `qa`, `qb`, `ua`, `ub`, `ka` and `kb`: a gateway `Create` of one Sales invoice whose party ledger is that name. Is it created, and on which ledger does the book-wide read then show it? After all six, does each ledger-scoped read of each name hold exactly the vouchers on its ledger?

**The one variable** is the ledger name a voucher carries. The voucher (SK's invoice, one new number each, the same day and amount), the book and the set-up are fixed.

## Book
`BRIDGE PILOT LAB` (synthetic), loaded alone, in a sitting of its own. `LAB-COMMON.md` (byte-equal to SC's, SK's and MW's) applies in full. Where it and this sheet differ, the stricter one holds.

- **Pinned by GUID,** as in SC: `count20.py`, `nm.py`, `st.py`, `wx.py` and `sk.py` each hold BRIDGE PILOT LAB's GUID as `BOOK` and STOP on a row with another; `sc.py` and `sv.py` use `sk.py`'s.
- **Its state:** ALTVCHID 68 and ALTMSTID 245, the marks SC's restore left (its `s9`, equal to its `s1`). Other marks at s1: stop before BK, with nothing sent.
- **It holds none of the eleven ledgers of `names.tsv`,** none of their numbers, and none of this row's six new numbers. Step 2 checks all seventeen numbers.
- **The set-up is SC's, byte for byte:** SC's steps 3 to 5, the same thirteen imports, the same gates, the same checks, with SC's alteration rule for `s4kb` alone. Under C3, a file that writes is sent once in an item's life: this row is its own item, and sends each at most once.
- **Exceptions, each only with the run coordinator's word,** given when the row is queued:
  - **(a) the empty answer,** for the `L-lo-`, `C-v-`, `Vm-v-` and `Vw-v-` requests of step 8, as in SC. Without that word, an empty answer to one of them is a STOP.
  - **(b) the refused import,** for the six `J-` imports of step 7 alone: an import `rf.py` reads as `IMPORT: REFUSED` is that id's result, not a STOP (step 7). Without that word, any refusal of a `J-` import is a STOP. A refusal at the set-up is still the STOP of the set-up, as in SC.
  - **(c) the alteration rule,** for `s4kb` alone, as in SC. Without that word the row is not run.
- **The date and the period:** read the Gateway's current date and period at step 1. If the date is before 1-Aug-2026, or the period is not 1-Apr-26 to 31-Mar-27: stop before BK. **Do not change Tally's current date or period.**
- **BK is this row's way back.** Taken at step 1, before anything is sent, and restored at step 9 by C6. The restore runs on every path from step 3 on that is not a STOP, **the end of step 7 after a wrong landing included**. After any STOP, at any step, C8 holds, and the restore waits for the run coordinator's word.
- Write down the release shown on Tally's screen.

**No Bridge build.** Raw requests, one at a time, through `run.py`. Nothing is keyed in Tally but BK (C5) and the restore (C6).

**The six vouchers** (`n` is the voucher's place in the order sent; each is SK's invoice for the id, or for `kb` the control's, with a new number and REMOTEID tail). The control `pl` gets none: see Sizes.

| n | id | name | new number | its TARGET, the ledger it must land on | master's post (design note) |
| --- | --- | --- | --- | --- | --- |
| 1 | `qa` | `BRIDGE Sk Ck Cm` | 0303 | `qa` | admits |
| 2 | `qb` | `BRIDGE Sk Ck' Cm` | 0304 | `qb` | admits |
| 3 | `ua` | `BRIDGE Sk Dk Dm` | 0305 | `ua` | admits |
| 4 | `ub` | `BRIDGE Sk Dk_Dm` | 0306 | `ub` | admits |
| 5 | `ka` | `BRIDGE Sk Ak Am` | 0307 | `ka` | admits |
| 6 | `kb` | `BRIDGE Sk Ak, Am` | 0308 | `ka` (the altered ledger) | refuses, `masters_not_exact` |

`qa` and `ua` go first, each before its sibling's: a plain name whose first invoice landed on its own ledger is this row's check that the new numbers and tails import as SK's did. `kb` is sent last, so that a refusal or a landing of it cannot change what the five before it found.

**No request this row sends holds a name inside `CHILDOF`.** SC's `C-u-`, `Vm-u-` and `Vw-u-` files are in the folder (SC's, byte for byte) and are not sent here: SC measured that form byte-equal to the `##LedgerName` form for these names.

## The requests
Every file but the twelve new ones is SC's, byte for byte (`lab/2480-sc-sheet` at `39162ad5`, folder `lab/2480-sc/`; SC's `SHA256.txt` is kept as `sc/SHA256.txt` and pinned by `gen-sv.py`). Requests are UTF-16LE with a BOM, as sent.

| file | what it is |
| --- | --- |
| SC's set-up and book reads | `LM-<id>.xml`, `I-<id>.xml`, `N-<number>.xml` (eleven), `B-bills.xml`, `S-snap-20260401-20260801.xml`, `C-book.xml`, `G-cat.xml`, `M-marks.xml`, `Vb-book.xml` |
| SC's `##LedgerName` reads | `C-v-<id>.xml`, `Vm-v-<id>.xml`, `Vw-v-<id>.xml` for `pl`, `ka`, `kb`, `qa`, `qb`, `ua`, `ub`; `L-lo-<id>-20260801.xml` for `pl`, `ka`, `kb` |
| **`J-<id>.xml`** (new, six) | the voucher of the table above: `I-pl.xml` with the control's name turned into the id's (seven times, escaped as master's `xml_text::escape_text` writes it), `BP/26-27/0281` into the new number (three times), and the REMOTEID tail `e81` into `ed<n+2>` (`ed3` to `ed8`, once), and nothing else |
| **`N-<new number>.xml`** (new, six) | SC's `N-0281.xml` with the number turned into the new one, once, and nothing else |

**Sizes.** Each `J-` file is 5,016 B but `qb`'s (5,100 B, its `&apos;` seven times) and `kb`'s (5,030 B). The book-wide read's answer is SC's `k8` (1,361,516 B, 17 vouchers) and grows by one invoice a landing: 91,914 B, measured as `k8` less `qa`'s invoice. Six landings make it about 1,913,000 B. An answer above 2,000,000 B is `run.py`'s stop (`BOUND`), and a seventh voucher would pass it (about 2,005,000 B). **So the control gets no voucher:** `qa` and `ua`, plain names sent first, are this row's control for the new import. The margin at the last `v` read is about 4%. An answer above the bound there is `run.py`'s STOP, and C8 holds with the vouchers in the book.

## The sends and the wedge rule
SC's, word for word (its ROW.md, "The sends and the wedge rule"), with this row's steps:
- `run.py` is SC's: one request at a time, each answer's headers kept, and it never abandons a request. **The wedge rule:** a send not answered after **5 minutes**: press nothing, end nothing, send nothing more (not even `GET /status`), screenshot the whole Tally window, and report to the run coordinator (C9).
- **The stops** are SC's: a curl exit other than 0, an Unknown Request, an empty answer outside exception (a), an answer above 2,000,000 B, and an exit 1 of any reader. Each stops the row there.
- **What the branch and any public text say about a STOP of this kind.** If a send wedges, Tally exits or stops answering, or a dialog the prompt rule names comes up, RESULTS and any public comment say only "a STOP at step N, id X" (or "a STOP at step N" where no id applies). In step 7 the id is the voucher's; in step 8, the name read. The request before it, the screens, the timings and anything else that would show how to cause it go to the run coordinator by the private channel.
- Check each file's sha256 against `SHA256.txt` before its send. No file in `sc/` or `sk/` is ever an argument to `run.py`.
- Close no Bridge process, and call no Bridge tool.

**Which script reads which answer**
- **`count20.py`**, **`nm.py`**, **`st.py`**, **`rf.py`**, **`wx.py`**, **`cv.py`**, **`sk.py`** and **`sc.py`**: SC's, byte for byte, each with the modes its header names.
- **`sv.py`** (new): `--landed` and `--judge2`, each whole in its header. It parses no answer itself: it loads `sk.py` and, through it, `wx.py`, `cv.py` and `nm.py`. It prints no `VOUCHERKEY` and no GUID. It is the "new `sc.py` line" of the proposal, in a file of its own so that `sc.py` stays SC's.

**Where a reader's exit 3 is expected.** No check of this sheet passes on an exit 3. Every check that must pass names exit 0, and every exit 3 is a result, recorded whole:
- `sv.py --landed` exits 0 only for `VERDICT <id>: ON ITS LEDGER`. Its exit 3 lines end the row at step 7.
- `sv.py --judge2` exits 3 on any read not holding exactly its ledger's vouchers. The row goes on (step 8).
- `st.py --import` gives exit 1 on any answer but `CREATED ONE`. In step 7 that path goes to `rf.py` under exception (b).

## Files
**SC's, byte for byte** (sha256 and size as `sc/SHA256.txt` lists them, which `gen-sv.py` checks): every file of SC's folder but its `ROW.md` and `SHA256.txt`, `sk/SHA256.txt` and `.gitattributes` included.

**New for this row:** this file, `gen-sv.py` and its twelve files, `sv.py`, `tried.txt`, `SHA256.txt`.

**`gen-sv.py`** checks every other file against `sc/SHA256.txt` before it writes anything. It writes the twelve files, each swap's count asserted and each swapped back to its source byte for byte. Where SK's invoice for the id is in the folder (all but `kb`), it asserts that `J-<id>` with SK's number and tail swapped in again is that invoice byte for byte, so each name is written as SK's imports wrote it. It asserts that no new number is one of `names.tsv`'s and no new tail is one MX or SK sent.

`SHA256.txt` lists every other file with its size (`sha256  size  path`, SK's form).

**Tried before this sheet was sent:**
- `gen-sv.py`, run in this folder: its twelve files; every SC file equal to SC's.
- **`sv.py`**, on SC's answers (`lab/2480-sc-answers` at `f8cd7d14`) and on hand-made copies of SC's `k8`. Every command, its output whole and its exit code are in `tried.txt`.
  - `--landed qa` with SC's `k8` as both reads: `NO NEW VOUCHER`, `NOT JUDGED`, exit 3.
  - In a copy of `sv.py` whose new numbers for `qa` and `kb` were SC's 0296 and 0292, so that SC's own invoices stand for new vouchers:
    - `--landed qa`, before = `k8` less `qa`'s invoice, after = `k8`: `ON qa`, `ON ITS LEDGER`, exit 0;
    - `--landed kb`, before = `k8` less `ka`'s invoice: `ON ka`, `ON ITS LEDGER`, exit 0.
  - **The hand-shifted copy, which must fail:** `k8` with `qa`'s invoice's one `LEDGERNAME` of `qa`'s name turned into `qb`'s.
    - `--landed qa` against it: `ON qb`, `ON ITS SIBLING`, exit 3.
    - With SC's `k8` as the before read: `BOOK CHANGED`, `NOT JUDGED`, exit 3.
  - **The unmodified `sv.py`, with this row's own numbers,** on two more copies of `k8`, each with one invoice's number (three times in it) turned into a new one:
    - `--landed qa`, before = `k8` less `qa`'s invoice, after = `k8` with that invoice numbered 0303: `ON qa`, `ON ITS LEDGER`, exit 0;
    - `--landed kb`, before = `k8` less `ka`'s invoice, after = `k8` with that invoice numbered 0308: `ON ka`, `ON ITS LEDGER`, exit 0;
    - `--landed qb` on the first pair: `NOT ITS NUMBER`, `NOT JUDGED`, exit 3.
  - `--judge2` on SC's `q6a`, `c6a` and `w6a` answers as `qa`, and on `b8a`, `b9a` and `b10a` as `kb`, each against `k8`: `OWN 1 OF 1, OTHERS 0`, exit 0. On `qb`'s movement answer judged as `qa`: `OWN 0 OF 1, OTHERS 1`, exit 3. On `qa`'s movement answer against the shifted copy: `OWN 0 OF 0, OTHERS 1`, exit 3.
- **Not tried:** any send to Tally; the twelve new requests, which no book has been sent; any book holding two vouchers on one of these ledgers.

## Steps
Run each label as `python3 -I run.py URL <label> <files>`, the Gateway on 9001 as C2 finds it. (The lab PC has no `python3` command; there, `python -I` with Python 3 is the same command.) The prompt rule, the wait rule and the path rule are SC's.

| step | labels |
| --- | --- |
| 1 | `s1` |
| 2 | `s2` |
| 3 to 5 | SC's labels, unchanged: `s4<id>`, `s5<id>` (with `m` and `x`), `s7<b>`, `s6…`, `m1` |
| 6 | `v0`, `k0` |
| 7 | for each voucher `n` = 1 to 6: `j<n>m`, `j<n>`, then `v<n>`, or `j<n>x` after a refusal |
| 8 | `k1`; `r<n>…` for each id; then `e1`, `e2` |
| 9 | `s9` |
| repeats | `m1r`, `k0r`, `e2r`: a marks read sent once more after MARKS DIFFER |

**The prompt rule, at every step.** The only prompts answered are C5's and C6's, each with the key named. For any other prompt, message or dialog: copy it word for word and screenshot it first; then answer No, or Esc if it offers no No; press nothing more and send nothing. That prompt is this row's result, and C8 holds.

### The set-up

1. **Load, check and back up.** SC's step 1, with the backup folder `C:\lab\2480-sv\backup-1` (Tally may show it as `C:\lab\2480-Sv\backup-1`). `s1`: `M-marks.xml`, EXPECT one COMPANY row, the book's GUID, ALTVCHID 68 and ALTMSTID 245; other marks: stop here, before BK.
2. **The book before.** `s2`: SC's `s2` files in SC's order (`N-0281.xml`, `N-0292.xml` to `N-0301.xml`, `B-bills.xml`, `S-snap-20260401-20260801.xml`, `C-book.xml`, `G-cat.xml`, `M-marks.xml`), then `N-0303.xml` to `N-0308.xml`: twenty-two reads.
   - SC's checks, each recorded whole. **EXPECT each of the seventeen N reads 0 rows** (`count20.py`). A row: stop before step 3.
   - Each answer's sha256 is written down: `s9` is compared with them.
3. **to 5. The set-up, the book-wide reads and `m1`:** SC's steps 3, 4 and 5, every label, file, check and STOP as SC's sheet writes them. **Every answer must be the one SC's sheet names**; any other is the STOP of the set-up.

### The vouchers and the reads

6. **The book before the vouchers.** Before `v0`, write down that each holds, with the line of the notes that shows it: no STOP at steps 1 to 5; every send answered; `m1` SAME MARKS against `s6m`; every `--before-write` passed. If any does not hold, that is a STOP at step 6, as every STOP is: nothing of steps 6 to 8 is sent, and C8 holds, so the restore waits for the run coordinator's word.
   - `v0`: `Vb-book.xml`, then `wx.py a-v0-Vb-book.xml`. EXPECT `ANSWER: VOUCHERS <n>`, SC's `k8` count (17) if the book is SC's. Record `<n>`, whatever it is.
   - `k0`: `M-marks.xml`, then `count20.py --same-row a-m1-M-marks.xml a-k0-M-marks.xml`. EXPECT SAME MARKS. On MARKS DIFFER: `k0r` once, record it, send nothing else, and STOP.
7. **The six vouchers,** in the table's order, `n` = 1 to 6. For each, by hand, one label a command:
   - (a) `j<n>m`: `M-marks.xml`, then `python3 -I count20.py --before-write a-s1-M-marks.xml a-j<n>m-M-marks.xml`.
   - (b) Only if that passes, and no key was pressed in Tally since: `j<n>`: `J-<id>.xml`, the import alone. If `--before-write` does not pass, or a key was pressed: `J-<id>.xml` is not sent, and that is a STOP at step 7, id `<id>`.
   - (c) Screenshot the whole window before the next request. A dialog: the prompt rule.
   - (d) `python3 -I st.py --import a-j<n>-J-<id>.xml`, recorded whole.
     - **Exit 0 (`CREATED ONE`):** go to (e).
     - **Exit 1:** `python3 -I rf.py a-j<n>-J-<id>.xml`, recorded whole. On `IMPORT: REFUSED` (exit 0), under exception (b): `j<n>x`: `M-marks.xml`, then `count20.py --same-row a-j<n>m-M-marks.xml a-j<n>x-M-marks.xml`. On SAME MARKS, the refusal is that id's result, **REFUSED**, its LINEERROR lines whole. No `v<n>` is sent, and the next voucher's before read is the last `v` sent. Any other line of `rf.py` or `count20.py`, or no word for (b): a STOP at step 7, id `<id>`.
     - **Any other exit of `st.py`:** a STOP at step 7, id `<id>`.
   - (e) `v<n>`: `Vb-book.xml`, then `wx.py a-v<n>-Vb-book.xml` and `python3 -I sv.py --landed <id> <BEFORE> a-v<n>-Vb-book.xml`, each recorded whole. `<BEFORE>` is the last book-wide read sent: `a-v0-Vb-book.xml` for `n` = 1, else the last `v`.
     - **`VERDICT <id>: ON ITS LEDGER` (exit 0):** that id's result, **LANDED ON ITS LEDGER**. Go on with the next voucher.
     - **Any other `VERDICT` (exit 3): `ON ITS SIBLING`, `NOT ON ITS LEDGER` or `NOT JUDGED`.** The landing is wrong, or not known to be right. **The row ends after this read:**
       - nothing more is written or read, but the restore;
       - RESULTS states it in one line, the first line of step 7's section: `WRONG LANDING: <id>: <the LANDED line>; <the VERDICT line>`. The label covers `NOT JUDGED` too (no new voucher, its number not found, the book changed): a landing not known to be right ends the row as a wrong one does;
       - **the operator tells the run coordinator at once**, as soon as the line is read, before anything else, and the hand-back says so again in its first line;
       - then step 9, the restore, and step 10.

       It would be a defect of a shipped post (`qa` to `ub`, `ka`) or a fact of the raw shape (`kb`). The coordinator decides where it is written, and nothing more is said of it in public text until then.

       This holds for `kb` too, though it is sent last and nothing is written after it: a wrong landing of `kb` also ends the row before step 8, and the scoped reads of the five before it are not sent. That is the run coordinator's condition for this row, kept as given.
   - **Step 7 has passed in full** only if every voucher is LANDED ON ITS LEDGER or REFUSED, and every send was answered with no dialog. Write it down, with the lines that show it.
8. **The reads after. Only after step 7 has passed in full.** First `k1`: `M-marks.xml`, then `count20.py --same-row a-k0-M-marks.xml a-k1-M-marks.xml`, recorded (a record, not a check: the vouchers moved the voucher mark). The book-wide read they are judged against, `BOOK`, is the last `v` sent. For each id, in the table's order (`n` = 1 to 6), each send once, each answer read by its reader before the next request, and a screenshot after each `a` send:
   - `r<n>ca`, `r<n>cb`: `C-v-<id>.xml`, then `cv.py` on each and `cv.py --same` on the two;
   - `r<n>qa`, `r<n>qb`: `Vm-v-<id>.xml`, then `wx.py` on each and `wx.py --same`;
   - `r<n>wa`, `r<n>wb`: `Vw-v-<id>.xml`, the same;
   - for `ka` and `kb` only (the folder holds no outstandings request of the others): `r<n>la`, `r<n>lb`: `L-lo-<id>-20260801.xml`, then `nm.py` on each and `wx.py --same` on the two;
   - then, before the next id, each one line, recorded whole: `python3 -I sv.py --judge2 <id> <form> a-r<n><x>a-<file> <BOOK>` for each form sent (`census`, `movement`, `window`, and `outstandings` where sent).
   - **An empty answer under (a)** is read by `wx.py` alone, and its pair by `wx.py --same`. Its `--judge2` line says `EMPTY ANSWER`, which is its result. No `cv.py` or `nm.py` line is run on it.
   - **An answer a reader reads never stops the row,** whatever it holds: an `OWN` below its count, an `OTHERS` above 0, a `DIFFER`, a `STATUS`, an empty answer under (a), and every exit 3 are results, recorded in full. Only an exit 1 of a reader, a dialog and a send with no answer are STOPs: "a STOP at step 8, id X".
   - After the last id: `e1`: `Vb-book.xml`, then `wx.py --same <BOOK> a-e1-Vb-book.xml`; then `e2`: `M-marks.xml`, then `count20.py --same-row a-k1-M-marks.xml a-e2-M-marks.xml`. EXPECT SAME MARKS: step 8 only read. On MARKS DIFFER: `e2r` once, record it, send nothing else, and STOP. If `e1` is not equal to `BOOK`, record both: the report says the book-wide read changed during step 8.
9. **Restore** BK by C6, with `restore-plan.txt` written first, as SC's step 9. Then `s9`: every file of `s2`, in `s2`'s order, each answer's sha256 equal to `s2`'s, and `count20.py --same-row a-s1-M-marks.xml a-s9-M-marks.xml` passes. **Any not equal stops the row:** send nothing more, and report before the close.
10. **Close** by C10. One tally.exe owns 9001.

**Stop**, with no key pressed, if Tally's window shows a dialog at any point but those the prompt rule and C6 name, or if the gateway stops answering. Screenshot the window, send it and the answers to the run coordinator, and ask before anything else is pressed or sent.

## Record
- **RESULTS begins with the `WRONG LANDING:` line if there is one** (step 7).
- For the set-up: as SC's record.
- For each voucher: its marks lines, its import's counters whole, `st.py`'s line, `rf.py`'s lines and the `j<n>x` marks line where they ran, and its `v<n>` lines (`wx.py`, `sv.py --landed` whole).
- For step 8: each pair of reads and its `--same` line, and every `JUDGE2` line.
- Each answer's bytes and sha256, its headers file whole, and each script's lines, whole. `run.py`'s log for each label, and how long each read took, except for the send of a STOP the STOP rule names. The window after each import and each shot named. Every prompt, message and busy screen, word for word, with the key that answered it and its time (UTC).
- The BK file and the restore, as C5 and C6 ask, with the data folder listed before and after. The release on Tally's screen, and the Gateway's date and period.

**Staging the evidence:** SC's rule. The check for 10 or more digits sets aside this book's registered GUID matched whole, and nothing else. Any other hit waits for the run coordinator. No script prints a `VOUCHERKEY`; `sv.py` prints no GUID.

## Expected
Estimates, not predictions.
- **Steps 1 to 5:** SC's answers.
- **Step 7:** no estimate for `qb` and `ub`: no row has sent a voucher naming a ledger whose key a sibling shares. For `qa`, `ua` and `ka`, `CREATED ONE` and LANDED ON ITS LEDGER, as their first invoices did. For `kb`, no estimate: SC's Part B read the altered ledger by that spelling, but no voucher has named it.
- **Step 8:** for each of `qa` to `ub` whose voucher landed, `OWN 2 OF 2, OTHERS 0` (its first invoice and this row's voucher); for `ka` and `kb`, the altered ledger's `OWN <k> OF <k>`, `<k>` 2 or 3 as `kb`'s voucher landed or was refused.
- **s9:** equal to s2 and s1.

## Decides
Each is PARTIAL: one book, one release, one sitting, master's import shape sent by curl, without the reads Bridge sends around it, and without the build's own post.
- **`qb` and `ub` LANDED ON ITS LEDGER:** on this book and release, a gateway voucher naming a ledger whose key its sibling shares, by an apostrophe or an underscore, lands on the ledger it names. Master's post admitting those names is right there.
- **A wrong landing of any id:** the gateway put a voucher on a ledger it did not name. For `qa` to `ub` and `ka` that is the shape master's post sends, and the coordinator decides where it is written.
- **`kb` LANDED ON ITS LEDGER (the altered ledger), or REFUSED:** how the gateway resolves the name a ledger's name list holds but its `NAME` attribute does not. Master refuses that voucher before any send, so this decides nothing about a shipped post. It informs whether that refusal is needed or conservative.
- **Step 8:** whether each scoped read of each name, with two or three vouchers on its ledger, holds exactly them.

It says nothing of vouchers made on Tally's screens, other marks, a voucher naming two such ledgers, an alteration or a pair made on Tally's screens, or the build's own post path.

## Code admits the trigger
**The design note,** traced before this sheet was sent. Lines are at master 7a3c9e49; this is a reading of the code, not a run of it.
- **The folded-twin refusal does not pair these names.**
  - Master's voucher post refuses a voucher naming a ledger whose fold keys match two or more live ledgers. The check is `folded_twins` (`src-tauri/src/agent_import.rs:708`), called at `src-tauri/src/agent_import_post.rs:985`, which refuses with `ledger_has_folded_twin` at `:999`.
  - Its keys are `twin_fold_keys` (`src-tauri/crates/bridge-tally-core/src/master_binding.rs:1947`), built from `comparison_key` (`:1891`), `master_identity_key` (`:1931`) and `verified_fold` (`:2107`). Between them they fold case, NFC, the dash and quote variants, whitespace runs, and `-` and `/` as a space.
  - None removes an apostrophe or turns an underscore into a space. So `qa` and `qb` have different keys, as do `ua` and `ub`. The post admits a voucher naming either, and sends it.
- **`kb`: master refuses the voucher before any send.**
  - The build (`build_import_xml`, `src-tauri/src/agent_import.rs:1420`) reads the ledger catalogue through `read_import_ledger_catalogue` (`:3092`). The names it matches against are `catalogue.catalog().names()` (`:3114`).
  - That is `StandardLedgerCatalog::names` (`src-tauri/crates/bridge-tally-protocol/src/standard_ledger_catalog.rs:232`): each row's `NAME` attribute (`name`, `:207`). The ledger's own name from `LANGUAGENAME.LIST`, where it differs, is kept apart as `stored_name` (`:212`) and is not among them.
  - For the altered ledger, that list holds `BRIDGE Sk Ak Am`, not `BRIDGE Sk Ak, Am`. `unbindable_masters` (called at `:1499`; `:5204`) matches the requested names through `masters_for_names` and `master_report` (`:5580`). No row's `NAME` attribute is `BRIDGE Sk Ak, Am`, so its match state is not `exact`.
  - The build refuses with `masters_not_exact` (`:5233`) before any file is written. `annotate_alias_of` can add an `alias_of` note to such an entry, but it leaves the refusal as it is.
  - At the post, the same check refuses with `import_masters_changed` (`src-tauri/src/agent_import_post.rs:971` and `:975`).
  - **So master would not admit that voucher at all.** The row's `J-kb.xml` import runs as a raw-shape measure: what the gateway does with a name a ledger's name list holds and its `NAME` attribute does not. It is not a measure of a request Bridge sends.
- **The voucher shape.** `J-<id>.xml` is master's import shape as rows MX to SC sent it (the control's invoice, `I-pl.xml`), not the build's output. The build's own post path is not sent.

## Not in this row
- ComplyEaze Bridge's own transport, or any Bridge build;
- any name inside `CHILDOF`;
- pairs `r` and `e`, a pair by case, a trailing line break, an alias, or any mark other than SK's; three or more ledgers sharing one key; a pair, an alteration or a voucher made on Tally's own screens;
- a voucher naming two of these ledgers, or a voucher of another type;
- the reads over more than one day, with another AlterID span, or on a book of real size; a settled bill;
- another to-date, book or release; a book with GST on; Educational mode.
