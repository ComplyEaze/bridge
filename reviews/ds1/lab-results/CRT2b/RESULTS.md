# CRT2b answers: sheet CRT2 as fixed (ea931452), run again on item 16's build, for #2390

**The fixed sheet ran as written, with no error, and both answers hold.** The new `bridge_mcp.exe` asks for no C runtime DLL (0 of 10 imports in the family). It started and answered `tools/list` with 32 tools, and it loaded every module the 0.7.0 control loaded except `VCRUNTIME140.dll`. The steps 3 to 5 block was run twice, unchanged:
- **Run B, the one that tests the fix.** PowerShell was started from a console whose input encoding is UTF-8 with a mark: preamble 3 bytes before the sheet's line, 0 after. This is the condition under which item 16's as-written run got `Parse error`; here both files answered.
- **Run A.** PowerShell was started from a console whose input encoding is IBM437: preamble 0 before and 0 after. This run did not test the fix, because there was no mark to remove.

Every value below is M, measured by this run, on 9 Nov 2026, 11:03Z to 11:06Z, on Windows 10 Home Single Language 10.0.19045, 64-bit (`evidence/s0.txt`).

## 1. The input
- Sheet `lab/2390-static-runtime-sheet` at ea931452: `README.md` and `static-runtime-run.md` are equal to its `SHA256.txt`.
- The build is item 16's: run `<CI run id>` on `feat/2390-static-runtime` (head 51ad5595a5da3dcfbb8a2d715d9174cfbf177d65), artifact `<CI artifact id>`. The files are the ones item 16 took from GitHub and kept on the lab PC. Nothing was downloaded.
- Three hashes, each equal to the queue's:
  - artifact zip: 60181141 B, sha256 d5bc5aed099b3720d65242fb816443a50dfb470b21b2c685db9e8328df376924;
  - `bridge-tally.mcpb`: 16635419 B, sha256 f52d8315278220ac698399287b09d2ec029f008da037aed11d2f15ded84920dd (`Get-FileHash`: equal);
  - `bridge_mcp.exe` inside it: sha256 1e9a2328dee09365bdd787ce7c143a997c9eb23c092105e3b4801e444e6a6035 (`evidence/s0.txt`, `evidence/s1.txt`).
- No Tally, no install, no uninstall, no setting changed. TallyPrime stayed at Select Company with no company loaded.

## 2. Step 1
| file | bytes | sha256 |
|---|---|---|
| `bridge_mcp.exe` (new) | 35,735,552 | `1e9a2328dee09365bdd787ce7c143a997c9eb23c092105e3b4801e444e6a6035` |
| `pdfium.dll` | 7,211,520 | `79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118` |

The new file is 3,030,016 B larger than 0.7.0's 32,705,536 B. Run as one script, step 1's `Get-FileHash` line printed its two rows blank: PowerShell formatted them into the `Name, Length` table that the line before it opened. The same `Get-FileHash` line was run once more on its own to record the values (`evidence/s1-hash.txt`). In a console, where each line is formatted alone, the rows print.

