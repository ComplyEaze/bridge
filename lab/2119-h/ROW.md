# Row 2119-H: how long the desktop app on the lab PC waits for one tool call, on its Chat tab and its Code tab, with and without progress

Not for merge. Part of #2119 and #2158. Reads only: no Tally request, no book, no ComplyEaze Bridge build. Lines are cited at master 187e882b. Confidence of everything here until the row runs: unverified.

## Question
On the Windows desktop app of the lab PC, what is the longest `sleep` call that each tab answers whole: the Chat tab at N = 60, 90, 130 and 250 s, and the Code tab at 90 s? When a call is cut, does `notifications/progress` every 10 s keep it alive? And does either tab send the `progressToken` that progress needs?

## Why
`outstandings` reads Tally's two Bills reports, each paired, on every call, whatever `direction` it is given:
- the Receivable pair at `src-tauri/src/tally/runtime.rs:4141-4147`;
- the Payable pair at `runtime.rs:4198-4202`.

(At 4c7b8db6 these were :4135-4141 and :4192-4196; master added six lines above them.)

Through #2158's build, which gives the two Bills posts 60 s each, a stdio client got its answer in 76.9 s on BRIDGE CORPUS DENSE. The desktop app's Code tab cut every call at 60 s (`lab/2119-bills-60s-row-answers`, RESULTS.md). The Chat tab is the main path, and it was never measured: the lab could not drive the app's own window that day (deviation 3 there). `docs/agent/mcp-host-call-timeouts.md` has no Windows row.

## What it decides
The deciding branch is the paired Bills reads above: does a host wait for them, or can it be kept waiting?

| this row shows | build |
|---|---|
| The Chat tab answers 130 s whole, plain. | Merge-up #2158 and merge it. Its body says this decides it. The Code tab's 60 s, and the heaviest real book, stay a known limit. |
| The Chat tab cuts a plain call below 130 s; its call carried a `progressToken`; the same N with `progress_every` 10 answers whole, and so does 2N with progress. | Bridge sends `notifications/progress` from the paired reads when the host gave a token. That is a new MCP surface, and its own design note comes first. |
| The Chat tab cuts below 130 s, and progress does not keep the call alive, or no token was sent. | Row L decides: a read of bills by party page, or no build. |
| The Code tab sends no `progressToken` (as on 29 Oct), or cuts at 60 s with progress too. | Progress cannot help the Code tab. It stays a known limit on any of the builds above. |

A result that fits no row goes to the run coordinator as it is. This row decides nothing about Tally's timings, other books, macOS or another version of the app.

## Rules
`LAB-COMMON.md` in this folder is a byte copy of v3 (sha256 5370c6d2…, as in `lab/2119-bills-60s-row`). Its Tally rules C1 to C11 apply only so far as they touch this row: no Tally request is sent, and no key goes to Tally. Tally is left exactly as the row finds it (running or not, with no company loaded). Where LAB-COMMON and this sheet differ, the stricter one holds.

**Who acts.**
- The lab, on the owner's standing word of 27 Oct 2026 and the owner's word of 4 Nov 2026 for this row: its config change and its restore.
- The Chat-tab steps need a person's hands at the lab PC: to type each prompt in the app's Chat window and click its tool prompt. On 29 Oct the lab could not drive the app's own window.
  - If the lab can drive it now, it does, and records how.
  - If it cannot and no person is at the PC, H-C is skipped, and the row runs H-K and the restore alone.

**One call at a time.** The sleep server runs one call at a time, and refuses a second in band (`scripts/mcp-sleep-server.mjs:145`). A cut call that did not send `notifications/cancelled` still sleeps to N. So no call is sent until `calls.py` shows the call before it ended: `result_sent`, `cancelled`, or `stdin_closed`/`signal`/`stdout_error` with `running` above 0.

**Never sent twice.** Each call of the ladder is sent once. A call whose `received` arguments differ from the ones asked for is recorded and shows nothing. The ladder goes on to the next N; it is not sent again.

## Build and files
- No ComplyEaze Bridge build. The server is `scripts/mcp-sleep-server.mjs` from master 187e882b, sha256 `dc236903e0ff09d7b2819013dd122432ac0b59df257bc5094928d410a83451ad`, unchanged since the 29 Oct row. Another sha256 = STOP.
- Copy it to `C:\bridge-lab\2119-h\sleep\mcp-sleep-server.mjs`.
- The log is `C:\bridge-lab\2119-h\sleep-log\h.jsonl`. It must not exist at setup; `desktop_config.py add` refuses if it does.

| file | what |
|---|---|
| `ROW.md` | this sheet |
| `LAB-COMMON.md` | v3, byte copy |
| `desktop_config.py` | adds the one entry `sleep-2119h` and keeps the before-copy (B0); restores those bytes and checks them by sha256; compares server names after a start |
| `calls.py` | reads `h.jsonl` into one line per call: arguments, `progressToken` yes or no, progress sent, how it ended |

