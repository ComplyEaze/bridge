# Windows item 1780-K v1: does the ledger-create box now take the keyboard when it opens, as the post's box does? The product's own dialog child, before and after the change, with the box opening late and early. (Part of #1780)

WRITES to the book: NONE. **No ComplyEaze Bridge server is launched.** The only requests to Tally are `LAB-COMMON.md`'s own read-only checks: GET /status and the MARKS read of C2 (c, d) at the start; the MARKS read again just before the company is shut; and GET /status after the shut (C10, which sends no XML request after the shut).

Every box comes from the product's own dialog child, fed a fixed text by the test parent. With no server, nothing can post or create. Each box is answered **No by mouse**.

**No key is pressed in this row**, in Tally or anywhere while a box is open. The cells read where the keyboard is from the foreground window; they do not test it with a key.

## Why
- On #1780 the maintainer decided on 21 Oct 2026 to keep the keyboard: each approval box's first line says "Press Y (or click Yes) … Press N (or click No) …", and Y and N answer it.
- In row 1780-H (`lab/1780-system-modal-box-answers` @ 968810d4):
  - **The ledger-create box never had the keyboard.** Tally held the foreground in every trace block and probe (cell L2, the box opened 9.9 s after the last input to Tally), yet its first line tells the person to press Y.
  - The post's box held the foreground from its first block in four cells, and in one cell (H1) only 2.4 s after it.
- The code at #2107's head 2bc31a31 differs between the two boxes:
  - The post, review and release boxes are shown through `tally/box_fit.rs` `guarded`. Its `Front` helper, once the box appears, attaches to the foreground thread's input and brings the box to the front, once.
  - The ledger box (`show_create_ledger` in `tally/approved_import.rs`) was shown with `MB_SETFOREGROUND` alone. Under the late-open timing that did not give it the foreground: not in L2, and not for row 1780-G's control box, which has no `Front` either. Whether Windows' foreground lock or something else decided that is what K2 tests.
- **The change under test** is the draft pull request whose CI run the run coordinator names (B1). It shows the ledger box through `guarded` too, so the box gets `Front`, the fit check and per-monitor scaling together, and it keeps `MB_SYSTEMMODAL`. This row tests the change as a whole, not `Front` alone.
- **What this row decides:**
  - K1. With the box opening late (seconds after the last input to Tally), does the ledger box take the foreground on the new build, where on the old build it did not? K1-K is the deciding cell, so it runs twice.
  - K2. With the box opening right after an input to Tally, does the old build's ledger box take the foreground with `MB_SETFOREGROUND` alone? That separates the foreground lock from the flag. Does the new build's?
  - K3. Does the post's box take the foreground at once, late and early, twice each? Row 1780-H's H1 gap is the case to repeat.
  - K4 (within K1-K1). After a click into Tally, does the ledger box on the new build stay topmost and above Tally, with Tally keeping the foreground? The helper raises once and never again.

## Who acts
- The lab does every step on this synthetic company, on the owner's standing word of 27 Oct 2026: the title clicks, the clicks into Tally and every No. Record each as "done by the lab on the owner's standing word of 27 Oct". Posting is not switched on.
- Downloading the draft pull request's CI bundle needs the owner's yes, as for row 1780-H.
- The go-ahead and any STOP ruling come from the run coordinator.

