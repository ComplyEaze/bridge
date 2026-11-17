# Row 703-A, version 4, 17 Nov 2026: the Windows desktop app's call limit on each tab (H1 to H4), a stepped read continued by the same arguments (S1), and each host's `initialize` (T1)

**Changed from version 3:**
- The owner runs every configuration `add-sleep`, `add` and `restore` in his own PowerShell window because the lab session's permission check always refuses them.
- A person quits and starts the app by hand; the lab session never ends a process.
- After each start the owner opens a new lab session and types the fixed first words.
- The configuration path is the new `Claude-Data` package path defined in "The owner's lines".
- The planned sitting is Chat-tab only because a ComplyEaze Bridge tool can be reached from a Code-tab session; every Code-tab chat is skipped and owed.
- The owner approves one access request for the Claude app; without window access he types the prompts and the lab session only reads the logs.

**The planned sitting is Chat-tab only.** The desktop extension's servers are shared into Code sessions on this PC although the Code tab's own files name none. Under the reachability rule in Rules, every Code-tab chat, including T1 on the Code tab, is skipped, recorded as skipped and owed to a later sitting.

Not for merge. Part of #703. Reads only: no Tally request, no book, no ComplyEaze Bridge build. Lines are cited in this folder's files. Confidence of everything here until the row runs: unverified.

## Question
The #703 proposal (11 Nov) recommends a stepped read, parked between calls and continued by a call with the same tool and the same arguments, and asks for these rows before anything is built (P1). Each changes one variable.

On the Windows desktop app of the lab PC:
- **H1 (Chat tab), H2 (Code tab, owed):** of calls of 30, 45 and 55 s to the repository's sleep server, which come back whole?
- **H3 (Chat tab):** does the tab's limit of about 60 s run from the tool prompt or from the person's answer to it?
- **H4 (each tab; Chat tab only in this sitting):** after the tab cuts a call, does the next call in the same chat reach the same server process?
- **S1 (each tab; Chat tab only in this sitting):** given Bridge's own `vouchers` text plus the planned stepping sentences for continuation by the same arguments, and a real task, does the model, in 3 of 3 new chats per plan, call `vouchers` once per step (5 for a week, 12 for a month) with the same arguments each time, start no second read, ask the person nothing before the answer, and answer from the last step's rows?
- **T1 (each tab; Chat tab only in this sitting):** what each tab's `initialize` declares: client name, protocol revision and capability keys, and whether `tasks` is among them.

O1 (a day, a week and a month on a lab book, with each send's and each operation's time) is a later sitting. It needs Tally and is not part of this sheet.

## Why
Row H (`lab/2119-h-answers` at `dfaf2737`, `docs/agent/mcp-host-call-timeouts.md`) saw both tabs end a call at about 60 s, one run each, with no `progressToken` on either and no cancel from the Chat tab. Shorter calls were not measured. The proposal's question 3 (the step budget) needs the longest call that returns, and its question 2 (continuation) needs to know whether the model repeats the same arguments without an id.

Version 2 of this sheet (`13239dba`, never run) measured continuation by a required `read_id`. The proposal recommends continuation by the same arguments instead, with `read_id` optional, so S1 measures that. Version 2's mode is kept in the kit and is S2; S2 is not run in this sitting.

Version 2 measured the margin under 60 s with 50 and 55 s steps inside the continuation chats. Here H1 and H2 measure the margin with the sleep server, and S1 runs its steps at half length (`--step-scale 0.5`: 15 to 27.5 s), so a cut in S1 never stands in for a failure to continue. Version 2's delayed-approval chat D becomes H3.

## What it decides

### H1, H2: the longest call that returns
For each call, from `steps.py h.jsonl --since <T> --until <T-next>` and the chat's words:

| the chat shows | the call line ends | it is |
|---|---|---|
| the tool's result, `slept <N> s` | `slept <N>` | **answered** |
| an error, a timeout or a disconnect | `cancelled <t>` | **cut, cancel sent** at t s |
| an error, a timeout or a disconnect | `slept <N>` | **cut, no cancel**: the server answered a call the host had dropped |
| an error, a timeout or a disconnect | `stdin_closed`, `signal` or `stdout_error` with `running 1` | **cut, the process closed** |
| a refusal that the tool is busy | `refused busy (inferred)` | **a second call while one ran** (the server logs nothing for it; `steps.py` infers it). Recorded, with whether the model or the host sent it, as far as the chat's words tell. |
| anything else, an end marked `(pid uncertain)`, or the call line is `open` 10 minutes after its `received` time | | goes to the run coordinator as it is, with the window's raw lines of `h.jsonl` |

An answered end reads `slept <elapsed> of <seconds asked>`. A `cancel_ignored` line naming the call means the host sent a cancel after the answer had gone.

The tab's result is the longest N answered and the shortest N cut, with the time of the cut from the chat (the time the chat showed the error) and from the call line. A 55 s call answered on both tabs is the margin the proposal's question 3 needs; one run per length is a sighting, recorded as partial.

### H3: from the prompt or from the answer
Two chats on the Chat tab, each one 45 s call. In H3a the approval is held 5 s after the prompt shows; in H3b it is held 30 s. From `approvals.txt` and the call line's `received` time:
- the host sends the call when the person approves if `received` is within 2 s of the approval's time. If it is near the prompt's time instead, record that; the rest of this reading then does not apply.
- **from the prompt:** H3a answered and H3b cut, at about 60 s after the prompt showed.
- **from the answer:** both answered, each about 45 s after its approval.
- anything else goes to the run coordinator as it is.