`SHA256.txt` lists them all. Check it first.

## Every dialog and prompt this row may meet, and its answer
Anything not in this table, in the app, in Windows or in Tally, is a STOP. Take a screenshot, copy its text exactly, press no key, and report.

| where | what shows | answer |
|---|---|---|
| Chat tab | the tool prompt for `sleep` showing exactly the arguments the step asked for | **Allow once**, by mouse; never the "always" choice |
| Chat tab | a tool prompt for `sleep` with other arguments, or for any other tool (a ComplyEaze Bridge tool included) | **Deny** |
| Chat tab | the model asks a question, or calls nothing | Send the step's prompt once more in the same chat. A second such reply skips that call. |
| Chat tab | a notice that the server `sleep-2119h` failed, disconnected or timed out | Click nothing. Copy its text; it ends that call. Go on to the wait. |
| Code tab | a permission prompt for `sleep` (there was none on 29 Oct) | as on the Chat tab: Allow once only for the asked arguments, else Deny |
| the app | a banner or button offering an update or a relaunch | Not clicked. Record its text. If it is a modal window over the app: STOP. |
| the app, at a start | the app opens on its last chat, or offers to resume the lab's session | Resume the lab's session only. Nothing else is clicked. |
| the app, at a quit | none expected: the lab's restart helper asks the app to close and ends what is left, as on 29 Oct | any window that asks to confirm, save or keep: STOP |
| the configuration file | no save prompt can appear: no editor opens it, `desktop_config.py` writes its bytes | an editor window over it: STOP, nothing saved |
| Windows | a security or permission prompt (for `node.exe` or the app) | STOP, nothing allowed |
| Tally | nothing is keyed and nothing is sent; any change on Tally's screen | STOP |

## Steps
One at a time. Every answer goes to a file, whole. All times are in UTC.

0. **Before.**
   - Check `SHA256.txt`. Check the sleep server copy's sha256.
   - Record `node --version`.
   - Record the app's version from its package listing (`Get-AppxPackage`; the name and version only), not from its window.
   - Record the Code tab session's version from its own `--version`.
   - Record the configuration file's path and the app's logs folder in the local notes. They are pushed only as `<desktop config file>` and `<logs folder>`.
   - Record the app's configured server names (names only), and whether the ComplyEaze Bridge extension is on.
   - Take a screenshot of Tally's window as found. It is not touched again until step 7's screenshot.
1. **Setup.**
   - `python -I desktop_config.py add <config> C:\bridge-lab\2119-h\desktop-config-before.json C:\bridge-lab\2119-h\sleep\mcp-sleep-server.mjs C:\bridge-lab\2119-h\sleep-log\h.jsonl`.
   - EXPECT `add ok`, B0, B1, and the server names of step 0 plus `sleep-2119h`. Anything else: STOP, with the configuration as the line says.
   - Hash the before-copy: it must be B0.
   - Then quit and start the app with the lab's restart helper, as on 29 Oct (RESTORE.md of `lab/2119-bills-60s-row-answers`: ask it to close, end what is left, start it by its AppID). This ends the lab's own session until the app is back.
2. **After the start.**
   - `desktop_config.py compare <config> C:\bridge-lab\2119-h\desktop-config-before.json`. EXPECT `added sleep-2119h`, `removed -`. A rewrite by the app (a new sha256, a changed top-level key) is recorded, not a STOP.
   - `calls.py h.jsonl`: record each `initialize`'s client name and protocol, and `calls 0`. No `started` within 2 minutes of the start: STOP. Then the entry did not load; the restore is step 6.