## 3. Step 2
Read with `Get-PeImports` (`scripts/reader.ps1`, CRT1's reader, a byte copy). `dumpbin` is not on PATH (`evidence/s2.txt`).
- `bridge_mcp.exe`: 10 lines: advapi32.dll, api-ms-win-core-synch-l1-2-0.dll, bcryptprimitives.dll, combase.dll, CRYPT32.dll, KERNEL32.dll, ntdll.dll, shell32.dll, user32.dll, ws2_32.dll. **Runtime-family names: 0.**
- `pdfium.dll`: 4 lines: ADVAPI32.dll, GDI32.dll, KERNEL32.dll, USER32.dll. **Runtime-family names: 0.**

## 4. Steps 3 and 4 (`evidence/s3-s5-utf8-host.txt` for run B, `evidence/s3-s5.txt` for run A)
| | run B (UTF-8 host) | run A (IBM437 host) |
|---|---|---|
| preamble bytes before, after | 3, 0 | 0, 0 |
| new file: exit; initialize | `exit code 0`; answered, `serverInfo` bridge-tally `0.7.0`, protocol 2025-06-18 | the same |
| new file: tools; modules | 32; 27 | 32; 28 |
| 0.7.0: exit; initialize | `exit code 0`; answered, bridge-tally `0.7.0` | the same |
| 0.7.0: tools; modules | 30; 28 | 30; 29 |
| `VCRUNTIME140.dll` in the 0.7.0 list | yes | yes |
| `NOTE: the server answered with an error` | not printed | not printed |

- The new file's 32 tools are the 0.7.0 file's 30 plus `stock_by_batch` and `stock_by_godown`.
- The new file's 27 modules in run B: advapi32.dll, bcrypt.dll, bcryptprimitives.dll, bridge_mcp.exe, combase.dll, CRYPT32.dll, GDI32.dll, gdi32full.dll, IMM32.DLL, KERNEL32.DLL, KERNELBASE.dll, msvcp_win.dll, msvcrt.dll, ntdll.dll, OLEAUT32.dll, profapi.dll, RPCRT4.dll, sechost.dll, SHCORE.dll, shell32.dll, shlwapi.dll, ucrtbase.dll, user32.dll, win32u.dll, windows.storage.dll, Wldp.dll, ws2_32.dll.
- Run A's lists hold `apphelp.dll` as well, for both files. Run B's do not (`evidence/runs-compare.txt`). This is recorded, not judged.

## 5. Step 5
- **Loaded by the new file only:** none, in both runs. **Loaded by the 0.7.0 file only:** `vcruntime140.dll`, in both runs.
- **Modules the new file loaded that import the runtime family:** 9 in each run, all Windows' own. None imports `ucrtbase.dll`, `vcruntime` or `msvcp140` by name; each imports `api-ms-win-crt-*` sets:
  - combase.dll, CRYPT32.dll, OLEAUT32.dll, profapi.dll, Wldp.dll: private, runtime, string;
  - gdi32full.dll: math, private, runtime, string;
  - msvcp_win.dll: locale, math, private, runtime, string;
  - shell32.dll, windows.storage.dll: math, private, runtime, string, time.
  - (each an `api-ms-win-crt-*-l1-1-0.dll`).
- `bridge_mcp.exe` is skipped by the sheet's loop; its own table (step 2) holds none.
- `System32\ucrtbase.dll`: ProductVersion 10.0.19041.3636, FileDescription "Microsoft® C Runtime Library". Run A's capture shows the ® as `r`, through that console's code page.

## 6. One line
The new file does not ask for the C runtime (0 runtime-family imports; 0.7.0 asked for `VCRUNTIME140.dll` and eleven `api-ms-win-crt-*`). It started and answered `tools/list` with 32 tools in both runs of the fixed sheet, including the run from a host whose input carries a byte-order mark. It loaded every module 0.7.0 did except `VCRUNTIME140.dll`. `ucrtbase.dll` in its list comes in through Windows modules that import the `api-ms-win-crt-*` sets.

## Findings
1. **The fixed step 3 works where the first version failed.** On a host with UTF-8 input and a 3-byte mark, the sheet's line set the preamble to 0, and both files answered `initialize` and `tools/list`.
2. **On a host whose input encoding has no mark, the line changes nothing** (0 before, 0 after). Such a run does not by itself show the fix; record which host a run came from.
3. **Step 1's two `Get-FileHash` rows print blank when step 1 runs as one script** (PowerShell's table formatting). Typed line by line in a console, they print.

## Not established here
A start on a PC with no Visual C++ runtime installed; any tool call beyond `tools/list`; the installers in the same artifact.

## Files
- `evidence/`: each step's output as captured; `runs-compare.txt`; the start and end times of both runs.
- `scripts/`: `reader.ps1` (CRT1's, a byte copy), `s0-s2.ps1` (the input check, step 1 with the handed path filled in, the control's unpack, step 2), `s3-s5.ps1` (the sheet's steps 3 to 5 block, line for line, after loading the reader). The pushed `s0-s2.ps1` has the lab folder replaced (`substitutions.txt`).
- Kept on the lab PC, by size and sha256 (`kept-local.txt`): the artifact zip, the `.mcpb`, the unpacked exe files, the 0.7.0 bundle copy and the stage script.
