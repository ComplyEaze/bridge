#!/usr/bin/env python3
# timed.py URL LABEL IMPORT READ [--pause]: sends IMPORT once, then READ at once (no wait), at +5 s and at +60 s,
# each counted from the moment curl returned the import's answer. Row 2162-S2's run.py (lab/2162-s2-sheet at
# a0d4c53b) for row 2346-N134, with row 2346-MN's run.py send and stops (lab/2346-mn-sheet at cec5d5a8): the header
# Content-Type: text/xml; charset=utf-16, and a stop at the first curl failure, empty answer, answer above BOUND
# bytes, or answer that says Unknown Request. A stop on the import ends the run: the reads are not sent.
# Answers: a-<LABEL>-<IMPORT> and a-<LABEL>t00-<READ>, a-<LABEL>t05-<READ>, a-<LABEL>t60-<READ>, so count22.py reads
# each by its kind. Every send and return time goes to times-<LABEL>.txt, and each send to log-<LABEL>.txt.
# --pause: after the import's answer, wait for Enter (the window's screenshot is taken then), and count the three
# reads from the Enter instead; ROW.md step 4 says when.
# Example: python3 -I timed.py http://localhost:9001 s4 I-NC1.xml R-MC1.xml
import pathlib, subprocess, sys, time
BOUND = 2_000_000
url, label, request, read = sys.argv[1:5]
pause = sys.argv[5:] == ["--pause"]
if sys.argv[5:] not in ([], ["--pause"]):
    raise SystemExit("usage: timed.py URL LABEL IMPORT READ [--pause]")
here = pathlib.Path(__file__).resolve().parent
sent = []
def texts(raw):
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    return (body.decode("utf-16-le", "replace"), raw.decode("utf-8", "replace"))
def send(name, answer_label):
    answer = here / f"a-{answer_label}-{name}"
    start = time.monotonic()
    done = subprocess.run(["curl.exe", "-sS", "--fail-with-body", "-H", "Content-Type: text/xml; charset=utf-16",
        "--data-binary", f"@{here / name}", "-o", str(answer), url])
    end = time.monotonic()
    size = answer.stat().st_size if answer.exists() else 0
    sent.append((name, answer.name, start, end, done.returncode, size))
    with open(here / f"log-{label}.txt", "a") as log:
        log.write(f"{time.strftime('%H:%M:%S')}\t{name}\t{answer.name}\tcurl {done.returncode}\t{size} bytes\n")
    if done.returncode != 0:
        stop(f"curl exit {done.returncode} on {name}: stop, do not resend")
    if size == 0:
        stop(f"empty answer on {name}: the request failed; stop, do not resend")
    if size > BOUND:
        stop(f"{name}: {size} bytes, above the {BOUND}-byte bound: stop; the answer is kept, not used")
    if any("Unknown Request" in t for t in texts(answer.read_bytes())):
        stop(f"Unknown Request on {name}: stop, do not resend; the answer is kept in {answer.name}")
    print(f"{answer.name}: {size} bytes")
    return end
def write_times():
    t0 = clock if clock is not None else (sent[0][3] if sent else 0)
    with open(here / f"times-{label}.txt", "w") as out:
        for name, answer, start, end, code, size in sent:
            out.write(f"{name}\t{answer}\tsent +{start - t0:.3f}s\treturned +{end - t0:.3f}s\tcurl {code}\t{size} bytes\n")
    print((here / f"times-{label}.txt").read_text())
def stop(message):
    write_times()
    raise SystemExit(message)
clock = None
clock = send(request, label)
if pause:
    input("The import answered. Take the window's screenshot, then press Enter to start the three reads: ")
    clock = time.monotonic()
for tag, at in (("t00", 0), ("t05", 5), ("t60", 60)):
    wait = clock + at - time.monotonic()
    if wait > 0:
        time.sleep(wait)
    send(read, label + tag)
write_times()