3. **H-K, the Code tab** (the lab's own session calls the tool; no prompt is expected).
   - **K1:** call `sleep` with `{"seconds": 90}` and nothing else. Record what the session showed, word for word, and its time.
   - When `calls.py` shows K1 ended: **K2**, `sleep` with `{"seconds": 90, "progress_every": 10}`. Record the same.
   - **EXPECT** (29 Oct): each cut at about 60.0 s, with `cancelled` reason `SdkError: Request timed out`, `progressToken no`, and `progress_sent 0`.
     - The server sends progress only when the call's `_meta` carries a token (`mcp-sleep-server.mjs:86`). So with no token, K2 cannot test progress; record that.
     - If K2 carries a token, record `progress_sent` and how K2 ended.
   - Sender: the app's Code tab path. The deciding line is the server's `tools/call` handler (`mcp-sleep-server.mjs:141-146`); it ends at `:106` (`result_sent`) or `:121` (`cancelled`).
4. **H-C, the Chat tab** (needs a person, or the lab, at the app's window; see Rules). Each call is made in a new chat, with this exact prompt:
   `Call the sleep tool once with seconds N and no other argument, then show me exactly what it returned or the exact error.`
   With progress, add `progress_every 10` instead of "no other argument":
   `Call the sleep tool once with seconds N and progress_every 10 and no other argument, then show me exactly what it returned or the exact error.`

   Answer the tool prompt by the table above. Record:
   - what the chat showed, word for word, and when;
   - the `calls.py` line for the call;
   - from the app's logs folder, the `mcp*.log` lines that name `sleep-2119h`, from the call's `tools/call` to its result or cancellation (times converted to UTC).

   The ladder, each call only after the one before ended (Rules):

   | call | N | `progress_every` | sent when |
   |---|---|---|---|
   | C1 | 60 | none | first |
   | C2 | 60 | 10 | after C1 |
   | C3 | 90 | none | if C1 was answered whole |
   | C4 | 90 | 10 | after C3 |
   | C5 | 130 | none | if C3 was answered whole |
   | C6 | 130 | 10 | after C5 |
   | C7 | 250 | none | if C5 was answered whole |
   | C8 | 250 | 10 | after C7 |
   | C9 | twice the first N whose plain call was cut (at most 500) | 10 | only if that N's progress call was answered whole |

   - The climb stops at the first plain call that is cut; that N's progress call still runs. "Answered whole" means the chat showed `slept N.x s`, and the app's log holds the result for that call's id. A `result_sent` in `h.jsonl` alone does not count.
   - A cut: the chat showed an error, a timeout or a cancellation; or `h.jsonl` shows `cancelled`, or `stdin_closed`/`signal`/`stdout_error` with `running` above 0.
   - **EXPECT** (inference, unverified; macOS Chat tab, protocol reference §11f, partial): 60, 90 and 130 answered whole, and 250 cut near 240 s.
   - Sender: the app's Chat tab path. The deciding lines are as in step 3; the progress line is `:86`.
5. **Hold check.**
   - `calls.py h.jsonl` whole: every call sent appears once, with its end.
   - Copy `h.jsonl` and the `mcp*.log` lines of the row.
6. **Restore.**
   - Quit the app with the lab's restart helper, set to close only. Before it starts again: `desktop_config.py restore <config> C:\bridge-lab\2119-h\desktop-config-before.json <B0>`. EXPECT `restore ok B0`. Anything else: STOP, leave the app shut, report the configuration's state.
   - Then start the app.
   - After the start: `desktop_config.py compare`. EXPECT no server added or removed. The app's rewrite of the file on start (seen on 30 Oct: one changed top-level value) is recorded, not a STOP.
   - The extension setting is as step 0 recorded. This row does not change it.
7. **Close.**
   - Take a screenshot of Tally's window: the same as step 0's.
   - The `node.exe` that ran `mcp-sleep-server.mjs` is gone (`Get-CimInstance Win32_Process`, read only). One still running is recorded; nothing is ended by hand.
   - Retire `C:\bridge-lab\2119-h\` for the answers.

## What to report
| step | result |
|---|---|
| versions: app, Code tab session, node; server sha256 | |
| setup: B0, B1, servers before and after | |
| the start: `initialize` clients and protocols | |
| K1, K2: arguments, token, progress sent, end, what the session showed | |
| C1 to C9: the same, plus the app's log lines | |
| the longest N answered whole, and the shortest cut, on each tab | |
| restore: the restore line (B0), the compare after the start | |
| any dialog or prompt, with its text and the answer given | |

Read every number from its file. Put any STOP, skip, or EXPECT not met in the first line. Fill the "What it decides" row this result meets.

## Stop rules
- A dialog or prompt not in the table.
- A sha256 not as named: the kit, the server copy, the before-copy, or B0 at the restore.
- `desktop_config.py` refusing at setup or restore.
- No `started` in `h.jsonl` after the start.
- A call with no end in `calls.py` 10 minutes after its N.
- Anything sent to Tally, or any key in Tally.

After a STOP, the restore (step 6) waits for the run coordinator's word. The report lists the configuration's state and whether the app is running.

## Limits
- One sitting, one app version, one PC. No step halves the ladder to within 10 s of the limit (method step 5).
- The count is calls, not Tally's work: the server sleeps, and no Bills read runs.
- A person at the app's window may answer the tool prompt slowly. The time is taken from the log's `tools/call`, not from the prompt.
- Whether the app's timer counts from the call or from the prompt's answer is not told apart.

## Keep
On `lab/2119-h-answers`, with `SHA256.txt`:
- every `calls.py` output and every compare line;
- `h.jsonl` (no names or values beyond the server's own);
- the `mcp*.log` lines of the row, with account, organisation and conversation ids replaced by `<redacted>` (the raw file's sha256 recorded beside it);
- the versions;
- a `RESULTS.md` holding the table above.

Screens and the configuration copies stay on the lab PC, listed by name, size and sha256. Notes carry no user names and no machine paths beyond `C:\bridge-lab\2119-h\`. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
