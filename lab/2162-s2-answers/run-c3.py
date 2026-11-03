#!/usr/bin/env python3
# run.py URL TAIL: sends I-<TAIL>.xml once, then R-<TAIL>.xml at once (no wait), at +5 s and at
# +60 s, each counted from the moment curl returned the import's answer. One request at a time,
# each answer written by curl -o, and every send and return time written to times-<TAIL>.txt.
# Example: python3 -I run.py http://localhost:9000 0046
import pathlib, subprocess, sys, time
url, tail = sys.argv[1], sys.argv[2]
here = pathlib.Path(__file__).resolve().parent
log = []
def send(request, answer):
    start = time.monotonic()
    done = subprocess.run(["curl.exe", "-sS", "--fail-with-body", "-H", "Content-Type: text/xml; charset=utf-16",
        "--data-binary", f"@{here / request}", "-o", str(here / answer), url])
    end = time.monotonic()
    log.append((request, answer, start, end, done.returncode))
    if done.returncode != 0:
        raise SystemExit(f"curl exit {done.returncode} on {request}: stop, do not resend")
    return end
t0 = send(f"I-{tail}.xml", f"a-import-{tail}.xml")
for label, at in (("00s", 0), ("05s", 5), ("60s", 60)):
    wait = t0 + at - time.monotonic()
    if wait > 0:
        time.sleep(wait)
    send(f"R-{tail}.xml", f"a-readback-{tail}-{label}.xml")
with open(here / f"times-{tail}.txt", "w") as out:
    for request, answer, start, end, code in log:
        out.write(f"{request}\t{answer}\tsent +{start - t0:.3f}s\treturned +{end - t0:.3f}s\tcurl {code}\n")
print((here / f"times-{tail}.txt").read_text())
