# 2433 name: Tally stores a 300-character company name whole, and trims edge spaces typed on its own screen

Run on 9 Nov 2026, 12:52Z to 13:08Z, on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2433-name-sheet` (27b0241e), in one sitting on `BRIDGE SHAPE LAB` (GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, synthetic). Raw reads only, sent by `send.py`; no Bridge build, no tool call. Tally's current date was Wednesday 20-May-2026 throughout. Every finding below comes from one run: **partial** at most.

**Result.**
- **T1 (#2397):** Tally stored the 300-character letters-only pattern whole: the `NAME` attribute and the `NAME` element each hold 300 characters, 300 UTF-8 bytes, sha256 `1d6366e1…` (the pattern's), ending in block 30 (`d`) character 10 of 10. The stored name is **over 255 bytes**. So a person can type, on Tally's own screen, a company name that Bridge's company-list parser and #2397's identity check refuse. Partial (one run, one release).
- **T2 (#2433 (b)), the stored name:** typed as two spaces, `BRIDGE SHAPE LAB`, two spaces, it was stored as `"BRIDGE SHAPE LAB"`, 16 characters, sha256 `54510f21…`: the name as found, **trimmed**. The field showed no edge space before the accept. #2433 (b) cannot arise from typing on Tally's screen on this release. Partial.
- **T2, the reads:** with the stored name trimmed, the trimmed-name ledger read is an exact match: 45 rows = B, every GUID and every `BRIDGECOMPANYGUID` THE COMPANY's. The spaced-name ledger read (two spaces on each side in `SVCURRENTCOMPANY`) answered byte-for-byte the same as the trimmed read: 45 rows, all THE COMPANY's. This is finding 3's direction (spaces in the request, none in the stored name), with two leading and two trailing spaces, on licensed Silver. With one company loaded, this row does not separate a match on the name from Tally answering for the loaded company. Partial.
- Both restores read equal to step 0: the GUID, ALTVCHID 126 and ALTMSTID 300 (M1), and the name as found. Nothing but the two renames was written. Nothing was deleted.

## Step 0
- Sheet files: all 10 equal to `SHA256.txt` at 27b0241e. `t2.py gen`, run in a separate folder, wrote `L-ledgers-spaced.utf16le.xml` byte-equal to the committed file (`515693fa…`). `measure.py pattern`: 300 characters, sha256 `1d6366e1…`, as the sheet.
- Bridge processes (the sheet's `Get-CimInstance` line): two `bridge_mcp.exe`, the installed extension. None was ended. No Bridge tool was called during the sitting. The listing stays local.
- C2: one `tally.exe`, which alone owns 9001; `GET /status` 200. Loaded alone by Select Company (`SHAPE` typed, one row, Enter, no prompt). The Gateway: BRIDGE SHAPE LAB alone, current period 1-Apr-26 to 31-Mar-27, current date Wednesday 20-May-2026, date of last entry 20-May-26, footnote "(e) Data exceptions exist".
- **M0** (s0 marks): one COMPANY row, the GUID, ALTVCHID 126, ALTMSTID 300.
- s0 company list: `"BRIDGE SHAPE LAB"`, 16 characters, 16 bytes, sha256 `54510f216f9b0854d58ad42e7d5b7fbbc38b130d9a6038ae89aab528dc1e3f7b`, as the sheet.
- **BK** (C5): Data > Backup & Restore > Backup. The Backup Data Path was changed to Specify Path and the new empty folder `<LAB_DIR>\bk` (listed empty before). BRIDGE SHAPE LAB alone; "Accept ?" y; no other message. `TDBK1800_<company number>.001`, 115,721 B, sha256 `9f62b45fef50d3f6abe5677becea646345fc5452f148d6e4092d3cb29efbd4fe`, 18:24 local (the Restore list showed 9-Nov-26, 6:24 pm). The listing after showed the one file.
- **M1** (s0b marks, after BK): 126 / 300, equal to M0.

## T1: the 300-character name
- **The rename (T1.1):** Company Alteration by Alt+K (the Company menu), Down to Alter, Enter. The Name field was cleared with End, then Backspace ×16 (Backspace at the field's start removed nothing). The pattern was typed. The field scrolled and showed the pattern's tail with the cursor after it; it gave no sign it stopped taking characters. Mailing Name and every other field were left as they were. Accepted with Ctrl+A: no prompt; the Gateway. Its Name of Company showed a solid dark bar in place of the name. No dialog.
- **The stored name (T1.2):** t1 company list, HTTP 200, 5,136 B. `measure.py native --pattern`:

| | characters | UTF-8 bytes | sha256 | prefix of the pattern | ends in | control characters | edge space |
|---|---|---|---|---|---|---|---|
| `NAME` attribute | 300 | 300 | `1d6366e1…` | 300 of 300 | block 30 (`d`), character 10 of 10 | 0 | no |
| `NAME` element 1 | 300 | 300 | `1d6366e1…` | 300 of 300 | block 30 (`d`), character 10 of 10 | 0 | no |

  The attribute equals the element. The stored length is over 255 bytes. No cut point was reached: whether Tally has a limit above 300 characters is not measured.
- **R1 (T1.3, C6):** BK re-hashed equal, one file in its folder; `restore-plan-r1.txt` written; the data folder listed before and after (both stay local). Shut by Alt+K, h, Enter: the Shut Company list showed the renamed company alone, with THE COMPANY's number; no prompt. Restore under Data, Backup & Restore, Restore. The Restore Data Path was Tally's data folder, as planned. The source was Specify Path, the BK folder. The list showed one row: BRIDGE SHAPE LAB, THE COMPANY's number, 9-Nov-26 (6:24 pm). It was selected alone, then End of List. The prompts, word for word:
  - "Accept ?" answered y.
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." answered r (Replace).
  - No other prompt. Tally went to Select Company.
- **Select Company after R1** still listed THE COMPANY's number under the T1 name (a row of dense text). `SHAPE` typed there matched no row. It was cleared with Backspace ×5, no other key. Every file in the company's data folder had been rewritten at the restore (all one time; `Company.1800` 9,728 B before, 8,704 B after). The row with THE COMPANY's number was loaded with Enter, no prompt. The Gateway showed BRIDGE SHAPE LAB alone, as at step 0. The list showing the name from before the restore fits a list that Tally does not re-read at the restore. After R2, Select Company listed BRIDGE SHAPE LAB under `SHAPE`, but the name before and after R2 was the same, so R2 does not test this.
- **R1 check (T1.4):** r1 marks: the GUID, 126 / 300 = M1. r1 company list: `"BRIDGE SHAPE LAB"`, 16 characters, `54510f21…`. Both answers are byte-equal to s0's. PASS.

## T2: edge spaces
- **The baseline (T2.1):** `t2base`, HTTP 200, 87,620 B. `STATUS` 1, one collection, **B = 45** `LEDGER` rows (the tree's capture of 2026-10-01 held 44). All 45 GUIDs begin with THE COMPANY's, all 45 `BRIDGECOMPANYGUID` equal to it (one distinct value), no `LINEERROR`, all 35 `CMPINFO` counts 0.
- **The rename (T2.2):** Company Alteration as T1.1; the Name field cleared (End, Backspace ×16); typed two spaces, `BRIDGE SHAPE LAB`, two spaces. Before the accept the field showed `BRIDGE SHAPE LAB` starting at the same pixel as before the change, with the cursor right after `LAB`. So no edge space showed. Ctrl+A: no prompt; the Gateway, Name of Company BRIDGE SHAPE LAB. No dialog.
- **The stored name (T2.3):** t2 company list, HTTP 200, 4,000 B. `t2.py name`: the `NAME` attribute and element are each `"BRIDGE SHAPE LAB"`, 16 characters, 16 bytes, first 4 bytes `42 52 49 44`, last 4 `20 4c 41 42`, sha256 `54510f21…`. This is **the name as found**, not the spaced name (`44e67913…`, 20 characters). Tally trimmed the name on entry. The input tool sends typed text as one paste. Whether the field dropped the spaces as they arrived or at the accept is not separated by this row. Against r1's company list the answer differs only in three `CMPINFO` lines (`COMPANY`, `CURRENCY`, `TAXUNIT`, 0 to 1); the COMPANY row is byte-equal.
- **The trimmed-name ledger read (T2.4):** `t2a`, HTTP 200, 87,620 B. `STATUS` 1, 45 rows = B, 45 GUIDs THE COMPANY's, 45 `BRIDGECOMPANYGUID` equal to it, no `LINEERROR`. `CMPINFO`: `COMPANY` 1, `CURRENCY` 1, `TAXUNIT` 1, the other 32 counts 0. Against `t2base` it differs only in those three lines; the rows are byte-equal. **Matched.**
- **The spaced-name ledger read (T2.5):** `t2b`, HTTP 200, 87,620 B, **byte-equal to `t2a`'s answer**: both sha256 `21bf316cba4aba0de0bd011a897ca851e267e693118f98d38eefdb0eda5a232b` (`t2base`: `a4633cb434cb95bea27c20d7160bc62210615de5ed227fe40eb0950df65bbdb3`). `STATUS` 1, 45 rows = B, all THE COMPANY's, no `LINEERROR`. **Matched.**
- **R2 (T2.6, C6):** as R1, with `restore-plan-r2.txt`. The list showed the same one row. The same two prompts got the same two keys (y, then r). No other prompt.
- **R2 check (T2.7):** r2 marks: the GUID, 126 / 300 = M1. r2 company list: `"BRIDGE SHAPE LAB"`, `54510f21…`. Both answers are byte-equal to s0's. PASS.

## Close (C10)
Shut Company by Alt+K, h (Company > Shut), BRIDGE SHAPE LAB alone, Enter, no prompt. Select Company with no company loaded. One `tally.exe`, which alone owns 9001; `GET /status` 200. No XML request was sent after the shut. PASS.

## The sheet's result
**PASS (the sheet ran).** Every step answered and was recorded. Both restores checked equal to M1 and to the name as found. No request was sent twice (`log-send.txt`: 12 sends, each label once). The only prompts answered were the two each restore names and the Backup screen's "Accept ?"; both renames were accepted with no prompt. Every read was answered with HTTP 200 and its `CONTENT-LENGTH`.

## Deviations and notes
- **Select Company after R1** showed the pre-restore name (above). The company was loaded by its row and number, not by typing `SHAPE`. The Gateway and both checks then showed the restored name.
- **No marks read in a renamed state**, as the sheet says. Whether either rename moved ALTMSTID is therefore not read. Both restores bring the marks back to M1.
- **B is 45**, one more than the tree's capture of 2026-10-01 (44). Which ledger was added since is not read here.
- **The Gateway's period** is 1-Apr-26 to 31-Mar-27, while Select Company lists 1-Apr-25 to 31-Mar-27 for this book; both as at step 0.

## Not measured
A name over 300 characters, so Tally's own limit for ASCII letters, if any. Multi-byte letters, and whether a limit counts characters or bytes. One edge space, a tab, or spaces on one side only. A name with edge spaces set by import (only Tally's screen was used). Two companies loaded at once. Any Bridge build. Another Tally release.

## Files
- `answers/`: every headers file and the marks answers, byte for byte. The five company-list answers go out with their `COMPANYNUMBER` value replaced by `redacted` (UTF-16LE kept). The originals stay on the lab PC by size and sha256 (`kept-local.txt`), and each change is in `substitutions.txt`.
- The three ledger answers (`t2base`, `t2a`, `t2b`) stay on the lab PC by size and sha256 (`kept-local.txt`). One ledger's opening balance holds, as a substring, a number that may not go in a public file. Their counts are in `evidence/t2ledgers-*.txt`, and `t2a` = `t2b` is shown by their sha256 above: 45 rows each.
- `logs/log-send.txt`: `send.py`'s log, whole.
- `evidence/`: `notes.txt` (UTC, one line per step), each `measure.py` and `t2.py` output whole, the step 0 and C10 check lines. The process and port listings, the folder listings and the restore plans stay local.
- `scripts/`: the step 0 helper and the window capture helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies.
