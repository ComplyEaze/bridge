# 2142 drain live: ComplyEaze Bridge's own drain after a cut post, on #2204's build (Part of #2142)

Sheet `lab/2142-drain-live` at 9a1ed737. Build: pull request #2204 at 651cdfa67f15056cd9017b12589e0a010c80637d, CI run 41025801031, artifact 12963818564 (58,275,049 B, digest sha256:72284e1e…0bae, equal). `bridge_mcp.exe` sha256 2c41a51ebca370ff205a7cad72ece2f77811a3221165e685e1977eb88c3a3eb2, equal to the bundle's smoke record and the same before and after. Company: BRIDGE CORPUS DENSE (synthetic), marks 328 / 29900 throughout. TallyPrime 7.2 Silver. 30 Oct 2026. Reads only. One sitting, so the result is PARTIAL.

## Results
| run | step | call | wall s | round trip s | answer | trail | drain record after |
|---|---|---|---:|---:|---|---|---|
| 1 | 3 | outstandings, as of 20260331 | 21.48 | 21.30 | refused: `native_outstandings_read_failed`, cause `native_bills_report_deadline_exceeded` | 26 sends; the last, a post, cut at 20,011 ms | owed, answered 0 |
| 1 | 4 | ledger_masters, sent 0.2 s later | 5.61 | 5.10 | refused: `tally_endpoint_draining`, `retry_after_s` 10 | a status cut at 5,012 ms, then the post refused (held 0 ms, not sent) | owed, answered 0, last_probe_ms set |
| 1 | 5 | ledger_masters, after 15 s | 5.30 | 4.87 | answered, 328 / 29900 | status, status, status (held 1, 0, 0 ms), then the post; 29 sends, none failed | clear |
| 2 | 3 | outstandings | 22.02 | 21.36 | refused: same code and cause | the last post cut at the limit | owed, answered 0 |
| 2 | 4 | ledger_masters, at once | 5.55 | 5.09 | refused: `tally_endpoint_draining`, `retry_after_s` 10 | a status cut at 5,010 ms, then the post refused (held 0 ms) | owed |
| 2 | 5 | ledger_masters, after 10 s | 5.52 | 4.88 | answered, 328 / 29900 | status, status, status, then the post; 29 sends, none failed | clear |

At rest, in step 2 and step 7, `ledger_masters` took 0.99 to 1.81 s wall. The threshold, taken from the second pair of step 2 reads, is 3.44 s wall (2.90 s round trip).

- **The cut, through ComplyEaze Bridge: yes, in both runs.** The Bills Receivable request was cut at the 20 s limit, the call was refused with the cause the pull request names, and the record then read owed.
- **A call refused while Tally still works: yes, in both runs.** `ledger_masters`, sent at once, was refused as draining in about 5.6 s with `retry_after_s` 10. Its one status probe got no answer within 5 s, and its post was refused before sending.
- **A call admitted after three quick probes: yes, in both runs, but not within the threshold.** The next call answered on its first try, after exactly three answered status sends and then the post, and the record read clear. It took 5.30 s and 5.52 s wall (4.87 s and 4.88 s round trip), against 3.44 s (2.90 s). The three probes are 2 s apart, so the admission itself adds about 4 s to an at-rest read of about 0.8 s.
- **Finding:** the sheet's last expectation, an answer within the at-rest threshold, cannot hold while three probes 2 s apart come before the post. Everything else ran as expected, and the two runs agree.
- Both runs' admission came at the first retry, 15 s and 10 s after the refusal. So Tally's leftover work after each cut ended within about 25 s of the cut, consistent with the curl rows' 8.9 to 25.4 s.
- The drain record's fields are `state`, `since_ms`, `answered` and `last_probe_ms`. The sheet's `quick` is named `answered` in this build. Each copy is kept under `debt/`, with bytes and sha256 in `SHA256.txt`.

## Deviations
- **A gap of about 82 minutes after step 2.** The lab's session stopped and the owner reopened it. Nothing was in flight, and the client and server stayed up, idle.
  - On resume, one `GET /status` was sent by curl as a liveness check. That is a raw request the sheet allows only at step 0 and C10.
  - Tally was found on the Dashboard; a click on its header returned it to the Gateway. No key was pressed.
  - `list_companies` and step 2 were run again before the cut.
- In run 1, the lab's driver misread where the refusal code sits, so step 5 started by hand 15 s after step 4's answer, rather than at `retry_after_s` 10. The driver was fixed before run 2.

Not covered (the sheet's list): a dropped connection, a size limit or an abandoned send in place of the cut; a second process sending during the drain; other reads, books and Gold; cut points other than 20 s.
