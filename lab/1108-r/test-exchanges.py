"""#1108: tests check-exchanges.py on folders relay-keep.py writes. Not for merge.

python -I test-exchanges.py PATH_TO_relay-keep.py PATH_TO_check-exchanges.py

A stand-in for Tally answers each request; relay-keep.py relays five into a new folder: three
reads (UTF-16 with and without a BOM, and UTF-8), a bodiless `GET /status` as Bridge sends it, and
one import. Then check-exchanges.py is run on that folder as written (EXPECT exit 0) and on copies
each changed one way (EXPECT exit 1 and the named line). Prints one line per test.
"""
import hashlib
import json
import os
import shutil
import socket
import subprocess
import sys
import tempfile
import threading
import time


def read_message(sock):
    buf = b""
    while b"\r\n\r\n" not in buf:
        data = sock.recv(65536)
        if not data:
            return None
        buf += data
    head, rest = buf.split(b"\r\n\r\n", 1)
    lengths = [int(l.split(b":")[1]) for l in head.split(b"\r\n") if l.lower().startswith(b"content-length:")]
    length = lengths[0] if lengths else 0
    while len(rest) < length:
        data = sock.recv(65536)
        if not data:
            return None
        rest += data
    return head, rest


def message(first, body):
    return first + b"\r\nContent-Length: %d\r\n\r\n" % len(body) + body


def envelope(kind, ident):
    return ("<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>%s</TALLYREQUEST><TYPE>Collection</TYPE>"
            "<ID>%s</ID></HEADER><BODY/></ENVELOPE>" % (kind, ident))


def stand_in():
    server = socket.socket()
    server.bind(("127.0.0.1", 0))
    server.listen(8)

    def serve(conn):
        while True:
            got = read_message(conn)
            if got is None:
                return
            conn.sendall(message(b"HTTP/1.1 200 OK", b"<ENVELOPE><HEADER><STATUS>1</STATUS></HEADER></ENVELOPE>"))

    def accept():
        while True:
            conn, _ = server.accept()
            threading.Thread(target=serve, args=(conn,), daemon=True).start()

    threading.Thread(target=accept, daemon=True).start()
    return server.getsockname()[1]


