# CRT1 results: which runtime DLLs the Windows package imports (reads only), for #2390

Run on 9 Nov 2026, 07:23Z to 07:40Z, by the sheet at `lab/2390-crt-sheet` (94a6089e). The sheet's two files are equal to its `SHA256.txt`. Nothing was started, installed, downloaded or changed. TallyPrime was not touched for this sheet: it stayed at Select Company with no company loaded, as the previous row left it. Every value below is M, measured by this run.

## 1. The PC and the bundle

- **Date:** 9 Nov 2026.
- **Windows:** Windows 10 Home Single Language, 22H2, build 19045.6466. Read from the registry's CurrentVersion key in place of `winver`, which opens a window (`evidence/s0-windows.txt`).
- **The bundle:** version 0.7.0, from the installed extension's `manifest.json`. The 0.7.0 file on the lab PC, `bridge-tally-mcp-v0.7.0-windows-x64.mcpb`, is 15,598,419 B with sha256 `291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec`. That equals its own `.sha256` file. Two other copies of it on the PC have the same sha256.
- **Its entries,** read in place by `scripts/mcpb-list.py` with nothing unpacked (`evidence/s2-mcpb-070-entries.txt`). There are 9 entries, by size:

  | entry | bytes |
  |---|---|
  | `bin/x86_64-pc-windows-msvc/bridge_mcp.exe` | 32,705,536 |
  | `bin/x86_64-pc-windows-msvc/pdfium.dll` | 7,211,520 |
  | `icon.png` | 15,503 |
  | `LICENSE` | 11,356 |
  | `manifest.json` | 8,297 |
  | `NOTICE` | 6,464 |
  | `THIRD_PARTY_LICENSES.txt` | 8,418 |
  | `THIRD_PARTY_LICENSES_PDFIUM.txt` | 109,658 |
  | `THIRD_PARTY_LICENSES_RUST.txt` | 677,163 |

**Step 1, the paths.** The sheet's `Get-ChildItem` over the roaming and local application-data folders ran 07:33:32Z to 07:36:51Z and found 15 files. Its raw output stays on the lab PC, because it names the user folder (`kept-local.txt`). In safe form:

- **The unpacked bundle**, used for this sheet: the desktop client's extension folder `local.mcpb.complyeaze-contributors.bridge-tally\bin\x86_64-pc-windows-msvc\` under the local application-data folder. It holds `bridge_mcp.exe` (32,705,536 B) and `pdfium.dll` (7,211,520 B), both written 8 Nov 2026.
- **The desktop app,** `Bridge\bridge.exe` (36,892,672 B), under the local application-data folder, with its `bridge_mcp.exe` (32,714,752 B).
  - The same two files show again, byte-equal, through the desktop client's package cache.
  - bridge.exe's file version reads 0.6.1.
  - This `bridge_mcp.exe` is the desktop app's own copy, not the bundle's, and was not measured.
- **Older build outputs,** not measured: four debug and two release files under the package manager's store, and three `bridge_mcp.exe` files in mcpb smoke folders under the temp folder, all dated 9 to 14 Oct.

## 2. The hashes

| file | bytes | sha256 | against the build record |
|---|---|---|---|
| `bridge_mcp.exe` (bundle) | 32,705,536 | `48d2f7430b26da0d8f670eb74a27df46b99b7d45e05e102504482ddf24c8ba2d` | equal to the 0.7.0 `.mcpb`'s entry |
| `pdfium.dll` (bundle) | 7,211,520 | `79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118` | equal to the 0.7.0 `.mcpb`'s entry |
| `bridge.exe` (desktop app 0.6.1) | 36,892,672 | `74c1c775501dde4fb5a4c1aad1b3c1e275e5bc3699e0ac77a5e0c8533a3aea1e` | not compared: no build record for it was checked |

The 0.7.0 `.mcpb` itself equals its release `.sha256`, and the two installed bundle files equal its entries byte for byte. `Get-FileHash`'s raw output stays local, because it prints full paths.