## Build and inputs (hash before the first step and at the end; a different hash = STOP; `SHA256.txt` lists every file of this folder with its size)
- **BUILD-K:** the `unsigned-bundle-smoke-windows-latest` artifact of the CI run the run coordinator names (the draft pull request's head).
  - Record the pull request, its head, the run id, the artifact id and its GitHub digest.
  - EXPECT the zip's sha256 = the digest, and `bridge_mcp.exe` and `pdfium.dll` = `mcpb-smoke.json`'s hashes.
  - Extract the WHOLE `.mcpb` into `C:\bridge-lab\1780k\build\`, with pdfium.dll beside the exe. A mismatch = STOP.
- **BUILD-H**, already on the lab PC from row 1780-H: EXPECT `C:\bridge-lab\1780h\build\bin\x86_64-pc-windows-msvc\bridge_mcp.exe` to have sha256 `a946979561b61f51e6dab3539d3de98fd48c8f103f61d2e3abe787acf3240c23`. Missing or different = STOP.
- `requests/box-trace-early.ps1`, `zorder-probe-k.ps1`, `confirm-input.txt`: byte copies from `lab/1780-system-modal-box` @ 693a04f5258cb14592b197b966de5aa1e60c0572, run in row 1780-H.
- `requests/box-rect-probe.ps1`: a byte copy from `lab/2161-taskbar-activate` @ de2b1d15c50f8d1d51eb3b8617a5ef76a6d89fc4. If that row has not run it yet, B2 checks it once before any cell.
- `requests/spawn-parent.ps1`: row 1780-H's copy with ONE change, the new `-ChildMode` parameter (default `journal`).
  - It picks the child's flag `--confirm-<ChildMode>`, so `-ChildMode masters` starts the ledger box's own child. Nothing else changed.
  - It ran in rows 1780-F, 1780-G and 1780-H before this change; the change itself has not run.
  - In `-Stdio files` it writes `stdout_bytes_read=0` whatever the child printed. The child's output is read from `<cell>.stdout` instead.
- `requests/confirm-masters-input.txt`: a nonce line, the count `1`, and the ledger box's text as row 1780-H's L2 showed it, with a zero GUID and company number and the ledger name `Lab Keyboard Suspense`. The child prints a token only on Yes, and with no server nothing can be created.
- Working folder `C:\bridge-lab\1780k\`; evidence in `C:\bridge-lab\1780k\evidence\`.
- THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`, loaded alone, Tally maximised on the Gateway of Tally.
- The `.ps1` files run from Windows PowerShell 5.1 as `powershell -NoProfile -ExecutionPolicy Bypass -File <file> ...`. A comma is a digit separator on this PC: no command takes a list.

### The cell runner
The cell runner is the lab's own, like row 1780-H's `h-cell.ps1`.
- **Before the first cell**, its sha256 goes into `notes.txt`. It is committed with the answers under `scripts/`, so a reviewer can see that it sends no key and never clicks Yes.
- **During a cell** it:
  - pre-arms the trace;
  - opens the `.cmd`;
  - makes the title click and the click into Tally at the stamps named;
  - clicks No.
- So nothing is typed during a cell.
- **No click:** the runner clicks No only at the centre of the box's `&No` button, read from the box at that moment. If it cannot find the button, it clicks nothing and the lab answers No by mouse.
- **Reads:** it runs every read inline in its own console, one after another, so no new window takes the foreground.
- **When it sees `<cell>-T0`**, it records `Fg`: the foreground window's hwnd, pid, image name and title. That is the foreground just before the box appears.
- **A read R** is, in this order:
  1. the line count of `<cell>.stderr` and its last line;
  2. `box-rect-probe.ps1 -Out <evidence>\rect-<cell>-<label>.txt`;
  3. `zorder-probe-k.ps1`;
  4. each with its start and end stamps.

## Rules (they add to `LAB-COMMON.md`; where they differ, the stricter holds)
- **KEYS.** No key is pressed in Tally at any point. None is pressed anywhere from a cell's start until its box is answered or gone. Keys are used only in the lab's own console, to start the runner before a cell starts. Boxes are answered by mouse at the No button's centre.
- **C2 carve-out (named).** `LAB-COMMON.md` C2 forbids ending any process. `spawn-parent.ps1` ends exactly one: the child it started itself, by the pid and handle its own `CreateProcess` returned, if still running after `-TimeoutSeconds` (60). Nothing else is ended. A box or process left after the sitting is recorded and left.
- **A box under Tally.** If a box lies under Tally and must be answered, the clicks are, in order (record each):
  1. one click on Tally's minimise button (never its close button beside it);
  2. No by mouse;
  3. one click on Tally's own taskbar button to restore it.
- **A click into Tally** lands on a blank part of the Gateway screen. It is at least 150 logical pixels from every edge of the box, and never on a menu item, button, list line or field. If Tally's screen changes (another report or menu opens), record it, change nothing, and STOP.
- **One cell at a time.** A cell starts only after the previous cell's trace has written its `END` line, which comes at most 100 s after its start.
- **Leftover list (B5)** before the sitting, after K2-K, and at the end:
  - processes whose command line contains `--confirm-journal`, `--confirm-masters`, `spawn-parent.ps1` or `box-trace-early.ps1`;
  - visible windows whose title contains "ComplyEaze Bridge".
  - EXPECT none. Any: record, end nothing, STOP.
- **Privacy.** Before committing evidence, search it for the user's name and for paths outside the lab folders. Replace each with `<user>` or `<path>` and record the edit. This includes each probe's `F` line and the runner's `Fg`, which name whatever window is in front. Screenshots stay local, with name, size and sha256 in `SCREENSHOTS.txt`.

## Before the sitting
- P0. `LAB-COMMON.md` C1 and C2: BRIDGE BATCH LAB is in Select Company's list and loaded alone, else STOP (do not restore it, do not use another book). Record ALTVCHID and ALTMSTID from C2's MARKS read.
- B1. BUILD-K fetched and checked, and BUILD-H checked, as above. Hash every file of this folder against `SHA256.txt`, and record the runner's sha256.
- B2. With no box open, `box-rect-probe.ps1` writes `dpi_context=ok`, `boxes=0`, the foreground window and four `K` lines. Anything else, or a throw: STOP, and record its output whole.
- B3. Probe check: Tally's title clicked, and `zorder-probe-k.ps1` prints Tally first and in front. Record P-time and R-time.
- B4. Record the two frames (the screen tool's frame, and the screen's logical size and scale), and `reg query "HKCU\Control Panel\Desktop" /v ForegroundLockTimeout` (read-only). Every click below is recorded in both frames.
- B5 before.

## The cells
A cell's `<cell>.cmd`, in the working folder, is opened by the runner through `explorer.exe` (row 1780-H's way). Its spawn-parent line is:
```
powershell -NoProfile -ExecutionPolicy Bypass -File C:\bridge-lab\1780k\requests\spawn-parent.ps1 -Target child -ChildExe <EXE> -ChildMode <MODE> -InputFile <INPUT> -Stdio files -Out C:\bridge-lab\1780k\evidence\<cell>.txt -TimeoutSeconds 60
```
- `<EXE>` is BUILD-H's or BUILD-K's `bridge_mcp.exe`, as the cell says.
- `<MODE>` and `<INPUT>` are `masters` and `C:\bridge-lab\1780k\requests\confirm-masters-input.txt` for the ledger box, or `journal` and `...\confirm-input.txt` for the post's box.

