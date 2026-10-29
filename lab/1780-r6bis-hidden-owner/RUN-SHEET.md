# Windows item 1780-R6-bis v2 (DRAFT, run only if R6 / Part D reads topmost 0): the approval box owned by a hidden topmost window, under the test parent, the lab's launch and the desktop client. Answered No by mouse. (Part of #1780)

WRITES to the book: NONE. Every box is answered **No, by mouse**. Posting is switched on for parts S and D only, and every `post_import` there is answered No. **No key is pressed in this row**, at any time a box exists. **No taskbar button is clicked**: the taskbar is only read.

## Why
- **Row 1780-H** (`lab/1780-system-modal-box-answers` @ 968810d4): with `MB_SYSTEMMODAL`, every box read topmost 1 under the test parent and the lab's launch.
- **Its part D**, run at the release sitting as R6, puts the desktop client in the parent's place. **This row is run only if R6 reads 0**: the box opened under the desktop client lacks the topmost bit, or Tally covers it after a click.
- **The fix it tests** is the branch `fix/1780-hidden-topmost-owner`:
  - `box_fit::guarded` makes a hidden, topmost tool window on the box's thread (`TopmostOwner`: `CreateWindowExW(WS_EX_TOPMOST | WS_EX_TOOLWINDOW, "Static", …, WS_POPUP, 0, 0, 0, 0, …)`, never shown) and passes it as each box's owner;
  - `MB_SYSTEMMODAL` is kept;
  - the fit check's hook skips the owner, so an owner that Windows activates as the box closes is not measured as a box that did not fit.
