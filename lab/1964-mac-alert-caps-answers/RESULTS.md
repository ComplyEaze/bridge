# 1964-M Part A: the macOS approval alert measured. Verdict: STOP (Part of #1780)

Sheet `lab/1964-mac-alert-caps` at 4360d8de, Part A only, 30 Oct 2026, 09:00 to 09:56 UTC. No server ran and no Tally was reached. Each alert came from the product's dialog child, fed a fixed synthetic text, and was answered Cancel by mouse. Nothing printed a token, and no key was pressed.

**BUILD-A:** a local debug build of `bridge_mcp` at 026ba163 (master, candidate A), sha256 `05f926ad1a2e5fafcea6854e7b0bcaf649da3b0374372bbb8b945006ecea2a67`, 74,391,232 bytes. macOS 26.6.2, one display of 1512 x 982 points, 1512 x 865 of them visible (top inset 33, Dock at the bottom), scale 2.0.

**Who acted:**
- A1's Cancel was not given (see the deviation).
- From A2 on, the lab clicked Cancel through the accessibility "Cancel" button of each alert, on the owner's word relayed by the run coordinator, after each cell's bounds read was recorded.

## Cells
`first read` is the runner's read, taken 0.5 to 1.5 s after the start, which can catch the opening animation. `settled` is the frame once the alert stopped moving (from the window list, or a second read).

| cell | text | layout | first read (w x h) | settled (w x h, top) | exit | `.out` |
|---|---|---|---|---|---|---|
| A1 | 1 short line | compact | 260 x 234 | 260 x 234, y 168 | not recorded | 0 B |
| A2 | 10 short lines | compact | 264 x 382 | 260 x 378, y 168 | 1 | 0 B |
| A3 | 20 short lines | wide | 538 x 486 | 520 x 470, y 168 | 1 | 0 B |
| A4a | 10 x 26 capitals | wide | 528 x 316 | 520 x 310, y 168 | 1 | 0 B |
| A4b | 10 x 31 capitals | wide | 532 x 318 | 520 x 310, y 168 | 1 | 0 B |
| A4c | 10 x 36 capitals | wide | 520 x 310 | 520 x 310, y 168 | 1 | 0 B |
| A4d | 10 x 41 capitals | wide | 520 x 310 | 520 x 310, y 168 | 1 | 0 B |
| A4e | 10 x 46 capitals | wide | 526 x 330 | 520 x 326, y 168 | 1 | 0 B |
| A5 | 10 x 100 prose | wide | 538 x 486 | 520 x 470, y 168 | 1 | 0 B |
| A6 | 10 x 100 digits | wide | 552 x 500 | 520 x 470, y 168 | 1 | 0 B |
| A7 | the smallest one-voucher post text, 21 lines | wide | 586 x 674 | 520 x 600, y 168 | 1 | 0 B |

**What the screen showed.**
- **Two layouts:**
  - Compact (A1, A2): 260 wide, the icon above the text, Cancel left and the positive button right.
  - Wide (A3 to A7): 520 wide, the icon left of the text, the positive button left and **Cancel right**.
  - The panel chose the wide layout for 20 short lines and for 10 lines of 26 capitals.
- **A4e:** nine of the ten 46-capital lines fitted one line; one was broken with a hyphen.
- **A5 and A6:** each 100-character line took two lines.
- **A7:** shown whole, in 28 lines.

## The five figures (wide layout; verified for this Mac and this sitting, one run each)
- **Line height: 16 points.** (A3 − A4a) / 10 = (470 − 310) / 10. This is the default line height of the 13-point system font, measured headlessly before the sitting, not the 11-point font the design note estimated.
- **Chrome: 150 points.** A4a − 10 x 16. The compact layout's chrome is 218 points (A1 − 16).
- **Text width:** the panel is 520 points. Capitals per line: 41 fit (A4d) and 46 did not all fit (A4e), so **`MAC_WRAP_CHARS` = 41**. The 228-point estimate was wrong: the text column is about 406 points.
- **Prose per line:** a 100-character line takes 2 lines (A5), so at least 50.
- **Digits per line:** a 100-character line takes 2 lines (A6), so at least 50.

**The measured cap:** by the sheet's formula, floor((792 − 12 − 150) / 16) = **39 lines**, so `MAC_WRAPPED_LINES` = 39.

## The question Part A answers
- **A7's count at the measured width (41): 40 lines.**
- **The measured cap: 39 lines.**

**Verdict: STOP.** The count is over the cap, so the change as built would refuse the smallest one-voucher post on a Mac. Neither figure was tuned, and Part B was not run.

Recorded beside it:
- **A7's real wrapped lines: 28** (600 = 150 + 28 x 16, and 28 lines were counted on the screen). The count over-counts real text by 12 lines, because it charges every character at the width of a capital.
- **The count at other widths:** at 45 characters a line, A7 counts 37; at 50, it counts 30. Whether a measured average width may replace the capitals width is for the coordinator; the sheet does not decide it.
- **A7 fits a 13-inch screen today:** its panel is 600 points tall, with its top at 168, so its bottom is at 768. A 13.3-inch Mac's visible bottom is about 816 (900 − 84).

## Findings
- **The panel does not centre and is not moved up.** Every panel opened with its top at y = 168 on this screen, whatever its height. A panel taller than the space below 168 runs off the bottom, which matches the owner's 43-line overflow on 23 October. So the room a text has is the visible bottom minus about 168, not the whole visible height. On a 13.3-inch Mac, if its top is also at 168 (not measured), that room is 900 − 84 − 168 − 12 = 636 points: a cap of 30 lines. Confidence: verified here (11 panels); the 13.3-inch figure is an estimate.
- **The orphan panel (seen twice: 26 Oct and here).** After A1's dialog child had ended, its panel stayed on screen with no process behind it (read at 09:39:53: `panels=1`, same frame, no `--confirm-` process). By 09:53:12 it was gone, and who or what cleared it was not observed. Nothing reads such a panel's answer. In the product, a parent that times out after 120 s and ends its child could leave such a panel asking a question nobody will hear answered. Confidence: observed twice; cause not established.
- **The buttons swap places with the layout.** In the compact layout Cancel is on the left; in the wide layout it is on the right, where the positive button is in the compact one. Real approval texts take the wide layout (A7). Cancel stays the default button in both (the `rfd` dispatch); the place a hand reaches for is what differs.
- **The runner's first read can catch the opening animation,** up to 74 points taller than the settled frame (A7). A check that reads the frame early must wait for it to settle.

## Deviation
A1's runner ran under a 30-minute watch. At about 09:30 the watch ended it and its dialog child, before any Cancel. A1's exit was not recorded. A1's frame (read on try 3) equals its settled position, so A1 counts as measured. A2 to A7 ran from 09:53 with no watch limit.

## Not measured
- a 13.3-inch Mac itself, and where its panel's top sits;
- other macOS versions, a second display, VoiceOver;
- whether a real text can ever take the compact layout, whose chrome is 68 points taller;
- capitals between 42 and 45 a line.

One run of each cell.
