#!/usr/bin/env python3 -I
"""A serial, read-only recording relay between ComplyEaze Bridge and Tally's gateway on this computer.

Run:  python -I gs1_relay.py --listen-port 9011 --record DIR
      python -I gs1_relay.py --self-test

It listens on 127.0.0.1 only and forwards to 127.0.0.1:9001 only (fixed, not an argument), one request at a time:
a request is read whole, checked, forwarded, and its answer read whole and returned before the next is accepted.
It never sets a timeout on the forwarded request (an abandoned request can leave the gateway stuck).

It forwards ONLY:
  - GET /status (no body), and
  - POST / whose body is an XML ENVELOPE with HEADER TALLYREQUEST = Export, TYPE = Collection, ID in ALLOWED_IDS,
    whose TDLMESSAGE (if any) holds only COLLECTION elements (ISMODIFY="No", or ISMODIFY="Yes" only on the
    built-in "List of Ledgers" or "List of Groups" named by the header ID) holding only TYPE, FETCH, COMPUTE,
    FILTERS, NATIVEMETHOD, CHILDOF or BELONGSTO, and SYSTEM elements with TYPE="Formulae", and whose text holds none of the write markers in FORBIDDEN (case-insensitive).
    and which matches, in full, one of the requests in templates/ (Bridge's own requests for these reads, byte for
    byte) with only the company name, 8-digit dates and a read planner's AlterID span allowed to differ.
Everything else is answered 403 by the relay itself, recorded, and NOT forwarded: an Import, any other request type,
another ID, a TDL with a FUNCTION, REPORT, OBJECT or any other definition, a body that does not decode or parse.
A forwarded request carries the client's own Content-Type lines unchanged (none when the client sent none, as a
GET /status may), because Tally picks its answer's encoding from them; the relay adds only Host and, on a POST,
Content-Length, and no other header.

It records every request it receives and every answer it returns, byte for byte, as NNNN-request.bin,
NNNN-response.bin and NNNN-response-headers.txt in DIR, and appends one line per request to DIR/manifest.tsv:
index, UTC time, method, path, verdict (forwarded, or refused:<reason>), header ID, request bytes and sha256,
HTTP status, response bytes and sha256. DIR must be new or empty; it refuses to start otherwise.

Exit codes: 0 = stopped by Ctrl+C (or self-test passed); 2 = could not start (bad arguments, DIR not empty,
port in use); 1 = self-test failed.
"""
import argparse
import hashlib
import http.client
import http.server
import os
import re
import sys
import time
import xml.etree.ElementTree as ET

TARGET_HOST = "127.0.0.1"
TARGET_PORT = 9001

# Exactly the request IDs ComplyEaze Bridge sends for list_companies, purchase_register and gst_by_counter_ledger
# on one company (company identity, currencies, the compliance and paired ledger listings, groups, the voucher
# census and window, the company marks), plus the count used on a large book.
ALLOWED_IDS = frozenset({
    "BridgeCompanyExtent",
    "BridgeCompanyBookExtentV2",
    "BridgeCompanyCurrencies",
    "BridgeCompanyBaseCurrency",
    "List of Ledgers",
    "List of Groups",
    "Bridge Agent Voucher Census",
    "Bridge Agent Vouchers",
    "Bridge Agent Company High Water",
})

# A built-in collection Bridge's ledger and group listings extend for the one request with ISMODIFY="Yes"
# (more fields, a computed company GUID): a change to the request's own TDL, not to the book.
MODIFIABLE_BUILT_INS = frozenset({"List of Ledgers", "List of Groups"})

# The only elements a forwarded COLLECTION may hold: what to read, never what to do.
COLLECTION_CHILDREN = frozenset({"TYPE", "FETCH", "COMPUTE", "FILTERS", "NATIVEMETHOD", "CHILDOF", "BELONGSTO"})

