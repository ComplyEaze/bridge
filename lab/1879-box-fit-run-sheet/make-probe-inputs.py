#!/usr/bin/env python3
"""Lab kit (not product code): stdin files for the approval dialog's child mode with N displayed lines, Windows wording,
to see what the native message box does when its text is taller than the screen. Run: python3 -I make-probe-inputs.py OUTDIR
(OUTDIR new or empty). Each file is `<nonce>\\n<count>\\n<displayed text>`, no final newline: the answer text (wrapped at 100,
as the parent writes it), a blank line, then N-3 synthetic lines. Line k is `P<k> ` + filler, 100 characters on every third line
and 52 on the others (60 and 40 for the 120-line file, to stay under the 8,000-byte limit), so the widest line and the line count are both exercised. Probe sizes: 29, 33, 40, 44, 48, 60, 80, 120 displayed lines (29 and 33: the height of a ten-payment batch shown as totals only, #1879 follow-up)."""
import os, sys
NONCE = "0f9d3c1e-6b7a-4c58-9a21-5d3e8b1c7f40"
WIDTH = 100
def wrapped(text):
    out, width = "", 0
    for word in text.split(" "):
        n = len(word)
        if width == 0:
            out += word; width = n
        elif width + 1 + n <= WIDTH:
            out += " " + word; width += 1 + n
        else:
            out += "\n" + word; width = n
    return out
ANSWER = wrapped("Press Y (or click Yes) to post this voucher into Tally. Press N (or click No) to cancel. Nothing is posted until you answer.")
def build(total):
    head = ANSWER + "\n\n"
    answer_lines = ANSWER.count("\n") + 1
    preview_lines = total - answer_lines - 1
    body = []
    for k in range(preview_lines):
        wide, narrow = (100, 52) if total <= 80 else (60, 40)
        width = wide if k % 3 == 0 else narrow
        text = f"P{k+1:03d} probe line " 
        body.append((text + "x" * width)[:width])
    shown = head + "\n".join(body)
    assert shown.count("\n") + 1 == total, (shown.count("\n") + 1, total)
    assert len(shown.encode()) <= 7900, len(shown.encode())
    return f"{NONCE}\n1\n{shown}"
def main():
    if len(sys.argv) != 2:
        sys.exit("usage: make-probe-inputs.py OUTDIR")
    out = sys.argv[1]
    if os.path.exists(out) and os.listdir(out):
        sys.exit("refused: OUTDIR must be new or empty")
    os.makedirs(out, exist_ok=True)
    for total in (29, 33, 40, 44, 48, 60, 80, 120):
        with open(os.path.join(out, f"probe-{total}.txt"), "w", newline="\n", encoding="utf-8") as f:
            f.write(build(total))
        print("probe-%d" % total)
if __name__ == "__main__":
    main()
