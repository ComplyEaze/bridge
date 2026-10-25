# Results: 1879-BOXFIT row B11 on PR #1879 head 62bf7126 (26 Oct 2026). Both boxes fit, at exactly the predicted heights

Windows lab PC: 1920x1080 at 125 percent; work area 1920x1030 at (0,0) in physical pixels; the display setting was not changed. Synthetic text only; nothing was sent to Tally; both boxes were answered No by mouse and no key was pressed while a box was open. The background build job on this PC was paused for the row (no cargo, rustc or linker process running when it started) and resumed afterwards.

## Build and files
- The sheet is `lab/1879-box-fit-run-sheet` @ 41557543e30ff4c2479ad00b90cc0d77a2a03c04: `SHA256.txt` 13/13 OK; `make-probe-inputs.py` (`python -I`, new folder) regenerates all eight inputs byte for byte, including the new `probe-29` (af4bbcdc…) and `probe-33` (62d9bf28…); `run-fit.ps1 -DryRun` printed and opened nothing.
- The build is the one B1 to B7 used (`RESULTS-62bf7126.md`): `bridge_mcp.exe` 33,771,008 bytes, sha256 b87ac71ba980cb5937c204c1633aee564c5336c86b40539550411411e7ac1440, rechecked before the row. PR #1879's head was still 62bf7126.

## B11
`run-fit.ps1` output in `evidence-b11/`, paths replaced by placeholders (the window title's dash shows as `â€”`, the console code page).

| Case | Window (L,T,R,B), size | Predicted height | `box-fit:` line | `cmd exit=` | Elapsed | stdout | Screenshot | Verdict |
|---|---|---|---|---|---|---|---|---|
| probe-29, No | 598,190,1342,916, **744x726**; all off-screen flags False | about 726 | `box=(598,190,1342,916) work=(0,0,1920,1030) fits=true`, printed twice (126 B, 66467c6a…) | 1 | 19,002 ms (includes answer time) | 0 B | b11-probe-29-box.jpg (55,753 B, eab61f74…) | fits |
| probe-33, No | 598,152,1342,954, **744x802**; all off-screen flags False | about 802 | `box=(598,152,1342,954) work=(0,0,1920,1030) fits=true` (63 B, 65d26f6d…) | 1 | 16,865 ms (includes answer time) | 0 B | b11-probe-33-box.jpg (56,936 B, 3711e8f9…) | fits |

Both boxes came to the front of the other windows without a click. Screenshots stay on the lab PC.

**The model holds.** The straight line through 40 lines at 935 px and 44 lines at 1011 px (19 px a line plus 175 px) gives 726 px for 29 lines and 802 px for 33; both measured heights equal the line to the pixel. So on this screen and scale, a ten-payment batch shown as totals only (about 29 lines) is 726 px tall and fits with 304 px to spare. Every box measured on this build so far lies on that line: 29, 33, 40, 44, 48 (1087 px) and 60 lines (1315 px).

The `box-fit:` line was printed twice for probe-29 (as for B1) and once for probe-33; the cause is not established.

## Not run
The 1366x768 case: not run, no such screen at the lab.