# The whole tree a forwarded request may have: each element name and the children it may hold. Any element,
# anywhere, that is not here (a FUNCTION, REPORT or OBJECT definition, a TDL block outside TDLMESSAGE, a second
# HEADER under another parent) refuses the request. Taken from every request ComplyEaze Bridge sends for these reads.
CHILDREN = {
    "ENVELOPE": frozenset({"HEADER", "BODY"}),
    "HEADER": frozenset({"VERSION", "TALLYREQUEST", "TYPE", "ID"}),
    "BODY": frozenset({"DESC"}),
    "DESC": frozenset({"STATICVARIABLES", "TDL"}),
    "STATICVARIABLES": frozenset({"SVCURRENTCOMPANY", "SVEXPORTFORMAT", "SVFROMDATE", "SVTODATE"}),
    "TDL": frozenset({"TDLMESSAGE"}),
    "TDLMESSAGE": frozenset({"COLLECTION", "SYSTEM"}),
    "COLLECTION": COLLECTION_CHILDREN,
}
LEAVES = frozenset({"VERSION", "TALLYREQUEST", "TYPE", "ID", "SVCURRENTCOMPANY", "SVEXPORTFORMAT", "SVFROMDATE",
                    "SVTODATE", "SYSTEM", "FETCH", "COMPUTE", "FILTERS", "NATIVEMETHOD", "CHILDOF", "BELONGSTO"})
# The attributes each element may carry; every other attribute (an ACTION above all) refuses the request.
ATTRIBUTES = {
    "COLLECTION": frozenset({"NAME", "ISMODIFY", "ISINITIALIZE"}),
    "SYSTEM": frozenset({"NAME", "TYPE"}),
    "SVFROMDATE": frozenset({"TYPE"}),
    "SVTODATE": frozenset({"TYPE"}),
}
# The only $$ functions a forwarded request may call: the ones Bridge's reads use, all of them reads.
ALLOWED_FUNCTIONS = frozenset({"$$Date", "$$IsContra", "$$IsCreditNote", "$$IsDebitNote", "$$IsJournal",
                               "$$IsPayment", "$$IsPurchase", "$$IsReceipt", "$$IsSales", "$$LicenseInfo",
                               "$$SysName"})
MAX_BODY = 1024 * 1024
# Every date in a forwarded request must be a real date in this range (the sitting's books, FY 2025-26, and the next).
FIRST_DATE, LAST_DATE = "20250401", "20270331"
# A whole request (headers and body) must arrive within this many seconds of the connection opening.
CLIENT_DEADLINE_SECONDS = 60

# The gate: every forwarded request must match, in full, one of the requests ComplyEaze Bridge sends for these reads,
# as captured byte for byte in templates/ (the repository's register request fixtures, which its tests compare with
# what the code sends), with only three things allowed to differ: the company name, 8-digit dates, and the AlterID
# span the read planner appends to a census or window formula when a window is read in parts.
TEMPLATE_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "templates")
COMPANY = r"[A-Za-z0-9 ._()-]{1,100}"
SPAN = r"(?: AND \$AlterID &gt; [0-9]{1,12} AND \$AlterID &lt;= [0-9]{1,12})?"


def template_pattern(text):
    """The full-match pattern a template allows."""
    company = re.search(r"<SVCURRENTCOMPANY>([^<]*)</SVCURRENTCOMPANY>", text)
    pattern = re.escape(text)
    if company:
        pattern = pattern.replace(re.escape(company.group(1)), COMPANY)
    pattern = re.sub(r"(?<![0-9])[0-9]{8}(?![0-9])", "[0-9]{8}", pattern)
    pattern = pattern.replace(re.escape("</SYSTEM>"), SPAN + re.escape("</SYSTEM>"))
    return re.compile(pattern, re.DOTALL)


def load_templates(folder=TEMPLATE_DIR):
    patterns = []
    for name in sorted(os.listdir(folder)):
        text = decode(open(os.path.join(folder, name), "rb").read())
        if text is None:
            raise SystemExit("template %s does not decode" % name)
        patterns.append((name, template_pattern(text.lstrip("\ufeff"))))
    if not patterns:
        raise SystemExit("no templates in %s" % folder)
    return patterns


TEMPLATES = []
CLIENT_READ_TIMEOUT_SECONDS = 60

# Defence in depth only: the gate is the tree check in verdict_for_post.
FORBIDDEN = ("import", "<action", "importdata", "xmlns", "<function", "<report", "<object",
             "<form", "<part", "<line", "<field", "<button", "<key", "<menu", "$$setvalue", "$$execute")


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


