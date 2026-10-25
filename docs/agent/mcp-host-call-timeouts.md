# How long MCP hosts wait for a tool call (#703 step 0)

A long Bridge read can outlast the host's patience: the host abandons the call, Bridge finishes
the read anyway, and the agent asks again. Before #703 chooses between per-part progress and a
background job, each supported host is measured with `scripts/mcp-sleep-server.mjs`, a dev-only
stdio MCP server whose one tool, `sleep`, waits for N seconds. It is not bundled, not part of the
`bridge` crate and not among Bridge's registered tools, so it cannot ship.

Per host, four things are recorded:
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
| Claude Code CLI (`claude -p`) | 2.1.285, macOS | 90, 300, 900 | none up to 900 | not reached | not received | not tested: no timeout reached | verified, one run each |
| Claude Code in the Claude desktop app | macOS | not measured | | | | | |
| Claude Desktop | macOS | not measured | | | | | |
| Claude Desktop | Windows | not measured | | | | | |
| a third-party client | | not measured | | | | | |

**Claude Code, details.** Each call carried `_meta.progressToken` (equal to the request id) and
`_meta["claudecode/toolUseId"]`, whether or not progress was asked for. Progress every 10 s caused
no error and changed nothing visible in the final answer. Every call returned, plain and with
progress every 10 s, at 90 s, 300 s and 900 s (the server's ceiling): the log shows each
`result_sent` within 0.1 s of N, and the host's answer quoted the `slept N s` text. The host asked
for protocol version `2025-11-25`, and no `MCP_TOOL_TIMEOUT` was set in its environment. So this
host has no call timeout of 900 s or less, and whether progress resets a timer cannot be tested on
it.

This does not explain the abandonment at about 60 s that #703 records on another setup. That one
remains to be found: the open rows of the table are the next hosts to measure.

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
6. One variable at a time: change only N or only `progress_every` between runs, and repeat any
   surprising result before recording it.
7. Record each run in the table above with the log's own times. Quote the key log lines in the
   pull request that adds the row; the logs themselves stay out of the repository.
