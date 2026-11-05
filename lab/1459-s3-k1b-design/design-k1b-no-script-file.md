# #1459 S3: taking the outside reading K1b (and K1c) with no script file on disk (design note, not a sheet)

Status: a design for review. Nothing here has been run. Every command below is marked U (never run) until a sheet runs it.

## What K1b measures, and why it must run outside

Step 0c decides which side is which before any row is read. The lab's own shell is started by the desktop client, so it runs inside the client's process tree. On this PC such a process has its writes under LA redirected into the package's private copy (Q, XQ), and its reads merge the copy with the real folder, so from inside the `R` name may open a `Q` file (`lab/1584-msix-paths-answers`). An inside reading therefore cannot tell where a file really is. K1b is the same reading taken by a process that is not in that tree, started by WMI (`Win32_Process.Create`), as a person's Explorer or another program would see the disk. It writes one nonce file under X and records whether it has package identity. It records which of the named folders and files open (R, Q, X, XQ, the two imports folders, the two journals) and lists those folders. It also opens the control file KA, which sits outside every redirected folder, from the same side. With K1a (the inside nonce) and K1c (the inside look at the outside nonce), it establishes three things: the out side writes to the real folder, the out side does not see the inside's redirected write under X, and both sides open the same KA. It is also the BEFORE listing for row S3-1459-2 (imports totals, any `.build-publication` entry, journal bytes). Every row's decision rests on "what opens from outside", so K1b cannot be replaced by any reading taken inside the tree.

## Candidates

### A (recommended). The reading given whole as plain text to `powershell.exe -Command`, started by the same WMI create

The reading K2 replaces `probe.ps1` with a plain-text command that runs without being stored. It loads no C# (`Add-Type` is not used: it compiles through files written to disk). The same text, with only the placeholders filled, is used for the in side and the out side.

Command line of the create (out) and of the in side's start, the same string:

```
powershell.exe -NoProfile -NonInteractive -Command "<T>"
```

`<T>` is the text below, joined into one line with single spaces. It contains no double quote and no `%`, so neither the command line nor cmd.exe changes it; cmd.exe is not involved. Placeholders: `<ROOT>` the working folder; `<STEP>`; `<SIDE>` `in` or `out`; `<F>` the package family read at 0b; `<X>` the name of the sheet's probe folder X under LA; `<NONCE>` 8 lowercase hex digits or empty; `<EXTRA>` a comma-separated list of single-quoted full paths, or empty.

