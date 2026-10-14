# Windows lab item 1641-ROWS: do the Tally port locks hold between a packaged extension and the desktop app? (#1641, Part of #1584; 15 Oct 2026). Synthetic company only. Reads only.

About 2 hours. The build under test is ComplyEaze Bridge at commit `0b2bd2de7fecca45c6fb38dfd41e43b26b127903` (pull request #1641). The acquirers are the shipped binaries only: the extension, running inside Claude Desktop from the Microsoft Store (an MSIX package), and the desktop app, installed normally (not packaged). The holder is a small PowerShell script (`hold-lock.ps1`) that takes a Windows file lock the way Bridge does; it is run once outside the package and once inside it. Nothing is written to Tally, and no Tally setting changes.

**WHAT IT DECIDES (#1641, #1584):** whether a lock held by a process on one side of the package boundary refuses a Tally request from the other side, now that each lock is also taken under `%USERPROFILE%\.complyeaze-bridge\coordination\native-dispatch-leases`, a folder Windows does not redirect for a packaged process. Also: whether the packaged extension can create that folder; whether the two sides see one file there (one NTFS file ID); and what each side shows when the folder cannot be used. Until this runs, the pull request says the Windows lock behaviour across the package boundary is checked by CI's Windows job only.

**ALREADY KNOWN (do not re-measure):** before this change, the extension's lock files sat only in the package's private copy of AppData and the desktop app's elsewhere (#1584, measured on Claude Desktop 2.31226.1.0 from the Store). A hold on one lock file is refused as busy by an acquirer that takes the same file, in tests on macOS and CI. A busy wire lock reaches a tool call as `tally_endpoint_busy` after about 10 seconds with `retry_after_s` 5 and nothing sent. `r01` and `r02` are the request files of an earlier item (company list; voucher marks), unchanged.

**NOT ESTABLISHED before this run:** that a packaged process can create the profile folder; that `Invoke-CommandInDesktopPackage` starts a process with the extension's package identity on this PC; the desktop control names used below (copied from the source at the build's commit, not seen on screen).

## Files and folders

Working folder `C:\bridge-lab\1641\` (create it; evidence goes in `C:\bridge-lab\1641\evidence\`). Put the files of this folder in it: `hold-lock.ps1` and `requests\`. Each SHA-256 must equal this table, else STOP.

| file | bytes | sha256 |
|---|---|---|
| hold-lock.ps1 | 3423 | 52a06c2263e0523aa0d3c3eb2121cfd8eeedf91f24ffec70944a837cf8315c74 |
| requests\r01-company-extent.xml | 2524 | 9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8 |
| requests\r02-marks.xml | 976 | 81892a744009b6cf462a6f8d2fd505e650f8ad022e1462b23414cf2b8b3d2d7c |

Names used below (compute `K` yourself and compare, else STOP): `K` = the SHA-256 in lower-case hex of the ASCII text `native-dispatch-port:9001`; it must equal `a905deaa97bc7d8acb82123c4057a3890b174daf0295ee5c10be17f6fa0f0a01`.
`[BitConverter]::ToString([Security.Cryptography.SHA256]::Create().ComputeHash([Text.Encoding]::ASCII.GetBytes('native-dispatch-port:9001'))).Replace('-','').ToLower()`

- `P` = `%USERPROFILE%\.complyeaze-bridge\coordination\native-dispatch-leases` (the new folder). Its lock files: `K.lock` (the dispatch lease), `K.wire.lock` (one per send), `K.snapshot.lock` (the snapshot).
- `A` = `%LOCALAPPDATA%\Bridge\agent\native-dispatch-leases` as seen from OUTSIDE the package; `Q` = the same folder inside the package's private copy, `%LOCALAPPDATA%\Packages\<PackageFamilyName>\LocalCache\Local\Bridge\agent\native-dispatch-leases`, which is also visible from outside.
- The build's lock files are empty; deleting is never needed. A folder is set aside by renaming it to the same name plus `.bak-1641` (RESTORE puts it back).

## Rules

ONE TALLY on port 9001, with BRIDGE PILOT LAB loaded alone (never a client company). Nothing typed in Tally. At most one holder at a time unless a row says two. A holder never creates a file; Bridge does. No file is deleted. **A folder is renamed only while no holder runs, no tool call is in flight and the desktop app is closed**; a rename refused with a sharing violation is STOP.
- Outside holder: `powershell.exe -NoProfile -File C:\bridge-lab\1641\hold-lock.ps1 -Path <file> -Mode Exclusive|Shared -Seconds <n> -Log C:\bridge-lab\1641\evidence\<row>-hold.log` started in its own window. Remove `C:\bridge-lab\1641\stop.txt` before each start; creating it ends every holder early.
- Inside holder: the same command run through `Invoke-CommandInDesktopPackage -PackageFamilyName <PFN> -AppId <AppId> -Command powershell.exe -Args "<the arguments above>"`. The package family name and app id come from `Get-AppxPackage` (name containing `Claude`) and its manifest (`(Get-AppxPackageManifest <package>).Package.Applications.Application.Id`). Add `-Probe` at the first inside run (step 1): it writes `%LOCALAPPDATA%\Bridge\probe-1641.txt`, which for a process with the package identity appears under `Q`'s parent in the package and NOT under the real `%LOCALAPPDATA%\Bridge`. If the probe appears under the real folder, or the command is refused, the inside rows cannot run: record it, skip the rows marked (in) and report; do not STOP the others.
- A holder row starts the holder, waits for `HELD` in its log (STOP after 30 s without it; `BUSY` or `OPEN_FAILED` is STOP), then runs the acquirer, then ends the holder (stop file) and waits for `RELEASED`. Record the UTC time of the start and end of each acquirer call.
- Extension acquirer: in a Claude Desktop conversation call the Bridge tool `tally_status` once. It takes the wire lock for each request it sends. Desktop acquirers: in the desktop app, **Check Tally Endpoint** (two requests, wire lock) and **Run read-only Core Accounting evidence read** for a one-day period on BRIDGE PILOT LAB (takes the shared lease and the snapshot lock; it first sends a canary of reads, then reads Tally only if it is allowed to start). The two control names are from the source, not yet seen on screen: record what the screen shows if they differ.
- Write each result to a file under `evidence\` (tool result text; screenshot of the desktop screen, with the licence serial and email cropped out; screenshots stay on the PC and are never committed).
- Parse nothing by substring: a tool result is read whole and its error code is the `code` field, or the message text exactly as printed.

## Steps

**0. Preflight (nothing started yet).**
 0a. Check the file hashes and `K`. Record Tally's version and that port 9001 answers.
 0b. Build identity. Record the installer's file name and sha256 for the desktop app and for the extension bundle, and that both were built from commit `0b2bd2de7fecca45c6fb38dfd41e43b26b127903` (the Windows bundle of Bridge CI run 38523573311, artifact 11875358455, sha256 `bed07851d01fac9ef6bdbbd5befe1f785b4f456e46d01045d264c7ed594dc23b`; if the PC built its own, record how its commit was read). Any other commit: STOP.
 0c. Record `Get-AppxPackage` for Claude (package family name, version, install location). No package: STOP (the item needs the Store install). Record that the desktop app is NOT a package (`Get-AppxPackage` finds no ComplyEaze Bridge package).
 0d. Send `r01` then `r02` with `curl.exe -sS --data-binary "@requests\<file>" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\<step>.xml" -w "%{http_code} %{size_download} %{time_total}\n" http://127.0.0.1:9001/` (no `--max-time`, never Ctrl+C). EXPECT HTTP 200, exactly one company BRIDGE PILOT LAB, Education mode No. Record its GUID, `ALTVCHID` and `ALTMSTID` (M0).
 0e. Known answer for the holder, on a scratch file (`New-Item C:\bridge-lab\1641\scratch\t.lock`, outside the Bridge folders): holder 1 Exclusive HELD; holder 2 Exclusive on the same file BUSY (exit 3); stop holder 1; holder 3 Shared HELD; holder 4 Shared HELD at the same time; holder 5 Exclusive BUSY. Any other result: STOP, the holder is not a valid instrument.
 0f. List P, A and Q (names, sizes, file IDs from `fsutil file queryfileid`), or "absent". Record whether `%USERPROFILE%\.complyeaze-bridge` exists.

**1. Placement, Order I: the extension creates the files first.**
 1a. With no holder, no call in flight and the desktop app closed: rename `%USERPROFILE%\.complyeaze-bridge` (if present), `A` (if present) and `Q` (if present) to `.bak-1641`. List what is now absent.
 1b. Inside probe: run one inside holder row's `-Probe` start against a scratch file (as in 0e) and read where `probe-1641.txt` landed; then remove the probe file from wherever it landed (this is the only file this item removes; it is the item's own). Result decides the (in) rows.
 1c. In Claude Desktop call `tally_status` (the C1 control below). EXPECT success. List P, A and Q now; record names, sizes and file IDs; record whether the profile folder `%USERPROFILE%\.complyeaze-bridge` was created by this call (it was absent in 1a). If the call is refused as `tally_endpoint_lock_unavailable`: record it, STOP the item (the packaged extension cannot create the folder).
 1d. File IDs: for each file in P compare `fsutil file queryfileid` run outside the package with the same command run inside it (`Invoke-CommandInDesktopPackage ... -Command cmd.exe -Args "/c fsutil file queryfileid <path> > C:\bridge-lab\1641\evidence\id-in.txt"`). EXPECT one ID per file. For `Q`'s files compare the outside view of `Q` with the inside view of `A`; EXPECT equal. A difference in P: STOP (the folder is redirected; the design fails).

