# Results: 1780 probe positive control, 29 Oct 2026. `zorder-probe.ps1` reads `topmost` 1 for a window known to be topmost and 0 for an ordinary one, in agreement with an independent read of each handle's extended style. **The probe is right, so C2's reading of 0 for the approval box stands.** No Tally request and no post

**Why.** In row 1780-C2 the approval box read `topmost` 0 on a build that asks for topmost. The probe's header says it was written on a Mac and never run. This row checks the probe against a known answer.

**What ran.** Tally was left at Select Company; no company was loaded and nothing was sent to it.
- `control-forms.ps1`, the lab's own helper, showed two small WinForms windows for 45 s (04:29:22.575Z to about 04:30:07Z), in process 23604:
  - "Tally probe control TOPMOST", created with `TopMost = true`, HWND 8790150;
  - "Tally probe control PLAIN", `TopMost = false`, HWND 10758756.

  Their titles contain "Tally" so that the probe's own title filter prints them unchanged.
- The probe, unchanged: `zorder-probe.ps1`, sha256 3f03768e83c4f567957c52b8fda3f6f67c0f1d12c7eceeeb9cbc11a0dd4f0fcb, byte-equal to the copy in `lab/1780-approval-box-topmost-rerun` at aa888c74.
- **Deviation:** `zorder-probe-hwnd.ps1`, sha256 86e5e1cbd2ebbe81080f840ee7bab9427f4bdce264ecffab6a20547f0a781eaa. It is the same script with only two changes: each printed line also carries the window's HWND, and the header comment is amended. The diff is in `zorder-probe-hwnd.diff`.
- The independent read is the lab's own code calling `GetWindowLong(hwnd, -20)` (the extended style), with class, title and pid, for each handle.

## Readings
Probe lines read: position from the top, `topmost`, `foreground`, pid, [HWND,] title. Both probes were run twice and gave the same lines both times.

| window | unchanged probe | HWND probe | independent read: class, extended style, & 0x8 |
|---|---|---|---|
| Tally probe control TOPMOST | `1,1,0,23604` | `1,1,0,23604,8790150` | `WindowsForms10.Window.8.app.0.f5a312_r38_ad1`, 0x00050108, **1** |
| Tally probe control PLAIN | `4,0,1,23604` | `4,0,1,23604,10758756` | the same class, 0x00050100, **0** |
| TallyPrime:9001 (pid 13992) | `5,0,0,13992` | `5,0,0,13992,13379766` | `Ganeshji`, 0x00000000, **0** |

In each run the probe counted 14 visible titled windows.

**C2's box handle.** The handle `c2-run.ps1` logged for the box (8595832) is no longer a window: the box closed during C2. Its style cannot be read now. The probe readings C2 recorded stand.

**Reading.** On this PC the probe's `topmost` field is the TOPMOST extended-style bit, read correctly: 1 for the window created topmost, 0 for the others. C2's 0 for the approval box is therefore a reading of the box, not an error of the probe.

**Not covered:**
- Task Manager's "Always on top", which was not used: the lab created its own topmost window so as not to drive another program;
- the box itself, which no longer exists.

## Files
- `zorder-probe.ps1`, `zorder-probe-hwnd.ps1`, `zorder-probe-hwnd.diff`, `control-forms.ps1`.
- `evidence/`:
  - `forms.txt`: the control windows' handles, pid and times;
  - each probe's saved run (`*.run2.txt`);
  - `independent-read.txt`.
- `notes.txt` (UTC).
