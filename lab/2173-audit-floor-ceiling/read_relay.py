#!/usr/bin/env python3 -I
"""A recording relay between ComplyEaze Bridge and Tally's gateway on this computer that passes READS ONLY and records
each one's size, rows and time.

Run:  python -I read_relay.py --listen-port 9012 --record DIR
      python -I read_relay.py --self-test

It listens on 127.0.0.1 only and forwards to 127.0.0.1:9001 only (fixed, not an argument). It writes no request of its
own: every byte it sends to Tally is a byte ComplyEaze Bridge sent it. A request is forwarded byte for byte, with the
client's own Content-Type lines (Tally picks its answer's encoding from them), and the body of its answer is returned byte
for byte (the relay's own reply adds Server and Date headers and recomputes Content-Length). The relay adds only Host and,
on a POST, Content-Length, to what it sends to Tally, and answers its client with Connection: close, one request to a
connection. Requests to Tally are made one at a time; a request to Tally has no time limit (an abandoned request can leave
the gateway stuck), so a request ComplyEaze Bridge gives up on is still read to its end and timed.

It forwards ONLY GET /status (no body) and POST / with a body of at most 1 MiB that decodes (UTF-16 with or without a
byte-order mark, or UTF-8) and is a read: its text holds exactly one TALLYREQUEST element and that holds Export, and it
holds no import marker, no ACTION attribute and no markup declaration or comment. Everything else is answered 403 by the
relay itself, recorded, and NOT forwarded, so no write can pass it.

It records every request it receives, byte for byte, as NNNN-request.bin in DIR. An answer is kept as NNNN-response.bin
only when it is at most 1 MiB (the marks and the small reads); a larger answer (a census span or a data part) is recorded
by its size and sha256 only. It appends one line per request to DIR/manifest.tsv, tab-separated, with these columns:
index, UTC time the request arrived (to the millisecond), method, path, verdict (forwarded, forward_failed or
refused:<reason>), the collection ID the request names (a fixed name in ComplyEaze Bridge's code), SVFROMDATE, SVTODATE,
the AlterID span the formula holds (after, through; empty when none), request bytes and sha256, HTTP status, response bytes
and sha256, rows, seconds to Tally's answer headers, seconds to its last byte, and whether the answer was kept. ROWS are
the VOUCHER (or COMPANY) elements that are children of ENVELOPE/BODY/DATA/COLLECTION in the parsed answer, after dropping
only the characters XML forbids; the CMPINFO block's count lines are not rows. An answer that does not parse gets rows
"unparsed" (the answer still goes back to ComplyEaze Bridge unchanged). DIR must be new or empty; the relay refuses to
start otherwise.

Exit codes: 0 = stopped by Ctrl+C (or self-test passed); 2 = could not start (bad arguments, DIR not empty, port in
use); 1 = self-test failed.
"""
import argparse
import hashlib
import http.client
import http.server
import os
import re
import sys
import threading
import time
import xml.etree.ElementTree as ElementTree

TARGET_HOST = "127.0.0.1"
TARGET_PORT = 9001
MAX_BODY = 1024 * 1024
KEEP_ANSWER_BYTES = 1024 * 1024
CLIENT_READ_TIMEOUT_SECONDS = 60
IMPORT = re.compile(r"<TALLYREQUEST>\s*Import|<IMPORTDATA", re.IGNORECASE)
TALLYREQUEST = re.compile(r"<TALLYREQUEST>(.*?)</TALLYREQUEST>", re.IGNORECASE | re.DOTALL)
UNREAD_MARKERS = re.compile(r"\bACTION\s*=|<!|<\?xml.*<\?", re.IGNORECASE | re.DOTALL)
COLLECTION_ID = re.compile(r"<ID>([^<]{1,100})</ID>")
FROM_DATE = re.compile(r"<SVFROMDATE[^>]*>([0-9]{8})</SVFROMDATE>")
TO_DATE = re.compile(r"<SVTODATE[^>]*>([0-9]{8})</SVTODATE>")
SPAN = re.compile(r"\$AlterID &gt; ([0-9]{1,12}) AND \$AlterID &lt;= ([0-9]{1,12})")
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));",
                       re.IGNORECASE)


