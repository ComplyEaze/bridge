# Windows item 1780-W (v3 of 1780-R6-bis): the approval box on master's build, through a real post, a key, Alt+Tab and the desktop client. Every post answered No by mouse. (Part of #1780)

Not for merge. Written 11 Nov 2026 on master 7a3c9e49, the build's head. Base: `lab/1780-r6bis-hidden-owner` at 8fd27158 (v2, never run). Confidence of everything here until the row runs: unverified.

WRITES to the book: NONE. Every box is answered **No**: by mouse in every cell except cell K, where one `n` key answers it. Posting is on for parts S and D only, and every `post_import` there is answered No by mouse. **No key is pressed while a box exists** except the three named carve-outs below (K, K2, HA). **No taskbar button is clicked**: the taskbar is only read.

## Why
On master, every Windows approval box is opened `MB_TOPMOST | MB_SYSTEMMODAL` (#2107) and owned by a hidden topmost window that its own process makes (#2193). See `show_review`, `src-tauri/src/tally/approved_import.rs:3419-3466`, `guarded` at `src-tauri/src/tally/box_fit.rs:120`, which makes the owner at `:141`. The box's first line says that Y and N answer it (#1832).

Already read on builds that hold both:
- **2161-C v3** (`lab/2161-c-answers`, `v3/`, 187e882b): the product's dialog child under the lab's stand-in parent, no server. The owner was `Static` with topmost 1, the box read topmost 1, and no taskbar button was added. Alt+Tab was not measured, because the box held the foreground and the gate needed Tally in front.
- **3 Nov rows** (`lab/852-capture-answers` `box/a-box.txt`, `lab/2210-census-answers` `box/batch-box.txt`, `lab/1038-release-answers` `box/posta-box.txt`): boxes opened by the server read topmost 1 and above Tally when they opened.
- **1392-E** (call 1) and **1713-S** (step 6): a mouse No declined.

What this row reads, and no earlier row has:
- **W1 (part S).** Where a box opened by the server ends up after one click into Tally.
- **W2 (part D).** The same under the desktop client as the server's parent. Never run: it was part D of row 1780-H.
- **W3 (cell HA).** Whether the box is listed in Alt+Tab while Tally holds the foreground.
- **W4 (cells K and K2).** Whether a typed `n` reaches the box when the box holds the foreground (last measured on the 0.6.0 file, 16 Oct). Whether a key goes to Tally, not the box, once a click has given Tally the foreground.

## Who acts
- **The lab** does every step on this synthetic company, every click into Tally, every No and the keys of cells K, K2 and HA, on the owner's standing word of 27 Oct 2026 (synthetic companies) and 28 Oct 2026 (posting on, answering the approval box). Record each as "done by the lab on the owner's standing word".
- **Part D** runs only through the lab's standing desktop-client entry (Part D below).
- **The run coordinator** gives the go-ahead and any STOP ruling, and the yes for cell HA.

## Build (hash before the first step and at the end; a different hash = STOP)
**BUILD-W:** the `unsigned-bundle-smoke-windows-latest` artifact of the Bridge CI merge-queue run of #2497 at 7a3c9e4991d967c33cc450e56cac6844db103353, master's head on 11 Nov 2026. The run's conclusion is success.
- Find the run in the repository's Actions list as the `merge_group` run of that head. Its run id and the artifact's id are not written here as numbers; record them in `notes.txt` as fetched.
- The artifact is 60,423,867 bytes, GitHub digest `sha256:bb058e556b791e0e1d744f0ef4bc5f57280eba84298ee60da27b3c94d6c09102`, and it expires on 18 Nov 2026.
- EXPECT the zip's sha256 to equal the digest, and `bridge_mcp.exe` and `pdfium.dll` to match `mcpb-smoke.json`'s hashes. Extract the WHOLE `.mcpb` into `C:\bridge-lab\1780w\build\`. A mismatch = STOP.
- Downloading master's bundles needs no ask (the owner's word of 29 Oct).

## Inputs (`SHA256.txt` lists every file of this folder with its size; check all before the first step)
- `LAB-COMMON.md` and `requests/` other than `tally-window-capture.ps1` are byte copies of v2's folder at 8fd27158. They come from rows 1780-H and 2161-B, where they ran.
  - `confirm-input.txt` holds a nonce line, the count 1, and a 23-line box text with a zero GUID and no amounts. The child prints a token only on Yes; with no server, nothing can post.
  - `build-1591-s1.json` is part S's voucher: a Receipt on 14 Sep 2026, entries of 141, narration "Lab topmost probe 1".
- **New: `requests/tally-window-capture.ps1`.** One `PrintWindow` copy of Tally's own window, so the lab can read which Tally screen shows while a box exists. It is adapted from `taskbar-capture.ps1`, which ran in 2161-B; only the window it copies differs. Its C# compiled on a Mac with Mono's `mcs`; its PowerShell was not run anywhere. **B3c is its known answer.**
- Working folder `C:\bridge-lab\1780w\`; evidence in `C:\bridge-lab\1780w\evidence\`.
- **THE COMPANY** is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`, synthetic, loaded alone, with Tally maximised on Gateway of Tally. Last marks (2161-C v3, 4 Nov): ALTVCHID 87, ALTMSTID 224.
- Scripts run from Windows PowerShell 5.1 as `powershell -NoProfile -ExecutionPolicy Bypass -File <file> ...`. A comma is a digit separator on this PC, so no command takes a list.
- **The screen tool** takes no capture while a box exists. While a box exists, screen copies come only from the scripts, started by the pre-armed timing helper.

## Rules (they add to `LAB-COMMON.md`; where they differ, the stricter holds)
- **KEYS.** From the start of a cell until its box is answered or gone, no key is pressed and nothing is typed, except under one of the three carve-outs. Everything that reads is pre-armed before the cell. Boxes are answered by mouse, at the centre of the No button.
- **Carve-out K (cell K only).** ONE press of the letter `n`, only while the probe just before shows the box holding the foreground. Never `y`, Enter, Space, Tab or any other key.
- **Carve-out K2 (cell K2 only).** ONE press of Down, only when the probe just before shows Tally holding the foreground with the box topmost above it, AND that cell's Tally copy shows Gateway of Tally. Any other Tally screen, or a copy that cannot be read: K2's key is not pressed. Write "K2 not run: <what the copy showed>".
- **Carve-out HA (cell HA only, on the run coordinator's yes).** Alt, Tab and Esc, in the order cell HA gives, only while the probe just before shows Tally holding the foreground. Never Enter, Space, Y, N or any other key. An Esc that reached the box would be its Cancel, which is a decline.
- **C2 carve-out.** As v2: `spawn-parent.ps1` ends only the child it started itself, by its own pid and handle, after `-TimeoutSeconds` (60). The lab stops only the server launch it started itself, by closing its own launcher. Nothing else is ended.
- **A box under Tally.** As v2: one click on Tally's minimise button, No by mouse, one click on Tally's own taskbar button. Record each click. The box's own taskbar button, if one shows, is never clicked.
- **WRITES OFF.** On any STOP or FAIL after S1, and after part D:
  1. answer any open box No by mouse;
  2. stop the launch;
  3. remove `BRIDGE_AGENT_ENABLE_WRITES` from the environment file and record the file's sha256;
  4. report.
- **A write in the book** (a voucher posted or a mark moved) = FAIL. Apply WRITES OFF and change nothing in Tally. BK0 is restored (C6) only on the run coordinator's word.
- **Leftover list (B5)** before the sitting, after part H, after part S, after part D and at the end:
  - processes whose command line contains `--confirm-journal`, `spawn-parent.ps1` or `box-trace-early.ps1`;
  - visible windows whose title contains "ComplyEaze Bridge".

  EXPECT none. Any: record it, end nothing, STOP.
- **Privacy.** Before committing evidence:
  - search it for the user's name and for paths outside the lab folders, replace each by `<user>` or `<path>`, and record the edit;
  - keep only the port-9001 LISTENING lines of netstat output;
  - keep screen copies and Tally copies local, with name, size and sha256 in `SCREENSHOTS.txt`;
  - write CI run and artifact ids in words, never as numbers.

## Before the sitting
- **P0.** BRIDGE BATCH LAB is in Select Company's list. Load it alone, by its name typed as a filter, one row, Enter. Otherwise STOP: do not restore it, and do not use another book. MARKS: record ALTVCHID and ALTMSTID.
- **B1.** Fetch and check BUILD-W as above, and hash every file of this folder against `SHA256.txt`.
- **B3. Probe known answer.** Click Tally's title once, then run `zorder-probe-k.ps1`. EXPECT Tally listed with foreground 1. Record P-time, the run time of one probe.
- **B3b. Trace known answer and the owner.** Run part H's recipe once as cell H0: no click into Tally at any point, and No by mouse at about T0+10. EXPECT:
  - a `FIRST` line, and a first window block within 1 s, in `trace-H0.txt`;
  - the box's line with `owner` not 0, `owner_class` `Static` and `owner_topmost` 1;
  - exit 1, `stdout_bytes=0`, and at least one `box-fit:` line in `H0.stderr`, each reading `fits=true` with the box's own rectangle (never `0,0,0,0`).

  `owner` 0 here means this is not BUILD-W or the owner was refused: STOP and report.
- **B3c. Tally copy known answer** (no box open, Tally maximised on Gateway of Tally):
  1. Take one screen-tool screenshot. It is allowed, since no box exists.
  2. Run `tally-window-capture.ps1 -Name b3c -OutDir <evidence> -Out <evidence>\tally-b3c.txt`.
  3. EXPECT one line with `print_window=True` and the rect of Tally's maximised window.
  4. EXPECT `b3c-tally.png` to show Gateway of Tally, its menu and its highlighted line as the screenshot shows them.

  A blank, black or different copy: the script has no known answer. K2 is then not run ("K2 not run: no Tally copy"); the rest of the row runs.
- **B4. The two frames.** Record the screen tool's frame, and the screen's logical size and scale. Every click below is recorded in both frames, with the box's rectangle in logical pixels.
- **B4b. The idle taskbar.** As v2, with the lab's consoles started minimised: `taskbar-cover.ps1` and `taskbar-capture.ps1 -Name idle`.
- **B5 before.**

## Part H: the product's child under the lab's stand-in parent (no server; nothing can post)
A cell is a two-line `<cell>.cmd` in the working folder:
```
powershell -NoProfile -Command Start-Sleep -Seconds 8
powershell -NoProfile -ExecutionPolicy Bypass -File C:\bridge-lab\1780w\requests\spawn-parent.ps1 -Target child -ChildExe C:\bridge-lab\1780w\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe -InputFile C:\bridge-lab\1780w\requests\confirm-input.txt -Stdio files -Out C:\bridge-lab\1780w\evidence\<cell>.txt -TimeoutSeconds 60
```
**The recipe**, for H0 and the cells below unless a cell says otherwise:
1. **Pre-arm**, each in its own window, started minimised and waiting:
   - `box-trace-early.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-<cell>.txt -Seconds 100`;
   - the timing helper, for `zorder-probe-k.ps1` at T0+8 and T0+11, and at C+0 and C+6 if the cell clicks into Tally;
   - after those, `box-rect-probe.ps1 -Out <evidence>\rect-<cell>.txt`.

   Tally is maximised on Gateway of Tally and in front.
2. **Start `<cell>.cmd`** from the lab's interactive PowerShell. About 4 s after the start, once its console is up, click Tally's title area ONCE. After that there is no input except what the cell names.
3. **T0** is the stamp on the `SPAWN ... child_pid=` line of `<cell>.txt`. If no box titled "ComplyEaze Bridge" exists by T0+15: record "no box", end nothing, and wait for `<cell>.txt`'s end line (at most the 60 s bound). Then go to the next cell.
4. **Record**, from the trace: `FIRST`; the box's hwnd; `exstyle` and `topmost` in every block, with the stamp of any change; `owner`, `owner_class`, `owner_exstyle`, `owner_topmost`; `fg_pid` at the first block. From the probes: z and the foreground at each read. From `<cell>.stderr`: each `box-fit:` line, whole. Then the end of `<cell>.txt`: exit code and `stdout_bytes`.

### Cell K: a typed `n` while the box holds the foreground
The recipe, with no click into Tally after step 2's title click.
1. **At T0+11,** the pre-armed probe must show the box topmost and holding the foreground. Otherwise write "K not run: the box did not hold the foreground", answer No by mouse, and go to K2.
2. **At about T0+13:** press `n` ONCE (carve-out K). Press nothing else.
3. **Read:** one `zorder-probe-k.ps1` at about T0+15.
4. EXPECT the box gone by that probe, and `<cell>.txt` ending exit 1, `stdout_bytes=0`, as a mouse No does in 2161-B and 2161-C. Expect only `fits=true` lines. If the box is still open at the probe: record "the key did not answer the box", answer No by mouse, and STOP before K2.

### Cell K2: Down after a click into Tally
The recipe, with these additions:
1. **The click into Tally, at T0+14 (±2 s):** ONE left click on a blank part of Tally's Gateway screen. It must be at least 150 logical pixels from every edge of the box, and not on a menu item, button, list line or field. Record the point in both frames and its distance from the box. The pre-armed reads run at once (C+0) and 6 s later (C+6). At C+6, the helper also runs `tally-window-capture.ps1 -Name k2a`.
2. **The gate (carve-out K2).** The C+6 probe shows Tally holding the foreground with the box topmost above it, AND `k2a-tally.png` shows Gateway of Tally. If either fails, write "K2 not run: <the probe line, or what the copy showed>", answer No by mouse, and go to part S.
3. **At about C+8:** press Down ONCE.
4. **At about C+10:** one `zorder-probe-k.ps1`, then `tally-window-capture.ps1 -Name k2b`.
5. EXPECT:
   - the box still open, topmost and above Tally;
   - Tally holding the foreground;
   - `k2b-tally.png` still on Gateway of Tally, with the highlighted line one below `k2a`'s, or at the next line Tally moves to.
6. **No** by mouse. EXPECT exit 1 and `stdout_bytes=0`.

### Cell HA: Alt+Tab while Tally holds the foreground (only on the run coordinator's yes; otherwise write "HA not run")
This runs under the stand-in parent, not inside part S, so that part S stays one post answered once, inside the call's wait of about 40 s (`CALL_BUDGET`, `src-tauri/src/agent_import_approval.rs:46`). The box under test is the same child and the same owner code.

The recipe, with K2's step 1 click and C+0 and C+6 reads, and no Tally copy.
1. **At C+6,** the probe must show the box topmost and Tally holding the foreground. Otherwise write "HA skipped: Tally did not hold the foreground", answer No by mouse, and go to B5.
2. **At about C+8:** press and HOLD Alt, press Tab ONCE, and keep Alt held. The switcher opens.
3. **Read the switcher by eye**, Alt still held. Write down the title of every tile, in order, and whether one reads "ComplyEaze Bridge". The screen tool takes no capture.
   - **STOP rule: HA never selects the box's tile.** If the switcher shows a "ComplyEaze Bridge" tile anywhere, highlighted or not, press no further Tab and never release Alt while the switcher is open: Esc comes first.
4. **With Alt still held, press Esc ONCE.** Then release Alt.
   - If the switcher is still open after the Esc: keep Alt held, press nothing else, click nothing, and STOP. Alt is released only on the run coordinator's word.
5. **Read:** one `zorder-probe-k.ps1`. EXPECT Tally in front. If the box is in front, record "the switch was not cancelled".
6. **No** by mouse. EXPECT exit 1 and `stdout_bytes=0`.

Run B5 after part H.

## Part S: a real post on BUILD-W under the lab's launch, answered No
- **S0.** Back up THE COMPANY (BK0, C5) to the new empty folder `C:\bridge-lab\1780w\bk0\`. Record its name, size and sha256.
- **S1. Posting on for this launch.** In the launch's environment file, set `BRIDGE_AGENT_ENABLE_WRITES` to `true` and point the launch at BUILD-W's `bridge_mcp.exe`. Record the file's key names (not values) and its sha256. Start the launch.
- **S2.** `tally_status` (agent.rs:3548) and `list_companies` (agent.rs:3507): EXPECT BUILD-W's version, and THE COMPANY as the one company. Then `ledger_masters` (agent.rs:3505), basic: record the voucher and master marks. Every later compare is against these.
- **S3.** `build_import_xml` (agent.rs:3495) with the exact content of `build-1591-s1.json`. EXPECT `result.voucher_count` 1, and `total_debit` and `total_credit` 141 each. Record `verification_preflight` whole. Keep `result.batch_id` as BATCH-S. A refusal, or another count or total = STOP (nothing has been sent).
- **S4.** With Tally maximised on Gateway of Tally and in front, click its title area ONCE and stamp it. Then call `post_import` (agent.rs:3533) with BOTH `company_guid` and `batch_id` BATCH-S, and stamp T0. The server starts the dialog child (`--confirm-journal`, lib.rs:113), which shows `show_review` inside `guarded`.
  - No input after T0 until S6.
  - Pre-armed: the trace (`-Seconds 150`), and `zorder-probe-k.ps1` at T0+3, T0+8 and T0+13. If P-time is over 3 s, start each read when the previous one ends.
  - Record the interval from the title click to `FIRST`.
- **S5.** The box's title reads `ComplyEaze Bridge — post this voucher?` and its first line `Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer.`. No is drawn as the default.
  - **STOP rule:** the lines under the first must name THE COMPANY and the content of `build-1591-s1.json` (a Receipt on 14-Sep-2026, entries of 141, the narration "Lab topmost probe 1"). A different company, voucher type, date, entry, total or narration: No by mouse, then STOP.
  - Other lines of ComplyEaze Bridge's own wording are recorded word for word and are not a STOP.
- **S6. The click into Tally at T0+16 (±2 s)**, chosen and recorded as in K2's step 1 (no Tally copy), with the reads at C+0 and C+6.
- **S7. No** by mouse, before T0+38. EXPECT `import_approval_declined`, `attempt_recorded` false, nothing dispatched. If the call has already answered `pending`, click No all the same and record "MODE BLOCKED".
- **S8.** `ledger_masters` (basic): EXPECT the marks as at S2; a moved mark = FAIL. Then B5, stop the launch, and apply WRITES OFF unless part D runs next.

## Part D: the desktop client as the server's parent
- **D0. The entry.** Part D runs only through the lab's standing desktop-client entry for ComplyEaze Bridge: one permanent entry with posting on and the owner's Terms value typed once, updated to a new build by the lab, path and hash recorded (the owner's word of 15 Oct). Write "Part D not run: <reason>" and finish the row with B5 and C10 if any of these holds:
  - no such entry exists;
  - pointing it at BUILD-W needs anything other than its executable path;
  - the Terms value would need to be typed again;
  - the desktop client is not signed in.

  Do not improvise another entry or route.
- **D1.** Point the entry at BUILD-W's `bridge_mcp.exe`, restart the desktop client, and record the entry's key names and the executable's sha256. THE COMPANY is loaded alone and Tally is in front on Gateway of Tally.
- **D2.** Build a new batch from `build-1591-s1.json` through the desktop client (BATCH-D), so that nothing answered in part S can answer it. Pre-arm the trace and reads as in S4. Then ask the desktop client to post BATCH-D, answer its own tool prompt for `post_import`, and stamp T0 when the call starts. Click once into Tally at T0+16 (±2 s), as in S6, then answer **No** by mouse.
- **D3.** Record what part H's step 4 records, except the stderr (a server does not keep the child's). EXPECT `import_approval_declined`.
- **D4.** Point the entry back at the executable it had before D1, recording its sha256. WRITES OFF, the marks unchanged, B5.

## Close
C10: shut THE COMPANY. EXPECT one tally.exe owning 9001 and `GET /status` 200, with none loaded.

## Reading it (for the requester)
| reading | meaning |
|---|---|
| S: topmost 1 in every block, and the box above Tally at C+0 and C+6 | **W1 holds:** a server-opened box on master stays above Tally after a click into it. |
| D: as S | **W2 holds:** the same under the desktop client. With W1, the Windows topmost question on #1780 is answered on this PC. |
| S or D: topmost 0, or Tally above the box at C+0 or C+6 | A finding: the combination does not hold under that parent. Report the trace whole; a code change is chosen from it, not before. |
| K: the box gone, exit 1, no token | **W4a holds:** a typed `n` answers the box, as its first line says. |
| K: the box still open, or the key reached Tally | A finding: the box did not take the keyboard although its first line says Y and N answer it. |
| K2: the box open, Tally's highlight moved | **W4b holds:** once Tally has the foreground, a key typed for Tally goes to Tally. |
| K2: the box open, Tally's highlight not moved | A finding: the key reached neither Tally's menu nor a visible answer. Record the box's focused button if it reads differently. |
| K2: the box answered or gone | A finding: a key typed for Tally answered the box after a click had given Tally the foreground. STOP. |
| HA: a "ComplyEaze Bridge" tile | **W3:** the owned box is listed in Alt+Tab, so there is a keyboard route back to it. |
| HA: no such tile | **W3:** the owned box is not in Alt+Tab; only its place on top shows it. |
| `owner` 0 anywhere, a `fits=false` line, `0,0,0,0`, exit 77, or `import_review_too_large` | Report the lines whole. |
| Anything else | Record it whole and infer nothing. |

**FAIL of the row:**
- a Yes, by click or key;
- any post;
- a mark changed;
- a key pressed while a box existed, other than under carve-outs K, K2 and HA;
- a taskbar click;
- a No (or cell K's `n`) that answered anything other than a decline;
- a box or process left at the end (B5);
- B3 or B3b's known answer not met.

## Evidence (stays on the lab PC except the files listed)
- `RESULTS.md`: each of W1 to W4 first, then one line per cell and step, with every stamp, probe line, exstyle, topmost, owner field, foreground, rectangle and answer.
- Each `<cell>.txt`, `<cell>.stderr`, trace, rect and Tally-copy text line, whole, after the privacy search.
- The MCP answers of parts S and D, as JSON.
- `notes.txt` in UTC order, `SCREENSHOTS.txt` and `SHA256.txt`.
- Also record the Windows build, the Tally release, the desktop client version and the monitor count, read afresh.

Evidence goes on a new branch, `lab/1780-w-answers`. Send its head to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.

## Not measured (say so in the results)
- The review, release, settlement and ledger boxes, though they share the owner code.
- `y` as a key: a Yes by key is never pressed.
- Keys other than `n`, Down, Alt, Tab and Esc.
- A box drawn under another window.
- A second monitor, Windows 11, scaling other than this PC's, and more than one sitting.
- macOS: Return and Escape on the Mac alert (row C2-6) is the Mac remainder of #1780.
