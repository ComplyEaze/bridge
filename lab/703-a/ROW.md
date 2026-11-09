# Row 703-A: does the model carry a server-issued `read_id` from call to call, on the desktop app's Chat tab and Code tab

Not for merge. Part of #703. Reads only: no Tally request, no book, no ComplyEaze Bridge build. Lines are cited in this folder's files; the server is derived from `scripts/mcp-sleep-server.mjs` at master 011dd96d. Confidence of everything here until the row runs: unverified.

## Question
#703's proposed direction (comment of 8 Nov) is a stepped read: a long read becomes several calls of about 40 s each, Bridge holds the plan under a `read_id` it mints, and only the last call carries the answer. That works only if the model, on its own, calls again with the id the step before returned, until the read is done.

On the Windows desktop app of the lab PC, given the prompt below, does the model:
- call `stepped_read` five times, each later call carrying the `read_id` the server issued, unchanged;
- not start a new read, invent an id, or ask the person before each step;
- and report the answer the last step returned?

And does each step of about 40 s come back whole on both tabs, which cut a 60 s call (row H)?

## Why
Row H (`lab/2119-h-answers` at `dfaf2737`, `docs/agent/mcp-host-call-timeouts.md`) measured about 60 s per call on both tabs, with no `progressToken`, so a read over about 60 s can only fit if it is split into calls. The stepped read is the split. Its one unmeasured dependency is the model's behaviour across calls. Nothing of the stepped read is built until this row keeps it (P1).

## What it decides
Each tab is read on its own, from `steps.py a.jsonl --since <T>`, where T is the UTC time written down just before that tab's prompt was sent (steps 3 and 4), and from the chat's words in the record. **The rows are read from the top; the first row that matches decides that tab.**

| # | this row shows, on one tab | then |
|---|---|---|
| 1 | `pid_changes` above 0, or an id in `ids_from_other_pid` | **The host started a new server between calls.** A plan held in one process does not survive that host, whatever the model did. Goes to the run coordinator as it is. |
| 2 | `evicted` names a read of this tab | The server dropped a read at its cap of four: the model started more reads than the row allows for. Goes to the run coordinator as it is. |
| 3 | `cut` or `busy` above 0, or the chat showed an error for a step | **The budget is too close to the host's wait,** or the wait counts from the tool prompt. Record `max_cut_elapsed`, `max_answered_elapsed` and the chat's times. The budget comes down before any build, and this row runs again on the lower budget. A `busy` is a step the host gave up without telling the server, which then refused the next call while it still ran. |
| 4 | `new_read_after_answered_step` above 0, or an id in `ids_not_issued`, or a `refused read_finished` | **A killed on that tab** (a new read once an id was given, an invented id, or a finished id reused). |
| 5 | the model stopped before `done`, or stopped again after the one nudge | **A killed on that tab.** Refusal with a next step stays. |
| 6 | one nudge was needed, then it carried through to the answer | **Not kept as built.** Goes to the run coordinator as it is. This row does not decide whether the first step's text may ask for one go-ahead. |
| 7 | `stepped_calls` 5, `answered` 5, `reads_started 1`, `reads_done 1`, `open 0`; the chat shows the answer the `read` line gives; no nudge | **A kept on that tab.** The `vouchers` slice of the design may be built, with tool text no weaker than this server's (Limits). |
| 8 | any `initialize` line with `tasks yes` (read alongside the row above) | Record it. #703's deferred background-job shape is revisited only on that. |

A result that fits no row goes to the run coordinator as it is. This row decides nothing about Tally, Bridge's own tool text, macOS or another version of the app. The two `initialize` connections row H saw at protocol 2025-11-25 declared no `tasks` capability (`lab/2119-h-answers`, `evidence/h.jsonl`); this row records it again.

## Rules
`LAB-COMMON.md` in this folder is a byte copy of v3 (sha256 5370c6d2…, as in `lab/2119-h-sheet`). Its Tally rules C1 to C11 apply only so far as they touch this row: no Tally request is sent, and no key goes to Tally. Tally is left exactly as the row finds it, and its window is not captured. Where LAB-COMMON and this sheet differ, the stricter one holds.

**Who acts.** The person at the lab PC (as in row H on 5 Nov):
- runs step 1's `add` and the restart, and step 5's restore, from their own PowerShell window: the lab session's own permission check refuses changes to the app's configuration (row H, notes at 11:29Z on 4 Nov);
- types the prompt, answers each tool prompt, and copies what the chat showed.
The lab session prepares the folder, checks the hashes, runs `steps.py` and writes the record. **The lab session never makes a `stepped_read` call itself:** it has read this sheet, so its own behaviour measures nothing.

**The configuration path.** The app is a packaged app: the path its own sessions see is redirected. `add` and `restore` use the real file outside the package, as row H's second try did (row H notes, 10:55Z on 5 Nov).

**The ComplyEaze Bridge extension is switched off** for the row, by the person, in the app's own settings, at step 1, and back on at step 5. A model that calls a ComplyEaze Bridge tool could send a request to Tally. Its state before and after is recorded.

