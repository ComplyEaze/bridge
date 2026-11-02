"""Row S relay: loses the answer to one ledger create, nothing else. Not for merge.

python -I relay.py LISTEN_PORT TALLY_PORT OUTDIR

Bridge is pointed here with BRIDGE_TALLY_PORT=LISTEN_PORT. Every request Bridge
sends is passed to Tally byte for byte and every answer back, except the answer
to the FIRST "Import Data" request: Tally has it, it goes to OUTDIR/create.answer,
and Bridge gets no byte, so Bridge's own 20 s limit ends the send. The relay
writes no request of its own. Loopback only. Refuses a non-empty OUTDIR.
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
    def __init__(self, tally_port, outdir):
        self.tally_port = tally_port
        self.outdir = outdir
        self.lock = threading.Lock()
        self.dropped = False

    def log(self, **event):
        event["utc"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
        with self.lock, open(os.path.join(self.outdir, "relay.jsonl"), "a") as out:
            out.write(json.dumps(event) + "\n")

    def take_drop(self, body):
        if MARK not in body:
            return False
        with self.lock:
            taken, self.dropped = not self.dropped, True
        return taken

    def serve(self, client):
        upstream = socket.create_connection(("127.0.0.1", self.tally_port), timeout=600)
        from_client, from_tally = bytearray(), bytearray()
        try:
            while True:
                head, from_client = read_head(client, from_client)
                body, from_client = read_exactly(
                    client, from_client, content_length(head) or 0
                )
                upstream.sendall(head + body)
                answer_head, from_tally = read_head(upstream, from_tally)
                length = content_length(answer_head)
                if length is None:
                    answer = read_to_end(upstream, from_tally)
                else:
                    answer, from_tally = read_exactly(upstream, from_tally, length)
                drop = self.take_drop(body)
                self.log(
                    line=head.split(b"\r\n")[0].decode("ascii", "replace"),
                    sent=len(body),
                    sent_sha256=hashlib.sha256(body).hexdigest(),
                    answered=len(answer),
                    answer_sha256=hashlib.sha256(answer).hexdigest(),
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
                    self.log(event="bridge_closed", held_s=round(time.monotonic() - held_from, 3))
                    return
                client.sendall(answer_head + answer)
                if length is None:
                    return
        except Closed:
            pass
        except (OSError, ValueError) as error:
            self.log(event="relay_error", error=type(error).__name__, text=str(error))
        finally:
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
        threading.Thread(target=relay.serve, args=(client,), daemon=True).start()


if __name__ == "__main__":
    main()
