#!/usr/bin/env python3
"""Reads the sleep server's event log (scripts/mcp-sleep-server.mjs, `--log`) into one line per tools/call.

  python -I calls.py <log>

Each line: the call's number in the log, the UTC time its `received` event was written, its JSON-RPC id, its
`arguments` exactly as received, whether its `_meta` carried a `progressToken`, the number of `progress_sent` events,
and how it ended: `result_sent <elapsed>`, `cancelled <elapsed> <reason>`, `stdin_closed running <n>`,
`signal running <n>`, `stdout_error running <n>`, or `open` (no end event yet; the server does not log a call it
refused in band, so a refused call also reads `open` until the next call's end). The row runs one call at a time, so a call's end is the first end event after
it. It also prints every `initialize`'s clientInfo name and protocol version, and the count of `started` events.

A line that does not parse as JSON, or an event without a time, fails the whole read (exit 1): a log that cannot be
read is never "no calls". Exit 0 = read, 2 = the command line is wrong.
"""
import json
import sys


class Failed(Exception):
    pass


def events(path):
    with open(path, "r", encoding="utf-8") as handle:
        for number, line in enumerate(handle, 1):
            if not line.strip():
                continue
            try:
                event = json.loads(line)
            except ValueError:
                raise Failed("line {} does not parse".format(number))
            if not isinstance(event, dict) or "time" not in event or "event" not in event:
                raise Failed("line {} has no time or event".format(number))
            yield event


def read(path):
    out = []
    calls = []
    starts = 0
    for event in events(path):
        kind = event["event"]
        if kind == "started":
            starts += 1
            continue
        message = event.get("message") if kind == "received" else None
        if isinstance(message, dict) and message.get("method") == "initialize":
            info = (message.get("params") or {}).get("clientInfo") or {}
            out.append("initialize {} client {} protocol {}".format(
                event["time"], info.get("name"), (message.get("params") or {}).get("protocolVersion")))
            continue
        if isinstance(message, dict) and message.get("method") == "tools/call":
            params = message.get("params") or {}
            meta = params.get("_meta") or {}
            calls.append({"time": event["time"], "id": message.get("id"), "arguments": params.get("arguments"),
                          "token": "progressToken" in meta, "meta_keys": sorted(meta), "progress": 0, "end": None})
            continue
        open_calls = [call for call in calls if call["end"] is None]
        if not open_calls:
            continue
        call = open_calls[0]
        if kind == "progress_sent":
            call["progress"] += 1
        elif kind == "result_sent":
            call["end"] = "result_sent {:.3f} at {}".format(event.get("elapsed", -1), event["time"])
        elif kind == "cancelled":
            call["end"] = "cancelled {:.3f} at {} reason {}".format(event.get("elapsed", -1), event["time"], event.get("reason"))
        elif kind in ("stdin_closed", "signal", "stdout_error") and event.get("running", 0) > 0:
            call["end"] = "{} running {} at {}".format(kind, event.get("running"), event["time"])
    for number, call in enumerate(calls, 1):
        out.append("call {} received {} id {} arguments {} progressToken {} meta_keys {} progress_sent {} end {}".format(
            number, call["time"], json.dumps(call["id"]), json.dumps(call["arguments"], sort_keys=True),
            "yes" if call["token"] else "no", ",".join(call["meta_keys"]) or "-", call["progress"], call["end"] or "open"))
    out.append("started {} calls {}".format(starts, len(calls)))
    return "\n".join(out)


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(2)
    try:
        print(read(sys.argv[1]))
    except (Failed, OSError, UnicodeDecodeError) as error:
        print("FAIL: {}".format(error))
        sys.exit(1)
    sys.exit(0)