Before each cell, the runner brings Tally to the front with one click on its title area (the pre-arm click). The **timing click** is a second, single click on Tally's title area, made as the cell's timing says. Record both.
- **Late open:** the `.cmd` has two lines: `powershell -NoProfile -Command Start-Sleep -Seconds 8`, then the spawn-parent line. The runner opens it, and about 4 s later, once its console is up, makes the timing click. The box then opens some seconds after that click (row 1780-H: 7.4 to 9.9 s).
- **Early open:** the `.cmd` has the spawn-parent line only. The runner opens it, and about 1.5 s later, once its console is up, makes the timing click. The box then opens about 2 s after that click (row 1780-H: about 3.4 s from a `.cmd`'s start to its box, beyond the sleep).
- Record the timing click's stamp, `Fg` at T0 and the trace's `FIRST` stamp. The cell's open delay is from the timing click to `FIRST`.

| cell | box | build | timing | click into Tally |
|---|---|---|---|---|
| K1-H | ledger | BUILD-H | late | no |
| K1-K1 | ledger | BUILD-K | late | **yes** (K4) |
| K1-K2 | ledger | BUILD-K | late | no |
| K2-H | ledger | BUILD-H | early | no |
| K2-K | ledger | BUILD-K | early | no |
| K3-L1, K3-L2 | post | BUILD-K | late | no |
| K3-E1, K3-E2 | post | BUILD-K | early | no |

Each cell, in the table's order:
1. **Pre-arm.** The runner starts `box-trace-early.ps1 -Title "ComplyEaze Bridge" -Out <evidence>\trace-<cell>.txt -Seconds 100` in its own window and waits. Tally is maximised and in front.
2. **Start.** The runner starts the cell as its timing says. There is no other input until step 4 or 5.
3. **First read.** `<cell>-T0` is the stamp on the `SPAWN ... child_pid=` line of `<cell>.txt`. At T0+6: R, then a screen-tool screenshot. If there is no box by T0+15: record "no box", end nothing, and wait for `<cell>.txt`'s end line and the trace's `END` line before the next cell.
4. **The click into Tally (K1-K1 only)**, at T0+12 (±2 s), by the runner, on a blank part of the Gateway. Record the point in both frames. R at once (C+0) and 6 s later (C+6), each with a screenshot.
5. **No.** By mouse on the box's No button. If the box lies under Tally, use "A box under Tally".
   - In K1-K1, the click comes after C+6's screenshot and not before T0+24.
   - In every other cell it comes at about T0+12.
   - EXPECT `exit_code=1` in `<cell>.txt` and `<cell>.stdout` of 0 bytes.
6. **Record:**
   - From the trace:
     - `FIRST` and the box's hwnd;
     - `exstyle` and `topmost` in every block;
     - `fg_hwnd` in the first block, and the stamp of each change of `fg_hwnd`.
   - Then:
     - **"in front"** means `fg_hwnd` equals the box's hwnd;
     - the time from `FIRST` to the first block in which the box is in front, or "never".
   - From each R:
     - the box's rectangle, `iconic` and `topmost`;
     - the foreground;
     - the z position of the box and of Tally.
   - From `<cell>.stderr`: each `box-fit:` line, and its count. BUILD-H's ledger box writes none, because it is not measured.

Run B5 after K2-K and at the end. Re-hash the inputs. Then close by `LAB-COMMON.md` C10:
1. the MARKS read while THE COMPANY is still loaded (EXPECT ALTVCHID and ALTMSTID as at P0);
2. Shut Company;
3. GET /status only, with no XML request after the shut.

## Reading it (for the requester)
| reading | meaning |
|---|---|
| K1-H: Tally in front throughout; K1-K1 and K1-K2: the box in front within 3 s of `FIRST` | The change gives the ledger box the keyboard under the late-open timing, as its first line says. |
| K1-K1 or K1-K2: the box in front, but later than 3 s after `FIRST` | It takes the keyboard, late. Record the delay, and whether `box-fit:` shows a second activation then. |
| K1-K1 or K1-K2: Tally in front throughout | The change does not give it the keyboard: report it; the fix fails. |
| K1-H: the box in front | The old build's ledger box took the keyboard here, unlike row 1780-H's L2. That L2 box was started by the server, not this test parent, so this cell does not separate the parent from the timing: record the delay and compare with L2. |
| `Fg` at T0 is the cell's console or another window, not Tally | That cell did not test "right after an input to Tally". Say so for that cell, and infer nothing about the lock from it. |
| K2-H: Tally is `Fg` at T0, and the box is in front | `MB_SETFOREGROUND` alone takes the foreground when the box opens right after an input to Tally: in L2 the lock decided, not the flag. |
| K2-H: Tally is `Fg` at T0, and Tally stays in front | Not even right after an input does `MB_SETFOREGROUND` alone take it. |
| K2-K: the box in front | The new build's ledger box takes it early as well. |
| K4: Tally is in front at C+0; at C+0 and C+6 the ledger box is z 1 with `topmost` 1 and Tally holds the foreground | System-modal topmost is kept, and the helper does not take the foreground back. |
| K4: at C+0 the box is still in front | The click did not take the foreground for Tally: infer nothing about a second raise. |
| K4: Tally in front at C+0, the box in front at C+6 | A second raise: report it. |
| K4: the box below Tally | Topmost lost: report it. |
| The box in front at first, and Tally in front before any click into Tally | Something took the foreground back from the box: record the stamp. |
| K1-K and K2-K: one or two `box-fit:` lines, all `fits=true` | The ledger box is now measured, and fits. Record its rectangle (row 1780-H's L2 read 713,370,1228,740 before it was measured). |
| Any cell ends with `exit_code=77` | The fit check refused the box: record its `box-fit:` line whole. |
| `exit_code=0` with a 0-byte `.stdout` | The child showed no box (`box-fit: not per-monitor DPI aware`, or the measuring hook could not be set): record `<cell>.stderr` whole. |
| K3: the post's box in front within 3 s of `FIRST` in all four | No long gap: row 1780-H's H1 (2.4 s) is in that range. |
| K3: a longer gap before the box is in front | Record its length and the cell's timing. |
| Anything else | Record it whole and infer nothing. |

**FAIL of the row:**
- a key pressed;
- a Yes;
- any request to Tally other than C2's and C10's reads;
- Tally's screen changed beyond focus;
- a mark different at the closing MARKS read;
- a box or process left at the end (B5);
- B2's or B3's check not met.

## Evidence (stays on the lab PC except the files listed)
- RESULTS.md: one line per cell and step, with every stamp, `Fg`, read, foreground reading and `box-fit:` line.
- Each `<cell>.txt`, `<cell>.stderr`, `<cell>.stdout` and trace, whole, after the privacy search.
- Every `rect-*.txt` and `zorder-probe-k` output.
- The runner under `scripts/`.
- `notes.txt` in UTC time order.
- `SCREENSHOTS.txt`.
- `SHA256.txt` over the committed files, with sizes.
- Also record:
  - BUILD-K's pull request, head, CI run, artifact and exe sha256;
  - the Windows build, the Tally release and the screen's scale, read afresh;
  - one monitor or more.

Evidence goes on a new branch `lab/1780-ledger-box-keyboard-answers`. Send its head to the run coordinator before pushing; push only after the pre-push scan and the coordinator's yes.

## Not measured (say so in the results)
- The server path (`post_ledger_creates`), and the review and invoice-stop release boxes. Their child code is the same as the post's, but they are not shown here.
- Whether a key reaches the box: the foreground window is read, and no key is pressed. Which control inside the box has the focus is not read.
- More than one run of most cells; a second monitor; a scale change while a box is open; Windows 11; macOS; more than one sitting.
