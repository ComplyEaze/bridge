# Logging HTTP proxy for the #1531 run: listens on 127.0.0.1:<listen>, forwards each request to Tally
# at 127.0.0.1:<target> one at a time, and saves request and response bytes plus an index line.
# Usage: python -I logproxy.py <listen_port> <target_port> <out_dir>
import hashlib
import http.client
import http.server
import os
import sys
import threading
import time

LISTEN, TARGET, OUT = int(sys.argv[1]), int(sys.argv[2]), sys.argv[3]
os.makedirs(OUT, exist_ok=True)
LOCK = threading.Lock()
SEQ = [0]


def ts():
    t = time.time()
    return time.strftime('%Y-%m-%dT%H:%M:%S', time.gmtime(t)) + '.%03dZ' % int((t % 1) * 1000)


class H(http.server.BaseHTTPRequestHandler):
    protocol_version = 'HTTP/1.1'

    def log_message(self, *a):
        pass

    def _relay(self):
        n = int(self.headers.get('Content-Length') or 0)
        body = self.rfile.read(n) if n else b''
        with LOCK:
            SEQ[0] += 1
            seq = SEQ[0]
            t0 = ts()
            try:
                c = http.client.HTTPConnection('127.0.0.1', TARGET, timeout=600)
                hdrs = {k: v for k, v in self.headers.items() if k.lower() not in ('host', 'connection', 'content-length')}
                c.request(self.command, self.path, body=body, headers=hdrs)
                r = c.getresponse()
                data = r.read()
                status, rh = r.status, r.getheaders()
                c.close()
            except Exception as e:
                data, status, rh = repr(e).encode(), 599, []
            t1 = ts()
            open(os.path.join(OUT, '%04d-req.bin' % seq), 'wb').write(body)
            open(os.path.join(OUT, '%04d-resp.bin' % seq), 'wb').write(data)
            with open(os.path.join(OUT, 'index.tsv'), 'a', encoding='utf-8', newline='\n') as f:
                f.write('\t'.join([str(seq), t0, t1, self.command, self.path, str(len(body)), hashlib.sha256(body).hexdigest(),
                                   str(status), str(len(data)), hashlib.sha256(data).hexdigest()]) + '\n')
        self.send_response(status if status != 599 else 502)
        for k, v in rh:
            if k.lower() not in ('content-length', 'connection', 'transfer-encoding'):
                self.send_header(k, v)
        self.send_header('Content-Length', str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    do_POST = _relay
    do_GET = _relay


http.server.ThreadingHTTPServer(('127.0.0.1', LISTEN), H).serve_forever()