def is_read(text):
    """Whether the request is an Export (a read): one TALLYREQUEST that is Export, in any case, and no marker of a write."""
    found = TALLYREQUEST.findall(text)
    return (len(found) == 1 and found[0].strip().lower() == "export" and not IMPORT.search(text)
            and not UNREAD_MARKERS.search(text) and text.lower().count("tallyrequest") == 2)


def decode(body):
    """The text, from UTF-16 (with or without a BOM) or UTF-8; None if it does not decode."""
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


def first(pattern, text):
    found = pattern.search(text or "")
    return found.group(1) if found else ""


def rows(body):
    """The rows of an answer, by structure (see the module text), or "unparsed"."""
    text = decode(body)
    if text is None:
        return "unparsed"
    try:
        root = ElementTree.fromstring(FORBIDDEN.sub("", text.lstrip("﻿")))
    except ElementTree.ParseError:
        return "unparsed"
    if root.tag != "ENVELOPE":
        return "unparsed"
    return sum(len(collection) for collection in root.findall("./BODY/DATA/COLLECTION"))


def utc_ms():
    now = time.time()
    return time.strftime("%Y-%m-%dT%H:%M:%S", time.gmtime(now)) + ".%03dZ" % int(now % 1 * 1000)


class Recorder:
    def __init__(self, folder):
        self.folder = folder
        self.index = 0
        self.lock = threading.Lock()

    def next_index(self):
        with self.lock:
            self.index += 1
            return self.index

    def write(self, index, arrived, method, path, verdict, request, status, response, to_headers, to_end):
        text = decode(request) if request else ""
        span = SPAN.search(text or "")
        kept = len(response) <= KEEP_ANSWER_BYTES
        stem = os.path.join(self.folder, "%04d" % index)
        with open(stem + "-request.bin", "wb") as out:
            out.write(request)
        if kept:
            with open(stem + "-response.bin", "wb") as out:
                out.write(response)
        counted = rows(response) if verdict == "forwarded" and method == "POST" else ""
        line = "\t".join(str(field) for field in (
            index, arrived, method, path, verdict, first(COLLECTION_ID, text), first(FROM_DATE, text),
            first(TO_DATE, text), span.group(1) if span else "", span.group(2) if span else "", len(request),
            hashlib.sha256(request).hexdigest(), status, len(response), hashlib.sha256(response).hexdigest(), counted,
            "" if to_headers is None else "%.3f" % to_headers, "" if to_end is None else "%.3f" % to_end,
            "yes" if kept else "no"))
        with self.lock:
            with open(os.path.join(self.folder, "manifest.tsv"), "a", encoding="utf-8") as out:
                out.write(line + "\n")


