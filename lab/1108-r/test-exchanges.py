"""#1108: tests check-exchanges.py on folders relay-keep.py writes. Not for merge.

python -I test-exchanges.py PATH_TO_relay-keep.py PATH_TO_check-exchanges.py

A stand-in for Tally answers each request; relay-keep.py relays it into a new folder.
Then check-exchanges.py is run on that folder as written (EXPECT exit 0) and on copies
each changed one way (EXPECT exit 1 and the named line). Prints one line per test.
"""
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
    length = next(int(l.split(b":")[1]) for l in head.split(b"\r\n") if l.lower().startswith(b"content-length:"))
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
        bodies = [
            b"\xff\xfe" + envelope("Export", "Bridge Marks").encode("utf-16-le"),
            envelope("Export", "Bridge Span").encode("utf-16-le"),
            envelope("Export", "Bridge Utf8").encode("utf-8"),
        ]
        for body in bodies:
            client.sendall(message(b"POST / HTTP/1.1", body))
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

    test("a folder as written: ok 3", outdir, 0, "exchanges: ok 3, FAIL 0")
    test("each request's HEADER is read, BOM or not, UTF-16 or UTF-8", outdir, 0, "Export Collection Bridge Utf8")
    test("a range lists only its exchanges", outdir, 0, "exchanges: ok 1, FAIL 0 (x-2 to x-2)", "2", "2")

    def tamper(folder):
        with open(os.path.join(folder, "x-2.answer"), "ab") as out:
            out.write(b"x")
    test("an answer one byte longer fails", copy("tamper", tamper), 1, "FAIL x-2: kept bytes differ")
    test("an answer removed fails", copy("noanswer", lambda f: os.remove(os.path.join(f, "x-1.answer"))), 1,
         "FAIL x-1: kept bytes differ")

    def unlogged(folder):
        shutil.copy(os.path.join(folder, "x-3.request"), os.path.join(folder, "x-4.request"))
    test("a request with no logged answer fails as NO ANSWER", copy("unlogged", unlogged), 1, "FAIL x-4 NO ANSWER")

    def gap(folder):
        os.rename(os.path.join(folder, "x-3.request"), os.path.join(folder, "x-5.request"))
        os.rename(os.path.join(folder, "x-3.answer"), os.path.join(folder, "x-5.answer"))
    test("a gap in the numbers fails", copy("gap", gap), 1, "exchange numbers have a gap")
    test("a range past the last exchange fails", outdir, 1, "FAIL x-4: request not kept whole", "1", "4")
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
