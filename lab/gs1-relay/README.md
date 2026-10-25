# gs1-relay: a serial, read-only recording relay for the GS1-L1 tool reads (steps 6 to 8)

A replacement for the relay that recorded the purchase register's captures, which is no longer available. Python
standard library only; no package to install. It forwards only the read requests ComplyEaze Bridge sends for
`list_companies`, `purchase_register` and `gst_by_counter_ledger`, refuses everything else (an Import above all)
without forwarding it, and records every request and answer byte for byte. Read the header of `gs1_relay.py` for
the exact rule. The gate: a forwarded request must match, in full, one of the requests in `templates/` (the
repository's register request fixtures, byte for byte, which its tests compare with what the code sends), with only
the company name, 8-digit dates and a read planner's AlterID span allowed to differ. Before that, every element, attribute and `$$` function of a forwarded request must be on its allowlist
(taken from the requests Bridge sends for these reads), no namespace is allowed, and a request body above 1 MiB, or a
client that stops mid-request for 60 seconds, is refused. The request to Tally itself has no time limit (an
abandoned request can leave the gateway stuck), so a stuck Tally holds the relay: that is a STOP under the sheet.

Headers: a forwarded request carries the client's own `Content-Type` lines unchanged, and none when the client sent
none (a `GET /status` may send none). Tally chooses its answer's encoding from that header (a `/status` asked with
`charset=utf-16` is answered in UTF-16), so the relay never substitutes its own. It adds only `Host` and, on a POST,
`Content-Length` (the client's own length), and no other header. The self-test's fake Tally answers differently for
each `Content-Type` and fails the test if the relay changes, adds or drops one, on `GET /status` and on POSTs.

Templates added for builds whose voucher read names `REFERENCEDATE` (pull request 1867's head
`796e94f18e5af783b291ddf3b5e60ec19132df9a` among them; the older fixtures end the field list at `PARTYGSTIN` and are
kept for builds that send that). Both were rendered by that commit's own builders, not written by hand:

| Template | Bytes | sha256 | Blob | Source at `796e94f1` |
| --- | --- | --- | --- | --- |
| `reference_date_class_window_request.utf16le.xml` | 3124 | `d73bda05e5caa52e8fbfb5116f7674db1ec3c309eaf7869b6c9e5ea86dcc2ad4` | `8ebe64a6f285d80266ad63745f4444404973190d` | The class-resolved voucher window `gst_by_counter_ledger` sent, byte for byte, when replayed against the recorded answers of `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/register-e2e/`. With `,REFERENCEDATE` removed its sha256 is `34847acdf64088170371e86f5aff33e991753778290dfe47122ff0498a64b90c`, the recorded window request that `src-tauri/src/agent_register_server_tests.rs` pins. |
| `reference_date_entry_window_request.utf16le.xml` | 1740 | `91abfe080fec2056de2e40b171e7df1362dae77774e2dc62fc85ad9f9b17b715` | `133ab33ddca962f2a20b0fc0b287a587baf09b6f` | The entry-wildcard voucher window (read when a window is empty, for the day either side): `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-reference-date-20260420.request.xml` (blob `a4ec5e86205501084268ffa0bb77d95428c5b5d8`, saved as UTF-8) re-encoded as sent, with a byte-order mark and UTF-16LE. Its sha256 is the as-sent sha256 that file's `vouchers-reference-date.PROVENANCE.md` records, and it equals what the builder renders for that company and day. |

A census of every request `gst_by_counter_ledger` sends at that commit (the replay above, 104 requests, plus the
builders' renders of the empty-window and divided-window reads) found each one allowed by a template here.

## Before the sitting
1. Check every file against `SHA256.txt` (`Get-FileHash -Algorithm SHA256 FILE`).
2. `python -I gs1_relay.py --self-test`. EXPECT `self-test: ok (10 forwarded, 41 refused, none of the refused reached
   the target)` and exit 0; anything else = do not use the relay.

## Running it (steps 6 to 8 of GS1-L1)
1. In its own console: `python -I gs1_relay.py --listen-port 9011 --record C:\bridge-lab\gs1\relay\run-1`
   (the folder must be new or empty; the relay refuses to start otherwise). It prints the line it listens on.
2. Point the build of pull request 1867 at the relay: in the lab config's line for `bridge-lab-gs1`, set the Tally
   port to 9011 (the relay), not 9001. Restart that server in the assistant host. The relay forwards to
   127.0.0.1:9001 itself, and to no other port.
3. Run steps 6 to 8. Every request appears as a line in `manifest.tsv`. A line whose verdict is `refused:...` means
   the tool sent something the relay does not allow: the tool's call then fails, which the sheet already treats as
   a STOP; keep the folder and report the line.
4. After step 8: Ctrl+C in the relay's console, set the port back to 9001, and hand back the whole recording folder
   with `manifest.tsv` (every file is listed there with its bytes and sha256).

The operator's own MARKS and LEDGERS reads go straight to 9001, as before; they are not part of the recording.
