# Row 703-A: on Bridge's own tool text and a real task, does the model carry a server-issued `read_id` from call to call, on the desktop app's Chat tab and Code tab

Not for merge. Part of #703. Reads only: no Tally request, no book, no ComplyEaze Bridge build. Lines are cited in this folder's files. Confidence of everything here until the row runs: unverified.

## Question
#703's proposed shape A is a stepped read: a read too long for one call is served in steps, Bridge holds the plan under a `read_id` it mints, every step but the last answers `done: false` with no rows, and only the last carries the rows. Shape A is the first handle in Bridge where skipping it gets nothing, so it works only if the model, on its own, calls again with the id the step before returned, until the read is done.

On the Windows desktop app of the lab PC, given Bridge's own server instructions and `vouchers` text plus the build's planned stepping sentences, and a real task, does the model, in 3 of 3 new chats:
- call `vouchers` once per step, 5 steps for a week and 12 for a month, each later call carrying the `read_id` the server issued, unchanged, with the same other arguments;
- not start a new read, invent an id, reuse a finished one, or ask the person before a step;
- and answer from the last step's rows?

And on each tab: does a step of 50 or 55 s come back whole (the app cut a 60 s call in row H); does the 60 s count from the tool prompt or from the person's approval; and does the app ask approval on every call, cap the calls in one turn, or ask whether to continue?

## Why
Row H (`lab/2119-h-answers` at `dfaf2737`, `docs/agent/mcp-host-call-timeouts.md`) measured about 60 s per call on both tabs, with no `progressToken`. The red-team review of the first version of this sheet (9 Nov) found that a sleep tool with a bespoke description measures that description, that one run per tab is a sighting rather than a result, that five calls say nothing about call twelve, and that 40 s steps never exercise the margin under 60 s. This version answers each: the text is Bridge's, there are three chats per plan per tab, plans of 5 and 12 steps, steps of 50 and 55 s, and a chat that delays approval. Nothing of the stepped read is built until this row keeps it (P1).

## What it decides
Each chat is read on its own, from `steps.py a.jsonl --since <T> --until <T-next>`, where T is the UTC time written down just before that chat's prompt was sent and T-next is the next chat's (none for the last), with the chat's words and the approval log. **The rows are read from the top; the first row that matches decides that chat.**