**One call at a time.** The server refuses a second call while one runs, in band (`mcp-sleep-server.mjs:252`). The person does not type while a step runs.

**At most eight tool prompts per run.** The ninth and later are denied, and the run ends there. A model that keeps starting new reads shows that in the first eight.

## Build and files
- No ComplyEaze Bridge build. The server is `mcp-sleep-server.mjs` in this folder: master 011dd96d's `scripts/mcp-sleep-server.mjs` (sha256 `dc236903e0ff09d7b2819013dd122432ac0b59df257bc5094928d410a83451ad`) with a second tool, `stepped_read`, added, and the server's pid on every log line. The other new code here is `steps.py` (new) and `desktop_config.py` (three lines renamed). Neither is tested in this folder; both were run on a synthetic log before the sheet was sent. `SHA256.txt` gives its hash. Another sha256 = STOP.
- `stepped_read`: STEPS 5 and STEP_SECONDS 40 (`:40-41`), fixed in the file, no argument changes them. A first call mints the id (`:186-189`); each call waits 40 s (`:203`) and answers its step (`:205-214`); the fifth answers `done: true` and a six-letter answer. An id this process did not issue, a finished one and one idle past 15 minutes are refused in band as `read_unknown`, `read_finished` and `read_expired` (`:192-196`). Every log line carries the server's pid (`:289`).
- The tool's description, which is all the model is told besides the prompt (`:62-71`): "Dev-only measurement tool. Reads a result that takes several calls: each call works for about 40 seconds and returns. Call it first with no read_id. While the result says done is false, call it again with exactly the read_id it returned; carry on without asking the user, because each call is short. Only the last call, where done is true, carries the answer."
- Copy the server to `C:\bridge-lab\703-a\sleep\mcp-sleep-server.mjs`.
- The log is `C:\bridge-lab\703-a\sleep-log\a.jsonl`. It must not exist at setup; `desktop_config.py add` refuses if it does.
- The Code-tab run uses a new empty folder, `C:\bridge-lab\703-a\empty\`, as its session folder, so the new session cannot read this sheet.

| file | what |
|---|---|
| `ROW.md` | this sheet |
| `LAB-COMMON.md` | v3, byte copy |
| `mcp-sleep-server.mjs` | the server, with `stepped_read` |
| `desktop_config.py` | row H's helper with its one entry renamed `sleep-703a` (three lines differ from `lab/2119-h-sheet`); adds the entry and keeps the before-copy (B0); restores those bytes and checks them by sha256; compares server names after a start |
| `steps.py` | reads `a.jsonl` into one line per call, one per read, and the `facts` line |

`SHA256.txt` lists them all. Check it first.

## The prompt
Both runs use exactly this text, typed once at the start of a new chat or session:

`Use the stepped_read tool to get the result, and tell me the answer when it is done.`

**The one nudge.** If the model stops before the answer, asks a question, or says it will go on and calls nothing, the person types once, exactly: `Yes, carry on until it is done.` A second stop ends the run. Record each nudge with its time and the model's words before it.

## Every dialog and prompt this row may meet, and its answer
Anything not in this table, in the app, in Windows or in Tally, is a STOP. Take a screenshot, copy its text exactly, press no key, and report.

| where | what shows | answer |
|---|---|---|
| Chat tab | a tool prompt for `stepped_read`, any arguments (the first eight of the run) | **Allow once**, by mouse, at once; never the "always" choice. Copy the arguments it shows. |
| Chat tab | a ninth tool prompt for `stepped_read` | **Deny.** The run ends. |
| Chat tab | a tool prompt for `sleep` or any other tool | **Deny** |
| Chat tab | the model asks a question or stops before the answer | the nudge above, once |
| Chat tab | an error that the device did not respond, or that `sleep-703a` failed or disconnected | Click nothing. Copy its text. The run goes on only if the model goes on by itself. |
| Code tab | a permission prompt for `stepped_read` (none for `sleep` in row H) | as on the Chat tab |
| Code tab | any other tool runs or asks to run | Deny if asked. Record it either way. |
| the app | a banner or button offering an update or a relaunch | Not clicked. Record its text. A modal window over the app: STOP. |
| the app, at a start | the app opens on its last chat, or offers to resume a session | Nothing is resumed. Open a new chat or session for the run. |
| the app, at a quit | none expected (row H's restart helper) | any window that asks to confirm, save or keep: STOP |
| Windows | a security or permission prompt (for `node.exe` or the app) | STOP, nothing allowed |
| Tally | nothing is keyed and nothing is sent; any change on Tally's screen | STOP |

## Steps
One at a time. Every answer goes to a file, whole. All times are in UTC.

0. **Before.**
   - Check `SHA256.txt`. Check the server copy's sha256.
   - Record `node --version`, the app's version (`Get-AppxPackage`, name and version only) and the Code tab's version from its own `--version`.
   - Record the app's configured server names (names only) and whether the ComplyEaze Bridge extension is on.
1. **Setup** (the person).
   - Switch the ComplyEaze Bridge extension off in the app's settings. Record that it shows off.
   - `python -I desktop_config.py add <config> C:\bridge-lab\703-a\desktop-config-before.json C:\bridge-lab\703-a\sleep\mcp-sleep-server.mjs C:\bridge-lab\703-a\sleep-log\a.jsonl`. EXPECT `add ok`, B0, B1, and step 0's server names plus `sleep-703a`. Anything else: STOP, with the configuration as the line says.
   - Hash the before-copy: it must be B0.
   - Quit and start the app with row H's restart helper (`lab/2119-h-answers`, `scripts/restart-desktop.ps1`), as on 5 Nov.
2. **After the start.**
   - `desktop_config.py compare <config> C:\bridge-lab\703-a\desktop-config-before.json`. EXPECT `added sleep-703a`, `removed -`. A rewrite by the app is recorded, not a STOP.
   - `steps.py a.jsonl`: record each `initialize` line. No `started` within 2 minutes of the start: STOP; the entry did not load, and the restore is step 5.
3. **A-C, the Chat tab** (the person). A new chat. Type the prompt. Answer each tool prompt by the table.
   Record:
   - every word the chat showed between the prompt and the end, with the time of each tool prompt and each result as the chat shows them;
   - the arguments each tool prompt showed;
   - each nudge, and the words before it;
   - whether the chat's final words hold the answer, copied exactly.
   Just before typing the prompt, write down the UTC time to the millisecond as T-C (`[DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ss.fffZ')`).
   The run ends at the model's last reply, a second stop, or the ninth prompt. Then `steps.py a.jsonl --since <T-C>`, whole, to `steps-c.txt`.
4. **A-K, the Code tab** (the person). Wait until `steps.py` shows no `open` call. A new Code-tab session in `C:\bridge-lab\703-a\empty\`, not the lab's own session. Write down T-K as in step 3, then type the prompt. Answer as on the Chat tab. Record the same. Then `steps.py a.jsonl --since <T-K>`, whole, to `steps-k.txt`, and `steps.py a.jsonl` with no window to `steps-all.txt`.
5. **Restore** (the person).
   - Quit the app with the restart helper, set to close only. Before it starts again: `desktop_config.py restore <config> C:\bridge-lab\703-a\desktop-config-before.json <B0>`. EXPECT `restore ok B0`. Anything else: STOP, leave the app shut, report the configuration's state.
   - Start the app. `desktop_config.py compare`: EXPECT no server added or removed. A rewrite on start is recorded, not a STOP.
   - Switch the ComplyEaze Bridge extension back to its step 0 state. Record it.
6. **Close.**
   - No `node.exe` runs this folder's `mcp-sleep-server.mjs` (`Get-CimInstance Win32_Process`, read only). One still running is recorded; nothing is ended by hand.
   - Retire `C:\bridge-lab\703-a\` for the answers.

## What to report
| step | result |
|---|---|
| versions: app, Code tab, node; server sha256 | |
| setup: B0, B1, servers before and after; the extension off | |
| `initialize` lines: clients, protocols, capability keys, `tasks` | |
| A-C: each call's arguments, carried id, end; the `facts` line; nudges; the answer the chat showed against the `read` line | |
| A-K: the same | |
| the longest step elapsed on each tab, and any step the chat showed as an error | |
| restore: the restore line (B0), the compare after the start; the extension as at step 0 | |
| any dialog or prompt, with its text and the answer given | |

Read every number from its file. Put any STOP, skip, or EXPECT not met in the first line. Fill the "What it decides" row each tab meets.

## Stop rules
- A dialog or prompt not in the table.
- A sha256 not as named: the kit, the server copy, the before-copy, or B0 at the restore.
- `desktop_config.py` refusing at setup or restore.
- No `started` in `a.jsonl` after the start.
- A call still `open` in `steps.py` 10 minutes after it was received.
- Anything sent to Tally, or any key in Tally.

After a STOP, the restore (step 5) waits for the run coordinator's word. The report lists the configuration's state, the extension's state, and whether the app is running.

## Limits
- One sitting, one app version, one PC, one model as the app chose it (record its name if the app shows it). One run per tab: a model's behaviour can vary between runs, so a pass is a first sighting, not a rate.
- The tool text here is the server's, not Bridge's. The description and each step's result both tell the model to go on, and the nudge, where used, tells it too. So a pass shows that the model follows such text across calls, not that it carries an id unprompted. A build must ship text no weaker than this, or this row does not cover it.
- The server sleeps; no Tally work runs. Whether the app's 60 s counts from the call or from the tool prompt is not told apart, though a 40 s step with a prompt answered at once leaves about 20 s.

## Keep
On `lab/703-a-answers`, with `SHA256.txt`:
- every `steps.py` output and every compare line;
- `a.jsonl` (no names or values beyond the server's own);
- the chat and session words, with account, organisation, device and conversation ids replaced by `<redacted>` (the raw file's sha256 recorded beside it);
- the versions;
- a `RESULTS.md` holding the table above.

Screens and the configuration copies stay on the lab PC, listed by name, size and sha256. Notes carry no user names and no machine paths beyond `C:\bridge-lab\703-a\`. Commits carry no co-author or generated-with line. Send the sha first; push on the run coordinator's yes.
