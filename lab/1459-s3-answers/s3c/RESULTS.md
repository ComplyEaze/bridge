# S3c (#1459) on the lab PC, 5 to 8 Nov 2026: the installed 0.7.0 extension built into the package copy (class E-A)

S3c is the S3 sheet (`lab/1459-s3-run-sheet` at f80fa78a) run as written, with ROOT, fresh nonces and `c` labels as the deviation `lab/1459-s3c/DEVIATION-2026-11-06.md` (df2fe83d) sets. The outside step K1bc became K1bc2 under the addendum `lab/1459-s3c/ADDENDUM-2026-11-06-K1bc2.md` (6ab7ce11), both on `lab/1459-s3-k1b-design`.

Names used here:
- `LA` is the local application-data folder.
- `R` = `LA\Bridge\agent`, the real folder.
- `Q` = `LA\Packages\<family>\LocalCache\Local\Bridge\agent`, the package copy.
- `<B>` = `bridge-cf7abfb8-4c1e-4a95-80c9-143583fb7502`.
- "out" is a process started by WMI `Win32_Process.Create`; "in" is the lab's own shell.

Nothing was posted. `post_import` was not called, and no import file was opened in Tally. Nothing was deleted, and no process was ended.

## Setup readings

| step | reading | mark |
|---|---|---|
| 0ac, 0ac2, 0ac3 | Windows 10.0.19045, 22H2. BRIDGE CORPUS DENSE was the one company loaded, SILVER, Gateway current date Tuesday, 31-Mar-2026, no dialog. tasklist: one tally.exe (18208 on 5 Nov; 12672 after the 6 Nov restart), exit 0. netstat: 9001 LISTENING on that pid, exit 0. GET /status: exit 0, 200, 51 bytes. | verified |
| 0bc | Exactly one package of the desktop client: version 4.16336.0.0, SignatureKind Developer. This is not the version `lab/1584-msix-paths-answers` recorded (4.4212.0.0). After the 6 Nov restart the same family read 4.23118.0.0 (0dc). | verified |
| 0c | K1ac (in, N5 34e7f032): package_family none; its file was written under X and listed in the package copy XQ. K1bc (out, N6 d9cb26f9): not run, because the lab PC's security software blocked its process chain. The owner removed that software and restarted the PC. K1bc2 (out, N7 f4c872bd), WMI: package_family none; its file was written in the real folder X; it does not see the in file under X, only under XQ. K1cc (in): sees the out file in the merged view, and XQ does not hold it. KA (`ROOT\ka.txt`) opens from both sides with equal final path and volid 1405e94e87ef. **Classification: met.** Out reads and writes the real disk; in writes land in the package copy. | verified |
| 0dc | Build identity. `bridge-tally-mcp-v0.7.0-windows-x64.mcpb` 15,598,419 B, 291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec. Its `bridge_mcp.exe` 32,705,536 B, 48d2f7430b26da0d8f670eb74a27df46b99b7d45e05e102504482ddf24c8ba2d. `pdfium.dll` 79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118. Manifest port default 9000. On 6 Nov the installed extension was 0.6.0 (1e41e039…), and a second Bridge server (a lab master build, master-fd6f7c0d) ran under the same desktop client. So O1 to O4 were needed, and owner act O1b was added. | verified |
| O1 to O5 | Each by the owner's word. The screen tools cannot see the desktop client's window, so there is no screenshot.<br>**O1:** 0.6.0 removed; the answer reached the lab by 8 Nov 04:54Z; the exact time is not known.<br>**O1b:** the lab master-build server entry turned off in the desktop client's settings, about 2 minutes.<br>**O2:** 0.7.0 installed, called an install, about 4 minutes; one process at 48d2f743.<br>**O3:** Terms ticked by the owner; posting and tax audit off; port 9001; about 2 minutes.<br>**O4:** quit and restart, twice.<br>**O3 again (after 0ec):** the port field read 8988 before the change, and the owner set 9001. This is recorded as an observation; no cause is stated.<br>**O5 (a), (b):** no desktop tool prompt reached the lab for any call; the owner was present for row 1. | partial (owner's word) |
| 0dc2, 0dc2b | After each O4, only the extension's two `bridge_mcp.exe` ran (16808/10640, then 14208/6468). Both are 48d2f743…, with parent the desktop client's `.exe` from WindowsApps. No master-build process ran. | verified |
| 0d2c, 0d2c2 | Package identity, using the instrument of `lab/1569-p-run-sheet` B6. Known answers: the desktop client's exe printed 122 and the lab shell printed 15700, both met. Every `bridge_mcp.exe` printed 15700 (no package identity): the 0.6.0 and master-build ones on 6 Nov, and the 0.7.0 ones 16808 and 10640 on 8 Nov. The processes that served rows 1 to 3 (14208, 6468, after the second O4) were not read with the instrument. | verified for the PIDs read; partial for 14208/6468 |
| 0ec, 0ec2 | First call (0ec): `tally_status` was refused with `status_probe_unavailable` / `endpoint_unreachable` at `http://127.0.0.1:8988`, the halt that led to O3 again. 0ec2 after O3 and O4 again: endpoint `:9001`, one company BRIDGE CORPUS DENSE, GUID d45bc1b0-e5e3-4261-b3b2-cce3915f42d3, identity_state verified_tuple, no terms refusal. | verified |

