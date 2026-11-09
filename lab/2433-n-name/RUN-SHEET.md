# 2433 N: what a company-scoped read answers when `SVCURRENTCOMPANY` holds a name over 255 bytes, read raw with no Bridge build

Not for merge. Part of #2433, row N. Written 2026-11-09. It takes about 40 minutes and needs no act by the owner: no Bridge build, no Terms, no installed extension and no tool call. The sheet sends raw reads only, by `send.py` with LAB-COMMON C3's own curl line. It makes one rename on Tally's own Company Alteration screen and undoes it by restoring one backup. This is the write class of row T1 of `lab/2433-name-sheet` (27b0241e): a rename on Tally's screen of `BRIDGE SHAPE LAB` only, with a backup and a restore check against the whole starting state. No Bridge write is involved, and no approval window is needed.

`LAB-COMMON.md` in this folder applies in full (v3, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`, 12,062 bytes). Where it and this sheet differ, the stricter rule holds. `SHA256.txt` lists every file here with its bytes; check it before step 0.

Marks: V means read at a named source, and U means not measured. **Every EXPECT on a ledger read is U, written as "record as found".** No read in this row has been sent live. Every outcome is recorded as found, never explained away.

## What it decides

Bridge refuses any company name over 255 bytes, and that limit has no Tally basis. Row T1 measured that Tally stores a 300-character name whole (`lab/2433-name-answers`, 9d4d226b, partial). The open question is whether a read whose `SVCURRENTCOMPANY` holds a name over 255 bytes reaches the company Tally stored under that name. The #2433 design keeps the 255-byte limit until this row answers that, and raises it only to a value this row observes.

The book is the same synthetic one throughout, loaded alone. Only one thing changes between reads: the `SVCURRENTCOMPANY` text, or, once only, the stored name.

- **N1, reads only, with the stored name as found (16 characters).** The row first reads the ledger list through the stored name, as a control. It then reads it through the 300-character pattern, which matches no company. Finding 2 (§9.11) says a name that matches no company answers zero rows. This read measures whether that holds for a value over 255 bytes, and whether such a value makes Tally answer with a line error or a dialog. It also settles the point row T2 left open. If a name matching no company answers zero rows while the company is loaded, then T2's 45 rows were a match on the name, and not Tally answering for the loaded company.
- **N2, after renaming the company to the pattern.** One read each through the full pattern, its first 255 characters, and the old short name.

| Read | Outcome | Decision |
|---|---|---|
| N1 `long` (no company has that name) | `STATUS` 1, 0 rows, no `LINEERROR`, no dialog | Finding 2 holds for a 300-character value. Since N1 `short` answered B rows, a name that matches no company answers nothing even while the company is loaded. That makes T2's 45 rows a match, and it lets N2 tell a match from no match. |
| N1 `long` | B rows, all THE COMPANY's | Tally answers an unmatched long name for the loaded company. That is a trap for §9.11. N2 cannot then tell a match from Tally answering for the loaded company: record N2 as found, with no decision drawn from it. |
| N1 `long` | a `LINEERROR`, or anything else | Recorded and named. A dialog is C7: STOP. |
| N2 `long` | B rows, all THE COMPANY's, while N2 `prefix255` and N2 `short` answer 0 rows | Tally matches a 300-character name exactly. Bridge's limit can rise, to at most 300 bytes for ASCII names, the longest value measured. Multi-byte names stay unmeasured. |
| N2 `long` | 0 rows, `STATUS` 1 | Tally stores a name that a read cannot name. Bridge keeps its 255-byte limit, and the limit now rests on evidence. |
| N2 `prefix255` | B rows, all THE COMPANY's | Tally matches on a prefix. That is a trap for §9.11, and the limit stays at 255. |
| N2 `short` | B rows, all THE COMPANY's | After the rename, Tally still answers the old name. That is recorded and named, because it bears on a company renamed while Bridge holds its old name. |
| N2 `C` | the stored name is not the whole pattern | N2's reads are recorded, but none of the N2 decisions above is drawn from them. |

## THE COMPANY and the book

The book is `BRIDGE SHAPE LAB`, synthetic and seeded by this project (V: `src-tauri/crates/bridge-tally-protocol/tests/fixtures/note-days/PROVENANCE.md`, Provenance, Book). Its GUID is `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, and its books start on 2025-04-01. It is loaded alone whenever anything is sent or keyed (C1). Licensed mode only: Education mode means STOP. Write down the Tally release shown on screen.

- **The book guard.** Every marks read and company-list read names THE COMPANY's GUID. `measure.py marks` and `measure.py native` stop with exit 3 when the row's GUID is not that GUID. A book with any other GUID means STOP: nothing is renamed, sent or restored.
- **One backup, BK,** taken at step 0 before the rename (C5), to a new empty folder `C:\bridge-lab\2433n\bk\`. The restore is from BK.
- **The restore** is checked against the whole starting state: the COMPANY row's GUID, ALTVCHID and ALTMSTID equal to step 0's M1 (C6), and the name equal to step 0's (`t2.py name`, the same sha256).
- **Its company number** is the one step 0's company-list read shows (`COMPANYNUMBER`); C5 and C6 use it.

## Files

Check each file against `SHA256.txt` before step 0; another hash means STOP. Work in `C:\bridge-lab\2433n\`, a new folder, and copy every file there. Answers land in `C:\bridge-lab\2433n\answers\`.

| file | source |
|---|---|
| `RUN-SHEET.md` | this sheet |
| `LAB-COMMON.md`, `.gitattributes`, `measure.py`, `C-company-list.utf16le.xml`, `M-marks-shape-lab.utf16le.xml`, `L-ledgers-shape-lab.utf16le.xml`, `t2.py` | byte copies of the same files in `lab/2433-name-sheet:lab/2433-name/` (27b0241e). Their sha256 values in `SHA256.txt` equal that sheet's. `L-ledgers-shape-lab` is a byte copy of `note-days/shape_lab_ledgers_1_request.utf16le.xml`: a `List of Ledgers` read whose `SVCURRENTCOMPANY` is `BRIDGE SHAPE LAB`, with `COMPUTE` `BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany`. `t2.py` is used here for `name` and `ledgers` only; its `gen` is not run. `measure.py` is used for `pattern`, `native` and `marks` only. |
| `send.py` | that sheet's `send.py`, with one line changed: its list of requests names this sheet's five request files in place of `L-ledgers-spaced.utf16le.xml` |
| `n.py` | new; `gen`, below |
| `L-ledgers-long.utf16le.xml` | written by `n.py gen`: `SVCURRENTCOMPANY` holds the 300-character pattern; 2,304 bytes |
| `L-ledgers-prefix255.utf16le.xml` | written by `n.py gen`: `SVCURRENTCOMPANY` holds the pattern's first 255 characters (blocks `A` to `Y`, then five `Z`); 2,214 bytes |
| `SHA256.txt` | the sha256 and bytes of every other file |

**`n.py gen`** (`python -I n.py gen`; exit 3 means STOP) writes the two files above from `L-ledgers-shape-lab.utf16le.xml`. For each, it asserts that the company name is the only change, byte for byte, and prints the pattern's sha256 and each file's bytes and sha256. The pattern is the one `measure.py pattern` prints: 30 blocks of 10, `A` to `Z` and then `a` to `d`, 300 characters, sha256 `1d6366e1ccaa762b2da904319dc3a5385513fc401f1c814bb7a16bb0a03e433e`. It holds ASCII letters only, so nothing in it needs XML escaping.

**Checked when the sheet was written:** `n.py gen` printed the sha256 values in `SHA256.txt` for both files. `measure.py pattern` printed the pattern's sha256 above. The seven copied files hash equal to 27b0241e's. `send.py` was checked with a stand-in for `curl.exe` only: one send of each new request file passed, and both a resend and a file that is not a request of this sheet were refused. Nothing here has been sent to Tally (U).

## Rules

- One Tally runs on port 9001, with THE COMPANY loaded alone. No Bridge process takes part. Before step 0, run `Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'" | Format-List ProcessId, ExecutablePath`; an error is an error, never "none found". Record every process found, end none, and let no chat call a Bridge tool during the sitting.
- **Requests sent:** only the five request files above, each a read, by `send.py`, plus C2's `GET /status`. No raw write is sent.
- **The write:** one rename on Tally's Company Alteration screen, after BK, undone by a restore of BK. Nothing else in the book is changed: on the alteration screen, only the Name field is touched.
- **New shapes (C3):** after each send of `L-ledgers-long` or `L-ledgers-prefix255`, and after every send while the company is renamed, screenshot the whole Tally window before the next request. A dialog is C7: press no key, send nothing, STOP.
- **A ledger read is a result whatever it answers.** Zero rows, another company's rows, or a `LINEERROR` is recorded by `t2.py ledgers`, not a STOP. Only `send.py`'s own stops (the request failed), an answer that does not parse, and the book guard are STOPs.
- **No marks read is sent while the company is renamed,** because the marks request names the company by its old name.
- Delete nothing, and end no process. **STD STOP** means: change nothing, screenshot the whole screen, save what exists with its size and sha256, report to the run coordinator, and key nothing.

## Steps

**0. Load, read and back up.**
1. `date` (UTC). Copy the files to `C:\bridge-lab\2433n\` and check each against `SHA256.txt`. Then:
   - `python -I n.py gen`: EXPECT both files' sha256 to equal their lines in `SHA256.txt`. Another value means STOP.
   - `python -I measure.py pattern`: EXPECT the pattern's sha256 above.
2. Load THE COMPANY alone (C1). Run LAB-COMMON C2 step 0 in full: a to c, and d as follows.
   - `python -I send.py s0 M-marks-shape-lab.utf16le.xml`.
   - `python -I measure.py marks answers\s0-M-marks-shape-lab.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`: record ALTVCHID and ALTMSTID as M0. A GUID stop here is the book guard: STOP.
3. `python -I send.py s0 C-company-list.utf16le.xml`, then `python -I t2.py name answers\s0-C-company-list.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. Record the name as found, and `COMPANYNUMBER` from the answer. EXPECT (V: `lab/2433-name-answers`, step 0) `"BRIDGE SHAPE LAB"`, 16 characters, sha256 `54510f216f9b0854d58ad42e7d5b7fbbc38b130d9a6038ae89aab528dc1e3f7b`. Another name means STOP: the book is not as the decisions assume.
4. **Take BK by C5** to `C:\bridge-lab\2433n\bk\`. Record the file's name, bytes and sha256, and the backup's local time to the minute. Then run `send.py s0b M-marks-shape-lab.utf16le.xml` and `measure.py marks`, and record M1. M1 is the mark the restore is checked against. A difference from M0 is recorded, not a STOP.

**N1. Reads only, with the stored name as found.**
1. **The control:** `send.py n1 L-ledgers-shape-lab.utf16le.xml`, then `t2.py ledgers answers\n1-L-ledgers-shape-lab.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. **Record as found:**
   - from `log-send.txt`, the HTTP status;
   - `STATUS`;
   - the row count, B;
   - how many GUIDs begin with THE COMPANY's;
   - how many `BRIDGECOMPANYGUID` values equal it;
   - every `LINEERROR`.

   V: `lab/2433-name-answers` read B = 45. If B is 0, every later read still runs, but none can tell a match from no match: record that.
2. **The long name, which matches no company:** `send.py n1 L-ledgers-long.utf16le.xml`, then screenshot the Tally window. Then `t2.py ledgers answers\n1-L-ledgers-long.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. **Record as found,** the same items as step 1.

**N2. The stored name is the pattern.**
1. Screenshot the Gateway. Open Company Alteration for THE COMPANY from Tally's own menu, as T1.1 did: Alt+K, Down to Alter, Enter. Write down the keys. On the Name field only:
   - clear it (T1.1 used End, then Backspace 16 times);
   - type the 300 characters `measure.py pattern` prints, in order;
   - screenshot the field before accepting;
   - accept the screen (T1.1 used Ctrl+A) and answer only its own accept prompt. Any other prompt is C7: STOP.

   Then screenshot the whole Tally window (no dialog) and the Gateway.
2. `send.py n2 C-company-list.utf16le.xml`, then screenshot the Tally window. Then `measure.py native answers\n2-C-company-list.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346 --pattern`. **Record as found,** for the `NAME` attribute and each `NAME` element: the characters and UTF-8 bytes, the sha256, whether it is the whole pattern, and whether the attribute and the element are equal. V: T1 read the whole pattern, 300 characters. If it is not the whole pattern, steps 3 to 5 still run, but no N2 decision is drawn from them.
3. **The full pattern:** `send.py n2 L-ledgers-long.utf16le.xml`, then screenshot the window. Then `t2.py ledgers answers\n2-L-ledgers-long.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. Record as found, the same items as N1.1.
4. **The first 255 characters:** `send.py n2 L-ledgers-prefix255.utf16le.xml`, then screenshot the window. Then `t2.py ledgers answers\n2-L-ledgers-prefix255.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. Record as found.
5. **The old short name:** `send.py n2 L-ledgers-shape-lab.utf16le.xml`, then screenshot the window. Then `t2.py ledgers answers\n2-L-ledgers-shape-lab.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. Record as found.

**R. Restore and check.**
1. **Restore from BK (C6),** answering its two prompts only. T1.3 recorded "Accept ?", answered y, and "Company data folder (<company number>) already exists...", answered r (Replace). Any other prompt is C7: STOP. Load THE COMPANY alone. If Select Company still lists the pattern, as it did after T1's restore, load THE COMPANY's row by its number with Enter, and record that. Screenshot the window and the Gateway.
2. `send.py r M-marks-shape-lab.utf16le.xml` and `measure.py marks`: EXPECT the GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, and ALTVCHID and ALTMSTID equal to M1. Then `send.py r C-company-list.utf16le.xml` and `t2.py name`: EXPECT step 0.3's name and sha256. If either differs, STOP; the run coordinator decides.

**Close.** Shut THE COMPANY by Tally's own Shut Company (Alt+K, h, Enter, as at T1's close). Run LAB-COMMON C10. Keep `log-send.txt` whole.

