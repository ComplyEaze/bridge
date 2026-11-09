# 2433 N3: whether Tally stores a 1,000-character company name and a Devanagari name over 255 bytes whole, and whether a read naming the stored name reaches it, read raw with no Bridge build

Not for merge. Part of #2433, row N3. Written 2026-11-09. It takes about 50 minutes and needs no act by the owner: no Bridge build, no Terms, no installed extension and no tool call. The sheet sends raw reads only, by `send.py` with LAB-COMMON C3's own curl line. It makes two renames on Tally's own Company Alteration screen, and undoes each by restoring one backup. This is the write class of row N (`lab/2433-n-name-sheet`, 1cd8de6f) and row T1 before it: a rename on Tally's screen of `BRIDGE SHAPE LAB` only, with a backup and a restore check against the whole starting state. No Bridge write is involved, and no approval window is needed.

`LAB-COMMON.md` in this folder applies in full (v3, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`, 12,062 bytes). Where it and this sheet differ, the stricter rule holds. `SHA256.txt` lists every file here with its bytes; check it before step 0.

Marks: V means read at a named source, and U means not measured. **Every EXPECT on a ledger read is U, written as "record as found".** No read in this row has been sent live. Every outcome is recorded as found, never explained away.

## What it decides

Row N measured that Tally matches a 300-character ASCII name exactly, and not by its first 255 characters (`lab/2433-n-name-answers`, ab5ef4d8, partial). So Bridge's limit can rise to 300 bytes for ASCII names, the longest value measured. Two questions stay open, and the #2433 design waits on them before it settles the limit:

- **Tally's own limit.** Does Tally store a name much longer than 300 characters whole, and if not, where does it cut it? Does a read naming the stored name then reach the company?
- **Bytes or characters.** For a name whose letters take more than one byte, does Tally's limit count bytes or characters, and does a read naming the stored name match?

The book is the same synthetic one throughout, loaded alone. Only one thing changes between reads: the `SVCURRENTCOMPANY` text, or the stored name.

**The two names** (`n3.py pattern` prints both):
- **A:** 1,000 ASCII letters, 1,000 UTF-8 bytes, sha256 `0761c84b2f0a8e7f35ea1fade874099b31b79e753393ab487f68cd1338d7c027`. It is 100 blocks of 10: a two-letter block label (`AA`, `AB`, … `DV`), then `abcdefgh`. Any cut point can be read from where the name ends.
- **D:** 120 Devanagari consonants, U+0915 to U+0920, each repeated 10 times: 12 blocks, 360 UTF-8 bytes, sha256 `80af7109bb397656bfc82f160ac7ddbf19e3bab9b180e78125f15ec73003e2a5`. It holds no vowel sign or other combining mark, so it is the same under every Unicode normal form. Its first 85 characters are exactly 255 UTF-8 bytes, and its first 100 are 300.

**The reads.**
- **N1, reads only, with the stored name as found (16 characters).** The control through the stored name gives B. Then the ledger list is read through A and through D, which match no company. Row N measured that a 300-character name matching no company answers zero rows with no dialog. N1 measures the same for a 1,000-character value and a multi-byte one before either is stored.
- **A, after renaming the company to A.** The company list shows the stored name. Then one read each: through A whole, through the stored name if Tally cut A, and through the old short name.
- **D, after restoring and renaming the company to D.** The company list shows the stored name. Then one read each: through D whole, through D's first 85 characters (255 bytes), through the stored name if Tally cut D at any other point, and through the old short name.

| Read | Outcome | Decision |
|---|---|---|
| N1 `a1000`, N1 `d120` (no company has either name) | `STATUS` 1, 0 rows, no `LINEERROR`, no dialog | Row N's finding holds for a 1,000-character value and a multi-byte one. A and D's reads below can tell a match from no match. |
| N1 `a1000` or N1 `d120` | B rows, all THE COMPANY's | Tally answers an unmatched name for the loaded company. That is a trap for §9.11. The reads below that use the same name are recorded as found, with no decision drawn from them. |
| N1 `a1000` or N1 `d120` | a `LINEERROR`, or anything else | Recorded and named. A dialog is C7: STOP. |
| A `C` | the whole of A, 1,000 characters | Tally stores 1,000 ASCII characters whole. Tally's own limit, if it has one, is above 1,000. |
| A `C` | a prefix of A, k characters | Tally cuts a name typed on its screen at k characters. `n3.py native` prints k, its bytes and the block it ends in. `n3.py stored` writes the read through that prefix. |
| A `C` | not a prefix of A | Recorded and named. None of the A decisions below is drawn. |
| A `a1000`, with A stored whole | B rows, all THE COMPANY's, while A `short` answers 0 rows | Tally matches a 1,000-character ASCII name exactly. Bridge's limit can rise, to at most 1,000 bytes for ASCII names, the longest value measured. |
| A `a1000`, with A stored whole | 0 rows, `STATUS` 1 | Tally stores a 1,000-character name that a read cannot name. The limit stays at the longest name a read was seen to match, 300 bytes for ASCII. |
| A `stored`, with A cut at k | B rows, all THE COMPANY's | A read naming the k-character stored name matches. Bridge's limit can rise to k bytes for ASCII names, and no higher, because Tally stores no longer name typed on its screen. |
| A `stored`, with A cut at k | 0 rows, `STATUS` 1 | Tally stores a cut name that a read cannot name. Recorded and named; the limit stays at 300 bytes for ASCII. |
| A `a1000`, with A cut at k | B rows, all THE COMPANY's | Tally cuts the request's name too, so a longer name reaches a company stored under its prefix. That is a trap for §9.11: Bridge must refuse a name longer than k, never send it. |
| A `a1000`, with A cut at k | 0 rows, `STATUS` 1 | Matching stays exact: a name longer than the stored one matches nothing. |
| A `short` | B rows, all THE COMPANY's | After the rename, Tally still answers the old name. Recorded and named, as in row N. |
| D `C` | the whole of D, 120 characters, 360 bytes | Tally stores a 360-byte multi-byte name whole. Neither a 255-byte nor a 300-byte limit applies to names typed on its screen. |
| D `C` | a prefix of D, k characters | `n3.py native` prints k and its bytes. A cut at 85 characters, with A cut at 255, is a cut at 255 UTF-8 bytes. A cut at 100, with A cut at 300, is a cut at 300 UTF-8 bytes. A cut at the same character count as A's is a limit on characters. Any other k is recorded with both counts, and no unit is named. Whatever the unit, it is INFERRED from where the cuts land, since Tally's own encoding is not read. |
| D `C` | not a prefix of D (for example `?` in place of a letter) | Tally did not store D as typed. `n3.py native` prints where it first differs. None of the D decisions below is drawn. |
| D `d120`, with D stored whole | B rows, all THE COMPANY's, while D `d85` and D `short` answer 0 rows | Tally matches a 360-byte Devanagari name exactly. Bridge's limit, for a name of these letters, can rise to 360 bytes, the longest value measured. |
| D `d120`, with D stored whole | 0 rows, `STATUS` 1 | Tally stores a multi-byte name that a read cannot name. Recorded and named. Bridge's limit for multi-byte names stays at 255 bytes. |
| D `d85`, with D stored whole | B rows, all THE COMPANY's | Tally matches a multi-byte name on a 255-byte prefix. That is a trap for §9.11, and the limit stays at 255 bytes. |
| D `d85` or D `stored`, with D cut at k | B rows, all THE COMPANY's (when its name equals the stored one) | A read naming the stored multi-byte name matches. The limit can rise to that name's bytes, and no higher. |
| D `d120`, with D cut at k | B rows, all THE COMPANY's | Tally cuts the request's name too. That is a trap for §9.11: Bridge must refuse a name longer than the cut, never send it. |
| D `short` | B rows, all THE COMPANY's | As A `short`. |

## THE COMPANY and the book

The book is `BRIDGE SHAPE LAB`, synthetic and seeded by this project (V: `src-tauri/crates/bridge-tally-protocol/tests/fixtures/note-days/PROVENANCE.md`, Provenance, Book). Its GUID is `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, and its books start on 2025-04-01. It is loaded alone whenever anything is sent or keyed (C1). Licensed mode only: Education mode means STOP. Write down the Tally release shown on screen, or, as row N did, the file version of the running `tally.exe`.

- **The book guard.** Every marks read and company-list read names THE COMPANY's GUID. `measure.py marks`, `t2.py name`, `n3.py native` and `n3.py stored` stop with exit 3 when the row's GUID is not that GUID. A book with any other GUID means STOP: nothing is renamed, sent or restored.
- **One backup, BK,** taken at step 0 before either rename (C5), to a new empty folder `C:\bridge-lab\2433n3\bk\`. Both restores are from BK.
- **Each restore** is checked against the whole starting state: the COMPANY row's GUID, ALTVCHID and ALTMSTID equal to step 0's M1 (C6), and the name equal to step 0's (`t2.py name`, the same sha256). D's rename starts only after R1's check passes.
- **Its company number** is the one step 0's company-list read shows (`COMPANYNUMBER`); C5 and C6 use it.

## Files

Check each file against `SHA256.txt` before step 0; another hash means STOP. Work in `C:\bridge-lab\2433n3\`, a new folder, and copy every file there. Answers land in `C:\bridge-lab\2433n3\answers\`.

| file | source |
|---|---|
| `RUN-SHEET.md` | this sheet |
| `LAB-COMMON.md`, `.gitattributes`, `measure.py`, `C-company-list.utf16le.xml`, `M-marks-shape-lab.utf16le.xml`, `L-ledgers-shape-lab.utf16le.xml`, `t2.py` | byte copies of the same files in `lab/2433-n-name-sheet:lab/2433-n-name/` (1cd8de6f). Their sha256 values in `SHA256.txt` equal that sheet's. `L-ledgers-shape-lab` is a `List of Ledgers` read whose `SVCURRENTCOMPANY` is `BRIDGE SHAPE LAB`. `t2.py` is used for `name` and `ledgers` only; `measure.py` for `marks` only. |
| `send.py` | that sheet's `send.py`, with its list of requests naming this sheet's request files in place of row N's two, and one addition: it also takes the sha256 and bytes of `L-ledgers-a-stored.utf16le.xml` and `L-ledgers-d-stored.utf16le.xml`, and of no other file, from `SHA256-run.txt` |
| `n3.py` | new; `gen`, `native` and `stored`, below |
| `L-ledgers-a1000.utf16le.xml` | written by `n3.py gen`: `SVCURRENTCOMPANY` holds A; 3,704 bytes |
| `L-ledgers-d120.utf16le.xml` | written by `n3.py gen`: `SVCURRENTCOMPANY` holds D; 1,944 bytes |
| `L-ledgers-d85.utf16le.xml` | written by `n3.py gen`: `SVCURRENTCOMPANY` holds D's first 85 characters, 255 UTF-8 bytes; 1,874 bytes |
| `SHA256.txt` | the sha256 and bytes of every other file |

Written during the run, only when Tally cuts a name: `L-ledgers-a-stored.utf16le.xml` and `L-ledgers-d-stored.utf16le.xml`, by `n3.py stored`, each listed by it in `SHA256-run.txt`. Each is 1,704 bytes plus 2 for each character of the stored name.

**`n3.py`** (`python -I n3.py …`; exit 3 means STOP, 2 a usage error):
- `gen` writes the three files above from `L-ledgers-shape-lab.utf16le.xml`. For each, it asserts that the company name is the only change, byte for byte, and prints each file's bytes and sha256. Neither name needs XML escaping.
- `native <answer> <guid> a|d` reads a company-list answer: the GUID's one row (else STOP, the book guard), its `NAME` attribute and `NAME` elements, each with its characters, UTF-8 bytes and sha256, whether it is the whole name, and, when it is a prefix, how many characters and bytes, and the block it ends in. A value that is not a prefix prints where it first differs.
- `stored <answer> <guid> a|d`: when the stored name is a prefix of A (or D) shorter than it, it writes the stored-name read. The name in that request is cut from A or D in the script, never copied from the answer. It appends that file's line to `SHA256-run.txt`. When the stored name is the whole name, or not a prefix of it, it writes nothing and says so. It writes each file once.

**Checked when the sheet was written:** `n3.py gen` printed the sha256 values in `SHA256.txt` for its three files. `n3.py pattern` printed A's and D's sha256 above. The seven copied files hash equal to 1cd8de6f's. `n3.py native` and `n3.py stored` were checked on synthetic answers only, made from row N's step 0 answer: A whole, A cut at 255, D whole, D cut at 100, a name that is neither, and a GUID that is not the row's. `send.py` was checked with a stand-in for `curl.exe` only. One send of each request file passed. A stored-name file was refused before `n3.py stored` wrote it, and after one byte of it was changed. A resend, and a file that is not a request of this sheet, were refused, even when `SHA256-run.txt` listed it. Nothing here has been sent to Tally (U).

## Rules

- One Tally runs on port 9001, with THE COMPANY loaded alone. No Bridge process takes part. Before step 0, run `Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'" | Format-List ProcessId, ExecutablePath`; an error is an error, never "none found". Record every process found, end none, and let no chat call a Bridge tool during the sitting.
- **Requests sent:** only the request files above, each a read, by `send.py`, plus C2's `GET /status`. No raw write is sent.
- **The writes:** two renames on Tally's Company Alteration screen, after BK, each undone by a restore of BK. Nothing else in the book is changed: on the alteration screen, only the Name field is touched.
- **The menus,** as row N found them: open Data and Company by clicking the `Y: Data` and `K: Company` labels at the Gateway. In row N, Alt+Y opened TallyCapital instead; do not press it.
- **New shapes (C3):** after every send in N1, A and D, screenshot the whole Tally window before the next request. A dialog is C7: press no key, send nothing, STOP.
- **On the alteration screen:** if the Name field stops taking characters before the whole name is typed, screenshot it, record where the field ends, and accept the screen as below; that is a result. If accepting shows any message other than the screen's own accept prompt, it is C7: screenshot it, record it word for word, press no key, STOP.
- **A ledger read is a result whatever it answers.** Zero rows, another company's rows, or a `LINEERROR` is recorded by `t2.py ledgers`, not a STOP. Only `send.py`'s own stops (the request failed), an answer that does not parse, and the book guard are STOPs.
- **No marks read is sent while the company is renamed,** because the marks request names the company by its old name.
- Delete nothing, and end no process. **STD STOP** means: change nothing, screenshot the whole screen, save what exists with its size and sha256, report to the run coordinator, and key nothing.

## Steps

**0. Load, read and back up.**
1. `date` (UTC). Copy the files to `C:\bridge-lab\2433n3\` and check each against `SHA256.txt`. Then:
   - `python -I n3.py gen`: EXPECT the three files' sha256 to equal their lines in `SHA256.txt`. Another value means STOP.
   - `python -I n3.py pattern`: EXPECT A's and D's sha256 above.
2. Load THE COMPANY alone (C1). Run LAB-COMMON C2 step 0 in full: a to c, and d as follows.
   - `python -I send.py s0 M-marks-shape-lab.utf16le.xml`.
   - `python -I measure.py marks answers\s0-M-marks-shape-lab.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`: record ALTVCHID and ALTMSTID as M0. A GUID stop here is the book guard: STOP. V: row N read 126 and 300.
3. `python -I send.py s0 C-company-list.utf16le.xml`, then `python -I t2.py name answers\s0-C-company-list.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. Record the name as found, and `COMPANYNUMBER` from the answer. EXPECT (V: `lab/2433-n-name-answers`, step 0) `"BRIDGE SHAPE LAB"`, 16 characters, sha256 `54510f216f9b0854d58ad42e7d5b7fbbc38b130d9a6038ae89aab528dc1e3f7b`. Another name means STOP: the book is not as the decisions assume.
4. **Take BK by C5** to `C:\bridge-lab\2433n3\bk\`, listed empty first. Record the file's name, bytes and sha256, and the backup's local time to the minute. Then run `send.py s0b M-marks-shape-lab.utf16le.xml` and `measure.py marks`, and record M1. M1 is the mark both restores are checked against. A difference from M0 is recorded, not a STOP.

**N1. Reads only, with the stored name as found.** For each read, `send.py n1 <file>`, then screenshot the Tally window, then `t2.py ledgers answers\n1-<file> 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. **Record as found:**
- from `log-send.txt`, the HTTP status;
- `STATUS`;
- the row count;
- how many GUIDs begin with THE COMPANY's;
- how many `BRIDGECOMPANYGUID` values equal it;
- every `LINEERROR`.

1. **The control:** `L-ledgers-shape-lab.utf16le.xml`. Its row count is B. V: row N read B = 45. If B is 0, every later read still runs, but none can tell a match from no match: record that.
2. **A, which matches no company:** `L-ledgers-a1000.utf16le.xml`.
3. **D, which matches no company:** `L-ledgers-d120.utf16le.xml`.

**A. The stored name is A.**
1. Screenshot the Gateway. Open Company Alteration for THE COMPANY: click the `K: Company` label, Down to Alter, Enter. Write down the keys. On the Name field only:
   - clear it (row N used End, then Backspace 16 times), and screenshot the empty field;
   - type the 1,000 characters of A that `n3.py pattern` prints, in order (in row N, the input tool pasted typed text);
   - screenshot the field before accepting;
   - accept the screen (row N used Ctrl+A) and answer only its own accept prompt. Any other message is C7: STOP.

   Then screenshot the whole Tally window (no dialog) and the Gateway.
2. `send.py a C-company-list.utf16le.xml`, then screenshot the Tally window. Then `n3.py native answers\a-C-company-list.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346 a`. **Record as found,** for the `NAME` attribute and each `NAME` element: the characters and UTF-8 bytes, the sha256, whether it is the whole of A or a prefix (and where it ends), and whether the attribute and the element are equal.
3. `n3.py stored answers\a-C-company-list.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346 a`. Record what it prints. If it wrote `L-ledgers-a-stored.utf16le.xml`, record its bytes and sha256.
4. Each read as in N1, with the label `a`, recording the same items:
   1. **A whole:** `L-ledgers-a1000.utf16le.xml`.
   2. **The stored name,** only if step 3 wrote it: `L-ledgers-a-stored.utf16le.xml`.
   3. **The old short name:** `L-ledgers-shape-lab.utf16le.xml`.

   If step 2 found the stored name is not a prefix of A, the reads still run, but no A decision is drawn from them.

**R1. Restore and check.**
1. **Restore from BK (C6),** under the `Y: Data` label, Backup & Restore, Restore, answering its two prompts only. Row N recorded "Accept ?", answered y, and "Company data folder (<company number>) already exists...", answered r (Replace). Any other prompt is C7: STOP. Load THE COMPANY alone. If Select Company still lists A, as it listed row N's pattern after its restore, load THE COMPANY's row by its number with Enter, and record that. Screenshot the window and the Gateway.
2. `send.py r1 M-marks-shape-lab.utf16le.xml` and `measure.py marks`: EXPECT the GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, and ALTVCHID and ALTMSTID equal to M1. Then `send.py r1 C-company-list.utf16le.xml` and `t2.py name`: EXPECT step 0.3's name and sha256. If either differs, STOP; the run coordinator decides, and D does not start.

**D. The stored name is D.**
1. As A.1, with D: clear the Name field (End, then Backspace 16 times), type the 120 characters of D that `n3.py pattern` prints, screenshot the field before accepting, and accept the screen. Then screenshot the whole Tally window and the Gateway.
2. `send.py d C-company-list.utf16le.xml`, then screenshot the Tally window. Then `n3.py native answers\d-C-company-list.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346 d`. **Record as found,** the same items as A.2.
3. `n3.py stored answers\d-C-company-list.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346 d`. Record what it prints. If it wrote `L-ledgers-d-stored.utf16le.xml`, record its bytes and sha256.
4. Each read as in N1, with the label `d`, recording the same items:
   1. **D whole:** `L-ledgers-d120.utf16le.xml`.
   2. **D's first 85 characters, 255 bytes:** `L-ledgers-d85.utf16le.xml`.
   3. **The stored name,** only if step 3 wrote it: `L-ledgers-d-stored.utf16le.xml`.
   4. **The old short name:** `L-ledgers-shape-lab.utf16le.xml`.

   If step 2 found D not stored as typed, the reads still run, but no D decision is drawn from them.

**R2. Restore and check.** As R1, from the same BK, with the label `r2`. EXPECT M1 and step 0.3's name. If either differs, STOP; the run coordinator decides.

**Close.** Shut THE COMPANY by Tally's own Shut Company (click the `K: Company` label, h, Enter, as at row N's close). Run LAB-COMMON C10. Keep `log-send.txt` and `SHA256-run.txt` whole.

