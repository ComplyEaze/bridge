"""#852: checks relay.py against a stand-in for Tally. Not for merge.

python -I check-relay.py PATH_TO_relay.py

The stand-in and the client each read a message to its head's end, parse the declared
length and read exactly that many bytes. Prints one line per check and exits non-zero
on any failure.
"""
import json
import os
import socket
import subprocess
import sys
import tempfile
import threading
import time

MARK = "<TALLYREQUEST>Import Data</TALLYREQUEST>".encode("utf-16-le")
ANSWER = "<RESPONSE><CREATED>50</CREATED></RESPONSE>".encode("utf-16-le")
READ = b"<ENVELOPE>a read</ENVELOPE>"


def read_message(sock):
    buf = b""
    while b"\r\n\r\n" not in buf:
        data = sock.recv(65536)
        if not data:
            return None
        buf += data
    head, rest = buf.split(b"\r\n\r\n", 1)
    length = next(
        int(line.split(b":")[1]) for line in head.split(b"\r\n") if line.lower().startswith(b"content-length:")
    )
    while len(rest) < length:
        data = sock.recv(65536)
        if not data:
            return None
        rest += data
    assert len(rest) == length, "a second message arrived early"
    return head + b"\r\n\r\n", rest


def request(body):
    return b"POST / HTTP/1.1\r\nContent-Length: %d\r\n\r\n" % len(body) + body


def answer(body):
    return b"HTTP/1.1 200 OK\r\nContent-Length: %d\r\n\r\n" % len(body) + body


class StandIn:
    """Answers a read at once and an import after `delay`; `close_next` hangs up instead."""

    def __init__(self):
        self.delay = 0.8
        self.close_next = False
        self.server = socket.socket()
        self.server.bind(("127.0.0.1", 0))
        self.server.listen(8)
        self.port = self.server.getsockname()[1]
        threading.Thread(target=self.accept, daemon=True).start()

    def accept(self):
        while True:
            conn, _ = self.server.accept()
            threading.Thread(target=self.serve, args=(conn,), daemon=True).start()

    def serve(self, conn):
        while True:
            message = read_message(conn)
            if message is None:
                return
            if MARK in message[1]:
                if self.close_next:
                    time.sleep(0.2)
                    conn.close()
                    return
                time.sleep(self.delay)
                conn.sendall(answer(ANSWER))
            else:
                conn.sendall(answer(b"read answer " + message[1]))


def free_port():
    probe = socket.socket()
    probe.bind(("127.0.0.1", 0))
    port = probe.getsockname()[1]
    probe.close()
    return port


def lines(path):
    with open(path) as source:
        return [json.loads(line) for line in source]


def main():
    relay_path = sys.argv[1]
    stand_in = StandIn()
    outdir = os.path.join(tempfile.mkdtemp(), "out")
    port = free_port()
    relay = subprocess.Popen([sys.executable, "-I", relay_path, str(port), str(stand_in.port), outdir])
    failures = []

    def check(name, held):
        print(("pass " if held else "FAIL ") + name)
        if not held:
            failures.append(name)

    def connect():
        for _ in range(100):
            try:
                return socket.create_connection(("127.0.0.1", port), timeout=10)
            except OSError:
                time.sleep(0.05)
        raise SystemExit("relay did not listen")

    def log():
        time.sleep(0.2)
        return lines(os.path.join(outdir, "relay.jsonl"))

    def rows(records, conn):
        return [r for r in records if r.get("conn") == conn and "event" not in r]

    def events(records, conn, name):
        return [r for r in records if r.get("conn") == conn and r.get("event") == name]

    try:
        one = connect()
        one.sendall(request(READ))
        check("a read comes back byte for byte", read_message(one)[1] == b"read answer " + READ)
        import_body = b"\xff\xfe" + MARK + b"x" * 40
        one.sendall(request(import_body))
        got = read_message(one)
        check("an import answer comes back byte for byte", got[1] == ANSWER)
        records = log()
        conn_rows = rows(records, 1)
        check("two requests on one connection", [r["request"] for r in conn_rows] == [1, 2])
        check("a read is timed and not kept", conn_rows[0]["number"] is None and conn_rows[0]["answer_s"] < 0.5)
        check("the import took the stand-in's delay", 0.75 <= conn_rows[1]["answer_s"] < 2)
        check("first byte is not after last", conn_rows[1]["first_byte_s"] <= conn_rows[1]["answer_s"])
        check("Bridge stayed", conn_rows[1]["bridge_left_s"] is None)
        check("the sent line came first", len(events(records, 1, "sent")) == 1)
        check("no lost reply", not any(r.get("event") == "reply_lost" for r in records))
        with open(os.path.join(outdir, "import-1.request"), "rb") as kept:
            check("the import request is kept", kept.read() == request(import_body))
        with open(os.path.join(outdir, "import-1.answer"), "rb") as kept:
            check("the import answer is kept", kept.read() == answer(ANSWER))
        one.close()

        two = connect()
        stand_in.delay = 1.0
        two.sendall(request(import_body))
        time.sleep(0.3)
        two.close()
        time.sleep(1.1)
        records = log()
        conn_rows = rows(records, 2)
        left = conn_rows[0]["bridge_left_s"] if conn_rows else None
        check("Bridge leaving mid-import is timed", left is not None and 0.25 <= left < 0.6)
        check("Tally's answer still read to its end", conn_rows and conn_rows[0]["answered"] == len(ANSWER))
        check("the lost reply is logged", len(events(records, 2, "reply_lost")) == 1)
        with open(os.path.join(outdir, "import-2.answer"), "rb") as kept:
            check("the lost answer is kept whole", kept.read() == answer(ANSWER))

        three = connect()
        three.sendall(request(READ))
        check("the relay still serves after a lost reply", read_message(three)[1] == b"read answer " + READ)
        stand_in.close_next = True
        three.sendall(request(import_body))
        three.settimeout(5)
        check("Bridge sees the connection end", three.recv(65536) == b"")
        records = log()
        closed = events(records, 3, "tally_closed")
        check("a Tally that hangs up is logged and timed", len(closed) == 1 and closed[0]["waited_s"] >= 0.15)
        check("that import's request is kept and no answer", os.path.exists(os.path.join(outdir, "import-3.request"))
              and not os.path.exists(os.path.join(outdir, "import-3.answer")))
        check("no relay_error in any of it", not any(r.get("event") == "relay_error" for r in records))
    finally:
        relay.kill()
    print("%d failed" % len(failures))
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