```
$ErrorActionPreference='Stop';
$r='<ROOT>'; $o=$r+'\evidence\<STEP>-<SIDE>.json'; $x=$r+'\evidence\<STEP>-<SIDE>.exit';
function W($p,$t){ $b=(New-Object Text.UTF8Encoding($false)).GetBytes($t); $s=New-Object IO.FileStream($p,'CreateNew','Write','Read'); $s.Write($b,0,$b.Length); $s.Dispose() };
function O($p){ try { $s=[IO.File]::Open($p,'Open','Read','ReadWrite, Delete'); $n=$s.Length; $s.Dispose(); 'opened bytes='+$n } catch [IO.FileNotFoundException],[IO.DirectoryNotFoundException] { 'absent' } catch { 'error '+$_.Exception.GetType().Name+': '+$_.Exception.Message } };
function L($p){ try { $c=@([IO.Directory]::EnumerateFileSystemEntries($p) | Sort-Object); [ordered]@{ total=$c.Count; shown=@($c | Select-Object -First 200 | ForEach-Object { [IO.Path]::GetFileName($_) }) } } catch [IO.DirectoryNotFoundException] { 'absent' } catch { 'error '+$_.Exception.GetType().Name+': '+$_.Exception.Message } };
function H($p){ try { (Get-FileHash -LiteralPath $p -Algorithm SHA256).Hash.ToLowerInvariant() } catch { 'error '+$_.Exception.Message } };
try {
 $la=[Environment]::GetFolderPath('LocalApplicationData'); $pl=$la+'\Packages\<F>\LocalCache\Local';
 $X=$la+'\<X>'; $XQ=$pl+'\<X>'; $R=$la+'\Bridge\agent'; $Q=$pl+'\Bridge\agent'; $KA=$r+'\ka.txt';
 $pf=$(try { [Windows.ApplicationModel.Package,Windows.ApplicationModel,ContentType=WindowsRuntime]::Current.Id.FamilyName } catch { $e=$_.Exception; if ($e.InnerException) { $e=$e.InnerException }; if ($e.HResult -eq 0x80073D54) { 'none' } else { 'error hresult='+$e.HResult } });
 $kn=[ordered]@{ absent=(O ($r+'\run\no-such-<STEP>.txt')); error=(O $r); opened=(O $KA) };
 $vi=$(if ($kn.absent -eq 'absent' -and $kn.error -like 'error *' -and $kn.opened -like 'opened *') { 'ok' } else { 'VOID' });
 $w=$null; if ('<NONCE>' -ne '') { try { [void][IO.Directory]::CreateDirectory($X); W ($X+'\<NONCE>-<SIDE>.txt') '<NONCE>-<SIDE>'; $w=$X+'\<NONCE>-<SIDE>.txt' } catch { $w='error '+$_.Exception.Message } };
 $fid=(& fsutil.exe file queryfileid $KA 2>&1 | Out-String).Trim(); $fidx=$LASTEXITCODE;
 $ka=[ordered]@{ open=(O $KA); sha256=(H $KA); created_utc=$(try { [IO.File]::GetCreationTimeUtc($KA).ToString('o') } catch { 'error '+$_.Exception.Message }); fileid=$fid; fileid_exit=$fidx };
 $rec=[ordered]@{ step='<STEP>'; side='<SIDE>'; utc=[DateTime]::UtcNow.ToString('o'); pid=$PID; parent_pid=$(try { (Get-CimInstance Win32_Process -Filter ('ProcessId='+$PID)).ParentProcessId } catch { 'error '+$_.Exception.Message }); shell_path=(Get-Process -Id $PID).Path; package_family=$pf; folders=[ordered]@{ local=$la; roaming=[Environment]::GetFolderPath('ApplicationData'); profile=[Environment]::GetFolderPath('UserProfile') }; known_answers=$kn; instrument=$vi; written=$w; paths=[ordered]@{}; listings=[ordered]@{}; ka=$ka; extra=@() };
 foreach ($n in @(@('R-journal',($R+'\agent-import-ledger.jsonl')),@('Q-journal',($Q+'\agent-import-ledger.jsonl')))) { $rec.paths[$n[0]]=[ordered]@{ path=$n[1]; open=(O $n[1]) } };
 foreach ($n in @(@('R',$R),@('Q',$Q),@('R-imports',($R+'\imports')),@('Q-imports',($Q+'\imports')),@('X',$X),@('XQ',$XQ))) { $rec.listings[$n[0]]=[ordered]@{ folder=$n[1]; list=(L $n[1]) } };
 foreach ($e in @(<EXTRA>)) { $op=(O $e); $rec.extra+=[ordered]@{ path=$e; open=$op; sha256=$(if ($op -like 'opened *') { H $e } else { $null }) } };
 W $o ($rec | ConvertTo-Json -Depth 6); W $x '0'
} catch { try { W $x ('error '+$_.Exception.GetType().Name+': '+$_.Exception.Message) } catch {} }
```

- **Out side:** `Invoke-CimMethod -ClassName Win32_Process -MethodName Create -Arguments @{CommandLine = $cl}`, where `$cl` is the command line above with `<T>` filled in. Record the return value and the pid.
- **In side:** the same `$cl`, started from the lab session's shell with `System.Diagnostics.ProcessStartInfo` (file `powershell.exe`, arguments the rest of `$cl`, `UseShellExecute` false) and waited for up to 90 s. The two command lines are then the same string, apart from the placeholders.
- **The text never touches a file.** The operator fills the placeholders inside the tool call itself. The text is never saved to disk and never read back from a file, the sheet included, at run time. Before each start the notes record the sha256 of the filled `$cl`, computed in memory, and the sha256 of the template with its placeholders unfilled, which must be equal on every side.
- **What it writes (output data only):** `<ROOT>\evidence\<STEP>-<SIDE>.json`, then `<ROOT>\evidence\<STEP>-<SIDE>.exit` holding `0`, or `error <type>: <message>` when the reading failed part-way. On K2a and K2b it also writes the one nonce file `<X>\<NONCE>-<SIDE>.txt` (17 ASCII bytes) under LA, as `probe.ps1` did. All three are created new; an existing file is never overwritten: the create fails, and that failure goes into the `.exit` file.
- **What is not carried over from `probe.ps1`, and why:**
  - The final path in three forms, the volume serial and the 128-bit file ID through `CreateFileW`, `GetFinalPathNameByHandleW` and `GetFileInformationByHandleEx`. They need a compiled helper.
  - The known folders read with the redirection flags.
  - The lease-lock counts. This sheet does not report them.

  For KA, `fsutil file queryfileid` (a built-in program) stands in for the file ID. Whether it answers without elevation on this PC is U, and its exit code is recorded.
