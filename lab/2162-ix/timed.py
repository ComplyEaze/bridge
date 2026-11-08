#!/usr/bin/env python3
# timed.py URL LABEL FILE...: run.py of row 2162-S5 (lab/2162-s5-sheet) with each send timed. It sends each FILE once,
# in the order given, one at a time, by curl.exe with the header every answered sheet since S1 used (Content-Type:
# text/xml; charset=utf-16), each answer written by curl -o to a-<LABEL>-<FILE>.
# It never gives up on a send: curl is given no time limit, because a read abandoned by its client keeps Tally busy
# for every company. It refuses an answer file that exists already, and writes <answer>.pending before each send and
# removes it only after that send's log line, so a send that never ended shows as pending, never as an answer.
# Each line of log-<LABEL>.txt: UTC start, UTC end, file, answer, curl exit, HTTP code, curl's seconds to the first
# byte and in all, and the bytes received.
# It stops at the first curl failure, at an HTTP code other than 200, and at an answer that says Unknown Request
# (HTTP 200, curl exit 0, but Tally did not run the request): each is a failure, never a recorded result.
# Each answer is decoded as UTF-16 little-endian, with or without a byte-order mark, and also as UTF-8.
# Example: python3 -I timed.py http://localhost:9001 s1 M-marks.xml
import datetime, pathlib, subprocess, sys
url, label, files = sys.argv[1], sys.argv[2], sys.argv[3:]
here = pathlib.Path(__file__).resolve().parent
def now():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%S.%fZ")[:-4] + "Z"
def texts(raw):
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    return (body.decode("utf-16-le", "replace"), raw.decode("utf-8", "replace"))
for name in files:
    answer = here / f"a-{label}-{name}"
    pending = here / f"a-{label}-{name}.pending"
    if answer.exists() or pending.exists():
        raise SystemExit(f"{answer.name} exists already: stop, do not resend")
    pending.write_text(f"{now()} sent {name}\n")
    start = now()
    done = subprocess.run(["curl.exe", "-sS", "--fail-with-body", "-H", "Content-Type: text/xml; charset=utf-16",
        "--data-binary", f"@{here / name}", "-o", str(answer),
        "-w", "%{http_code} %{time_starttransfer} %{time_total} %{size_download}", url],
        stdout=subprocess.PIPE, text=True)
    end = now()
    timing = (done.stdout or "").strip() or "- - - -"
    line = f"{start}\t{end}\t{name}\t{answer.name}\tcurl {done.returncode}\t{timing}"
    with open(here / f"log-{label}.txt", "a") as log:
        log.write(line + "\n")
    pending.unlink()
    print(line)
    if done.returncode != 0:
        raise SystemExit(f"curl exit {done.returncode} on {name}: stop, do not resend")
    if timing.split()[0] != "200":
        raise SystemExit(f"HTTP {timing.split()[0]} on {name}: stop, do not resend")
    if any("Unknown Request" in t for t in texts(answer.read_bytes())):
        raise SystemExit(f"Unknown Request on {name}: stop, do not resend; the answer is kept in {answer.name}")