- **Its precedent.** Row 1780-G's variant i measured that owner on a stand-in box, without `MB_SYSTEMMODAL`: topmost 1, above Tally, twice (`lab/1780-topmost-fixes-answers` @ ba103f22). The combination with `MB_SYSTEMMODAL` is not measured.
- **What this row decides:**
  - **R1.** Is the product's box owned by a hidden topmost window: does the trace's box line read `owner` not 0, `owner_class` `Static` and `owner_topmost` 1? Does the box read topmost 1 and stay above Tally after a click into Tally, under the test parent, the lab's launch and the desktop client?
  - **R2.** Was a taskbar button added for the box? EXPECT none: an owned window has no button. Read against the idle taskbar copy, not alone.
  - **R3.** Does a mouse No still answer `import_approval_declined`, with nothing sent? Does the fit check still read the box and only the box (`fits=true`, the box's own rectangle, never exit 77 after a No)?

## Who acts
- **The lab** does every step on this synthetic company, the clicks into Tally and every No included, on the owner's standing word of 27 Oct 2026. Record each as "done by the lab on the owner's standing word of 27 Oct". Posting for parts S and D is on the owner's word of 28 Oct.
- **The owner's own steps.** Part D needs the owner at the PC, as R6 did.
- **Downloading** the branch's CI bundle needs the owner's OK in the lab chat.
- **The run coordinator** gives the go-ahead and any STOP ruling.

## Build and inputs (hash before the first step and at the end; a different hash = STOP; `SHA256.txt` lists every file of this folder with its size)
- **BUILD-R6B:** the `unsigned-bundle-smoke-windows-latest` artifact of the CI run on `fix/1780-hidden-topmost-owner` at the head the run coordinator names.
  - The run's conclusion must be success.
  - Record the run id, the artifact id and its GitHub digest.
  - EXPECT:
    - the zip's sha256 equals the digest;
    - `bridge_mcp.exe` and `pdfium.dll` match `mcpb-smoke.json`'s hashes.
  - Extract the WHOLE `.mcpb` into `C:\bridge-lab\r6b\build\` (pdfium.dll beside the exe). A mismatch = STOP.
- **Byte copies:**
  - `requests/box-trace-early.ps1`, `zorder-probe-k.ps1`, `spawn-parent.ps1`, `confirm-input.txt` and `build-1591-s1.json` from `lab/1780-system-modal-box` @ 693a04f5258cb14592b197b966de5aa1e60c0572;
  - `requests/box-rect-probe.ps1`, `taskbar-capture.ps1` and `taskbar-cover.ps1` from `lab/2161-taskbar-activate-b` @ 996f869ffa4eec7c4a9be53a27b89ea50d1b57b0.

  All of them ran in rows 1780-H, 2161-A and 2161-B, and are kept byte for byte so their hashes hold. Some headers still say "NOT RUN"; that predates those rows.
  - `confirm-input.txt` holds a nonce line, the count 1 and a 23-line box text with zero GUID and amounts.
  - The child prints a token only on Yes. With no server, nothing can post.
- **Working folder** `C:\bridge-lab\r6b\`; evidence in `C:\bridge-lab\r6b\evidence\`.
- **THE COMPANY** is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`, loaded alone, with Tally maximised on the Gateway of Tally.
- **Scripts** run from Windows PowerShell 5.1 as `powershell -NoProfile -ExecutionPolicy Bypass -File <file> ...`.
  - A comma is a digit separator on this PC, so no command takes a list.
  - Point names (`-Name`) differ by more than letter case.
- **The screen tool** takes no capture while a box exists. Screen copies come only from the scripts above, started by the pre-armed timing helper.

## Rules (they add to `LAB-COMMON.md`; where they differ, the stricter holds)
- **KEYS.** No key is pressed and nothing is typed from the start of a cell until its box is answered or gone.
  - Everything that reads (the trace, the probes and the copies) is started before the cell, by a pre-armed timing helper.
  - From T0 until the box is answered or gone, no one types, presses Enter or clicks in any console.
  - Boxes are answered by mouse, at the centre of the No button.
- **C2 carve-out (named).** `LAB-COMMON.md` C2 forbids ending any process. This row has two exceptions:
  - `spawn-parent.ps1` ends exactly one process: the child it started itself, by the pid and handle its own `CreateProcess` returned, if that child is still running after `-TimeoutSeconds` (60).
  - The lab stops the server launch it started itself (S8, and under WRITES OFF) by closing its own launcher, never by process name or number.

  Nothing else is ended. A box or process left after the sitting is recorded and left.
- **A box under Tally.** If a box lies under Tally and must be answered:
  1. one click on Tally's minimise button (never the close button beside it);
  2. No by mouse;
  3. one click on Tally's own taskbar button to restore it.

  Record each click. Nothing is typed. The box's own taskbar button, if one shows, is never clicked.
- **WRITES OFF.** On any STOP or FAIL after S1, and after part D:
  1. answer any open box **No** by mouse;
  2. stop the launch, as the carve-out says;
  3. remove `BRIDGE_AGENT_ENABLE_WRITES` from the environment file and record the file's sha256;
  4. report.

  This sends nothing to Tally (C8). Removing the key alone does not change a running server: the launch must be stopped.
- **A write in the book.** A voucher posted or a mark moved = FAIL.
  - Apply WRITES OFF, record what the book holds, and change nothing in Tally.
  - BK0 is restored (C6) only on the run coordinator's word.
- **Leftover list (B5)** before the sitting, after part H, after part S and at the end:
  - processes whose command line contains `--confirm-journal`, `spawn-parent.ps1` or `box-trace-early.ps1`;
  - visible windows whose title contains "ComplyEaze Bridge".

  EXPECT none. Any: record it, end nothing, STOP.
  - The hidden owner belongs to the box's child process and ends with it. B5 cannot see whether it was destroyed before that; this row does not claim to measure it (see "Not measured").
- **Privacy.** Before committing evidence:
  - search it for the user's name and for paths outside the lab folders; replace each by `<user>` or `<path>` and record the edit;
  - keep only the port-9001 LISTENING lines of netstat output;
  - keep screen copies local, with name, size and sha256 in `SCREENSHOTS.txt`.

## Before the sitting
- **P0.** BRIDGE BATCH LAB is in Select Company's list and loaded alone; otherwise STOP. Do not restore it, and do not use another book.
- **B1.** Fetch and check BUILD-R6B as above, and hash every file of this folder against `SHA256.txt`.
- **B3. Probe known answer.** Click Tally's title once, then run `zorder-probe-k.ps1`. EXPECT Tally listed with foreground 1. Record P-time, the run time of one probe.
- **B3b. Trace known answer and the owner.** Run part H's recipe once as cell H0, with no click into Tally at any point and **No** by mouse at about T0+10. EXPECT:
  - a `FIRST` line, and a first window block within 1 s, in `trace-H0.txt`;
  - the box's line with `owner` not 0, `owner_class` `Static` and `owner_topmost` 1;
  - exit 1, `stdout_bytes=0`, and at least one `box-fit:` line in `H0.stderr`, each reading `fits=true` with the box's own rectangle (never `0,0,0,0`).

  `owner` 0 here = not BUILD-R6B, or the owner was refused: STOP and report.
- **B4. The two frames.** Record the screen tool's frame, and the screen's logical size and scale. Every click below is recorded in both, with the box's rectangle in logical pixels.
- **B4b. The idle taskbar.** With no box open, Tally maximised and in front, and the lab's consoles started minimised as row 2161-B's part P-min:
  - run `taskbar-cover.ps1 -Out <evidence>\cover-idle.txt`;
  - run `taskbar-capture.ps1 -Name idle -OutDir <evidence> -Out <evidence>\capture-idle.txt`.

  Record the buttons the idle copy shows. Each later copy is compared with this one.
- **B5 before.**

## Part H: the product's child under the test parent (no server; nothing can post)
A cell is a two-line `<cell>.cmd` in the working folder:
```
powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -ExecutionPolicy Bypass -File C:\bridge-lab\r6b\requests\spawn-parent.ps1 -Target child -ChildExe C:\bridge-lab\r6b\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -InputFile C:\bridge-lab\r6b\requests\confirm-input.txt -Stdio files -Out C:\bridge-lab\r6b\evidence\<cell>.txt -TimeoutSeconds 60
```
`-Stdio files` writes the child's error output to `<cell>.stderr`, so its `box-fit:` lines are kept. Cells **H1** and **H2** follow the same recipe, with the timings of row 1780-H's part H:
1. **Pre-arm**, each in its own window, started minimised and waiting:
   - `box-trace-early.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-<cell>.txt -Seconds 100`;
   - the timing helper, for `zorder-probe-k.ps1` at `<cell>-T0+8` and `+11`, and at C+0 and C+6 for `zorder-probe-k.ps1`, then `box-rect-probe.ps1 -Out <evidence>\rect-<cell>.txt`, then `taskbar-cover.ps1 -Out <evidence>\cover-<cell>.txt`, then `taskbar-capture.ps1 -Name <cell>-c<n> -OutDir <evidence> -Out <evidence>\capture-<cell>.txt`.

   Tally is maximised and in front.
2. **Start `<cell>.cmd`** from the lab's interactive PowerShell. About 4 s after the start, once its console is up, **click Tally's title area ONCE.** Then there is no input until step 4.
3. **T0** is the stamp on the `SPAWN ... child_pid=` line of `<cell>.txt`. The probes run at T0+8 and T0+11.
   - If no box titled "ComplyEaze Bridge" exists by T0+15: record "no box", end nothing, and wait for `<cell>.txt`'s end line (at most the 60 s bound). Then run the next cell.
4. **The click into Tally, at T0+14 (±2 s):** ONE left click on a blank part of Tally's Gateway screen.
   - It must be at least 150 logical pixels from every edge of the box, and not on a menu item, button, list line or field.
   - Record the point in both frames and its distance from the box.
   - The pre-armed reads run at once (C+0) and 6 s later (C+6).
5. **No** by mouse on the box's No button, after the C+6 reads, at about T0+24. EXPECT exit 1 and `stdout_bytes=0`.
6. **Record:**
   - from the trace: `FIRST`; the box's hwnd; `exstyle` and `topmost` in the first and every later block, with the stamp of any change; `owner`, `owner_class`, `owner_exstyle` and `owner_topmost`; `fg_pid` at the first block;
   - from the probes: z and the foreground at T0+8, T0+11, C+0 and C+6;
   - from the cover reads: any window over the task list;
   - from the capture: whether a button was added for the box, against the idle copy;
   - from `<cell>.stderr`: each `box-fit:` line, whole.

   A cell reads **0** if any block shows `topmost=0` for the box, or a probe at C+0 or C+6 shows Tally above the box. Otherwise it reads **1**.

Run B5 after H2.

## Part S: a real post on BUILD-R6B under the lab's launch, answered No
- **S0.** Back up THE COMPANY (BK0, C5) before any launch with posting on, to the new empty folder `C:\bridge-lab\r6b\bk0\`. Record its name, size and sha256.
- **S1. Posting on for this launch.**
  - In the launch's environment file, set `BRIDGE_AGENT_ENABLE_WRITES` to `true` and point the launch at BUILD-R6B's `bridge_mcp.exe`.
  - Record the file's key names (not values) and its sha256.
  - Start the launch.
- **S2.** Run `tally_status` and `list_companies`: EXPECT BUILD-R6B's version, and THE COMPANY as the one company. Then run `ledger_masters` (basic) and record the voucher and master marks; every later compare is against these.
- **S3.** Run `build_import_xml` with the exact content of `build-1591-s1.json`. EXPECT:
  - `result.voucher_count` 1;
  - `total_debit` and `total_credit` 141 each;
  - `verification_preflight.state` `current_window_readable`, with `source_rows` 0 for the voucher's date.

  Keep `result.batch_id` as BATCH-S. Anything else = STOP (nothing has been sent).
- **S4.** With Tally maximised on the Gateway and in front, click its title area ONCE and stamp it. Then call `post_import` with BOTH `company_guid` and `batch_id` BATCH-S, and stamp T0.
  - No input after T0 until S6.
  - Pre-armed: the trace (`-Seconds 150`), and the reads of part H's step 1 at T0+3, T0+8 and T0+13. If P-time is over 3 s, start each read when the previous one ends.
  - Record the interval from the title click to `FIRST`. Under 5 s, record that this run did not test the late-open timing.
- **S5.** The box's title reads `ComplyEaze Bridge — post this voucher?`, its first line `Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer.`, and No is drawn as the default.
  - **STOP rule:** the lines under the first must name THE COMPANY and the content of `build-1591-s1.json`: a Receipt on 14 Sep 2026, entries of 141, and the narration "Lab topmost probe 1".
  - A different company, voucher type, date, entry, total or narration = No by mouse, then STOP.
  - Other lines of ComplyEaze Bridge's own wording are recorded word for word and are not a STOP.
- **S6. The click into Tally at T0+16 (±2 s)**, chosen and recorded as in part H's step 4, with the reads at C+0 and C+6.
- **S7. No** by mouse, before T0+38. EXPECT `import_approval_declined`, with nothing dispatched. If the call has already answered `pending` (it waits about 40 s), click No all the same and record "MODE BLOCKED".
- **S8.** Run `ledger_masters` (basic): EXPECT the marks as at S2; a moved mark = FAIL. Then B5, stop the launch, and apply WRITES OFF if part D is not run now.

## Part D: the desktop client as the server's parent (the owner at the PC)
- **D1.** The desktop client is configured to start BUILD-R6B's server with posting on, as R6 was. THE COMPANY is loaded alone and Tally is in front.
- **D2.** The lab pre-arms the trace and the reads as in S4.
  - The owner asks the desktop client for a post of BATCH-S, or of a new batch built from the same file as S3, and stamps T0 when the call starts.
  - The owner clicks once into Tally at T0+16 (±2 s), as in S6, then answers **No** by mouse.
- **D3.** Record what part H's step 6 records, except the stderr: the child's error output is not kept under a server. EXPECT `import_approval_declined`.
- **D4.** WRITES OFF; the marks unchanged; THE COMPANY shut by C10. If the owner is not at the PC, write "Part D not run" and finish the row.

## Reading it (for the requester)
| reading | meaning |
|---|---|
| The box's line reads `owner` a `Static` window with `owner_topmost` 1; the box reads topmost 1 and stays above Tally at C+0 and C+6, in H, S and D | The hidden owner holds the box on top: the fix-forward works where R6 failed. |
| The box reads topmost 0, or Tally covers it, though owned, under any parent | The owner does not hold it under that parent: report the trace whole; neither candidate works there. |
| Topmost 1 under some parents and 0 under others | Report a table per parent; nothing is concluded from the others. |
| `owner` 0 | Not BUILD-R6B, or the owner was refused (`TopmostOwner::create` gave none): check B1, then report. |
| A `box-fit:` line reading `fits=false`, a rectangle of `0,0,0,0`, exit 77 in H, or `import_review_too_large` in S or D | The fit check measured a window that is not the box, or the box did not fit: report the lines whole. The skip in the hook was meant to rule out the first. |
| No box at all, exit 0 with no token in H, or an unavailable code in S or D | The box could not be shown or measured (the hook was refused, or `MessageBoxW` failed): report it. |
| No button added for the box against the idle copy | An owned box has no taskbar button, as expected: #2161's taskbar question does not arise. |
| A button added | Record it: the box is still on the taskbar. |
| The copy shows fewer buttons than the idle copy, or none | The taskbar was emptied or covered, not "no button for the box": record the cover read and do not read R2. |
| Anything else | Record it whole and infer nothing. |

**FAIL of the row:**
- a Yes, by click or key;
- any post;
- a mark changed;
- a key pressed while a box existed;
- a taskbar click;
- a No that answered anything other than a decline (exit other than 1 in H; a code other than `import_approval_declined` or "MODE BLOCKED" in S or D);
- a box or process left at the end (B5);
- B3 or B3b's known answer not met.

## Evidence (stays on the lab PC except the files listed)
- `RESULTS.md`: one line per cell and step, with every stamp, probe line, exstyle, topmost, owner field, foreground, rectangle and answer.
- Each `<cell>.txt`, `<cell>.stderr`, trace, rect, cover and capture text, whole, after the privacy search.
- The MCP answers of parts S and D, as JSON.
- `notes.txt`, in UTC time order.
- `SCREENSHOTS.txt`.
- `SHA256.txt`, over the committed files with their sizes.
- Also record:
  - BUILD-R6B's branch, head, CI run, artifact and exe sha256;
  - the Windows build, the Tally release and the desktop client version, read afresh;
  - whether there is one monitor or more.

Evidence goes on a new branch, `lab/1780-r6bis-hidden-owner-answers`. Send its head to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.

## Not measured (say so in the results)
- **The owner's destruction before its process ends.** The child exits within milliseconds of the box, and B5 and the 50 ms trace cannot separate the two. The unit test `the_box_s_owner_is_a_hidden_topmost_tool_window_until_it_is_dropped` covers it.
- **Whether Windows activated the owner as the box closed.** The hook's skip is covered by the unit test `the_hook_does_not_measure_the_box_s_owner`. On screen, only its absence of effect is read (R3).
- **Alt+Tab**, which needs a key while a box is open.
- **The taskbar flash.** The owned box has no button for `Front`'s flash to reach. Whether the box is still noticed when it is refused the foreground is read only as its place on top.
- **A box drawn under another window and sought on the taskbar**, which this fix has no button for.
- **The other boxes on a screen:** the review, release, settlement and ledger boxes, though they share the owner code.
- **A second monitor, Windows 11, and more than one sitting.**