class State:
    def __init__(self, recorder, target_port):
        self.recorder = recorder
        self.target_port = target_port
        self.tally = threading.Lock()  # one exchange with Tally at a time


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
                # The client went away (ComplyEaze Bridge gave up); the answer is still recorded.
                pass

        def refuse(self, index, arrived, method, reason, request):
            body = ("refused by the relay: %s" % reason).encode("utf-8")
            self.reply(403, [("Content-Type", "text/plain; charset=utf-8")], body)
            state.recorder.write(index, arrived, method, self.path, "refused:" + reason, request, 403, body, None, None)

        def forward(self, index, arrived, method, request):
            """Sends the request to Tally, reads the whole answer, returns it, then records it. The client's own
            Content-Type lines go on unchanged, in order, and none when it sent none."""
            with state.tally:
                started = time.monotonic()
                connection = http.client.HTTPConnection(TARGET_HOST, state.target_port, timeout=None)
                try:
                    connection.putrequest(method, self.path, skip_accept_encoding=True)
                    for value in self.headers.get_all("Content-Type") or []:
                        connection.putheader("Content-Type", value)
                    if method == "POST":
                        connection.putheader("Content-Length", str(len(request)))
                    connection.endheaders(request if method == "POST" else None)
                    answer = connection.getresponse()
                    to_headers = time.monotonic() - started
                    status, headers, body = answer.status, answer.getheaders(), answer.read()
                    to_end = time.monotonic() - started
                except (OSError, ValueError, http.client.HTTPException):
                    status = None
                finally:
                    connection.close()
            if status is None:
                text = b"the relay could not reach Tally"
                self.reply(502, [("Content-Type", "text/plain; charset=utf-8")], text)
                return state.recorder.write(index, arrived, method, self.path, "forward_failed", request, 502, text,
                                            None, time.monotonic() - started)
            self.reply(status, headers, body)
            state.recorder.write(index, arrived, method, self.path, "forwarded", request, status, body, to_headers,
                                 to_end)

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
            index, arrived = state.recorder.next_index(), utc_ms()
            if self.path != "/status" or self.headers.get("Content-Length", "0") not in ("", "0"):
                return self.refuse(index, arrived, "GET", "get_not_status", b"")
            self.forward(index, arrived, "GET", b"")

        def do_POST(self):
            index, arrived = state.recorder.next_index(), utc_ms()
            length = self.headers.get("Content-Length")
            if length is None or not re.fullmatch(r"[0-9]{1,8}", length) or self.headers.get("Transfer-Encoding"):
                return self.refuse(index, arrived, "POST", "no_content_length", b"")
            if int(length) > MAX_BODY:
                return self.refuse(index, arrived, "POST", "body_too_large", b"")
            request = self.read_body(int(length))
            if request is None:
                return self.refuse(index, arrived, "POST", "client_too_slow", b"")
            if self.path != "/":
                return self.refuse(index, arrived, "POST", "path_not_root", request)
            text = decode(request)
            if text is None:
                return self.refuse(index, arrived, "POST", "body_not_decodable", request)
            if not is_read(text):
                return self.refuse(index, arrived, "POST", "import" if IMPORT.search(text) else "not_an_export",
                                   request)
            self.forward(index, arrived, "POST", request)

        def do_PUT(self):
            self.refuse(state.recorder.next_index(), utc_ms(), self.command, "method_not_allowed", b"")

        do_DELETE = do_PATCH = do_HEAD = do_OPTIONS = do_PUT

    return Handler


class Server(http.server.ThreadingHTTPServer):
    daemon_threads = True


def serve(listen_port, folder, target_port=TARGET_PORT):
    return Server(("127.0.0.1", listen_port), make_handler(State(Recorder(folder), target_port)))


