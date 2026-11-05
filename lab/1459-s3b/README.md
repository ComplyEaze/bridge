# Windows lab sitting S3b (#1459): does the path that the installed extension's `build_import_xml` shows for its import file open on disk, on a Microsoft Store (MSIX) install of the desktop client? Published 0.7.0; 5 Nov 2026. Synthetic company only. No Tally write.

Not for merge. This is the ONE sheet of the sitting. It replaces `lab/1459-s3-run-sheet` at f80fa78a for #1459. That sheet's sitting S3 ended at step 0c: its outside probe was not run, because the lab PC refused the probe file (`lab/1459-s3-answers` at a21b92f3). This sheet keeps what S3 measures and changes only how the outside reading is taken: no script file is written or read at run time (design note `lab/1459-s3-k1b-design/design-k1b-no-script-file.md` on this branch, design A). About 75 minutes; the owner about 20 of them, at the start and at row S3-1459-1.

LAB-COMMON v3 applies in full: the byte copy at `lab/1459-s3-run-sheet:lab/1459-s3-run-sheet/LAB-COMMON.md`, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`, which the lab PC already holds from S3 (check its sha256 before step 0; another hash = halt). Where it and this sheet differ, the stricter holds, except the three stated differences under Rules. In this sheet "halt" is LAB-COMMON's stop, and "the run lead" is the person LAB-COMMON names for every decision it reserves. `SHA256.txt` lists every file here with its bytes; check it before step 0.

Marks: V = read at a named source; U = not measured. Every outcome is recorded as found, never explained away.

## What it decides

Acceptance (a) of #1459, for the import file, with the extension as a person runs it: on this PC's Store install, does `result.path` of the installed extension's `build_import_xml` name a file that a process outside the package opens, whose sha256 equals `result.sha256`, and what `result.path_state` says beside it. One build, one company, posting off.

**Already known (do not re-measure; V: `lab/1584-msix-paths-answers` at 7c330007, `RESULTS.md`):** the same 0.7.0 `bridge_mcp.exe`, started from the lab's own shell (package family `none`, its writes under LA redirected), built into the package copy `Q` and showed that real path, `resolved`, opened from outside with an equal sha256 (outcome I-A). A process started outside built into the real folder `R` (O-A). The lab's inside processes carried no package identity; the extension's own process, started by the desktop client, has never built on this PC (the 1584 E rows were not run; `lab/2246-split-journal` L3 was not run; S3 ended at 0c). That is the one variable this sitting changes: the acquirer is the installed extension; the build, the company and the voucher are I1's.

**Dated deviation from S3 (5 Nov 2026):** 0c's KA identity is "the same sha256, bytes and creation time from both sides, with the same `fsutil` file ID where `fsutil` answers", in place of "the same `vol`, `id` and `final`". No compiled helper is used. Where `fsutil` does not answer on a side, the KA line is PARTIAL. Acceptance (a) is still fully answered for the import file: it is decided in row S3-1459-2 by an outside opening and an outside hash, never by a file ID or a final path; KA is only 0c's control, and the sides are told apart by the two nonces. The design note gives the reasoning in full.

## BUILD

The published release `mcp-v0.7.0` (tag commit fcfb5ff6, published 30 Oct 2026), asset `bridge-tally-mcp-v0.7.0-windows-x64.mcpb`, 15,598,419 bytes, sha256 `291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec` (V: the release's asset digest and its `.mcpb.sha256` file, read 5 Nov). Its `.mcpb.smoke.json` (V, read 5 Nov): `server_version` 0.7.0, `archive_sha256` as above, `binary_sha256` `48d2f7430b26da0d8f670eb74a27df46b99b7d45e05e102504482ddf24c8ba2d` (`bridge_mcp.exe`, 32,705,536 bytes per `lab/1584-msix-paths` PIN), `pdfium_library_sha256` `79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118`, `default_tool_count` 33.

Why this build: as `lab/1459-s3-run-sheet` gives it, unchanged (every line the row relies on is in 0.7.0 and unchanged on master 428fba57; #2246 and #2272 are not in 0.7.0 and not needed to show the path).

Source of the file: the copy the lab PC already holds from `lab/0.7.0-post-publish`, if its sha256 equals the value above (S3's search found it equal on 5 Nov; check again); otherwise the release asset, downloaded only on the run lead's yes. Copy it into the EMPTY folder `ROOT\bundle\` and hash it there before anything is installed; another hash = halt. Copy the `.mcpb` to a `.zip`, `Expand-Archive` it into `ROOT\bundle\x\`, hash `bin\x86_64-pc-windows-msvc\bridge_mcp.exe` and `pdfium.dll` against the smoke values above. Nothing runs from that folder; nothing is built on the PC.

## The code admits each trigger (read, not run)

As `lab/1459-s3-run-sheet` gives it, unchanged, at master 428fba57 and the 0.7.0 tag: the data folder is the default `LA\Bridge\agent`; `build_import_xml` runs with posting off; the file is `<batch_id>.xml` in the data folder's `imports`; `result.path` and `result.path_state` come from `std::fs::canonicalize` on Windows (`resolved` or `unresolved`; `as_is` cannot occur on this PC); why `Q` is EXPECTED; the 0.7.0 refusal `import_publication_recovery_required` that row S3-1459-1 halts on; `import_journal_split`, which this build cannot give; why the chat-side `local_data_report` is not a row. Each with its file and line in that sheet.

## THE OWNER'S ACTS (every other act is the lab operator's; each is marked OWNER in the steps)

STANDING RULE (as the earlier sheets): Terms acceptance, posting, batch posting, ledger creation and the tax-audit box are the owner's acts in every form. This sheet writes the marker word OWNER-ACCEPTS-TERMS-HERE and never a value. The lab never reads, copies or edits the desktop client's configuration file. Posting found ON is a halt (nothing here posts; no act turns it on). The owner is at the PC for O1 to O5; O5's second half is at row S3-1459-1.

O1 to O4 run ONLY if step 0d finds the installed extension is not 0.7.0 (a running `bridge_mcp.exe` whose sha256 is not `48d2f743…`, or no ComplyEaze Bridge entry). If it is 0.7.0 with the Terms box ticked, port 9001 and posting off, O1 to O4 are skipped and recorded "not needed". (S3 read the installed extension as 0.6.0 before its 0c; 0d reads it again here.)

| act | who | exactly what | the operator checks after | about (min) |
|---|---|---|---|---|
| O1 | OWNER | The desktop client, Settings, Extensions, ComplyEaze Bridge, Uninstall. Answer only the uninstall prompt. | screenshot: no ComplyEaze Bridge entry; no other extension changed | 2 |
| O2 | OWNER | Settings, Extensions, Advanced settings, Install Extension, choose `ROOT\bundle\bridge-tally-mcp-v0.7.0-windows-x64.mcpb` (hashed by the operator just before). Answer the client's install prompts; the operator screenshots each one before the owner answers it. | screenshot: one ComplyEaze Bridge entry, version label 0.7.0; whether the client called it an install, update or reinstall | 4 |
| O3 | OWNER (the Terms box only); the operator reads the rest | In the extension's own settings screen: tick the Terms box, OWNER-ACCEPTS-TERMS-HERE. The posting box and the tax-audit box stay UNTICKED (found ticked: the owner unticks them, the owner's act). Port 9001 (the lab's Tally); host a loopback name as found. If the operator cannot drive that screen, the owner types the port. | screenshot of every setting as left: Terms ticked by the owner, posting and tax audit unticked, port 9001 | 2 |
| O4 | OWNER | Quit the desktop client fully (tray icon, Quit), then start it from the Start menu. | `@(Get-CimInstance Win32_Process -Filter "Name='<client exe>'" -ErrorAction Stop).Count` is 0 between the two, where `<client exe>` is the image name of the desktop client's process recorded at 0d (an exception is an error); a process left after Quit: halt and ask, never ended by the lab. If the quit ended the operator's session, a new session gets this sheet and the record so far and resumes at 0d | 3 |
| O5 | OWNER | The permission rehearsal. (a) The operator makes its first two extension calls, `tally_status` then `list_companies` (step 0e); on each tool prompt of the client the owner clicks Allow, the option that does NOT ask again (never "once": after an allow-once the next call waits on a prompt nobody answers). (b) At row S3-1459-1 the owner is present for the one `build_import_xml` call and clicks the same option on its prompt. If the client's extension page offers a per-tool permission instead, the owner may set the three tools there; the operator records which way. | (a) both calls answered, the raw results shown, `endpoint` on port 9001, no `terms_not_accepted`; (b) the build answer received. A prompt not answered within 5 minutes = the row is VOID, never retried another way | 5 + 2 |
| total | | | | about 20 (O1 to O4 only if 0d needs them) |

## Files and folders

`LABDIR` is the lab's root folder on the lab PC, as in the earlier sheets. Working folder `ROOT` = `LABDIR\s3b-1459\` (create it; it must not exist before). New folders `run\`, `evidence\`, `bundle\` (empty). Write `ROOT\ka.txt` holding any one line (the KA control; a data file). No helper file of any kind is copied in: this sitting has no `probe.ps1`, no `client.py`, no `hold-lock.ps1`.

## Names

- `LA` = the local application-data folder of the lab's user; `F` = the desktop client's package family, read at 0b.
- `R` = `LA\Bridge\agent` (the real data folder); `Q` = `LA\Packages\F\LocalCache\Local\Bridge\agent` (the package's private copy). `R-imports`, `Q-imports`, `R-journal`, `Q-journal` as the K2 text names them (the journal is `agent-import-ledger.jsonl`).
- `X` = `LA\lab1459-k2` (the nonce folder of this sitting, new: it is not S3's); `XQ` = `LA\Packages\F\LocalCache\Local\lab1459-k2`.
- **in** = the lab session's shell, inside the desktop client's process tree. **out** = a process started by WMI, outside that tree. Which side is which is decided at 0c, not assumed.
- **THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, the book of `lab/1584-msix-paths` and `lab/2246-split-journal`. `G` = its `company_guid` from `list_companies`.
- **THE VOUCHER** (I1's input, the only change being its id): `{"company_guid": G, "vouchers": [{"bridge_txn_id": "lab1459-s3e2", "date": <books_from as YYYY-MM-DD>, "voucher_type": "Journal", "entries": [{"ledger": "Cash", "amount": "1.00", "side": "Dr"}, {"ledger": "DN Sales", "amount": "1.00", "side": "Cr"}]}]}`. (S3 never sent it; the id is unused.)

## THE K2 READING (the outside and inside instrument of this sitting)

The command line, the same string for both sides:

```
powershell.exe -NoProfile -NonInteractive -Command "<T>"
```

`<T>` is the TEMPLATE below with its placeholders filled. The template's sha256 is `194533af3ec4c758e7b3442a6a15e5a3754cc081748b2906bfa10a67bbee350f`: the sha256 of the block's lines, each with its leading and trailing spaces removed, empty lines dropped, joined with one space, in UTF-8 with no line end. Before every start the operator computes that sha256 in memory from the template it is about to fill: another value = halt, nothing is started.

Placeholders:
- `<ROOT>`: ROOT, without the final backslash.
- `<STEP>`.
- `<SIDE>`: `in` or `out`.
- `<F>`: F.
- `<X>`: `lab1459-k2`.
- `<NONCE>`: 8 lowercase hex digits, or empty.
- `<EXTRA>`: a comma-separated list of single-quoted full paths, or empty.

**A placeholder value that contains a single quote or a double quote halts the step before the start;** nothing is escaped or rewritten to fit.

TEMPLATE:

```
$ErrorActionPreference='Stop';
$r='<ROOT>'; $o=$r+'\evidence\<STEP>-<SIDE>.json'; $x=$r+'\evidence\<STEP>-<SIDE>.exit';
function E($v){ $e=$v.Exception; while ($e.InnerException) { $e=$e.InnerException }; $e };
function M($e){ 'error '+$e.GetType().FullName+': '+$e.Message };
function N($e){ ($e -is [IO.FileNotFoundException]) -or ($e -is [IO.DirectoryNotFoundException]) };
function W($p,$t){ $b=(New-Object Text.UTF8Encoding($false)).GetBytes($t); $s=New-Object IO.FileStream($p,'CreateNew','Write','Read'); $s.Write($b,0,$b.Length); $s.Dispose() };
function O($p){ try { $s=[IO.File]::Open($p,'Open','Read','ReadWrite, Delete'); $n=$s.Length; $s.Dispose(); 'opened bytes='+$n } catch { $e=E $_; if (N $e) { 'absent' } else { M $e } } };
function L($p){ try { $c=@([IO.Directory]::EnumerateFileSystemEntries($p) | Sort-Object); [ordered]@{ total=$c.Count; shown=@($c | Select-Object -First 200 | ForEach-Object { [IO.Path]::GetFileName($_) }) } } catch { $e=E $_; if (N $e) { 'absent' } else { M $e } } };
function A($p){ try { $a=[IO.File]::GetAttributes($p); [ordered]@{ attributes=$a.ToString(); reparse_point=(($a -band [IO.FileAttributes]::ReparsePoint) -ne 0) } } catch { $e=E $_; if (N $e) { 'absent' } else { M $e } } };
function H($p){ try { (Get-FileHash -LiteralPath $p -Algorithm SHA256).Hash.ToLowerInvariant() } catch { M (E $_) } };
try {
 $la=[Environment]::GetFolderPath('LocalApplicationData'); $pl=$la+'\Packages\<F>\LocalCache\Local';
 $X=$la+'\<X>'; $XQ=$pl+'\<X>'; $R=$la+'\Bridge\agent'; $Q=$pl+'\Bridge\agent'; $KA=$r+'\ka.txt';
 $pf=$(try { [Windows.ApplicationModel.Package,Windows.ApplicationModel,ContentType=WindowsRuntime]::Current.Id.FamilyName } catch { $e=E $_; if ($e.HResult -eq 0x80073D54) { 'none' } else { 'error hresult='+$e.HResult+' '+$e.GetType().FullName } });
 $kn=[ordered]@{ absent=(O ($r+'\run\no-such-<STEP>.txt')); error=(O $r); opened=(O $KA) };
 $vi=$(if ($kn.absent -eq 'absent' -and $kn.error -like 'error *' -and $kn.opened -like 'opened *') { 'ok' } else { 'VOID' });
 $w=$null; if ('<NONCE>' -ne '') { try { [void][IO.Directory]::CreateDirectory($X); W ($X+'\<NONCE>-<SIDE>.txt') '<NONCE>-<SIDE>'; $w=$X+'\<NONCE>-<SIDE>.txt' } catch { $w=M (E $_) } };
 $fid=$(try { (& fsutil.exe file queryfileid $KA 2>&1 | Out-String).Trim() } catch { M (E $_) }); $fidx=$LASTEXITCODE;
 $ka=[ordered]@{ open=(O $KA); sha256=(H $KA); created_utc=$(try { [IO.File]::GetCreationTimeUtc($KA).ToString('o') } catch { M (E $_) }); fileid=$fid; fileid_exit=$fidx };
 $rec=[ordered]@{ step='<STEP>'; side='<SIDE>'; utc=[DateTime]::UtcNow.ToString('o'); pid=$PID; parent_pid=$(try { (Get-CimInstance Win32_Process -Filter ('ProcessId='+$PID)).ParentProcessId } catch { M (E $_) }); shell_path=(Get-Process -Id $PID).Path; package_family=$pf; folders=[ordered]@{ local=$la; roaming=[Environment]::GetFolderPath('ApplicationData'); profile=[Environment]::GetFolderPath('UserProfile') }; known_answers=$kn; instrument=$vi; written=$w; paths=[ordered]@{}; listings=[ordered]@{}; ka=$ka; extra=@() };
 foreach ($n in @(@('R-journal',($R+'\agent-import-ledger.jsonl')),@('Q-journal',($Q+'\agent-import-ledger.jsonl')))) { $rec.paths[$n[0]]=[ordered]@{ path=$n[1]; open=(O $n[1]) } };
 foreach ($n in @(@('R',$R),@('Q',$Q),@('R-imports',($R+'\imports')),@('Q-imports',($Q+'\imports')),@('X',$X),@('XQ',$XQ))) { $rec.listings[$n[0]]=[ordered]@{ folder=$n[1]; attributes=(A $n[1]); list=(L $n[1]) } };
 foreach ($p in @(<EXTRA>)) { $op=(O $p); $rec.extra+=[ordered]@{ path=$p; open=$op; sha256=$(if ($op -like 'opened *') { H $p } else { $null }) } };
 W $o ($rec | ConvertTo-Json -Depth 6); W $x '0'
} catch { try { W $x (M (E $_)) } catch {} }
```

Rules for every K2 step:
- **Out:** `$r = Invoke-CimMethod -ClassName Win32_Process -MethodName Create -Arguments @{CommandLine = $cl}`, where `$cl` is the filled command line. Record `ReturnValue`, `ProcessId` and the UTC time.
- **In:** the same `$cl`, started from the lab session's shell with `System.Diagnostics.ProcessStartInfo` (`FileName` `powershell.exe`, `Arguments` the rest of `$cl`, `UseShellExecute` false), waited for up to 90 s. Record its pid, exit code and the UTC time.
- **No file holds the text.** The operator fills the template inside the tool call itself. `$cl` is never saved to disk and never read from a file (this sheet included) at run time. The notes record, before each start, the template's sha256 and the sha256 of the filled `$cl`, both computed in memory.
- **What it writes (data only):** `ROOT\evidence\<STEP>-<SIDE>.json`, then `ROOT\evidence\<STEP>-<SIDE>.exit` (`0`, or `error <type>: <message>`). On a step with a nonce it also writes `X\<NONCE>-<SIDE>.txt` as seen by that side. Each is created new: an existing file is never overwritten.
- **A reading** exists only when the `.json` exists and parses, `.exit` holds `0`, and `instrument` is `ok`. No `.json` and no `.exit` within 90 s, or `instrument` `VOID`, = the reading was not taken (VOID), never "absent".
- In a reading, `absent` comes only from the innermost exception being "file not found" or "folder not found". Every other failure is `error <full type>: <message>`, the innermost exception's, never the wrapper's (PowerShell wraps a .NET method failure in `MethodInvocationException`). If the unwrapping fails, `absent` never appears, the known answer for an absent name reads `error`, and `instrument` is `VOID`: the reading fails loud (U until the first run).
- `package_family` is `none` only on HRESULT `0x80073D54` (`APPMODEL_ERROR_NO_PACKAGE`). It is recorded, not decided (U).
- **A refused create** (`ReturnValue` not 0), or no `.json` and no `.exit` within 90 s on a create that returned 0, is the result of that step: record the return value and the time only, halt, and try no other route. Any read of why, about a protection product, goes to the run lead only, never into a public file.

## Rules

- One Tally on port 9001, THE COMPANY loaded alone, licensed mode (Education mode = halt). One request at a time; a call in flight is never cancelled, abandoned or repeated. Nothing typed in Tally.
- **No script file.** This sitting writes no `.ps1`, `.cmd`, `.bat`, `.vbs`, `.js` or other code file, by any route, under any name, and starts nothing whose command text is read from a file. The only files it writes are data: `ka.txt`, `run\nonces-k2.txt`, the K2 outputs, the evidence files and screenshots. A write it attempts that is refused is recorded and halts the step; it is never retried under another name, extension or folder.
- **Tools sent, all through the installed extension:** `tally_status`, `list_companies`, `build_import_xml` (one call). Never `post_import`, `acknowledge_post_review`, `verify_import`, `local_data_report`, a master write, a `lab_` tool or a raw request to port 9001 other than LAB-COMMON C2's `GET /status`.
- **How the operator calls a tool.** Mode A (preferred, as `lab/1641-rows-run-sheet` Acquirers): the operator's own conversation in the desktop client lists ComplyEaze Bridge's tools; it calls the tool itself, once, with no other tool between, and shows the raw result. Mode B (only if Mode A's tool list lacks them; as `lab/2246-split-journal` step 14): in a new test chat the owner sends exactly `Call the ComplyEaze Bridge tool <name> with these arguments, and nothing else:` followed by the arguments. Either way the operator records the UTC time just before and after the call, saves the answer whole, and, if the client's MCP log for the extension can be found (its location is U), the log line for the call by its JSON-RPC id with size and sha256; a transcription from the chat is marked TRANSCRIPTION.
- Differences from LAB-COMMON (stricter elsewhere): (1) C2 d's MARKS read is replaced by `list_companies` holding exactly one company, THE COMPANY's GUID, as this sheet sends no raw request; (2) C3's raw-request rules do not apply (none is sent); (3) a tool refusal is a result: record its `code`, `cause` and the whole answer; it ends the row where the row says so.
- **Delete nothing.** The built file, its journal line and the nonce files stay where they land. No lock file, folder or leftover `.build-publication` is removed, renamed or opened for write by this sheet. The desktop client is quit only at O4; its settings file is never read or written.
- **Existence** is decided by the K2 reading (opening the path), never by `Test-Path`. An error is recorded as an error, never as "absent".
- No approval window is expected (posting is off). Bridge's approval window (title beginning `ComplyEaze Bridge —`) opening = press No by mouse, screenshot, halt. The client's own tool prompt is not that window (O5).
- **STD halt** = change nothing, take a whole-screen screenshot, save what exists with size and sha256, report to the run lead, call no tool, key nothing. After a halt no tool is called until the run lead says so.

## Setup

- **0a.** `date`; the run lead's grant for this sitting is held (its time recorded). Windows version and build (`[Environment]::OSVersion`, `(Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion').DisplayVersion`). LAB-COMMON C1 and C2 a to c with THE COMPANY loaded alone (C2 d at 0e). Save: `evidence\0a-step0.txt`. Halt: any miss = STD halt.
- **0b.** `Get-AppxPackage` with the same `Where-Object Name -like` filter on the desktop client's package name as `lab/1459-s3-run-sheet` 0b gives it: record `Name`, `PackageFamilyName` (= `F`), `Version`, `SignatureKind`, `InstallLocation`. Not exactly one = halt. Record whether `Version` equals the one `lab/1584-msix-paths-answers` recorded (yes/no). Save: `evidence\0b-appx.txt`.
- **0c. Which side is which (the K2 readings).** New nonces N3 and N4: 8 lowercase hex digits each from a cryptographic random source, neither equal to S3's N1 `341c7fc2` or N2 `afc70663` nor to each other; written to `run\nonces-k2.txt` before K2a. Then, one at a time, each as THE K2 READING says:
  - K2a (in): `<STEP>` K2a, `<NONCE>` N3, `<EXTRA>` empty.
  - K2b (out): `<STEP>` K2b, `<NONCE>` N4, `<EXTRA>` empty.
  - K2c (in): `<STEP>` K2c, `<NONCE>` empty, `<EXTRA>` empty.

  EXPECT:
  - All three readings taken: the `.json` parses, `.exit` is `0`, `instrument` is `ok`, and the template sha256 is the same for all three.
  - K2b: `package_family` `none`; `written` = `X\N4-out.txt`; its X listing holds `N4-out.txt` and not `N3-in.txt`; its XQ listing holds `N3-in.txt` and not `N4-out.txt`.
  - K2c: its X listing holds `N4-out.txt`.
  - KA, from all three readings: `open` `opened`, with the same bytes, sha256 and `created_utc`. Where `fileid_exit` is 0 on both K2a and K2b, the same `fileid`; otherwise the KA line is PARTIAL, under the dated deviation.
  - From K2b, `reparse_point` false for R, Q, R-imports, Q-imports, X and XQ wherever the folder opens. True on any of them = halt: the literal path is not the real one.

  Any other miss = STD halt: the outside reading every row rests on is not established. K2b is also the BEFORE listing: record from it `Q-imports` and `R-imports` (total, and whether an entry named `.build-publication` is present), and `Q-journal` and `R-journal` (`opened bytes=` or `absent`). K1a from S3 (`lab/1459-s3-answers`, kept on the lab PC by sha256 `cd216b71…`) is compared with K2a as an instrument cross-check only, on the folder values, `package_family`, and which of R and Q open. A difference is recorded, never a decision. Save: the three `.json` and three `.exit` files, `run\nonces-k2.txt`, and the two sha256 values per step in the notes.
- **0d. The installed extension.** Screenshot the client's Settings, Extensions: entries, version label, settings as found (Terms, posting, tax audit, host, port), read by eye, nothing typed. Then `Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'" | Format-List ProcessId, ParentProcessId, ExecutablePath` (an error is an error, never "none found"). EXPECT only processes whose parent chain reaches the desktop client's process: record the chain and the client's image name as `<client exe>`. The client may start two for one entry: record both. There must be no lab client. Hash each distinct `ExecutablePath`; if it does not exist from outside, take the same path with the roaming application-data folder replaced by `LA\Packages\F\LocalCache\Roaming`, as `lab/1641-rows-run-sheet` 0c did, and record both paths. Hash = `48d2f743…`, Terms ticked, posting and tax audit off, port 9001: skip O1 to O4 ("not needed"). Otherwise the operator hashes the bundle (BUILD) and the owner does O1 to O4; then repeat 0d: EXPECT the hash `48d2f743…`, else halt (wrong build). Posting found ON and not unticked by the owner = halt. Save: `evidence\0d-extension.txt`, screenshots (stay on the PC, listed by name, size, sha256).
- **0d2. Package identity of the extension's process (a record, not an EXPECT).** Run the PACKAGE IDENTITY OF THE SERVER instrument exactly as written in `lab/1569-p-run-sheet:lab/1569-p-run-sheet/WINDOWS-1569-P.md`, step B6, with its two known answers first (a current desktop-client PID prints 122; this shell's own `$PID` prints 15700), then each `bridge_mcp.exe` PID from 0d. It has never been run (U). It uses `Add-Type`, the one compile in the sitting. A known answer that fails makes this reading VOID (record it, continue); 122 or 15700 for the extension is recorded as found. If the PC refuses it, that refusal is its result: record it, halt 0d2 only, and go on to 0e; no other route. Save: `evidence\0d2-identity.txt`.
- **0e. OWNER O5 (a), and C2 d.** `tally_status` with `{}`: EXPECT `endpoint` on port 9001, one loaded company THE COMPANY, `education_mode` false; record `books_from`. Then `list_companies` with `{}`: EXPECT exactly THE COMPANY, its GUID as in Names; `G` = its `company_guid`. `terms_not_accepted` = halt (the owner repeats O3 and O4 once; no workaround by the lab). Also screenshot the Gateway showing THE COMPANY as the one company. Save: `evidence\0e-tally_status.json`, `evidence\0e-list_companies.json`, each with the MCP log line if found.

