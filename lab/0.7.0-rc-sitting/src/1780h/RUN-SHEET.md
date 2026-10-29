# Windows item 1780-H v1: the approval boxes opened system-modal, on the product's own child, on a real post and on a ledger create, each answered No by mouse. Does the box keep the topmost bit under the late-open timing, stay above Tally after a click into Tally, and survive a taskbar activation? (Part of #1780)

WRITES to the book: NONE. Every box is answered **No, by mouse**. Posting is switched on for parts S and L only (ledger creates for part L), and every post_import and post_ledger_creates there is answered No. The one click into Tally in each cell lands on a blank part of its screen. Part S presses the Down arrow once, only while Tally holds the foreground; nothing else is keyed.

## Why
- Row 1780-G (lab/1780-topmost-fixes-answers @ ba103f22ed) measured, on the control box under the F1d timing (the box opens seconds after the last input went to Tally): the approval box's flags read topmost 0; adding `MB_SYSTEMMODAL` read 1 twice (exstyle 0x00010108), above Tally, with Tally keeping the foreground; re-asserting `HWND_TOPMOST` after the box is shown was accepted and ignored.
- The pull request on branch `fix/1780-approval-box-stays-topmost` (#2107) now adds `MB_SYSTEMMODAL` to the post, review, invoice-stop release and ledger-create boxes and drops the re-assert. This row runs the product's own child, the real post path and the ledger-create path on that pull request's Windows build.
- A taskbar or thumbnail activation ended an unanswered box about 78 ms after the click in rows 1780-E, 1780-F (part R), 1780-C2 and 1780-D. The child's error output was discarded, so the cause is not known. This row records what that end answers.
- **What this row decides:**
  - T1. Does the product's box read topmost 1 under the late-open timing, from the child and from the server?
  - T2. After one click into Tally, is the box still above Tally, at once and 6 s later? Where is the foreground?
  - T3. Does a harmless key after that click go to Tally, not the box?
  - T4. Does a taskbar activation leave the box open? If it ends it, what did the end answer?
  - T5. A mouse No still declines with nothing sent; the box's rectangle compared with earlier rows (the frame style changed).
  - T6. Does the ledger-create box (not shown through the fit check) read topmost 1 and stay above Tally after a click into Tally?

## Who acts
- The lab does every step on this synthetic company, including the clicks into Tally, the taskbar clicks, the one Down key and every No, on the owner's standing word of 27 Oct 2026; record each as "done by the lab on the owner's standing word of 27 Oct". Posting (and, for part L, ledger creates) is switched on for parts S and L and off after them, on the owner's word of 28 Oct. Downloading the pull request's CI bundle is the owner's yes of 29 Oct.
- Part D needs the owner at the PC (see Part D). The go-ahead and any STOP ruling come from the run coordinator.

## Build and inputs (hash before the first step and at the end; a different hash = STOP; `SHA256.txt` lists every file of this folder with its size)
- **BUILD-H:** the `unsigned-bundle-smoke-windows-latest` artifact of the CI run of #2107 at the head the run coordinator names. Record the run id, the artifact id and its GitHub digest. EXPECT the zip's sha256 = the digest, and `bridge_mcp.exe` and `pdfium.dll` = `mcpb-smoke.json`'s hashes. Extract the WHOLE `.mcpb` into `C:\bridge-lab\1780h\build\` (pdfium.dll beside the exe). A mismatch = STOP.
- `requests/box-trace-early.ps1`, `zorder-probe.ps1`, `zorder-probe-k.ps1`, `spawn-parent.ps1`: byte copies from lab/1780-topmost-fixes @ c0805b145adcfbc3997f35aff1fd93e4b8f1f8f6. Their headers still say they were written on a Mac and not run; they have since run in rows 1780-F and 1780-G, and are kept byte for byte so their hashes hold.
- `requests/confirm-input.txt`: a byte copy from lab/1780-topmost-spawn @ ac1d9da957856332718a912512e903a4fb3cb502 (a nonce line, the count 1 and a 23-line box text with zero GUID and amounts). The child prints a token only on Yes, and with no server nothing can post.
- `requests/build-1591-s1.json` (one Receipt, 14 Sep 2026, "Lab topmost probe 1"): a byte copy from lab/1780-approval-box-topmost-rerun @ aa888c7475d1.
- Working folder `C:\bridge-lab\1780h\`; evidence in `C:\bridge-lab\1780h\evidence\`. THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`, loaded alone, Tally maximised on the Gateway of Tally. The `.ps1` files run from Windows PowerShell 5.1 as `powershell -NoProfile -ExecutionPolicy Bypass -File <file> ...`. A comma is a digit separator on this PC: no command takes a list.

## Rules (they add to `LAB-COMMON.md`; where they differ, the stricter holds)
- **KEYS.** From the start of a cell until its box is answered or gone, no key is pressed and nothing is typed, except part S's one Down arrow under its own condition. Everything that reads (trace, probe and screenshot helper) is started before the cell; in parts S and L a pre-armed timing helper starts every probe and screenshot, and no one types, presses Enter or clicks in any console from T0 until the box is answered or gone. Boxes are answered by mouse at the button's centre.
- **C2 carve-out (named).** `LAB-COMMON.md` C2 forbids ending any process. `spawn-parent.ps1` ends exactly one: the child it started itself, by the pid and handle its own `CreateProcess` returned, if still running after `-TimeoutSeconds` (60). The lab also stops the server launch it started itself (S10, L6, and under WRITES OFF), by closing its own launcher, never by process name or number. Nothing else is ended; a box or process left after the sitting is recorded and left.
- **A box under Tally.** If a box lies under Tally and must be answered: one click on Tally's minimise button (never its close button beside it), No by mouse, then one click on Tally's own taskbar button to restore it; record each click. Nothing is typed. The box's own taskbar button is used only in the T4 steps.
- **WRITES OFF.** On any STOP or FAIL after S1, and after part D: answer any open box **No** by mouse, stop the launch (as the carve-out says), remove `BRIDGE_AGENT_ENABLE_WRITES` and `BRIDGE_AGENT_ENABLE_MASTER_WRITES` from the environment file, record its sha256, then report. This sends nothing to Tally (C8). Removing the keys alone does not change a running server: the launch must be stopped.
- **A write in the book.** A voucher posted, a ledger created or a mark moved = FAIL: WRITES OFF, record what the book holds, change nothing in Tally; BK0 is restored (C6) only on the run coordinator's word.
- **Leftover list (B5)** before the sitting, after part H, and at the end: processes whose command line contains `--confirm-journal`, `--confirm-masters`, `spawn-parent.ps1` or `box-trace-early.ps1`, and visible windows whose title contains "ComplyEaze Bridge". EXPECT none. Any: record, end nothing, STOP.
- **Privacy.** Before committing evidence, search it for the user's name and for paths outside the lab folders; replace by `<user>` or `<path>` and record the edit. netstat output: only the port-9001 LISTENING lines. Screenshots stay local (name, size, sha256 in `SCREENSHOTS.txt`).

## Before the sitting
- P0. BRIDGE BATCH LAB is in Select Company's list and loaded alone, else STOP (do not restore it, do not use another book).
- B1. BUILD-H fetched and checked as above; hash every file of this folder against `SHA256.txt`.
- B3. Probe known answer: Tally's title clicked, `zorder-probe.ps1` prints Tally first and in front. Record P-time (one probe's run time).
- B3b. Trace known answer: run part H's recipe once as cell H0, with no click into Tally at any point and **No** by mouse at about T0+10. EXPECT a `FIRST` line and a first window block within 1 s in `trace-H0.txt`, exit 1, `stdout_bytes=0`.
- B4. The two frames: the screen tool's frame and the screen's logical size and scale. Every click below is recorded in both, with the box's rectangle in logical pixels.
- B5 before.

