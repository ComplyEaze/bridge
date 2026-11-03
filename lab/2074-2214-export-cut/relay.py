"""#2074 and #2214 relay: holds a mailing read behind an import. Not for merge.

python -I relay.py LISTEN_PORT TALLY_PORT OUTDIR ARMDIR

The #2210 relay in pass mode: each connection Bridge opens gets one connection to Tally
of its own, every request and answer goes through byte for byte, relay.jsonl logs them,
and bodies/<sha256> holds each distinct one once. Two changes:

- A closed client closes its Tally connection at once, as Bridge's own close would, and
  logs bridge_cut. The earlier relays stayed waiting on Tally there.
- ARMDIR/front, when present, holds the path of one request file. The next request that
  names LEDMAILINGDETAILS.LIST consumes it: the relay sends that file to Tally on a new
  connection of its own, waits for the last byte to be written and 0.3 s more, and then
  forwards the mailing request. front-<n>.answer keeps Tally's answer. With no ARMDIR/front
  nothing of this happens, so an unarmed relay sends no request of its own.

Loopback only. Refuses a non-empty OUTDIR.
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
MAILING = "LEDMAILINGDETAILS.LIST"
MARKS = (MAILING.encode("utf-16-le"), MAILING.encode("utf-8"))


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
    def __init__(self, tally_port, outdir, armdir):
        self.tally_port = tally_port
        self.outdir = outdir
        self.armdir = armdir
        self.lock = threading.Lock()
        self.conns = 0
        self.fronts = 0

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

    def take_front(self, body):
        """The armed request file, once, for a request that names the mailing details."""
        if not any(mark in body for mark in MARKS):
            return None
        path = os.path.join(self.armdir, "front")
        with self.lock:
            if not os.path.exists(path):
                return None
            with open(path, encoding="ascii") as armed:
                name = armed.read().strip()
            os.replace(path, path + ".used-{}".format(self.fronts + 1))
            self.fronts += 1
            return name, self.fronts

    def send_front(self, name, number, written):
        try:
            with open(name, "rb") as source:
                data = source.read()
            sock = socket.create_connection(("127.0.0.1", self.tally_port), timeout=600)
            head = (
                "POST / HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nContent-Type: text/xml; charset=utf-16\r\n"
                "Content-Length: {}\r\nConnection: close\r\n\r\n"
            ).format(self.tally_port, len(data)).encode("ascii")
            self.log(event="front_send", front=number, bytes=len(data), sent_sha256=hashlib.sha256(data).hexdigest())
            sock.sendall(head + data)
            self.log(event="front_written", front=number)
            written.set()
            answer_head, rest = read_head(sock, bytearray())
            self.log(event="front_first_byte", front=number)
            length = content_length(answer_head)
            if length is None:
                answer = read_to_end(sock, rest)
            else:
                answer, _ = read_exactly(sock, rest, length)
            with open(os.path.join(self.outdir, "front-{}.answer".format(number)), "wb") as out:
                out.write(answer_head + answer)
            self.log(event="front_answered", front=number, answered=len(answer))
            sock.close()
        except (OSError, ValueError, Closed) as error:
            self.log(event="relay_error", front=number, error=type(error).__name__, text=str(error))
        finally:
            written.set()

    def wait_for_tally(self, client, upstream):
        """True when Tally has bytes for us, False when the client closed first."""
        while True:
            ready, _, _ = select.select([client, upstream], [], [], 1.0)
            if upstream in ready:
                return True
            if client in ready and not client.recv(65536, socket.MSG_PEEK):
                return False

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
                front = self.take_front(body)
                if front:
                    written = threading.Event()
                    threading.Thread(target=self.send_front, args=(*front, written), daemon=True).start()
                    written.wait(120)
                    time.sleep(0.3)
                upstream.sendall(head + body)
                self.log(event="forwarded", conn=conn, request=served, sent=len(body), front=front[1] if front else None)
                if not from_tally and not self.wait_for_tally(client, upstream):
                    self.log(event="bridge_cut", conn=conn, request=served, sent=len(body), sent_sha256=self.keep(body))
                    return
                try:
                    answer_head, from_tally = read_head(upstream, from_tally)
                except Closed:
                    self.log(event="tally_closed", conn=conn, request=served)
                    raise
                self.log(event="first_byte", conn=conn, request=served, sent=len(body))
                length = content_length(answer_head)
                if length is None:
                    answer = read_to_end(upstream, from_tally)
                else:
                    answer, from_tally = read_exactly(upstream, from_tally, length)
                self.log(
                    conn=conn,
                    request=served,
                    connection=header(answer_head, b"connection"),
                    line=head.split(b"\r\n")[0].decode("ascii", "replace"),
                    sent=len(body),
                    sent_sha256=self.keep(body),
                    answered=len(answer),
                    answer_sha256=self.keep(answer),
                )
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
    listen_port, tally_port, outdir, armdir = int(sys.argv[1]), int(sys.argv[2]), sys.argv[3], sys.argv[4]
    os.makedirs(outdir, exist_ok=True)
    if os.listdir(outdir):
        sys.exit("OUTDIR is not empty")
    if not os.path.isdir(armdir) or os.path.exists(os.path.join(armdir, "front")):
        sys.exit("ARMDIR must exist, not armed")
    relay = Relay(tally_port, outdir, armdir)
    server = socket.socket()
    server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server.bind(("127.0.0.1", listen_port))
    server.listen(16)
    os.makedirs(os.path.join(outdir, "bodies"))
    relay.log(event="listening", port=listen_port, tally_port=tally_port)
    while True:
        client, _ = server.accept()
        relay.conns += 1
        threading.Thread(target=relay.serve, args=(client, relay.conns), daemon=True).start()


if __name__ == "__main__":
    main()
