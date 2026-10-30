# Windows item 1584-OUTFIRST: the port lock files created first by a process OUTSIDE the Claude package (Part of #1584)

Not for merge. Runs after `README.md` of this branch (item 1584-MSIX) has reached its End, in the same setup (`C:\bridge-lab\1584`, the same helpers, the same out-step method K1 passed). If 1584-MSIX's K1 did not decide **out**, or its KH, C1s or C2s controls failed, this item does not run. **Tally is not used at all:** every call goes to a loopback port where nothing listens, so it fails after the lock is taken. No download, no install, no owner act, Claude Desktop never quit. About 30 minutes.

Marks as in `README.md`.

## What it decides

#1584 asks for "each lock file created first by each side in turn". 1584-MSIX always creates port 9001's lock files from the in side first. Lock files are named per port (V: `src-tauri/src/endpoint_coordination.rs:143-147`), and a port nobody has used has none, so this item uses port **9011** to have the OUT side create them first, without deleting anything. A Tally send takes the wire lock before it connects (V: `src-tauri/crates/bridge-tally-transport/src/lib.rs:619-628`: the status send takes `wire_lock_observed` (`:544`, which waits through `wire_lock`, `:609-617`) before `get_status_decoded_locked` sends), so the lock files are created and taken even though nothing answers. Windows uses an EXISTING real AppData file in place for a packaged process (only new files are redirected): EXPECT one file per lock from both sides, under `R` as well as `P`, so a hold on either refuses the other.

- `K2` = `45354b5d45e8c1b4f7889a4222bf8210044f23f98a2206262f922ae6a8968313` (the hex sha256 of `native-dispatch-port:9011`, computed with `shasum`).
- Probe lines are written out in full as in `README.md` (`powershell -NoProfile -NonInteractive -ExecutionPolicy Bypass -File C:\bridge-lab\1584\probe.ps1 -Side <in|out> -Step <step> -Family F -Key K2`). The probe still names the nine files `R-K.lock` … `P-K.wire.lock`, and records `lock_key` = `K2`; below they are written `R-K2.*` etc.
- A call with no holder fails with Bridge's transport error for a refused connection (record its code); a held lock gives `tally_endpoint_busy` after up to 9.5 s. The two are different codes, so a refusal is never read as busy.
- Clients: `client.py` with `launch-env.json` = the PIN's, with `"BRIDGE_TALLY_PORT": "9011"`, started as `"<python>" C:\bridge-lab\1584\client.py <row> <in|out> 9011`. Only `tally_status` is sent.

## Steps

- X0. Nothing listens on 9011: `@(Get-NetTCPConnection -State Listen | Where-Object LocalPort -eq 9011).Count` is 0 (it lists every listener, so an empty answer is not a failed query; record the command, its output and the total listener count, which must be above 0). The 9011 files are absent on both sides: probe out then in, `-Step X0 -Key K2` (all nine `R-`, `Q-`, `P-K2.*` entries `absent`). Any present = STOP (this port has history; the item does not run, and another port needs a sheet change).
- X1 (out first). New folders `x-out`, `x-in` (each with `inbox\`, `answers\`). Out step: client `x-out`, port 9011. Send `000-list`, then `001-tally-status`: EXPECT the refused-connection error, not `tally_endpoint_busy` and not `tally_endpoint_lock_unavailable`. Keep the client up.
- X2. Probe out then in, `-Step X2 -Key K2`. EXPECT `R-K2.wire.lock` and `P-K2.wire.lock` present from both sides, each one file (same `vol`+`id`), and `Q-K2.wire.lock` absent from out.
- X3. In: client `x-in`, port 9011; `000-list`, `001-tally-status`: EXPECT the refused-connection error. Keep it up. Then probe out then in, `-Step X3 -Key K2`: EXPECT still no `Q-K2.wire.lock` (the packaged process opened the existing real file in place) and the same `vol`+`id` as X2.
- X4. Contention on the out-created files (holders as in `README.md` C: own log and stop file each, `-NonInteractive`, `-Seconds 900`, the same VOID rule for a hold that ended before the call did; call only after `HELD`; stop file and `RELEASED` after the answer):

| row | holder (side, file) | call | EXPECT |
|---|---|---|---|
| XC1 | out, `R-K2.wire.lock` (the real AppData file) | `tally_status` from `x-in` | `tally_endpoint_busy`: the AppData lock alone holds in this order |
| XC1r | (released) | `tally_status` from `x-in` | the refused-connection error |
| XC2 | in, `R-K2.wire.lock` as the in shell names it | `tally_status` from `x-out` | `tally_endpoint_busy` |
| XC3 | out, `P-K2.wire.lock` | `tally_status` from `x-in` | `tally_endpoint_busy` |
| XC4 | in, `P-K2.wire.lock` | `tally_status` from `x-out` | `tally_endpoint_busy` |
| XC4r | (released) | `tally_status` from `x-out` | the refused-connection error |

- End: `stop` for both clients; no `bridge_mcp.exe` of this item left running.

## Evidence

In the same answers branch, folder `lab/1584-msix-paths-answers/out-first/`, with the same substitution and privacy rules: the X0-X3 presence and same-file table, the XC table (row, holder log line, answer code, seconds), what was VOID and why.

## Not measured here

The data files (journal, import files) created first from out: this item sends no read that writes them. Shared holds; the dispatch and snapshot lease files.
