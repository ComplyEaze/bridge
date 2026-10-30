# Row E (#2119 B): the desktop client's configuration restored, 30 Oct 2026 (UTC)

A follow-up note to this row's answers (da4015419b253cb8a40144ac35ebcc68b0686274). It records how long the row's configuration stayed live and how it was put back. Nothing is sent to Tally.

## How long the row's configuration stayed live
- **Set up:** 29 Oct 2026, 14:27:22Z to 14:28:27Z (`restart-log.txt`, setup lines): the desktop client's configuration swapped to exactly the sheet's two entries (the before-copy's sha256 65a4e50524a732ebc900beef07ff62383c495ddbc73546429a76ac448d1aea22 kept), the ComplyEaze Bridge extension turned off in its settings file (the before-copy's sha256 9ced1574ed877f4c5521cbc720e8063ee93b4e5902983a0717cc8086f3d35c0f kept), and the client quit and started again. The client rewrote the configuration on start: sha256 b461df0f8c56002d2806c209c2e84b4380fa999879afe7bd817ccb8d0f00b633 (the same entries).
- **Still live before the restore** (read 2026-10-30T03:57:43Z, configuration sha256 b461df0f...): configuration sha256 b461df0f8c56002d2806c209c2e84b4380fa999879afe7bd817ccb8d0f00b633; extension settings sha256 9456d68f1b6c157190a870c4c5a71346e08998d4a01288235d544076fdeb77ac (the extension off).
- **Restore started:** 2026-10-30T03:57Z (about), on the run lead's word to restore at 04:00 UTC (09:30 IST), after every other row of the sitting was pushed. The row's configuration was therefore live for about 13.5 hours, from 14:28Z on 29 Oct.
- **During that time** the lab's own session served the night's rows through its own stdio clients (each row's own build, launched by the row's scripts); the desktop client's configured servers were not used for any other row.

## How it was put back
`scripts/restore-desktop.ps1` (the lab's helper, on the owner's word of 29 Oct 2026, "you do it on my behalf"): it writes the two before-copies back byte for byte (the configuration, expect sha256 65a4e505...; the extension settings, expect 9ced1574...), records both hashes in `restart-log.txt`, and starts `scripts/restart-desktop.ps1` detached with a 20 s delay: it asks the client to close, ends any client process still running after the request (as at the setup), ends only orphaned `bridge_mcp.exe` processes of the two entries the row had turned off, and starts the client again by its AppID.

The restart ends the lab's own session on this PC (it runs inside the client). The hashes after the restore and the restart lines are in the lab PC's `restart-log.txt`; if the session resumes after the restart, they are added here by a forward commit.

## After the restore (added 30 Oct 2026 by a forward commit, after the lab's session came back)
The restore's lines of `restart-log.txt` (times UTC):
- `2026-10-30T03:58:39.446Z restore live config before restore sha256 b461df0f8c56002d2806c209c2e84b4380fa999879afe7bd817ccb8d0f00b633`
- `2026-10-30T03:58:39.477Z restore config restored sha256 65a4e50524a732ebc900beef07ff62383c495ddbc73546429a76ac448d1aea22 (expect 65a4e505...); extension settings sha256 9ced1574ed877f4c5521cbc720e8063ee93b4e5902983a0717cc8086f3d35c0f (expect 9ced1574...)`
- `2026-10-30T03:58:39.587Z restore restart-desktop.ps1 started detached (20 s delay)`
- `2026-10-30T03:58:40.070Z restore start; waiting 20 s`
- `2026-10-30T03:59:00.171Z restore <desktop client> processes before: 14`
- `2026-10-30T03:59:05.292Z restore ended 14 <desktop client> still running after the close request`

Read at 2026-10-30T05:54Z, after the client had restarted and the lab's session had come back:
- the configuration: sha256 7eb72fa41ca330916f5a6c6149ac292787b66d84fcc5d05bbeba9b83a07e6f49. The client rewrote the restored before-copy (65a4e505...) on start, as it did at the setup. Compared with the before-copy by a JSON parser: the configured servers are the same; top-level keys added 0, removed 0, with a changed value 1.
- the extension settings: sha256 9ced1574ed877f4c5521cbc720e8063ee93b4e5902983a0717cc8086f3d35c0f (the before-copy, byte for byte).
