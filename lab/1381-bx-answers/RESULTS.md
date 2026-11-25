# Row 1381-BX: results

Sheet `lab/1381-bx-sheet` at 92f48553697a352167dae8d1ae8e3cd99d318627 (base master 1ecd2c70de74bf4509bf85790496ccff8f088722). Run 2026-11-25, 19:41Z to 19:47Z, in one sitting, on the Windows lab PC. The build: master 6c9e6b33b531d0bd23bf6c4c62ba0d01a0aa4a1d's Windows bundle (the CI run and artifact `BUILD.txt` names), exe sha256=71193ce40dabd7cb2b916a9ca5eb25b7e16aeb27cf9584cf783c7c2667dc2e05, 35,942,400 B, staged on GRANTED PATH for the five arms and moved out at the close. No Tally request, no company loaded, no key or click in Tally; Tally (pid 23160, Educational Mode) stayed at Select Company with no company loaded throughout.

## Each arm's ARM line and exit

| arm | mode, answer | ARM line | arm exit | child exit | standard output | box-fit lines |
|---|---|---|---|---|---|---|
| LN | masters, No | `ARM LN AS THE CODE SAYS; recorded lines: 6 of 6 PASS` | 0 | 1 | 0 bytes | one, `box=(688,342,1252,764) work=(0,0,1920,1030) fits=true` |
| SN | stock, No | `ARM SN AS THE CODE SAYS; recorded lines: 6 of 6 PASS` | 0 | 1 | 0 bytes | two, both `box=(686,152,1252,954) work=(0,0,1920,1030) fits=true` |
| SC | stock, Cancel | `ARM SC AS THE CODE SAYS; recorded lines: 6 of 6 PASS` | 0 | 1 | 0 bytes | one, `box=(686,152,1252,954) work=(0,0,1920,1030) fits=true` |
| SY | stock, Yes | `ARM SY AS THE CODE SAYS; recorded lines: 6 of 6 PASS` | 0 | 0 | 62 bytes: `bridge-create-stock-item:` and the arm's own nonce, one LF | one, `box=(686,152,1252,954) work=(0,0,1920,1030) fits=true` |
| ST | stock (too tall), none | `ARM ST AS THE CODE SAYS` | 0 | 77 | 0 bytes | one, `box=(620,0,1320,2512) work=(0,0,1920,1030) fits=false` |

