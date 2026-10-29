# Results: #2119 B, the Bills report requests' 60-second limit, on #2158's build, 29 Oct 2026. **S1 answered in 76.9 s with R4's totals; the limit holds on the stdio path. The host the row could drive (the desktop client 4.29.0.0's Code tab, the Code tab CLI 2.1.309) cut every tool call at 60 s: both sleep calls and D1's `outstandings`, which was cancelled after its Receivable pair. By rules (d) and (e) the limit is HELD for that host, and the call budget applies (a cut at 60 s, under 130 s). The Chat tab was not measured.**

| step | what | reading |
|---|---|---|
| 0 | one tally.exe owns 9001; `/status` 200; exe 1ab49eaf... equal; BRIDGE CORPUS DENSE loaded alone; Tally's date Tuesday 31-Mar-2026 | PASS |
| 1 | `tools/list` 30 tools, the five present; `list_companies` exactly THE COMPANY, `verified_tuple`, books from 20250401; marks M0 328, V0 29900 | PASS |
| 2 S1 | `outstandings` as_of 20260331 through A: answered in **76.875 s**, `isError` false, truncated (the client's size bound); `open_bills_total` 29,900; `receivable` 1,425,845,144.5; `payable` 0 | PASS, equal to R4 |
| 2 P1 | B's `tally_status` sent at S1+10.36 s: refused `tally_endpoint_busy` after 9.873 s | as expected |
| 3 S2 | `ledger_masters` straight after S1: answered in 0.81 s, M1 328, V1 29900 | PASS |
| 4 | S1's trail: sent 48, failed 0; Receivable posts held **34,880 and 30,081 ms**; Payable 2,264 and 3,447 ms; every other post under 20,000 ms | as expected |
| 5 H1 | sleep 130: cut at **60.06 s**; sleep 90: cut at **60.01 s**; both "SdkError: Request timed out" | not answered whole |
| 6 D1 | `outstandings` through D: cut by the host ("Request timed out"); D's record `request_cancelled` at 17:13:15.551Z, the Receivable pair answered (31,786 and 31,122 ms), then the cancel | given up by the host |
| 7 | the pre-check found D's set changed (below); on the run lead's word 14120 taken as D; A again: M2 328, V2 29900; exe equal; C10: none loaded, one tally.exe owns 9001, `/status` 200 | PASS |

## The decision
- **(a) S1 refused for the deadline:** no. **(b) S1 not answered:** no. **(c) totals differ from R4's:** no.
- **(d) the host gave D1 up:** YES, for the host measured. It showed "Error: Request timed out", and D recorded `request_cancelled`. Deviation: Desktop's own MCP log has no `tools/call` line for these calls (the Code tab reaches Desktop's servers through the SDK), so (d) is read from what the host showed and from D's own record, not from Desktop's MCP log.
- **(e) a limit of at least 90 s not shown:** YES. Neither sleep call answered whole, and D1 was cut.
- **The call budget:** the shortest cut was 60.01 s (H1's 90 s call), under 130 s, so for this host the limit is under that. The budget rule applies.

**What cut the calls (`evidence/H1-cause.txt`):** no `timeout` field in Desktop's configuration or any other configuration, and no `MCP_TOOL_TIMEOUT` variable. Neither server is a the Code tab CLI MCP server: Desktop serves its configured servers to a Code tab session through the SDK, and the cancellation names "SdkError". The 60 s is the default of that path on this PC, not the lab's configuration.

**Who acted.** The lab, on the owner's standing word of 27 Oct and, for the Desktop setup and its restore (the owner's acts in the sheet), on the owner's word in the lab's chat of 29 Oct, "you do it on my behalf".

## Deviations
1. **Setup done by the lab,** on the owner's word: the configuration swapped to exactly the sheet's two entries (the before-copy 65a4e505...), the ComplyEaze Bridge extension turned off in its settings file, and Desktop quit and restarted. Desktop rewrote the file on start (sha256 b461df0f...; the same mcpServers).
2. **The operator's session** is the lab's own session, resumed by Desktop after the restart with the row's servers loaded, not one started after setup.
3. **H1 and D1 were made from that Code tab session,** not from a Chat-tab test chat: The operator cannot be given screen control of the desktop client's own window. There was no approval dialog, Allow once or Deny; the calls carried exactly the sheet's arguments.
4. **`node --version`** was read from the lab's own shell (v24.21.0), not from one started by explorer.exe.
5. **Step 7's pre-check found D's set changed.** D's process 25312 (it served the cancelled D1 call) was gone, and a new process 14120 had started at 17:16:34Z, the moment of step 6's `egress_log` call through D. 14120 is the row's exe (sha256 equal), Desktop's child, and with 26244 the only bridge_mcp.exe. On the run lead's word it was taken as process D. **Finding about the host:** after the host cancelled D1, Desktop replaced the server instance that served the call; the next call started a new one. Process A had been started just before the listing was read, and was stopped 9 s later by its stop file with nothing sent (initialize only).

## Not measured
The Chat tab's limit; Desktop's MCP log lines for H1 and D1 (none written for this path); other books, other days, Gold; a second sitting.

**Confidence:** one sitting, PARTIAL.
