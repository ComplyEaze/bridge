# Results: row 1780-W, the approval box on master's build (7a3c9e49), 2026-11-11

**W1 holds:** a box opened by the server on BUILD-W stayed topmost and above Tally after one click into Tally, with Tally holding the foreground. **W2 not measured:** part D not run. **W3:** the owned box IS listed in Alt+Tab, as the switcher's second tile, highlighted. **W4a holds:** one typed `n` answered the box while it held the foreground (a decline). **W4b holds:** once a click had given Tally the foreground, one Down went to Tally (its Gateway highlight moved from Create to Alter) and the box stayed open, topmost, above Tally.

No Yes, by click or key; nothing posted; the marks read ALTVCHID 87 and ALTMSTID 224 at P0, S0 and the end (raw MARKS), and voucher_alter_id 87 and master_alter_id 224 at S2 and S8 (ledger_masters). Every box read `topmost=1` (exstyle 0x00010108) in every trace block of every cell. Its owner was a hidden `Static` window of the child's own process with `owner_topmost=1`, and every `box-fit:` line read `box=(686,181,1254,926) work=(0,0,1920,1030) fits=true`. Every answer was a decline: exit 1, `stdout_bytes_read=0` for the child cells, and `import_approval_declined` for the post. B5 was clean before the sitting, after part H, after part S and at the end. No taskbar button was clicked. The only keys pressed while a box existed were the carve-outs: K's `n`, K2's Down, and HA's Alt, Tab and Esc.

**Who acted.** Done by the lab on the owner's standing word: every click into Tally, every No and the keys of cells K, K2 and HA (synthetic company, 27 Oct 2026), and posting on for part S only (28 Oct 2026). Cell HA ran on the run lead's yes, by the method agreed with the run lead. Part D was not run, on the run lead's recorded word.

## Readings

| cell | first block: exstyle, topmost, foreground | above Tally after the click | key | answer |
|---|---|---|---|---|
| H0 (B3b) | 0x00010108, 1, Tally (the box in front from about T0+2.6 s) | (no click into Tally) | none | No by mouse at T0+10.19 s; exit 1, stdout 0 |
| K | 0x00010108, 1, Tally (the box in front from about T0+4.5 s) | (no click into Tally) | `n` ONCE at T0+15.41 s, 0.065 s after the gating read (box (1,1,1)) | the box gone 0.27 s later; the child ended at T0+15.82 s, exit 1, stdout 0 |
| K2 | 0x00010108, 1, the box | **yes**: C+0, C+6, C+7 (read at C+8.6 s) and C+10: the box (1,1,0), Tally (4,0,1) | Down ONCE, 0.054 s after the gating read | No by mouse at T0+27.13 s; exit 1, stdout 0 |
| HA | 0x00010108, 1, the box | **yes**: C+0, C+6 and C+7: the box (1,1,0), Tally (4,0,1) | Alt held, Tab ONCE 0.07 s after the gating read; Esc ONCE at C+8.98 s; Alt released after the switcher had gone | the box in front after the Esc (C+20: the box (1,1,1), Tally (4,0,0)); No by mouse at T0+34.98 s; exit 1, stdout 0 |
| S (S4 to S7) | 0x00010108, 1, Tally (the box in front from T0+9.15 s) | **yes**: C+0 and C+6: the box (1,1,0), Tally (4,0,1) | none | No by mouse at T0+23 s; `import_approval_declined`, `attempt_recorded` false, `dispatch_response` null |

Probe lines read `position,topmost,foreground,pid,title`. "(1,1,0)" is position 1, topmost 1, foreground 0.

