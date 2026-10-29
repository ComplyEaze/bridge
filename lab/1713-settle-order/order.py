#!/usr/bin/env python3
"""Lab probe for the order rows of ORDER-ROWS.md (#1713).  python -I order.py plan.json OUTDIR

plan.json:
 {"host": "127.0.0.1", "port": 9001,
  "conns": {"K1": {"warm": "marks.xml", "sha256": "<hex>"}},
  "steps": [{"name": "B", "at": 5.0, "conn": "new" | "K1", "file": "b.xml", "sha256": "<hex>",
             "limit": 20 | null, "write": true | false, "max_bytes": 33554432}]}
Before anything is sent: OUTDIR must be empty, every file must match its sha256, each named
connection must be used by exactly one step, and each named connection is opened and warmed in
this order (one harmless read, answered in full with status 200); a warm that is not answered
ends the run with nothing else sent. T0 is the moment the last warm was answered; each step's
"at" is seconds after T0. A step sends its request file in one sendall on a fresh connection or
on its named, warmed one (checked alive just before: a connection the peer closed ends the step
as "stale"), reads the answer to its Content-Length, streams the body to OUTDIR/<name>.answer
with a running sha256 and byte count, and gives up after "limit" seconds counted from just before
the send (all bytes written, then the socket is closed): that end is "abandoned". With no limit
the wait is bounded at 900 s ("timeout_bound"). A file OUTDIR/STOP stops all sends: it is checked
before every send and every 0.5 s while waiting. Every step logs exactly one "end" event with
its class: answered, bad_status, abandoned, timeout_bound, refused, stale, closed_before_headers,
no_content_length, bad_header, short, over_bound, stopped, error:<Name>; "done" is written only
when every step logged an end. A write step leaves OUTDIR/<name>.sent before its send and is
refused if that marker exists. Times are seconds since T0 (monotonic) with the UTC wall clock of
T0 in the first line. Request headers: HTTP/1.1 POST /, Host, Content-Type text/xml;
charset=utf-16, Content-Length, Connection: keep-alive (the set the lab's earlier curl line sent
plus keep-alive); it is not claimed to equal ComplyEaze Bridge's header set byte for byte."""
import datetime, hashlib, json, os, select, socket, sys, threading, time

BOUND_S = 900.0


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def request_bytes(data):
    head = ("POST / HTTP/1.1\r\nHost: tally\r\nContent-Type: text/xml; charset=utf-16\r\n"
            f"Content-Length: {len(data)}\r\nConnection: keep-alive\r\n\r\n").encode("ascii")
    return head + data


def read_headers(sock, deadline):
    buf = b""
    while b"\r\n\r\n" not in buf:
        sock.settimeout(max(0.01, deadline - time.monotonic()))
        chunk = sock.recv(65536)
        if not chunk:
            return None, buf, "closed_before_headers"
        buf += chunk
    head, _, rest = buf.partition(b"\r\n\r\n")
    return (head, rest), None, None


def parse_head(head):
    lines = head.decode("latin-1").split("\r\n")
    status = int(lines[0].split()[1])
    length = None
    for line in lines[1:]:
        if line.lower().startswith("content-length:"):
            length = int(line.split(":", 1)[1].strip())
    return status, length