## 3. The imports

`dumpbin` is not on this PC (`evidence/s3-dumpbin.txt`), so nothing was installed and the sheet's reader was used. `scripts/reader.ps1` is the sheet's `Get-PeImports` text, equal to it line for line. It was dot-sourced and run on each file, and it ran without error on all three. Its full output for each file is in `evidence/s3-imports-<file>.txt`. It printed no `delay-load:` line for any of the three.

- **`bridge_mcp.exe`, 22 lines:**
  - advapi32.dll, api-ms-win-core-synch-l1-2-0.dll;
  - the eleven `api-ms-win-crt-*` DLLs: convert, environment, filesystem, heap, locale, math, runtime, stdio, string, time, utility (each `-l1-1-0.dll`);
  - bcryptprimitives.dll, combase.dll, CRYPT32.dll, kernel32.dll, ntdll.dll, shell32.dll, user32.dll, **VCRUNTIME140.dll**, WS2_32.dll.
- **`pdfium.dll`, 4 lines:** ADVAPI32.dll, GDI32.dll, KERNEL32.dll, USER32.dll.
- **`bridge.exe`, 31 lines:**
  - advapi32.dll, api-ms-win-core-synch-l1-2-0.dll, api-ms-win-core-winrt-error-l1-1-0.dll;
  - the same eleven `api-ms-win-crt-*` DLLs;
  - bcryptprimitives.dll, combase.dll, comctl32.dll, CRYPT32.dll, dwmapi.dll, gdi32.dll, imm32.dll, kernel32.dll, ntdll.dll, ole32.dll, oleaut32.dll, shell32.dll, shlwapi.dll, user32.dll, **VCRUNTIME140.dll**, **VCRUNTIME140_1.dll**, ws2_32.dll.

## 4. The runtime in System32

All four are present (`evidence/s4-runtime.txt`):

| file | version | bytes |
|---|---|---|
| vcruntime140.dll | 14.50.35719.0 | 123,472 |
| vcruntime140_1.dll | 14.50.35719.0 | 47,264 |
| msvcp140.dll | 14.50.35719.0 | 553,552 |
| ucrtbase.dll | 10.0.19041.3636 | 1,046,080 |

## 5. One line on each file

- **`bridge_mcp.exe` (bundle 0.7.0):** asks for the Visual C++ runtime: `VCRUNTIME140.dll`. It also asks for the Universal CRT through the eleven `api-ms-win-crt-*` DLLs.
- **`pdfium.dll` (bundle 0.7.0):** does not. It imports only ADVAPI32, GDI32, KERNEL32 and USER32, with no `VCRUNTIME`, `MSVCP`, `api-ms-win-crt-` or `ucrtbase` name.
- **`bridge.exe` (desktop app 0.6.1, measured because present):** asks for the Visual C++ runtime: `VCRUNTIME140.dll` and `VCRUNTIME140_1.dll`. It also asks for the same eleven `api-ms-win-crt-*` DLLs.

Read against the sheet's closing note: of the bundle's two files, only `bridge_mcp.exe` asks for the runtime.

## Findings on the sheet as written

- `winver` opens a window. The Windows version was read from the registry instead.
- The sheet asks for the bundle's version and sha256 "from its `.sha256` file". The installed extension has no `.sha256`. The version came from its `manifest.json`, and the sha256 from the 0.7.0 `.mcpb` on the PC, which equals its own `.sha256`.
- Step 1's search found the desktop app's copies and older build outputs besides the bundle. The bundle's two files are named above, by folder.
- The reader's PowerShell text, which the sheet says had not been run, ran here as written, with no error.

## Kept on the lab PC

Listed by size and sha256 in `kept-local.txt`: step 1's raw search output and `Get-FileHash`'s raw output (both name the user folder), the stage script, and the 0.7.0 `.mcpb` itself.
