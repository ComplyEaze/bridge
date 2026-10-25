# Results: 1879-BOXFIT re-run on PR #1879 head 62bf7126 (26 Oct 2026). B1, B2, B4, B5, B6, B7 PASS; B3 a FINDING against the prediction (48 lines no longer fit)

Windows lab PC: 1920x1080 at 125 percent; work area 1920x1030 at (0,0) in physical pixels. The display setting was not changed. Synthetic text only; nothing was sent to Tally, and no key was pressed while a box was open (every answer was a mouse click). The sheet is `lab/1879-box-fit-run-sheet` @ 0a483e316c76acf38eee13e57a38e585585e7a21.

## Build and files
- **PR:** #1879; its head is `62bf7126d1c2c88d5a5fd905f4039a1f8733fc36` both on the sheet's line and as the PR's head when the item started (PR open). Run 40189393912; artifact 12600475049 `unsigned-bundle-smoke-windows-latest`, expires 2026-11-01. The owner approved the download.
- **Hashes, every one equal to the sheet's line:**

  | File | Bytes | sha256 |
  |---|---|---|
  | artifact zip | 61,260,254 | 8414d22345473c1d3f2a8ef66997b8a7b7372e59e820333afd28c6c455e0c285 |
  | `bridge-tally.mcpb` | 16,393,197 | 5fbd8e2bea6648cd92fd34d608632e9d990ca0f1abac1958ac3de0abdb9a2072 (equal to `archive_sha256` in `mcpb-smoke.json`) |
  | `bridge_mcp.exe` | 33,771,008 | b87ac71ba980cb5937c204c1633aee564c5336c86b40539550411411e7ac1440 (equal to `binary_sha256`) |

  Server version 0.6.0. The exe was unpacked from a copy of the `.mcpb` and staged only; nothing was installed.
- **Sheet files:** `SHA256.txt` 11/11 OK. `make-probe-inputs.py` (`python -I`, new folder) regenerates all six inputs byte for byte.
- **Dry run:** `run-fit.ps1 -DryRun` printed the case, the exe hash, the screen and work area and the command, and opened nothing.
- **Who clicks:** the lab, by mouse, with desktop-control access to the box's process granted by the owner, so the box appears in the lab's screenshots (kept on the lab PC, not pushed).

## Rows
`run-fit.ps1` output for B1, B2, B3 and B7 is in `evidence-62bf7126/`, paths replaced by placeholders (`<staged build>`, `inputs`, `<new run folder>`); the window title's dash shows as `â€”` because the script's console output was captured in the console code page. B3 to B6 also ran through the sheet's manual fallback (`evidence-62bf7126/b3-b6-manual-fallback.txt`), see "The script on a fast exit".

