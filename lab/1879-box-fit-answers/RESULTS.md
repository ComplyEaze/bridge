# Results: 1879-BOXFIT on a real Windows box (25 Oct 2026). B1 is a FINDING; B2 to B7 held

Windows lab PC: 1920x1080 at 125 percent; work area 1920x1030 at (0,0) in physical pixels. The display setting was not changed. Synthetic text only; nothing was sent to Tally, and no key was pressed while a box was open.

## Build and files
- **PR:** #1879; its head is `dafa1c9fcb3040b1ab8888a07f7c6821ecdb2d3c` both on the sheet's line and as the PR's head when the item started. Run 40171649980; artifact 12591232885 `unsigned-bundle-smoke-windows-latest`, expires 2026-11-01. The owner approved the download.
- **Hashes, every one equal to the sheet's line:**

  | File | Bytes | sha256 |
  |---|---|---|
  | artifact zip | 61,131,103 | 77f1c4ccc4de8ef22dd45b97a9b21f5fb902aa2165bb0ea1a314c6201033cb32 |
  | `bridge-tally.mcpb` | 16,327,659 | 5a2351d9db5fdc6137c4821c7856d80761d8d225d8b90f24798114e9f5fa5326 (equal to `archive_sha256` in `mcpb-smoke.json`) |
  | `bridge_mcp.exe` | 33,627,136 | d50abce7eaf9ce06b48563cc4b01ce8a013eac11071ab687390c0e79639b7014 (equal to `binary_sha256`) |

  The manifest version is 0.6.0. The exe was unpacked from a copy of the `.mcpb` and staged only; nothing was installed.
- **Sheet files:** `SHA256.txt` 12/12 OK. `make-probe-inputs.py` (run with `python -I` into a new folder) regenerates all six inputs byte for byte.
- **Dry run:** `run-fit.ps1 -DryRun` printed the case, the exe hash, the screen and the command, and opened nothing.
- **Who clicks:** the owner declined desktop-control access to the box's process, so the box was hidden from the lab's screenshots and the owner was to answer by mouse.

## B1, probe-44 (to be answered Yes by mouse): FINDING
The `run-fit.ps1` output is in `evidence/b1-probe-44-run-fit.txt`, with paths replaced by placeholders.

| | Expected | Seen |
|---|---|---|
| window | the box whole on screen | `ComplyEaze Bridge — post this voucher?`, rect (L,T,R,B) 588,116,1353,990, 765x874 physical pixels. All four off-screen flags False: inside the 1030-high work area with 40 px to spare at the bottom |
| came to the front without a click | asked | **No.** The child was started from a hidden background shell. The box did not come to the front; it stayed behind other windows |
| answer | Yes by mouse | **nothing was clicked** (the owner's own word) |
| `cmd exit=` | 0 | **77** ("did not fit") |
| elapsed | (user time) | 316,649 ms. The stdout file was created at 15:03:21 IST and the run ended at 15:08:37 IST |
| stdout | `bridge-post-approved:` + nonce | 0 bytes (sha256 e3b0c442…) |

What this shows:
1. **The hook fires:** the dialog was ended by the product with 77, not by a person and not by a timeout.
2. **A box wholly inside the work area was judged not to fit and cancelled.** This is the sheet's thing 4 (units or work area differ from what the code assumes), seen on B1 rather than on B3. The product is DPI-unaware: in virtualised units the box is about 612x699 at top 93 and bottom 792, against a work area of about 1536x824. In both unit systems the box fits.
3. **It was cancelled minutes after it opened, not when it opened.** The cancel came 5 min 16 s after the box opened, which points to activation happening late. The box was not activated when shown from a background shell. It was activated when the foreground later changed: around 15:08:37 the assistant's own window changed state at the end of a turn. What exactly activated it is not established.
4. **The box did not come to the front by itself.** A person would not have seen the approval box without looking for it behind other windows.

No screenshot of the box exists: the box's process was hidden from the lab's screenshots by the owner's choice.

## Not run
- **B2 to B7:** held on review. On this build they would only reproduce the cancel. B1 re-runs on a build with the fix.
- **B8 and B9:** not run, because a display change was not allowed.

Counted writes: none. Tally was not touched (one company was loaded throughout, as found).
