# CRT2 answers: the Windows MCP server with the C runtime linked in, for #2390

**The new `bridge_mcp.exe` asks for no C runtime DLL (0 of 10 imports in the family), and it started and answered `tools/list` with 32 tools. The second half holds only in a diagnostic run, not the sheet's own run.** Run as written, the sheet's `Invoke-Mcp` got a JSON-RPC `Parse error` for its first line from both the new file and the 0.7.0 control, and a tool count of 0. On this PC, Windows PowerShell 5.1 writes the child's standard input as UTF-8 with a 3-byte byte-order mark, so the first JSON line reaches the server with a BOM in front of it. With the same function and only that changed (UTF-8 with no mark, `scripts/diag.ps1`), both files answered `initialize` and `tools/list` normally. This is a finding about the sheet's harness on this host, not about either build. Every value below is M, measured by this run, on 9 Nov 2026, 10:30Z to 10:45Z.

## 1. The run and the input
- Windows 10 Home Single Language, version 10.0.19045, build 19045, 64-bit (`evidence/s0-windows.txt`).
- The input, from the queue entry (the queue wins where the sheet waits for values): `unsigned-bundle-smoke-windows-latest` from the dispatched CI run `<CI run id>` (workflow_dispatch, success) on branch `feat/2390-static-runtime`, head 51ad5595a5da3dcfbb8a2d715d9174cfbf177d65, artifact `<CI artifact id>` (both ids as the queue entry gives them). The sheet says it downloads nothing. The lab took the artifact from GitHub because the queue names the run and artifact and gives the hashes to check.
- Three hashes, each equal to the queue entry:
  - artifact zip: 60181141 B, sha256 d5bc5aed099b3720d65242fb816443a50dfb470b21b2c685db9e8328df376924, also equal to the digest GitHub lists for the artifact;
  - `bridge-tally.mcpb`: 16635419 B, sha256 f52d8315278220ac698399287b09d2ec029f008da037aed11d2f15ded84920dd (`Get-FileHash`: equal, `evidence/s0-input-hash.txt`);
  - `bridge_mcp.exe` inside it: sha256 1e9a2328dee09365bdd787ce7c143a997c9eb23c092105e3b4801e444e6a6035.
- Sheet `lab/2390-static-runtime-sheet` at caca0d73: `README.md` and `static-runtime-run.md` equal to its `SHA256.txt`. No Tally, no install, no uninstall, no setting changed. TallyPrime stayed at Select Company with no company loaded, as item 14 left it.

## 2. The unpacked files (step 1)
| file | bytes | sha256 |
|---|---|---|
| `bridge_mcp.exe` (new) | 35,735,552 | `1e9a2328dee09365bdd787ce7c143a997c9eb23c092105e3b4801e444e6a6035` |
| `pdfium.dll` (new bundle) | 7,211,520 | `79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118` |

The new `bridge_mcp.exe` is 3,030,016 B larger than 0.7.0's 32,705,536 B. `pdfium.dll` is byte-equal to the 0.7.0 bundle's, as CRT1 recorded it (`evidence/s1-files.txt`).

## 3. The import tables (step 2)
Read by CRT1's `Get-PeImports` (`scripts/reader.ps1`, a byte copy). `dumpbin` is not on PATH (`evidence/s2-dumpbin.txt`).
- **`bridge_mcp.exe` (new):** 10 lines: advapi32.dll, api-ms-win-core-synch-l1-2-0.dll, bcryptprimitives.dll, combase.dll, CRYPT32.dll, KERNEL32.dll, ntdll.dll, shell32.dll, user32.dll, ws2_32.dll. Runtime-family names (`VCRUNTIME`, `MSVCP`, `api-ms-win-crt-`, `ucrtbase`): **0**. For 0.7.0, CRT1 read 22 names, 12 of them in that family. `api-ms-win-core-synch-l1-2-0.dll` is a core API set, not one of the `api-ms-win-crt-` sets (`evidence/s2-imports-bridge_mcp.txt`).
- **`pdfium.dll`:** 4 lines: ADVAPI32.dll, GDI32.dll, KERNEL32.dll, USER32.dll. Runtime-family names: **0** (`evidence/s2-imports-pdfium.txt`).

