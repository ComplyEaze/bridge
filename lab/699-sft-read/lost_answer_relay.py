#!/usr/bin/env python3 -I
"""A recording relay between ComplyEaze Bridge and Tally's gateway on this computer that withholds the answer to
ONE request: ComplyEaze Bridge's own one-ledger create.

Run:  python -I lost_answer_relay.py --listen-port 9011 --record DIR --company NAME --ledger-name NAME [--hold-seconds 45]
      python -I lost_answer_relay.py --self-test

It listens on 127.0.0.1 only and forwards to 127.0.0.1:9001 only (fixed, not an argument). It writes no request of its
own: every byte it sends to Tally is a byte ComplyEaze Bridge sent it. A request is forwarded byte for byte, with the
client's own Content-Type lines (Tally picks its answer's encoding from them), and the body of its answer is returned byte for
byte (the relay's own reply adds Server and Date headers and recomputes Content-Length), except in the one case below. The relay adds only Host and, on a POST, Content-Length, to what it sends to Tally, and
answers its client with Connection: close, one request to a connection. Requests to Tally are made one at a time; a
request to Tally has no time limit (an abandoned request can leave the gateway stuck).

It forwards ONLY GET /status (no body) and POST / with a body of at most 1 MiB that decodes (UTF-16 with or without a
byte-order mark, or UTF-8). Everything else is answered 403 by the relay itself, recorded, and NOT forwarded.

THE MATCH RULE. A POST is "the create" when its decoded text, with any byte-order mark removed, equals in full the text
of templates/suspense_create_request.utf16le.xml with its company name replaced by --company and its ledger name (the
LEDGER NAME attribute and the NAME element) replaced by --ledger-name. That template is the request ComplyEaze Bridge
renders for a Suspense ledger (`SUSPENSE_GOLDEN` in src-tauri/src/tally/ledger_create_item_tests.rs, which the
repository's own test compares with what the code renders). Both names must be plain: letters, digits, space, and
. _ ( ) -, no repeated or surrounding space.

What it does with the create. The first request that matches is forwarded to Tally; Tally's whole answer is read and
recorded (as the real answer); and the answer is NOT returned. The connection is held open with nothing sent for
--hold-seconds (default 45; ComplyEaze Bridge gives up at 20 s), then closed by the relay. If the client closes first,
the relay notes when. Other requests are served during the hold. A second request that matches is answered 403 and not
forwarded, so a create is never sent twice through the relay.

Every other POST is forwarded only if it is a read: its text holds exactly one TALLYREQUEST element and that holds
Export, and it holds no import marker, no ACTION attribute and no markup declaration or comment. Anything else (an
import above all) is answered 403 and not forwarded, so the relay passes no write but the one create.

It records every request it receives and every answer, byte for byte, as NNNN-request.bin, NNNN-response.bin and
NNNN-response-headers.txt in DIR, and appends one line per request to DIR/manifest.tsv: index, UTC time to the
millisecond, method, path, verdict (forwarded, forwarded_answer_withheld, or refused:<reason>), request bytes and
sha256, HTTP status, response bytes and sha256 (of Tally's real answer, also when it was not returned), and
answer_returned (yes or no). DIR/events.txt gets one line when the create is matched (with the match rule's expected
sha256 and the request's), and one line when the held connection ends: who closed it and how long after the answer was
withheld. DIR must be new or empty; the relay refuses to start otherwise.

Exit codes: 0 = stopped by Ctrl+C (or self-test passed); 2 = could not start (bad arguments, DIR not empty, port in
use); 1 = self-test failed.
"""
import argparse
import hashlib
import http.client
import http.server
import os
import re
import select
import sys
import threading
import time

TARGET_HOST = "127.0.0.1"
TARGET_PORT = 9001
MAX_BODY = 1024 * 1024
CLIENT_READ_TIMEOUT_SECONDS = 60
TEMPLATE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "templates",
                        "suspense_create_request.utf16le.xml")