def verdict_for_post(path, body):
    """(None, id) when the body may be forwarded, else (reason, id)."""
    if path != "/":
        return "path_not_root", ""
    text = decode(body)
    if text is None:
        return "body_not_decodable", ""
    lowered = text.lower()
    if re.search(r"\baction\s*=", lowered):
        return "forbidden_marker:action", ""
    for marker in FORBIDDEN:
        if marker in lowered:
            return "forbidden_marker:" + marker.strip("<=\""), ""
    try:
        root = ET.fromstring(text.lstrip("﻿"))
    except ET.ParseError:
        return "body_not_xml", ""
    if root.tag != "ENVELOPE":
        return "not_an_envelope", ""
    header = root.find("HEADER")
    if header is None:
        return "no_header", ""
    request_id = header.findtext("ID") or ""
    if (header.findtext("TALLYREQUEST") or "").strip() != "Export":
        return "not_export", request_id
    if (header.findtext("TYPE") or "").strip() != "Collection":
        return "not_collection", request_id
    if request_id not in ALLOWED_IDS:
        return "id_not_allowed", request_id
    reason = tree_refusal(root, request_id)
    if reason:
        return reason, request_id
    if "<!" in text or "<?" in text:
        return "markup_not_allowed", request_id
    # The bytes forwarded are the bytes matched: a BOM and UTF-16LE, exactly as every template is.
    if not body.startswith(b"\xff\xfe"):
        return "encoding_not_template", request_id
    try:
        bare = body[2:].decode("utf-16-le")
    except UnicodeDecodeError:
        return "encoding_not_template", request_id
    if "\ufeff" in bare or b"\xff\xfe" + bare.encode("utf-16-le") != body:
        return "encoding_not_template", request_id
    if not any(pattern.fullmatch(bare) for _, pattern in TEMPLATES):
        return "not_a_known_bridge_read", request_id
    reason = date_refusal(bare)
    if reason:
        return reason, request_id
    return None, request_id


def date_refusal(text):
    """The first reason the request's dates are not real dates of the allowed range, in order, or None."""
    import datetime
    dates = re.findall(r"(?<![0-9])[0-9]{8}(?![0-9])", text)
    for value in dates:
        try:
            datetime.datetime.strptime(value, "%Y%m%d")
        except ValueError:
            return "date_not_a_date"
        if not FIRST_DATE <= value <= LAST_DATE:
            return "date_out_of_range"
    from_date = re.search(r"<SVFROMDATE[^>]*>([0-9]{8})<", text)
    to_date = re.search(r"<SVTODATE[^>]*>([0-9]{8})<", text)
    if from_date and to_date:
        if from_date.group(1) > to_date.group(1):
            return "dates_reversed"
        formula = re.findall(r'\$\$Date:&quot;([0-9]{8})&quot;|\$\$Date:"([0-9]{8})"', text)
        formula = [a or b for a, b in formula]
        if formula and formula != [from_date.group(1), to_date.group(1)]:
            return "formula_dates_not_the_window"
    return None


def tree_refusal(root, request_id):
    """The first reason the parsed request is not a read of the allowed shape, or None."""
    for parent in root.iter():
        tag = parent.tag
        if "{" in tag or any("{" in name or name.startswith("xmlns") for name in parent.attrib):
            return "namespace_not_allowed"
        allowed_attributes = ATTRIBUTES.get(tag, frozenset())
        for name in parent.attrib:
            if name not in allowed_attributes:
                return "attribute_not_allowed:%s.%s" % (tag, name)
        if tag == "COLLECTION":
            modify = parent.get("ISMODIFY", "No")
            if modify != "No" and not (
                modify == "Yes" and parent.get("NAME") in MODIFIABLE_BUILT_INS and parent.get("NAME") == request_id
            ):
                return "collection_modifies"
            if parent.get("ISINITIALIZE", "No") not in ("Yes", "No"):
                return "collection_initialize_value"
        if tag == "SYSTEM" and parent.get("TYPE") != "Formulae":
            return "system_not_formulae"
        children = list(parent)
        if tag in LEAVES:
            if children:
                return "leaf_has_children:" + tag
        elif tag in CHILDREN:
            for child in children:
                if child.tag not in CHILDREN[tag]:
                    return "element_not_allowed:%s>%s" % (tag, child.tag)
        else:
            return "element_not_allowed:" + tag
        for text in (parent.text or "", parent.tail or ""):
            for function in re.findall(r"\$\$[A-Za-z_][A-Za-z0-9_]*", text):
                if function not in ALLOWED_FUNCTIONS:
                    return "function_not_allowed:" + function
        for value in parent.attrib.values():
            if "$$" in value:
                return "function_in_attribute"
    return None


