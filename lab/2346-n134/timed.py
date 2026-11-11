#!/usr/bin/env python3
# timed.py URL LABEL IMPORT READ [--pause]: sends IMPORT once, then READ at once (no wait), at +5 s and at +60 s,
# each counted from the moment curl returned the import's answer. Row 2162-S2's run.py (lab/2162-s2-sheet at
# a0d4c53b) for row 2346-N134, with row 2346-MN's run.py send and stops (lab/2346-mn-sheet at cec5d5a8): the header
# Content-Type: text/xml; charset=utf-16, and a stop at the first curl failure, empty answer, answer above BOUND
# bytes, or answer that says Unknown Request. A stop on the import ends the run: the reads are not sent.
# The import's answer is also judged by C4 before any read: every counter present, CREATED 1 and every other
# counter 0, no LINEERROR; else stop (ROW.md step 4: a decline is a result, and nothing more is sent).
# It refuses to start if this label's import answer or log already exists: the import is sent once in the row's life.
# Without --pause, it waits after the +5 s read for Enter, pressed once the window's screenshot is taken and shows no
# dialog, and sends the +60 s read at +60 s or at the Enter, whichever is later. The Enter's time goes to the times.
# The times file is rewritten after every send, so an interrupt (Ctrl+C) keeps every send made before it.
# curl also gets C3's -w "%{http_code} %{size_download}" and -D h-<answer label>-<FILE>.txt, which change no
# byte Tally receives; each send STOPs after the curl exit check unless c3_check passes (as run.py's).
# Answers: a-<LABEL>-<IMPORT> and a-<LABEL>t00-<READ>, a-<LABEL>t05-<READ>, a-<LABEL>t60-<READ>, so count22.py reads
# each by its kind. Every send and return time goes to times-<LABEL>.txt, and each send to log-<LABEL>.txt.
# --pause: after the import's answer, wait for Enter (the window's screenshot is taken then), and count the three
# reads from the Enter instead, with no later wait; ROW.md step 4 says when.
# Example: python3 -I timed.py http://localhost:9001 s4 I-NC1.xml R-MC1.xml
import pathlib, re, subprocess, sys, time
import xml.etree.ElementTree as ET
BOUND = 2_000_000
url, label, request, read = sys.argv[1:5]
pause = sys.argv[5:] == ["--pause"]
if sys.argv[5:] not in ([], ["--pause"]):
    raise SystemExit("usage: timed.py URL LABEL IMPORT READ [--pause]")
here = pathlib.Path(__file__).resolve().parent
for done_before in (here / f"a-{label}-{request}", here / f"log-{label}.txt"):
    if done_before.exists():
        raise SystemExit(f"{done_before.name} exists: this import was sent already. Nothing is sent; use run.py for reads")
sent = []
marks = []
COUNTERS = ("CREATED", "ALTERED", "DELETED", "LASTVCHID", "LASTMID", "COMBINED", "IGNORED", "ERRORS", "CANCELLED",
            "EXCEPTIONS")