## Rows (run order)

### S3-1459-1: the installed extension's `build_import_xml` (#1459 E2)

- **Precondition:** 0a to 0e met (0d2 recorded, whatever its reading); K2b showed no `.build-publication` entry in `Q-imports` (one present = halt before the call: the tag's `require_settled` would refuse the build, `agent_import_persistence.rs:21-29` in the tag, which is not a path result); the owner at the PC for O5 (b); no other Bridge tool call in flight; no `bridge_mcp.exe` other than 0d's.
- **Steps:** a screenshot of the whole Tally window (a dialog = halt). One `build_import_xml` call with THE VOUCHER (Mode A or B). The owner answers the client's tool prompt (O5 b).
- **Save:** `evidence\S3-1459-1-build.json` (the answer whole; JSON escaping kept, `\\` for each backslash), the MCP log line with size and sha256, a screenshot of the answer with the `path`, `path_state` and `sha256` lines visible (stays on the PC). Record `result.batch_id` (= `B`), `result.sha256`, `result.path`, `result.path_state`, the UTC times.
- **EXPECT:** no `error`; `result.batch_id` of `bridge-` and a UUID; `result.sha256` 64 hex characters; `result.path_state` `resolved` (`visible_path.rs:76-93`) and `result.path` not starting with `\\?\` (`visible_path.rs:104-115`); `result.path` equals `Q\imports\<B>.xml` with `LA` and `F` expanded (the shown path set after the write, `agent_import.rs:1951`, the tag `:1749`).
- **Confidence of the EXPECT:** PARTIAL. Verified for the same build from the lab's inside shell (I1, `lab/1584-msix-paths-answers`); unverified for the extension's own process. `path_state` `unresolved`, or a path under `R`, is a result, not a halt.
- **Halt:** `import_publication_recovery_required` (the tag `agent_import_persistence.rs:82-85`): record it whole, do NOT build again and do NOT touch the folder; run S3-1459-2 to list what is on disk (read-only), then halt and report (the extension's later builds in `Q` are refused until a person decides, which is the run lead's word). `import_journal_split`, or `path_state` `as_is`: halt (not this build). `terms_not_accepted`: halt. Any other refusal: record `code`, `cause`, the whole answer; halt. No answer in 5 minutes: leave it, record, the row is undecided. A build is never repeated.

### S3-1459-2: the shown path, from outside and inside (#1459 E2, measured)

- **Precondition:** S3-1459-1 answered (built, or the recovery code above).
- **Steps:** form `<EXTRA>` in the tool call: first `result.path` as decoded by a JSON parser from `S3-1459-1-build.json` (omitted if the answer carries none), then `R\imports\<B>.xml`, then `Q\imports\<B>.xml`, each with `LA` and `F` expanded and each in single quotes. A path holding a quote = halt before the start. Then E2 out (`<STEP>` E2, `<SIDE>` out, `<NONCE>` empty), then E2 in with the same `<EXTRA>`, each as THE K2 READING says. No `e2-paths.txt` is written.
- **Save:** `evidence\E2-out.json`, `evidence\E2-in.json` and their `.exit` files; the filled `<EXTRA>` and both sha256 values in the notes.
- **Classify** (paths compared case-insensitively), as `lab/1584-msix-paths` E3, using each `extra` entry's `open` and `sha256`: **E-A** from out only the `Q` name opens; `result.path` equals that `Q` name; `path_state` `resolved`; the out reading's sha256 of it equals `result.sha256`. **E-B** from out only the `Q` name opens; `result.path` is the `R` name (the shown path does not exist outside). **E-C** from out the `R` name opens (the extension's file was not redirected). **E-D** anything else, described whole (including `unresolved`, and a `result.path` equal to neither name).
- **EXPECT:** E-A. Also: from the BEFORE listing (K2b) to E2 out, `Q-journal` grew and `R-journal` did not; `Q-imports` gained exactly the name `<B>.xml`; `R-imports` did not change (record totals). The in reading is a second reading (inside, reads merge: the `R` name may open the `Q` file there); it is never the decision.
- **Confidence of the EXPECT:** PARTIAL, as row 1; the classes are from reading `visible_path.rs:76-115` and the I1 result, not from a run of the extension.
- **Halt:** a VOID reading (no `.json` or `.exit` in 90 s, or `instrument` `VOID`) = this row VOID: take it once more with `<STEP>` E2b, then report. A refused create is the result: record and halt, as THE K2 READING says. E-B, E-C and E-D are findings: report at once, then S3-1459-3 still runs (it is read-only).

### S3-1459-3: a person opens it (File Explorer)

- **Precondition:** S3-1459-2 recorded; `$f` = the full path of the one `<B>.xml` that E2 out opened (the `Q` or `R` name, whichever opened; neither = skip this row and say so). `$f` holding a double quote = halt.
- **Steps:** an out step with no file: `Invoke-CimMethod -ClassName Win32_Process -MethodName Create -Arguments @{CommandLine = 'explorer.exe /select,"' + $f + '"'}`; record `ReturnValue`, `ProcessId` and the UTC time (an outside process; whether Explorer reuses its running instance is U and does not change the reading). Screenshot the whole screen. If `result.path` differs from `$f` (case-insensitive), the same with `result.path`. Close each Explorer window by its own close button.
- **Save:** the screenshot (stays on the PC; name, size, sha256 listed); a one-line transcription of the address bar and the selected name, with `<home>` and `<family>` in place of the real strings.
- **EXPECT:** by eye, the address bar shows the parent folder of `$f` and `<B>.xml` is selected; for `result.path`, the same folder and file (E-A). A "can't find" message or a default folder for `result.path` = the shown path does not open: recorded (E-B).
- **Confidence of the EXPECT:** PARTIAL (V: `lab/1569-p-run-sheet` D4 wrote this step; it was never run).
- **Halt:** an Explorer dialog the step does not name: screenshot, press nothing, halt. A refused create: record its return value and time, halt. The file is never opened in its default program, in Tally or by any Import screen.

### S3-1459-4: close

- **Steps:** `Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'"` (`ProcessId`, `ParentProcessId`, `ExecutablePath`): EXPECT only the extension's, hash still `48d2f743…`. LAB-COMMON C10: THE COMPANY shut by Tally's own Shut Company, one `tally.exe` owning 9001, `GET /status` 200, a screenshot with no company loaded. No request after the shut. The 0.7.0 extension stays installed, Terms as the owner left it, posting off; whether to put back an earlier extension is the owner's word, recorded as said (none is put back by this sheet).
- **Save:** `evidence\S3-1459-4-close.txt`.
- **Halt:** a process or a screen unlike the EXPECT: do nothing further and report exactly what is there.

**What this sitting leaves behind, never posted:** the `lab1459-s3e2` batch and its journal line (expected in `Q`), the K2 nonce files under `X` or `XQ`, the client's tool permissions as the owner set them. Listed in `kept-local.txt`. No later sheet may post this batch; a later build on this company uses another `bridge_txn_id`.

## Not in this sitting

As `lab/1459-s3-run-sheet` lists it, unchanged: the chat-side `local_data_report --show-paths` (the former E1); which folder the installed desktop app uses, and the lock files of the real folder (the 1,403-file question); a second version of the desktop client; the lock rows of #1584 and the rows of #2233 (out of scope by the run lead's word of 5 Nov 2026); #293; Windows 11, a non-Store install, a master build, proofs and bank-statement files. Also: the final path, volume serial and 128-bit file ID of `probe.ps1`, the known folders with redirection flags, and the lease counts (the dated deviation; none is needed for acceptance (a)).

## Never run

Resetting, repairing or removing the Store package of the desktop client, by anyone, in any sitting. Deleting, moving or merging either data folder, a journal, an import file or a leftover transaction folder. Opening a built file in Tally's Import screen. Writing or starting any script or code file (Rules).

## Evidence

A new branch `lab/1459-s3b-answers`, folder `lab/1459-s3b-answers/` (LAB-COMMON C11), pushed only after the pre-push scan and the run lead's yes; no `Co-Authored-By` or "Generated with" line in any commit:
- `RESULTS.md`: the setup readings (0b version and signature kind, 0c classification and KA line with its mark, 0d build identity and whether O1 to O4 ran, 0d2 identity, VOID or refused), the S3-1459-1 answer fields as printed (substituted), the E-class with each name's out-side opening and sha256, the journal and imports deltas, the Explorer reading, what was VOID or not run and why. Each finding marked verified, partial or unverified.
- Every answer, K2 JSON and `.exit` file, `notes.txt` (UTC times, one line per step, the template and filled sha256 values), `kept-local.txt`, `SCREENSHOTS.txt` (names, sizes, sha256; the captures stay local), `substitutions.txt`, and `SHA256.txt` (`# sha256  bytes  file`).
- **Every pushed file is a substituted copy**, as `lab/1584-msix-paths-answers` and `lab/2246-split-journal-answers` did: each `fileid` replaced by the first 12 hex digits of its sha256; the home folder by `<home>` in every form (plain, JSON-escaped, after `\\?\` or `\Device\HarddiskVolumeN\`, any 8.3 form); the user name by `<user>`; `F` by `<family>`; the desktop client's name by `<desktop client>`; the company number by `<company_number>`; by a JSON parser where the file is JSON. Each replacement recorded with the original's sha256 and bytes; originals stay on the lab PC. Before the push, a case-insensitive count of the user name, the home folder's last part and its 8.3 form over every pushed file must be 0. The built import file stays on the PC, listed by sha256.

## Report (one message)

The build identity (bundle and exe hashes, 0d), whether O1 to O5 ran and how long each took, 0c's classification and KA mark, 0d2's reading, VOID or refusal, S3-1459-1's `result.path` (substituted), `path_state`, `sha256`, `batch_id`, the E-class, the journal and imports deltas, the Explorer reading, and a statement that nothing was posted and nothing was deleted. Confidence: one run, one computer, one Store package: PARTIAL.

## Files here (sha256 and bytes in `SHA256.txt`)

| file | source |
|---|---|
| `README.md` | this sheet |
| `SHA256.txt` | `# sha256  bytes  file` for the file above |