**Book state after each step:**

| step | name | marks |
|---|---|---|
| 0 | as found | M0, then M1 after BK |
| N1 | as found | not read; B from N1.1 |
| A | A as typed, as Tally stored it | not read |
| R1 | restored from BK; EXPECT as found | EXPECT M1 |
| D | D as typed, as Tally stored it | not read |
| R2 | restored from BK; EXPECT as found | EXPECT M1 |
| close | shut | |

If a STOP comes after a rename and before its restore, the book holds the renamed company. BK is the way back, restored only on the run coordinator's word (C8).

## Result

**PASS (the sheet ran):** every step answered and was recorded; both restores checked equal to M1 and to the name as found; no request was sent twice; and no dialog was answered except those named.

**FAIL:** any one of these:
- a send recorded as an answer when `send.py` stopped;
- a request sent twice;
- a dialog answered that is not named;
- a restore not equal to M1 and the name as found.

The decisions are the table under "What it decides". Every finding is marked verified, partial or unverified; one run is partial at most.

## Not measured

- A name between 300 and 1,000 characters, when A is stored whole, so the exact point of any limit above 300.
- Letters other than these Devanagari consonants: vowel signs, other combining marks, other scripts, and letters outside the Basic Multilingual Plane.
- Whether Tally stores D in UTF-8, UTF-16 or another form. A unit named from where the cuts land is INFERRED.
- A name with characters that need XML escaping.
- A name set by an import rather than typed on Tally's screen.
- A voucher read, or a write, through a long name (only the ledger list is sent).
- Two companies loaded at once.
- Any Bridge build.
- Another Tally release.