If the Chat tab shows no tool prompt for the sleep server, H3 cannot be read: record `no prompt` and skip it.

### H4: the same process after a cut
On each tab, one chat: a 75 s call, which row H says is cut at about 60 s. After the chat shows the cut, wait until the call line is no longer `open`, or 120 s after its `received` time, whichever comes first. Then type the second prompt, a 5 s call, in the same chat.
- **The same process:** no `started` time on the `started` line falls between the 75 s call's `received` time and the 5 s call's.
- **A new process:** one does. Then record any `closed` line (`stdin_closed` or `signal`: the host closed a process) by its time, against the `started` times and the call lines. A sleep-server close after the second start is marked `(pid uncertain)`: which process it closed is not told apart by the log, and the record says so.
- Record how the 75 s call ended (H1's table) and whether the 5 s call answered.

The sleep server writes a pid only on its `started` line. So H4 is read from the `started` and `closed` times, never from a call line's or a `closed` line's pid (`steps.py`, docstring). If the window holds more than one `started` time, the window's raw lines of `h.jsonl` are copied into the record beside `steps.py`'s output.

### S1: continuation by the same arguments
Each chat is read on its own, from `steps.py a.jsonl --since <T> --until <T-next>`, with the chat's words and the approval log. **The rows are read from the top; the first row that matches decides that chat.**

| # | this chat shows | then |
|---|---|---|
| 1 | `pid_changes` above 0 | **The host started a new server between calls.** A read parked in one process does not survive that host, whatever the model did. Goes to the run coordinator as it is. |
| 2 | `cut` above 0, or the chat showed an error for a call | **A step was cut** at no more than 27.5 s. Record the step, `max_cut_elapsed` and the approval log for that call. Goes to the run coordinator: H1 and H2 say whether it should have been. |
| 3 | `busy` above 0 | **Two calls overlapped.** Either the host gave up on a call without telling the server and the model called again, or the model sent two calls at once. The log does not tell these apart; the chat's words may. Goes to the run coordinator as it is; S1 is not kept in that chat. |
| 4 | the app stopped the turn at a cap, or asked whether to continue (its own control, not the model's words) | **A host cap.** Record the call count it stopped at and its text. Goes to the run coordinator: a hard cap bounds the steps a read may plan. |
| 5 | `continued_leftover` above 0 | **The chat ran a step of an earlier chat's read** (the same arguments, within 15 minutes). Goes to the run coordinator as it is; not counted for or against S1. |
| 6 | `reads_started` above 1, `dropped` above 0, `expired` above 0, `new_read_after_answered_step` above 0, `distinct_arguments` above 1, or `refused` above 0 | **S1 fails in that chat:** the model changed the arguments (even an argument sent on one call at its default and left out on another), started over, waited past the idle limit, or sent an argument the tool does not serve (a `read_id` is refused as `argument_unknown:read_id`). |
| 7 | the model stopped before `done`, asked the person anything before the answer, or needed the nudge | **S1 fails in that chat.** A nudge is recorded, never averaged away. |
| 8 | one `read` line, `steps <n> of <n> done true` with n the plan's, `vouchers_calls` n, `answered` n, `cut 0`, `busy 0`, `refused 0`, `open 0`, `continued_leftover 0`, `distinct_arguments 1`, `other_calls_not_answered 0`; the chat showed no error for any call; the chat's answer names the party `Lab Party <code>` with the `read` line's code; no nudge | **S1 kept in that chat.** |

`leftover_dropped` above 0 means the chat's first call dropped an earlier chat's unfinished read. It is recorded and does not fail the chat; `dropped` and `expired` count only the chat's own reads.

A chat that fits no row goes to the run coordinator as it is. "The chat showed no error" is part of row 8 because a host that gives up on a call without telling the server leaves the server's log looking whole: the chat's words are the only trace, so the lab session copies them in full.

**A tab keeps S1 only when all six of its chats (C1 to C6, or K1 to K6) meet row 8.** Five of six is a fail, recorded with the chat that failed. Each plan's 3 of 3 is reported on its own, so a tab that keeps the week and fails the month says so.

**Tool prompts are the host's, not the model's.** If a tab asks approval on every step, that is recorded per tab (prompts per call) and does not fail S1: the proposal needs to know it, because a click per step is the person's cost of a stepped read.

### T1: what each host declares
From every `initialize` line in `h.jsonl` and `a.jsonl`: the client's name, its protocol revision, its capability keys and `tasks yes|no`. An `initialize` is put to a tab only by its client name, or by its time against the first use of that tab after a start. If the two tabs cannot be told apart that way, record "not told apart"; that answers the proposal's question 3(b) for this app.

**Also recorded, whatever a chat shows:** the `meta` keys of each call.

The row decides nothing about Tally, about Bridge's tool text beyond what is served here, about macOS, or about another version of the app.

## Rules
`LAB-COMMON.md` in this folder is a byte copy of v3 (sha256 5370c6d2…). Its Tally rules C1 to C11 apply only so far as they touch this row: no Tally request is sent, and no key goes to Tally. Tally is left exactly as the row finds it, and its window is not captured. Where LAB-COMMON and this sheet differ, the stricter one holds.

**Who acts.**
- **The owner, at the PC, in his own PowerShell window** runs every `desktop_config.py` `add-sleep`, `add` and `restore` line. He quits and starts the app by hand, opens each new lab session with the fixed words, and approves the one access request for the Claude app at the start of the sitting. He switches the ComplyEaze Bridge extension off and back in the app's settings. If the lab session cannot drive the Chat tab, the owner opens each new chat, types each prompt from "Prompts the owner types", answers each tool prompt by the dialog table and types the nudge when it is due.
- **The lab session** prepares the folder, checks the hashes, runs `desktop_config.py compare` and `steps.py` (both only read), drives the Chat tab if it has access, keeps the approval log and writes the record. It never runs `add-sleep`, `add` or `restore`, never quits or starts the app, and never ends a process. **The lab session never calls the servers' tools itself, and never types anything into a chat but the prompt and, where the sheet says, the second prompt or the nudge.** It has read this sheet, so its own words would measure nothing.

**Across a quit.** The lab session runs inside the app, so a quit ends it. There is no detached hand-over and no forced process end. `C:\bridge-lab\703-a\handover.txt` is the one record of every change to the configuration. Every writer of `handover.txt`, the lab session included, appends UTF-8:
- before each quit, the lab session appends the step, the before-copy paths and the UTC time, then writes the step's lines with every value filled in to `C:\bridge-lab\703-a\owner-lines-step<N>.txt`, one command per line, in order, and tells the owner that file's path. Writing this file changes nothing in the app's configuration;
- the owner quits the app with the tray icon's Quit, opens that file and pastes its lines one at a time, in order, in his own PowerShell window. He replaces nothing. Each line shows its output and appends its command line, output and exit code to `handover.txt`;
- the owner stops at the first exit code that is not 0, pastes no later line and leaves the app shut. This is a STOP. A command line with no output after it is a line that was cut off while it ran;
- only if every line exited 0 and met its EXPECT does the owner start the app by hand and open a new session with the fixed words. The new lab session records the start in its report.

**The next session.** The owner opens a new lab session in the app after each start, with these first words, exactly: `Read C:\bridge-lab\703-a\handover.txt and go on with row 703-A from the step it names.` That session reads `handover.txt` first and checks each EXPECT against the lines there before anything else. A command line with no output after it, an exit code that is not 0, or an EXPECT not met, is a STOP. After a STOP, whoever opens the app next reads `handover.txt` for the configuration's state.

**The owner's lines.**
The lines below are the template the lab session fills in; the owner never fills in a placeholder. Here `<config>` means `%LOCALAPPDATA%\Claude-Data\claude_<pkg>\claude_desktop_config.json`, where `<pkg>` is the app's package name. Before writing each owner-lines file, the lab session fills in the real configuration path, real B0 and real before-copy path, and checks the value substituted for `<config>` for a single quote, a double quote or a space. If it finds any, it stops before writing the file (the pasted form passes the path unquoted, so such a path would be split or cut). The pasted form itself was not run before this sitting: step 1's line is its first use, and a `handover.txt` that does not then show the command, its output and `exit code 0` is a STOP. Step 1 needs no B0. For steps 5 and 7, B0 comes from the `add ok` line of step 1 in `handover.txt`. Each pasted line shows the command's output with `$o` and appends the command, its output and the exit code to `handover.txt` as UTF-8.

**Step 1, setup for phase H:**
```powershell
$c = 'python -I C:\bridge-lab\703-a\kit\desktop_config.py add-sleep <config> C:\bridge-lab\703-a\desktop-config-before-h.json C:\bridge-lab\703-a\kit\mcp-sleep-server.mjs C:\bridge-lab\703-a\log\h.jsonl'; $o = cmd /c "$c 2>&1" | Out-String; $o; Add-Content -Encoding utf8 C:\bridge-lab\703-a\handover.txt ("> $c`r`n" + $o + "exit code $LASTEXITCODE")
```

EXPECT `add ok`, B0, B1, and step 0's server names plus `bridge-lab-703h`. Anything else: STOP, with the configuration as the line says.

After this group, in this order: **the lab session appends its note and writes `owner-lines-step1.txt`; the owner quits the app (tray icon, Quit); the owner opens that file and pastes its lines one at a time, in order; only if every line exited 0 and met its EXPECT, the owner starts the app, opens a new session and types the fixed words**.

**Step 5, from phase H to phase S:**
```powershell
$c = 'python -I C:\bridge-lab\703-a\kit\desktop_config.py restore <config> C:\bridge-lab\703-a\desktop-config-before-h.json <B0>'; $o = cmd /c "$c 2>&1" | Out-String; $o; Add-Content -Encoding utf8 C:\bridge-lab\703-a\handover.txt ("> $c`r`n" + $o + "exit code $LASTEXITCODE")
```

EXPECT `restore ok B0`.

```powershell
$c = 'python -I C:\bridge-lab\703-a\kit\desktop_config.py add <config> C:\bridge-lab\703-a\desktop-config-before-s.json C:\bridge-lab\703-a\kit\stepped-vouchers-server.mjs C:\bridge-lab\703-a\kit\tools-args.json C:\bridge-lab\703-a\log\a.jsonl arguments 0.5'; $o = cmd /c "$c 2>&1" | Out-String; $o; Add-Content -Encoding utf8 C:\bridge-lab\703-a\handover.txt ("> $c`r`n" + $o + "exit code $LASTEXITCODE")
```

EXPECT `add ok` with the same B0 as step 1, and step 0's server names plus `bridge-lab-703a`. A different B0: STOP.

After this group, in this order: **the lab session appends its note and writes `owner-lines-step5.txt`; the owner quits the app (tray icon, Quit); the owner opens that file and pastes its lines one at a time, in order; only if every line exited 0 and met its EXPECT, the owner starts the app, opens a new session and types the fixed words**.

**Step 7, restore:**
```powershell
$c = 'python -I C:\bridge-lab\703-a\kit\desktop_config.py restore <config> <the before-copy of the last add> <B0>'; $o = cmd /c "$c 2>&1" | Out-String; $o; Add-Content -Encoding utf8 C:\bridge-lab\703-a\handover.txt ("> $c`r`n" + $o + "exit code $LASTEXITCODE")
```

EXPECT `restore ok B0`.

After this group, in this order: **the lab session appends its note and writes `owner-lines-step7.txt`; the owner quits the app (tray icon, Quit); the owner opens that file and pastes its lines one at a time, in order; only if every line exited 0 and met its EXPECT, the owner starts the app, opens a new session and types the fixed words**.

**The ComplyEaze Bridge extension is switched off** by the owner in the app's settings for the row (step 1) and back to its step 0 state at step 8, because this changes the app's configuration. A model that called a ComplyEaze Bridge tool could send a request to Tally. Its state before and after is recorded.

**One call at a time.** Both servers refuse a second call while one runs, in band (`busy`). The sleep server logs nothing for that refusal; `steps.py` ends such a call as `refused busy (inferred)`. Nothing is typed while a call runs.

**At most plan + 4 tool prompts per S1 chat:** 9 in a week chat, 16 in a month chat. **At most 3 per H chat, and 4 in H4's** (its second prompt is one). The next is denied, and the chat ends there.

**Code-tab reachability decides the gate.** A ComplyEaze Bridge tool can be reached from a Code-tab chat on this PC because the desktop extension's servers are shared into Code sessions, although the Code tab's own files name none. The sitting is therefore Chat-tab only, recorded as such and not a STOP: every Code-tab chat is skipped and owed to a later sitting. Step 0 records both the Code tab's files and the `mcp__` tool names a Code session lists; the gate is what the session can reach, not only what its files name. A tool prompt for any other tool is denied and recorded (the dialog table). A ComplyEaze Bridge tool call that ran with no prompt, or any request seen at Tally, is a STOP.

## Build and files
- No ComplyEaze Bridge build.
- **Phase H's server** is `mcp-sleep-server.mjs`, a byte copy of master `7a3c9e49`'s `scripts/mcp-sleep-server.mjs` (sha256 in `SHA256.txt`; `git show 7a3c9e49:scripts/mcp-sleep-server.mjs` prints the same bytes). Its one tool is `sleep` with `seconds`; it logs every message received, whole, and every result and cancel. Its entry is `bridge-lab-703h`.
- **Phase S's server** is `stepped-vouchers-server.mjs` in this folder, run with `--continue-by arguments --step-scale 0.5`. Its entry is `bridge-lab-703a`. It serves two tools, `list_companies` and `vouchers`, and the server instructions, all read from `tools-args.json`. It refuses to start if the text and the mode disagree (`loadTools`).
- **The text the model sees is Bridge's.** `tools-args.json` is written by `extract_text.py <folder> e24f1244 arguments` from master e24f1244's `src-tauri/src/` files `agent_protocol.rs`, `agent_catalog.rs`, `agent_import.rs` and `agent_voucher_search.rs`, each piece taken exactly once or the extractor fails, and put together as Bridge puts it together: the server instructions (`SERVER_INSTRUCTIONS` and the closing sentence of the default bundle, which prepares but does not post); the `list_companies` and `vouchers` descriptions, each followed by the read-receipt sentence every read tool carries, with the read tools' annotations (`readOnlyHint` true); and the whole shipped `vouchers` input schema, with each parameter's description by the first matching arm of `parameter_description`. Every object is served with its keys sorted, as Bridge's `serde_json` sends them. It was run on the build Mac on the four files as `git show e24f1244:<path>` prints them; anyone can repeat it and compare the hash in `SHA256.txt`. The lab PC checks the hash only. Without the third word it writes version 2's `tools.json`, byte for byte (`tried.txt`).
- **What differs from what Bridge ships, and is the one thing S1 varies:** three planned texts the build would add (`extract_text.py`, `PLANNED_STEP_SENTENCE_ARGS`, `PLANNED_INSTRUCTION_ARGS`, `NEXT_STEP_ARGS`): one sentence appended to the server instructions, one paragraph appended to the `vouchers` description, and the `next_step` a `done: false` result carries ("Call vouchers again with exactly the same arguments."). No `read_id` is served or accepted. They are listed under `planned` in `tools-args.json`.
- **How the server continues** (`stepped-vouchers-server.mjs`, `vouchersCall`): it holds one live read. A `vouchers` call whose other arguments equal the live read's, with a schema default standing for an argument left out, continues it. Any other drops it (`read_dropped`) and starts a new read; so does the same call after 15 minutes idle (`read_expired`). A finished read is never continued, so the same arguments after it start a new read. A cancelled step does not count, so the next call runs it again.
- **Stated differences that are not under test:** the results are in Bridge's shape (`company`, then `result`; refusals as `vouchers: refused` with a code, `isError` true), but every value is invented here: one company, `Lab Stepped Traders`, and three vouchers, two with the party `Lab Party <code>` (a five-letter code minted per read) and one with `Lab Rent`. The server names itself `bridge-lab-703a`, declares only `tools` and answers the protocol revision the host asks for. The server instructions are e24f1244's: master `7a3c9e49` adds one sentence about `tally_status`, a tool not served here.
- **The plans** (`PLANS`, at `--step-scale 0.5`): a window of up to 10 days is `five`, 20, 25, 20, 27.5 and 15 s; a longer one is `twelve`, 20, 20, 25, 20, 20, 27.5, 20, 20, 25, 20, 27.5 and 15 s. Every log line carries the pid; every message received is logged whole, `initialize` capabilities and any `_meta` included.
- Copy the folder to `C:\bridge-lab\703-a\kit\`. The logs are `C:\bridge-lab\703-a\log\h.jsonl` (phase H) and `C:\bridge-lab\703-a\log\a.jsonl` (phase S); neither may exist at its `add` (`desktop_config.py` refuses if it does).
- The Code-tab chats each use a new empty folder, `C:\bridge-lab\703-a\empty-<n>\`, as the session folder, so no session can read this sheet.

| file | what |
|---|---|
| `ROW.md` | this sheet |
| `LAB-COMMON.md` | v3, byte copy |
| `mcp-sleep-server.mjs` | phase H's server, a byte copy of master `7a3c9e49`'s |
| `stepped-vouchers-server.mjs` | phase S's server; `--continue-by id` (the default) is version 2's server |
| `tools-args.json` | the text served in phase S, from `extract_text.py ... arguments` |
| `tools.json` | version 2's text (S2), not served in this sitting |
| `extract_text.py` | writes either text from Bridge's source |
| `desktop_config.py` | row H's helper: `add-sleep` adds `bridge-lab-703h`; `add ... arguments 0.5` adds `bridge-lab-703a`; each keeps the before-copy (B0) and refuses, leaving nothing behind, if either entry is already there; a write that fails says whether the configuration is unchanged (the before-copy is then removed) or gives its sha256; `restore` puts those bytes back and checks them by sha256; `compare` lists server names after a start |
| `steps.py` | reads either log into one line per call (with the gap before it), one per read, a `started` line with each process's start time, `closed`, `cancel_ignored` and `live_reads` lines, a `sleep` line, and the `facts` line, between `--since` and `--until` |
| `prescreen.sh` | the macOS CLI pre-screen (below); not run on the lab PC |
| `tried.txt` | the build Mac's smoke runs of both servers and both modes, with `steps.py`'s output |

`SHA256.txt` lists them all. Check it first.

## The prompts
Typed once, exactly, at the start of a new chat or session.

Phase H, with N the call's seconds:
- **First prompt:** `Call the sleep tool once with seconds set to <N>, then tell me in one line what it returned.`
- **Second prompt (H4 only, in the same chat):** `Call the sleep tool once more with seconds set to 5, then tell me in one line what it returned.`

Phase S:
- **Week (W):** `Show me my company's vouchers from 2 to 8 November 2026.`
- **Month (M):** `Show me my company's vouchers for October 2026.`

The phase S prompts give the dates, so the server instructions' rule to ask before a read over more than one month does not apply, and they name no company, so with one company open the instructions say to use it.

**The one nudge (phase S).** If the model stops before the answer, asks a question, or says it will go on and calls nothing, whoever types the prompt (the lab session with access, otherwise the owner) types once, exactly: `Yes, carry on until it is done.` A second stop ends the chat. Each nudge is recorded with its time and the model's words before it. Any nudge fails that chat (row 7); the nudge is there to see whether the read could finish, not to pass it. Phase H has no nudge: a chat that does not call the tool is recorded and ends.

## Prompts the owner types
Use this section only if the lab session cannot drive the Chat tab. The owner opens each new chat and types the exact text below in this order. He answers tool prompts as the table says. The nudge is due only if the model stops before the answer, asks a question, or says it will go on and calls nothing. He reports every dialog and copies every word the chat shows for the lab session's record. The lab session never shows him this sheet's expected results while a chat runs.

| chat | exact prompt text | answer to each tool prompt | nudge |
|---|---|---|---|
| H1-30 | `Call the sleep tool once with seconds set to 30, then tell me in one line what it returned.` | **Allow once**, by mouse, at once; never an "always" choice. Cap 3; past it, **Deny**. Any other tool: **Deny**. | None. |
| H1-45 | `Call the sleep tool once with seconds set to 45, then tell me in one line what it returned.` | **Allow once**, by mouse, at once; never an "always" choice. Cap 3; past it, **Deny**. Any other tool: **Deny**. | None. |
| H1-55 | `Call the sleep tool once with seconds set to 55, then tell me in one line what it returned.` | **Allow once**, by mouse, at once; never an "always" choice. Cap 3; past it, **Deny**. Any other tool: **Deny**. | None. |
| H3a | `Call the sleep tool once with seconds set to 45, then tell me in one line what it returned.` | **Allow once**, by mouse, held 5 s after the prompt shows; never an "always" choice. Cap 3; past it, **Deny**. Any other tool: **Deny**. | None. |
| H3b | `Call the sleep tool once with seconds set to 45, then tell me in one line what it returned.` | **Allow once**, by mouse, held 30 s after the prompt shows; never an "always" choice. Cap 3; past it, **Deny**. Any other tool: **Deny**. | None. |
| H4-C | `Call the sleep tool once with seconds set to 75, then tell me in one line what it returned.` After the chat shows the cut, when the lab session says the H4 wait is over, type `Call the sleep tool once more with seconds set to 5, then tell me in one line what it returned.` | **Allow once**, by mouse, at once for each prompt; never an "always" choice. Cap 4; past it, **Deny**. Any other tool: **Deny**. | None. |
| C1 | `Show me my company's vouchers from 2 to 8 November 2026.` | **Allow once**, by mouse, at once; never an "always" choice. Cap 9; past it, **Deny**. Any other tool: **Deny**. | If due, once: `Yes, carry on until it is done.` A second stop ends the chat. |
| C2 | `Show me my company's vouchers from 2 to 8 November 2026.` | **Allow once**, by mouse, at once; never an "always" choice. Cap 9; past it, **Deny**. Any other tool: **Deny**. | If due, once: `Yes, carry on until it is done.` A second stop ends the chat. |
| C3 | `Show me my company's vouchers from 2 to 8 November 2026.` | **Allow once**, by mouse, at once; never an "always" choice. Cap 9; past it, **Deny**. Any other tool: **Deny**. | If due, once: `Yes, carry on until it is done.` A second stop ends the chat. |
| C4 | `Show me my company's vouchers for October 2026.` | **Allow once**, by mouse, at once; never an "always" choice. Cap 16; past it, **Deny**. Any other tool: **Deny**. | If due, once: `Yes, carry on until it is done.` A second stop ends the chat. |
| C5 | `Show me my company's vouchers for October 2026.` | **Allow once**, by mouse, at once; never an "always" choice. Cap 16; past it, **Deny**. Any other tool: **Deny**. | If due, once: `Yes, carry on until it is done.` A second stop ends the chat. |
| C6 | `Show me my company's vouchers for October 2026.` | **Allow once**, by mouse, at once; never an "always" choice. Cap 16; past it, **Deny**. Any other tool: **Deny**. | If due, once: `Yes, carry on until it is done.` A second stop ends the chat. |

T1 on the Chat tab follows H4-C and precedes C1. It needs no new chat or prompt: read its `initialize` evidence from `h.jsonl` and `a.jsonl` as "What it decides" says.

## The chats, in this order
| chat | tab | prompt | approval |
|---|---|---|---|
| H1-30, H1-45, H1-55 | Chat | first prompt, N 30, 45, 55 | at once |
| H3a, H3b | Chat | first prompt, N 45 | H3a held 5 s after the prompt shows; H3b held 30 s |
| H4-C | Chat | first prompt, N 75; then the second prompt (H4) | at once |
| T1 | Chat | no new chat; read the `initialize` lines | none |
| C1, C2, C3 | Chat | W | at once |
| C4, C5, C6 | Chat | M | at once |

**Skipped and owed to a later sitting:** H2-30, H2-45, H2-55, H4-K, K1, K2, K3, K4, K5, K6, and T1 on the Code tab. Record each as skipped.

Each Chat-tab chat is new. Nothing is resumed. Before each, `steps.py` on the phase's log shows no `open` call.

## The approval log
For every tool prompt, one line in `approvals.txt`: the chat, the prompt's number in that chat, the tool and arguments it shows, the UTC time it was first seen, the UTC time Allow once was clicked, and what was clicked. Times from `[DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ')`, run at the moment of each, so the log is the lab session's own clock, to the second at best; the server's `received` time is the exact moment the host sent the call. A call that ran with no prompt is a line too: `no prompt`. For a held approval (H3), the hold is timed from the "first seen" time, and the click is made when the clock reads that time plus the hold. If the owner types, he tells the lab session when each prompt appears and what it shows, and when he clicks; the lab session runs the time command at each report and keeps the log. For H3, the owner uses the first-seen time and the stated hold before clicking.

## Every dialog and prompt this row may meet, and its answer
Anything not in this table, in the app, in Windows or in Tally, is a STOP. Take a screenshot, copy its text exactly, press no key, and report.

| where | what shows | answer |
|---|---|---|
| either tab | a tool prompt for `sleep` (phase H), or `list_companies` or `vouchers` (phase S), within the chat's cap | **Allow once**, by mouse, at once (H3: as its row says); never an "always" choice. Log it. |
| either tab | a tool prompt past the chat's cap | **Deny.** The chat ends. |
| either tab | a tool prompt for any other tool | **Deny.** Record it. |
| either tab, phase S | the model asks a question or stops before the answer | the nudge above, once |
| either tab | the app's own control saying a tool-call limit was reached, or asking whether to continue | Record its text and the call count. Click its continue control once; a second one ends the chat. Row 4 decides the chat. |
| either tab | an error that the server did not respond, timed out, failed or disconnected | Click nothing. Copy its text and the time it showed. Phase H: the chat ends, except H4, which goes on to its second prompt. Phase S: the chat goes on only if the model goes on by itself. |
| the app | a banner or button offering an update or a relaunch | Not clicked. Record its text. A modal window over the app: STOP. |
| the app, at the start of the sitting | the one access request for the Claude app | The owner approves it once. If he refuses, use "Prompts the owner types" and record that case; not a STOP. |
| the app, at a start | the app opens on its last chat, or offers to resume a session | Nothing is resumed. Open a new chat or session. |
| the app, at a quit | none expected | any window that asks to confirm, save or keep: STOP |
| Windows | a security or permission prompt (for `node.exe` or the app) | STOP, nothing allowed |
| Tally | nothing is keyed and nothing is sent; any change on Tally's screen | STOP |

## Steps
One at a time. Every answer goes to a file, whole. All times are in UTC.

0. **Before.**
   - The lab session checks `SHA256.txt`.
   - Record `node --version`, the app's version (`Get-AppxPackage`, name and version only) and the Code tab's version from its own `--version`.
   - Record the names and values of any `MCP_TIMEOUT` or `MCP_TOOL_TIMEOUT` in the user and machine environment (row H recorded none of the Code tab's settings), or that none is set.
   - Record the app's configured server names (names only) and whether the ComplyEaze Bridge extension is on.
   - The Code tab's own MCP servers and allow rules, names only, read from its files and never by a command that starts a server: `mcpServers` at the top of `.claude.json` in the user's folder and under each of its projects; `permissions.allow` entries beginning `mcp__` and `enabledPlugins` in `.claude\settings.json` there; for each enabled plugin, the servers its own folder declares; and a managed settings file, if the machine has one. A server that none of these names (one an administrator pushed by another route) is not covered, and the record says the check covers these files only.
   - Add one line recording which `mcp__` tool names the existing Code session lists, names only, without calling a tool. Together with the file record, this decides the gate by what a Code session can reach. On this PC it includes a ComplyEaze Bridge tool, so the sitting is Chat-tab only (Rules).
   - The owner approves the one access request for the Claude app. If he refuses it, or the app hides its own window from the lab session, use "Prompts the owner types": the owner drives every Chat-tab chat and the lab session only reads the logs. Record which case held: `lab session drives Chat tab` or `owner types prompts; lab session reads logs`. This is not a STOP. A Windows security prompt remains a STOP.
1. **Setup for phase H.**
   - The owner switches the ComplyEaze Bridge extension off in the app's settings. Record that it shows off.
   - The lab session appends the step, before-copy path and UTC time to `handover.txt`. It fills in step 1's template with the real configuration path and before-copy path (no B0 is needed), writes `C:\bridge-lab\703-a\owner-lines-step1.txt` and tells the owner the path. The owner opens that file and pastes its line. EXPECT `add ok`, B0, B1, and step 0's server names plus `bridge-lab-703h`. Anything else: STOP, with the configuration as the line says.
   - After the start, the new lab session hashes the before-copy: it must be B0.
2. **After the start** (a new lab session opened by the owner with the fixed words, which reads `handover.txt` first).
   - The lab session runs `desktop_config.py compare <config> C:\bridge-lab\703-a\desktop-config-before-h.json`. EXPECT `added bridge-lab-703h`, `removed -`. A rewrite by the app is recorded, not a STOP.
   - `steps.py h.jsonl`: record each `initialize` line. No `started` within 2 minutes of the start: STOP; the entry did not load, and the restore is step 7.
3. **Phase H, the Chat tab:** H1-30, H1-45, H1-55, H3a, H3b, H4-C. For each: the lab session with access, otherwise the owner, opens a new chat; the lab session writes down T, from the approval log's time command, in the form `2026-11-10T09:30:00.000Z` (`steps.py` refuses any other); the driver types its prompt and answers each tool prompt by the table, with the lab session logging it. Record every word the chat showed, with the time of each tool prompt, each result and each error as the chat shows them, and the chat's final words, copied exactly. Then the lab session runs `steps.py h.jsonl --since <T> --until <T of the next chat, or omitted for the last>`, whole, to `steps-<chat>.txt`. (Run each `--until` once the next chat's T is written, so every file is complete.)
4. **T1 on the Chat tab, then the Code-tab skips.** Read the Chat tab's `initialize` evidence from `h.jsonl` as "What it decides" says. Record each phase H Code-tab chat and T1 on the Code tab as skipped and owed, as the skipped list says. Then `steps.py h.jsonl` with no window to `steps-h-all.txt`.
5. **From phase H to phase S.**
   - The lab session appends the step, both before-copy paths and the UTC time to `handover.txt`. It fills in step 5's two template lines with the real configuration path, both real before-copy paths and B0 from step 1's `add ok` line in `handover.txt`, writes `C:\bridge-lab\703-a\owner-lines-step5.txt` and tells the owner the path. With the app shut, the owner opens that file and pastes its lines in order: first restore, then add. Check both EXPECTs there.
   - After the start, the new lab session reads `handover.txt`, checks both EXPECTs there, and runs `desktop_config.py compare <config> C:\bridge-lab\703-a\desktop-config-before-s.json`: EXPECT `added bridge-lab-703a`, `removed -`. `steps.py a.jsonl`: record each `initialize` line; no `started` within 2 minutes: STOP, and the restore is step 7.
6. **Phase S:** C1 to C6 on the Chat tab, as step 3, with the phase S prompts and the nudge, each chat's window read from `a.jsonl`. Record each phase S Code-tab chat as skipped and owed, as the skipped list says. Before each chat, `steps.py a.jsonl` with no window: a `live_reads` line other than `-` is written down, and that chat is recorded as "after a leftover". Record any cap or continue control and each nudge with the words before it. Then `steps.py a.jsonl` with no window to `steps-a-all.txt`.
7. **Restore.**
   - The lab session appends the step, the before-copy path and the UTC time to `handover.txt`. It fills in step 7's template with the real configuration path, the real before-copy path of the last add and B0 from step 1's `add ok` line in `handover.txt`, writes `C:\bridge-lab\703-a\owner-lines-step7.txt` and tells the owner the path. The owner opens that file and pastes its line. EXPECT `restore ok B0`.
   - After the start, the new lab session reads `handover.txt`, checks the EXPECT there, and runs `desktop_config.py compare <config> <that before-copy>`: EXPECT no server added or removed. A rewrite on start is recorded, not a STOP.
8. **Close.**
   - The owner switches the ComplyEaze Bridge extension back to its step 0 state in the app's settings. Record it.
   - No `node.exe` runs either of this folder's servers (`Get-CimInstance Win32_Process`, read only). One still running is recorded; nothing is ended by hand.
   - Retire `C:\bridge-lab\703-a\` for the answers.

Phase H's calls take about 15 minutes; a week chat takes about 2 minutes of steps and a month chat about 4.5, so phase S takes about 40. With approvals and three starts the sitting takes about 1 h 15 min.

## What to report
| step | result |
|---|---|
| versions: app, Code tab, node; kit hashes; the timeout variables | |
| access: which case held; the Code tab's file record and the `mcp__` tool names its session listed | |
| setup: B0, B1 of each add, servers before and after; the extension off | |
| T1: Chat tab's `initialize` lines: client, protocol, capability keys, `tasks`; the `meta` keys the calls carried; T1 on the Code tab skipped and owed | |
| H1, H2: H1's each N answered or cut (and how, by H1's table), with the chat's error time and the call line's end; each H2 chat skipped and owed | |
| H3: H3a and H3b, the prompt, approval and `received` times, answered or cut; the reading | |
| H4: on the Chat tab, how the 75 s call ended, the `started` times, the same process or a new one, the old process's close, the 5 s call's end; H4-K skipped and owed | |
| S1: each chat C1 to C6: the row of "What it decides" it meets, its `read` and `facts` lines, nudges, the answer's party name against the `read` line's code; K1 to K6 skipped and owed | |
| S1 on the Chat tab: 3 of 3 week, 3 of 3 month, kept or not; prompts per call; any cap or continue control; the largest `gap` and its approval | |
| restore: each restore line (B0), the compare after the last start; the extension as at step 0 | |
| any dialog or prompt, with its text and the answer given | |

Read every number from its file. Put any STOP, skip, or EXPECT not met in the first line.

## Stop rules
- A dialog or prompt not in the table.
- A sha256 not as named: the kit, a before-copy, B0 at a restore, or a second B0 unlike the first.
- `desktop_config.py` refusing at an add or a restore.
- No `started` in a phase's log after its start.
- A call still `open` in `steps.py` 10 minutes after it was received.
- Anything sent to Tally, or any key in Tally.

If the sitting ends for any other reason (the PC sleeps, time runs out, the lab session ends), step 7 is run before the sitting closes, or a line in `handover.txt` and in the report names the entry still in the configuration and its before-copy.

After a STOP, the restore (step 7) waits for the run coordinator's word. The report lists the configuration's state, the extension's state, and whether the app is running.

## The macOS CLI pre-screen (before the sitting, not on the lab PC)
`sh prescreen.sh <empty folder> 3 sonnet arguments` runs phase S's server and `tools-args.json` through `claude -p` on a Mac: three week and three month runs, each a new session with only this server's two tools allowed and no other MCP server, at `--step-scale 0.05` (it measures the model, not a host's wait). It prints PASS for a run only on row 8's conditions that a log can show (not the chat's error, which `claude -p` prints in its answer). A FAIL there is reported before the sitting, with its `facts` line; the text is not changed to make it pass (one variable at a time). It does not stand in for S1: the host, its prompts and its wait differ. Its verdict lines go with this sheet's answers. Phase H does not depend on it.

## Limits
- One sitting, one app version, one PC, the model each tab chose (record its name if shown). One call per length in H1 and H2, one chat each in H3 and H4: sightings, recorded as partial. Three chats per plan per tab in S1: a 3 of 3 is a small sample, recorded as partial, not a rate.
- The text is Bridge's at e24f1244 plus the planned texts. A build that ships weaker stepping text than `tools-args.json`'s, or a different result shape for `done: false`, is not covered by this row.
- The servers wait; no Tally work runs. A real step's length varies with the book, and one operation of a real read is several sends (the proposal, "One thing a build must size"); O1 measures that, not this row.
- S1's server holds one live read and refuses a second call while one runs. Design 1 would queue a re-ask behind a running step; this server answers `busy`. So S1 says nothing about a re-ask after a cut, which is H4's and a later row's.
- The approval log is the lab session's clock read at each act, so the gap between a prompt and its approval is known to about a second.

## Keep
On `lab/703-a-answers`, with `SHA256.txt`:
- every `steps.py` output, `approvals.txt`, `handover.txt`, every `owner-lines-step<N>.txt` file, and every compare line;
- `h.jsonl` and `a.jsonl` (no names or values beyond the servers' own);
- each chat's and session's words, with account, organisation, device and conversation ids replaced by `<redacted>` (the raw file's sha256 recorded beside it);
- the versions and the timeout variables;
- a `RESULTS.md` holding the table above.

Screens and the configuration copies stay on the lab PC, listed by name, size and sha256. Notes carry no user names and no machine paths beyond `C:\bridge-lab\703-a\`. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
