#!/usr/bin/env python3
# send.py LABEL FILE...: LAB-COMMON C3 for each FILE, one at a time, in the order given, to http://127.0.0.1:9001,
# with C3's own curl.exe line (-sS, -w "%{http_code} %{size_download}", --data-binary, the utf-16 header, -D, -o).
# Modelled on lab/2162-bc's run.py, with C3's checks in place of its curl line:
# - before each send, the file's sha256 and bytes must equal its line in SHA256.txt beside this script, else nothing
#   is sent;
# - an answer or headers file that already exists stops it: a request is never sent twice;
# - after each send it stops at a curl exit other than 0, a status other than 200, an absent CONTENT-LENGTH, a size
#   that differs from CONTENT-LENGTH or from the file on disk, an empty answer, an answer above BOUND bytes (kept,
#   not used), and an answer that says Unknown Request (Tally did not run the request). Each is "the request
#   failed", never "nothing found".
# Answers go to answers\<LABEL>-<FILE>, headers to answers\<LABEL>-<FILE>.headers.txt; one line per send is
# appended to log-send.txt (UTC). No client timeout: a send without an answer is left running, never repeated.
# Example: python -I send.py t0 M-marks-shape-lab.utf16le.xml
import hashlib, pathlib, subprocess, sys, time
BOUND = 2_000_000
URL = "http://127.0.0.1:9001"
REQUESTS = ("C-company-list.utf16le.xml", "M-marks-shape-lab.utf16le.xml", "M-marks-spaced.utf16le.xml")
here = pathlib.Path(__file__).resolve().parent
label, files = sys.argv[1], sys.argv[2:]
if not files or not label.replace("-", "").isalnum():
    raise SystemExit("usage: send.py LABEL FILE...")
listed = {}
for line in (here / "SHA256.txt").read_text(encoding="utf-8").splitlines():
    if line and not line.startswith("#"):
        digest, size, name = line.split("  ", 2)
        listed[name] = (digest, int(size))
(here / "answers").mkdir(exist_ok=True)


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


# Every file named is checked before the first is sent: one that is not a request of this sheet, or whose bytes are
# not those listed, stops the call with nothing sent.
for name in files:
    if name not in REQUESTS or name not in listed:
        raise SystemExit(f"{name} is not a request this sheet sends: nothing sent")
    if not (here / name).is_file() or (sha(here / name), (here / name).stat().st_size) != listed[name]:
        raise SystemExit(f"{name}: sha256 or bytes differ from SHA256.txt: STOP, nothing sent")

for name in files:
    request = here / name
    answer = here / "answers" / f"{label}-{name}"
    headers = here / "answers" / f"{label}-{name}.headers.txt"
    if answer.exists() or headers.exists():
        raise SystemExit(f"{answer.name} already exists: this request is never sent again")
    start = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    done = subprocess.run(["curl.exe", "-sS", "-w", "%{http_code} %{size_download}", "--data-binary", f"@{request}",
                           "-H", "Content-Type: text/xml; charset=utf-16", "-D", str(headers), URL, "-o", str(answer)],
                          capture_output=True, text=True)
    printed = done.stdout.strip().split()
    status, size_download = (printed + ["", ""])[:2]
    lengths = [line.split(":", 1)[1].strip() for line in
               (headers.read_bytes().decode("latin-1").splitlines() if headers.exists() else [])
               if line.lower().startswith("content-length:")]
    content_length = lengths[-1] if lengths else "absent"
    size = answer.stat().st_size if answer.exists() else 0
    line = (f"{start}\t{label}\t{name}\trequest {listed[name][0]}\tcurl {done.returncode}\tstatus {status}\t"
            f"size_download {size_download}\tcontent-length {content_length}\ton disk {size}\t"
            f"answer {sha(answer) if answer.exists() else 'none'}")
    with open(here / "log-send.txt", "a", encoding="utf-8", newline="\n") as log:
        log.write(line + "\n")
    print(line)
    if done.stderr.strip():
        print(f"curl said: {done.stderr.strip()}")
    if done.returncode != 0:
        raise SystemExit(f"curl exit {done.returncode} on {name}: the request failed; STOP, do not resend")
    if status != "200":
        raise SystemExit(f"status {status} on {name}: the request failed; STOP, do not resend")
    if content_length == "absent" or content_length != size_download or size_download != str(size):
        raise SystemExit(f"{name}: sizes disagree (content-length {content_length}, size_download {size_download}, "
                         f"on disk {size}): the request failed; STOP, do not resend")
    if size == 0:
        raise SystemExit(f"empty answer on {name}: the request failed; STOP, do not resend")
    if size > BOUND:
        raise SystemExit(f"{name}: {size} bytes, above the {BOUND}-byte bound: STOP; the answer is kept, not used")
    raw = answer.read_bytes()
    body = raw[2:] if raw[:2] == b"\xff\xfe" else raw
    if any("Unknown Request" in text for text in (body.decode("utf-16-le", "replace"), raw.decode("utf-8", "replace"))):
        raise SystemExit(f"Unknown Request on {name}: the request failed; STOP, do not resend")
