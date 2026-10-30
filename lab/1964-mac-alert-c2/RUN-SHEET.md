# macOS row 1964-C2 (DRAFT, not run): the measured macOS alert, where it opens, and what Return and Escape do (Part of #1780)

Runs only after the owner's OK in chat for this sitting. WRITES: NONE. No server runs and no Tally is reached. Each cell starts the dialog child by hand with a synthetic input. The child prints a token only if its positive button is chosen, and with no parent reading it nothing can post. #2211 stays a draft until this row passes.

## Why
#2211 (option (c2), the owner's choice of 30 Oct) keeps `rfd`'s system alert. Before the alert opens, the dialog child:
- lays the text out with AppKit at 13 points in a 406-point column;
- adds the alert's 150 points of title, icon and buttons and 16 points of slack;
- refuses the alert (exit 77, nothing shown) if, from where its top opens, it would end below the visible bottom of the menu-bar screen.

Those figures are from row 1964-M Part A: one Mac and one run each, so partial. Two things are still open:
- **C2-2, where the top opens.** Part A saw it 168 points down on a 982-point screen whose visible frame starts 33 down. The candidates are a fixed offset, 135 points below the visible frame, or a share of the height; until this row decides, the child takes the lowest of the three.
- **C2-6, Return and Escape.** Cancel is `rfd`'s default button, but it sits left in the compact layout and right in the wide one. Whether Return still chooses Cancel in both layouts is unverified. A token there is a key that approves, and it would bear on today's build too.

## Who acts
- **The owner** is at the Mac. The owner presses every key in C2-6, chooses the display modes in C2-2 in System Settings, and says what each panel looks like.
- **The lab:**
  - starts each command;
  - reads the bounds once the panel has settled (a first read can catch its opening animation);
  - answers each panel Cancel by its accessibility "Cancel" button, after the bounds are recorded, on the owner's word for this sitting.

  The lab presses no key and changes no setting.
- **The run coordinator** gives the go-ahead and any STOP ruling.

## Build and tools
- **BUILD-C2:** #2211's head, built here as a debug `bridge_mcp` (or the CI artifact of that head). Record the commit and the binary's sha256 before C2-0.
- **`requests/panel-bounds.swift`:** prints every on-screen UserNotificationCenter window's frame and each screen's frame and visible frame. It reads only, and exits 0 with a panel, 2 with none and 3 if the list cannot be read.
- **`requests/measure.swift`:** prints the height AppKit gives a request's text, exactly as the child measures it. It reads only.
- **Inputs:** a nonce line, a count line, then the text; synthetic.

| file | text | measured height (this Mac) |
|---|---|---|
| `c2-compact-short.txt` | one short line | 16 |
| `c2-wide-voucher.txt` | the smallest one-voucher post text (Part A's A7) | 448 |
| `c2-cap-in.txt` | 35 short lines | 560 |
| `c2-cap-over.txt` | 36 short lines | 576 |

On this Mac's default mode (1512 x 982, visible bottom 898, top 168) the room for text is 898 − 168 − 150 − 16 = 564. So `c2-cap-in` (560) fits by 4 points, and `c2-cap-over` (576) is over by 12.

- **The command of a cell:**
  ```
  <BIN> --confirm-journal < requests/<input> > evidence/<cell>.out 2> evidence/<cell>.err &
  ```
  Then, once it settles: `swift requests/panel-bounds.swift > evidence/<cell>.bounds`. Then the cell's action, then `wait $!; echo "exit=$?" > evidence/<cell>.exit`.
  - **No runner under a watch with a time limit:** a watch's expiry ended the child in Part A.
- **Leftovers:** check `pgrep -fl -- '--confirm-'` before, after each cell and at the end. EXPECT none. An orphan panel with no process behind it is recorded as such, with its bounds.

## Rules
- **No window without the OK.**
- **KEYS.** Only the owner presses keys, only in C2-6, only Return or Escape, once per cell. The lab presses none.
- **A token** (`.out` not empty) in any cell is a FAIL. Record it, STOP, and report it at once.
- **Privacy:** the bounds name screen sizes only. Search the evidence for the user's name and for paths outside the lab folder before committing.

## Cells
| cell | input | action | EXPECT |
|---|---|---|---|
| C2-0 | `c2-wide-voucher` | lab: bounds, then Cancel | Shown in the wide layout. The settled height is within 16 of 448 + 150 = 598 (Part A: 600). Exit 1, `.out` empty. |
| C2-1 | `c2-cap-in` | lab: bounds, then Cancel | Shown, wide. The settled bottom (y + h) is at most 898 and both buttons are on the screen. Height within 16 of 560 + 150 = 710. Exit 1. |
| C2-1x | `c2-cap-over` | none (nothing should open) | No panel (`panels=0`, bounds exit 2), exit 77, `.out` empty. |
| C2-2a, C2-2b | `c2-wide-voucher` | the owner first switches the display to another scaled mode (System Settings › Displays), one per cell; lab: bounds, then Cancel; the owner restores the default mode at the end | Record each mode's screen frame, visible frame and the panel's top y. Which rule holds: 168 fixed, visible top + 135, or 0.171 × screen height. |
| C2-3 | `c2-cap-in` | as C2-1 | Record the layout 35 short lines take (compact would have a 218-point chrome) |
| C2-6a | `c2-compact-short` | the owner presses **Return** once | The panel closes as Cancel: exit 1, `.out` empty. If nothing happens, the lab clicks Cancel and records "Return did nothing". |
| C2-6b | `c2-compact-short` | the owner presses **Escape** once | as C2-6a |
| C2-6c | `c2-wide-voucher` | the owner presses **Return** once | as C2-6a |
| C2-6d | `c2-wide-voucher` | the owner presses **Escape** once | as C2-6a |

Before every C2-6 cell the owner confirms the panel is in front and its layout (compact: about 260 wide, Cancel on the left; wide: 520, Cancel on the right).

## Reading it
| reading | meaning |
|---|---|
| C2-0, C2-1 within 16 and inside the screen; C2-1x refused with no panel | The measure matches the panel, and the check refuses only what would run off. |
| C2-1's bottom below 898 | The measure under-counts. FAIL; the change does not ship as is. |
| C2-2 settles the rule | The three-rule constant becomes the one rule, with its cell cited (a later change). |
| C2-3 compact | Short-line texts can take the compact layout, whose chrome is 68 points taller. The check must allow for it before it ships. |
| **Any C2-6 token** | **A key approved.** STOP. Report it at once: it affects today's build, whatever #2211 does. |
| C2-6: a key did nothing | Record it; Cancel by mouse. Not a FAIL. |

**FAIL of the row:**
- a token in any cell;
- a panel in C2-1x;
- C2-1 off the screen;
- a key pressed by anyone but the owner, or outside C2-6;
- a leftover process.

## Evidence
- `RESULTS.md`, with one line per cell:
  - the exit and the `.out` bytes;
  - the settled bounds whole;
  - the layout;
  - what the owner saw, in the owner's words.
- Each cell's `.out`, `.err`, `.exit` and `.bounds`.
- The build as recorded, the macOS version, the display modes used, and the Dock's position and size.
- `SHA256.txt` over the committed files, with their sizes.

The evidence goes on `lab/1964-mac-alert-c2-answers`, pushed only after the pre-push scan and the run coordinator's yes.

## Not measured (say so in the results)
- a 13.3-inch Mac itself;
- other macOS versions;
- a second display (record which screen the panel opens on if one is attached);
- VoiceOver;
- the server path with a Tally behind it.