# The names the template carries; the relay replaces them with its arguments.
TEMPLATE_COMPANY = "BRIDGE MASTERS LAB"
TEMPLATE_NAME = "MLab M1 Suspense"
PLAIN_NAME = re.compile(r"[A-Za-z0-9._()-]+(?: [A-Za-z0-9._()-]+)*")
IMPORT = re.compile(r"<TALLYREQUEST>\s*Import|<IMPORTDATA", re.IGNORECASE)
TALLYREQUEST = re.compile(r"<TALLYREQUEST>(.*?)</TALLYREQUEST>", re.IGNORECASE | re.DOTALL)
UNREAD_MARKERS = re.compile(r"\bACTION\s*=|<!|<\?xml.*<\?", re.IGNORECASE | re.DOTALL)


def is_read(text):
    """Whether the request is an Export (a read): one TALLYREQUEST that is Export, in any case, and no marker of a write.

    The build sends EXPORT in capitals for some reads (seen on 29 Oct: List of Ledgers), others send Export."""
    found = TALLYREQUEST.findall(text)
    return (len(found) == 1 and found[0].strip().lower() == "export" and not IMPORT.search(text)
            and not UNREAD_MARKERS.search(text) and text.lower().count("tallyrequest") == 2)


def decode(body):
    """The request text, from UTF-16 (with or without a BOM) or UTF-8; None if it does not decode."""
    try:
        if body[:2] in (b"\xff\xfe", b"\xfe\xff"):
            return body.decode("utf-16")
        if body[:3] == b"\xef\xbb\xbf":
            return body[3:].decode("utf-8")
        if len(body) >= 2 and body[1:2] == b"\x00":
            return body.decode("utf-16-le")
        return body.decode("utf-8")
    except UnicodeDecodeError:
        return None


def expected_create_text(company, name):
    """The create's text for this company and ledger name, from the template. SystemExit if the template or a name is
    not as this relay expects."""
    for value in (company, name):
        if not PLAIN_NAME.fullmatch(value) or len(value) > 100:
            raise SystemExit("name not plain: %r" % value)
    text = decode(open(TEMPLATE, "rb").read())
    if text is None:
        raise SystemExit("the template does not decode")
    text = text.lstrip("﻿")
    if text.count(TEMPLATE_COMPANY) != 1 or text.count(TEMPLATE_NAME) != 2:
        raise SystemExit("the template is not the one this relay was written for")
    return text.replace(TEMPLATE_COMPANY, company).replace(TEMPLATE_NAME, name)


def utc_ms():
    now = time.time()
    return time.strftime("%Y-%m-%dT%H:%M:%S", time.gmtime(now)) + ".%03dZ" % int(now % 1 * 1000)


class Recorder:
    def __init__(self, folder):
        self.folder = folder
        self.index = 0
        self.lock = threading.Lock()

    def write(self, method, path, verdict, request, status, headers, response, returned):
        with self.lock:
            self.index += 1
            index = self.index
            stem = os.path.join(self.folder, "%04d" % index)
            with open(stem + "-request.bin", "wb") as out:
                out.write(request)
            with open(stem + "-response.bin", "wb") as out:
                out.write(response)
            with open(stem + "-response-headers.txt", "w", encoding="utf-8") as out:
                out.write("%s\n" % status)
                for name, value in headers:
                    out.write("%s: %s\n" % (name, value))
            line = "\t".join(str(field) for field in (
                index, utc_ms(), method, path, verdict, len(request), hashlib.sha256(request).hexdigest(), status,
                len(response), hashlib.sha256(response).hexdigest(), "yes" if returned else "no"))
            with open(os.path.join(self.folder, "manifest.tsv"), "a", encoding="utf-8") as out:
                out.write(line + "\n")
            return index

    def event(self, text):
        with self.lock:
            with open(os.path.join(self.folder, "events.txt"), "a", encoding="utf-8") as out:
                out.write("%s\t%s\n" % (utc_ms(), text))


class State:
    """What one relay run shares between its handler threads."""

    def __init__(self, recorder, target_port, expected_text, hold_seconds):
        self.recorder = recorder
        self.target_port = target_port
        self.expected_text = expected_text
        self.hold_seconds = hold_seconds
        self.tally = threading.Lock()  # one exchange with Tally at a time
        self.create_seen = False
        self.create_lock = threading.Lock()


def withhold(connection, seconds):
    """Holds the connection with nothing sent: ("client_closed", seconds) when the client closes first, else
    ("relay_closed", seconds) when the time is up."""
    start = time.monotonic()
    while True:
        left = seconds - (time.monotonic() - start)
        if left <= 0:
            return "relay_closed", time.monotonic() - start
        try:
            ready, _, _ = select.select([connection], [], [], min(left, 0.2))
            if ready and not connection.recv(65536):
                return "client_closed", time.monotonic() - start
        except OSError:
            return "client_closed", time.monotonic() - start


