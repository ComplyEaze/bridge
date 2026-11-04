#!/usr/bin/env python3
# run.py URL LABEL FILE...: sends each FILE once, in the order given, one at a time, by curl.exe with the header
# every answered sheet since S1 used (Content-Type: text/xml; charset=utf-16), each answer written by curl -o to
# a-<LABEL>-<FILE>. It stops at the first curl failure, at the first answer that says Unknown Request (HTTP 200,
# curl exit 0, but Tally did not run the request): that is a failure, never a recorded result, at an empty answer,
# and at an answer above BOUND bytes (LAB-COMMON C3's default read bound), which is kept and not used.
# Each answer is decoded by the request's charset, UTF-16 little-endian, whether or not it starts with a byte-order
# mark (Tally's answers carry none), and also as UTF-8, so the check finds the words in either.
# S5's run.py (lab/2162-s5-sheet), with the empty-answer and size-bound stops added.
# Example: python3 -I run.py http://localhost:9000 s1 M-marks.xml
import pathlib, subprocess, sys, time
BOUND = 2_000_000
url, label, files = sys.argv[1], sys.argv[2], sys.argv[3:]
here = pathlib.Path(__file__).resolve().parent
def texts(raw):
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    return (body.decode("utf-16-le", "replace"), raw.decode("utf-8", "replace"))
with open(here / f"log-{label}.txt", "a") as log:
    for name in files:
        answer = here / f"a-{label}-{name}"
        start = time.strftime("%H:%M:%S")
        done = subprocess.run(["curl.exe", "-sS", "--fail-with-body", "-H", "Content-Type: text/xml; charset=utf-16",
            "--data-binary", f"@{here / name}", "-o", str(answer), url])
        size = answer.stat().st_size if answer.exists() else 0
        log.write(f"{start}\t{name}\t{answer.name}\tcurl {done.returncode}\t{size} bytes\n")
        if done.returncode != 0:
            raise SystemExit(f"curl exit {done.returncode} on {name}: stop, do not resend")
        if size == 0:
            raise SystemExit(f"empty answer on {name}: the request failed; stop, do not resend")
        if size > BOUND:
            raise SystemExit(f"{name}: {size} bytes, above the {BOUND}-byte bound: stop; the answer is kept, not used")
        if any("Unknown Request" in t for t in texts(answer.read_bytes())):
            raise SystemExit(f"Unknown Request on {name}: stop, do not resend; the answer is kept in {answer.name}")
        print(f"{name}: {size} bytes")