class Recorder:
    def __init__(self, folder):
        self.folder = folder
        self.index = 0

    def write(self, method, path, verdict, request_id, request, status, headers, response):
        self.index += 1
        stem = os.path.join(self.folder, "%04d" % self.index)
        with open(stem + "-request.bin", "wb") as out:
            out.write(request)
        with open(stem + "-response.bin", "wb") as out:
            out.write(response)
        with open(stem + "-response-headers.txt", "w", encoding="utf-8") as out:
            out.write("%s\n" % status)
            for name, value in headers:
                out.write("%s: %s\n" % (name, value))
        line = "\t".join(str(field) for field in (
            self.index, time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), method, path, verdict, request_id,
            len(request), hashlib.sha256(request).hexdigest(), status, len(response),
            hashlib.sha256(response).hexdigest()))
        with open(os.path.join(self.folder, "manifest.tsv"), "a", encoding="utf-8") as out:
            out.write(line + "\n")


def make_handler(recorder, target_port):
    class Handler(http.server.BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"
        # A client that stops mid-request is dropped after this long, so it cannot hold the serial relay.
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
                # The client went away, or the deadline shut its socket; the request is already recorded.
                pass

        def refuse(self, method, reason, request_id, request):
            body = ("refused by the read-only relay: %s" % reason).encode("utf-8")
            headers = [("Content-Type", "text/plain; charset=utf-8")]
            recorder.write(method, self.path, "refused:" + reason, request_id, request, 403, headers, body)
            self.reply(403, headers, body)

        def forward(self, method, request_id, request):
            connection = http.client.HTTPConnection(TARGET_HOST, target_port, timeout=None)
            try:
                # Tally chooses the answer's encoding from the request's Content-Type (a /status asked with
                # charset=utf-16 is answered in UTF-16), so the client's own Content-Type lines go on unchanged, in
                # order, and none when it sent none. The relay adds only Host (HTTP/1.1 requires it) and, on a POST,
                # Content-Length (the client's own length: exactly that many bytes were read and are forwarded).
                connection.putrequest(method, self.path, skip_accept_encoding=True)
                for value in self.headers.get_all("Content-Type") or []:
                    connection.putheader("Content-Type", value)
                if method == "POST":
                    connection.putheader("Content-Length", str(len(request)))
                connection.endheaders(request if method == "POST" else None)
                answer = connection.getresponse()
                body = answer.read()
                pairs = answer.getheaders()
            except OSError as error:
                connection.close()
                text = ("the relay could not reach Tally: %s" % error.__class__.__name__).encode("utf-8")
                failure = [("Content-Type", "text/plain; charset=utf-8")]
                recorder.write(method, self.path, "forward_failed", request_id, request, 502, failure, text)
                return self.reply(502, failure, text)
            connection.close()
            recorder.write(method, self.path, "forwarded", request_id, request, answer.status, pairs, body)
            self.reply(answer.status, pairs, body)

        def read_body(self, length):
            """The body, or None when it did not arrive within the deadline counted from the connection's start."""
            deadline = self.opened + CLIENT_DEADLINE_SECONDS
            chunks, left = [], length
            while left:
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    return None
                self.connection.settimeout(remaining)
                try:
                    chunk = self.rfile.read1(min(left, 65536))
                except (TimeoutError, OSError):
                    return None
                if not chunk:
                    return None
                chunks.append(chunk)
                left -= len(chunk)
            return b"".join(chunks)

        def setup(self):
            import threading
            self.opened = time.monotonic()
            super().setup()
            # The whole request, headers included, must arrive within the deadline: past it the socket is shut, so
            # a client trickling header bytes cannot hold the serial relay.
            self.watchdog = threading.Timer(CLIENT_DEADLINE_SECONDS, self.shut)
            self.watchdog.daemon = True
            self.watchdog.start()

        def shut(self):
            try:
                self.connection.shutdown(2)
            except OSError:
                pass

        def finish(self):
            self.watchdog.cancel()
            super().finish()

        def send_error(self, code, message=None, explain=None):
            recorder.write(self.command or "?", getattr(self, "path", "?"), "refused:http_%d" % code, "", b"",
                           code, [], (message or "").encode("utf-8", "replace"))
            super().send_error(code, message, explain)

        def do_GET(self):
            if self.path != "/status" or self.headers.get("Content-Length", "0") not in ("", "0"):
                return self.refuse("GET", "get_not_status", "", b"")
            self.forward("GET", "", b"")

        def do_POST(self):
            length = self.headers.get("Content-Length")
            if length is None or not re.fullmatch(r"[0-9]{1,8}", length) or self.headers.get("Transfer-Encoding"):
                return self.refuse("POST", "no_content_length", "", b"")
            if int(length) > MAX_BODY:
                return self.refuse("POST", "body_too_large", "", b"")
            request = self.read_body(int(length))
            if request is None:
                return self.refuse("POST", "client_too_slow", "", b"")
            reason, request_id = verdict_for_post(self.path, request)
            if reason:
                return self.refuse("POST", reason, request_id, request)
            self.forward("POST", request_id, request)

        def do_PUT(self):
            self.refuse("PUT", "method_not_allowed", "", b"")

        do_DELETE = do_PATCH = do_HEAD = do_OPTIONS = do_PUT

    return Handler


def serve(listen_port, folder, target_port=TARGET_PORT):
    server = http.server.HTTPServer(("127.0.0.1", listen_port), make_handler(Recorder(folder), target_port))
    return server


def self_test():
    import socket
    import tempfile
    import threading
    TEMPLATES[:] = load_templates()
    global CLIENT_DEADLINE_SECONDS
    CLIENT_DEADLINE_SECONDS = 2
    target_log = []

    def target_answer(method, content_types):
        """What the fake Tally answers: like Tally, UTF-16 when the request's Content-Type names charset=utf-16 and
        ASCII otherwise, naming every Content-Type it received, so a relay that changes, adds or drops one changes
        the answer."""
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
            names = [name.lower() for name in self.headers.keys()]
            target_log.append((self.command, self.path, body, content_types, names))
            answer = target_answer(self.command, content_types)
            self.send_response(200)
            self.send_header("Content-Length", str(len(answer)))
            self.end_headers()
            self.wfile.write(answer)

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

    def send(method, path, body=None, content_type="text/xml; charset=utf-16"):
        """(status, answer) for one request; content_type None sends no Content-Type at all."""
        connection = http.client.HTTPConnection("127.0.0.1", port, timeout=10)
        connection.request(method, path, body=body, headers={} if content_type is None else {
            "Content-Type": content_type})
        answer = connection.getresponse()
        return answer.status, answer.read()

    def real(name, company="BRIDGE GST LEDGER LAB"):
        text = decode(open(os.path.join(TEMPLATE_DIR, name), "rb").read()).lstrip("\ufeff")
        found = re.search(r"<SVCURRENTCOMPANY>([^<]*)</SVCURRENTCOMPANY>", text)
        return text.replace(found.group(1), company) if found else text

    marks = real("shape_lab_company_marks_request.utf16le.xml")
    utf16 = b"\xff\xfe" + marks.encode("utf-16-le")
    ledgers = real("shape_lab_ledgers_1_request.utf16le.xml")
    census = real("credit_note_day_voucher_census_request.utf16le.xml")
    (day,) = set(re.findall(r"(?<![0-9])[0-9]{8}(?![0-9])", census))
    census = census.replace(day, "20251101")
    census = census.replace('<SVTODATE TYPE="Date">20251101', '<SVTODATE TYPE="Date">20251130')
    census = census.replace('&lt;= $$Date:"20251101"', '&lt;= $$Date:"20251130"')
    assert census.count("20251130") == 2, "the census window was not widened"
    census = census.replace("</SYSTEM>", " AND $AlterID &gt; 120 AND $AlterID &lt;= 182</SYSTEM>")
    # Each forwarded request must reach the target with the client's own Content-Type lines and no other header but
    # Host (and Content-Length on a POST), and its answer must be the one the target gives that Content-Type.
    cases = [
        ("GET", "/status", None, None),
        ("GET", "/status", None, "text/xml"),
        ("POST", "/", b"\xff\xfe" + ledgers.encode("utf-16-le"), "text/xml; charset=utf-16"),
        ("POST", "/", utf16, "text/xml; charset=utf-16"),
        ("POST", "/", b"\xff\xfe" + census.encode("utf-16-le"), "text/xml; charset=utf-16"),
        ("POST", "/", utf16, "text/xml"),
        ("POST", "/", utf16, None),
    ]
    for index, (method, path, body, content_type) in enumerate(cases):
        sent = [] if content_type is None else [content_type]
        status, answer = send(method, path, body, content_type)
        assert status == 200, "%s %s was not forwarded" % (method, path)
        seen = target_log[index]
        assert seen[:3] == (method, path, body or b""), "%s %s did not reach the target as sent" % (method, path)
        assert seen[3] == sent, "%s %s reached the target with Content-Type %r, not the client's %r" % (
            method, path, seen[3], sent)
        expected_names = ["host"] + ["content-type"] * len(sent) + (["content-length"] if method == "POST" else [])
        assert sorted(seen[4]) == sorted(expected_names), "%s %s reached the target with headers %r" % (
            method, path, seen[4])
        assert answer == target_answer(method, sent), "%s %s got the answer to another Content-Type" % (method, path)
    forwarded = len(target_log)
    assert forwarded == len(cases), forwarded
    controls = [
        ("an import", marks.replace("<TALLYREQUEST>Export", "<TALLYREQUEST>Import")),
        ("another ID", marks.replace("<ID>Bridge Agent Company High Water", "<ID>All Masters")),
        ("a modifying collection", marks.replace('ISMODIFY="No"', 'ISMODIFY="Yes"')),
        ("a built-in modified under another ID", ledgers.replace("<ID>List of Ledgers", "<ID>List of Groups")),
        ("a collection holding a definition", marks.replace("<TYPE>Company</TYPE>", "<TYPE>Company</TYPE><DETAILS>X</DETAILS>")),
        ("a function", marks.replace("</TDLMESSAGE>", "<FUNCTION NAME=\"F\"></FUNCTION></TDLMESSAGE>")),
        ("a data request", marks.replace("<TYPE>Collection</TYPE>", "<TYPE>Data</TYPE>")),
        ("an action attribute", marks.replace("<VERSION>1", '<VERSION ACTION="Create">1')),
        ("not xml", "<ENVELOPE><HEADER>"),
        ("a prefixed definition outside TDLMESSAGE", marks.replace("<ENVELOPE>", '<ENVELOPE xmlns:t="urn:x">').replace(
            "</TDL>", '</TDL><t:FUNCTION NAME="F"/>')),
        ("an action with spaces", marks.replace('ISMODIFY="No"', 'ISMODIFY="No" ACTION = "Delete"')),
        ("another function in a formula", marks.replace("<FETCH>", "<COMPUTE>X:$$Call:F</COMPUTE><FETCH>")),
        ("a definition in DESC", marks.replace("</TDL>", "</TDL><REPORT NAME=\"R\"/>")),
        ("an attribute not allowed", marks.replace("<FETCH>", '<FETCH KIND="x">')),
        ("an object type not sent by Bridge", marks.replace("<TYPE>Company</TYPE>", "<TYPE>EditLog</TYPE>")),
        ("a second request type", marks.replace("</TALLYREQUEST>", "</TALLYREQUEST><TALLYREQUEST>Execute</TALLYREQUEST>", 1)),
        ("a comment", marks.replace("<HEADER>", "<HEADER><!-- x -->")),
        ("a document type", "<!DOCTYPE ENVELOPE []>" + marks),
        ("an ID with a trailing space", marks.replace("<ID>Bridge Agent Company High Water</ID>", "<ID>Bridge Agent Company High Water </ID>")),
        ("a span on a read Bridge never spans", ledgers.replace("</FETCH>", " AND $AlterID &gt; 1</FETCH>")),
        ("a wider fetch", ledgers.replace("<FETCH>NAME,", "<FETCH>*, NAME,")),
        ("a span that is not numbers", census.replace("&gt; 120", "&gt; $$Call:F")),
        ("a company slot holding a field", marks.replace("BRIDGE GST LEDGER LAB", "$Name")),
        ("a date slot that is not digits", census.replace("20251101", "2025110x")),
        ("a span slot with other logic", census.replace(" AND $AlterID &gt; 120 AND $AlterID &lt;= 182", " OR 1=1")),
        ("a date that is not a date", census.replace("20251101", "20251301")),
        ("a date out of range", census.replace("20251101", "19000101")),
        ("dates reversed", census.replace("20251130", "20251001")),
        ("formula dates not the window", census.replace('$$Date:"20251101"', '$$Date:"20251102"')),
    ]
    for name, body in controls:
        assert body != marks and body != ledgers and body != census, "control unchanged: " + name
        status, _ = send("POST", "/", b"\xff\xfe" + body.encode("utf-16-le"))
        assert status == 403, name
    encodings = [
        ("utf-8", marks.encode("utf-8")),
        ("utf-8 with a BOM", b"\xef\xbb\xbf" + marks.encode("utf-8")),
        ("utf-16le without a BOM", marks.encode("utf-16-le")),
        ("utf-16be with a BOM", b"\xfe\xff" + marks.encode("utf-16-be")),
        ("a second BOM", b"\xff\xfe" + ("\ufeff" + marks).encode("utf-16-le")),
    ]
    for name, body in encodings:
        assert send("POST", "/", body)[0] == 403, name
    controls += encodings
    with socket.create_connection(("127.0.0.1", port), timeout=10) as raw:
        raw.sendall(b"POST / HTTP/1.1\r\nHost: relay\r\nContent-Length: %d\r\n\r\n" % len(utf16) + utf16[:10])
        time.sleep(CLIENT_DEADLINE_SECONDS + 1)
        answer = raw.recv(64)
        assert answer == b"" or answer.startswith(b"HTTP/1.1 403"), "a slow body held the relay"
    with socket.create_connection(("127.0.0.1", port), timeout=10) as raw:
        raw.sendall(b"BREW / HTTP/1.1\r\nHost: relay\r\n\r\n")
        assert raw.recv(64).startswith(b"HTTP/1.1 501"), "an unknown method was not refused"
    controls += [("slow body", ""), ("unknown method", "")]
    assert send("GET", "/")[0] == 403
    assert send("PUT", "/", utf16)[0] == 403
    assert send("POST", "/other", utf16)[0] == 403
    with socket.create_connection(("127.0.0.1", port), timeout=10) as raw:
        raw.sendall(b"POST / HTTP/1.1\r\nHost: relay\r\nContent-Length: %d\r\n\r\n" % (MAX_BODY + 1))
        assert raw.recv(64).startswith(b"HTTP/1.1 403"), "an oversized body was not refused before it was read"
    assert len(target_log) == forwarded, "a refused request reached the target"
    lines = open(os.path.join(folder, "manifest.tsv"), encoding="utf-8").read().splitlines()
    assert len(lines) == forwarded + len(controls) + 4, len(lines)
    assert sum(1 for line in lines if "\tforwarded\t" in line) == forwarded
    first = open(os.path.join(folder, "0004-request.bin"), "rb").read()
    assert first == utf16 and target_log[3][2] == utf16, "the forwarded bytes are not the bytes received"
    relay.shutdown()
    target.shutdown()
    print("self-test: ok (%d forwarded, %d refused, none of the refused reached the target)"
          % (forwarded, len(lines) - forwarded))
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
        except AssertionError as error:
            print("self-test FAILED: %s" % error)
            return 1
    if not args.listen_port or not args.record or args.listen_port == TARGET_PORT:
        print("usage: gs1_relay.py --listen-port PORT --record DIR (PORT must not be 9001)")
        return 2
    os.makedirs(args.record, exist_ok=True)
    if os.listdir(args.record):
        print("refused: %s is not empty (the recording folder must be new or empty)" % args.record)
        return 2
    TEMPLATES[:] = load_templates()
    print("loaded %d request templates" % len(TEMPLATES))
    try:
        server = serve(args.listen_port, args.record)
    except OSError as error:
        print("could not listen on 127.0.0.1:%d: %s" % (args.listen_port, error))
        return 2
    print("relay: 127.0.0.1:%d -> %s:%d, recording into %s; Ctrl+C to stop"
          % (args.listen_port, TARGET_HOST, TARGET_PORT, args.record))
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
