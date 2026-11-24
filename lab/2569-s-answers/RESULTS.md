# Lab 2569-S results

Scope: one synthetic book (BRIDGE EINVOICE LAB, GUID eb2f5190-292e-4ed8-bfd3-eace839fe699), TallyPrime 7.2 Silver, one Windows account, one sitting on 2026-11-24. Each launch has one box, and no box was answered. The sheet is `lab/2569-s-sheet` at cd80f722327ca5e3df2729e85f7e3fc0ec6fea2d.

## Q1, Q2, Q3

**Q1. On the launch path of 19 and 20 Nov, does the old build's box open minimised again?**

No. On that path the old build's box opened normal and in the foreground. It was minimised across one screenshot by the screen tool. The tool's own note for that screenshot listed `bridge_mcp.exe` among the processes it hid.

On row 2569-R's granted path, the same exe bytes gave a box that stayed open through the screenshot, and the tool gave no such note.

Reading: the boxes of 19 and 20 Nov are explained by the screen tool hiding the window of a process outside its grant, not by how the build launches its box.

**Confidence: partial.**
- One sitting and one launch per path.
- Between L1 and L0 the exe path and its grant state changed together; the bytes were the same (OLD sha256 c437138a08dc10a8609e42d28104920401b4615daa69f3cdf404f6e37d6323dd). This row cannot tell the path from the grant.
- The tool's note names the grant list ("not in the session allowlist", "aren't in the installed-apps list"), which points to the grant.
- This is consistent with the #2161 record's finding 5.

**Q2. What show state and start-up flags did the server and its `--confirm-journal` helper receive, on that path and on row 2569-R's?**

The same on both paths, and **verified** for this sitting: the self-test passed with 12 parse and 4 live cases, wrong 0.

| process | window_flags | show_window_flags | pe_subsystem |
|---|---|---|---|
| helper (parent: the server) | 0x100 (USESTDHANDLES) | 0 (SW_HIDE), USESHOWWINDOW clear | 3 (WINDOWS_CUI) |
| server | 0x100 (USESTDHANDLES) | 0 (SW_HIDE), USESHOWWINDOW clear | 3 (WINDOWS_CUI) |
| server's python.exe parent | 0x100 (USESTDHANDLES) | 0, USESHOWWINDOW clear | 3 (WINDOWS_CUI) |

- The helper was given no show state, so the minimised box did not come from its STARTUPINFO.
- The server's console line in both launches was `CONSOLE pid=<server> attached window=none`.
- The path change did not change what the processes received.
- Limit: creation flags themselves are not read, and the bash ancestor's read failed in both launches (see L1 and L0).

**Q3. If a box of the old build is minimised, is the same launch's box restored under the build holding the fix, and is the restore line in the helper's error output?**

**Not answered live: L2 was not run.**
- L1's box was minimised (`M1`), but L2 PATH was `no`: no NEW-hash path off the grant list was allowed, and no exe was copied to make one.
- L0's box was not minimised (`N`), so there was no box to pair with at `exe-new`.

From the code only (unverified live):
- The fix's `IsIconic` check runs when Windows activates the box (`box_fit.rs:557-570` at 8c3fb9f2). A box minimised after its activation, as L1's was across the screenshot, would stay minimised.
- The restore line goes to the null device on this path in both builds (`approved_import.rs:2800`), so it is not readable here.

## Table

| launch | exe path (grant list) | word | R1 | R1b | R2 | R3 | helper window_flags | helper show_window_flags | helper console_handle | helper pe_subsystem | server window_flags | console line | final error_code | marks before | marks after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| L1 | `<lab root>/2440-cp/build/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe` (not on it) | M1 | iconic=False | iconic=False | iconic=True | iconic=True | 0x100 (USESTDHANDLES) | 0 (SW_HIDE), USESHOWWINDOW clear | 0x48 | 3 | 0x100 (USESTDHANDLES) | attached window=none | import_approval_timed_out | V 3, M 220 | V 3, M 220 |
| L0 | `<lab root>/2569-r/exe-old/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe` (on it) | N | iconic=False | iconic=False | iconic=False | iconic=False | 0x100 (USESTDHANDLES) | 0 (SW_HIDE), USESHOWWINDOW clear | 0x54 | 3 | 0x100 (USESTDHANDLES) | attached window=none | import_approval_timed_out | V 3, M 220 | V 3, M 220 |
| L2 | not run: L1 alone was minimised, L2 PATH `no`; L0 read `N` | | | | | | | | | | | | | | |

