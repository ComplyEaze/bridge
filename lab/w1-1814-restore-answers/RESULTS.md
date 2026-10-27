# Results: #1814 W1, a restore remembered across a restart, on master fd6f7c0d and TallyPrime 7.2, 27 Oct 2026. Steps 0 to 10 PASS

The sheet is `lab/write-path-rows-7-2/W1-1814-RESTORE-REMEMBERED.md` at 81bb5fef9c3c8b57627b5934bcdca8abd0b75b73.

## Build and setting
- `bridge_mcp.exe` of master fd6f7c0d, sha256 63641231787492201133443cb1a887705c7137f495a9205ffd33c2d0068f7484 at both launches. It reports `bridge-tally` 0.6.0.
- Two launches over MCP stdio, both with the same exe, the same launch configuration and the same data folder (`client-log.txt`): the first from 06:29:31 to 08:16:50 UTC (steps 0 to 8), the second from 08:17:39 to 08:21:29 UTC (steps 9 and 10). The configuration held `BRIDGE_TERMS_ACCEPTED`, host 127.0.0.1, port 9001, the row's own new data folder and `BRIDGE_AGENT_ENABLE_WRITES`; `_ENABLE_BATCH_POST`, `_MAX_BYTES` and `_REDACTION` were absent.
- TallyPrime 7.2, Silver. The book: BRIDGE TRANSFER PAIR LAB (company number 100066, GUID 71ec7ca4-c7b1-4f14-b5a5-7cb140bb56c5), loaded alone. Day D is 20 Aug 2026.
- BK0, taken by Tally's Backup screen before step 1: `TDBK1800_100066.001`, 27,871 bytes, sha256 aa316567d5f7bbf8465077845659528ca7455e3451ec6c7334dc386617ba01a6. Hashed again before each restore: equal. It stays on the lab PC.

## Result by step

| step | act | read | result |
|---|---|---|---|
| 0 | none | marks V0 7, M 211; no voucher on D | PASS |
| 1 | post P1 (Receipt, 1.00, D, "W1 P1") | `posted_verified`; V1 8; P1 = Receipt 1, MasterID 8, GUID ...-00000008 | PASS (see the approval windows below) |
| 2 | post P2 (Receipt, 2.00, D, "W1 P2") | `posted_verified`; V2 9; P2 = Receipt 2, MasterID 9, GUID ...-00000009 | PASS |
| 3 | BK0 restored over the book (Replace) | V3 7 = V0; one company | PASS |
| 4 | none | batch journal: 16 records, 0 `rollback_seen` | PASS |
| 5 | `verify_import` P2 | `verification_incomplete`; `post_span_binding` `book_rolled_back`, current mark 7, expected at least 9, `rollback_recorded` true, `other_batches_covered` [P1]; `unverified_vouchers[0].status` `book_rolled_back`; journal 18 records, one `rollback_seen` (P2) | PASS |
| 6 | K1 keyed on Tally's voucher screen: Receipt, D, Dr TLab Bank A 1.00, Cr TLab Suspense 1.00, "W1 P1" | V4 8; K1 = Receipt 1, MasterID 8, AlterID 8, GUID ...-00000008: the same GUID and MasterID as P1 | PASS, discriminating |
| 7 | `verify_import` P1 | `verification_incomplete`; `post_span_binding` `book_rolled_back`, `mark_read` 7, `seen_by` P2; no `rollback_recorded` and no `other_batches_covered` anywhere; `unverified_vouchers[0].status` `matching_content_observed` (diffs []); journal 19 records, still one `rollback_seen` | PASS |
| 8 | `verify_import` P1 again | identical to step 7 in every field named there; journal 20 records, one `rollback_seen`; H8 ae40b87c | PASS |
| 9 | the server stopped and launched again; `verify_import` P1 | H8 = H9a = H9b = ae40b87ce8c6e4b1e02c0a58d59aab125f32af17743281dedf96cd02770f3ef1; the answer identical to step 7; journal 21 records, one `rollback_seen` | PASS |
| 10 | BK0 restored over the book (Replace) | V 7 = V0, M 211; no voucher on D | PASS |

Every `verify_import` answer of steps 5, 7, 8 and 9 carries `isError` true with `result.error.code` `import_reconciliation_required` beside the full result: the build marks a batch that verifies as other than posted as needing reconciliation and the server sets `isError` from `result.error`. The fields the sheet names are all in the result.

K1 and P1 differ only in the order of their two amount lines (P1: Bank A first; K1: Suspense first) and in the bank allocation Tally's voucher screen asks for on a bank line (K1: Cheque/DD, no instrument number, defaults accepted); the voucher read does not carry the allocation.

## The approval windows of step 1
P1 was posted on its third approval window. The first two windows closed by time-out without a recorded answer (`import_approval_timed_out`, `attempt_recorded` false; answers 006 to 008 and 010 to 012); the marks read after each (009, 013) were 7 / 211, unchanged. The third window was answered within its first call (014, 24.0 s) and posted. P2's window was answered within its first call (018, 25.1 s). The PC holds no record of a click on the two timed-out windows; the server's error stream was not kept for the first launch.

## Files
- `answers/NNN-*.json`: every call's answer whole, with its arguments, send time and round trip; each sha256 is in `client-log.txt` and `SHA256.txt`.
- `journal-log.txt`: the batch journal's sha256, size, record count and `rollback_seen` count at steps 4, 5, 7, 8 and 9 and at the end. Its 08:17:28 line was taken while a first relaunch failed before the program started (a mistyped script path; no server ran); H9b is the 08:17:40 line, after the second launch had initialised.
- `notes.txt`: one line per step with UTC times.
- Kept on the lab PC, listed by sha256: BK0 (above); the screen captures `shot-K1-filled.jpg` 8c1e854754eda9890e482f31bad3f247525ee2936566f55611d3f3c96f6ee0bb, `shot-P1-window-3-dpi.png` 96aadee8a667b750ea002189779d88767df8eed76da04c1a54d9133de21259d1, `shot-P1-window.png` 5ac263be3f8771cc7e3726ff5cda7dcd37fa553bbb31b1ceafc22608091b3cb9 (a capture that covered only part of the screen), `shot-orphan-check-dpi.png` 61c9b5398a046aa7895ff9c6ae1291c5252ac9df37b826815e163fbe3b704865, `shot-test-dpi.png` e9122a64088710afd1bd0de25b0bc69c1a7a79aa89ef7e6b87a2a5534c512c6f; the data folder.
