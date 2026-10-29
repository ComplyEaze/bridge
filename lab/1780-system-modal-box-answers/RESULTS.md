# Results: #1780-H, the approval boxes opened system-modal on #2107's build, 29 Oct 2026. **The fix holds: every box read topmost 1 from its first block, stayed above Tally after the click into Tally, and Tally kept the foreground. T4 (the taskbar activation) could not be made: while a box was open the taskbar drew no task buttons. Part D not run (the owner was not at the PC).**

| cell | path | first block: exstyle, topmost, fg | above Tally at C+0 and C+6 | key / taskbar | answer |
|---|---|---|---|---|---|
| H0 (B3b) | child | 0x00010108, 1, the box (no click into Tally) | (no click) | - | No at T0+10.53 s; exit 1, stdout 0 |
| H1 | child | 0x00010108, 1, Tally | **yes** (box z 1, Tally z 4, Tally foreground) | T4: no task button drawn | No at T0+48.15 s (the runner's own fallback); exit 1, stdout 0 |
| H2 | child | 0x00010108, 1, the box | **yes** | T4: no task button drawn | No at T0+37.16 s; exit 1, stdout 0 |
| S1 | server, post_import | 0x00010108, 1, the box | **yes** | S7: Down went to Tally; S8: no task button drawn | No at T0+36.12 s; `import_approval_declined`, nothing dispatched |
| S9 | server, post_import (repeat) | 0x00010108, 1, the box | **yes** | - | No at T0+30.05 s; `import_approval_declined` |
| L2 | server, post_ledger_creates | 0x00010108, 1, Tally | **yes** | - | No at T0+30.1 s; `master_approval_declined`, `not_dispatched` |

**The sheet's readings:**
- **T1: every cell reads 1.** In all six traces every block shows the box with `topmost=1` (exstyle 0x00010108) from the first block to the end; no change. H1, H2 and both S runs read 1: "the fix holds on the child and the server path".
- **T2: the box stays above Tally after the click.** At C+0 and C+6 of H1, H2, S1, S9 and L2 the probe reads the box at position 1 with topmost 1, Tally at position 4, and Tally holds the foreground.
- **T3 (S7): the key went to Tally.** The one Down was pressed at T0+23.38 s in S1 with the box open and Tally holding the foreground (both checked just before). Tally's Gateway highlight was on Create before part S and on Alter in the next full-screen capture (part L, T0+13); no other key was sent to Tally in between. The S1 captures themselves (1536 x 864, the top-left of the panel) do not show the menu, so the reading rests on the part L capture. Whether the box's focused button changed was not seen. "Topmost does not take the keyboard."
- **T4: not measured.** At T0+24 to T0+27 of H1, H2 and S1 a capture of the taskbar (DPI-aware, the bottom 60 px of the 1920 x 1080 panel) showed the whole task-button area as one undrawn dark band (from just right of the search box to just left of the tray), with no button to click. The idle taskbar, captured between cells the same way, drew every button. So no taskbar or thumbnail click was made; the boxes were answered No. Whether the band is the system-modal box's doing was not tested.
- **T5:** every No by mouse declined with nothing sent. The box's rectangle: the child's `box-fit:` line reads `box=(686,181,1254,926) work=(0,0,1920,1030) fits=true` in H0, H1 and H2, the same as the sheet's reference; the server's post box was 688,181,1252,926 physical (S9), the ledger box 713,370,1228,740 physical (L2).
- **T6: the ledger-create box reads 1** and stays above Tally after the click; it opened without taking the foreground (Tally kept it from T0+3).
- **Late-open timing:** the title click to `FIRST` was 8.1 s in S1 (12:25:42.651Z to 12:25:50.750Z), 7.7 s in S9 (12:27:40.122Z to 12:27:47.848Z) and 9.9 s in L2 (12:29:24.346Z to 12:29:34.265Z) (the 8 s wait plus the call's own read): over 5 s in each, so the late-open timing was tested.

No Yes; nothing posted and no ledger created: the marks read 224/87 at S2, S10, L0 and L6, and no ledger named `Lab Topmost Suspense` at L0 or L6. No key other than S7's one Down. B5 was clean before, after part H, at S10 and at the end.

**Who acted.** Done by the lab on the owner's standing word of 27 Oct (the clicks into Tally, the one Down key and every No), with posting (and master writes for part L) on for parts S and L only, on the owner's word of 28 Oct. The CI bundle was downloaded on the owner's yes of 29 Oct. The sheet is `lab/1780-system-modal-box` at 693a04f5258cb14592b197b966de5aa1e60c0572; `SHA256.txt` matched 9 of 9 before the sitting and the request files were equal again at the end. The row note is `lab/0.7.0-auto-order` ORDER.md "Row 23" at 36584e07.

## Build and setup
- **BUILD-H:** #2107 at 488983145f940dfd9fa84e33a93eec37f59328e2, Bridge CI run 40865537013, artifact 12894022601 (`unsigned-bundle-smoke-windows-latest`). The zip: 57,083,486 B, sha256 970e1e422270cfdde68b4a58cc6b8bc7d202ea39f17ac7a1ce7d410577234ab0 = GitHub's digest. `bridge-tally.mcpb` e6767ac5f8abaa39b34a8ada2d37b8b956e71ddfc72c5878d433f93e04302b16, `bridge_mcp.exe` a946979561b61f51e6dab3539d3de98fd48c8f103f61d2e3abe787acf3240c23, `pdfium.dll` 79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118: each equal to `mcpb-smoke.json`, before the first step and at the end. The whole bundle extracted into the build folder. The tool reports bridge-tally 0.6.1.
- **The PC:** Windows 10.0.19045.6466; TallyPrime 7.2 Silver; one monitor, 1536 x 864 logical, 1920 x 1080 physical (125 %); the screen tool's frame 1456 x 819. The desktop client's version was not read afresh.
- **P0:** BRIDGE BATCH LAB (100050) loaded alone; Tally maximised on the Gateway; Tally's date Friday 21-Aug-2026.
- **B3:** Tally first among the titled windows and in front (3,0,1), maximised; P-time 1.28 s.
- **B3b (cell H0):** `FIRST` 12:18:02.226Z, first block 12:18:02.379Z (153 ms); exit 1, `stdout_bytes=0`.
- **S0 (BK0):** Tally's Backup to the new empty bk0 folder (empty before; one file after): `TDBK1800_100050.001`, 57,180 B, sha256 4b10ffcbc393cac9a8d43774b8e520000373b7378ea40922bb8db226a3bec356, 12:23Z. Not restored (nothing changed).
- **S1 to S3:** `launch-env.json` keys `BRIDGE_AGENT_DATA_DIR`, `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_TALLY_HOST`, `BRIDGE_TALLY_PORT`, `BRIDGE_TERMS_ACCEPTED`, sha256 29643b7d...; `tally_status` 7.2 silver, not education; `list_companies` BRIDGE BATCH LAB alone; `ledger_masters` marks 224/87. `build_import_xml` with `build-1591-s1.json`: `voucher_count` 1, `total_debit` and `total_credit` 141, `verification_preflight.state` `current_window_readable`, `source_rows` 0. BATCH-H `bridge-554b8c2b-85af-495c-b8b4-c91727d4010c`.
- **S5:** the post box's title "ComplyEaze Bridge — post this voucher?", the first line as the sheet quotes it, and the lines under it naming BRIDGE BATCH LAB, its GUID and number 100050, one Receipt, 14-Sep-2026, Dr 141.00 "Synthetic Bank Ledger", Cr 141.00 "Party A", the totals, BATCH-H, Reference (none) and the narration "Lab topmost probe 1" (`evidence/s1-box-text.txt`, `s9-box-text.txt`). Dates in day-month-year form.
- **L0 to L3:** the same file with `BRIDGE_AGENT_ENABLE_MASTER_WRITES` added (sha256 c4619b8f...); `prepare_ledger_creates` suspense `Lab Topmost Suspense`: BATCH-L `bridge-d02ea637-6bf6-436d-81d0-c48f12330ab4`, parent Suspense A/c, nothing sent. The ledger box: title "ComplyEaze Bridge — create one ledger?", naming BRIDGE BATCH LAB and `Lab Topmost Suspense` under Suspense A/c (`evidence/l2-box-text.txt`). `post_ledger_creates` takes only `batch_id`.
- **WRITES OFF** after L6: the launch stopped by closing its input (server exit 0, error stream 0 B, both launches); both keys removed: `launch-env.json` sha256 d2622a36dfacc655d53739dcd774213ed123e985345199bb76cd13d69d47f522.
- **Close:** BRIDGE BATCH LAB shut; none loaded; one tally.exe owns 9001; `/status` 200.

## Deviations
1. **Each `.cmd` is opened through `explorer.exe`,** not typed in an interactive PowerShell (as in row 1780-G). The `.cmd` holds the sheet's two lines exactly.
2. **T4 not made** (see T4 above). H1's box was answered by the runner's own fallback at T0+48.15 s while the lab looked for a task button; H2's and S1's after the taskbar capture showed none (S1's at T0+36.12 s, still before T0+38).
3. **Captures:** part H and S1 captures are from a process that is not DPI-aware (the top-left 1536 x 864 of the panel); S9 and L2 captures are whole-screen (1920 x 1080), and their click points are given in physical pixels (the same spot on the screen).
4. **Probes in part S and L** are single `zorder-probe.ps1` runs at T0+3, +8 and +13, C+0 and C+6 (part H used the probe pairs the sheet names).

## Not measured
T4 on this PC; whether the box's focused button changed at S7; part D (the desktop client as the server's parent); the review acknowledgement and invoice-stop release boxes; any key other than one Down; a second monitor; Windows 11; macOS; more than one sitting.

## Files
`evidence/`: each cell's timeline, `<cell>.txt`, `<cell>.stderr`, `<cell>.stdout`, trace and box text; the probes; B3 and B5 files; the BK0 listing and hash; the close checks (netstat cut to port 9001). `answers/`: the MCP answers of parts S and L, whole. `notes.txt`, `client-log.txt`, `SCREENSHOTS.txt` (captures kept on the lab PC), `substitutions.txt`, `SHA256.txt`, and the lab's runners under `scripts/`.