## The two boxes, read by read (UTC)

### L1

- helper pid 11748, created 15:10:39.558Z
- R1, read 15:10:40.849Z, lag_s 1.3: `WINDOW ... name=[ComplyEaze Bridge — post this voucher?] class=#32770 iconic=False visible=True topmost=True rect=(688,162,1252,945) visual_state=Normal foreground=True`; FOREGROUND the box; FOCUS `[No]` Pane of the helper
- R1b, read 15:10:42.851Z, lag_s 3.3: the same line, iconic=False, Normal, foreground=True
- S1, from 15:10:48.251Z to 15:10:59.636Z: exactly one whole-screen screenshot by the screen tool. Its result note, word for word:

  > "Windows PowerShell", "Brave" were open and got hidden before this screenshot (not in the session allowlist). If a previous action was meant to open one of them, that's why you don't see it — call request_access to add them. "bridge_mcp.exe", "systemsettings.exe" were also hidden. These processes own the visible windows but aren't in the installed-apps list — likely a worker process spawned by a launcher you already granted (e.g. LibreOffice's simpress.exe launches soffice.bin, which owns the actual window). Pass the exact basenames above to request_access.

  The image showed the Gateway and a minimised title bar "ComplyEaze B..." at the bottom left.
- R2, read 15:11:00.308Z: `WINDOW ... class=#32770 iconic=True visible=True topmost=True rect=(0,996,199,1030) visual_state=Minimized foreground=False`; FOREGROUND Tally; FOCUS `[TallyPrime:9001]`
- R3, read 15:11:16.096Z: the same line, iconic=True, Minimized
- final post answer, at 15:12:54Z: `import_approval_timed_out`
- R4, read 15:13:03Z: windows 0; the helper GONE

### L0

- helper pid 18620, created 15:17:50.067Z
- R1, read 15:17:51.022Z, lag_s 1.0: `WINDOW ... name=[ComplyEaze Bridge — post this voucher?] class=#32770 iconic=False visible=True topmost=True rect=(688,162,1252,945) visual_state=Normal foreground=True`; FOREGROUND the box; FOCUS `[No]` Pane of the helper
- R1b, read 15:17:52.802Z, lag_s 2.7: the same line
- S1, from 15:17:58.042Z to 15:18:09.298Z: exactly one whole-screen screenshot by the screen tool. Its result carried no note (no process listed as hidden). The image showed the box open over the Gateway.
- R2, read 15:18:09.987Z: the same line as R1, iconic=False, Normal, foreground=True; FOREGROUND the box; FOCUS `[No]`
- R3, read 15:18:21.042Z: the same line, iconic=False, Normal
- final post answer, at 15:20:01Z: `import_approval_timed_out`
- R4, read 15:20:12.242Z: windows 0; the helper GONE

The helper's other windows, in both launches, read by Q2h (whose reader is not DPI-aware, so its box rect reads (550,130,1001,756)):
- the box `#32770`: visible, not iconic, topmost; its owner is a hidden Static toolwindow;
- that Static window;
- `MSCTFIME UI`;
- `Default IME`.

## Setup: step 0 to step 5

**Step 0.** Every EXPECT was met.
- `parse` self-test: 12 cases. `startinfo.ps1` self-test: 4 live cases, wrong 0.
- Tally's own start-up block read `RESULT OK`, USESHOWWINDOW clear.
- The grant list held TallyPrime, Bridge, `exe-new` and `exe-old`, and not the 2440-CP folder.
- The exe inventory listed the L0 path and the L1 path, each `EXE OLD`.
- The step 0 reply was taken by the sheet's five commands (the head became cd80f722): L1 word the 2440-CP path; L2 PATH `no`.

