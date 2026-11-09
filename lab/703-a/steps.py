#!/usr/bin/env python3
"""Reads the lab server's event log (lab/703-a/stepped-vouchers-server.mjs, `--log`) into one line per call.

  python -I steps.py <log> [--since <UTC time>] [--until <UTC time>]

`--since` and `--until` (ISO times as the log writes them, for example 2026-11-10T09:30:00.000Z) count only the calls
received, and the reads started, at or after `--since` and before `--until`: the sheet reads each chat from its own
start time to the next chat's, so one chat's run is never counted with another's. Everything is still read, so an
unreadable log fails whatever the window.

Prints, in this order:
  - one `initialize` line per initialize in the whole log: time, pid, the client's name and protocol, the top-level
    capability keys it declared, and `tasks yes|no` (whether `tasks` is among them).
  - one `call` line per tools/call in the window: its number, the UTC time it was received, `gap` (seconds from the
    previous call's end to this one's arrival: the model's turn plus any tool prompt), the pid that received it, its
    JSON-RPC id, the tool, the keys of its `_meta` (`-` for none; the values are in the log's received line), its
    arguments exactly as received, the read_id
    it carried (`-` for none), and how it ended: `step <k> of <n> done <true|false> read_id <id> <elapsed>` (a step
    answered), `refused <code>` (an in-band refusal: read_unknown, read_finished, read_expired,
    read_arguments_changed, company_not_open, busy, or an argument code), `companies` (list_companies answered),
    `error <code> <message>` (a JSON-RPC error), `cancelled <elapsed> <reason>`, `stdin_closed`/`signal`/
    `stdout_error` with `running` above 0, or `open` (no end yet).
  - one `read` line per read the server minted in the window: its id, the pid, its plan (five or twelve), the window's
    span in days, its code (the word in the last step's party name), the steps answered of the plan's, done.
  - `evicted`: the reads the server dropped at its live cap, in the window.
  - `started <n> pids <pid,...>`: every server process that wrote to the log.
  - `facts`: the counts the sheet reads its result from, never a verdict. Over the window's vouchers calls: how many;
    how many were answered, cut (cancelled, or the server's input closed while one ran), refused busy, refused in band
    (busy excluded), refused read_arguments_changed, and still open; `new_read_after_answered_step`, calls with no
    read_id after the model had been given an id by an answered step; `no_read_id_after_cut`, calls with no read_id
    after a cut step but before any answered one; read_ids received that no process issued; read_ids received from a
    different pid than issued them; the pid changes between consecutive calls; the largest elapsed of an answered step
    and of a cancelled one; the reads started and finished; and `other_calls_not_answered`, the calls to any other
    tool (list_companies) in the window that were not answered.

A line that does not parse as JSON, an event without a time, a log with no `started` event, or a `received` event the
server truncated, fails the whole read (exit 1): a log that cannot be read whole is never "no calls". Exit 0 = read,
2 = the command line is wrong.
"""
import datetime
import json
import re
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


def ended(call, text, kind, at):
    if call["end"] is None:
        call["end"] = text
        call["kind"] = kind
        call["ended_at"] = at


