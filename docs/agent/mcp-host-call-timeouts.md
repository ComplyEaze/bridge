# How long MCP hosts wait for a tool call (#703 step 0)

A long Bridge read can outlast the host's patience: the host abandons the call, Bridge finishes
the read anyway, and the agent asks again. Before #703 chooses between per-part progress and a
background job, each supported host is measured with `scripts/mcp-sleep-server.mjs`, a dev-only
stdio MCP server whose one tool, `sleep`, waits for N seconds. It is not bundled, not part of the
`bridge` crate and not among Bridge's registered tools, so it cannot ship.

Per host and per timeout setting, four things are recorded:
- the largest N that returns;
- what the host shows when it gives up;
- whether `notifications/cancelled` arrives;
- whether `notifications/progress` resets the host's timer.

The server's event log (`--log <file>`) is the evidence. It records, with times, every message
the host sent and every progress notification and result the server sent. What the host shows on
screen is recorded beside it, never instead of it.

## Results

Confidence markers follow `docs/tally/TALLY_PROTOCOL_REFERENCE.md`.

| Host | Version, OS | N returned | N abandoned | Shown on abandon | `cancelled` | Progress resets the timer | Confidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Claude Code CLI (`claude -p`), default settings | 2.1.285, macOS | 90, 300, 900 (this server); 150 (earlier server, 30 Sep) | none up to 900 | not reached | n/a: no call was abandoned | not tested: no timeout reached | partial, one run each |
| Claude Code CLI, per-server `timeout` of 60000 ms in `.mcp.json` | 2.1.285, macOS | none (150 s was cut) | 150 s call cut at 60.0 s, with and without progress every 10 s | `MCP server ... timed out after 60s` | received (reason `SdkError: Request timed out`) | no: progress every 10 s did not reset it | partial, one run each (30 Sep, earlier server, see #703) |
| Claude Code in the Code tab of the Claude desktop app, default settings | 2.1.284, macOS | 150 (earlier server, 30 Sep) | none up to 150 | not reached | n/a: no call was abandoned | not tested | partial, one run, run by hand |
| The Claude desktop app's configured servers, served to a Code tab session, default settings | desktop app 4.29.0.0 with Claude Code 2.1.309, Windows | none | 130 s and 90 s calls each cut at 60.0 s; the host then replaced the server process | `Request timed out` | received (reason `SdkError: Request timed out`) | not testable: the calls carried no `_meta`, so no `progressToken` | partial, one run each (`lab/2119-bills-60s-row-answers`, 29 Oct 2026) |
| Claude Desktop (chat app) | macOS | not measured | | | | | |
| Claude Desktop (chat app) | Windows | not measured | | | | | |
| a third-party client | | not measured | | | | | |

**Claude Code, details.** Each call carried `_meta.progressToken` (equal to the request id) and
`_meta["claudecode/toolUseId"]`, whether or not progress was asked for. Progress every 10 s caused
no error and changed nothing visible in the final answer. Every call returned, plain and with
progress every 10 s, at 90 s, 300 s and 900 s (the server's ceiling): the log shows each
`result_sent` within 0.1 s of N, and the host's answer quoted the `slept N s` text. The host asked
for protocol version `2025-11-25`, and no `MCP_TOOL_TIMEOUT` was set in its environment. So with default settings
(no per-server `timeout`, no `MCP_TOOL_TIMEOUT`) this host has no call timeout of 900 s or less.
That holds for default settings only: the host's timeout is configurable, and the second row of the
table shows a configured timeout cutting a call exactly when set, sending `notifications/cancelled`,
and not being reset by progress. Whether progress resets a timer is therefore known for a configured
wall-clock timeout (it does not) and not for any timeout a host applies by default, because none was
reached. `MCP_TOOL_TIMEOUT` was not measured.

The 30 Sep runs (rows 2 and 3 and the 150 s default run) used an earlier dev server, outside the
repository, with the same one-tool shape; their logs are not kept here, and the figures are quoted
from the #703 thread (comments of 30 Sep, 09:13 and 09:16 UTC).

This does not explain the abandonment at about 60 s that #703 records on another setup: the
likeliest explanation is a configured timeout or an older build (unverified; nothing records which
it was). The open rows of the table are the next hosts to measure.

## Measuring a host

Use a scratch folder for the logs. Never point the server at a folder holding client data; it
writes only its own log.

1. Register the server with the host. Most hosts take a JSON entry like this one:

   ```json
   {"mcpServers": {"sleep": {"command": "node", "args": ["<checkout>/scripts/mcp-sleep-server.mjs", "--log", "<scratch>/<host>-<N>.jsonl"]}}}
   ```

   Both paths must be absolute: a host starts the server in a working directory of its own, and
   the server exits at once if it cannot write its log. On Windows, write each backslash in a path
   as `\\`, as JSON requires. Claude Code takes the entry as a file: `claude -p "<prompt>"
   --mcp-config <file> --strict-mcp-config --allowedTools mcp__sleep__sleep --output-format json
   < /dev/null`. Claude Desktop takes it in its configuration file, and needs a restart to load it.
2. Ask the host to call `sleep` once with `seconds` N and no other argument. Record the wall time
   and exactly what the host showed. In the log, check the `received` event for `tools/call`: its
   `arguments` are what was actually sent (an agent may change N), and its `_meta` shows whether
   the host sent a `progressToken`.
3. Repeat with `progress_every` 10. Without a `progressToken` in `_meta` the server sends no
   progress, and progress cannot be tested on that host.
4. **Reading an abandoned call.** A host that gives up without sending `notifications/cancelled`
   leaves the server sleeping, so the log still ends with `result_sent` at N. The log alone cannot
   show that the host abandoned the call. Read it from what the host showed (an error, or an
   answer without the `slept N s` text) and compare the time it showed it with the log's times. A
   `cancelled` event, or a `stdin_closed`, `signal` or `stdout_error` event with `running` above
   0, shows the host ended the call itself.
5. Start at N = 10, then 30, 60, 90, and double from there. When a plain call is abandoned, halve
   the step between the last N returned and the first N abandoned until they are within 10 s.
   Then run the first abandoned N with progress. If it now returns, progress resets the timer; to
   be sure, also run twice that N with progress. N is at most 900: above it the server refuses the
   call (`seconds` out of range), which is not a host timeout.
6. Repeat steps 2 to 5 with a timeout set, because a host's timeout is configurable and a default
   run can hide it: for Claude Code, a per-server `"timeout": 60000` in the `.mcp.json` entry, then
   separately the `MCP_TOOL_TIMEOUT` environment variable (in milliseconds). Run a call longer than
   the timeout, plain and with progress, and record whether `cancelled` arrives and whether progress
   changed when the call was cut.
7. One variable at a time: change only N or only `progress_every` between runs, and repeat any
   surprising result before recording it.
8. Record each run in the table above with the log's own times. Quote the key log lines in the
   pull request that adds the row; the logs themselves stay out of the repository.
