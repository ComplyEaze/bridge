"""#683 register capture relay: keeps every exchange, refuses every import. Not for merge.

python -I relay.py LISTEN_PORT TALLY_PORT OUTDIR

The #852 row's relay (lab/852-capture-sheet), changed in two ways for a read-only row:
every request and answer is kept, as exchange-N.request and exchange-N.answer (head and
body, as sent), and an Import Data request is never forwarded: the relay logs
`import_refused` and closes that connection, so the row cannot write to the book.

From the #852 relay, unchanged: the relay of lab/2176-r0, with its drop mode removed. Bridge is
pointed here with BRIDGE_TALLY_PORT=LISTEN_PORT, and each connection Bridge opens gets
one connection to Tally of its own, so a reused connection stays a reused one. Every
request and answer goes through byte for byte.

relay.jsonl logs each connection's open and close and, per request, these as well:
  first_byte_s  request held by the relay to Tally's first answer bytes
  answer_s      the same to Tally's last answer byte
  bridge_left_s the same to Bridge closing its end, or null if it stayed
Times are the relay's clock, seconds. Every forwarded request also logs a `sent` line
before Tally answers, so one Tally never answers still leaves a line, and its request is
kept.

If Bridge has gone when the answer arrives, or the forward fails, the relay logs
`reply_lost` and ends that connection, with Tally's whole answer in the file.

The answer is read whole before it is forwarded, so a large read reaches Bridge later
than it would direct. The relay writes no request of its own. Loopback only. Refuses a
non-empty OUTDIR.
"""
import hashlib
import json
import os
import select
import socket
import sys
import threading
import time

BOUND = 400_000_000
MARK = "<TALLYREQUEST>Import Data</TALLYREQUEST>".encode("utf-16-le")
MARK_UTF8 = b"<TALLYREQUEST>Import Data</TALLYREQUEST>"


class Closed(Exception):
    pass


def read_head(sock, buf):
    while b"\r\n\r\n" not in buf:
        data = sock.recv(65536)
        if not data:
            raise Closed
        buf += data
        if len(buf) > 65536 and b"\r\n\r\n" not in buf:
            raise ValueError("head over bound")
    end = buf.index(b"\r\n\r\n") + 4
    return bytes(buf[:end]), bytearray(buf[end:])


def content_length(head):
    found = [
        line.split(b":", 1)[1].strip()
        for line in head.split(b"\r\n")[1:]
        if line.lower().startswith(b"content-length:")
    ]
    if len(found) > 1:
        raise ValueError("two content lengths")
    if not found:
        return None
    length = int(found[0])
    if not 0 <= length <= BOUND:
        raise ValueError("length over bound")
    return length


def header(head, name):
    found = [
        line.split(b":", 1)[1].strip().decode("ascii", "replace")
        for line in head.split(b"\r\n")[1:]
        if line.lower().startswith(name + b":")
    ]
    return found[0] if found else None


def read_exactly(sock, buf, length):
    while len(buf) < length:
        data = sock.recv(min(1 << 20, length - len(buf)))
        if not data:
            raise Closed
        buf += data
    return bytes(buf[:length]), bytearray(buf[length:])


def read_to_end(sock, buf):
    while len(buf) <= BOUND:
        data = sock.recv(1 << 20)
        if not data:
            return bytes(buf)
        buf += data
    raise ValueError("answer over bound")


def client_gone(sock):
    """True once the far end has closed. Never blocks, never takes a byte."""
    try:
        if not select.select([sock], [], [], 0)[0]:
            return False
        return sock.recv(1, socket.MSG_PEEK) == b""
    except OSError:
        return True


def wait_for_tally(upstream, client, started):
    """Blocks until Tally has bytes; returns when Bridge left, or None if it stayed."""
    left = None
    while True:
        watched = [upstream] if left is not None else [upstream, client]
        ready = select.select(watched, [], [], 600)[0]
        if not ready:
            raise TimeoutError("no answer from Tally in 600 s")
        if upstream in ready:
            return left
        if client_gone(client):
            left = round(time.monotonic() - started, 3)
        else:
            # Bytes from Bridge while it waits: not expected, and not its leaving.
            time.sleep(0.05)