| # | this chat shows | then |
|---|---|---|
| 1 | `pid_changes` above 0, or an id in `ids_from_other_pid` | **The host started a new server between calls.** A plan held in one process does not survive that host, whatever the model did. Goes to the run coordinator as it is. |
| 2 | `evicted` names a read of this chat | The model started more than four reads. Goes to the run coordinator as it is. |
| 3 | `cut` above 0, or the chat showed an error for a call | **The worst step is too close to the host's wait,** or the wait counts from the tool prompt. Record which step (its planned seconds are in the `read` line's plan: five is 40, 50, 40, 55, 30; twelve is 40, 40, 50, 40, 40, 55, 40, 40, 50, 40, 55, 30), `max_cut_elapsed`, `max_answered_elapsed`, and the approval log for that call. The design's start cutoff comes down before any build. |
| 4 | `busy` above 0 | **Two calls overlapped:** either the host gave up on a call without telling the server and the model called again, or the model sent two calls at once. The log does not tell these apart; the chat's words may. Goes to the run coordinator as it is; A is not kept in that chat. |
| 5 | the app stopped the turn at a cap, or asked whether to continue (its own control, not the model's words) | **A host cap.** Record the call count it stopped at and its text. Goes to the run coordinator: a hard cap bounds the steps a read may plan. |
| 6 | `new_read_after_answered_step` above 0, or an id in `ids_not_issued`, or a `refused read_finished`, or `arguments_changed` above 0 | **A fails in that chat** (a new read once an id was given, an invented id, a finished id reused, or the arguments changed under one id). |
| 7 | the model stopped before `done`, asked the person anything before the answer, or needed the nudge | **A fails in that chat.** A nudge is recorded, never averaged away. |
| 8 | one `read` line, `steps <n> of <n> done true` with n the plan's, `vouchers_calls` n, `answered` n, `cut 0`, `busy 0`, `refused 0`, `open 0`, `other_calls_not_answered 0`; the chat showed no error for any call; the chat's answer names the party `Lab Party <code>` with the `read` line's code; no nudge | **A kept in that chat.** |

A chat that fits no row goes to the run coordinator as it is. "The chat showed no error" is part of row 8 because a host that gives up on a call without telling the server leaves the server's log looking whole: the chat's words are the only trace, so the lab session copies them in full.

**Also recorded, whatever row a chat meets:** any `initialize` line with `tasks yes` (#703's deferred background-job shape is revisited only on that), and the `meta` keys of each call.

**A tab keeps A only when all six of its counted chats (C1 to C6, or K1 to K6) meet row 8.** Five of six is a fail, recorded with the chat that failed. Each plan's 3 of 3 is reported separately, so a tab that keeps the week and fails the month says so. Chat D (the delayed approval) is not counted for or against A: it answers whether the 60 s counts from the prompt.

**Reading chat D.** In chat D, two approvals are held 30 s after the prompt shows: the first `vouchers` prompt after step 1 has answered (the one that runs step 2, 50 s) and the first after step 3 has answered (step 4, 55 s). A prompt after a cut runs the cut step again and is approved at once. If the host counts from the prompt, each held step is about 80 s and is cut (row 3 in that chat). If it counts from the request it sends after approval, each is about 50 to 55 s and answers. Record which, from the approval log and the `call` lines; one chat per tab is a sighting, recorded as partial.

The row decides nothing about Tally, about Bridge's tool text beyond what is served here, about macOS, or about another version of the app.

## Rules
`LAB-COMMON.md` in this folder is a byte copy of v3 (sha256 5370c6d2…). Its Tally rules C1 to C11 apply only so far as they touch this row: no Tally request is sent, and no key goes to Tally. Tally is left exactly as the row finds it, and its window is not captured. Where LAB-COMMON and this sheet differ, the stricter one holds.

**Who acts.**
- **The person** (steps marked PERSON): the configuration `add`, the app's quit and start, the `restore`, and switching the ComplyEaze Bridge extension off and on. The lab session's own permission check refuses changes to the app's configuration (row H, notes at 11:29Z on 4 Nov). If no person is at the PC, the row waits; it is not run without the PERSON steps. The owner is away until 10 Nov 2026: the row is on the owner's list for that date or later, and no build date assumes it has run.
- **The lab session** prepares the folder, checks the hashes, drives the app (opens each new chat or session, types each prompt, answers each tool prompt by the table), keeps the approval log, runs `steps.py` and writes the record. **The lab session never calls the server's tools itself, and never types anything into a chat but the prompt and, where the table says, the nudge.** It has read this sheet, so its own words would measure nothing.

**The configuration path.** The app is a packaged app: the path its own sessions see is redirected. `add` and `restore` use the real file outside the package, as row H's second try did (row H notes, 10:55Z on 5 Nov).

**The ComplyEaze Bridge extension is switched off** for the row (PERSON, step 1) and back to its step 0 state at step 6. A model that called a ComplyEaze Bridge tool could send a request to Tally. Its state before and after is recorded.

**One call at a time.** The server refuses a second call while one runs, in band (`busy`). Nothing is typed while a step runs.

**At most plan + 4 tool prompts per chat:** 9 in a week chat, 16 in a month chat. The next is denied, and the chat ends there.

## Build and files
- No ComplyEaze Bridge build. The server is `stepped-vouchers-server.mjs` in this folder (new; derived from master's `scripts/mcp-sleep-server.mjs`, whose message handling, logging and cancel handling it keeps). It serves two tools, `list_companies` and `vouchers`, and the server instructions, all read from `tools.json`.
- **The text the model sees is Bridge's.** `tools.json` is written by `extract_text.py` from master e24f1244's `src-tauri/src/` files `agent_protocol.rs`, `agent_catalog.rs`, `agent_import.rs` and `agent_voucher_search.rs`, each piece taken exactly once or the extractor fails, and put together as Bridge puts it together: the server instructions (`SERVER_INSTRUCTIONS` and the closing sentence of the default bundle, which prepares but does not post); the `list_companies` and `vouchers` descriptions, each followed by the read-receipt sentence every read tool carries, with the read tools' annotations (`readOnlyHint` true); and the whole shipped `vouchers` input schema, with each parameter's description by the first matching arm of `parameter_description`. Every object is served with its keys sorted, as Bridge's `serde_json` sends them. It was run on the build Mac on the four files as `git show e24f1244:<path>` prints them; anyone can repeat it and compare the hash in `SHA256.txt`. The lab PC checks the hash only.
- **What differs from what Bridge ships, and is the one thing this row varies:** four planned texts the build would add (`extract_text.py`, `PLANNED_*`, `NEXT_STEP` and `READ_ID_DESCRIPTION`): one sentence appended to the server instructions, one paragraph appended to the `vouchers` description, the `read_id` parameter, and the `next_step` a `done: false` result carries ("Call vouchers again with the same arguments and this read_id."). They are listed under `planned` in `tools.json`, and `read_id` is marked there; the server strips the marker before serving.
- **Stated differences that are not under test:** the results are in Bridge's shape (`company`, then `result`; refusals as `vouchers: refused` with a code, `isError` true), but every value is invented here: one company, `Lab Stepped Traders`, and three vouchers whose party is `Lab Party <code>`, a five-letter code minted per read. The server's name is `bridge-lab-703a`.
- **The plans** (`stepped-vouchers-server.mjs`, `PLANS`): a window of up to 10 days is `five`, 40, 50, 40, 55 and 30 s; a longer one is `twelve`, 40, 40, 50, 40, 40, 55, 40, 40, 50, 40, 55 and 30 s. `--step-scale` is not passed, so the steps run at those lengths. In band, as Bridge refuses: `read_unknown`, `read_finished`, `read_expired` (idle 15 minutes), `read_arguments_changed`, `company_not_open`, `busy`, and an argument code for an unknown or ill-typed argument. Every log line carries the pid; every message received is logged whole, `initialize` capabilities and any `_meta` included.
- Copy the folder to `C:\bridge-lab\703-a\kit\`. The log is `C:\bridge-lab\703-a\log\a.jsonl`; it must not exist at setup (`desktop_config.py add` refuses if it does).
- The Code-tab chats each use a new empty folder, `C:\bridge-lab\703-a\empty-<n>\`, as the session folder, so no session can read this sheet.

| file | what |
|---|---|
| `ROW.md` | this sheet |
| `LAB-COMMON.md` | v3, byte copy |
| `stepped-vouchers-server.mjs` | the server |
| `tools.json` | the served text, from `extract_text.py` |
| `extract_text.py` | writes `tools.json` from Bridge's source |
| `desktop_config.py` | row H's helper with its entry renamed `bridge-lab-703a` and a `--tools` argument added; adds the entry and keeps the before-copy (B0); restores those bytes and checks them by sha256; compares server names after a start |
| `steps.py` | reads `a.jsonl` into one line per call (with the gap before it), one per read, and the `facts` line, between `--since` and `--until` |
| `prescreen.sh` | the macOS CLI pre-screen (below); not run on the lab PC |

`SHA256.txt` lists them all. Check it first.

## The prompts
Typed once, exactly, at the start of a new chat or session:
- **Week (W):** `Show me my company's vouchers from 2 to 8 November 2026.`
- **Month (M):** `Show me my company's vouchers for October 2026.`

The prompts give the dates, so the server instructions' rule to ask before a read over more than one month does not apply, and they name no company, so with one company open the instructions say to use it.

**The one nudge.** If the model stops before the answer, asks a question, or says it will go on and calls nothing, the lab session types once, exactly: `Yes, carry on until it is done.` A second stop ends the chat. Each nudge is recorded with its time and the model's words before it. Any nudge fails that chat (row 7); the nudge is there to see whether the read could finish, not to pass it.

## The chats, in this order, on each tab
| chat | prompt | approval |
|---|---|---|
| C1, C2, C3 (Chat tab) / K1, K2, K3 (Code tab) | W | at once |
| C4, C5, C6 / K4, K5, K6 | M | at once |
| CD / KD | W | at once, except the prompts that run steps 2 and 4 (Reading chat D): held until 30 s after the prompt shows |

Each is a new chat (Chat tab) or a new session in a new empty folder (Code tab). Nothing is resumed. Before each, `steps.py a.jsonl` shows no `open` call.

## The approval log
For every tool prompt, one line in `approvals.txt`: the chat, the prompt's number in that chat, the tool and arguments it shows, the UTC time it was first seen, the UTC time Allow once was clicked, and what was clicked. Times from `[DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ')`, run at the moment of each, so the log is the lab session's own clock, to the second at best; the server's `received` time is the exact moment the host sent the call. A call that ran with no prompt is a line too: `no prompt`.

## Every dialog and prompt this row may meet, and its answer
Anything not in this table, in the app, in Windows or in Tally, is a STOP. Take a screenshot, copy its text exactly, press no key, and report.

| where | what shows | answer |
|---|---|---|
| either tab | a tool prompt for `list_companies` or `vouchers` (within the chat's cap) | **Allow once**, by mouse, at once (chat D: as its row says); never an "always" choice. Log it. |
| either tab | a tool prompt past the chat's cap | **Deny.** The chat ends. |
| either tab | a tool prompt for any other tool | **Deny.** Record it. |
| either tab | the model asks a question or stops before the answer | the nudge above, once |
| either tab | the app's own control saying a tool-call limit was reached, or asking whether to continue | Record its text and the call count. Click its continue control once; a second one ends the chat. Row 5 decides the chat. |
| either tab | an error that the server did not respond, timed out, failed or disconnected | Click nothing. Copy its text. The chat goes on only if the model goes on by itself. |
| the app | a banner or button offering an update or a relaunch | Not clicked. Record its text. A modal window over the app: STOP. |
| the app, at a start | the app opens on its last chat, or offers to resume a session | Nothing is resumed. Open a new chat or session. |
| the app, at a quit | none expected (row H's restart helper) | any window that asks to confirm, save or keep: STOP |
| Windows | a security or permission prompt (for `node.exe` or the app) | STOP, nothing allowed |
| Tally | nothing is keyed and nothing is sent; any change on Tally's screen | STOP |

## Steps
One at a time. Every answer goes to a file, whole. All times are in UTC.

0. **Before.**
   - Check `SHA256.txt`, `tools.json` included.
   - Record `node --version`, the app's version (`Get-AppxPackage`, name and version only) and the Code tab's version from its own `--version`.
   - Record the app's configured server names (names only) and whether the ComplyEaze Bridge extension is on.
1. **Setup** (PERSON).
   - Switch the ComplyEaze Bridge extension off in the app's settings. Record that it shows off.
   - `python -I desktop_config.py add <config> C:\bridge-lab\703-a\desktop-config-before.json C:\bridge-lab\703-a\kit\stepped-vouchers-server.mjs C:\bridge-lab\703-a\kit\tools.json C:\bridge-lab\703-a\log\a.jsonl`. EXPECT `add ok`, B0, B1, and step 0's server names plus `bridge-lab-703a`. Anything else: STOP, with the configuration as the line says.
   - Hash the before-copy: it must be B0.
   - Quit and start the app with row H's restart helper (`lab/2119-h-answers`, `scripts/restart-desktop.ps1`), as on 5 Nov.
2. **After the start.**
   - `desktop_config.py compare <config> C:\bridge-lab\703-a\desktop-config-before.json`. EXPECT `added bridge-lab-703a`, `removed -`. A rewrite by the app is recorded, not a STOP.
   - `steps.py a.jsonl`: record each `initialize` line. No `started` within 2 minutes of the start: STOP; the entry did not load, and the restore is step 5.
3. **The Chat tab: C1 to C6, then CD.** For each: a new chat; write down T; type its prompt; answer each tool prompt by the table, logging it. Record every word the chat showed between the prompt and the end, with the time of each tool prompt and each result as the chat shows them; each nudge and the words before it; any cap or continue control; and the chat's final words, copied exactly. The chat ends at the model's last reply, a second stop, or the cap. Then `steps.py a.jsonl --since <T> --until <T of the next chat, or omitted for the last>`, whole, to `steps-<chat>.txt`. (Run each `--until` once the next chat's T is written, so every file is complete.)
4. **The Code tab: K1 to K6, then KD.** As step 3, each in a new session in `C:\bridge-lab\703-a\empty-<n>\`, never the lab's own session. Then `steps.py a.jsonl` with no window to `steps-all.txt`.
5. **Restore** (PERSON).
   - Quit the app with the restart helper, set to close only. Before it starts again: `desktop_config.py restore <config> C:\bridge-lab\703-a\desktop-config-before.json <B0>`. EXPECT `restore ok B0`. Anything else: STOP, leave the app shut, report the configuration's state.
   - Start the app. `desktop_config.py compare`: EXPECT no server added or removed. A rewrite on start is recorded, not a STOP.
6. **Close** (PERSON for the first item).
   - Switch the ComplyEaze Bridge extension back to its step 0 state. Record it.
   - No `node.exe` runs this folder's server (`Get-CimInstance Win32_Process`, read only). One still running is recorded; nothing is ended by hand.
   - Retire `C:\bridge-lab\703-a\` for the answers.

About 14 chats; a week chat takes about 4 minutes of steps and a month chat about 9, so the chats take about 1 h 30 min plus approvals.

## What to report
| step | result |
|---|---|
| versions: app, Code tab, node; kit hashes | |
| setup: B0, B1, servers before and after; the extension off | |
| `initialize` lines: clients, protocols, capability keys, `tasks`; the `meta` keys the calls carried | |
| each chat C1 to C6, K1 to K6: the row of "What it decides" it meets, its `read` and `facts` lines, nudges, the answer's party name against the `read` line's code | |
| the Chat tab and the Code tab: 3 of 3 week, 3 of 3 month, kept or not | |
| CD, KD: steps 2 and 4 answered or cut, with the approval log's times and the `received` times | |
| per tab: prompts per call (every call or not), any cap or continue control, the largest `gap` and its approval | |
| the longest answered step and any cut step, per tab, with its planned seconds | |
| restore: the restore line (B0), the compare after the start; the extension as at step 0 | |
| any dialog or prompt, with its text and the answer given | |

Read every number from its file. Put any STOP, skip, or EXPECT not met in the first line.

## Stop rules
- A dialog or prompt not in the table.
- A sha256 not as named: the kit (`tools.json` included), the before-copy, or B0 at the restore.
- `desktop_config.py` refusing at setup or restore.
- No `started` in `a.jsonl` after the start.
- A call still `open` in `steps.py` 10 minutes after it was received.
- Anything sent to Tally, or any key in Tally.

After a STOP, the restore (step 5) waits for the run coordinator's word. The report lists the configuration's state, the extension's state, and whether the app is running.

## The macOS CLI pre-screen (before the sitting, not on the lab PC)
`sh prescreen.sh <empty folder> 3 sonnet` runs the same server and `tools.json` through `claude -p` on a Mac: three week and three month runs, each a new session with only this server's two tools allowed and no other MCP server, at `--step-scale 0.05` (it measures the model, not a host's wait). It prints PASS for a run only on row 8's conditions that a log can show (not the chat's error, which `claude -p` prints in its answer). A FAIL there is reported before the sitting, with its `facts` line; the text is not changed to make it pass (one variable at a time). It does not stand in for this row: the host, its prompts and its wait differ. **Pending: the owner logs in to the CLI on the build Mac.** On 9 Nov it could not run, because the CLI's sign-in had expired. The sheet is held from the lab queue until the pre-screen has run and its verdict lines are attached to this sheet's answers.

## Limits
- One sitting, one app version, one PC, the model each tab chose (record its name if shown). Three chats per plan per tab: a 3 of 3 is a small sample, recorded as partial, not a rate.
- The text is Bridge's at e24f1244 plus the planned texts. A build that ships weaker stepping text than `tools.json`'s, or a different result shape for `done: false`, is not covered by this row.
- The server waits; no Tally work runs. A real step's length varies with the book; the plans' 50 and 55 s steps stand for the worst step under the design's start cutoff (35 s plus a 20 s request).
- The approval log is the lab session's clock read at each act, so the gap between a prompt and its approval is known to about a second.

## Keep
On `lab/703-a-answers`, with `SHA256.txt`:
- every `steps.py` output, `approvals.txt`, and every compare line;
- `a.jsonl` (no names or values beyond the server's own);
- each chat's and session's words, with account, organisation, device and conversation ids replaced by `<redacted>` (the raw file's sha256 recorded beside it);
- the versions;
- a `RESULTS.md` holding the table above.

Screens and the configuration copies stay on the lab PC, listed by name, size and sha256. Notes carry no user names and no machine paths beyond `C:\bridge-lab\703-a\`. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
