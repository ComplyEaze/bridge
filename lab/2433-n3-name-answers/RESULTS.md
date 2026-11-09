# 2433 N3: Tally stores a 1,000-character ASCII company name and a 120-letter Devanagari name whole, and a ledger read reaches the company only by the whole name

Run on 9 Nov 2026, 14:33Z to 14:47Z, on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2433-n3-name-sheet` (e7985400), in one sitting on `BRIDGE SHAPE LAB` (GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, synthetic). Raw reads only, sent by `send.py`; no Bridge build, no tool call, no Terms, no extension. Two renames on Tally's Company Alteration screen, each undone by a restore of one backup. Every value below is M, measured by this run, in one run: **partial**.

**Result.**
- **B = 45.** N1's control, the ledger list through the stored name `BRIDGE SHAPE LAB`, answered 45 rows, every GUID and every `BRIDGECOMPANYGUID` THE COMPANY's, no `LINEERROR`.
- **N1 `a1000` and N1 `d120`** (A and D, which no company had): each HTTP 200, `STATUS` 1, **0 rows**, no `LINEERROR`, no dialog. Decision table row 1: row N's finding holds for a 1,000-character value and a multi-byte one.
- **A `C`:** after the rename to A, the `NAME` attribute and the `NAME` element each hold **the whole of A**: 1,000 characters, 1,000 UTF-8 bytes, sha256 `0761c84b…`, equal to each other. The Name field took every character. `n3.py stored` wrote nothing.
- **A `a1000`:** **45 rows**, all THE COMPANY's, `STATUS` 1, no `LINEERROR`. **A `short`** (the old name): 0 rows. Decision table: **Tally matches a 1,000-character ASCII name exactly.** Bridge's limit can rise, to at most 1,000 bytes for ASCII names, the longest value measured.
- **D `C`:** after restoring and renaming to D, the `NAME` attribute and the `NAME` element each hold **the whole of D**: 120 characters, 360 UTF-8 bytes, sha256 `80af7109…`, equal to each other. `n3.py stored` wrote nothing.
- **D `d120`:** **45 rows**, all THE COMPANY's, `STATUS` 1, no `LINEERROR`. **D `d85`** (the first 85 characters, 255 bytes): 0 rows. **D `short`:** 0 rows. Decision table: **Tally matches a 360-byte Devanagari name exactly.** Bridge's limit for a name of these letters can rise to 360 bytes, the longest value measured. Neither a 255-byte nor a 300-byte limit applies to names typed on Tally's screen.
- Both restores from BK read equal to step 0: the GUID, ALTVCHID 126 and ALTMSTID 300 (M1, byte-equal), and the name as found (the company-list answer byte-equal to step 0's). Nothing but the two renames was written. Nothing was deleted.

## Step 0
- Sheet files: all 13 equal to `SHA256.txt` at e7985400. `n3.py gen` wrote the three request files with the sha256 values in `SHA256.txt`. `n3.py pattern`: A 1,000 characters, 1,000 bytes, sha256 `0761c84b2f0a8e7f35ea1fade874099b31b79e753393ab487f68cd1338d7c027`; D 120 characters, 360 bytes, sha256 `80af7109bb397656bfc82f160ac7ddbf19e3bab9b180e78125f15ec73003e2a5`; both as the sheet.
- Before step 0, THE COMPANY's own `Company.1800` held the GUID `3a6bd6e1-…` (UTF-16LE) and the name BRIDGE SHAPE LAB.
- Bridge processes (the sheet's `Get-CimInstance` line): two `bridge_mcp.exe`, the installed extension. None was ended. No Bridge tool was called during the sitting. The listing stays local.
- C2: one `tally.exe`, which alone owns 9001; `GET /status` 200. Loaded alone by Select Company (`SHAPE` typed, one row, Enter, no prompt). The Gateway: BRIDGE SHAPE LAB alone, current period 1-Apr-26 to 31-Mar-27, current date Wednesday 20-May-2026, date of last entry 20-May-26, the company row carrying "(e) (Schedule Backup)" and the footnote "(e) Data exceptions exist".
- **M0** (s0 marks): one COMPANY row, the GUID, ALTVCHID 126, ALTMSTID 300.
- s0 company list: `"BRIDGE SHAPE LAB"`, 16 characters, 16 bytes, sha256 `54510f216f9b0854d58ad42e7d5b7fbbc38b130d9a6038ae89aab528dc1e3f7b`, as the sheet.
- **BK** (C5): the `Y: Data` label (clicked), Backup & Restore, Backup. The Backup Data Path was changed from C:\ by Specify Path to the new empty folder `<LAB_DIR>\bk` (listed empty before). BRIDGE SHAPE LAB alone; End of List; "Accept ?" y; no other message. `TDBK1800_<company number>.001`, 115,721 B, sha256 `2a4c2c7350caa730a3ba9dfa99d59d867daf114557a796e45244d0d73b0895e8`, 20:06 local (the Restore list showed 9-Nov-26, 8:06 pm).
- **M1** (s0b marks, after BK): 126 / 300, the answer byte-equal to M0's.

## N1: reads only, with the stored name as found
| read | HTTP | `STATUS` | rows | GUIDs THE COMPANY's | `BRIDGECOMPANYGUID` equal | `LINEERROR` | answer sha256 |
|---|---|---|---|---|---|---|---|
| N1.1 `shape-lab` (control) | 200 | 1 | 45 | 45 | 45 | 0 | `a4633cb4…` (kept local) |
| N1.2 `a1000` | 200 | 1 | 0 | 0 | 0 | 0 | `6e9e95e8…` |
| N1.3 `d120` | 200 | 1 | 0 | 0 | 0 | 0 | `6e9e95e8…` |

- After each send the capture of the Tally window shows the plain Gateway, no dialog.
- N1.1's answer is byte-equal to row N's N1.1 answer, and N1.2's and N1.3's to row N's zero-row answer (`6e9e95e8…`).

## A: the stored name is A
- **The rename (A.1):** Company Alteration by clicking the `K: Company` label, Down once to Alter, Enter: the screen opened on BRIDGE SHAPE LAB with the Name field focused. The field was cleared with End, then Backspace ×16, and showed empty. The 1,000 characters were typed (the input tool pastes them). The field scrolled and ended on `…DVabcdefgh`; it did not stop taking characters. Mailing Name and every other field were left as they were. Ctrl+A: **no prompt and no message**; the Gateway, with the company row drawn as a solid bar.
- **A.2 `C`** (`n3.py native … a`): one COMPANY row, the GUID. `NAME` attribute and `NAME` element 1: 1,000 characters, 1,000 UTF-8 bytes, sha256 `0761c84b…`, the whole name; attribute equal to element.
- **A.3** (`n3.py stored … a`): "the stored name is A whole: L-ledgers-a-stored.utf16le.xml is not needed and is not written". `SHA256-run.txt` was not made.

| read | HTTP | `STATUS` | rows | GUIDs THE COMPANY's | `BRIDGECOMPANYGUID` equal | `LINEERROR` | answer sha256 |
|---|---|---|---|---|---|---|---|
| A.4.1 `a1000` | 200 | 1 | 45 | 45 | 45 | 0 | `21bf316c…` (kept local) |
| A.4.2 stored name | not sent: A was stored whole | | | | | | |
| A.4.3 `shape-lab` | 200 | 1 | 0 | 0 | 0 | 0 | `6e9e95e8…` |

- After each send the capture of the Tally window shows the Gateway with the bar, no dialog.
- A.4.1 against N1.1: the same 87,620 bytes and 45 rows; they differ in `CMPINFO`, where A.4.1 counts COMPANY 1, CURRENCY 1 and TAXUNIT 1 and N1.1 counts all 35 as 0 (LAB-COMMON C4: those lines are not counts of anything read). A.4.1's answer is byte-equal to row N's N2.3 answer (`21bf316c…`), the read through its 300-character name.

## R1: restore and check
- BK re-hashed equal, one file in its folder; `restore-plan.txt` written; the company's data folder listed (local).
- Shut by clicking the `K: Company` label, h, Enter: the Shut Company list showed the renamed company alone, with THE COMPANY's number; no prompt. Select Company then listed the renamed company first.
- Restore under the `Y: Data` label (clicked), Backup & Restore, Restore. The Restore Data Path was Tally's data folder, as planned. The source was Specify Path, the BK folder. The list showed one row: BRIDGE SHAPE LAB, THE COMPANY's number, 9-Nov-26 (8:06 pm). It was selected alone, then End of List. The prompts, word for word:
  - "Accept ?" answered y.
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." answered r (Replace).
  - No other prompt. Tally went to Select Company.
- Select Company still listed A (a bar) first, with THE COMPANY's number, highlighted. That row was loaded by Enter, as the sheet says; no prompt. The Gateway showed BRIDGE SHAPE LAB alone, 1-Apr-26 to 31-Mar-27, current date Wednesday 20-May-2026, date of last entry 20-May-26.
- **R1.2:** marks: the GUID, 126 / 300, the answer byte-equal to M1's. Company list: `"BRIDGE SHAPE LAB"`, sha256 `54510f21…`, the answer byte-equal to step 0's. PASS; D started.

## D: the stored name is D
- **The rename (D.1):** as A.1. The field showed Devanagari letters as typed and ended on the last block; it did not stop taking characters. Mailing Name and every other field were left as they were. Ctrl+A: **no prompt and no message**; the Gateway, with the company row drawn as a bar.
- **D.2 `C`** (`n3.py native … d`): one COMPANY row, the GUID. `NAME` attribute and `NAME` element 1: 120 characters, 360 UTF-8 bytes, sha256 `80af7109…`, the whole name; attribute equal to element.
- **D.3** (`n3.py stored … d`): "the stored name is D whole: L-ledgers-d-stored.utf16le.xml is not needed and is not written".

| read | HTTP | `STATUS` | rows | GUIDs THE COMPANY's | `BRIDGECOMPANYGUID` equal | `LINEERROR` | answer sha256 |
|---|---|---|---|---|---|---|---|
| D.4.1 `d120` | 200 | 1 | 45 | 45 | 45 | 0 | `21bf316c…` (kept local) |
| D.4.2 `d85` (255 bytes) | 200 | 1 | 0 | 0 | 0 | 0 | `6e9e95e8…` |
| D.4.3 stored name | not sent: D was stored whole | | | | | | |
| D.4.4 `shape-lab` | 200 | 1 | 0 | 0 | 0 | 0 | `6e9e95e8…` |

- After each send the capture of the Tally window shows the Gateway with the bar, no dialog.
- D.4.1's answer is byte-equal to A.4.1's.

## R2: restore and check
- As R1, from the same BK (re-hashed equal), with the same two prompts word for word, answered y and r; no other prompt.
- Select Company did not list BRIDGE SHAPE LAB: `SHAPE` typed matched no row, since the list still held D for THE COMPANY's row. The filter was cleared with Backspace ×5 (End and PageDown ×4 moved nothing). THE COMPANY's number was typed: one row, D drawn as a bar, with that number. It was loaded by Enter; no prompt. The Gateway showed BRIDGE SHAPE LAB alone, 1-Apr-26 to 31-Mar-27, current date Wednesday 20-May-2026, date of last entry 20-May-26.
- **R2.2:** marks: the GUID, 126 / 300, the answer byte-equal to M1's. Company list: `"BRIDGE SHAPE LAB"`, sha256 `54510f21…`, the answer byte-equal to step 0's. PASS.

## Close (C10)
Shut Company by clicking the `K: Company` label, h, Enter: BRIDGE SHAPE LAB alone, no prompt. Select Company with no company loaded. One `tally.exe`, which alone owns 9001; `GET /status` 200. No XML request was sent after the shut. PASS.

## The sheet's result
PASS (the sheet ran): every step answered and was recorded; both restores checked equal to M1 and to the name as found; no request was sent twice (`log-send.txt`: 17 sends, each label and file once); no dialog was answered except those named.

Decision table rows that apply:
- N1 `a1000` and N1 `d120`: `STATUS` 1, 0 rows, no `LINEERROR`, no dialog. Row N's finding holds for a 1,000-character value and a multi-byte one; A's and D's reads can tell a match from no match.
- A `C`: the whole of A, 1,000 characters. Tally stores 1,000 ASCII characters whole; its own limit, if it has one, is above 1,000.
- A `a1000`, with A stored whole: B rows, all THE COMPANY's, while A `short` answers 0 rows. Tally matches a 1,000-character ASCII name exactly; Bridge's limit can rise, to at most 1,000 bytes for ASCII names.
- D `C`: the whole of D, 120 characters, 360 bytes. Tally stores a 360-byte multi-byte name whole; neither a 255-byte nor a 300-byte limit applies to names typed on its screen.
- D `d120`, with D stored whole: B rows, all THE COMPANY's, while D `d85` and D `short` answer 0 rows. Tally matches a 360-byte Devanagari name exactly; Bridge's limit for a name of these letters can rise to 360 bytes.
- A `short` and D `short` fell in no trap row, and D `d85` fell in no trap row: none answered rows.

## Deviations and notes
- **No accept prompt.** Ctrl+A on Company Alteration saved both renames with no prompt and no message; the sheet allowed for "the screen's own accept prompt", and none appeared.
- **Select Company after each restore** listed the renamed company, not the restored one. After R1 the row was first and highlighted and was loaded by Enter. After R2, `SHAPE` matched nothing, and the row was found by THE COMPANY's number and loaded by Enter. After the close, the list was in its usual order. As in row N.
- **The data folder** was listed four times (the listings stay local): before R1 (renamed to A), after R1's load and reads, before R2 (renamed to D) and after the close. Measured only:
  - before R1, 21 files, with `Company.1800` and `CmpSave.1800` at 12,800 B each;
  - after R1, 23 files: `TUPDATE.TSF` absent, `TCMPSCHMESSAGE.TSF`, `TINTMSG.TSF` and `TMESSAGE.TSF` present, `Company.1800` and `CmpSave.1800` at 8,704 B each;
  - before R2, 24 files: `TUPDATE.TSF` present again and `TMESSAGE.TSF` at 5,632 B; `Company.1800` and `CmpSave.1800` still 8,704 B;
  - after the close, 20 files: `TCMPSCHMESSAGE.TSF`, `TINTMSG.TSF`, `TMESSAGE.TSF` and `TUPDATE.TSF` absent; no new file; every other file at its size before R2.
- **Labels.** Each label was used once per request file. `s0b` is the marks read after BK that the sheet asks for.

## Not measured
A name between 1,000 characters and any limit above it; a name over 1,000 characters; letters other than these Devanagari consonants (vowel signs, other combining marks, other scripts, letters outside the Basic Multilingual Plane); whether Tally stores D in UTF-8, UTF-16 or another form; a name with characters that need XML escaping; a name set by an import rather than typed on Tally's screen; a voucher read, or a write, through a long name; two companies loaded at once; any Bridge build; another Tally release.

## Files
- `answers/`: every answer and headers file, byte for byte, except any that the run lead's standing rules keep on the lab PC (listed by size and sha256 in `kept-local.txt`). The three 45-row ledger answers (N1.1, A.4.1 and D.4.1) stay on the lab PC, as row N's did: one ledger's amount holds, as a substring, a number that may not go in a public file. Their counts are in `evidence/`. A company-list answer goes out with its `COMPANYNUMBER` value replaced, as the sheet says; each change is in `substitutions.txt`.
- `logs/log-send.txt`: `send.py`'s log, whole.
- `evidence/`: `notes.txt` (UTC, one line per step), each `n3.py`, `measure.py` and `t2.py` output whole, the step 0 and C10 check lines and the BK record. The process and port listings, the folder listings and the restore plan stay local.
- `scripts/`: the step 0 helper and the window capture helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies.