## Part H: the product's child under the test parent (no server; nothing can post)
A cell is a two-line `<cell>.cmd` in the working folder:
```
powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -ExecutionPolicy Bypass -File C:\bridge-lab\1780h\requests\spawn-parent.ps1 -Target child -ChildExe C:\bridge-lab\1780h\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -InputFile C:\bridge-lab\1780h\requests\confirm-input.txt -Stdio files -Out C:\bridge-lab\1780h\evidence\<cell>.txt -TimeoutSeconds 60
```
`-Stdio files` writes the child's error output to `<cell>.stderr`, so its `box-fit:` lines are kept (rows 1780-F/G showed the stdio kind does not change the bit). Cells **H1** and **H2**, the same recipe each:
1. Pre-arm, each in its own window, started and waiting: `box-trace-early.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-<cell>.txt -Seconds 100`, and the timing helper for `zorder-probe.ps1` then `zorder-probe-k.ps1` pairs at `<cell>-T0+8` and `+11`. Tally maximised and in front.
2. Start `<cell>.cmd` from the lab's interactive PowerShell. **About 4 s after the start, once its console is up, click Tally's title area ONCE.** Then no input until step 4.
3. `<cell>-T0` is the stamp on the `SPAWN ... child_pid=` line of `<cell>.txt`. Probes at T0+8 and +11; a screenshot after the second. No box titled "ComplyEaze Bridge" by T0+15: record "no box", end nothing, wait for `<cell>.txt`'s end line (at most the 60 s bound), then the next cell.
4. **The click into Tally, at T0+14 (±2 s):** ONE left click on a blank part of Tally's Gateway screen, at least 150 logical pixels from every edge of the box, not on a menu item, button, list line or field. Record the point in both frames and its distance from the box. Probes (`zorder-probe.ps1`) and screenshots at once (C+0) and 6 s later (C+6): is the box above Tally, visible, and who holds the foreground?
5. **T4, the taskbar activation, at about T0+24:** click the box's taskbar button ONCE. If that opens window thumbnails, click ONCE on the preview image of the thumbnail titled "ComplyEaze Bridge" (not Tally's, never a close mark). If no such thumbnail shows, click nothing more and record it. Record each click in both frames with its stamp. One probe at once.
   - If the box is still open: answer **No** by mouse on its No button. EXPECT exit 1, `stdout_bytes=0`.
   - If the box is gone: click nothing more; record the exit code, `stdout_bytes` and `<cell>.stderr` whole.