- **Telling success from "nothing found" (P5):**
  1. A reading exists only when the `.json` exists, parses, and `.exit` holds `0`. No `.json` and no `.exit` within 90 s is VOID: "the reading was not taken", never "absent".
  2. Inside the reading, a path is `absent` only on "file not found" or "folder not found" (the win32 2 and 3 cases). Every other failure, including access denied and a sharing conflict, is `error <type>: <message>`. Existence is decided by opening, never by `Test-Path` or `Exists`, which turn errors into False.
  3. Each reading carries its own three known answers: a name that cannot exist must read `absent`; the working folder opened as a file must read `error`; KA must read `opened`. If any one misses, `instrument` is `VOID` and that reading decides nothing.
  4. `package_family` is `none` only on the documented no-identity code (`APPMODEL_ERROR_NO_PACKAGE`, 15700, seen as HRESULT `0x80073D54`, which Windows PowerShell reads as a signed 32-bit value equal to the exception's `HResult`; U). Any other failure is `error hresult=<n>`. No known answer for "has identity" can be taken from the lab's shell, so this field is recorded, not decided (U). On this PC the inside processes also showed no identity (`lab/1584-msix-paths-answers`), so identity is not what separates the two sides.
- **One variable (P6):**
  - K1a was taken with `probe.ps1`. Comparing it with a K2 out reading would change two things at once, the side and the instrument. So K1a is not used as the partner. The in side is taken again with the same K2 text as K2a, and K2a against K2b differs by the side only: the same template, sha256, start shape and command line, with only the step, side and nonce filled differently, as `probe.ps1` allowed.
  - K1a against K2a (same side, different instrument) is then recorded as an instrument cross-check only. It covers the fields both readings hold that time does not change: the folder values, package identity, and which of R, Q, X, XQ and KA open. It is never a decision.

### B. cmd.exe built-ins on the create's command line (`cmd /c dir ...`, `echo ... > ...`, `if exist ...`)

Rejected for K1b because it cannot meet P5:
- `if exist` and `dir` exit codes do not separate "not found" from "access denied" except by matching the message text, which depends on the language.
- `echo ... > file` overwrites an existing file, so the nonce cannot be created as new.
- There is no hash without bringing in another program.

It would write only its redirected output files. It is not used anywhere in the redesign.

### C. A reading taken directly by the lab session

Rejected:
- The session's shells are inside the tree, so any reading they take is an in reading by construction.
- File Explorer, driven by the screen, is outside the tree, but it gives sight only. It cannot write a nonce, cannot hash, and cannot tell an error from absence except by a dialog. It stays a cross-check, as row S3-1459-3 already uses it, not a K1b.

## If the decision needs what A drops

Suppose the run lead keeps the 0c EXPECT "KA opens from both sides with the same `vol`, `id` and `final`" as written. Then the honest answer is that K1b needs a compiled helper, which writes source and a library to disk while it compiles, that is, a code file. Under A, KA is instead the same sha256, bytes and creation time from both sides, plus the same `fsutil` file ID where `fsutil` answers. Where it does not answer, the KA line is marked PARTIAL.

## The refusal

If the PC refuses A as well (the create's return value is not 0, or no `.json` and no `.exit` appear within 90 s), that is the result: record it, halt, and do not try another route.

## Later steps and rows

| step or row | under A |
|---|---|
| 0a, 0b | as they are |
| 0c | changes: K2a (in, `<NONCE>` N3), K2b (out, `<NONCE>` N4), K2c (in, no nonce), all with the K2 text. EXPECT: K2b `written` is `X\N4-out.txt`; K2c lists N4-out under X and not under XQ; from out, N3-in is listed under XQ and not under X; KA the same sha256, bytes and creation time from both sides (and the same `fsutil` ID where it answers). The BEFORE listing comes from K2b. K1a is kept as recorded, as a cross-check only. |
| 0d | as it is (in-session commands; no file) |
| 0d2 | text as it is, but it uses `Add-Type`, whose compile writes files to disk. If that counts as a code file, 0d2 is not run and is recorded as such. K1a's own `Add-Type` compiled on the in side at 14:48Z on 5 Nov. |
| 0e, O1 to O5 | as they are |
| S3-1459-1 | as it is; its precondition reads K2b's listing in place of K1b's |
| S3-1459-2 | changes: E2 out and E2 in use the K2 text with `<STEP>` E2 and `<EXTRA>` the three paths in single quotes, in place of `-ExtraFile`. No `e2-paths.txt`, no `.tmp` rename, no `.cmd`. The classes E-A to E-D, the EXPECT and the one E2b repeat for a VOID reading are as they are. |
| S3-1459-3 | changes: the create's command line is `explorer.exe /select,"<f>"` itself. No `.cmd` and no exit line; record the create's return value and pid. The rest as it is. |
| S3-1459-4 | as it is |
