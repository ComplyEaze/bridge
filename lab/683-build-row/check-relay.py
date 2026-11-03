"""Checks relay.py on a stand-in Tally, with no Tally. Not for merge.

python -I check-relay.py relay.py

Starts a stand-in that answers every request with a fixed body and records what reached it,
runs the relay in front of it, and checks:
  1. a read reaches the stand-in byte for byte, and its answer reaches the client byte for byte;
  2. the read's request and answer are kept as exchange-1.request / exchange-1.answer, equal
     to what was sent and answered;
  3. two reads on one connection are two exchanges, both kept, on one upstream connection;
  4. an Import Data request (UTF-16LE, and UTF-8) never reaches the stand-in, is logged
     `import_refused`, and its connection is closed with no answer;
  5. the relay refuses a non-empty OUTDIR.
Prints one PASS or FAIL line per check, and exits 1 on any FAIL.
"""
import json
import os
import socket
import subprocess
import sys
import tempfile
import threading
import time

RELAY = sys.argv[1]
ANSWER_BODY = "<ENVELOPE><STAND>in</STAND></ENVELOPE>".encode("utf-16-le")


def request(body):
    return (
        b"POST / HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: text/xml; charset=utf-16\r\n"
        + b"Content-Length: " + str(len(body)).encode() + b"\r\n\r\n" + body
    )


READ = request("<ENVELOPE><HEADER><TALLYREQUEST>Export</TALLYREQUEST></HEADER></ENVELOPE>".encode("utf-16-le"))
READ2 = request("<ENVELOPE><HEADER><TALLYREQUEST>Export</TALLYREQUEST><ID>2</ID></HEADER></ENVELOPE>".encode("utf-16-le"))
IMPORT16 = request("<ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER></ENVELOPE>".encode("utf-16-le"))
IMPORT8 = request(b"<ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER></ENVELOPE>")
ANSWER = b"HTTP/1.1 200 OK\r\nContent-Length: " + str(len(ANSWER_BODY)).encode() + b"\r\n\r\n" + ANSWER_BODY


class StandIn:
    def __init__(self):
        self.received = []
        self.connections = 0
        self.server = socket.socket()
        self.server.bind(("127.0.0.1", 0))
        self.server.listen(8)
        self.port = self.server.getsockname()[1]
        threading.Thread(target=self.accept, daemon=True).start()

    def accept(self):
        while True:
            conn, _ = self.server.accept()
            self.connections += 1
            threading.Thread(target=self.serve, args=(conn,), daemon=True).start()

    def serve(self, conn):
        buf = b""
        while True:
            while b"\r\n\r\n" not in buf:
                data = conn.recv(65536)
                if not data:
                    return
                buf += data
            end = buf.index(b"\r\n\r\n") + 4
            head = buf[:end]
            length = int([l for l in head.split(b"\r\n") if l.lower().startswith(b"content-length:")][0].split(b":")[1])
            while len(buf) < end + length:
                buf += conn.recv(65536)
            self.received.append(buf[: end + length])
            buf = buf[end + length :]
            conn.sendall(ANSWER)


def free_port():
    s = socket.socket()
    s.bind(("127.0.0.1", 0))
    port = s.getsockname()[1]
    s.close()
    return port


def read_answer(sock):
    buf = b""
    while b"\r\n\r\n" not in buf:
        data = sock.recv(65536)
        if not data:
            return buf
        buf += data
    end = buf.index(b"\r\n\r\n") + 4
    length = int([l for l in buf[:end].split(b"\r\n") if l.lower().startswith(b"content-length:")][0].split(b":")[1])
    while len(buf) < end + length:
        buf += sock.recv(65536)
    return buf


def connect(port):
    for _ in range(50):
        try:
            return socket.create_connection(("127.0.0.1", port), timeout=10)
        except OSError:
            time.sleep(0.1)
    raise SystemExit("relay did not listen")


def main():
    results = []

    def check(name, ok):
        results.append(ok)
        print(("PASS " if ok else "FAIL ") + name)

    stand = StandIn()
    out = tempfile.mkdtemp()
    port = free_port()
    relay = subprocess.Popen([sys.executable, "-I", RELAY, str(port), str(stand.port), out])
    try:
        c = connect(port)
        c.sendall(READ)
        got = read_answer(c)
        check("1 read forwarded byte for byte", stand.received[:1] == [READ])
        check("1 answer returned byte for byte", got == ANSWER)
        time.sleep(0.2)
        kept_req = open(os.path.join(out, "exchange-1.request"), "rb").read()
        kept_ans = open(os.path.join(out, "exchange-1.answer"), "rb").read()
        check("2 request kept", kept_req == READ)
        check("2 answer kept", kept_ans == ANSWER)
        c.sendall(READ2)
        got2 = read_answer(c)
        time.sleep(0.2)
        check("3 second read answered", got2 == ANSWER)
        check("3 second read kept", open(os.path.join(out, "exchange-2.request"), "rb").read() == READ2)
        check("3 one upstream connection", stand.connections == 1)
        c.close()
        for name, body in (("utf-16le", IMPORT16), ("utf-8", IMPORT8)):
            before = len(stand.received)
            d = connect(port)
            d.sendall(body)
            d.settimeout(5)
            try:
                tail = d.recv(65536)
            except socket.timeout:
                tail = None
            d.close()
            time.sleep(0.2)
            check("4 import (%s) not forwarded" % name, len(stand.received) == before)
            check("4 import (%s) closed with no answer" % name, tail == b"")
        time.sleep(0.2)
        events = [json.loads(line) for line in open(os.path.join(out, "relay.jsonl"))]
        check("4 import_refused logged twice", sum(e.get("event") == "import_refused" for e in events) == 2)
    finally:
        relay.terminate()
        relay.wait()
    busy = tempfile.mkdtemp()
    open(os.path.join(busy, "x"), "w").close()
    refused = subprocess.run([sys.executable, "-I", RELAY, str(free_port()), str(stand.port), busy], capture_output=True)
    check("5 non-empty OUTDIR refused", refused.returncode != 0)
    sys.exit(0 if all(results) else 1)


if __name__ == "__main__":
    main()