def main(plan_path, outdir):
    if os.listdir(outdir):
        sys.exit("OUTDIR is not empty: refused")
    plan = json.load(open(plan_path, encoding="utf-8"))
    steps = plan["steps"]
    for step in steps:
        if sha(step["file"]) != step["sha256"]:
            sys.exit(f"{step['name']}: file differs from its sha256: refused")
    for name, conn in plan.get("conns", {}).items():
        if sha(conn["warm"]) != conn["sha256"]:
            sys.exit(f"{name}: warm file differs from its sha256: refused")
        if [s["conn"] for s in steps].count(name) != 1:
            sys.exit(f"{name}: a named connection is used by exactly one step: refused")
    host, port = plan["host"], plan["port"]
    log = open(os.path.join(outdir, "events.jsonl"), "a", encoding="utf-8")
    lock = threading.Lock()
    clock = {"t0": time.monotonic()}

    def note(**event):
        event["t"] = round(time.monotonic() - clock["t0"], 3)
        event["utc"] = datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="milliseconds")
        with lock:
            log.write(json.dumps(event) + "\n")
            log.flush()

    ends = {}
    conns, warmed_at = {}, {}
    for name, conn in plan.get("conns", {}).items():
        try:
            s = socket.create_connection((host, port), timeout=20)
            s.sendall(request_bytes(open(conn["warm"], "rb").read()))
            got, _, why = read_headers(s, time.monotonic() + 60)
            ok = got is not None
            if ok:
                status, length = parse_head(got[0])
                body = got[1]
                while length is not None and len(body) < length:
                    s.settimeout(60)
                    chunk = s.recv(1 << 20)
                    if not chunk:
                        ok = False
                        break
                    body += chunk
                ok = ok and status == 200 and length is not None
        except OSError as error:
            ok, why = False, type(error).__name__
        if not ok:
            note(ev="aborted", conn=name, reason="warm not answered in full with status 200")
            sys.exit("a warm read was not answered: nothing else was sent")
        conns[name] = s
        warmed_at[name] = time.monotonic()
        note(ev="warmed", conn=name)
    clock["t0"] = time.monotonic()
    note(ev="t0")
    stop = os.path.join(outdir, "STOP")

    def finish(step, how, **extra):
        with lock:
            ends[step["name"]] = how
        note(ev="end", name=step["name"], how=how, **extra)

    def run(step):
        name, k = step["name"], step["conn"]
        sock = None
        try:
            while time.monotonic() - clock["t0"] < step["at"]:
                if os.path.exists(stop):
                    return finish(step, "stopped")
                time.sleep(0.05)
            if os.path.exists(stop):
                return finish(step, "stopped")
            marker = os.path.join(outdir, f"{name}.sent")
            if step.get("write"):
                if os.path.exists(marker):
                    return finish(step, "error:AlreadySent")
                open(marker, "w").write("sent\n")
            data = open(step["file"], "rb").read()
            if k == "new":
                sock = socket.create_connection((host, port), timeout=20)
            else:
                sock = conns.pop(k)
                if select.select([sock], [], [], 0)[0]:
                    return finish(step, "stale", idle_s=round(time.monotonic() - warmed_at[k], 3))
                note(ev="reused", name=name, conn=k, idle_s=round(time.monotonic() - warmed_at[k], 3))
            sent = time.monotonic()
            note(ev="send", name=name, conn=k, bytes=len(data))
            sock.sendall(request_bytes(data))
            note(ev="written", name=name)
            limit = step.get("limit")
            deadline = sent + (limit if limit else BOUND_S)
            got, partial, why = read_headers(sock, deadline)
            if got is None:
                return finish(step, why)
            try:
                status, length = parse_head(got[0])
            except (ValueError, IndexError):
                return finish(step, "bad_header")
            if length is None:
                return finish(step, "no_content_length", status=status)
            note(ev="first_byte", name=name)
            digest, count = hashlib.sha256(), 0
            with open(os.path.join(outdir, f"{name}.answer"), "wb") as out:
                body = got[1]
                while True:
                    if body:
                        count += len(body)
                        if count > step.get("max_bytes", 1 << 25):
                            return finish(step, "over_bound", bytes=count)
                        digest.update(body)
                        out.write(body)
                    if count >= length:
                        break
                    sock.settimeout(max(0.01, deadline - time.monotonic()))
                    body = sock.recv(1 << 20)
                    if not body:
                        return finish(step, "short", bytes=count, sha256=digest.hexdigest())
            open(os.path.join(outdir, f"{name}.answer.sha256"), "w").write(digest.hexdigest() + "\n")
            finish(step, "answered" if status == 200 else "bad_status", status=status, bytes=count,
                   sha256=digest.hexdigest())
        except socket.timeout:
            finish(step, "abandoned" if step.get("limit") else "timeout_bound")
        except ConnectionRefusedError:
            finish(step, "refused")
        except Exception as error:
            finish(step, f"error:{type(error).__name__}")
        finally:
            if sock is not None:
                sock.close()

    threads = [threading.Thread(target=run, args=(s,)) for s in steps]
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    for s in conns.values():
        s.close()
    if len(ends) == len(steps):
        note(ev="done")
    else:
        note(ev="not_done", missing=[s["name"] for s in steps if s["name"] not in ends])


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