def main():
    relay_path, check_path = sys.argv[1], sys.argv[2]
    base = tempfile.mkdtemp()
    outdir = os.path.join(base, "relay")
    probe = socket.socket()
    probe.bind(("127.0.0.1", 0))
    port = probe.getsockname()[1]
    probe.close()
    relay = subprocess.Popen([sys.executable, "-I", relay_path, str(port), str(stand_in()), outdir])
    failures = []
    try:
        for _ in range(100):
            try:
                client = socket.create_connection(("127.0.0.1", port), timeout=10)
                break
            except OSError:
                time.sleep(0.05)
        requests = [
            message(b"POST / HTTP/1.1", b"\xff\xfe" + envelope("Export", "Bridge Marks").encode("utf-16-le")),
            message(b"POST / HTTP/1.1", envelope("Export", "Bridge Span").encode("utf-16-le")),
            message(b"POST / HTTP/1.1", envelope("Export", "Bridge Utf8").encode("utf-8")),
            b"GET /status HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
            message(b"POST / HTTP/1.1", envelope("Import Data", "All Masters").encode("utf-16-le")),
        ]
        for request in requests:
            client.sendall(request)
            read_message(client)
        client.close()
        time.sleep(0.3)
    finally:
        relay.kill()

    def run(folder, *extra):
        done = subprocess.run([sys.executable, "-I", check_path, folder, *extra], capture_output=True, text=True)
        return done.returncode, done.stdout

    def test(name, folder, code, needle, *extra):
        got, out = run(folder, *extra)
        held = got == code and needle in out
        print(("pass " if held else "FAIL ") + name)
        if not held:
            failures.append(name)
            print(out)

    def copy(name, change):
        target = os.path.join(base, name)
        shutil.copytree(outdir, target)
        change(target)
        return target

    test("a folder as written: ok 5, one import", outdir, 0, "exchanges: ok 5, FAIL 0 (x-1 to x-5), imports 1")
    test("each request's HEADER is read, BOM or not, UTF-16 or UTF-8", outdir, 0, "Export Collection Bridge Utf8")
    test("a bodiless GET /status is listed as a status exchange", outdir, 0, "x-4 conn 1 request 4 sent 0 answered")
    test("an import is checked against its import-N files", outdir, 0, "(import-1, byte-equal)")
    test("a range lists only its exchanges", outdir, 0, "exchanges: ok 1, FAIL 0 (x-2 to x-2)", "2", "2")
    test("FIRST above LAST fails, never ok 0", outdir, 1, "FAIL range 3 to 2 holds no exchange", "3", "2")
    test("a range of zero fails, never ok 0", outdir, 1, "FAIL range 0 to 0 holds no exchange", "0", "0")

    def tamper(folder):
        with open(os.path.join(folder, "x-2.answer"), "ab") as out:
            out.write(b"x")
    test("an answer one byte longer fails", copy("tamper", tamper), 1, "FAIL x-2: kept bytes differ")
    test("an answer removed fails", copy("noanswer", lambda f: os.remove(os.path.join(f, "x-1.answer"))), 1,
         "FAIL x-1: kept bytes differ")

    def unparsable(folder):
        body = b"<ENVELOPE><HEADER>"
        path = os.path.join(folder, "x-2.request")
        with open(path, "rb") as source:
            head = source.read().split(b"\r\n\r\n", 1)[0]
        with open(path, "wb") as out:
            out.write(head + b"\r\n\r\n" + body)
        log = os.path.join(folder, "relay.jsonl")
        with open(log, encoding="utf-8") as source:
            lines = [json.loads(line) for line in source if line.strip()]
        for line in lines:
            if line.get("x") == 2 and "event" not in line:
                line["sent"], line["sent_sha256"] = len(body), hashlib.sha256(body).hexdigest()
        with open(log, "w", encoding="utf-8") as out:
            out.write("".join(json.dumps(line) + "\n" for line in lines))
    test("a HEADER that does not parse fails, the log agreeing", copy("unparsable", unparsable), 1,
         "FAIL x-2: HEADER not parsed (ParseError)")

    def import_differs(folder):
        with open(os.path.join(folder, "import-1.answer"), "ab") as out:
            out.write(b"x")
    test("an import-N answer that is not byte-equal to its x-N fails", copy("importdiff", import_differs), 1,
         "FAIL x-5: import-1 is not byte-equal to it")

    def import_unlogged(folder):
        shutil.copy(os.path.join(folder, "import-1.request"), os.path.join(folder, "import-2.request"))
    test("an import-N file no log line names fails", copy("importextra", import_unlogged), 1,
         "FAIL import-2: kept, but no log line names it")

    def unlogged(folder):
        shutil.copy(os.path.join(folder, "x-3.request"), os.path.join(folder, "x-6.request"))
    test("a request with no logged answer fails as NO ANSWER", copy("unlogged", unlogged), 1, "FAIL x-6 NO ANSWER")

    def gap(folder):
        os.rename(os.path.join(folder, "x-3.request"), os.path.join(folder, "x-7.request"))
        os.rename(os.path.join(folder, "x-3.answer"), os.path.join(folder, "x-7.answer"))
    test("a gap in the numbers fails", copy("gap", gap), 1, "exchange numbers have a gap")
    test("a range past the last exchange fails", outdir, 1, "FAIL x-6: request not kept whole", "1", "6")
    empty = os.path.join(base, "empty")
    os.mkdir(empty)
    test("a folder with no log is NOT READ", empty, 1, "NOT READ")

    def nofiles(folder):
        for name in os.listdir(folder):
            if name.startswith("x-"):
                os.remove(os.path.join(folder, name))
    test("a log with no x-N file is NOT READ, never 0 exchanges", copy("nofiles", nofiles), 1, "NOT READ")
    print("%d failed" % len(failures))
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