**2. Controls (Order I files, no holder).**
 C1. `tally_status` from the extension. EXPECT success.
 C2. **Check Tally Endpoint** in the desktop app. EXPECT success.
 C3. The one-day Core Accounting read in the desktop app. EXPECT it starts and reaches a finished state; record the run id and the state.

**3. Contention rows, Order I.** Each holder takes the named file in P. (out) = outside holder, (in) = inside holder. For "refused" rows the whole acquirer call is recorded with its UTC start and end.

| row | holder | acquirer | EXPECT |
|---|---|---|---|
| W1 | (out) Exclusive `K.wire.lock` | extension `tally_status` | refused `tally_endpoint_busy` with `retry_after_s` 5, after about 10 s (between 8 and 12), nothing sent |
| W2 | (out) Exclusive `K.wire.lock` | desktop Check Tally Endpoint | refused `tally_endpoint_busy`, text "Another ComplyEaze Bridge window or AI client is talking to Tally right now. Try again in a few seconds. Nothing was sent to Tally." |
| W3 (in) | Exclusive `K.wire.lock` | desktop Check Tally Endpoint | the same refusal as W2 |
| W4 (in) | Exclusive `K.wire.lock` | extension `tally_status` | the same refusal as W1 |
| L1 | (out) Exclusive `K.lock` | desktop Core Accounting read | refused, text "Another ComplyEaze Bridge snapshot or Journal posting is using this Tally endpoint. Wait for it to finish."; no run starts (the canary's reads may have been sent first, on the first calendar day; record whether they were) |
| L2 (in) | Exclusive `K.lock` | desktop Core Accounting read | the same refusal as L1 |
| L3 | (out) Exclusive `K.snapshot.lock` | desktop Core Accounting read | the same refusal as L1 |
| S1 | (out) Shared `K.lock` AND, at the same time, (in) Shared `K.lock` | desktop Core Accounting read | the read starts and reaches a finished state (a shared hold does not refuse it, on either side of the boundary); record run id and state |
| S2 | (out) Shared `K.lock` | extension `tally_status` | success (a shared lease does not hold the wire lock) |

 After each refused row run the same acquirer once more with no holder: EXPECT success (the refusal left nothing held).

**4. Placement, Order II: the desktop app creates the files first.**
 4a. Set the folders aside again (rules above), then run the desktop app's **Check Tally Endpoint** FIRST. EXPECT success. List P, A and Q (names, sizes, IDs). Record where each lock file is now.
 4b. Then call `tally_status` from the extension. EXPECT success. List P, A and Q again. Record whether the extension's `A` is the same file as the desktop app's (inside view of `A` against the outside view of `A`: file ID), or a second file in `Q`. This answers the unmeasured case of #1584.
 4c. Repeat rows W1, W2 and L1 with these files.

**5. Earlier-build side (the AppData lock is still taken).** With the Order II files: (out) Exclusive on the file `K.wire.lock` in `A`, then the desktop Check Tally Endpoint. EXPECT the W2 refusal (the AppData lock still excludes, as before this change). Then (out) Exclusive on `K.wire.lock` in `A` and the extension `tally_status`: record the result either way (it is refused only if the extension's AppData file is that same file; 4b says which).

**6. The folder cannot be used.** With no holder: rename `%USERPROFILE%\.complyeaze-bridge` aside, then create a plain EMPTY FILE of that exact name, `%USERPROFILE%\.complyeaze-bridge`. Run, one by one, `tally_status` and Check Tally Endpoint. EXPECT each refused: `tally_endpoint_lock_unavailable` (desktop text: "ComplyEaze Bridge could not open its local Tally coordination file, so it sent nothing to Tally."), nothing sent. Remove that plain file and rename the folder back, then run C1 and C2 again: EXPECT success.

**7. The report of what Bridge keeps.** From outside: `bridge_mcp --local-data-report --show-paths` (the executable in the install folder; record its path); from inside: the same through `Invoke-CommandInDesktopPackage` with its output sent to `evidence\report-in.json`. EXPECT `lock_folder_paths` to name P and the AppData folder of that view, and the lock-file count to equal what step 3's listing shows for those folders (count structure of the JSON; do not search the text).

**8. After.** Send `r02` again. EXPECT `ALTVCHID` and `ALTMSTID` equal M0 (nothing was written). Then RESTORE.

## STOP

Change nothing, take a screenshot, save the logs, report, and send nothing more to Tally: a file hash, `K` or build commit not matching; no Store package; a holder that fails 0e; `BUSY` or `OPEN_FAILED` where a hold was to start; `HELD` not seen within 30 s; the profile folder not created by the extension in 1c; a different file ID for one file of P seen from the two sides; an expected refusal that is a success (the lock did not hold: save both logs, the exact UTC times and the listing, and do not retry); an expected success that is a refusal; a refusal that arrives in under 8 s or over 12 s for W1/W4; a dialog in Tally; a second company loaded; Education mode; a rename refused with a sharing violation; any result unlike an EXPECT line.

## RESTORE (run after the last step and after any STOP)

1. End every holder (create `stop.txt`, wait for `RELEASED` in each log), then remove `stop.txt`. Close the desktop app. No tool call in flight.
2. Remove the plain file named `.complyeaze-bridge` if step 6 left it.
3. For each of `%USERPROFILE%\.complyeaze-bridge`, `A` and `Q`: if a `.bak-1641` copy exists, remove the folder this item made under the real name (only lock files, all empty; list it first) and rename the `.bak-1641` copy back to the real name. If none existed before, leave the folder this item made in place and record it.
4. Remove `probe-1641.txt` wherever step 1b put it (if still present).
5. Tally: nothing was written, so nothing to restore; compare step 8's marks with M0.
6. List P, A and Q (names, sizes, IDs) and keep it with the evidence.

## Requests this item sends to Tally

Read from the source at the build's commit (not run): every request is a read (`TALLYREQUEST` Export, or `GET /status`); no Import or other write is sent by any control below. A control that is refused sends nothing unless a row says otherwise. Each send takes the wire lock for the one HTTP exchange.

| control | requests, in order | count |
|---|---|---|
| `tally_status` (extension) and **Check Tally Endpoint** (desktop) | `GET /status`; Export of collection `BridgeCompanyExtent`; only if that answer does not parse, Export `Company Report` | 2, or 3 on the fallback (the desktop button can add one more `BridgeCompanyExtent` read after the fallback: at most 6) |
| **Run read-only Core Accounting evidence read**, one day, one leaf | `GET /status`; `BridgeCompanyExtent`; `BridgeCompanyBookExtentV2` (always twice in a row); `List of Groups`; `List of Ledgers`; `List of VoucherTypes`; `BridgeVoucherExport` | about 104 sends in all: 21 for a command canary (first calendar day only), then the run (21 + 1 + 20 + 20 + 21); each further leaf adds 40; transient retries add more |
| `r01`, `r02` (curl) | company list; voucher marks | 1 each |

Lease use: `tally_status` and Check Tally Endpoint take the wire lock only, never a lease file. The Core Accounting read first sends its canary (21 sends, wire lock only, on the first calendar day), then takes the shared lease on `K.lock` and the exclusive `K.snapshot.lock` for the whole run. It never takes the exclusive dispatch lease. So a refusal by a held `K.lock` or `K.snapshot.lock` arrives AFTER the canary's reads; the run itself does not start.

## REPORT

- A row table: row, UTC start and end, the exact answer text or code, EXPECT met or not.
- The listings of P, A and Q after steps 1, 4 and 6, with file IDs, and the placement answers (1c, 1d, 4b).
- The probe result (1b), and which rows were skipped for it.
- M0 and the marks after; every evidence file with `SHA256.txt`.
Confidence: one run, PARTIAL.

**Not established:** timings under load; a Claude Desktop that is not the Store install; the case of a pull request build that differs from commit `0b2bd2de7fecca45c6fb38dfd41e43b26b127903`; an earlier build run alongside this one.