Every answered arm: the guard passed (the box the child's, `boxstate.ps1` exit 0 within 30 s, one `WINDOW` line, `iconic=False`, `visible=True`, `topmost=True`, `boxstate.ps1`'s rect equal to `box.ps1`'s, the box inside the work area, the button inside the box, the box first in the z-order), `boxcheck.py` `ALL PASS`, `CLICKED` once with the arm's answer, the box gone after the click, and the replay `REPLAY ALL PASS`, exit 0. Seen to click: LN 1.9 s, SN 2.4 s, SC 1.8 s, SY 1.4 s. After each arm `tasklist` found no `bridge_mcp.exe`.

## Every FAIL line

One, recorded and not deciding: ST's `FAIL W6 box.ps1 saw a box (BOX SEEN; box.ps1 exit 3: 3 none seen, 4 gone before any click)`. `box.ps1` logged `NO BOX seen within 30 s`: the child had started at 19:46:16.508Z and ended with exit 77 at 19:46:16.821Z, 0.31 s later, so the fit guard cancelled the box before `box.ps1`'s watch read it. No other FAIL line in any arm, replay or self-check.

## The W lines

| arm | W1 not minimised | W2 in front | W3 focus on No | W4 work area | W5 first box-fit box = box.ps1's | W6 box seen | W7 not opened minimised |
|---|---|---|---|---|---|---|---|
| LN | PASS | PASS | PASS | PASS | PASS (688,342,1252,764) | n/a | PASS |
| SN | PASS | PASS | PASS | PASS | PASS (686,152,1252,954) | n/a | PASS |
| SC | PASS | PASS | PASS | PASS | PASS (686,152,1252,954) | n/a | PASS |
| SY | PASS | PASS | PASS | PASS | PASS (686,152,1252,954) | n/a | PASS |
| ST | n/a | n/a | n/a | PASS | n/a | FAIL (exit 3, none seen) | PASS |

## The display facts

`screen.ps1` at step 0 and in each arm, the same each time: Windows 10 Home Single Language, display version 22H2, build 19045.6466; one monitor, primary, 1920x1080; work area 0,0,1920,1030; DPI 120, scale 125%; the reader per-monitor (v2) DPI aware. As Display facts expects.

## The questions

- **Q1: yes, for this PC.** SN, SC and SY each: title exactly `ComplyEaze Bridge — create one stock item?`, three buttons Yes, No and Cancel, exactly one text child, its text equal to the input line for line (32 lines read, 32 fed: the answer lines, a blank line and the 29-line preview), text and buttons inside the box, buttons below the text; the box 686,152,1252,954, wholly inside the work area 0,0,1920,1030, topmost, first in the z-order; every `box-fit` line `fits=true`; W3 the focus on No in the box's process in all three, so No the default. By eye (the box copies `SN-open.png`, `SC-open.png` and `SY-open.png`, read with the operator's file tool, no viewer opened): every line wholly visible, none cut at the box's edge, no scroll bar, No outlined as the default button. Confidence: verified on this PC, one display, one scale.
- **Q2: yes.** SN and SC: 0 bytes, exit 1. SY: exactly `bridge-create-stock-item:` and the arm's nonce (the first line of `local/run/SY-in.txt`) and one LF, 62 bytes, exit 0. Confidence: verified.
- **Q3: yes.** LN opened the ledger box titled `ComplyEaze Bridge — create one ledger?`, its text line for line (12 lines read, 12 fed), `fits=true`, answered No: 0 bytes, exit 1. W5: the box 688,342,1252,764, equal to lab 1780-K's 688,342,1252,764 in the same work area 0,0,1920,1030, so the input's dropped final LF did not change the box's height. Confidence: verified.
- **Q4: yes.** ST: no click, exit 77 (the fit guard's), 0 bytes, the one error line `box-fit: box=(620,0,1320,2512) work=(0,0,1920,1030) fits=false`. The box measured 2,512 px high, which is the sheet's estimate (175 + 123 x 19 = 2,512), about 2.4 times the work area's 1,030; it began at the top of the screen and ran 1,482 px below the work area. W6: `box.ps1` saw no box (the child ended 0.31 s after it started). Confidence: verified for this text at this display.

## Observations, not findings

- SN's `box.ps1` read the foreground before the click as a browser window (title kept as its length and sha256 only), where its first read had the box in front (W2 PASS); the child logged two `box-fit` lines, both the same box and `fits=true`, so the box was activated twice. The guard had read the box first in the z-order and topmost, the click landed and the box was gone after it. Nobody was at the PC; the browser was already open on the desktop. The other three answered arms had the box in front before the click and one `box-fit` line.
- The stock preview shows the literal text `&#4; Applicable`, `&#4; Any` and `&#4; Not Applicable`, as the input holds them (the test literal at 48a64965); `boxcheck.py` found it equal line for line.
- In `SY-open.png` the Cancel button shows its hover colour: the mouse was still where SC's click left it. No is outlined as the default, as in SN and SC.
- The box copies are of the whole screen, so they show other windows on the desktop; they stay on the lab PC (below).

## Steps and EXPECT

| step | EXPECT | met |
|---|---|---|
| 0 | `made`, fetch exit 0, copy `exit 0, 0`; `check_sheet.py --fresh` `OK: local/ holds only step 0's own records` and three `OK`, exit 0; the pull request's ref fetched, both commits `present`; the test file exit 0; the fill check nothing above exit 0; eight self-checks (47 of 47, 17 of 17, 26 of 26 `ALL CASES RIGHT`, three `PASS` and `EQUAL 1494 bytes … c94ecf5c…`, four `PASS` and `EQUAL 2568 bytes … 4190800d…`, parse exit 0, four names each 0, three `OK`); the ledger blob exit 0 and three `PASS`, `EQUAL 475 bytes … 1ba480b8…`; `build.py check` `0 field(s) to fill: filled`; the aside exe `: OK`; no `bridge_mcp.exe`; tally.exe one line, pid 23160; the screenshot with no dialog and Tally at Select Company with no company loaded; GRANTED PATH among the grants; the display as above | met (deviation 1) |
| 1 | the download `exit 0` alone (60,688,313 B); verify: zip 7e535782…, mcpb e27b8c87… equal to `archive_sha256`, exe 71193ce4… equal to `binary_sha256`, `VERIFIED`, exit 0; stage: granted path before 1e3dd540… (ASIDE SHA256), `STAGED`, exit 0, `copies.txt` three lines | met |
| 2 | each arm `ARM <arm> AS THE CODE SAYS`, `arm exit 0`; `REPLAY ALL PASS`, exit 0 (not for ST); no `bridge_mcp.exe`; a screenshot with no box and no dialog, Tally as at step 0; the by-eye read | met (deviation 2) |
| 3 | on the GO's RESTORE: `RESTORED`, exit 0, `copies.txt` five lines (extract, move-aside, copy-in, move-out, move-back); no `bridge_mcp.exe`; `check_sheet.py` exit 0; the listing (268 files under `local/`); the screenshot: no box, no dialog, Tally as at step 0 | met; GRANTED PATH read back as 1e3dd540…, row 2569-R's exe |

## Deviations

1. At step 0 the grant list was read before the screenshot; the sheet names the screenshot first. Both were made with no `bridge_mcp.exe` running and nothing changed between them.
2. Each arm's wait was the shell tool's own notice that the background run had ended, then the wait line once, whose last line was `arm exit 0`; the wait line was not typed in a 5 s loop. No screen-tool act was made from any start line to its `arm exit` line.
3. The screen tool reported a Settings window and two other windows masked as not granted at every screenshot; none was a dialog over the screen.

## Not covered

As the sheet's Not covered: other displays, scales and work areas; a box too wide; the parent's nonce check, 120 s limit and reading of exit 77; the Y and N keys, Escape and the close box; the Mac alert.

## The record

`evidence/` (every helper's output with its exit code, `copies.txt`, `s3-listing.txt` and the notes), `box/` without its `.png` files, `run/` (each arm's input and its hash, the screen read, `startc.ps1`'s lines, the child's standard output and error output). Kept on the lab PC, listed in `kept-local.txt` by sha256: the box copies (`.png`), `local/private/` (the foreground window titles), the screenshots, the download, the exes, the test file and the ledger blob from the clone, the self-tests' scratch and the grant list with its whole paths. Text files had lab and clone paths and the gate's desk words replaced; `substitutions.txt` counts every replacement.
