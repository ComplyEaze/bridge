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

## Before the sitting
1. Check the two files against `SHA256.txt` (`Get-FileHash -Algorithm SHA256 FILE`).
2. `python -I gs1_relay.py --self-test`. EXPECT `self-test: ok (4 forwarded, 26 refused, none of the refused reached
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
