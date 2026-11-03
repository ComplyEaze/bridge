"""#2210 relay: records every request and answer as sent. Not for merge.

python -I relay.py LISTEN_PORT TALLY_PORT OUTDIR pass|drop

The #2176 R0 relay, which also writes each distinct request and answer once to
OUTDIR/bodies/<sha256>, exact bytes, so nothing about the wire is retyped.

Row S's relay, with each connection numbered. Bridge is pointed here with
BRIDGE_TALLY_PORT=LISTEN_PORT, and each connection Bridge opens gets one connection
to Tally of its own, so a reused connection stays a reused one. relay.jsonl logs
each connection's open and close and, per request, its connection and its number
on that connection (1 is the connection's first request).

pass: every request and answer goes through byte for byte.
drop: the same, except the answer to the FIRST "Import Data" request: Tally has it,
it goes to OUTDIR/create.answer, and Bridge gets no byte, so Bridge's own 20 s limit
ends the send.

The relay writes no request of its own. Loopback only. Refuses a non-empty OUTDIR.
"""
import hashlib
import json
import os
import socket
import sys
import threading
import time

BOUND = 400_000_000
MARK = "<TALLYREQUEST>Import Data</TALLYREQUEST>".encode("utf-16-le")


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


class Relay:
    def __init__(self, tally_port, outdir, drop_mode):
        self.tally_port = tally_port
        self.outdir = outdir
        self.lock = threading.Lock()
        self.dropped = not drop_mode
        self.conns = 0

    def log(self, **event):
        event["utc"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
        event["at"] = round(time.monotonic(), 3)
        with self.lock, open(os.path.join(self.outdir, "relay.jsonl"), "a") as out:
            out.write(json.dumps(event) + "\n")

    def keep(self, body):
        digest = hashlib.sha256(body).hexdigest()
        path = os.path.join(self.outdir, "bodies", digest)
        with self.lock:
            if not os.path.exists(path):
                with open(path, "wb") as out:
                    out.write(body)
        return digest

    def take_drop(self, body):
        if MARK not in body:
            return False
        with self.lock:
            taken, self.dropped = not self.dropped, True
        return taken

    def serve(self, client, conn):
        upstream = socket.create_connection(("127.0.0.1", self.tally_port), timeout=600)
        from_client, from_tally = bytearray(), bytearray()
        served = 0
        self.log(event="open", conn=conn, client_port=client.getpeername()[1])
        try:
            while True:
                head, from_client = read_head(client, from_client)
                body, from_client = read_exactly(
                    client, from_client, content_length(head) or 0
                )
                served += 1
                upstream.sendall(head + body)
                try:
                    answer_head, from_tally = read_head(upstream, from_tally)
                except Closed:
                    self.log(event="tally_closed", conn=conn, request=served)
                    raise
                length = content_length(answer_head)
                if length is None:
                    answer = read_to_end(upstream, from_tally)
                else:
                    answer, from_tally = read_exactly(upstream, from_tally, length)
                drop = self.take_drop(body)
                self.log(
                    conn=conn,
                    request=served,
                    connection=header(answer_head, b"connection"),
                    line=head.split(b"\r\n")[0].decode("ascii", "replace"),
                    sent=len(body),
                    sent_sha256=self.keep(body),
                    answered=len(answer),
                    answer_sha256=self.keep(answer),
                    dropped=drop,
                )
                if drop:
                    path = os.path.join(self.outdir, "create.answer")
                    with open(path, "wb") as out:
                        out.write(answer_head + answer)
                    held_from = time.monotonic()
                    try:
                        while client.recv(65536):
                            pass
                    except OSError:
                        pass
                    self.log(event="bridge_closed", conn=conn, held_s=round(time.monotonic() - held_from, 3))
                    return
                client.sendall(answer_head + answer)
                if length is None:
                    return
        except Closed:
            pass
        except (OSError, ValueError) as error:
            self.log(event="relay_error", conn=conn, error=type(error).__name__, text=str(error))
        finally:
            self.log(event="close", conn=conn, requests=served)
            upstream.close()
            client.close()


def main():
    listen_port, tally_port, outdir, mode = int(sys.argv[1]), int(sys.argv[2]), sys.argv[3], sys.argv[4]
    if mode not in ("pass", "drop"):
        sys.exit("mode is pass or drop")
    os.makedirs(outdir, exist_ok=True)
    if os.listdir(outdir):
        sys.exit("OUTDIR is not empty")
    relay = Relay(tally_port, outdir, mode == "drop")
    server = socket.socket()
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(("127.0.0.1", listen_port))
    server.listen(16)
    os.makedirs(os.path.join(outdir, "bodies"))
    relay.log(event="listening", port=listen_port, tally_port=tally_port, mode=mode)
    while True:
        client, _ = server.accept()
        relay.conns += 1
        threading.Thread(target=relay.serve, args=(client, relay.conns), daemon=True).start()


if __name__ == "__main__":
    main()
