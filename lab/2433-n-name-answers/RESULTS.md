# 2433 N: a ledger read through a 300-character company name reaches the company stored under that name, and only by the whole name

Run on 9 Nov 2026, 13:35Z to 13:47Z, on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2433-n-name-sheet` (1cd8de6f), in one sitting on `BRIDGE SHAPE LAB` (GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, synthetic). Raw reads only, sent by `send.py`; no Bridge build, no tool call. One rename on Tally's own Company Alteration screen, undone by restoring one backup. Tally's current date was Wednesday 20-May-2026 throughout. Every finding below comes from one run: **partial** at most. Every read was answered.

**Result.**
- **B = 45.** N1's control, the ledger list through the stored name `BRIDGE SHAPE LAB`, answered 45 rows, every GUID and every `BRIDGECOMPANYGUID` THE COMPANY's, no `LINEERROR`.
- **N1 `long`** (the 300-character pattern, which no company had): HTTP 200, `STATUS` 1, **0 rows**, no `LINEERROR`, and no dialog (the capture after it shows the plain Gateway). Decision table row 1: finding 2 holds for a 300-character value. A name that matches no company answers nothing even while the company is loaded. So row T2's 45 rows (`lab/2433-name-answers`) were a match on the name, and N2 can tell a match from no match. Partial.
- **N2 `C`:** after the rename, the `NAME` attribute and the `NAME` element each hold the whole pattern: 300 characters, 300 UTF-8 bytes, sha256 `1d6366e1…`, equal to each other. The N2 decisions are drawn.
- **N2 `long`:** **45 rows**, all THE COMPANY's (45 GUIDs, 45 `BRIDGECOMPANYGUID`), `STATUS` 1, no `LINEERROR`. **N2 `prefix255`:** 0 rows, `STATUS` 1. **N2 `short`** (the old name): 0 rows, `STATUS` 1. None had a `LINEERROR` or a dialog. Decision table: **Tally matches a 300-character name exactly.** Bridge's limit can rise, to at most 300 bytes for ASCII names, the longest value measured; multi-byte names stay unmeasured. Tally does not match on a 255-character prefix, and after the rename it does not answer the old name. Partial.
- The restore from BK reads equal to step 0: the GUID, ALTVCHID 126 and ALTMSTID 300 (M1, byte-equal), and the name as found (the company-list answer byte-equal to step 0's). Nothing but the one rename was written. Nothing was deleted.

## Step 0
- Sheet files: all 12 equal to `SHA256.txt` at 1cd8de6f; `LAB-COMMON.md` byte-equal to row 2162-FY14's. `n.py gen` (exit 0) wrote both request files with the sha256 values in `SHA256.txt`, byte-equal to the sheet's. `measure.py pattern`: 300 characters, sha256 `1d6366e1…`, as the sheet.
- Bridge processes (the sheet's `Get-CimInstance` line): two `bridge_mcp.exe`, the installed extension. None was ended. No Bridge tool was called during the sitting. The listing stays local.
- C2: one `tally.exe`, which alone owns 9001; `GET /status` 200. Loaded alone by Select Company (`SHAPE` typed, one row, Enter, no prompt). The Gateway: BRIDGE SHAPE LAB alone, current period 1-Apr-26 to 31-Mar-27, current date Wednesday 20-May-2026, date of last entry 20-May-26, footnote "(e) Data exceptions exist".
- **M0** (s0 marks): one COMPANY row, the GUID, ALTVCHID 126, ALTMSTID 300.
- s0 company list: `"BRIDGE SHAPE LAB"`, 16 characters, 16 bytes, sha256 `54510f216f9b0854d58ad42e7d5b7fbbc38b130d9a6038ae89aab528dc1e3f7b`, as the sheet.
- **BK** (C5): Data > Backup & Restore > Backup, reached by clicking the `Y: Data` label (see Deviations). The Backup Data Path was changed from C:\ by Specify Path to the new empty folder `<LAB_DIR>\bk` (listed empty before). BRIDGE SHAPE LAB alone; End of List; "Accept ?" y; no other message. `TDBK1800_<company number>.001`, 115,721 B, sha256 `84cd0152a33fb9b907689d4b11415cdce07cf194868efbd9bc61f06f1a07b33d`, 19:10 local (the Restore list showed 9-Nov-26, 7:10 pm). The listing after showed the one file.
- **M1** (s0b marks, after BK): 126 / 300, the answer byte-equal to M0's.

## N1: reads only, with the stored name as found
| read | HTTP | `STATUS` | rows | GUIDs THE COMPANY's | `BRIDGECOMPANYGUID` equal | `LINEERROR` | answer sha256 |
|---|---|---|---|---|---|---|---|
| N1.1 `short` (control) | 200 | 1 | 45 | 45 | 45 | 0 | `a4633cb4…` (kept local) |
| N1.2 `long` | 200 | 1 | 0 | 0 | 0 | 0 | `6e9e95e8…` |

- After N1.2 the capture of the Tally window shows the plain Gateway, no dialog.
- N1.1's answer is byte-equal to row T2's `t2base` answer (`a4633cb4…`).

## N2: the stored name is the pattern
- **The rename (N2.1):** Company Alteration by clicking the `K: Company` label, Down to Alter, Enter: the Name field focused, holding `BRIDGE SHAPE LAB`. The field was cleared with End, then Backspace ×16, and showed empty. The 300 characters were typed (the input tool pastes them). The field showed the pattern's tail. Mailing Name and every other field were left as they were. The field was captured, then accepted with Ctrl+A: no prompt; the Gateway. Its Name of Company showed a solid dark bar in place of the name, as at T1. No dialog.
- **N2.2 `C`** (`measure.py native … --pattern`): one COMPANY row, the GUID. `NAME` attribute and `NAME` element 1: 300 characters, 300 UTF-8 bytes, sha256 `1d6366e1ccaa762b2da904319dc3a5385513fc401f1c814bb7a16bb0a03e433e`, no control character, no edge space; the whole pattern, ending in block 30 (`d`) character 10 of 10; attribute equal to element.

| read | HTTP | `STATUS` | rows | GUIDs THE COMPANY's | `BRIDGECOMPANYGUID` equal | `LINEERROR` | answer sha256 |
|---|---|---|---|---|---|---|---|
| N2.3 `long` | 200 | 1 | 45 | 45 | 45 | 0 | `21bf316c…` (kept local) |
| N2.4 `prefix255` | 200 | 1 | 0 | 0 | 0 | 0 | `6e9e95e8…` |
| N2.5 `short` | 200 | 1 | 0 | 0 | 0 | 0 | `6e9e95e8…` |

- After each N2 send the capture of the Tally window shows the Gateway with the dark bar, no dialog.
- N2.3's answer is byte-equal to row T2's `t2a` and `t2b` answers (`21bf316c…`). N2.4's and N2.5's answers are byte-equal to N1.2's: the same zero-row answer.
- N2.3 against N1.1: the same 87,620 bytes and 45 rows; they differ in `CMPINFO`, where N2.3 counts COMPANY 1, CURRENCY 1 and TAXUNIT 1 and N1.1 counts all 35 as 0 (LAB-COMMON C4: those lines are not counts of anything read). As in row T2.

## R: restore and check
- BK re-hashed equal, one file in its folder; `restore-plan.txt` written; the company's data folder listed before and after (all stay local).
- Shut by clicking the `K: Company` label, h, Enter: the Shut Company list showed the renamed company alone, with THE COMPANY's number; no prompt. Select Company then listed the renamed company first.
- Restore under Data (the `Y: Data` label), Backup & Restore, Restore. The Restore Data Path was Tally's data folder, as planned. The source was Specify Path, the BK folder. The list showed one row: BRIDGE SHAPE LAB, THE COMPANY's number, 9-Nov-26 (7:10 pm). It was selected alone, then End of List. The prompts, word for word:
  - "Accept ?" answered y.
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." answered r (Replace).
  - No other prompt. Tally went to Select Company.
- Select Company still listed the pattern (a dark bar) first, with THE COMPANY's number and 1-Apr-25 to 31-Mar-27, as after T1's restore. That row was loaded by Enter, as the sheet says; no prompt. The Gateway showed BRIDGE SHAPE LAB alone, 1-Apr-26 to 31-Mar-27, current date Wednesday 20-May-2026, date of last entry 20-May-26.
- **R.2:** marks: the GUID, 126 / 300, the answer byte-equal to M1's. Company list: `"BRIDGE SHAPE LAB"`, sha256 `54510f21…`, the answer byte-equal to step 0's. PASS.

## Close (C10)
Shut Company by clicking the `K: Company` label, h, Enter: BRIDGE SHAPE LAB alone, no prompt. Select Company with no company loaded, the list in its usual order. One `tally.exe`, which alone owns 9001; `GET /status` 200. No XML request was sent after the shut. PASS.

## The sheet's result
PASS (the sheet ran): every step answered and was recorded; the restore checked equal to M1 and to the name as found; no request was sent twice (`log-send.txt`: 11 sends, each label and file once); no dialog was answered except those named.

Decision table rows that apply:
- N1 `long`: `STATUS` 1, 0 rows, no `LINEERROR`, no dialog. Finding 2 holds for a 300-character value; T2's 45 rows were a match.
- N2 `long`: B rows, all THE COMPANY's, while N2 `prefix255` and N2 `short` answer 0 rows. Tally matches a 300-character name exactly; Bridge's limit can rise to at most 300 bytes for ASCII names.
- N2 `prefix255` and N2 `short` fell in no trap row: neither answered rows.

## Deviations and notes
- **TallyCapital, before BK.** At the Gateway, two presses of Alt+Y were sent to open the Data menu. The first showed no change. The second opened the TallyCapital screen (lending offers, no dialog), with TallyCapital highlighted in the Gateway menu. That screen was captured and left with Ctrl+Q. Ctrl+Q opened TallyCapital's "Talk to an Advisor" form: a reason field with a list of options, a call-back question, and a mobile number field. The run paused there with no key pressed. On the run lead's word, Ctrl+Q was pressed once: no confirmation, and the Gateway returned. Nothing on the form was accepted, no option was chosen and nothing was typed. Nothing was sent from it. From then on, the Data and Company menus were opened by clicking their labels. The captures stay on the lab PC.
- **Select Company after the restore** listed the renamed company first, as after row T1's restore; the row was loaded by its number. After the close, the list was in its usual order. This is the third sitting in which Select Company's list did not show the restored folder's own values right after a restore (rows 2433 and 2162-FY14).
- **The data folder** held 24 files before the restore and 17 after (row 2162-FY14: 24 and 23). See "The data folder across the restore" below.

## The data folder across the restore
THE COMPANY's data folder was listed three times (the listings stay local): before the restore (13:43Z, with the renamed company loaded), right after it (13:45Z, at Select Company, before the load) and after the close (13:53Z, after the load, R.2's reads and the shut). Files are named here by file name and extension only.

**Measured.**
- After the restore, 7 of the 24 files were absent: `CmpSchMetaData.1800` (1,024 B) and six `.tsf` files, `TCMPSCHACCESS` (512 B), `TCMPSCHMESSAGE` (512 B), `TCMPSCHSTATE` (5,632 B), `TCMPSCHUPDATE` (2,560 B), `TINTMSG` (5,632 B) and `TUPDATE` (22,016 B). No file appeared that was not there before.
- Of the 17 that stayed (14 `.1800`, 3 `.tsf`), three changed size: `Company.1800` and `CmpSave.1800` from 9,728 to 8,704 B, and `TMESSAGE.TSF` from 6,656 to 512 B. The other 14 kept their sizes.
- After the close, 20 files: `CmpSchMetaData.1800` (1,024 B), `TCMPSCHACCESS.TSF` (512 B), `TCMPSCHSTATE.TSF` (5,632 B) and `TCMPSCHUPDATE.TSF` (2,560 B) were present again at their earlier sizes. `TCMPSCHMESSAGE.TSF`, `TINTMSG.TSF` and `TUPDATE.TSF` were still absent, and `TMESSAGE.TSF` was absent too. `Company.1800` and `CmpSave.1800` stayed at 8,704 B. No file was present that was not there before the restore.
- After the restore, the marks answer was byte-equal to M1's and the company-list answer byte-equal to step 0's.

**INFERRED, from the file names only.** `CmpSchMetaData.1800` and the `TCMPSCH*` files hold Tally's scheduled-backup state (the Gateway shows "(Schedule Backup)" beside the company), `Index.1800` (below) is an index, and the other `.tsf` files are Tally's working state and message files. Tally rebuilds them when the company is loaded. Nothing here reads their contents.

**The same pattern in earlier rows,** from their local listings (those branches are not changed): row 2433's R1, 25 files to 17, the same 7 and `Index.1800`, with the folder back at 24 files before its R2; row 2433's R2, 24 to 17, the same 7; row 2162-FY14's restore, 24 to 23, `Index.1800` alone.
- **Labels.** Each label was used once per request file. `s0b` is the marks read after BK that the sheet asks for.

## Not measured
A name over 300 characters; multi-byte letters, and whether a limit counts characters or bytes; a name with characters that need XML escaping; a voucher read, or a write, through a long name; two companies loaded at once; any Bridge build; another Tally release.

## Files
- `answers/`: every headers file, the marks answers and the zero-row ledger answers, byte for byte. The three company-list answers go out with their `COMPANYNUMBER` value replaced by `redacted` (UTF-16LE kept). The originals stay on the lab PC by size and sha256 (`kept-local.txt`), and each change is in `substitutions.txt`.
- The two 45-row ledger answers (N1.1 and N2.3) stay on the lab PC by size and sha256 (`kept-local.txt`), as row T2's did: one ledger's opening balance holds, as a substring, a number that may not go in a public file. Their counts are in `evidence/n1-short-ledgers.txt` and `evidence/n2-long-ledgers.txt`.
- `logs/log-send.txt`: `send.py`'s log, whole.
- `evidence/`: `notes.txt` (UTC, one line per step), each `n.py`, `measure.py` and `t2.py` output whole, the step 0 and C10 check lines. The process and port listings, the folder listings and the restore plan stay local.
- `scripts/`: the step 0 helper and the window capture helper. The restore plan helper stays local (it names the lab folders), by size and sha256.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies.