def import_clean(raw):
    # C4: decode, drop only the characters XML forbids, parse; every counter once, CREATED 1, the others but the two
    # LAST ids 0, and no LINEERROR. Returns the counters as read, or a reason to stop.
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    text = re.sub(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(0*([0-8]|1[124]|1[4-9]|2[0-9]|3[01]));", "",
                  body.decode("utf-16-le"))
    try:
        root = ET.fromstring(text)
    except ET.ParseError as error:
        return None, f"the import's answer does not parse ({error})"
    read = {}
    for name in COUNTERS:
        found = [e for e in root.iter(name)]
        if len(found) != 1:
            return None, f"the import's answer holds {len(found)} {name} elements, not one"
        read[name] = (found[0].text or "").strip()
    errors = [(e.text or "").strip() for e in root.iter("LINEERROR")]
    shown = " ".join(f"{k} {v!r}" for k, v in read.items()) + "".join(f" LINEERROR {e!r}" for e in errors)
    clean = read["CREATED"] == "1" and not errors and all(
        read[k] == "0" for k in COUNTERS if k not in ("CREATED", "LASTVCHID", "LASTMID"))
    return shown, None if clean else "the import was not created clean"
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
    lengths = [l.split(":", 1)[1].strip() for l in lines[1:] if l.split(":", 1)[0].strip().lower() == "content-length"]
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
def send(name, answer_label):
    answer = here / f"a-{answer_label}-{name}"
    start = time.monotonic()
    headers = here / f"h-{answer_label}-{name}.txt"
    done = subprocess.run(["curl.exe", "-sS", "--fail-with-body", "-w", "%{http_code} %{size_download}",
        "-D", str(headers), "-H", "Content-Type: text/xml; charset=utf-16",
        "--data-binary", f"@{here / name}", "-o", str(answer), url], stdout=subprocess.PIPE, text=True)
    end = time.monotonic()
    size = answer.stat().st_size if answer.exists() else 0
    (code, length), refused = c3_check(done.stdout, headers, size)
    sent.append((name, answer.name, start, end, done.returncode, size, code, length))
    with open(here / f"log-{label}.txt", "a") as log:
        log.write(f"{time.strftime('%H:%M:%S')}\t{name}\t{answer.name}\tcurl {done.returncode}\tstatus {code}\t"
                  f"CONTENT-LENGTH {length}\t{size} bytes\n")
    if done.returncode != 0:
        stop(f"curl exit {done.returncode} on {name}: stop, do not resend")
    if refused:
        stop(f"{refused} on {name}: the request failed (C3); stop, do not resend")
    if size == 0:
        stop(f"empty answer on {name}: the request failed; stop, do not resend")
    if size > BOUND:
        stop(f"{name}: {size} bytes, above the {BOUND}-byte bound: stop; the answer is kept, not used")
    if any("Unknown Request" in t for t in texts(answer.read_bytes())):
        stop(f"Unknown Request on {name}: stop, do not resend; the answer is kept in {answer.name}")
    print(f"{answer.name}: {size} bytes")
    write_times(show=False)
    return end
def write_times(show=True):
    t0 = clock if clock is not None else (sent[0][3] if sent else 0)
    with open(here / f"times-{label}.txt", "w") as out:
        for name, answer, start, end, exit_code, size, status, length in sent:
            out.write(f"{name}\t{answer}\tsent +{start - t0:.3f}s\treturned +{end - t0:.3f}s\tcurl {exit_code}\t"
                      f"status {status}\tCONTENT-LENGTH {length}\t{size} bytes\n")
        for what, at in marks:
            out.write(f"{what}\t+{at - t0:.3f}s\n")
    if show:
        print((here / f"times-{label}.txt").read_text())
def stop(message):
    write_times()
    raise SystemExit(message)
clock = None
try:
    clock = send(request, label)
    counters, refused = import_clean((here / f"a-{label}-{request}").read_bytes())
    print(f"import counters: {counters}")
    if refused:
        stop(f"{refused}: no read is sent. Record the counters and every LINEERROR; ROW.md step 4's decline rule")
    if pause:
        input("The import answered. Take the window's screenshot, then press Enter to start the three reads: ")
        clock = time.monotonic()
        marks.append(("Enter (screenshot taken, before the reads)", clock))
    for tag, at in (("t00", 0), ("t05", 5), ("t60", 60)):
        if tag == "t60" and not pause:
            input("Take the window's screenshot now. Press Enter only if it shows no dialog (else Ctrl+C): ")
            marks.append(("Enter (screenshot taken, no dialog)", time.monotonic()))
        wait = clock + at - time.monotonic()
        if wait > 0:
            time.sleep(wait)
        send(read, label + tag)
except (KeyboardInterrupt, EOFError):
    marks.append(("interrupted (Ctrl+C, or no Enter to read)", time.monotonic()))
    stop("interrupted: nothing more is sent; times-<label>.txt holds every send made")
write_times()