def read(path, since="", until="9999"):
    out = []
    calls = []
    pids = []
    reads = {}
    evicted = []
    truncated = 0
    for event in events(path):
        kind = event["event"]
        pid = event.get("pid")
        if kind == "started":
            pids.append(pid)
            continue
        if kind == "received" and event.get("truncated"):
            truncated += 1
            continue
        message = event.get("message") if kind == "received" else None
        if isinstance(message, dict) and message.get("method") == "initialize":
            params = message.get("params") or {}
            info = params.get("clientInfo") or {}
            caps = params.get("capabilities") or {}
            keys = sorted(caps) if isinstance(caps, dict) else []
            out.append("initialize {} pid {} client {} protocol {} capabilities {} tasks {}".format(
                event["time"], pid, info.get("name"), params.get("protocolVersion"), ",".join(keys) or "-",
                "yes" if "tasks" in keys else "no"))
            continue
        if isinstance(message, dict) and message.get("method") == "tools/call":
            params = message.get("params") or {}
            args = params.get("arguments")
            carried = args.get("read_id") if isinstance(args, dict) else None
            meta = params.get("_meta")
            calls.append({"time": event["time"], "pid": pid, "id": message.get("id"), "tool": params.get("name"),
                          "meta": ",".join(sorted(meta)) if isinstance(meta, dict) and meta else "-",
                          "arguments": args, "read_id": carried, "end": None, "kind": None, "elapsed": None,
                          "ended_at": None})
            continue
        if kind == "read_started":
            reads[event.get("read_id")] = {"pid": pid, "time": event["time"], "code": event.get("code"),
                                           "plan": event.get("plan"), "of": event.get("of"),
                                           "span": event.get("span_days"), "steps": 0, "done": False}
        if kind == "read_evicted":
            evicted.append((event["time"], event.get("read_id")))
            continue
        # Ends are matched by JSON-RPC id and pid where the event names an id; the server runs one call at a time.
        matching = [c for c in calls if c["end"] is None and c["pid"] == pid and ("id" not in event or c["id"] == event.get("id"))]
        if not matching:
            continue
        call = matching[0]
        if kind == "step_result":
            call["elapsed"] = event.get("elapsed", -1)
            ended(call, "step {} of {} done {} read_id {} {:.3f}".format(
                event.get("step"), event.get("of"), "true" if event.get("done") else "false", event.get("read_id"),
                event.get("elapsed", -1)), "answered", event["time"])
            entry = reads.get(event.get("read_id"))
            if entry is not None:
                entry["steps"] = event.get("step")
                entry["done"] = bool(event.get("done"))
        elif kind == "refused":
            ended(call, "refused {}".format(event.get("code")), "busy" if event.get("code") == "busy" else "refused",
                  event["time"])
        elif kind == "error_sent":
            ended(call, "error {} {}".format(event.get("code"), event.get("message")), "error", event["time"])
        elif kind == "companies_sent":
            ended(call, "companies", "companies", event["time"])
        elif kind == "cancelled":
            call["elapsed"] = event.get("elapsed", -1)
            ended(call, "cancelled {:.3f} reason {}".format(event.get("elapsed", -1), event.get("reason")), "cut",
                  event["time"])
        elif kind in ("stdin_closed", "signal", "stdout_error") and event.get("running", 0) > 0:
            ended(call, "{} running {} at {}".format(kind, event.get("running"), event["time"]), "cut", event["time"])
    if not pids:
        raise Failed("no started event: the server never wrote to this log")
    if truncated:
        raise Failed("{} received event(s) were truncated, so a call may be missing".format(truncated))
    # Only the run after --since is counted; the calls before it stay in the log but not in these lines.
    window = [c for c in calls if since <= c["time"] < until]
    # A call with no end event stays `open`, which the sheet treats as a call to look at, never as a success.
    # gap: seconds from the previous call's end to this call's arrival (the model's turn plus any tool prompt).
    previous = None
    for number, call in enumerate(window, 1):
        gap = "-" if previous is None or previous["ended_at"] is None else "{:.1f}".format(seconds_between(previous["ended_at"], call["time"]))
        out.append("call {} received {} gap {} pid {} id {} tool {} meta {} arguments {} read_id {} end {}".format(
            number, call["time"], gap, call["pid"], json.dumps(call["id"]), call["tool"], call["meta"],
            json.dumps(call["arguments"], sort_keys=True), call["read_id"] or "-", call["end"] or "open"))
        previous = call
    window_reads = {k: v for k, v in reads.items() if since <= v["time"] < until}
    for read_id, entry in window_reads.items():
        out.append("read {} pid {} plan {} span_days {} code {} steps {} of {} done {}".format(
            read_id, entry["pid"], entry["plan"], entry["span"], entry["code"], entry["steps"], entry["of"],
            "true" if entry["done"] else "false"))
    out.append("evicted {}".format(",".join(e for t, e in evicted if e and since <= t < until) or "-"))
    stepped = [c for c in window if c["tool"] == "vouchers"]
    # A call with no read_id counts as a new read only once the model has been given an id by an answered step.
    new_read_after_answered = 0
    no_id_after_cut = 0
    answered_before = False
    cut_before = False
    for c in stepped:
        if not c["read_id"] and answered_before:
            new_read_after_answered += 1
        elif not c["read_id"] and cut_before:
            no_id_after_cut += 1
        answered_before = answered_before or c["kind"] == "answered"
        cut_before = cut_before or c["kind"] == "cut"
    not_issued = sorted({c["read_id"] for c in stepped if c["read_id"] and c["read_id"] not in reads})
    other_pid = sorted({c["read_id"] for c in stepped if c["read_id"] in reads and reads[c["read_id"]]["pid"] != c["pid"]})
    pid_changes = sum(1 for a, b in zip(stepped, stepped[1:]) if a["pid"] != b["pid"])
    answered = [c["elapsed"] for c in stepped if c["kind"] == "answered"]
    cut = [c["elapsed"] for c in stepped if c["kind"] == "cut" and c["elapsed"] is not None]
    count = lambda kind: sum(1 for c in stepped if c["kind"] == kind)
    fmt = lambda values: "{:.3f}".format(max(values)) if values else "-"
    out.append("started {} pids {}".format(len(pids), ",".join(str(p) for p in pids) or "-"))
    changed = sum(1 for c in stepped if c["end"] == "refused read_arguments_changed")
    # Any call to another tool that did not end as answered: a refused, cut or open list_companies counts here.
    others = sum(1 for c in window if c["tool"] != "vouchers" and c["kind"] != "companies")
    out.append("facts since {} until {} vouchers_calls {} answered {} cut {} busy {} refused {} arguments_changed {} open {} "
               "new_read_after_answered_step {} "
               "no_read_id_after_cut {} ids_not_issued {} ids_from_other_pid {} pid_changes {} max_answered_elapsed {} "
               "max_cut_elapsed {} reads_started {} reads_done {} other_calls_not_answered {}".format(
                   since or "-", until if until != "9999" else "-", len(stepped), count("answered"), count("cut"),
                   count("busy"), count("refused"), changed,
                   sum(1 for c in stepped if c["end"] is None), new_read_after_answered, no_id_after_cut,
                   ",".join(not_issued) or "-", ",".join(other_pid) or "-", pid_changes, fmt(answered), fmt(cut),
                   len(window_reads), sum(1 for r in window_reads.values() if r["done"]), others))
    return "\n".join(out)


def seconds_between(earlier, later):
    parse = lambda text: datetime.datetime.strptime(text, "%Y-%m-%dT%H:%M:%S.%fZ")
    return (parse(later) - parse(earlier)).total_seconds()


TIME = r"\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d\.\d{3}Z"

if __name__ == "__main__":
    argv = sys.argv[1:]
    bounds = {"--since": "", "--until": "9999"}
    while len(argv) >= 3 and argv[-2] in bounds and re.fullmatch(TIME, argv[-1]):
        bounds[argv[-2]] = argv[-1]
        argv = argv[:-2]
    if len(argv) != 1:
        print(__doc__)
        sys.exit(2)
    try:
        print(read(argv[0], bounds["--since"], bounds["--until"]))
    except (Failed, OSError, UnicodeDecodeError) as error:
        print("FAIL: {}".format(error))
        sys.exit(1)
    sys.exit(0)
