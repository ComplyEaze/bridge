# Results: the #1975 release box on Windows, 26 Oct 2026. The title, the default (No), the fit and the exit are as expected; the text was NOT checked line by line (no screenshot), so the row is PARTIAL on that one point

The sheet is `lab/1975-release-box-row` at 576ff7a7d1647a30361524d80723a1d4f1f371a8. Its `SHA256.txt` checks 4/4 OK, on both sha256 and bytes. No Tally step: nothing was sent to Tally or read from it, and no company was loaded or named. **Yes was not pressed.**

## Build (recorded before step 1)
- PR #1975's head when the row started: **af0efdb511420a696314daaffb92ae1477cddf88**, state open. It equals the head the sheet names.
- A clean detached checkout of exactly that commit, with `git status --porcelain` empty. rustc and cargo 1.96.0.
- `cargo build --locked --release --manifest-path src-tauri/Cargo.toml -p bridge --bin bridge_mcp`: exit 0, 1,933 s.
- `bridge_mcp.exe`: 34,083,328 bytes, sha256 **7c546a17f949916ed4d899774b2f3e066cc437169f9cafa78a0c84b0bb9c2559**. It was copied to a new folder; the copy equals the original and was re-hashed before use. It was not run before step 1.
- Details are in `build-SUMMARY.txt`. That file also records where the source sets the title and the buttons: native `MessageBoxW` with `MB_YESNOCANCEL | MB_DEFBUTTON2 | MB_ICONWARNING | MB_SETFOREGROUND | MB_TOPMOST`, inside the box-fit guard.

## Step 0
- Screen 1920x1080, scale 125% (120 dpi), one monitor. The work area is 1920x1030 at (0,0).
- `inputs\` was copied into a new folder, and the script was run from that folder, never from the branch's.
- `release-probe.txt` sha256 bcee8bcc256d33d2238fad9044ad19e4e1def019da289f3f03f97576198bea81, equal to the script's expected hash.
- Four other `bridge_mcp.exe` processes were already running, all children of the desktop assistant app: its two Bridge connectors, each started twice. None had `--confirm-release`, so the script's child search did not match them.

## Step 1 (dry run)
Deviation: the sheet's command line passes `-ExecutionPolicy Bypass`, and this session's tool refused that flag. The script was therefore run **unchanged** with PowerShell's call operator, under the PC's existing policy (CurrentUser RemoteSigned; a local script).

The output is in `step1-dryrun.txt`: the case, the exe's sha256, the screen and the command, then "DRY RUN: nothing opened". Nothing opened, and the output folder stayed empty. **As expected.**

## Step 2 (the box)
The box opened through the child mode. It was answered by the owner at the PC, with the mouse.

The lab asked for screen access to the box's process, and the owner declined it. So the lab took **no screenshot of the box** and pressed no key. The owner answered the lab's questions in its chat; the recorded words are the owner's choices, verbatim:

| Record | Expected | Recorded |
|---|---|---|
| Window title (read by the script with GetWindowText) | "ComplyEaze Bridge — release the invoice stop for this company?" | **'ComplyEaze Bridge — release the invoice stop for this company?'**, as expected |
| Buttons and the default | Yes, No, Cancel; No the default | default: owner's answer **"No"**. The three buttons were named in the question; the owner said nothing to contradict them. |
| The whole box on the screen | title bar, every line, the buttons | owner: **"All visible"**. The script found all four off-screen flags False. |
| Came to the front by itself | (recorded) | owner: **"Yes, on top"** |
| Text line by line against `release-probe.txt` | the same 13 lines | **NOT RECORDED**: there is no screenshot, and the owner was not asked to read the lines |
| Answer | No, by mouse | owner: **"Clicked No"**, with the mouse |

## Step 3 (the script's output, whole in `step2-run.txt`)
- **Window line:** rect (L,T,R,B) = 686,266,1252,840, size 566x574. `top_off_screen=False bottom_below_workarea=False left_off=False right_off=False`.
- **Exit:** `cmd exit=1`, after 1,106,853 ms from start. The box stayed open while the owner was asked. **As expected: declined.**
- **stdout** (`out/release-probe.stdout.txt`): **0 bytes**, sha256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855. No token was printed, so Yes was not pressed.
- **stderr** (`out/release-probe.stderr.txt`): 127 bytes, sha256 8958d8e5850d58f44b45fea7f95ec5a459f37c054adb031ffd1456bbf5d78cc0. It holds two box-fit lines:
  - `box=(686,266,1252,840) work=(0,0,1920,1030) fits=true`;
  - `box=(1162,119,1728,693) work=(0,0,1920,1030) fits=true`.

  The second line shows the same 566x574 box at a second position, fit-checked again. This is recorded, not a finding. Whether the owner moved the box, and what triggers the second check, was not recorded.
- Afterwards, no `--confirm-release` process was left running.

## Outcome
| | |
|---|---|
| Title | as expected |
| Default button | No (owner's word), as expected; the source sets MB_DEFBUTTON2 |
| Fit | wholly on the screen (script and owner); product fit check true twice |
| Exit | 1, stdout empty |
| Text line by line | **not recorded**: the one point not met. A rerun with a screenshot, or with the owner reading the lines, would close it |

No FINDING for #1975.

## In this commit
- `RESULTS.md`, `notes.txt` (UTC times).
- `step1-dryrun.txt`, `step2-run.txt`: the script's whole output. The second was written as UTF-16 by PowerShell and is stored here as UTF-8.
- `out/`: the stdout and stderr files, exactly as written.
- `build-SUMMARY.txt`: the build record.
- `SHA256.txt`.

Local folder paths are replaced by placeholders (`<item folder>`, `<build folder>`, `<lab folder>`, `<clean checkout>`, `<target dir>`) in four files. In `build-SUMMARY.txt`, the requesting session's name is also replaced, and in `notes.txt` the assistant app's product name. The sha256 of those files as kept on the lab PC:

| File | sha256 |
|---|---|
| `notes.txt` | bebd21ca1168e1eca1293a5a5d428ebcc53b783e5c6bf90ce6e757d906770edd |
| `step1-dryrun.txt` | 8f0ab709db621d546d7eb280a3dd1faa3cc0b6511e102488be1c578e53061ef6 |
| `step2-run.txt` | d41713e6dcce9907e3d2395fe495492d859482824b705efcc9f38e80e5723220 |
| `build-SUMMARY.txt` | aa79d40077a8eaa3ff54670cc6a887a2fe2b469a7b8add7a2bc913c527394d76 |

No screenshots exist for this row.