## 4. The start (steps 3 and 4), as written
- New file, `Invoke-Mcp` as written: `exit code 0`; `InitializeReply` `{"error":{"code":-32700,"message":"Parse error"},"id":null,"jsonrpc":"2.0"}`; `ToolCount` 0; `FirstTools` empty; `Modules` shortened by `Format-List` to `{advapi32.dll, apphelp.dll, bcrypt.dll, bcryptprimitives.dll...}` (`evidence/s3-start-new.txt`).
- 0.7.0 control: the release `.mcpb` the lab kept, 15,598,419 B, sha256 `291a4ed9…`, equal to CRT1's. A copy was unpacked in its own scratch folder; its `bridge_mcp.exe` is 32,705,536 B, sha256 `48d2f743…`, equal to CRT1's (`evidence/s4-control-hash.txt`). `Invoke-Mcp` as written gave the same as the new file: `exit code 0`, the same `Parse error`, `ToolCount` 0, the same shortened module list (`evidence/s4-start-070.txt`).
- Whether `vcruntime140.dll` is in the 0.7.0 list cannot be read from the as-written output, which `Format-List` cut after four names.
- This host's console input encoding: `utf-8`, preamble 3 bytes.

## 5. The diagnostic run, and the comparison (step 5)
`scripts/diag.ps1`: the sheet's function, with standard input as UTF-8 with no mark, and every tool and module name returned. Each file was started once more (`evidence/diag-start-new.txt`, `evidence/diag-start-070.txt`, `evidence/diag-s5-compare.txt`).
- **New file:** `exit code 0`; `initialize` answered (`serverInfo` bridge-tally `0.7.0`, protocol 2025-06-18); `tools/list` 32 tools; 27 modules: advapi32.dll, bcrypt.dll, bcryptprimitives.dll, bridge_mcp.exe, combase.dll, CRYPT32.dll, GDI32.dll, gdi32full.dll, IMM32.DLL, KERNEL32.DLL, KERNELBASE.dll, msvcp_win.dll, msvcrt.dll, ntdll.dll, OLEAUT32.dll, profapi.dll, RPCRT4.dll, sechost.dll, SHCORE.dll, shell32.dll, shlwapi.dll, ucrtbase.dll, user32.dll, win32u.dll, windows.storage.dll, Wldp.dll, ws2_32.dll.
- **0.7.0:** `exit code 0`; `initialize` answered (`serverInfo` bridge-tally `0.7.0`); `tools/list` 30 tools; 28 modules: the same 27 and `VCRUNTIME140.dll`.
- **Loaded by the new file and not by 0.7.0:** none. **Loaded by 0.7.0 and not by the new file:** `VCRUNTIME140.dll`.
- `ucrtbase.dll` is in both lists (recorded, not judged: Windows components load it). `msvcp_win.dll` and `msvcrt.dll` are Windows' own and in both lists.
- The as-written list for the new file began with `apphelp.dll`, which the diagnostic list does not hold. That is one start against another, read at a different moment; it is recorded, not judged.
- Tools in the new file and not in 0.7.0: `stock_by_batch`, `stock_by_godown`. None the other way. The branch's server still reports version `0.7.0`.

## 6. One line
The new file does not ask for the C runtime (0 runtime-family imports; 0.7.0 asked for `VCRUNTIME140.dll` and eleven `api-ms-win-crt-*`). It started and answered `tools/list` with 32 tools in the diagnostic run, and loaded every module 0.7.0 did except `VCRUNTIME140.dll`. The sheet's own run, as written, got a parse error from both files on this host because of the BOM on standard input. That is a finding about the sheet's harness.

## Findings
1. **The sheet's `Invoke-Mcp` does not run as written in Windows PowerShell 5.1 on this PC.** `ProcessStartInfo` gives the child's standard input the console input encoding, here UTF-8 with a BOM. The server answers the first line with `-32700 Parse error` and `id` null, so the tool count reads 0. The 0.7.0 control fails the same way, so this is the harness, not the build. Setting `[Console]::InputEncoding` to UTF-8 with no BOM before the start is the one change the diagnostic made.
2. **`Format-List` shortens the module list** to four names and `...` unless `$FormatEnumerationLimit` is raised. Step 4's question (is `vcruntime140.dll` in the list?) cannot be answered from the as-written output. The diagnostic sets it to -1.

## Not established here
A start on a PC with no Visual C++ runtime installed (this PC has it, as CRT1 found); any tool call beyond `tools/list`; the installers in the same artifact (`msi/`, `nsis/`), which were not opened.

## Files
- `evidence/`: the files named above.
- `scripts/reader.ps1` (CRT1's reader, a byte copy), `scripts/diag.ps1`.
- Kept on the lab PC, by size and sha256 in `kept-local.txt`: the artifact zip, the new `.mcpb`, the unpacked new `bridge_mcp.exe`, the 0.7.0 `.mcpb` and its unpacked `bridge_mcp.exe`, and the stage script.