class Relay:
    def __init__(self, tally_port, outdir):
        self.tally_port = tally_port
        self.outdir = outdir
        self.lock = threading.Lock()
        self.conns = 0
        self.exchanges = 0

    def log(self, **event):
        now = time.time()
        event["utc"] = time.strftime("%Y-%m-%dT%H:%M:%S", time.gmtime(now)) + ".%03dZ" % (now % 1 * 1000)
        event["at"] = round(time.monotonic(), 3)
        with self.lock, open(os.path.join(self.outdir, "relay.jsonl"), "a") as out:
            out.write(json.dumps(event) + "\n")

    def keep(self, number, kind, data):
        with open(os.path.join(self.outdir, "exchange-%d.%s" % (number, kind)), "wb") as out:
            out.write(data)

    def serve(self, client, conn):
        upstream = socket.create_connection(("127.0.0.1", self.tally_port), timeout=600)
        from_client, from_tally = bytearray(), bytearray()
        served = 0
        started = None
        self.log(event="open", conn=conn, client_port=client.getpeername()[1])
        try:
            while True:
                head, from_client = read_head(client, from_client)
                body, from_client = read_exactly(
                    client, from_client, content_length(head) or 0
                )
                served += 1
                with self.lock:
                    self.exchanges += 1
                    number = self.exchanges
                self.keep(number, "request", head + body)
                if MARK in body or MARK_UTF8 in body:
                    self.log(event="import_refused", conn=conn, request=served, number=number, sent=len(body))
                    return
                self.log(event="sent", conn=conn, request=served, number=number, sent=len(body))
                started = time.monotonic()
                upstream.sendall(head + body)
                try:
                    left = None if from_tally else wait_for_tally(upstream, client, started)
                    answer_head, from_tally = read_head(upstream, from_tally)
                except Closed:
                    self.log(
                        event="tally_closed",
                        conn=conn,
                        request=served,
                        waited_s=round(time.monotonic() - started, 3),
                    )
                    raise
                first_byte = time.monotonic() - started
                length = content_length(answer_head)
                if length is None:
                    answer = read_to_end(upstream, from_tally)
                else:
                    answer, from_tally = read_exactly(upstream, from_tally, length)
                done = time.monotonic() - started
                if left is None and client_gone(client):
                    left = round(time.monotonic() - started, 3)
                self.keep(number, "answer", answer_head + answer)
                self.log(
                    conn=conn,
                    request=served,
                    number=number,
                    connection=header(answer_head, b"connection"),
                    line=head.split(b"\r\n")[0].decode("ascii", "replace"),
                    sent=len(body),
                    sent_sha256=hashlib.sha256(body).hexdigest(),
                    answered=len(answer),
                    answer_sha256=hashlib.sha256(answer).hexdigest(),
                    first_byte_s=round(first_byte, 3),
                    answer_s=round(done, 3),
                    bridge_left_s=left,
                )
                if left is not None:
                    self.log(event="reply_lost", conn=conn, request=served, number=number, cause="bridge_left")
                    return
                try:
                    client.sendall(answer_head + answer)
                except OSError as error:
                    self.log(
                        event="reply_lost",
                        conn=conn,
                        request=served,
                        number=number,
                        cause=type(error).__name__,
                    )
                    return
                if length is None:
                    return
        except Closed:
            pass
        except (OSError, ValueError) as error:
            self.log(
                event="relay_error",
                conn=conn,
                request=served,
                error=type(error).__name__,
                text=str(error),
                waited_s=round(time.monotonic() - started, 3) if started else None,
            )
        finally:
            self.log(event="close", conn=conn, requests=served)
            upstream.close()
            client.close()


def main():
    listen_port, tally_port, outdir = int(sys.argv[1]), int(sys.argv[2]), sys.argv[3]
    os.makedirs(outdir, exist_ok=True)
    if os.listdir(outdir):
        sys.exit("OUTDIR is not empty")
    relay = Relay(tally_port, outdir)
    server = socket.socket()
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(("127.0.0.1", listen_port))
    server.listen(16)
    relay.log(event="listening", port=listen_port, tally_port=tally_port)
    while True:
        client, _ = server.accept()
        relay.conns += 1
        threading.Thread(target=relay.serve, args=(client, relay.conns), daemon=True).start()


if __name__ == "__main__":
    main()
