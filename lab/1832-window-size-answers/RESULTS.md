# Results: Windows lab item 1832-BOX, approval box size at the PC's own display setting (25 Oct 2026)

Sheet `lab/1832-window-size-run-sheet` at bd33d8cc (SHA256.txt checked, 19/19 OK). No Tally used and nothing sent to it; every box was opened through the executable's own `--confirm-journal` child mode with the sheet's synthetic or real-preview stdin, and answered **No by mouse** (exit code 1 every time). Nothing was typed while a box was open.

## Build (the requester's addendum: current master build instead of the sheet's older bundle)
- Scheduled Bridge CI run 40077056130 on master 2306d86dfd4c86a3c5f6c19ea9ddff6818c460fb (conclusion success), artifact `unsigned-bundle-smoke-windows-latest`, id **12553782422**, 60,880,528 bytes.
- Zip sha256 **74ae2daa851397b71cb59e80373d9478a2da72c74398108e33e34ea83e56eb1e** = the artifact digest.
- `bridge-tally.mcpb` sha256 efcdb419389fcfe0a0f6d52857c614c364669e83ea4a2f80c94f193300e77111 = `archive_sha256` in the artifact's `mcpb-smoke.json`.
- `bridge_mcp.exe` sha256 **92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524** = `binary_sha256` in `mcpb-smoke.json` (server_version 0.6.0). Staged only, nothing installed.

## Display settings
- The PC's setting (original, and the only one measured): **1920x1080 at 125%** (S3). Screen 1920x1080 physical, work area 1920x1030 at (0,0), taskbar at the bottom. One monitor.
- S1 (1366x768 100%), S2 (1366x768 125%) and S4 (1920x1080 150%) were **not run**: the owner chose to measure the current setting only, so no display setting was changed and nothing needed restoring.

## S3 results (rectangles DPI-aware, physical pixels)
| Case | Title | Lines shown | Last line visible | Yes/No/Cancel visible | Box rectangle (L,T,R,B), size | Inside work area | Result |
|---|---|---|---|---|---|---|---|
| win-real-1 | ComplyEaze Bridge — post this voucher? | all 27 (24-line preview + 3-line answer text; long lines wrap) | yes: `Narration: "Synthetic test only"` | yes | 704,198,1238,909 (534x711) | yes | PASS |
| win-real-10-40 | ComplyEaze Bridge — post 10 vouchers? | all 43 (40-line preview + 3) | yes: `After a timeout, reconcile this batch; do not rebuild or resend it.` (wraps to a second row) | yes | 704,54,1238,1025 (534x971) | yes, 5 px above the work-area bottom | PASS |
| win-10 (worst case) | ComplyEaze Bridge — post 10 vouchers? | all 43 (L01-L40 + 3) | yes: L40 | yes | 591,126,1350,984 (759x858) | yes | PASS |
| win-1 (worst case) | ComplyEaze Bridge — post this voucher? | all 27 (L01-L24 + 3; the 100-character lines wrap) | yes: L24 | yes | 705,149,1234,958 (529x809) | yes | PASS |

- First line of every box: "Press Y (or click Yes) to post this voucher/these 10 vouchers into Tally. Press N (or click No) to cancel. Nothing is posted until you answer."
- win-real-1 was opened twice (both answered No): the first time the rectangle was not measured, the second time it was.
- The real 43-line box (win-real-10-40) is 971 px tall with 5 px to spare in a 1,030 px work area at this setting. Any setting with a smaller work area (the not-run S1, S2, S4) is the open question.

## Script notes (`run-box.ps1`)
- The dry run and every case ran. The script's own rectangle finder printed `rect(L,T,R,B)=not found in 10 s` in all five runs, although the box was open with exactly the expected title. A likely cause: the em dash in the title passed into its background job does not arrive intact. The rectangles above come from a lab helper that enumerates the visible top-level windows of the box process (the one `bridge_mcp.exe` with `--confirm-journal`) with the same DPI awareness, and found exactly one window, with the expected title.
- The script reported `exit=1` for every case, as the sheet expects for No.

## Evidence kept on the lab PC (not committed): full-screen captures while each box was open
| sha256 | bytes | file |
|---|---|---|
| 6fa8dc0b16c13c2562f758b69bd6a295de30fbb2977af8cbe4cc55f37dd534a7 | 525,026 | shot-s3-win-real-1.png |
| 4e08417b1e1492616e2bd35973eda2928f10f109c727a8b4f6ca147c8a8ea06e | 531,177 | shot-s3-win-real-1-b.png |
| 8aa351692b339ed3025e45fa2e98a9e82ec79bff819b0796746cee29c0a89531 | 634,499 | shot-s3-win-real-10-40.png |
| e29f6648bbbf941f0448ce10af12e5fd63fe82b25944cafbb921b70f2e16219c | 837,733 | shot-s3-win-10.png |
| 27fcc6edfcb55df82bc31cecf44d776d91ce1b518dbcab9bbf46adf0d720bd56 | 618,275 | shot-s3-win-1.png |

Counted writes: none. Not covered: S1, S2 and S4; a second monitor; non-English Windows; the real parent path (needs Tally).
