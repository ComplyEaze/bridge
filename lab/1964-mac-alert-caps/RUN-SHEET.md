# macOS row 1964-M (DRAFT, not run): how much text the macOS approval alert holds, measured, then the cap at it (Part of #1780)

Runs only after the owner's OK in chat for this sitting. WRITES: NONE. No server runs and no Tally is reached. Each cell starts the dialog child by hand with a synthetic input. The child prints a token only if its positive button is clicked; every cell is answered Cancel, and with no parent reading it nothing can post.

## Why
- **What draws the alert.** Every macOS approval box is `rfd`'s parentless alert: a `CFUserNotificationDisplayAlert` panel, drawn by the system process UserNotificationCenter. It wraps each line of its text itself and has no scroll. A text taller than the screen leaves its buttons off the screen.
- **The change** (branch `fix/1964-mac-alert-caps`) keeps that panel. Before any of the five boxes opens on a Mac, the dialog child counts the wrapped lines of its text:
  - each line counts once for every `MAC_WRAP_CHARS` characters, and an empty line counts once;
  - when the count is over `MAC_WRAPPED_LINES`, the child shows nothing and exits 77, the status the parent reads as a box that did not fit, as on Windows.
- **The two figures (31 and 37) are estimates.** They rest on three unverified guesses about the panel: its 11-point font, its 228-point text width and its 260 points of title, icon and buttons. The guesses aim at a 13.3-inch Mac at 1440 x 900, whose visible height is about 792 points with an 84-point Dock (the Dock measured on this row's Mac).
- **What this row decides:**
  - **Part A** measures the panel and fixes the two figures. It runs on the build that ships today's panel, before the change is built. On its own it answers whether the change can ship at all: whether the smallest one-voucher post still fits under the measured cap (cell A7). If it does not, the row stops after Part A.
  - **Part B** checks the change's build at the cap.

## Who acts
- **The owner** is at the Mac, reads each panel, and answers each one Cancel by mouse.
- **The lab:**
  - starts each cell's command after the owner's OK;
  - runs the bounds read while the panel is open;
  - reads the exit and the output.

  The lab never presses a key or clicks while a panel exists.
- **The run coordinator** gives the go-ahead and any STOP ruling.

## Builds
- **BUILD-A,** for Part A: the master build that ships `rfd`'s panel unchanged.
  - Either the 0.7.0 macOS package once it is published, or the `unsigned-bundle-smoke-macos-latest` artifact of a master CI run.
  - Record the run, the head, the artifact's digest and the sha256 of `bin/aarch64-apple-darwin/bridge_mcp`.
- **BUILD-B,** for Part B: the same artifact of the CI run on `fix/1964-mac-alert-caps`'s head, recorded the same way.
- **Fallback, for either build:** if the system refuses an unsigned binary, use a release build of the same head made on this Mac. Record the commit and the binary's sha256, and say which was used.

## Inputs and tools (`requests/`, synthetic: no company, no amount)
Each input is a nonce line (`00000000-0000-4000-8000-000000000001`), a count line, then the text. The child shows that text as given.

| file | count | text |
|---|---|---|
| `a1-one-line.txt` | 1 | one short line |
| `a2-ten-lines.txt` | 1 | 10 short lines |
| `a3-twenty-lines.txt` | 1 | 20 short lines |
| `a4a` to `a4e-capitals-<n>.txt` | 1 | 10 lines of capitals, each 26, 31, 36, 41 or 46 characters |
| `a5-prose-100.txt` | 1 | 10 lines of prose, each 100 characters |
| `a6-digits-100.txt` | 1 | 10 lines of digits, each 100 characters |
| `a7-smallest-voucher.txt` | 1 | the smallest one-voucher post text: a two-ledger Payment, with its answer line |
| `b-cap-37.txt` | 1 | 37 lines of 31 capitals: exactly the cap, in the worst shape the count admits |
| `b-cap-37-batch.txt` | 50 | the same, for the longest batch title |
| `b-over-38.txt` | 1 | 38 such lines: one over the cap |

`panel-bounds.swift` (run as `swift requests/panel-bounds.swift`) reads the frame of every on-screen UserNotificationCenter window and every screen's visible frame, in points. It opens no window and presses nothing. It exits:
- 0 with one or more panels;
- 2 when the read worked and found none;
- 3 when the window list could not be read.

## The command of a cell
`<mode>` is one of `--confirm-journal`, `--confirm-review`, `--confirm-release`, `--confirm-settle` and `--confirm-masters`.
```
<BIN> <mode> < requests/<input> > evidence/<cell>.out 2> evidence/<cell>.err &
sleep 3; swift requests/panel-bounds.swift > evidence/<cell>.bounds; echo "bounds_exit=$?" >> evidence/<cell>.bounds
wait $!; echo "exit=$?" > evidence/<cell>.exit
```
- The owner reads the panel and clicks Cancel after the bounds line is written; the `wait` then records the child's exit.
- **A bounds exit of 2 or 3 is not a measurement.** Run the read again once while the panel is open. If it fails again, record it and infer nothing from that cell.
- **Leftovers:** check `pgrep -fl -- '--confirm-'` before the sitting, after each cell and at the end. EXPECT none. If there is any: record it, end nothing, STOP.

## Rules
- **No window without the OK.** No cell starts before the owner's OK in chat for this sitting.
- **KEYS.** Nobody presses a key while a panel exists. Every answer is Cancel by mouse.
- **A token in any `.out`** is a FAIL. Record it and STOP.
- **Privacy.** The bounds name screen sizes only. Before committing evidence, search it for the user's name and for paths outside the lab folder, and replace each with `<user>` or `<path>`.

## Part A: calibration (BUILD-A, `--confirm-journal`)
| cell | input | EXPECT and record |
|---|---|---|
| A1 | `a1-one-line.txt` | one panel; its `w` and `h` |
| A2 | `a2-ten-lines.txt` | `h` |
| A3 | `a3-twenty-lines.txt` | `h`; **line height** = (A3 − A2) / 10; **chrome** = A1 − line height |
| A4a to A4e | `a4*-capitals-*.txt` | `h` of each. While one capitals line stays one wrapped line, `h` = chrome + 10 x line height. **Capitals per line** = the longest width that still gives that height. |
| A5 | `a5-prose-100.txt` | `h`, and wrapped lines per 100-character prose line = (`h` − chrome) / (10 x line height) |
| A6 | `a6-digits-100.txt` | the same for digits |
| A7 | `a7-smallest-voucher.txt` | `h`; its **real wrapped lines** = (`h` − chrome) / line height; whether it fits this Mac's visible height |

The owner also reads A1 aloud for the record: the panel's width, whether its text is centred, and whether its buttons sit side by side or stacked.

**Reading Part A.** Five figures come out of it:
- the line height, the chrome and the capitals per line (from A1 to A4);
- prose and digits per line (from A5 and A6).

From them:
- `MAC_WRAP_CHARS` = the capitals per line;
- `MAC_WRAPPED_LINES` = floor((visible height − 12 − chrome) / line height), where:
  - the visible height is the 13.3-inch target, 900 − 24 − 84 = 792 points, unless the coordinator rules otherwise;
  - the 12 points of slack absorb a second title line.

Record both, and the arithmetic, in `RESULTS.md`.

**The question Part A answers alone: does the smallest one-voucher post still fit under the measured cap?**

A7's text is the smallest one-voucher post text in the product's tests, as the parent would show it: a two-ledger Payment with a short narration, with its answer line first. It has 21 lines. Their lengths in characters, in order:

113, 0, 42, 50, 46, 28, 92, 0, 18, 15, 31, 79, 50, 90, 45, 99, 67, 100, 94, 15, 17.

(The company number is a synthetic `424242`, the same length as the test's.)

The change's count for it is the sum over those lines of max(1, ceil(length / `MAC_WRAP_CHARS`)). At some widths:

| `MAC_WRAP_CHARS` | 26 | 28 | 30 | 31 | 32 | 34 | 36 | 38 | 40 | 42 | 44 | 46 | 48 | 50 | 58 | 68 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| count | 52 | 50 | 48 | 46 | 45 | 42 | 42 | 41 | 40 | 39 | 39 | 35 | 34 | 30 | 29 | 28 |

Record in `RESULTS.md`:
- the count at the measured `MAC_WRAP_CHARS`;
- the measured `MAC_WRAPPED_LINES`;
- A7's real wrapped lines from its height.

| reading | what follows |
|---|---|
| A7's count at the measured width is at most the measured cap | The change can ship at the measured figures. Set both constants to them and go on to Part B. |
| A7's count is over the measured cap | **STOP. Do not tune either figure, and do not run Part B.** At the measured figures the change refuses every one-voucher post on a Mac, which turns Mac native posting off. Bring the five figures, the cap and A7's count to the run coordinator. It goes back to the owner as a new decision: keep the alert as it is, or a measured panel. |
| A7's real wrapped lines are well under its count | The count over-counts real text. Record both. The coordinator decides whether a measured average width may replace the capitals width; the sheet does not. |
| A7's panel is taller than this Mac's visible height | Today's smallest voucher post already runs off this screen. Report it at once: it is a live defect of today's build, whatever this change does. |

## Part B: the cap (BUILD-B; after the two figures are set from Part A and built)
Rebuild `b-cap-37.txt`, `b-cap-37-batch.txt` and `b-over-38.txt` at the measured figures before this part: lines of exactly `MAC_WRAP_CHARS` capitals, `MAC_WRAPPED_LINES` of them, and one more for the over cell. Re-hash and re-commit them.

| cell | mode, input | EXPECT |
|---|---|---|
| B1 | `--confirm-journal`, `b-cap-37-batch.txt` (count 50) | Shown whole: `h` at most 780, `y + h` inside the visible frame, both buttons on the screen. exit 1; `.out` empty |
| B2 | `--confirm-review`, `b-cap-37-batch.txt` | as B1 |
| B3 | `--confirm-release`, `b-cap-37.txt` | as B1 |
| B4 | `--confirm-settle`, `b-cap-37.txt` | as B1 |
| B5 | `--confirm-masters`, `b-cap-37.txt` | as B1 |
| B6 to B10 | each mode as B1 to B5, `b-over-38.txt` | **No panel appears.** Bounds `panels=0`, `bounds_exit=2`; exit 77; `.out` empty. The owner confirms nothing opened. |

In B6 to B10 there is nothing to answer, so the owner touches nothing. The lab records the exit after the bounds read.

## Reading it
| reading | meaning |
|---|---|
| A1 to A6 read, and B1 to B5 as expected | The cap holds every box on the 13.3-inch target with the measured figures. |
| A4: even 26 capitals wrap | The text is narrower than estimated. `MAC_WRAP_CHARS` goes below 26; re-run A4 lower before Part B. |
| A7 over the visible height of this Mac | Today's smallest voucher post already runs off this screen. Report it at once: it is a live defect, not this change's. |
| B1 to B5: any `h` over 780 or a button off the screen | The count under-counts some line. Report it, with the cell's whole bounds. The change does not ship as is. |
| B6 to B10: a panel appears, or the exit is not 77 | The child's check did not refuse a text over the cap. FAIL; the change does not ship as is. |
| A bounds read with exit 2 or 3 twice | Nothing is measured in that cell. Record it and infer nothing. |

**FAIL of the row:**
- a token in any cell;
- a key pressed by anyone;
- a click on a positive button;
- a leftover process.

## Evidence
- `RESULTS.md`: one line per cell, with the exit, the `.out` bytes, the bounds lines whole, and what the owner saw in the owner's words; then the Part A arithmetic.
- Each `<cell>.out`, `.err`, `.exit` and `.bounds`, whole, after the privacy search.
- The builds as recorded, the macOS version, the number of displays, each one's resolution and scaling mode, and the Dock's position and size.
- `SHA256.txt` over the committed files, with their sizes.

The evidence goes on a new branch, `lab/1964-mac-alert-caps-answers`, pushed only after the pre-push scan and the run coordinator's yes.

## Not measured (say so in the results)
- the 13.3-inch screen itself (the panel's height in points does not depend on the screen, so its bounds are compared with the target's 792);
- other macOS versions, a second display and VoiceOver;
- the server path with a Tally behind it.
