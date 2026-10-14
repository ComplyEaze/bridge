# Windows lab item 1641-ROWS: do the Tally port locks hold between a packaged extension and the desktop app? (#1641, Part of #1584; 15 Oct 2026, version 2). Synthetic company only. Reads only.

About 3 hours. The build under test is ComplyEaze Bridge at commit `0b2bd2de7fecca45c6fb38dfd41e43b26b127903` (pull request #1641), built ON THIS PC. The acquirers are the shipped binaries only: the extension (`bridge_mcp.exe`, from the `.mcpb` bundle) running inside Claude Desktop from the Microsoft Store (an MSIX package), and the desktop app, installed normally (not packaged). The holder is a small PowerShell script (`hold-lock.ps1`) that takes a Windows file lock the way Bridge does; it is run once outside the package and once inside it. Nothing is written to Tally, and no Tally setting changes.

**WHAT IT DECIDES (#1641, #1584):** whether a lock held by a process on one side of the package boundary refuses a Tally request from the other side, now that each lock is also taken under `%USERPROFILE%\.complyeaze-bridge\coordination\native-dispatch-leases`, a folder Windows does not redirect for a packaged process. Also: whether the packaged extension can create that folder; whether the two sides see one file there (one NTFS file ID); and what each side shows when the folder cannot be used. Until this runs, the pull request says the Windows lock behaviour across the package boundary is checked by CI's Windows job only.

**ALREADY KNOWN (do not re-measure):** before this change, the extension's lock files sat only in the package's private copy of AppData and the desktop app's elsewhere (#1584, measured on Claude Desktop 2.31226.1.0 from the Store). A hold on one lock file is refused as busy by an acquirer that takes the same file, in tests on macOS and CI. `r01` and `r02` are the request files of an earlier item, unchanged.

**THE THREE KINDS OF RESULT.** A row ends as one of: MET (the EXPECT line held), NOT MET (a real result against the design: stop and report), or VOID (the instrument or the setup failed: the row proves nothing, record why, run it once more or skip it). An instrument that fails its known-answer step makes the rows that depend on it VOID (skip them, continue the others); a failed step that everything depends on is a STOP. This is stated for each such step below.

## Files and folders

Working folder `C:\bridge-lab\1641\` (create it; evidence goes in `C:\bridge-lab\1641\evidence\`). Put the files of this folder in it: `hold-lock.ps1` and `requests\`. Each SHA-256 must equal this table, else STOP.

| file | bytes | sha256 |
|---|---|---|
| hold-lock.ps1 | 3564 | e9cac8ce447ab3be72bd25419c0d0a8b83db6c5f42aa9473c3975f832076162d |
| requests\r01-company-extent.xml | 2524 | 9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8 |
| requests\r02-marks.xml | 976 | 81892a744009b6cf462a6f8d2fd505e650f8ad022e1462b23414cf2b8b3d2d7c |

`K` = the SHA-256 in lower-case hex of the ASCII text `native-dispatch-port:9001`; it must equal `a905deaa97bc7d8acb82123c4057a3890b174daf0295ee5c10be17f6fa0f0a01` (else STOP):
`[BitConverter]::ToString([Security.Cryptography.SHA256]::Create().ComputeHash([Text.Encoding]::ASCII.GetBytes('native-dispatch-port:9001'))).Replace('-','').ToLower()`

- `P` = `$env:USERPROFILE\.complyeaze-bridge\coordination\native-dispatch-leases` (the new folder), always written out in full, expanded, in commands. Its lock files: `K.lock` (the dispatch lease), `K.wire.lock` (one per send), `K.snapshot.lock` (the snapshot).
- `A` = `$env:LOCALAPPDATA\Bridge\agent\native-dispatch-leases` as seen from OUTSIDE the package (the real path); `Q` = the package's private copy, `$env:LOCALAPPDATA\Packages\<PackageFamilyName>\LocalCache\Local\Bridge\agent\native-dispatch-leases`, also visible from outside.
- Bridge's coordination roots are `$env:LOCALAPPDATA\Bridge\agent` and `$env:USERPROFILE\.complyeaze-bridge\coordination`. The "sets" this item moves are: `$env:USERPROFILE\.complyeaze-bridge`, `A`, and `Q`.

## Rules

ONE TALLY on port 9001, with BRIDGE PILOT LAB loaded alone (never a client company). Nothing typed in Tally. **No file or folder is ever deleted by this item**; a folder is moved aside only by renaming it to the exact names in "Names" below.

**Names (never improvise another).** Step 1a renames each ORIGINAL (a set that existed before this item) to `<set>.orig-1641`. Step 4a renames each Order I set to `<set>.order1-1641`. Step 6 renames the profile folder to `.complyeaze-bridge.step6-1641`. A target name that already exists: STOP (never overwrite, never pick another name).

**Listings (the instrument).** For any folder or file: `Test-Path -LiteralPath` False, then `Get-Item -LiteralPath <p> -ErrorAction Stop` raising ItemNotFoundException = ABSENT. `Get-ChildItem -LiteralPath <p> -Force -ErrorAction Stop` returning zero items = EMPTY. Any other exception = UNREADABLE = STOP. Names, sizes and file IDs go to `evidence\` files; "nothing printed" is never a result.

**Processes.** Before 1a, 4a, 6, and before every row: `Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'"` lists exactly one process (the extension's) with the path recorded in 0c; the desktop app is running only when the step says so; no other ComplyEaze Bridge process (any other `bridge_mcp.exe`, any desktop-app instance in the tray: `Get-Process` by the app's process name recorded in 0b). Any other: STOP. For the renames, quit Claude Desktop entirely and confirm both products' processes are gone, then start Claude Desktop again afterwards; a rename refused with a sharing violation is STOP.

**Holders.** Outside holder: `powershell.exe -NoProfile -ExecutionPolicy Bypass -File C:\bridge-lab\1641\hold-lock.ps1 -Path <expanded file> -Mode Exclusive|Shared -Seconds 300 -Log C:\bridge-lab\1641\evidence\<row>-<out|in>-hold.log`, started in its own window; ONE LOG PER HOLDER. The execution policy is bypassed for that process only; never change the machine's policy. Remove `C:\bridge-lab\1641\stop.txt` before each start; creating it ends every holder. Inside holder: the same arguments through `Invoke-CommandInDesktopPackage -PackageFamilyName <PFN> -AppId <AppId> -PreventBreakaway -Command powershell.exe -Args "<the arguments above, with expanded paths>"`; the package family name and app id come from `Get-AppxPackage` (name containing `Claude`) and `(Get-AppxPackageManifest <package>).Package.Applications.Application.Id`. A holder row: start the holder, wait for `HELD` in its log (30 s; `BUSY`, `OPEN_FAILED` or `INSTRUMENT_FAILED` = the row is VOID), run the acquirer, then end the holder (stop file) and wait for `RELEASED`. **A row is VOID, not a result,** if the holder's log shows `RELEASED` at a UTC time earlier than the acquirer's end, or the extension's answer carries `status_heuristic_unavailable` (the hold ended during the call).

**Acquirers.** Extension: in a Claude Desktop conversation, the prompt exactly: "Call the ComplyEaze Bridge tool tally_status exactly once, with no other tool, and do not repeat it; show me its raw result." More than one tool call in the transcript = VOID. Record the UTC time of the prompt and of the answer; the 8 to 12 second window is NOT a stop condition for an extension row (model time is in it), only a record. Desktop: **Check Tally Endpoint** (two requests, wire lock) and **Run read-only Core Accounting evidence read** (below); the stopwatch from the click to the banner for **Check Tally Endpoint** refused as busy must read 8 to 12 s (outside it: VOID, repeat once, then NOT MET). The two control names are from the source at the build's commit; if the screen differs, record what it shows.

**Core Accounting runs.** The run is never cancelled and the desktop app is never closed while a run is active; wait for the phase to read `completed`, `partial`, `failed` or `cancelled` (record which) before any next step, any STOP action and any RESTORE step. A run not finished 10 minutes after its start: touch neither the desktop app nor Tally, take a screenshot, keep waiting, report. Preconditions: the book's `ALTVCHID` (M0, step 0d) is 170 or less, else STOP (a larger book is not sized by this item); after Check Tally Endpoint select BRIDGE PILOT LAB in the desktop app; the period is the single day 2026-08-01. A run counts as MET only with phase `completed`. This item makes about three complete runs (C3, S1, 4c), roughly 330 sends in all by the source count, plus the canary of 21 reads on the first run of a calendar day (counts from reading the source, not measured). After a refusal in an L row the desktop disables the next start and **Check Tally Endpoint**: click **Refresh and confirm no active run** and confirm the dashboard shows no active run before anything else.

**Ports.** Both products default to port 9000. Set the extension's Tally port to 9001 in Claude Desktop's extension settings and the desktop app's endpoint to 127.0.0.1:9001; record both; C1's `endpoint` field must name port 9001, else STOP (the lock key depends on the port).

**curl** (reads of 0d and 8): `curl.exe -sS --data-binary "@requests\<file>" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\<step>.xml" -w "%{http_code} %{size_download} %{time_total}\n" http://127.0.0.1:9001/`; no `--max-time`, never Ctrl+C; record `$LASTEXITCODE` (non-zero = request failed, never "no rows"). **If a request has not returned after 5 minutes:** touch neither curl nor Tally; take a screenshot, keep waiting, report. Parse (never count substrings) with: `$t=[IO.File]::ReadAllText($path,[Text.Encoding]::Unicode); $t=[regex]::Replace($t,'[\x00-\x08\x0B\x0C\x0E-\x1F]',''); $x=[xml]$t; $status=$x.SelectSingleNode('/ENVELOPE/HEADER/STATUS').InnerText; $lineErrors=$x.SelectNodes('//LINEERROR').Count`. A parse error is a failed answer, never zero rows.

Screenshots stay on the PC (licence serial and email cropped out) and are never committed. A tool result is read whole; its error code is the `code` field, or the text exactly as printed on the screen's "Technical details".

## Steps

**0. Preflight (nothing started yet).**
 0a. Check the file hashes and `K`. Record Tally's version and that port 9001 answers.
 0b. Build identity. Only the installer and the `bridge-tally.mcpb` built on THIS PC from commit 0b2bd2de7fecca45c6fb38dfd41e43b26b127903 are installed. Record `git rev-parse HEAD` (must equal that commit) and `git status --porcelain` (empty) in the build tree, the sha256 of the installer and of the `.mcpb` as built, the sha256 of `bridge_mcp.exe` in the `.mcpb` and of the desktop app's exe in the build output, and the app's process name. Record the versions of both products installed BEFORE this item (to reinstall at RESTORE if the owner asks). The installer of Bridge CI run 38523573311 is NOT used (that run built the pull request's merge ref, and its recorded digest is of the whole bundle ZIP).
 0c. Record `Get-AppxPackage` for Claude (package family name, version, install location). No package: STOP (the item needs the Store install). Record that the desktop app is NOT a package. With Claude Desktop running and the extension idle: the single `bridge_mcp.exe` process's `ExecutablePath` and its sha256 must equal the sha256 from 0b (and exactly one such process exists); the desktop exe's installed sha256 must equal the build output's. A mismatch: STOP (wrong build).
 0d. Send `r01` then `r02` (curl rules above). EXPECT HTTP 200, exit 0, exactly one company BRIDGE PILOT LAB, Education mode No, STATUS 1, no LINEERROR. Record its GUID, `ALTVCHID` (M0, must be 170 or less) and `ALTMSTID` (S0).
 0e. Holder known answer, on a scratch file (`New-Item -ItemType File -Force C:\bridge-lab\1641\scratch\t.lock`, outside the Bridge folders), each holder with its own log: holder 1 Exclusive HELD; holder 2 Exclusive on the same file `BUSY win32=33` (exit 3); end holder 1 (stop file, then remove it); holder 3 Shared HELD; holder 4 Shared HELD at the same time; holder 5 Exclusive `BUSY win32=33`; end 3 and 4. Any other result, or `INSTRUMENT_FAILED`: STOP (the holder is not a valid instrument; do not change any policy).
 0f. List P, A and Q, the three sets' parents and `$env:USERPROFILE\.complyeaze-bridge` (ABSENT / EMPTY / names, sizes, file IDs); record which originals exist.
 0g. Set the ports (Ports above) and confirm the desktop app can be opened and closed cleanly.

**1. Placement, Order I: the extension creates the files first.**
 1a. Processes rule satisfied, Claude Desktop quit: rename each original set that exists to `<set>.orig-1641` (names above). List what is absent now (ABSENT, not "nothing"). Start Claude Desktop again.
 1b. Inside known answer (decides every row marked (in) and 1d): `Invoke-CommandInDesktopPackage ... -Args "<hold-lock.ps1 -Probe on the scratch file, Exclusive, -Seconds 120>"`. EXPECT `HELD` in its log. Then an outside holder Exclusive on the same scratch file: EXPECT `BUSY win32=33`; end both; then the reverse (outside first, inside second): EXPECT `BUSY win32=33`. Then from OUTSIDE the package read where `probe-1641.txt` landed: it appears under `$env:LOCALAPPDATA\Packages\<PFN>\LocalCache\Local\Bridge\probe-1641.txt` (the package's private copy) and NOT under `$env:LOCALAPPDATA\Bridge\`. Any other result (the command refused, `INSTRUMENT_FAILED`, no `HELD` in 30 s, the probe under the real folder): record it and mark every (in) row and the inside comparisons of 1d and 4b VOID; continue with the others. Never change a Windows setting to make it work. The probe file is then renamed to `probe-1641.txt.done-1641` where it landed (not deleted); if `$env:LOCALAPPDATA\Bridge` did not exist before and the probe created it outside the package, record that.
 1c. In Claude Desktop run C1 below. EXPECT success, and the answer's endpoint on port 9001. List P, A and Q, with sizes and file IDs; record whether this call created `$env:USERPROFILE\.complyeaze-bridge` (it was ABSENT or renamed in 1a). If the call is refused as `tally_endpoint_lock_unavailable`, record it and STOP the item: the packaged extension cannot create the folder.
 1d. File IDs. Instrument known answer first, from both sides, on the scratch file: output files `id-<side>-<name>.txt` made with `fsutil file queryfileid <path> 2>&1` then `echo EXIT %ERRORLEVEL%`; the outside runs directly, the inside through `Invoke-CommandInDesktopPackage ... -Command cmd.exe -Args "/c ..."`. Each output must hold exactly one line `File ID is 0x...` and `EXIT 0`, and the two IDs must be equal; otherwise the file-ID instrument is not valid: skip 1d and 4b, report, and conclude nothing about redirection. A result without a `File ID is` line is FAILED, never DIFFERENT. Only with a valid instrument: for each file in P compare the outside and inside IDs: EXPECT equal. For `Q`'s files compare the outside view of `Q` with the inside view of `A`: EXPECT equal. A different ID for a file in P: NOT MET (the folder is redirected; the design fails): STOP.

**2. Controls (Order I files, no holder).** After each, the processes rule.
 C1. `tally_status` from the extension (the prompt above). EXPECT success.
 C2. **Check Tally Endpoint** in the desktop app. EXPECT success.
 C3. The one-day Core Accounting read (rules above). EXPECT it starts and reaches phase `completed`; record the run id. If C3 is refused or fails: record it, skip the L and S rows, continue with the others.

**3. Contention rows, Order I.** Each holder takes the named file in P (expanded path), `-Seconds 300`. (out) = outside holder, (in) = inside holder (VOID if 1b failed). Order of rows: W2, W1, W3, W4, L1, L2, L3, S1, S2. W2 first, so a W1 success cannot be put down to the package boundary alone.

| row | holder | acquirer | EXPECT |
|---|---|---|---|
| W2 | (out) Exclusive `K.wire.lock` | desktop Check Tally Endpoint | refused: Technical details show `Code tally_endpoint_busy`, next step "Try again in a few seconds. Nothing was sent to Tally.", message "Another ComplyEaze Bridge window or AI client is talking to Tally right now. Try again in a few seconds."; banner after 8 to 12 s |
| W1 | (out) Exclusive `K.wire.lock` | extension `tally_status` | refused `tally_endpoint_busy` with `retry_after_s` 5 (the time is only recorded) |
| W3 (in) | Exclusive `K.wire.lock` | desktop Check Tally Endpoint | as W2 |
| W4 (in) | Exclusive `K.wire.lock` | extension `tally_status` | as W1 |
| L1 | (out) Exclusive `K.lock` | desktop Core Accounting read | refused; the screen shows "Start outcome was not confirmed. Recent durable runs were refreshed and a new start is locked until you review them. Another ComplyEaze Bridge snapshot or Journal posting is using this Tally endpoint. Wait for it to finish."; no run starts (the canary's reads cannot be observed in this item) |
| L2 (in) | Exclusive `K.lock` | desktop Core Accounting read | as L1 |
| L3 | (out) Exclusive `K.snapshot.lock` | desktop Core Accounting read | as L1 |
| S1 | (out) Shared `K.lock` AND, at the same time, (in) Shared `K.lock` (two logs) | desktop Core Accounting read | the read starts and reaches phase `completed` (a shared hold does not refuse it on either side of the boundary); record run id |
| S2 | (out) Shared `K.lock` | extension `tally_status` | success; this is a negative control (a shared lease does not hold the wire lock), not evidence for #1641 |

 After each refused row: end the holder, click **Refresh and confirm no active run**, confirm the dashboard shows none, then run C2 (the processes rule first): EXPECT success (the refusal left nothing held).

**4. Placement, Order II: the desktop app creates the files first.**
 4a. Quit the desktop app and Claude Desktop (Processes rule); rename each Order I set to `<set>.order1-1641` (names above); start the desktop app, run **Check Tally Endpoint** FIRST, EXPECT success. List P, A and Q (ABSENT / names, sizes, IDs). Start Claude Desktop.
 4b. Then run C1 from the extension. EXPECT success. List again. Record whether the extension's `A` is the same file as the desktop app's (inside view of `A` against the outside view of `A`: file ID, only with a valid instrument from 1d), or a second file in `Q`. This answers the case #1584 left unmeasured.
 4c. Run W2 and W1 with these files. Then run C3 (so `K.lock` exists in P) and wait for phase `completed`, then L1.

**5. Earlier-build side (the AppData lock is still taken).** With the Order II files: (out) Exclusive on `$env:LOCALAPPDATA\Bridge\agent\native-dispatch-leases\K.wire.lock` (the real path, from outside), then the desktop Check Tally Endpoint (dashboard showing no active run first). EXPECT the W2 refusal. Then the same holder and the extension `tally_status`: record the result either way (it is refused only if the extension's AppData file is that same file; 4b says which).

**6. The folder cannot be used.** Processes rule, both apps quit. Rename `$env:USERPROFILE\.complyeaze-bridge` to `.complyeaze-bridge.step6-1641`, then create a plain EMPTY FILE of the exact name `$env:USERPROFILE\.complyeaze-bridge`. Start the apps. Run, one by one, `tally_status` and Check Tally Endpoint. EXPECT each refused: `tally_endpoint_lock_unavailable` (desktop message: "ComplyEaze Bridge could not open its local Tally coordination file, so it sent nothing to Tally."). Then quit both apps, rename the plain file to `.complyeaze-bridge.file-1641`, rename `.complyeaze-bridge.step6-1641` back to `.complyeaze-bridge`, start the apps, run C1 and C2: EXPECT success.

**7. The report of what Bridge keeps.** The executable is `bridge_mcp.exe` (path from 0c). From outside: `<exe> --local-data-report --show-paths > evidence\report-out.json 2>&1`, then record the exit status. From inside: `Invoke-CommandInDesktopPackage ... -Command cmd.exe -Args "/c <exe> --local-data-report --show-paths > C:\bridge-lab\1641\evidence\report-in.json 2>&1 & echo EXIT %ERRORLEVEL% >> C:\bridge-lab\1641\evidence\report-in-exit.txt"`. EXPECT exit status 0 (2 unreadable, 3 incomplete: a report with either is VOID) and `folders_that_could_not_be_listed` empty; `lock_folder_paths` = [`$env:LOCALAPPDATA\Bridge\agent`, `$env:USERPROFILE\.complyeaze-bridge\coordination`] in that order from BOTH sides (the inside view prints the real LOCALAPPDATA path); `classes.locks.files` = the lock files in that view's `native-dispatch-leases` folders plus the admission lock if present, against the listing taken immediately before this step. Count structure in the JSON; never search the text.

**8. After.** Send `r02` again (curl rules). EXPECT `ALTVCHID` = M0 and `ALTMSTID` = S0 (nothing was written). After a STOP, "not compared: step 8 not run" is the entry. Then RESTORE.

## STOP

Change nothing further, take a screenshot, save the logs, report, and send nothing more to Tally; first let any running Core Accounting read or tool call finish (Core Accounting runs rule). STOP is: a file hash, `K`, build commit or process hash not matching; no Store package; the step 0e holder failing; a name in "Names" already taken; UNREADABLE; a process census not matching; a rename refused with a sharing violation; the profile folder not created by the extension in 1c; a different file ID for a file of P with a valid instrument; a NOT MET result (an expected refusal that is a success, with the holder confirmed held for the whole call: save both logs, the UTC times and the listing, and do not retry; or an expected success that is a refusal); a dialog in Tally; a second company loaded; Education mode; a Core Accounting run not finished after 10 minutes (report, keep waiting); any result unlike an EXPECT line that is not VOID.

## RESTORE (after the last step and after any STOP)

1. Let any Core Accounting run reach a terminal phase and any tool call return (never cancel, never close the app first). End every holder (stop file; wait for `RELEASED` in each log; remove `stop.txt`). Then quit the desktop app and confirm both products' processes are gone.
2. Nothing is deleted. For each set `S` (`$env:USERPROFILE\.complyeaze-bridge`, `A`, and `Q`): (a) a plain file named `.complyeaze-bridge` is put aside as `.complyeaze-bridge.file-1641` and `.complyeaze-bridge.step6-1641` is put back if present; (b) the folder now under the real name is set aside as `S.end-1641`; (c) `S.orig-1641` is renamed back to `S` where it exists. Where no original existed, the folder this item made stays under `S.end-1641` and the real name is absent, which is the starting state.
3. List every `*-1641` folder and file left (names, sizes, file IDs) and keep the listing: they are the item's own lock files (empty) and the probe file. Record whether the probe created `$env:LOCALAPPDATA\Bridge`.
4. The lab-built desktop app and extension remain installed. State their versions before (0b) and after, and whether the owner asked to reinstall the earlier ones. Record the Core Accounting run ids left in the desktop app's local mirror.
5. Tally: nothing was written, so nothing to restore; compare step 8's marks with M0 and S0 (or "not compared").
6. Reset the two ports to what they were before 0g only if the owner asks; record the values left.

## Requests this item sends to Tally

Read from the source at the build's commit (not run): every request is a read (`TALLYREQUEST` Export, or `GET /status`); no Import or other write is sent by any control below. A control that is refused sends nothing unless a row says otherwise. Each send takes the wire lock for the one HTTP exchange.

| control | requests, in order | count |
|---|---|---|
| `tally_status` (extension) and **Check Tally Endpoint** (desktop) | `GET /status`; Export of collection `BridgeCompanyExtent`; only if that answer does not parse, Export `Company Report` | 2, or 3 on the fallback (the desktop button can add one more `BridgeCompanyExtent` read after the fallback: at most 6) |
| **Run read-only Core Accounting evidence read**, one day, one leaf | `GET /status`; `BridgeCompanyExtent`; `BridgeCompanyBookExtentV2` (always twice in a row); `List of Groups`; `List of Ledgers`; `List of VoucherTypes`; `BridgeVoucherExport` | about 104 sends in all: 21 for a command canary (first run of a calendar day only), then the run (21 + 1 + 20 + 20 + 21); each further leaf adds 40; transient retries add more |
| `r01`, `r02` (curl) | company list; voucher marks | 1 each |

Lease use: `tally_status` and Check Tally Endpoint take the wire lock only, never a lease file. The Core Accounting read first sends its canary (21 sends, wire lock only, first run of a calendar day), then takes the shared lease on `K.lock` and the exclusive `K.snapshot.lock` for the whole run. It never takes the exclusive dispatch lease. So a refusal by a held `K.lock` or `K.snapshot.lock` arrives after the canary's reads; the run itself does not start. These counts are from reading the source and are not measured.

## REPORT

- A row table: row, result (MET / NOT MET / VOID with the reason), UTC start and end of the acquirer, the holder's logs, the exact answer text or code.
- The listings of P, A and Q after steps 1, 4 and 6, with file IDs, and the placement answers (1c, 1d, 4b).
- The known-answer outcomes (0e, 0c, 1b, 1d) and which rows each made VOID.
- The build identity record (0b, 0c), the process census, M0 and S0 against the marks after, every evidence file with `SHA256.txt`.
Confidence: one run, PARTIAL.

**Not established:** timings under load; a Claude Desktop that is not the Store install; a build other than commit `0b2bd2de7fecca45c6fb38dfd41e43b26b127903`; an earlier build run alongside this one; the packaged extension taking `K.lock` (only a post does; no row posts, so the dispatch lease across the boundary is shown only with the inside script as holder, L2, not with the extension binary); a book above 170 on the voucher mark; the PowerShell execution policy, Developer Mode and elevation needs of the tools used (the known-answer steps find out and nothing is changed to satisfy them).
