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

Sheet version 3 adds the following. What it reads from a log of version 2's server is unchanged; the output gains
the lines and `facts` fields below.
  - the repository's sleep server (`mcp-sleep-server.mjs`, rows H1 to H4) logs `result_sent` for an answered call;
    its call line ends `slept <elapsed> of <seconds asked>`, counted as answered;
  - a log of the server run with `--continue-by arguments` (its `company` line says so) carries no read_id, so
    there `new_read_after_answered_step` counts the reads started after the window's first answered vouchers step,
    not the calls without an id;
  - `dropped` and `expired` in `facts`: the live reads that server dropped for a call with other arguments, and for
    15 minutes idle, in the window;
  - `started` also prints each process's start time. The sleep server writes a pid only on that line, so its other
    lines take the latest start's pid (by order, exact only while one process writes the log); row H4 reads whether a
    process started between two calls from these times, never from a call's pid;
  - a `sleep` line before `facts`: the window's sleep calls, how many were answered, cut, refused busy and still open,
    and the largest elapsed of an answered and of a cut one. `other_calls_not_answered` no longer counts sleep calls;
  - a sleep call that arrives while another of the same process is open ends `refused busy (inferred)`: that server
    refuses it in band and logs nothing;
  - a sleep-server end is put to an open call with its JSON-RPC id (ids restart in each process); a result goes to
    the call that asked for about that many seconds, and an end that could belong to more than one call is marked
    `(pid uncertain)`. A sleep call's answered end reads `slept <elapsed> of <seconds asked>`;
  - one `closed <kind> pid <pid> running <n> at <time>` line per stdin_closed, signal or stdout_error in the whole
    log, a `cancel_ignored` line (cancels for a call already ended, in the window) and a `live_reads` line (reads
    over the whole log not finished, dropped, expired or evicted);
  - in `facts`: `leftover_dropped`, reads started before the window and dropped or expired in it (not counted in
    `dropped` or `expired`); `continued_leftover`, the window's vouchers calls that ran a step of a read started
    before it; and `distinct_arguments`, the number of different argument objects among the window's vouchers
    calls, compared as sent with `read_id` left out (an `offset: 0` sent on one call and left out on another counts
    as two).

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
    dropped = []
    truncated = 0
    starts = []
    closed = []
    ignored = []
    arguments_mode = False
    for event in events(path):
        kind = event["event"]
        # The sleep server writes a pid only on its `started` line; its other lines take the pid of the latest
        # `started` before them, an attribution by order that is exact only while one process writes the log.
        pid = event.get("pid", pids[-1] if pids else None)
        if kind == "started":
            pids.append(pid)
            starts.append(event["time"])
            continue
        if kind == "company" and event.get("continue_by") == "arguments":
            arguments_mode = True
        if kind in ("stdin_closed", "signal", "stdout_error"):
            closed.append("closed {} pid {} running {} at {}".format(kind, pid, event.get("running"), event["time"]))
        if kind == "cancel_ignored":
            ignored.append((event["time"], event.get("id")))
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
            call = {"time": event["time"], "pid": pid, "id": message.get("id"), "tool": params.get("name"),
                    "meta": ",".join(sorted(meta)) if isinstance(meta, dict) and meta else "-",
                    "arguments": args, "read_id": carried, "end": None, "kind": None, "elapsed": None,
                    "ended_at": None, "served": None,
                    "seconds": args.get("seconds") if isinstance(args, dict) else None}
            # The sleep server refuses a second call while one runs and logs nothing for it, so a sleep call that
            # arrives while another of the same process is open is ended here as an inferred busy refusal.
            if call["tool"] == "sleep" and any(c["end"] is None and c["tool"] == "sleep" and c["pid"] == pid for c in calls):
                ended(call, "refused busy (inferred)", "busy", event["time"])
            calls.append(call)
            continue
        if kind == "read_started":
            reads[event.get("read_id")] = {"pid": pid, "time": event["time"], "code": event.get("code"),
                                           "continue_by": event.get("continue_by", "id"),
                                           "plan": event.get("plan"), "of": event.get("of"),
                                           "span": event.get("span_days"), "steps": 0, "done": False}
        if kind == "read_evicted":
            evicted.append((event["time"], event.get("read_id")))
            continue
        if kind in ("read_dropped", "read_expired"):
            dropped.append((event["time"], kind, event.get("read_id")))
            continue
        # Ends are matched by JSON-RPC id and pid where the event names an id; the server runs one call at a time.
        if "pid" in event:
            matching = [c for c in calls if c["end"] is None and c["pid"] == pid and ("id" not in event or c["id"] == event.get("id"))]
            uncertain = ""
        else:
            # A sleep-server line: no pid of its own, and JSON-RPC ids restart in each process. Any open call with
            # this id may be the one; a result is put to the call that asked for about that many seconds.
            matching = [c for c in calls if c["end"] is None and ("id" not in event or c["id"] == event.get("id"))]
            if kind == "result_sent":
                near = [c for c in matching if isinstance(c["seconds"], (int, float))
                        and abs(c["seconds"] - event.get("elapsed", -1)) < 1]
                matching = near or matching
            ours = [c for c in matching if c["pid"] == pid]
            uncertain = " (pid uncertain)" if len(matching) > 1 else ""
            matching = ours + [c for c in matching if c["pid"] != pid] if len(matching) > 1 else matching
        if not matching:
            continue
        call = matching[0]
        if kind == "step_result":
            call["elapsed"] = event.get("elapsed", -1)
            call["served"] = event.get("read_id")
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
        elif kind == "result_sent":
            # The repository's sleep server (rows H1 to H4): one answered call.
            call["elapsed"] = event.get("elapsed", -1)
            ended(call, "slept {:.3f} of {}{}".format(event.get("elapsed", -1), call["seconds"], uncertain), "answered",
                  event["time"])
        elif kind == "cancelled":
            call["elapsed"] = event.get("elapsed", -1)
            ended(call, "cancelled {:.3f} reason {}{}".format(event.get("elapsed", -1), event.get("reason"), uncertain),
                  "cut", event["time"])
        elif kind in ("stdin_closed", "signal", "stdout_error") and event.get("running", 0) > 0:
            ended(call, "{} running {} at {}{}".format(kind, event.get("running"), event["time"], uncertain), "cut",
                  event["time"])
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
            number, call["time"], gap, "-" if call["pid"] is None else call["pid"], json.dumps(call["id"]), call["tool"], call["meta"],
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
    if arguments_mode:
        # No id is served in this mode, so a new read shows only as a read started after a step was answered.
        first = next((c["ended_at"] for c in stepped if c["kind"] == "answered"), None)
        new_read_after_answered = sum(1 for r in window_reads.values() if first is not None and r["time"] > first)
        no_id_after_cut = "-"
    # A read started before the window and dropped, expired or continued inside it is a leftover of an earlier chat,
    # counted apart so it is never put to this one.
    window_dropped = [k for at, k, rid in dropped if since <= at < until and rid in window_reads]
    leftover_dropped = sum(1 for at, k, rid in dropped if since <= at < until and rid not in window_reads)
    continued_leftover = sum(1 for c in stepped if c["served"] is not None and c["served"] not in window_reads)
    gone = {rid for at, k, rid in dropped} | {e for t, e in evicted}
    live = sorted(rid for rid, r in reads.items() if not r["done"] and rid not in gone)
    strip = lambda a: json.dumps({k: v for k, v in a.items() if k != "read_id"} if isinstance(a, dict) else a, sort_keys=True)
    distinct = len({strip(c["arguments"]) for c in stepped})
    not_issued = sorted({c["read_id"] for c in stepped if c["read_id"] and c["read_id"] not in reads})
    other_pid = sorted({c["read_id"] for c in stepped if c["read_id"] in reads and reads[c["read_id"]]["pid"] != c["pid"]})
    pid_changes = sum(1 for a, b in zip(stepped, stepped[1:]) if a["pid"] != b["pid"])
    answered = [c["elapsed"] for c in stepped if c["kind"] == "answered"]
    cut = [c["elapsed"] for c in stepped if c["kind"] == "cut" and c["elapsed"] is not None]
    count = lambda kind: sum(1 for c in stepped if c["kind"] == kind)
    fmt = lambda values: "{:.3f}".format(max(values)) if values else "-"
    out.append("started {} pids {} at {}".format(len(pids), ",".join(str(p) for p in pids) or "-", ",".join(starts) or "-"))
    out.extend(closed)
    out.append("cancel_ignored {}".format(",".join("{} id {}".format(t, json.dumps(i)) for t, i in ignored if since <= t < until) or "-"))
    out.append("live_reads {}".format(",".join(live) or "-"))
    changed = sum(1 for c in stepped if c["end"] == "refused read_arguments_changed")
    # Any call to another tool that did not end as answered: a refused, cut or open list_companies counts here.
    others = sum(1 for c in window if c["tool"] not in ("vouchers", "sleep") and c["kind"] != "companies")
    slept = [c for c in window if c["tool"] == "sleep"]
    out.append("sleep calls {} answered {} cut {} busy {} open {} max_answered_elapsed {} max_cut_elapsed {}".format(
        len(slept), sum(1 for c in slept if c["kind"] == "answered"), sum(1 for c in slept if c["kind"] == "cut"),
        sum(1 for c in slept if c["kind"] == "busy"),
        sum(1 for c in slept if c["end"] is None), fmt([c["elapsed"] for c in slept if c["kind"] == "answered"]),
        fmt([c["elapsed"] for c in slept if c["kind"] == "cut" and c["elapsed"] is not None])))
    out.append("facts since {} until {} vouchers_calls {} answered {} cut {} busy {} refused {} arguments_changed {} open {} "
               "new_read_after_answered_step {} "
               "no_read_id_after_cut {} ids_not_issued {} ids_from_other_pid {} pid_changes {} max_answered_elapsed {} "
               "max_cut_elapsed {} reads_started {} reads_done {} dropped {} expired {} leftover_dropped {} "
               "continued_leftover {} distinct_arguments {} other_calls_not_answered {}".format(
                   since or "-", until if until != "9999" else "-", len(stepped), count("answered"), count("cut"),
                   count("busy"), count("refused"), changed,
                   sum(1 for c in stepped if c["end"] is None), new_read_after_answered, no_id_after_cut,
                   ",".join(not_issued) or "-", ",".join(other_pid) or "-", pid_changes, fmt(answered), fmt(cut),
                   len(window_reads), sum(1 for r in window_reads.values() if r["done"]),
                   window_dropped.count("read_dropped"), window_dropped.count("read_expired"), leftover_dropped,
                   continued_leftover, distinct, others))
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
