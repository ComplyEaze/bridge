# Results: Windows lab item 1832-CLIP, DPI awareness of the exe and the approval box taller than the screen (25 Oct 2026)

Sheet `lab/1832-box-clip-probe-run-sheet` at ddfe4da9 (SHA256.txt: 12 entries, all OK). No Tally used, nothing sent to it, no display change (the PC's own 1920x1080 at 125%, work area 1920x1030), no key pressed while a box was open. The owner approved, in the lab's chat, opening the six probe boxes and the substitute DPI read below.

Build: the staged artifact 12553782422 of scheduled CI run 40077056130 (master 2306d86d), zip sha256 74ae2daa851397b71cb59e80373d9478a2da72c74398108e33e34ea83e56eb1e; `bridge_mcp.exe` sha256 92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524 (33,385,472 bytes), the same file the 1832-BOX item used. Nothing installed.

## W1a: the exe's manifest (`dump-manifest.ps1`, ran as written)
- One manifest found (332 characters), in full:
  `<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">  <dependency>  <dependentAssembly>  <assemblyIdentity  type="win32"  name="Microsoft.Windows.Common-Controls"  version="6.0.0.0"  processorArchitecture="*"  publicKeyToken="6595b64144ccf1df"  language="*"  />  </dependentAssembly>  </dependency>  </assembly>`
- Occurrences: dpiAware 0, dpiAwareness 0, PerMonitor 0, longPathAware 0, activeCodePage 0, Microsoft.Windows.Common-Controls 1, supportedOS 0; dpiAware as UTF-16 0.
- RESULT line: "the file DOES NOT CONTAIN a dpiAware / dpiAwareness setting."

## W1b: the running box's DPI awareness (substitute for Task Manager, with the owner's OK)
Task Manager usually runs elevated, and the lab's desktop control cannot operate an elevated window (not tried here). The same fact was read with a lab helper. It calls `GetWindowDpiAwarenessContext` on each visible window of the probe process (the `bridge_mcp.exe` with `--confirm-journal`), then `GetAwarenessFromDpiAwarenessContext` and `GetDpiForWindow`. Result for every one of the six boxes: **awareness 0 = Unaware**, window DPI **96**. At 125% Windows therefore bitmap-scales the box; the rectangles below are physical pixels (the script and the helper both call SetProcessDPIAware).

## W2: `run-probe.ps1` (ran as written; its rectangle finder worked)
| Case (displayed lines) | Rectangle L,T,R,B (size) | Flags: top off / bottom below work area / left / right | Text | Buttons | Closed by | cmd exit |
|---|---|---|---|---|---|---|
| probe-40 (3 + P001-P037) | 588,149,1353,958 (765x809) | F / F / F / F | all visible, last P037 | Yes/No/Cancel visible, reachable | No (mouse) | 1 |
| probe-44 (3 + P001-P041) | 588,116,1353,990 (765x874) | F / F / F / F | all visible, last P041 | visible, reachable | No | 1 |
| probe-48 (3 + P001-P045) | 588,84,1353,1023 (765x939) | F / F / F / F | all visible, last P045 | visible, reachable (bottom edge 7 px above the work-area bottom) | No | 1 |
| probe-60 (3 + P001-P057) | 588,0,1353,1134 (765x1134) | F / **T** / F / F | all text visible down to P057 at the screen's bottom edge; the box covers the taskbar | **not visible**: below the bottom of the screen, unreachable | X (mouse) | 1 |
| probe-80 (3 + P001-P077) | 588,0,1353,1459 (765x1459) | F / **T** / F / F | cut by the screen edge after P059 (P060 partly visible); P061-P077 off screen | not visible, unreachable | X | 1 |
| probe-120 (3 + P001-P117) | 610,0,1330,2109 (720x2109) | F / **T** / F / F | cut by the screen edge after P059 (P060 partly visible) | not visible, unreachable | X | 1 |

- (d) Windows did NOT shrink the font, clip the text inside the box, or re-lay the box out to fit. Once the text is taller than the work area, it pins the box's top at y=0, lets the box grow past the work area and past the bottom of the screen (over the taskbar), and the buttons go off screen. The title bar and its X stay on screen and closing by X answered the box like Cancel (exit 1). The width stayed 765 for probes 40-80; probe-120 is 720 wide because its input lines are shorter (61/41 characters, against 101/53 in the others; checked in the input files), not because Windows narrowed it.
- The sheet predicted that probe-44 would just fit and probe-48 would not. **probe-48 fits** (bottom 1023 of 1030); the boundary on this PC lies between 48 and 60 displayed lines.
- No probe needed its taskkill; no second box, no other box, Tally (at Select Company, no company loaded) never came in front.

## Evidence kept on the lab PC (not committed)
| sha256 | bytes | file |
|---|---|---|
| 4df30cad005b2a34b29cca1a21e3a6e90c8c4d0fff73da6fb8692ea8f2a1fd3a | 1,348 | W1a output |
| dd182af1c1d600596ed6c1de6803d21fee197803df4f687a1bda351a66327fc9 | 390,817 | whole-screen shot, probe-40 |
| a67d4deb4141541f343e7583e735fe1110242564d6465fb189e1f5f9b0f81519 | 524,566 | probe-44 |
| 5b667543f27334dce6bb071562d40abb3704b849b5d5e5e6b715030f97e8527a | 516,402 | probe-48 |
| 1c4b4fadb7b8d924073cd646bc274632bc5c826b1d470de91eb67276b02fabf4 | 528,460 | probe-60 |
| 3766752b26a2d2f64bfc715efbeb5882312daacd4fb205c7aa1b9efe842c99c3 | 553,878 | probe-80 |
| b02514a844dd7ea72152c5f0df10b8d2f09bdc4b33dd7cca8f287864de7f8d8c | 538,877 | probe-120 |

Counted writes: none. Display unchanged. Not covered: other scales, a second monitor, Tally in front, the real parent path.
