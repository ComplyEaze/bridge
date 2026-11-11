#!/usr/bin/env python3
# run.py URL LABEL FILE...: sends each FILE once, in the order given, one at a time, by curl.exe with the header
# every answered sheet since S1 used (Content-Type: text/xml; charset=utf-16), each answer written by curl -o to
# a-<LABEL>-<FILE>. It stops at the first curl failure, at the first answer that says Unknown Request (HTTP 200,
# curl exit 0, but Tally did not run the request): that is a failure, never a recorded result, at an empty answer,
# and at an answer above BOUND bytes (LAB-COMMON C3's default read bound), which is kept and not used.
# Each answer is decoded by the request's charset, UTF-16 little-endian, whether or not it starts with a byte-order
# mark (Tally's answers carry none), and also as UTF-8, so the check finds the words in either.
# S5's run.py (lab/2162-s5-sheet), with the empty-answer and size-bound stops added.
# N134 (row 2346-MN's run.py, lab/2346-mn-sheet at cec5d5a8, with one change): curl also gets C3's
# -w "%{http_code} %{size_download}" and -D h-<LABEL>-<FILE>.txt, which change no byte Tally receives, and
# each send STOPs after the curl exit check unless c3_check passes. The status and CONTENT-LENGTH go to the log.
# Example: python3 -I run.py http://localhost:9000 s1 M-marks.xml
import pathlib, subprocess, sys, time
BOUND = 2_000_000
url, label, files = sys.argv[1], sys.argv[2], sys.argv[3:]
here = pathlib.Path(__file__).resolve().parent
def c3_check(printed, headers, size):
    # N134: LAB-COMMON C3. curl's -w line is "<status> <bytes>"; -D keeps the headers. STOP unless the status is
    # 200 in both, the last header block holds exactly one CONTENT-LENGTH (any case), and it, the -w byte count and
    # the answer file's size are equal. Returns (status, content length) as read, and a reason to stop or None.
    words = printed.split()
    if len(words) != 2 or not all(w.isdigit() for w in words):
        return (None, None), f"curl's -w line is {printed!r}, not '<status> <bytes>'"
    code, downloaded = int(words[0]), int(words[1])
    if not headers.exists():
        return (code, None), f"no headers file {headers.name}"
    blocks = [b for b in headers.read_bytes().decode("latin-1").replace("\r\n", "\n").split("\n\n") if b.strip()]
    if not blocks:
        return (code, None), f"the headers file {headers.name} is empty"
    lines = blocks[-1].split("\n")
    status = lines[0].split()
    lengths = [l.split(":", 1)[1].strip() if ":" in l else "" for l in lines[1:]
               if l.split(":", 1)[0].strip().lower() == "content-length"]
    length = int(lengths[0]) if len(lengths) == 1 and lengths[0].isdigit() else None
    if code != 200 or len(status) < 2 or status[1] != "200":
        return (code, length), f"status {code} (headers: {lines[0]!r}), not 200"
    if length is None:
        return (code, None), f"the headers hold {len(lengths)} CONTENT-LENGTH lines {lengths!r}, not one number"
    if not length == downloaded == size:
        return (code, length), f"CONTENT-LENGTH {length}, curl read {downloaded}, the file holds {size}: not equal"
    return (code, length), None
def texts(raw):
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    return (body.decode("utf-16-le", "replace"), raw.decode("utf-8", "replace"))
with open(here / f"log-{label}.txt", "a") as log:
    for name in files:
        answer = here / f"a-{label}-{name}"
        start = time.strftime("%H:%M:%S")
        headers = here / f"h-{label}-{name}.txt"
        done = subprocess.run(["curl.exe", "-sS", "--fail-with-body", "-w", "%{http_code} %{size_download}",
            "-D", str(headers), "-H", "Content-Type: text/xml; charset=utf-16",
            "--data-binary", f"@{here / name}", "-o", str(answer), url], stdout=subprocess.PIPE, text=True)
        size = answer.stat().st_size if answer.exists() else 0
        (code, length), refused = c3_check(done.stdout, headers, size)
        log.write(f"{start}\t{name}\t{answer.name}\tcurl {done.returncode}\tstatus {code}\t"
                  f"CONTENT-LENGTH {length}\t{size} bytes\n")
        if done.returncode != 0:
            raise SystemExit(f"curl exit {done.returncode} on {name}: stop, do not resend")
        if refused:
            raise SystemExit(f"{refused} on {name}: the request failed (C3); stop, do not resend")
        if size == 0:
            raise SystemExit(f"empty answer on {name}: the request failed; stop, do not resend")
        if size > BOUND:
            raise SystemExit(f"{name}: {size} bytes, above the {BOUND}-byte bound: stop; the answer is kept, not used")
        if any("Unknown Request" in t for t in texts(answer.read_bytes())):
            raise SystemExit(f"Unknown Request on {name}: stop, do not resend; the answer is kept in {answer.name}")
        print(f"{name}: {size} bytes")
