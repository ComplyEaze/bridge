# Windows item 1780-R6-bis v1 (DRAFT, run only if R6 / Part D reads topmost 0): the approval box owned by a hidden topmost window, under the desktop client as the server's parent and under the test parent. (Part of #1780)

WRITES to the book: NONE. Every box is answered **No by mouse**. Posting is switched on only for part S, and every `post_import` there is answered No. Keys are as in row 1780-H: none while a box holds the keyboard; the one Down key of S5 only while Tally holds the foreground.

## Why
- **Row 1780-H** (`lab/1780-system-modal-box-answers` @ 968810d4): with `MB_SYSTEMMODAL`, every box read topmost 1 under the test parent and the server.
- **Its part D**, run at the release sitting as R6, puts the desktop client in the parent's place. **This row is run only if R6 reads 0**, that is, if the box opened under the desktop client lacks the topmost bit or Tally covers it after a click.
- **The fix it tests** is the parked branch `fix/1780-hidden-topmost-owner`. `box_fit::guarded` makes a hidden, topmost tool window on the box's thread (`TopmostOwner`, made with `CreateWindowExW(WS_EX_TOPMOST | WS_EX_TOOLWINDOW, "Static", …, WS_POPUP, …)`, never shown) and passes it as each box's owner, keeping `MB_SYSTEMMODAL`.
- **Its precedent.** Row 1780-G's variant i measured that owner on a stand-in box, without `MB_SYSTEMMODAL`: topmost 1, above Tally, twice (`lab/1780-topmost-fixes-answers` @ ba103f22). The combination with `MB_SYSTEMMODAL` is not measured.
- **What this row decides:**
  - R1. Is the product's box owned by a hidden window that is topmost? (The trace's box row reads `owner` not 0, `owner_class` `Static` and `owner_topmost` 1.) Does the box read topmost 1 and stay above Tally after a click into Tally, under the test parent and under the desktop client?
  - R2. Does the box have a taskbar button? (EXPECT none: an owned window's button is not shown.)
  - R3. Does a No still answer it (`import_approval_declined`), and is the owner gone when the box is?

## Who acts
- **The lab** does the test parent's cells on the owner's standing word of 27 Oct 2026. Posting for part S is on the owner's word of 28 Oct.
- **The owner's own steps.** The desktop-client cell (part D) needs the owner at the PC, as R6 did.
- **Downloading** the branch's CI bundle needs the owner's OK in the lab chat. The bundle comes from a run started by hand on the branch, once it is pushed.

## Build and inputs (hash before the first step and at the end; a different hash = STOP; `SHA256.txt` lists every file of this folder with its size)
- **BUILD-R6B:** the `unsigned-bundle-smoke-windows-latest` artifact of `gh workflow run ci.yml --ref fix/1780-hidden-topmost-owner` at the head the run coordinator names.
  - The run's conclusion must be success.
  - Record the run id, the artifact id and the digest. EXPECT the zip's sha256 to equal the digest, and `bridge_mcp.exe` to equal `mcpb-smoke.json`'s `binary_sha256`.
- **Byte copies:**
  - `requests/box-trace-early.ps1`, `zorder-probe-k.ps1`, `spawn-parent.ps1`, `confirm-input.txt` and `build-1591-s1.json` from `lab/1780-system-modal-box` @ 693a04f5258cb14592b197b966de5aa1e60c0572;
  - `box-rect-probe.ps1`, `taskbar-capture.ps1` and `taskbar-cover.ps1` from `lab/2161-taskbar-activate-b` @ 996f869ffa4eec7c4a9be53a27b89ea50d1b57b0;
  - all of them have run in rows 1780-H, 2161-A and 2161-B.
- **The rest** are as in row 1780-H: the working folder `C:\bridge-lab\r6b\`, THE COMPANY (BRIDGE BATCH LAB, loaded alone), the cell recipe with the late-open timing, and the runner's rules from row 2161-B. The runner's rules are: point names that differ by more than letter case; a click into Tally must leave Tally in front; and no screen-tool capture while a box is open.

## Rules
They are row 1780-H's KEYS, C2 carve-out, "A box under Tally" (by `ShowWindow` on Tally's handle, as in row 2161-B) and WRITES OFF. Also:
- **No taskbar click** is made in this row. The taskbar is only read: `taskbar-cover.ps1`, and `taskbar-capture.ps1` with the trace console started minimised, as row 2161-B's part P-min.
- **Leftovers (B5)** before, after each part, and at the end. They include any window of class `Static` owned by a `bridge_mcp.exe` process: EXPECT none.

## Part H: the child under the test parent (no server)
- Two cells, **H1 and H2**, each run as row 1780-H's H cells on BUILD-R6B, with the trace pre-armed:
  1. the title click about 4 s after the start;
  2. the box opening some seconds later;
  3. the click into Tally at T0+16;
  4. reads at T0+6, C+0 and C+6: `zorder-probe-k.ps1`, `box-rect-probe.ps1`, and `taskbar-cover.ps1` with `taskbar-capture.ps1`;
  5. No by mouse.
- **Record:**
  - from the trace's box row: `exstyle`, `topmost`, and `owner`, `owner_class`, `owner_exstyle` and `owner_topmost`;
  - z and the foreground from the probes;
  - from the cover reads, any window over the task list;
  - from the `PrintWindow` copy, whether a button was added for the box against the idle copy;
  - the exit code (EXPECT 1), the `.stdout` size (EXPECT 0) and the `box-fit:` lines (EXPECT `fits=true`).
- **After the box is gone:** B5, including no `Static` window of that `bridge_mcp.exe`.

## Part S: a real post under the lab's launch, answered No
- **S0.** Back up BK0 (C5).
- **S1.** Turn posting on for the launch, as row 1780-H's S1 to S4, on BUILD-R6B.
- **S2.** Make the post (`post_import` of `build-1591-s1.json`'s batch) with the late-open timing, the same reads as part H, and the click into Tally at T0+16.
- **S3.** No by mouse. EXPECT `import_approval_declined`, with nothing sent.
- **S4.** Check the marks are unchanged, then WRITES OFF.

## Part D: the desktop client as the server's parent (the owner at the PC)
- **D1.** The desktop client is configured to start BUILD-R6B's server with posting on, as R6 was. THE COMPANY is loaded alone and Tally is in front.
- **D2.** The lab pre-arms the trace and the reads as in S2. The owner asks the desktop client for a post of the S2 batch, clicks once into Tally at about 16 s after the box appears, then answers **No** by mouse.
- **D3.** Record what part H records. EXPECT `import_approval_declined`.
- **D4.** WRITES OFF; marks unchanged; the company shut by C10.

## Reading it (for the requester)
| reading | meaning |
|---|---|
| The box's row reads `owner` a `Static` window with `owner_topmost` 1; the box reads topmost 1 and stays z 1 above Tally at C+0 and C+6, in H, S and D | The hidden owner holds the box on top: the fix-forward works where R6 failed. |
| The box reads topmost 0, or Tally covers it, though owned | The owner does not hold it under this parent: report the trace whole; neither candidate works there. |
| `owner` 0 | Not BUILD-R6B, or the owner was refused (`TopmostOwner::create` gave none): check B1, then report. |
| No button added for the box in the `PrintWindow` copy | An owned box has no taskbar button, as expected: #2161's taskbar question does not arise. |
| A button added | Record it: the box is still on the taskbar. |
| A `Static` window of `bridge_mcp.exe` left after the box | The owner was not destroyed: report it. |
| Anything else | Record it whole and infer nothing. |

**FAIL of the row:** a Yes; a post; a key while a box held the keyboard; a mark changed; a taskbar click; a leftover window or process.

## Not measured
- Alt+Tab, which needs a key while a box is open.
- A box drawn under another window and then sought on the taskbar, which this fix has no button for.
- The review, release and settlement boxes on a screen, though they have the same owner code.
- The ledger box.
- Windows 11, and a second monitor.
