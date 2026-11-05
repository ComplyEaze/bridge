# Results: #2119 row H, how long the desktop client on the lab PC waits for one tool call, 5 Nov 2026

**EXPECT not met on the Chat tab: C1 (60 s, plain) was cut at the client's 60 s device wait, so the climb stopped at C1; C2 (60 s, progress every 10 s) was cut the same way, and C9 was not sent. Neither tab sent a `progressToken` on any call, so no progress was ever sent.** The Code tab cut K1 and K2 at about 60 s with `SdkError: Request timed out`, as on 29 Oct. The Chat-tab calls reached the sleep server on a `local-agent-mode-sleep-2119h` connection, not on the desktop client's own `<client>` connection, and the client's server log holds no line for them. The configuration was restored to B0 by sha256 and the client's server list is as before the row; Tally's window is byte-equal to step 0's capture. One sitting, one PC, one client version.

Sheet `lab/2119-h-sheet` at 16169508, `SHA256.txt` 5 of 5 equal. No Tally request was sent and no key went to Tally; Tally was left as the owner had it.

## What to report

| step | result |
|---|---|
| versions | desktop client package 4.4212.0.0; the Code tab CLI 2.1.309; node v24.21.0; sleep server sha256 `dc236903e0ff09d7b2819013dd122432ac0b59df257bc5094928d410a83451ad` (= sheet) |
| setup | B0 `f7a9ab6c732491f7f46f800b2238040dcc7ac594eb04cd8f4459ea38891b8c1d` (9,420 B; equal to `desktop-config-before.json`); B1 `f4c315c781af4c60e73d31defd5d6254d294e96952f91585fc464f2380c691f1` (9,630 B). Servers before `<lab server 724m7>`; after `<lab server 724m7>`, `sleep-2119h`. Compare after the start: added `sleep-2119h`, removed -, keys changed -. |
| the start | `initialize` 13:06:03.385Z `local-agent-mode-sleep-2119h` protocol 2025-11-25; 13:06:11.769Z `<client>` protocol 2025-11-25. Later `initialize` lines, each from `local-agent-mode-sleep-2119h` on a new process after the one before closed its input (`stdin_closed`, `running` 0): 13:18:05.294Z (just before K2) and 13:21:15.882Z (just before C1). |
| K1 | `{"seconds": 90}`, received 13:08:39.697Z id 2; `progressToken` no; `progress_sent` 0; `cancelled` at 59.969 s (13:09:39.729Z), reason `SdkError: Request timed out`. The session showed: `Error: Request timed out`. |
| K2 | `{"seconds": 90, "progress_every": 10}`, received 13:18:05.309Z id 2; `progressToken` no; `progress_sent` 0; `cancelled` at 60.013 s (13:19:05.335Z), reason `SdkError: Request timed out`. The session showed: `Error: Request timed out`. With no token the server can send no progress (`mcp-sleep-server.mjs:86`), so K2 does not test progress. |
| C1 | 60 s plain, new chat, the sheet's prompt, typed by the owner, tool prompt answered by the owner. Received 13:21:15.895Z id 2 `{"seconds": 60}`; `progressToken` no; `progress_sent` 0; `result_sent` at 60.003 s (13:22:15.905Z). The chat showed the tool error: "Device '\<device\>' did not respond within 60s. This call was sent to the user's computer and only the wait for its reply timed out: the tool may still be running there, may already have finished, or may never have started. Do not run it again unless you know that running it twice is harmless and you have not already retried it; otherwise tell the user it may still finish and ask before running it again." A cut: the chat showed an error. |
| C2 | 60 s, `progress_every` 10, new chat, typed by the owner, tool prompt answered by the owner. Received 13:25:59.672Z id 3 `{"progress_every": 10, "seconds": 60}` on the same connection as C1; `progressToken` no; `progress_sent` 0; `result_sent` at 60.010 s (13:26:59.692Z). The chat showed the same error as C1, word for word. A cut. |
| C3 to C9 | not sent: the climb stops at the first plain cut (C1), and C9 runs only if C2 was answered whole. |
| the client's log lines | `mcp-server-sleep-2119h.log` (3,292 B) ends at 13:06:12.334Z (`tools/list` result on the `<client>` connection). It has no `tools/call` line for any call of the row. |
| longest whole, shortest cut | Code tab: none whole; cut at 59.969 s (K1, N 90). Chat tab: none whole; cut at the 60 s device wait (C1, N 60). |
| hold check | `calls.py` over `h.jsonl` whole: started 4, calls 4, each sent once and each with its end (cancelled, cancelled, `result_sent`, `result_sent`). |
| restore | Close only at 14:29:08Z, from the owner's own window; then `restore ok f7a9ab6c732491f7f46f800b2238040dcc7ac594eb04cd8f4459ea38891b8c1d (9420 bytes)` (= B0); the client started by the owner. Compare after the start (14:31:40Z): sha256 = B0, servers `<lab server 724m7>` before and now, added -, removed -, keys changed - (no rewrite by the client so far). The extension setting is as step 0 (on). |
| close | No `node.exe` runs the sleep server. Tally's window, captured at step 0's scale, is byte-equal to step 0's capture (kept on the lab PC; client data). Nothing was sent or keyed to Tally. |
| dialogs and prompts | The Chat-tab tool prompts for `sleep`, answered by the owner; the server received exactly the asked arguments both times. No permission prompt on the Code tab. A "Relaunch to update" item in the client's sidebar: not clicked. A Bitdefender antivirus entry at about 13:05Z (Critical, "Infected item detected"): detection `CMD:Heur.BZC.PZQ.Boxter.949.0E3A44C9`, action blocked; the file is outside the kit and the lab folders; nothing in Bitdefender was changed. No dialog from Windows or Tally. |

Each answer the model gave alongside the error suggested retrying with fewer than 60 seconds. Not done: each call of the ladder is sent once.

## What it decides

The Chat tab cut below 130 s, and no token was sent: the sheet's third row, **row L decides** (a read of bills by party page, or no build). The Code tab sent no `progressToken` and cut at 60 s: the fourth row, progress cannot help the Code tab, a known limit on any build.

## Reading

On this client both tabs give one tool call about 60 s. The Code tab cancels with `notifications/cancelled` at 60 s. The Chat tab's wait for the lab PC ended at 60 s while the server's result came 3 and 10 ms later, and no cancellation reached the server. Neither tab asks for progress, so a server cannot keep a call alive with it here. [measured: the times, tokens and ends; hypothesis: that the Chat tab's 60 s is its device route's fixed wait]

## Limits

One sitting, one client version, one PC, Windows only. N 60 equals the wait, so whether a call just under 60 s is answered whole on the Chat tab was not measured (the sheet does not halve the ladder). The calls were made from new chats typed by a person; the times are the server's, not the prompt's. Whether the Chat tab's timer runs from the call or from the prompt's answer is not told apart.

## Other processes

The four desktop-client connector servers ended with the client at the owner's restart (13:04Z) and four new ones started with it; the same again at the restore's close (14:29Z) and the owner's start. No call was made to them from this sitting, and their calls were not measured.

**Confidence:** measured for this client version on this PC, both tabs; the Chat tab's wait below 60 s not measured.