**Step 1.** The book was loaded alone. The Gateway showed 1-Apr-25 to 31-Mar-26, current date Thursday, 3-Apr-2025.
- s1: one COMPANY row, the book's GUID, ALTVCHID 3, ALTMSTID 215.
- `ISGSTCLASSIFON` No; one `CMPGSTDETAILS.LIST` from 20250401.
- K-sales `BRIDGE IRN Sales`, IGST 18, HSN 998313.
- K-customer, K-cgst, K-sgst: 0 LEDGER rows each.

**Step 2.** BK1 was taken by C5: one file, 32,101 B, sha256=15b02086f6ae1fc4f8870c136350cf29212404f72eca43e6b7f49a698b7dbfc3. s1b equal to s1.

**Step 3.** The seed came back CREATED, with every other counter 0 and no LINEERROR. s3r: `BRIDGE CP Customer MH` (Sundry Debtors, Regular, Maharashtra), ALTMSTID 216.

**Step 4.** `BRIDGE CP CGST` and `BRIDGE CP SGST` were keyed: Duties & Taxes, GST, CGST and SGST/UTGST, 9, Not Applicable. ALTMSTID 218.

**Step 5.** `BRIDGE CP Sales` was keyed: Sales, Manual, duplicate numbers prevented. ALTMSTID 220, ALTVCHID 3.

## L1: OLD from row 2440-CP's folder (step 6)

**Frame a.** Three folders ABSENT, then five MADE. The launch file placed and matched (d94ecc7fd8f6fe3053aa129bb1afff78bb910f97b94db2ebf5168064faa2af8f). The exe hash matched OLD, 35,869,184 B.

**Frame b.**
- The Gateway with the one company and no dialog.
- C2: one tally.exe on 9001; /status status=200, bytes=51.
- pre-L1 marks equal to s5r.
- Processes: `count bridge_mcp.exe 0`.

**Frame c.** The client started as a background command of the operator's shell tool (Git Bash, no window, standard input from the null device). Server pid 17448 at 15:09:11Z; initialized bridge-tally 0.7.0.

**Frame d.**
- 01: 37 tools, the five present.
- 02: one company, `guid_is_book True`.
- 03: `master_alter_id 220 voucher_alter_id 3`.

**Frame e (chain read).**
- Server and parent: as in Q2. The server's console_handle was 0x54, its std handles 0x1D4/0x1DC/0x1D8.
- The bash ancestor (level 2): `LAYOUT_CHECK_FAILED` at image_path (a `bin\..\usr\bin` path alias). Its Q2 line: failed.

**Frame f.** 04-build-a: `voucher_count 1`, batch bridge-dfaecd58-f72b-4f22-9655-572ea6e0ca0d, `hsn_warnings 0`.

**Frame g.**
- The watch started; post 05 was committed at 15:10:22Z.
- Reads as above.
- Post answers:
  - 05: pending, dialog_remaining_s 96, not_dispatched;
  - 06: pending, dialog_remaining_s 9, not_dispatched;
  - 07: `import_approval_timed_out`, attempt_recorded False, dispatch state None.

**Frame h.**
- 08: `master_alter_id 220 voucher_alter_id 3`.
- The console line as in Q2.
- The stop file: `session ended; server pid 17448 exit code 0`. The server GONE; `count bridge_mcp.exe 0`.
- The server's error log was 0 bytes.

**Frame i.** after-L1 marks equal to s5r; F-0101 had 0 VOUCHER rows.

## L0: OLD from `exe-old` (step 7)

**Frame a.** Three folders ABSENT, then five MADE. The launch file placed and matched (59f99e32bdbd30c43f55e329a2298d757c1a958ee904a894c9c91b21296f5b85). The exe hash matched OLD, 35,869,184 B.

**Frame b.**
- The Gateway with the one company and no dialog.
- C2: one tally.exe on 9001; /status status=200, bytes=51.
- pre-L0 marks equal to s5r.
- Processes: `count bridge_mcp.exe 0`.

**Frame c.** The same start command and shell as L1. Server pid 16700 at 15:16:22Z; initialized 0.7.0.

**Frame d.** 01, 02 and 03 as in L1.

**Frame e (chain read).**
- Server and parent: as in Q2. The server's console_handle was 0x48, its std handles 0x15C/0x16C/0x164.
- The bash ancestor: `LAYOUT_CHECK_FAILED`, as in L1.