## Evidence

The evidence goes on a new branch, `lab/2433-n3-name-answers` (LAB-COMMON C11). It is pushed only after the pre-push scan and the run coordinator's yes, with no AI attribution in any commit.
- `RESULTS.md` holds:
  - M0, M1 and both restores' marks;
  - the BK file's name, bytes and sha256;
  - A's and D's stored names as `n3.py native` printed them, and what `n3.py stored` printed;
  - each ledger read as `t2.py ledgers` printed it, with its HTTP status from `log-send.txt`;
  - every prompt and message, word for word, with the key that answered it;
  - where the Name field ended while typing, if it stopped taking characters;
  - which row of the decision table each read fell in.
- Every answer and headers file under `answers\`, `log-send.txt`, `SHA256-run.txt`, any stored-name request file, and the `n3.py`, `measure.py` and `t2.py` output whole, each with its size and sha256. The book is synthetic, so the answers are pushed. A ledger answer that holds a number which may not go in a public file stays on the PC, recorded by size and sha256, with its counts in `RESULTS.md`, as `lab/2433-n-name-answers` did. Each company-list answer goes out with its `COMPANYNUMBER` value replaced, as row N's did. If any file holds the home folder or the user name, it is replaced by `<home>` or `<user>`, with the original's sha256 recorded.
- `notes.txt` (UTC times, one line per step), `SCREENSHOTS.txt` (names, bytes and sha256; the captures stay on the PC), `restore-plan.txt`, and `SHA256.txt` (`# sha256  bytes  file`).

## Report (one message)

- M0 and M1;
- B;
- what N1's two unmatched reads answered;
- A's stored name, in characters and bytes, and where it ends;
- what each of A's reads answered (HTTP status, `STATUS`, rows, GUIDs of THE COMPANY's);
- D's stored name, in characters and bytes, and whether it is D as typed;
- what each of D's reads answered;
- both restore checks;
- the decision-table rows that apply;
- a statement that nothing but the two renames was written and nothing was deleted.
