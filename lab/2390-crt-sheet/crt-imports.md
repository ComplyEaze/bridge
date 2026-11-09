# Sheet: which runtime DLLs the Windows package imports (reads only), for #2390

Purpose: find out whether the Windows files need the Visual C++ runtime at all, and which file asks for it. Nothing here starts Tally, installs anything, downloads anything or changes a setting. It only reads files already on the PC. Record every output in full; do not tidy it.

Marks used in the result: M = measured by this run.

## Before you start
- Use the Windows lab PC that already holds the built bundle. Note the date, the Windows version (`winver`), and the bundle's version and sha256 (from its `.sha256` file).
- Do not open TallyPrime for this sheet.

## Step 1. Find the files
In PowerShell (no administrator needed), find the unpacked bundle. Claude Desktop keeps extensions under the user profile; search both places and write down every hit.

```powershell
Get-ChildItem -Path $env:APPDATA, $env:LOCALAPPDATA -Recurse -Include bridge_mcp.exe,pdfium.dll,bridge.exe -ErrorAction SilentlyContinue | Select-Object FullName, Length
```

If nothing is found, use the folder the `.mcpb` file was unpacked into (a `.mcpb` is a zip; the files are `bin\x86_64-pc-windows-msvc\bridge_mcp.exe` and `bin\x86_64-pc-windows-msvc\pdfium.dll`). Write down the paths you use. `bridge_mcp.exe` is the file Claude Desktop starts; `bridge.exe` is the desktop app and is measured only if present.

## Step 2. The hash of each file
```powershell
Get-FileHash -Algorithm SHA256 -Path "<path to bridge_mcp.exe>", "<path to pdfium.dll>"
```
Add `bridge.exe` if present. Compare `bridge_mcp.exe` and `pdfium.dll` with the hashes in the release's own `.sha256` or the build record, and write down whether they match.

## Step 3. The imports of each file
First try `dumpbin`, which is present only where Visual Studio tools are installed:
```powershell
dumpbin /dependents "<path>"
```
If `dumpbin` is not found, do not install it. Paste this reader into PowerShell and run it for each file; it reads the file's import tables and prints DLL names.

```powershell
function Get-PeImports([string]$Path) {
  $b = [IO.File]::ReadAllBytes($Path)
  $pe = [BitConverter]::ToInt32($b, 0x3C)
  if ([Text.Encoding]::ASCII.GetString($b, $pe, 2) -ne "PE") { throw "not a PE file: $Path" }
  $sections = [BitConverter]::ToUInt16($b, $pe + 6)
  $optSize = [BitConverter]::ToUInt16($b, $pe + 20)
  $opt = $pe + 24
  if ([BitConverter]::ToUInt16($b, $opt) -ne 0x20B) { throw "not a 64-bit (PE32+) file: $Path" }
  $table = $opt + $optSize
  function ToOffset([uint32]$rva) {
    for ($i = 0; $i -lt $sections; $i++) {
      $s = $table + 40 * $i
      $va = [BitConverter]::ToUInt32($b, $s + 12); $raw = [BitConverter]::ToUInt32($b, $s + 16); $ptr = [BitConverter]::ToUInt32($b, $s + 20)
      $size = [Math]::Max([BitConverter]::ToUInt32($b, $s + 8), $raw)
      if ($rva -ge $va -and $rva -lt $va + $size) { return [int]($ptr + ($rva - $va)) }
    }
    throw "rva not in any section"
  }
  function Name([uint32]$rva) {
    $o = ToOffset $rva; $e = $o; while ($b[$e] -ne 0) { $e++ }
    [Text.Encoding]::ASCII.GetString($b, $o, $e - $o)
  }
  $out = @()
  $imp = [BitConverter]::ToUInt32($b, $opt + 120)   # data directory 1: imports
  if ($imp -ne 0) {
    $d = ToOffset $imp
    while ([BitConverter]::ToUInt32($b, $d + 12) -ne 0) { $out += "import:      " + (Name ([BitConverter]::ToUInt32($b, $d + 12))); $d += 20 }
  }
  $del = [BitConverter]::ToUInt32($b, $opt + 216)   # data directory 13: delay-load imports
  if ($del -ne 0) {
    $d = ToOffset $del
    while ([BitConverter]::ToUInt32($b, $d + 4) -ne 0) { $out += "delay-load:  " + (Name ([BitConverter]::ToUInt32($b, $d + 4))); $d += 32 }
  }
  $out | Sort-Object -Unique
}
Get-PeImports "<path to bridge_mcp.exe>"
Get-PeImports "<path to pdfium.dll>"
```
The reader's offsets were checked by porting the same steps to Python and running them on one older Windows `bridge_mcp.exe` kept from an earlier build (it listed its imports, among them `VCRUNTIME140.dll` and the `api-ms-win-crt-*` set); the PowerShell text itself has not been run, so if it errors, say so and use `dumpbin` or tell the sheet's owner.

Copy the full output for each file. A name beginning `VCRUNTIME` or `MSVCP`, or `api-ms-win-crt-` or `ucrtbase`, is what this sheet is looking for; write each one next to the file that asks for it.

## Step 4. Is the runtime on this PC
```powershell
Get-Item "$env:SystemRoot\System32\vcruntime140.dll", "$env:SystemRoot\System32\vcruntime140_1.dll", "$env:SystemRoot\System32\msvcp140.dll", "$env:SystemRoot\System32\ucrtbase.dll" -ErrorAction SilentlyContinue | Select-Object Name, @{n="Version";e={$_.VersionInfo.ProductVersion}}, Length
```
Write down which of the four exist and their versions. A name missing from the output is absent, not unchecked.

## Record (save under `reviews/ds1/lab-results/CRT1/`)
1. Date, Windows version, bundle version and sha256, the paths used.
2. The sha256 of each file, and whether each matches the build record.
3. The full import output of `bridge_mcp.exe` and of `pdfium.dll` (and `bridge.exe` if present), and which tool produced it (`dumpbin` or the reader).
4. Which of `vcruntime140.dll`, `vcruntime140_1.dll`, `msvcp140.dll`, `ucrtbase.dll` exist in System32.
5. One line on each file: asks for the Visual C++ runtime (names), or does not. Anything that did not run as written is a finding, not an error to hide.

What the result decides (not part of the run): if only `bridge_mcp.exe` asks for the runtime, linking it statically is enough to remove the prerequisite; if `pdfium.dll` also asks, it is not, and the owner is asked about the runtime's redistribution terms.