**Frame f.** 04-build-a: `voucher_count 1`, batch bridge-60c3cc1d-19ff-4066-8eab-498b0ea91c7e, `hsn_warnings 0`.

**Frame g.**
- Post 05 was committed at 15:17:33Z.
- Reads as above.
- Post answers:
  - 05: pending, dialog_remaining_s 96;
  - 06: pending, dialog_remaining_s 15;
  - 07: `import_approval_timed_out`, attempt_recorded False, dispatch state None.

**Frame h.**
- 08: `master_alter_id 220 voucher_alter_id 3`.
- The console line as in Q2.
- `session ended; server pid 16700 exit code 0`. The server GONE.
- The server's error log was 0 bytes.

**Frame i.** after-L0 marks equal to s5r; F-0101 had 0 VOUCHER rows.

## L2 (step 8)

Not run.
- Its pair would be an OLD launch read `M0`, `MB` or `M1`.
- L1 read `M1`, but L2 PATH is `no`.
- L0 read `N`.
- No exe was copied to make a path.

## Restore (step 9, on the recorded RESTORE)

**C6 prep.**
- BK1 was re-hashed: the same file line as step 2, files 1.
- The restore plan was written.
- The company data folder listed 23 files before.

**The restore.**
- Shut Company: the list showed BRIDGE EINVOICE LAB alone.
- Data, Backup & Restore, Restore, Specify Path, the BK1 folder typed in parts.
- The list showed BRIDGE EINVOICE LAB at 24-Nov-26 (8:32 pm), the step 2 time.
- `Accept ?` answered `y`; "Company data folder (<company number>) already exists" answered Replace (`r`).
- Select Company followed, with no other prompt.

**Data folder after.** 15 files. Absent after (names only): CmpSchMetaData.1800, Index.1800, TCMPSCHACCESS.TSF, TCMPSCHMESSAGE.TSF, TCMPSCHSTATE.TSF, TCMPSCHUPDATE.TSF, TINTMSG.TSF and TUPDATE.TSF. Row 2569-R's restore left out the first seven; TUPDATE.TSF is an eighth here, recorded as it stands.

**Read-back.** The book was loaded alone. s12 was equal to s1: the book's GUID, ALTVCHID 3, ALTMSTID 215. K-customer had 0 LEDGER rows; F-0101 had 0 VOUCHER rows.

**C10.**
- Shut Company: Select Company, with no company loaded.
- One tally.exe on 9001; /status status=200, bytes=51.
- `count bridge_mcp.exe 0`, `count python.exe 0`.
- Current date Thursday, 3-Apr-2025 at the Gateway before the shut.
- No XML was sent after the shut.

## Deviations

1. **Lab helpers in `local/`.** `c2.py`, `cap.py` and `capwin2.ps1` were added there, the last one copied with the shell's `cp`. This was accepted on the recorded word, on condition that such acts stay inside `local/`.
2. **An empty file.** A stray shell redirect made an empty `local/notes-add.txt`. It was recorded and left in place.
3. **Spaces in typed text.** The type tool's lone spaces did not reach Tally.
   - The step 1 and step 2 filters read without spaces; each still gave the one right line.
   - The first CGST ledger name was cleared with Backspace before any other field and typed again with the space key.
   - Every later space was typed with the space key.
4. **The sleep 15 began late.** In both launches, post 05's answer arrived while S1 to R3 ran, so the `sleep 15` began after R3 (L1 at 15:11:27Z, L0 at about 15:18:27Z), not right after the answer.
5. **The bash ancestor's read.** Its chain line read `LAYOUT_CHECK_FAILED` in both launches. It was recorded, not a stop.
6. **The S1 images.** They were copied by `python -I` from the screen tool's own saves into `local/shots/` and are kept local. The L0 image shows the box text.
7. **Files written to `evidence/`.** `restore-plan.txt` was written to `evidence/`, not the row folder's top. `c10-procs.txt` is a file the sheet does not name, written to `evidence/`. Both were recorded and left in place.

## Not covered

- The helper's error output on this path, and so the restore line.
- Creation flags, read directly.
- Other MCP clients, a server started by the desktop app, other Windows versions or accounts, other boxes.
- Why the screen tool hides a window outside its grant. This row reads only that it did.