def self_test():
    import socket
    import tempfile
    seen = []
    voucher = "<VOUCHER REMOTEID=\"r\"><DATE>20250401</DATE><VOUCHERNUMBER>1</VOUCHERNUMBER><VOUCHERTYPENAME>J" \
              "</VOUCHERTYPENAME><ALTERID> %d</ALTERID></VOUCHER>"

    def collection(count, pad=0):
        cmpinfo = "<CMPINFO><VOUCHER>0</VOUCHER><COMPANY>0</COMPANY></CMPINFO>"
        body = "".join(voucher % index for index in range(1, count + 1))
        return ("<ENVELOPE><HEADER><VERSION>1</VERSION><STATUS>1</STATUS></HEADER><BODY><DESC>%s</DESC><DATA>"
                "<COLLECTION>%s</COLLECTION></DATA></BODY></ENVELOPE>%s" % (cmpinfo, body, " " * pad))

    def target_answer(method, body):
        if method == "GET":
            return b"<RESPONSE>TallyPrime Server is Running</RESPONSE>"
        text = decode(body)
        if "Big" in text:
            return b"\xff\xfe" + collection(5, pad=KEEP_ANSWER_BYTES).encode("utf-16-le")
        if "Slow" in text:
            time.sleep(1.5)
        if "Forbidden" in text:
            return b"\xff\xfe" + collection(2).replace("<DESC>", "<DESC>&#4;\x04").encode("utf-16-le")
        return b"\xff\xfe" + collection(3).encode("utf-16-le")

    class Target(http.server.BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def log_message(self, *args):
            pass

        def answer(self, body):
            content_types = self.headers.get_all("Content-Type") or []
            seen.append((self.command, self.path, body, content_types, sorted(self.headers.keys(), key=str.lower)))
            data = target_answer(self.command, body)
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
    relay = serve(0, folder, target.server_address[1])
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

    def read_of(collection_id, span=None):
        formula = "$Date &gt;= $$Date:\"20250401\" AND $Date &lt;= $$Date:\"20250630\""
        if span:
            formula += " AND $AlterID &gt; %d AND $AlterID &lt;= %d" % span
        return ("<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
                "<ID>%s</ID></HEADER><BODY><DESC><STATICVARIABLES><SVFROMDATE TYPE=\"Date\">20250401</SVFROMDATE>"
                "<SVTODATE TYPE=\"Date\">20250630</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><SYSTEM "
                "TYPE=\"Formulae\" NAME=\"F\">%s</SYSTEM></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"
                % (collection_id, formula))

    # 1. Reads and status checks are forwarded unchanged: the same bytes reach Tally with the client's own
    # Content-Type and no other header but Host (and Content-Length on a POST); the answer is Tally's, byte for byte.
    reads = [("GET", "/status", None, None),
             ("POST", "/", utf16(read_of("Bridge Agent Voucher Census", (0, 8192))), "text/xml; charset=utf-16"),
             ("POST", "/", utf16(read_of("Bridge Agent Vouchers Big", (0, 42))), "text/xml; charset=utf-16"),
             ("POST", "/", utf16(read_of("Bridge Agent Vouchers Slow")), "text/xml"),
             ("POST", "/", utf16(read_of("Bridge Agent Forbidden")), None),
             ("POST", "/", utf16(read_of("Upper").replace(">Export<", ">EXPORT<")), "text/xml")]
    for index, (method, path, body, content_type) in enumerate(reads):
        sent = [] if content_type is None else [content_type]
        status, answer = send(method, path, body, content_type)
        assert status == 200, "%s %s was not forwarded" % (method, path)
        assert seen[index][:3] == (method, path, body or b""), "%s %s did not reach Tally as sent" % (method, path)
        assert seen[index][3] == sent, "Content-Type changed on %s %s" % (method, path)
        names = sorted(["host"] + ["content-type"] * len(sent) + (["content-length"] if method == "POST" else []))
        assert [n.lower() for n in seen[index][4]] == names, "headers added or dropped on %s %s" % (method, path)
        assert answer == target_answer(method, body or b""), "%s %s got another answer" % (method, path)
    forwarded = len(seen)
    # 2. Writes and anything else reach Tally never.
    write = read_of("X").replace("<TALLYREQUEST>Export</TALLYREQUEST>", "<TALLYREQUEST>Import Data</TALLYREQUEST>")
    controls = [
        ("an import", utf16(write)),
        ("an import data element", utf16(read_of("X").replace("<BODY>", "<BODY><IMPORTDATA/>"))),
        ("an execute request", utf16(read_of("X").replace("Export", "Execute"))),
        ("a spaced import", utf16(write.replace("<TALLYREQUEST>Import", "<TALLYREQUEST> Import"))),
        ("a read with a comment", utf16(read_of("X").replace("<HEADER>", "<HEADER><!-- x -->"))),
        ("a read with an action", utf16(read_of("X").replace("<ID>", "<ID ACTION=\"Create\">"))),
        ("two requests in one", utf16(read_of("X") + read_of("Y"))),
        ("an undecodable body", b"\xff\xfe\x00\xd8\x00"),
    ]
    for label, body in controls:
        assert send("POST", "/", body)[0] == 403, label
    assert send("POST", "/other", utf16(read_of("X")))[0] == 403
    assert send("GET", "/")[0] == 403
    assert send("PUT", "/", utf16(read_of("X")))[0] == 403
    with socket.create_connection(("127.0.0.1", port), timeout=10) as raw:
        raw.sendall(b"POST / HTTP/1.1\r\nHost: relay\r\nContent-Length: %d\r\n\r\n" % (MAX_BODY + 1))
        assert raw.recv(64).startswith(b"HTTP/1.1 403"), "an oversized body was not refused before it was read"
    assert len(seen) == forwarded, "a refused request reached Tally"
    # 3. A client that gives up before the answer: the answer is still read to its end, recorded and timed.
    raw = socket.create_connection(("127.0.0.1", port), timeout=10)
    slow = utf16(read_of("Bridge Agent Vouchers Slow"))
    raw.sendall(b"POST / HTTP/1.1\r\nHost: relay\r\nContent-Length: %d\r\n\r\n" % len(slow) + slow)
    time.sleep(0.3)
    raw.close()
    time.sleep(2.0)
    # 4. The record.
    lines = [line.split("\t") for line in open(os.path.join(folder, "manifest.tsv"), encoding="utf-8")
             .read().splitlines()]
    assert all(len(line) == 19 for line in lines), "a manifest line has the wrong columns"
    by_id = {line[5]: line for line in lines if line[4] == "forwarded"}
    census = by_id["Bridge Agent Voucher Census"]
    assert census[6:10] == ["20250401", "20250630", "0", "8192"], census
    assert census[15] == "3", "rows counted wrongly (CMPINFO or VOUCHERNUMBER counted?): %r" % census
    assert census[18] == "yes" and open(os.path.join(folder, "%04d-response.bin" % int(census[0])), "rb").read() \
        == target_answer("POST", b"x")
    big = by_id["Bridge Agent Vouchers Big"]
    assert big[15] == "5" and big[18] == "no" and int(big[13]) > KEEP_ANSWER_BYTES, big
    assert not os.path.exists(os.path.join(folder, "%04d-response.bin" % int(big[0]))), "a large answer was kept"
    assert by_id["Bridge Agent Forbidden"][15] == "2", "forbidden characters stopped the count"
    slow_lines = [line for line in lines if line[5] == "Bridge Agent Vouchers Slow"]
    assert len(slow_lines) == 2 and all(float(line[17]) >= 1.4 for line in slow_lines), slow_lines
    assert by_id[""][2] == "GET" and by_id[""][15] == "", by_id[""]
    refused = [line for line in lines if line[4].startswith("refused:")]
    assert len(refused) == len(controls) + 4, len(refused)
    assert len([line for line in lines if line[4] == "forwarded"]) == forwarded + 1
    relay.shutdown()
    target.shutdown()
    print("self-test: ok (%d forwarded unchanged, %d refused, none of the refused reached Tally; rows by structure; "
          "an abandoned answer read to its end)" % (forwarded + 1, len(refused)))
    return 0


def main(argv):
    parser = argparse.ArgumentParser(add_help=True)
    parser.add_argument("--listen-port", type=int)
    parser.add_argument("--record")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args(argv[1:])
    if args.self_test:
        try:
            return self_test()
        except Exception as error:  # an assertion, or any failure of the test's own machinery
            print("self-test FAILED: %s: %s" % (error.__class__.__name__, error))
            return 1
    if not args.listen_port or not args.record or args.listen_port == TARGET_PORT:
        print("usage: read_relay.py --listen-port PORT --record DIR (PORT must not be 9001)")
        return 2
    os.makedirs(args.record, exist_ok=True)
    if os.listdir(args.record):
        print("refused: %s is not empty (the recording folder must be new or empty)" % args.record)
        return 2
    try:
        server = serve(args.listen_port, args.record)
    except OSError as error:
        print("could not listen on 127.0.0.1:%d: %s" % (args.listen_port, error))
        return 2
    print("relay: 127.0.0.1:%d -> %s:%d, reads only, recording into %s; Ctrl+C to stop"
          % (args.listen_port, TARGET_HOST, TARGET_PORT, args.record))
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