| Row | Case, answer | Window (L,T,R,B), size | Front without a click | `cmd exit=` | Elapsed | stdout | `box-fit:` line (stderr) | Screenshot | Verdict |
|---|---|---|---|---|---|---|---|---|---|
| B1 | probe-44, Yes | 598,15,1342,1026, 744x1011; all four off-screen flags False | **Yes**: in front of Tally at the first screenshot (about 4 s after start; nothing clicked first); no taskbar flash noticed | 0 | 44,927 ms (includes the time to look and click) | 58 B `bridge-post-approved:0f9d3c1e-6b7a-4c58-9a21-5d3e8b1c7f40` (= the input's first line) + line end; sha256 3130925b… | `box=(598,15,1342,1026) work=(0,0,1920,1030) fits=true`, printed **twice** (126 B) | b1-probe-44-box.jpg | PASS |
| B2 | probe-44, No | 598,15,1342,1026, 744x1011 | Yes, as B1 | 1 | 13,399 ms | 0 B | same line, once (63 B) | b2-probe-44-box.jpg | PASS |
| B3 | probe-48, Yes (never shown) | no window line (see below) | not seen | 77 (fallback) | 154 ms (fallback) | 0 B | `box=(598,0,1342,1087) work=(0,0,1920,1030) fits=false` | none: closed before a screenshot could be taken | **FINDING** (prediction was "fits") |
| B4 | probe-60, nothing clicked | not found | not seen | 77 | 264 ms | 0 B | `box=(598,0,1342,1315) … fits=false` | none | PASS |
| B5 | probe-80, nothing clicked | not found | not seen | 77 | 246 ms | 0 B | `box=(598,0,1342,1695) … fits=false` | none | PASS |
| B6 | probe-120, nothing clicked | not found | not seen | 77 | 249 ms | 0 B | `box=(618,0,1320,2455) … fits=false` | none | PASS |
| B7 | probe-40, Yes | 598,86,1342,1021, 744x935 | Yes, as B1 | 0 | 20,210 ms (includes answer time) | 58 B, the same token; sha256 3130925b… | `box=(598,86,1342,1021) work=(0,0,1920,1030) fits=true` | b7-probe-40-box.jpg | PASS |

stderr hashes: B1 0de00887…, B2 1d81533a…, B3 a03a4e41… (the script run and the fallback run are byte-identical), B4 c8e06543…, B5 847f5dcd…, B6 f768d2b0…, B7 c2c1f689….

## What this shows (the sheet's four things)
1. **The hook fires.** A box that fits is answered normally: Yes prints the token (B1, B7), No ends 1 (B2).
2. **It measures the final size and position.** The rectangle the product printed equals the one `run-fit.ps1` read from the live window (B1, B2, B7), so the hook saw the laid-out box.
3. **A box taller than the screen is cancelled at once.** B4 to B6 ended with 77 in about a quarter of a second, with no token; no box was left open and none was seen on screen.
4. **The units agree.** The product's `box-fit:` rectangle and the script's window rectangle (physical pixels, after `SetProcessDPIAware`) are identical, and the work area is the same 1920x1030 in both. The earlier build's B1 (a fitting box cancelled) is not reproduced.

## B3: 48 lines no longer fit at 125 percent
The sheet predicted that probe-48 fits, from the earlier clipping item, where the DPI-unaware exe drew 48 lines with its bottom at 1023. This build is per-monitor DPI aware, so the box is drawn at full physical size: 44 lines are 1011 px tall (bottom at 1026, 4 px inside the work area), 48 lines would be 1087 px tall and do not fit. The product placed the box at top 0 and cancelled it with 77 before it was seen. That is the gate working on a box that really is too tall, not a units disagreement: the rectangle itself is taller than the work area.

The heights recorded here answer B10 (re-measure the line caps) without a display change: 40 lines 935 px, 44 lines 1011 px, 48 lines 1087 px, 60 lines 1315 px (clamped boxes, top 0). On this display the largest probe that fits is 44 lines; 48 does not.

## The script on a fast exit
When the box is cancelled within a few hundred milliseconds, the child has exited before `run-fit.ps1` finds it, so the script prints `child process not found in 15 s` and returns without the exit status (B3's script run). The stdout and stderr files were still written (B3's at 00:04:28.960 and 00:04:29.050 IST). B3 was therefore run once more, and B4 to B6 only, through the sheet's manual fallback (`cmd`, the same redirections, exit status from `$LASTEXITCODE`, elapsed from a stopwatch around the call). The box was not seen in any of these runs.

## Other observations
- The `box-fit:` line was printed twice in B1 (the same numbers) and once in every other row. One possibility: the box was activated a second time when the lab's screenshot tool or its access prompt took and returned the focus. Not established.
- Every box opened in front of the other windows without a click, though `cmd` was started from a hidden background shell. On the earlier build it stayed behind other windows.

## Not run
- **B8 and B9:** not run, because a display change was not allowed and the PC has one monitor.
- **B10:** not run as a separate row; the heights above come from B1 to B7.

Counted writes: none. Tally was not touched during these rows; one lab company was loaded during B1, and no company was loaded from B2 on (another item's close).