**Book state after each step:**

| step | name | marks |
|---|---|---|
| 0 | as found | M0, then M1 after BK |
| N1 | as found | not read; B from N1.1 |
| N2 | the typed pattern, as Tally stored it | not read |
| R | restored from BK; EXPECT as found | EXPECT M1 |
| close | shut | |

If a STOP comes after the rename and before the restore, the book holds the renamed company. BK is the way back, restored only on the run coordinator's word (C8).

## Result

**PASS (the sheet ran):** every step answered and was recorded; the restore checked equal to M1 and to the name as found; no request was sent twice; and no dialog was answered except those named.

**FAIL:** any one of these:
- a send recorded as an answer when `send.py` stopped;
- a request sent twice;
- a dialog answered that is not named;
- a restore not equal to M1 and the name as found.

The decisions are the table under "What it decides". Every finding is marked verified, partial or unverified; one run is partial at most.

## Not measured

- A name over 300 characters, so Tally's own limit, if it has one.
- Multi-byte letters, and whether a limit counts characters or bytes.
- A name with characters that need XML escaping.
- A voucher read, or a write, through a long name (only the ledger list is sent).
- Two companies loaded at once.
- Any Bridge build.
- Another Tally release.

## Evidence

The evidence goes on a new branch, `lab/2433-n-name-answers` (LAB-COMMON C11). It is pushed only after the pre-push scan and the run coordinator's yes, with no AI attribution in any commit.
- `RESULTS.md` holds:
  - M0, M1 and the restore's marks;
  - the BK file's name, bytes and sha256;
  - N2's stored name as `measure.py native` printed it;
  - each ledger read as `t2.py ledgers` printed it, with its HTTP status from `log-send.txt`;
  - every prompt, word for word, with the key that answered it;
  - which row of the decision table each read fell in.
- Every answer and headers file under `answers\`, `log-send.txt`, and the `n.py`, `measure.py` and `t2.py` output whole, each with its size and sha256. The book is synthetic, so the answers are pushed. A ledger answer that holds a number which may not go in a public file stays on the PC, recorded by size and sha256, with its counts in `RESULTS.md`, as `lab/2433-name-answers` did. If any file holds the home folder or the user name, it is replaced by `<home>` or `<user>`, with the original's sha256 recorded.
- `notes.txt` (UTC times, one line per step), `SCREENSHOTS.txt` (names, bytes and sha256; the captures stay on the PC), `restore-plan.txt`, and `SHA256.txt` (`# sha256  bytes  file`).

## Report (one message)

- M0 and M1;
- B;
- what N1's long read answered;
- N2's stored name, in characters and bytes;
- what each of N2's three reads answered (HTTP status, `STATUS`, rows, GUIDs of THE COMPANY's);
- the restore check;
- the decision-table rows that apply;
- a statement that nothing but the one rename was written and nothing was deleted.