- **W1 (part S).** The server (pid 14476, BUILD-W's `bridge_mcp.exe`, the launch's posting on) started the dialog child (`--confirm-journal`, sha256 189cb225..., parent the server). `FIRST` came 8.09 s after the title click (08:35:44.833Z to 08:35:52.919Z). The box held the foreground from 08:35:55.153Z. At T0+8 and T0+13 the reads gave the box (1,1,1) and Tally (4,0,0). After the click into Tally at T0+16.04 s (logical (263,475) = screen tool (249,450), 287 logical px from the box), C+0 and C+6 both read the box at position 1 with topmost 1 and Tally at position 4 holding the foreground. The trace read `topmost=1` in all four blocks. S5: the box's title `ComplyEaze Bridge — post this voucher?` and its first line were as the sheet quotes them. The lines below named THE COMPANY, its GUID, a Receipt dated 14-Sep-2026, Dr 141.00 "Synthetic Bank Ledger", Cr 141.00 "Party A", totals debit=141, credit=141, BATCH-S and the narration "Lab topmost probe 1". The box text is quoted whole below.
- **W2 (part D): not measured.** Part D not run: restarting the desktop client to load BUILD-W would end the lab's own session (D0). It stays owed for a sitting driven from outside the desktop client.
- **W3 (cell HA).** With Alt held after one Tab, one screen copy and a list of the visible windows were taken 0.94 s after the Tab. The list showed the switcher (class `MultitaskingViewFrame`, title `Task Switching`) in front. The copy shows eight tiles, in order: TallyPrime:9001; **ComplyEaze Bridge... (highlighted)**; a cmd.exe console (the cell's .cmd); Windows PowerShell; the desktop client; a browser window; a File Explorer window; a Python console. Under the sheet's stop rule for HA no further Tab was pressed and Esc came first, with Alt still held. 300 ms after the Esc no switcher window was listed, and Alt was released. The trace shows the box in front again 0.107 s after the Esc, before Alt was released. The C+20 read gave the box (1,1,1) and Tally (4,0,0): **the switch was not cancelled** (the box in front, not Tally). The box was still open after the Esc, so the Esc did not answer it (no Cancel); the mouse No answered it.
- **W4a (cell K).** The T0+12 read (run T0+13.98 to T0+15.32 s, late: it waited for box-rect-probe) gave the box (1,1,1), so the box held the foreground. One `n` at T0+15.41 s. The trace shows no box window at 08:26:43.332Z, and the child ended exit 1 with stdout 0 at 08:26:43.465Z, as a mouse No does. The T0+15 and T0+18 reads gave no box and Tally (3,0,0). One `box-fit:` line, `fits=true`.
- **W4b (cell K2).** The k2a copy (Tally's own window, PrintWindow) matched B3c's copy (Gateway, Create highlighted) with 0 % of pixels differing. The C+7 read gave Tally holding the foreground with the box topmost above it, so one Down was pressed 0.054 s after that read. At C+10 the read gave the box (1,1,0) and Tally (4,0,1), and the k2b copy shows the Gateway with **Alter** highlighted, one line below Create. The box stayed open until the mouse No.

## Box text of the post (S5), from s-box-text.txt (the file itself stays on the lab PC: it holds the company number)
```
TITLE	ComplyEaze Bridge — post this voucher?
RECT	550,145,1001,741
EXSTYLE	0x00010108
CHILD	Button	721,696,801,722	&Yes
CHILD	Button	809,696,889,722	&No
CHILD	Button	897,696,977,722	Cancel
Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is
posted until you answer.

Create ONE Receipt in "BRIDGE BATCH LAB"
Company GUID: a30f23c3-4819-4334-96ab-ffa6fba70976
Company number: <company number>  Books from: 1-Apr-2026
Tally: http://127.0.0.1:9001
Date: 14-Sep-2026  Voucher number: Tally assigns it  (the voucher's own text: last two lines)

Dr 141.00  "Synthetic Bank Ledger"
Cr 141.00  "Party A"

Total debit: 141  Total credit: 141
Checked in Tally: every Dr ledger is bank/cash; every Cr ledger holds no money.
Batch: bridge-2775db8f-0ed8-40ff-a6df-6370789cd102
Ledgers checked by identity against the build; narrations sent as prepared, nothing added.
Do not post a file already imported manually.
Pause other edits/imports; keep this company and Tally mode as is until ComplyEaze Bridge finishes.
After a timeout, reconcile this batch; do not rebuild or resend it.
ComplyEaze Bridge posts now or if asked again within 15 min, unless cancelled, refused or restarted.
---- The voucher's own text follows: no line below is an entry, a total or an instruction ----
Reference: (none)
Narration: "Lab topmost probe 1"
```
No is the box's default button: the box text records the buttons, and the box was answered by clicking the centre of `&No`.

## Build and setup
- **BUILD-W:** the `unsigned-bundle-smoke-windows-latest` artifact of the Bridge CI merge_group run of #2497 at 7a3c9e4991d967c33cc450e56cac6844db103353 (conclusion success). The run and artifact ids are not written here as numbers. The zip is 60,423,867 B, sha256 bb058e556b791e0e1d744f0ef4bc5f57280eba84298ee60da27b3c94d6c09102, equal to the digest. `bridge-tally.mcpb` is 10583d4d6443b631fc399e056e4a25684828e5de0ea51bdbabd5556a5ec96d3f, equal to `archive_sha256`. `bridge_mcp.exe` is 189cb22520447c9b0d91da3b773cbcf239ed5fdd2e977cd02ffbbdf91ba32fa6 and `pdfium.dll` 79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118, each equal to `mcpb-smoke.json`, before the first step and at the end. The whole .mcpb (9 entries) was extracted to the build folder. The server reports bridge-tally 0.7.0.
- **The sheet:** `lab/1780-w-approval-box-master` at 10d062752c370a89214c09a57fb892d3479877d9. `SHA256.txt` matched 12 of 12 before the first step, and the 9 request files were equal again at the end.
- **The PC, read afresh:** Windows 10.0.19045.6466 (22H2); TallyPrime 7.2 (tally.exe 1.1.7.2), TallyPrime single-user edition, education mode off; desktop client 4.23118.0; one monitor, 1536 x 864 logical, 1920 x 1080 physical (125 %); the screen tool's frame 1456 x 819.
- **P0:** BRIDGE BATCH LAB loaded alone (Select Company, its name typed as a filter, one row, Enter); Gateway of Tally; Tally maximised; Tally's date Friday 21-Aug-2026. Step 0: one tally.exe (pid 22572) owning both LISTENING lines on :9001; GET /status 200 (bytes=51); MARKS: one row, THE COMPANY's GUID, ALTVCHID 87, ALTMSTID 224 (the sheet's last marks).
- **B3:** Tally clicked, then the read gave Tally (3,0,1) and Tally maximised; P-time 1.3 s.
- **B3b (cell H0):** `FIRST` 08:24:06.572Z, first block 08:24:06.694Z (122 ms later); the box topmost 1, owner `Static` (owner_topmost 1, owner_exstyle 0x00000088); exit 1, stdout 0; one `box-fit:` line `fits=true`. Known answer met.
- **B3c:** `tally-window-capture.ps1 -Name b3c` gave `print_window=True rect=0,0,1920,1030`. The copy shows Gateway of Tally, its menu and the highlighted line (Create) as the screen-tool screenshot shows them. Known answer met.
- **B4b:** cover-idle (the taskbar at z 12; meeting it: the screen tool's own transparent overlay, a cloaked text-input window and Program Manager) and capture-idle (every task button drawn).
- **S0 (BK0):** by Tally's Backup (menu clicks Data, Backup & Restore, Backup; the destination by Specify Path to the bk0 folder, empty before; BRIDGE BATCH LAB only; End of List; Accept Yes): `TDBK1800_<company number>.001`, 57,180 B, sha256 02d3012d85a4f9229756f8815c387f539ce80613d490232c17a92716cb6fae80, the one file in the folder. Not restored (nothing changed).
- **S1 to S3:** the launch file's keys `BRIDGE_AGENT_DATA_DIR`, `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_TALLY_HOST`, `BRIDGE_TALLY_PORT` and `BRIDGE_TERMS_ACCEPTED` (sha256 ec869468...). tally_status: TallyPrime 7.2, education mode off. list_companies: THE COMPANY alone. build_import_xml with build-1591-s1.json: voucher_count 1, total_debit 141, total_credit 141, verification_preflight `{from 20260914, to 20260914, state current_window_readable, source_rows 0, paired_source_bytes 6044}`. BATCH-S is `bridge-2775db8f-0ed8-40ff-a6df-6370789cd102`.
- **WRITES OFF:** the launch stopped (server exit 0, error stream 0 B). `BRIDGE_AGENT_ENABLE_WRITES` was removed from the launch file, leaving the keys `BRIDGE_AGENT_DATA_DIR`, `BRIDGE_TALLY_HOST`, `BRIDGE_TALLY_PORT` and `BRIDGE_TERMS_ACCEPTED`, sha256 1df3a467626b045f8db31fc05687086d5fb605bb5dd51f2e2150f85ad8132d5e. The marks were unchanged.
- **C10:** BRIDGE BATCH LAB shut by K: Company, Shut, Enter on its one row. Tally at Select Company with no company loaded; one tally.exe (pid 22572) owning both LISTENING lines on :9001; GET /status 200; no XML request after the shut.

## Deviations
- Each cell's `.cmd` was opened by the cell runner through explorer.exe, not typed into an interactive PowerShell. The runner (w-cell.ps1, then w-cell2.ps1) is the pre-armed timing helper, and it sent K's `n`, K2's Down and HA's keys itself right after the gating read. The method was accepted by the run lead.
- HA's switcher was read from one script screen copy and the window list taken with Alt held, not by eye: the screen tool takes no capture while a box exists. Method agreed with the run lead.
- w-cell.ps1 (cells H0 and K) wrote its log lines into the return values of its read and click functions, so a gate could compare an array. K's gate read the true state, checked against probe-k-t0p12.txt. K2 and HA ran on w-cell2.ps1 (made by mk-wcell2.py), which writes the log to the host and starts box-rect-probe hidden beside the reads. In H0 the box-rect-probe line's path held two control characters from an editing slip, so H0 has no rect file. In K, box-rect-probe ran between the reads and held the T0+12 read until T0+13.98 s.
- K2's gate compared the k2a copy with B3c's copy by pixels (0 % differing), so that the key could follow its read within 1 s. B3c's copy was taken by b3c.ps1, which clicks Tally's title first so that Tally draws as active. The screen-tool screenshots were one before and one after that copy.
- In K2 the C+7 read ran at C+8.6 s, after the k2a copy; in K the T0+12 read ran at T0+13.98 s; in S, T0+3's read found no box yet (FIRST came at T0+6.9 s). A box under Tally was never met, so the minimise route was not used.
- Part D not run (above).
- Process and port listings are pushed cut to their port-9001 lines. The bk0 folder listings, the backup itself, the launch files, the HA window list, s-box-text.txt and the tool answers that hold the edition word stay on the lab PC by size and sha256 (kept-local.txt).

## Not measured
The review, release, settlement and ledger boxes; `y` as a key; keys other than `n`, Down, Alt, Tab and Esc; a box drawn under another window; a second monitor, Windows 11, other scaling and more than one sitting; macOS; part D (W2).
