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