## S3-1459-1c: the build

Call: one `build_import_xml` with THE VOUCHER (bridge_txn_id `lab1459-s3e2`, Journal, 2025-04-01, 1.00), 2026-11-08 06:51:28Z to 06:51:42Z. The answer had no error. Fields as printed, substituted:
- `result.batch_id` = `<B>`.
- `result.path` = `LA\Packages\<family>\LocalCache\Local\Bridge\agent\imports\<B>.xml` (Q). It is printed with `LA` in full, as the home folder's local application-data folder, and has no `\\?\` prefix.
- `result.path_state` = `resolved`.
- `result.sha256` = de3aed11a324b361aba8f85def5ba2f16be76b9329af85d0954b692f285cbff1.
- `endpoint_origin` `http://127.0.0.1:9001`, `voucher_count` 1, preflight `source_rows` 82.
- No `import_publication_recovery_required`.

The answer is in `evidence/S3-1459-1c-build.json`, a transcription with the company number masked. Mark: verified (the tool answer). The desktop client's screen shows the tool result collapsed, so the screenshot does not show these fields.

## S3-1459-2c: the E-class from outside (E2c)

| name | out side | in side |
|---|---|---|
| line 1, `result.path` | opens; volid 47b12b873609; sha256 = `result.sha256` | opens the same file |
| line 2, the R name `R\imports\<B>.xml` | **absent** | opens the Q file (the read merge) |
| line 3, the Q name `Q\imports\<B>.xml` | opens; same volid; sha256 = `result.sha256` | opens the same file |

**Class: E-A.** From outside only the Q name opens, `result.path` equals it, `path_state` is resolved, and the sha256 values are equal. The out probe had package_family none. Mark: verified.

Deltas from K1bc2 (before row 1) to E2c-out (after):
- Q journal 175,278 B to 176,356 B (grew by 1,078 B). R journal 1,074 B, unchanged.
- Q imports total 379 to 380. R imports total 1 to 1.
- No `.build-publication` in either imports folder.

Mark: verified for sizes and totals. A full-name diff of Q imports is not established, because the probe lists at most 200 names.

## S3-1459-3c: Explorer from outside

An out step ran `explorer.exe /select,"<result.path>"` (one step, because `result.path` is the Q name). Its `.exit` reads 1, which is explorer.exe's own exit code; the window opened.
- Address bar: `LA\Packages\<family>\LocalCache\Local\Bridge\agent\imports`, which is Q\imports.
- Selected: `<B>.xml`, one item selected of 380.

This rests on the lab's transcription and a shell read of the open window (folder, selected item, focused item, item count). The screen tool's view showed the row ticked and "1 item selected, 926 bytes". **EXPECT met.** Mark: verified. Both PNGs of this row are partial frames (display scaling) and do not show the selected row.

Recorded deviations, accepted on the recorded word:
- (a) The window was moved left by `SetWindowPos`, position only.
- (b) It was closed by posting its own close command (`WM_SYSCOMMAND` `SC_CLOSE`), because its close button lay under the desktop client's window.

Nothing was opened in its default program, and no dialog appeared.

## S3-1459-4c: close

- Only the extension's two `bridge_mcp.exe` ran (48d2f743…).
- THE COMPANY was shut by Tally's own Company menu, SHut (Ctrl+F3), with Enter on BRIDGE CORPUS DENSE. Tally showed Select Company with no company loaded and no prompt.
- One tally.exe (12672) owns 9001.
- GET /status: 200, 51 B. No XML request after the shut.
- The 0.7.0 extension stays installed, with posting off. No earlier extension was put back. The lab master-build server entry stays off; the owner may turn it back on.

Mark: verified.

## Not run, not read, VOID

- K1bc: not run; kept as not run (see 0c).
- The desktop client's settings screens: no screenshots.
- The desktop client's process count between quit and start at O4: not read.
- The MCP log line for each call: not searched (U).
- The chat-side `local_data_report` and the other rows the sheet defers: not run.
- No reading is VOID.

## What it decides

For acceptance (a) of #1459, on this one Store install (Windows 10, package 4.23118.0.0):
- The installed 0.7.0 extension's own process has no package identity, yet it wrote its import file into the package copy Q.
- It showed that Q path, `resolved`, with the file's sha256.
- From a process outside the package that path opens to the same bytes, and Explorer selects the file.
- The real-folder name of the same file does not exist.

The shown path is the one that opens. Confidence: one run, one computer, one Store package: PARTIAL.