def make_handler(state):
    class Handler(http.server.BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"
        # A client that stops mid-request is dropped after this long, so it cannot hold a handler thread.
        timeout = CLIENT_READ_TIMEOUT_SECONDS

        def log_message(self, *args):
            pass

        def reply(self, status, headers, body):
            self.close_connection = True
            try:
                self.send_response(status)
                for name, value in headers:
                    if name.lower() not in ("content-length", "transfer-encoding", "connection"):
                        self.send_header(name, value)
                self.send_header("Content-Length", str(len(body)))
                self.send_header("Connection", "close")
                self.end_headers()
                self.wfile.write(body)
            except OSError:
                # The client went away; the request is already recorded.
                pass

        def refuse(self, method, reason, request):
            body = ("refused by the relay: %s" % reason).encode("utf-8")
            headers = [("Content-Type", "text/plain; charset=utf-8")]
            state.recorder.write(method, self.path, "refused:" + reason, request, 403, headers, body, True)
            self.reply(403, headers, body)

        def exchange(self, method, request):
            """(status, headers, body) from Tally, or None when the relay could not reach it. The client's own
            Content-Type lines go on unchanged, in order, and none when it sent none."""
            with state.tally:
                connection = http.client.HTTPConnection(TARGET_HOST, state.target_port, timeout=None)
                try:
                    connection.putrequest(method, self.path, skip_accept_encoding=True)
                    for value in self.headers.get_all("Content-Type") or []:
                        connection.putheader("Content-Type", value)
                    if method == "POST":
                        connection.putheader("Content-Length", str(len(request)))
                    connection.endheaders(request if method == "POST" else None)
                    answer = connection.getresponse()
                    return answer.status, answer.getheaders(), answer.read()
                except (OSError, ValueError, http.client.HTTPException):
                    return None
                finally:
                    connection.close()

        def forward(self, method, request):
            got = self.exchange(method, request)
            if got is None:
                text = b"the relay could not reach Tally"
                failure = [("Content-Type", "text/plain; charset=utf-8")]
                state.recorder.write(method, self.path, "forward_failed", request, 502, failure, text, True)
                return self.reply(502, failure, text)
            status, headers, body = got
            state.recorder.write(method, self.path, "forwarded", request, status, headers, body, True)
            self.reply(status, headers, body)

        def withhold_create(self, request):
            started = time.monotonic()
            got = self.exchange("POST", request)
            if got is None:
                state.recorder.write("POST", self.path, "forward_failed", request, 502, [], b"", False)
                state.recorder.event("the create could not be completed with Tally; it may have been sent")
                return self.reply(502, [("Content-Type", "text/plain; charset=utf-8")], b"the relay could not reach Tally")
            status, headers, body = got
            index = state.recorder.write("POST", self.path, "forwarded_answer_withheld", request, status, headers,
                                         body, False)
            self.close_connection = True
            who, seconds = withhold(self.connection, max(0.0, state.hold_seconds - (time.monotonic() - started)))
            state.recorder.event("%04d held connection ended: %s, %.1f s after the create was sent to Tally"
                                 % (index, who, time.monotonic() - started))

        def read_body(self, length):
            """The body, or None when it did not arrive within the client deadline."""
            deadline = time.monotonic() + CLIENT_READ_TIMEOUT_SECONDS
            chunks, left = [], length
            while left:
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    return None
                self.connection.settimeout(remaining)
                try:
                    chunk = self.rfile.read1(min(left, 65536))
                except OSError:
                    return None
                if not chunk:
                    return None
                chunks.append(chunk)
                left -= len(chunk)
            return b"".join(chunks)

        def do_GET(self):
            if self.path != "/status" or self.headers.get("Content-Length", "0") not in ("", "0"):
                return self.refuse("GET", "get_not_status", b"")
            self.forward("GET", b"")

        def do_POST(self):
            length = self.headers.get("Content-Length")
            if length is None or not re.fullmatch(r"[0-9]{1,8}", length) or self.headers.get("Transfer-Encoding"):
                return self.refuse("POST", "no_content_length", b"")
            if int(length) > MAX_BODY:
                return self.refuse("POST", "body_too_large", b"")
            request = self.read_body(int(length))
            if request is None:
                return self.refuse("POST", "client_too_slow", b"")
            if self.path != "/":
                return self.refuse("POST", "path_not_root", request)
            text = decode(request)
            if text is None:
                return self.refuse("POST", "body_not_decodable", request)
            if text == state.expected_text:
                with state.create_lock:
                    first = not state.create_seen
                    state.create_seen = True
                if not first:
                    return self.refuse("POST", "second_create", request)
                state.recorder.event("create matched: request sha256 %s, expected text sha256 %s (utf-8)"
                                     % (hashlib.sha256(request).hexdigest(),
                                        hashlib.sha256(state.expected_text.encode("utf-8")).hexdigest()))
                return self.withhold_create(request)
            if not is_read(text):
                return self.refuse("POST", "import_not_the_create" if IMPORT.search(text) else "not_an_export", request)
            self.forward("POST", request)

        def do_PUT(self):
            self.refuse(self.command, "method_not_allowed", b"")

        do_DELETE = do_PATCH = do_HEAD = do_OPTIONS = do_PUT

    return Handler


class Server(http.server.ThreadingHTTPServer):
    daemon_threads = True


def serve(listen_port, folder, company, name, hold_seconds, target_port=TARGET_PORT):
    state = State(Recorder(folder), target_port, expected_create_text(company, name), hold_seconds)
    return Server(("127.0.0.1", listen_port), make_handler(state))


def self_test():
    import socket
    import tempfile
    company, name = "BRIDGE LOSTANS LAB", "LAns Suspense"
    expected = expected_create_text(company, name)
    template_text = decode(open(TEMPLATE, "rb").read()).lstrip("﻿")
    assert "<NAME>%s</NAME>" % name in expected and company in expected and "ACTION=\"Create\"" in expected
    assert expected != template_text and "MLab" not in expected and "MASTERS LAB" not in expected
    answer_of = b"<ENVELOPE><LINEERROR/><CREATED>1</CREATED></ENVELOPE>"
    seen = []

    def target_answer(method, content_types, body):
        if method == "POST" and decode(body) is not None and IMPORT.search(decode(body)):
            return answer_of
        text = "<RESPONSE>TallyPrime Server is Running</RESPONSE>" if method == "GET" else "<ENVELOPE/>"
        text += "<!-- content-type: %s -->" % (" | ".join(content_types) or "none")
        if "charset=utf-16" in " ".join(content_types).lower():
            return b"\xff\xfe" + text.encode("utf-16-le")
        return text.encode("ascii")

    class Target(http.server.BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def log_message(self, *args):
            pass

        def answer(self, body):
            content_types = self.headers.get_all("Content-Type") or []
            seen.append((self.command, self.path, body, content_types, sorted(self.headers.keys(), key=str.lower)))
            data = target_answer(self.command, content_types, body)
            self.send_response(200)
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)

        def do_GET(self):
            self.answer(b"")

        def do_POST(self):
            self.answer(self.rfile.read(int(self.headers["Content-Length"])))

    target = http.server.HTTPServer(("127.0.0.1", 0), Target)
    threading.Thread(target=target.serve_forever, daemon=True).start()
    folder = tempfile.mkdtemp()
    hold = 6.0
    relay = serve(0, folder, company, name, hold, target.server_address[1])
    threading.Thread(target=relay.serve_forever, daemon=True).start()
    port = relay.server_address[1]

    def send(method, path, body=None, content_type="text/xml; charset=utf-16", timeout=10):
        connection = http.client.HTTPConnection("127.0.0.1", port, timeout=timeout)
        connection.request(method, path, body=body, headers={} if content_type is None else {
            "Content-Type": content_type})
        answer = connection.getresponse()
        return answer.status, answer.read()

    def utf16(text):
        return b"\xff\xfe" + text.encode("utf-16-le")

    read_text = ('<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>'
                 '<ID>Bridge Agent Company High Water</ID></HEADER></ENVELOPE>')
    read = utf16(read_text)
    read_upper = utf16(read_text.replace("<TALLYREQUEST>Export<", "<TALLYREQUEST>EXPORT<"))
    # 1. Unrelated requests are forwarded unchanged: the same bytes reach Tally, with the client's own Content-Type
    # lines and no other header but Host (and Content-Length on a POST), and the answer is the one Tally gives them.
    cases = [("GET", "/status", None, None), ("GET", "/status", None, "text/xml"),
             ("POST", "/", read, "text/xml; charset=utf-16"), ("POST", "/", read, "text/xml"), ("POST", "/", read, None),
             ("POST", "/", read_upper, "text/xml")]
    for index, (method, path, body, content_type) in enumerate(cases):
        sent = [] if content_type is None else [content_type]
        status, answer = send(method, path, body, content_type)
        assert status == 200, "%s %s was not forwarded" % (method, path)
        assert seen[index][:3] == (method, path, body or b""), "%s %s did not reach Tally as sent" % (method, path)
        assert seen[index][3] == sent, "Content-Type changed on %s %s" % (method, path)
        names = sorted(["host"] + ["content-type"] * len(sent) + (["content-length"] if method == "POST" else []))
        assert [n.lower() for n in seen[index][4]] == names, "headers added or dropped on %s %s" % (method, path)
        assert answer == target_answer(method, sent, body or b""), "%s %s got another answer" % (method, path)
    forwarded = len(seen)
    # 2. Refusals reach Tally never: another import, the create for another company or name, a second create's
    # twin comes later, an undecodable body, a wrong path, a wrong method, an oversized body.
    controls = [
        ("an import of another name", utf16(expected.replace(name, "LAns Other"))),
        ("the create for another company", utf16(expected.replace(company, "BRIDGE OTHER LAB"))),
        ("the create with a field added", utf16(expected.replace("</LEDGER>", "<OPENINGBALANCE>1</OPENINGBALANCE></LEDGER>"))),
        ("a voucher import", utf16(expected.replace("All Masters", "Vouchers"))),
        ("an undecodable body", b"\xff\xfe\x00\xd8\x00"),
        ("an execute request", utf16(read_text.replace("Export", "Execute"))),
        ("a spaced import", utf16(expected.replace("<TALLYREQUEST>Import Data", "<TALLYREQUEST> Import Data"))),
        ("a read with a comment", utf16(read_text.replace("<HEADER>", "<HEADER><!-- x -->"))),
        ("a read with an action", utf16(read_text.replace("<ID>", "<ID ACTION=\"Create\">"))),
        ("the create under two byte-order marks", b"\xff\xfe\xff\xfe" + expected.encode("utf-16-le")),
    ]
    for label, body in controls:
        assert send("POST", "/", body)[0] == 403, label
    assert send("POST", "/other", read)[0] == 403
    assert send("GET", "/")[0] == 403
    assert send("PUT", "/", read)[0] == 403
    with socket.create_connection(("127.0.0.1", port), timeout=10) as raw:
        raw.sendall(b"POST / HTTP/1.1\r\nHost: relay\r\nContent-Length: %d\r\n\r\n" % (MAX_BODY + 1))
        assert raw.recv(64).startswith(b"HTTP/1.1 403"), "an oversized body was not refused before it was read"
    assert len(seen) == forwarded, "a refused request reached Tally"
    # 3. The create: forwarded once, byte for byte; Tally's real answer is recorded; the client gets no byte, and an
    # unrelated read is answered at once while the connection is held.
    create = utf16(expected)
    result = {}

    def client():
        opened = time.monotonic()
        raw = socket.create_connection(("127.0.0.1", port), timeout=hold + 4)
        raw.sendall(b"POST / HTTP/1.1\r\nHost: relay\r\nContent-Type: text/xml; charset=utf-16\r\n"
                    b"Content-Length: %d\r\n\r\n" % len(create) + create)
        got = b""
        try:
            while True:
                chunk = raw.recv(4096)
                if not chunk:
                    break
                got += chunk
        except socket.timeout:
            result["timed_out"] = True
        result["ended_after"] = time.monotonic() - opened
        result["received"] = got
        raw.close()

    thread = threading.Thread(target=client)
    thread.start()
    time.sleep(1.0)
    started = time.monotonic()
    assert send("POST", "/", read)[0] == 200 and time.monotonic() - started < 2.0, "the hold blocked another request"
    thread.join(20)
    assert not result.get("timed_out") and result.get("received") == b"", "the client received something: %r" % result
    assert result["ended_after"] >= hold - 0.5, "the relay ended the held connection early: %r" % result
    assert seen[-2][2] == create and seen[-2][0] == "POST", "the create did not reach Tally byte for byte"
    assert sum(1 for entry in seen if entry[2] == create) == 1
    # 4. A second create, identical, is refused and not forwarded.
    before = len(seen)
    assert send("POST", "/", create)[0] == 403
    assert len(seen) == before, "a second create reached Tally"
    main_seen = len(seen)
    # 5. A client that gives up first is noted as such.
    folder2 = tempfile.mkdtemp()
    relay2 = serve(0, folder2, company, name, hold, target.server_address[1])
    threading.Thread(target=relay2.serve_forever, daemon=True).start()
    raw = socket.create_connection(("127.0.0.1", relay2.server_address[1]), timeout=10)
    raw.sendall(b"POST / HTTP/1.1\r\nHost: relay\r\nContent-Length: %d\r\n\r\n" % len(create) + create)
    time.sleep(1.5)
    raw.close()
    time.sleep(1.5)
    events = open(os.path.join(folder2, "events.txt"), encoding="utf-8").read()
    assert "client_closed" in events, events
    relay2.shutdown()
    # 6. The record: one line per request, the withheld answer's real bytes recorded, and no refused request forwarded.
    lines = [line.split("\t") for line in open(os.path.join(folder, "manifest.tsv"), encoding="utf-8").read().splitlines()]
    withheld = [line for line in lines if line[4] == "forwarded_answer_withheld"]
    assert len(withheld) == 1 and withheld[0][10] == "no", withheld
    assert withheld[0][9] == hashlib.sha256(answer_of).hexdigest() and int(withheld[0][8]) == len(answer_of)
    index = int(withheld[0][0])
    assert open(os.path.join(folder, "%04d-response.bin" % index), "rb").read() == answer_of
    assert open(os.path.join(folder, "%04d-request.bin" % index), "rb").read() == create
    refused = [line for line in lines if line[4].startswith("refused:")]
    assert len(refused) == len(controls) + 5 and sum(1 for line in refused if line[4] == "refused:second_create") == 1
    assert len(lines) == main_seen + len(refused), (len(lines), main_seen, len(refused))
    assert sum(1 for line in lines if line[4] == "forwarded") == main_seen - 1
    events = open(os.path.join(folder, "events.txt"), encoding="utf-8").read()
    assert "create matched" in events and "relay_closed" in events, events
    relay.shutdown()
    target.shutdown()
    print("self-test: ok (%d forwarded unchanged, 1 answer withheld, %d refused, none of the refused reached Tally)"
          % (main_seen - 1, len(refused)))
    return 0


def main(argv):
    parser = argparse.ArgumentParser(add_help=True)
    parser.add_argument("--listen-port", type=int)
    parser.add_argument("--record")
    parser.add_argument("--company")
    parser.add_argument("--ledger-name")
    parser.add_argument("--hold-seconds", type=float, default=45.0)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args(argv[1:])
    if args.self_test:
        try:
            return self_test()
        except Exception as error:  # an assertion, or any failure of the test's own machinery
            print("self-test FAILED: %s: %s" % (error.__class__.__name__, error))
            return 1
    if (not args.listen_port or not args.record or not args.company or not args.ledger_name
            or args.listen_port == TARGET_PORT or not 25 <= args.hold_seconds <= 300):
        print("usage: lost_answer_relay.py --listen-port PORT --record DIR --company NAME --ledger-name NAME "
              "[--hold-seconds 25..300] (PORT must not be 9001)")
        return 2
    os.makedirs(args.record, exist_ok=True)
    if os.listdir(args.record):
        print("refused: %s is not empty (the recording folder must be new or empty)" % args.record)
        return 2
    try:
        server = serve(args.listen_port, args.record, args.company, args.ledger_name, args.hold_seconds)
    except OSError as error:
        print("could not listen on 127.0.0.1:%d: %s" % (args.listen_port, error))
        return 2
    print("relay: 127.0.0.1:%d -> %s:%d, recording into %s; withholds the answer to the create of %r in %r for %g s; "
          "Ctrl+C to stop" % (args.listen_port, TARGET_HOST, TARGET_PORT, args.record, args.ledger_name,
                              args.company, args.hold_seconds))
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