6. Record from the trace: `FIRST`, the box's hwnd, `exstyle` and `topmost` in the first and every later block, the stamp of any change, `fg_pid` at the first block; and from `<cell>.stderr` each `box-fit:` line. A cell reads **0** if any block shows `topmost=0` for the box, else **1**.

Run B5 after H2.

## Part S: a real post_import on BUILD-H, answered No by mouse
- S0. Backup of THE COMPANY (BK0) before any launch with posting on (C5), to the new empty folder `C:\bridge-lab\1780h\bk0\`: name, size and sha256.
- S1. **Posting on for this launch.** In the launch's environment file set `BRIDGE_AGENT_ENABLE_WRITES` to `true` and point the launch at BUILD-H's `bridge_mcp.exe`. Record the file's key names (not values) and its sha256. Start the launch.
- S2. `tally_status` and `list_companies`: EXPECT BUILD-H's version and THE COMPANY as the one company. `ledger_masters` (basic): record the voucher and master marks; every later compare is against these.
- S3. `build_import_xml` with the exact content of `build-1591-s1.json`. EXPECT `result.voucher_count` 1, `total_debit` and `total_credit` each 141, `verification_preflight.state` `current_window_readable` with `source_rows` 0 for the voucher's date. Keep `result.batch_id` as BATCH-H. Anything else = STOP (nothing has been sent).
- S4. Tally maximised on the Gateway and in front; click its title area ONCE; stamp it. Then `post_import` with BOTH `company_guid` and `batch_id` BATCH-H; stamp T0. No input after T0 until S6. Pre-armed trace (`-Title "ComplyEaze Bridge" -Seconds 150`); probes at T0+3, +8 and +13 (if P-time is over 3 s, start each when the previous ends). Record the title click to `FIRST` interval: this is the late-open timing the row needs (the server reads the book for several seconds before the box). If it is under 5 s, record that this run did not test the late-open timing.
- S5. From the trace and the T0+13 probe and screenshot: the box's `exstyle`, `topmost`, `foreground` and rectangle; the title `ComplyEaze Bridge — post this voucher?`; the first line `Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer.`; No drawn as the default. The lines under it are as row 1780-C's sheet lists them (lab/1780-approval-box-click-into-tally @ f16bbddbc7, step C2), with `<BATCH-C>` read as BATCH-H, except that the dates may read in day-month-year form (`14-Sep-2026`, `01-Apr-2026`) if BUILD-H includes #2105. **STOP rule:** a difference in any line that names the company, its GUID or number, the voucher type, the date, an entry, a total, the batch, the reference or the narration = No by mouse, then STOP. Other lines of ComplyEaze Bridge's own wording are recorded word for word and are not a STOP.
- S6. **The click into Tally at T0+16 (±2 s)**, chosen and recorded as in part H step 4. Probes and screenshots at C+0 and C+6.
- S7. **One harmless key**, right after the C+6 probe: press the **Down arrow** ONCE only if the C+6 probe and a screenshot taken immediately before the press both show the box open and Tally holding the foreground; otherwise skip. Then a screenshot. Record whether Tally's Gateway highlight moved (the key went to Tally), the box's focused button changed, or neither. If the box holds the foreground, skip and write "skipped: the box held the foreground".
- S8. **T4, the taskbar activation, after S7's last screenshot and not before T0+26:** as in part H step 5 (button once; thumbnail once if thumbnails open; one probe).
  - If the box is still open: **No** by mouse, aiming before T0+38. EXPECT the call answers `import_approval_declined`, nothing dispatched. If it already answered `pending` (it waits about 40 s), click No all the same and record "MODE BLOCKED".
  - If the box is gone: record the call's answer if it is still waiting. If it already answered `pending`, make ONE more `post_import` with the same `company_guid` and BATCH-H and record `approval.state`, the error code (`import_approval_declined` = the box was closed as a No; `import_review_too_large` = the fit check ended it; an unavailable code = the child gave no answer) and `previous_approval`. If that call opens a box, answer it **No** by mouse. Nothing more.
- S9. **Repeat for T1 and T2:** S4 to S6 once more with BATCH-H (a declined batch was never sent), then **No** by mouse before T0+38 (no Down key, no taskbar step). EXPECT `import_approval_declined`.
- S10. End of part S: `ledger_masters` (basic): EXPECT the marks as at S2; a moved mark = FAIL (see "A write in the book"). No `bridge_mcp.exe` child with `--confirm-journal` remains (B5). Stop the launch. If part L is not run, apply WRITES OFF now.

## Part L: the ledger-create box on BUILD-H, answered No by mouse
- L0. Set `BRIDGE_AGENT_ENABLE_MASTER_WRITES` to `true` as well (with `BRIDGE_AGENT_ENABLE_WRITES`) in the same environment file, record its key names and sha256, and start the launch again. `ledger_masters` (basic): EXPECT the marks as at S2, and no ledger named `Lab Topmost Suspense`; if one exists, STOP.
- L1. `prepare_ledger_creates` with `company_guid` and `ledger` `{"kind": "suspense", "name": "Lab Topmost Suspense"}`. Keep its `batch_id` as BATCH-L. A refusal = STOP (nothing has been sent).
- L2. Tally maximised on the Gateway and in front; click its title area ONCE; stamp it. **Wait 8 s with no input**, then `post_ledger_creates` with `batch_id` BATCH-L (and `company_guid` only if its schema lists it; no other argument); stamp T0. The wait gives the late-open timing whatever this call's own read takes. Pre-armed trace (`-Title "ComplyEaze Bridge" -Seconds 150`), probes at T0+3, +8 and +13 by the pre-armed helper. Record the title click to `FIRST` interval.
- L3. From the trace and the T0+13 probe: the box's `exstyle`, `topmost`, `foreground` and rectangle, and its title and first line word for word. Its text must name THE COMPANY and `Lab Topmost Suspense` as a ledger to create; a different company or ledger name = No by mouse, then STOP.
- L4. The click into Tally at T0+16 (±2 s), chosen and recorded as in part H step 4; probes and screenshots at C+0 and C+6.
- L5. **No** by mouse on the box's No button, before T0+38 (no taskbar step, no key; the box's buttons are Yes, No and Cancel; anything else = No if present, otherwise Cancel, then STOP). EXPECT `master_approval_declined` (or `pending` with "MODE BLOCKED", as in S8). Nothing created.
- L6. End: `ledger_masters` (basic): EXPECT the master mark as at L0 and no ledger named `Lab Topmost Suspense`. A new ledger or a moved master mark = FAIL (see "A write in the book"). No `bridge_mcp.exe` child with `--confirm-masters` or `--confirm-journal` remains (B5). Re-hash the inputs. Apply WRITES OFF. Shut THE COMPANY after part D, or now if part D is not run.

## Part D (owner step; not blocking the row)
With the owner at the PC: the desktop client as the server's parent, posting on, on BUILD-H. The owner asks the desktop client for a post of BATCH-H; the lab pre-arms the trace and the probes as in S4, and the owner does the one click into Tally (as S6) and answers **No** by mouse. Record `exstyle`, `topmost` and the probes. Then apply WRITES OFF and shut THE COMPANY. If the owner is not at the PC, write "Part D not run" and finish the row.

## Reading it (for the requester)
| reading | meaning |
|---|---|
| H1, H2 and both S runs read 1 and the box stays above Tally after the click | The fix holds on the child and the server path. |
| Any reads 0, or Tally covers the box | MB_SYSTEMMODAL alone is not enough on the product; a hidden topmost owner window is next. |
| S7: Tally's highlight moved | Topmost does not take the keyboard. |
| S7: the box's focused button changed while Tally held the foreground | A keyboard finding, separate from topmost: report it. |
| T4: the box stays open after the taskbar activation | The taskbar no longer ends it (the box above Tally is activated, not uncovered). |
| T4: the box ends; stderr's last `box-fit:` line reads `fits=false`, exit 77, or the follow-up reads `import_review_too_large` | The fit check ended it; its rectangle names what it measured. |
| T4: the box ends with exit 1 and no new `fits=false` line, or `import_approval_declined` | The box was closed as a No by something other than the fit check. |
| The box's rectangle differs from 686,181,1254,926 (part H) | The frame style changed its size; record it, compare with the work area. |
| L3 reads 1 and the box stays above Tally after L4's click | The ledger-create box is covered by the same fix. |

**FAIL of the row:** a Yes by click or key; any post; a ledger created; a mark changes; a key pressed while a box held the keyboard; a box or process left after L6 (B5); B3 or B3b's known answer not met.

## Evidence (stays on the lab PC except the files listed)
RESULTS.md (one line per cell and step, with every stamp, probe line, exstyle, topmost, foreground, rectangle and answer); each `<cell>.txt`, `<cell>.stderr` and trace, whole after the privacy search; the MCP answers of parts S and L as JSON; `notes.txt` in UTC time order; `SCREENSHOTS.txt`; `SHA256.txt` over the committed files with sizes. Also record BUILD-H's pull request, head, CI run, artifact and exe sha256; the Windows build, the Tally release and the desktop client version, read afresh; one monitor or more. Evidence on a new branch `lab/1780-system-modal-box-answers`; send its head to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.

## Not measured (say so in the results)
- The review acknowledgement and invoice-stop release boxes (same flags and code path, not shown here).
- Any key other than one Down arrow; a click on a Tally menu item, button or field; a Tally entry screen under the box.
- A second monitor, a scale or resolution change while a box is open; Windows 11; macOS; more than one sitting.
