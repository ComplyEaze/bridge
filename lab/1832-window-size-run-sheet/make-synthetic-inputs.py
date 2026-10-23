#!/usr/bin/env python3
"""Lab kit (not product code): stdin files for the approval dialog's child mode, to open the real native box
at its largest sizes (#1832). Run: python3 -I make-window-inputs.py OUTDIR  (OUTDIR new or empty).

The parent of the dialog writes `<nonce>\\n<count>\\n<displayed text>` to the child's stdin, where the displayed text is the
answer text, a blank line, then the preview (src-tauri/src/tally/approved_import.rs: answer_text, wrapped, displayed_text).
This writes that form for four cases, with synthetic preview lines (layout depends on line count and width, not content):
  win-1.txt   Windows wording, one voucher, preview 24 lines (the cap), shown 27 lines
  win-10.txt  Windows wording, batch of 10, preview 40 lines (the cap), shown 43 lines
  mac-1.txt   Mac wording, one voucher, 24 + 3 lines
  mac-10.txt  Mac wording, batch of 10, 40 + 3 lines
Each file ends with no final newline. The nonce is a fixed synthetic UUID; the child prints a token ONLY if the person
presses the positive button, so the lab answers No / Cancel."""
import os, sys

NONCE = "0f9d3c1e-6b7a-4c58-9a21-5d3e8b1c7f40"
WIDTH = 100

def wrapped(text):
    lines, width = "", 0
    for word in text.split(" "):
        n = len(word)
        if width == 0:
            lines += word; width = n
        elif width + 1 + n <= WIDTH:
            lines += " " + word; width += 1 + n
        else:
            lines += "\n" + word; width = n
    return lines

def answer(platform, count):
    these = "this voucher" if count == 1 else f"these {count} vouchers"
    if platform == "win":
        return f"Press Y (or click Yes) to post {these} into Tally. Press N (or click No) to cancel. Nothing is posted until you answer."
    button = "Post voucher" if count == 1 else f"Post {count} vouchers"
    return f'Click "{button}" to post {these} into Tally. Click Cancel to cancel. Nothing is posted until you answer.'

def preview(lines, chars_cap, every, narrow):
    # alternate 100-character and shorter lines so both the widest line and the line count are exercised
    out = []
    for i in range(lines):
        width = 100 if i % every == 0 else narrow
        body = f"L{i+1:02d} synthetic preview line "
        body += ("x" * (width - len(body)))
        out.append(body[:width])
    text = "\n".join(out)
    assert len(text) <= chars_cap, (len(text), chars_cap)
    return text

def build(platform, count, lines, chars_cap, shown_chars_cap, every, narrow):
    shown = wrapped(answer(platform, count)) + "\n\n" + preview(lines, chars_cap, every, narrow)
    assert len(shown) <= shown_chars_cap, (len(shown), shown_chars_cap)
    return f"{NONCE}\n{count}\n{shown}", shown

def main():
    if len(sys.argv) != 2:
        sys.exit("usage: make-window-inputs.py OUTDIR")
    out = sys.argv[1]
    if os.path.exists(out) and os.listdir(out):
        sys.exit("refused: OUTDIR must be new or empty")
    os.makedirs(out, exist_ok=True)
    for name, platform, count, lines, pc, sc, every, narrow in [
        ("win-1", "win", 1, 24, 1600, 1742, 4, 52), ("win-10", "win", 10, 40, 3200, 3342, 3, 62),
        ("mac-1", "mac", 1, 24, 1600, 1742, 4, 52), ("mac-10", "mac", 10, 40, 3200, 3342, 3, 62)]:
        data, shown = build(platform, count, lines, pc, sc, every, narrow)
        with open(os.path.join(out, name + ".txt"), "w", newline="\n", encoding="utf-8") as f:
            f.write(data)
        print(name, "shown lines", shown.count("\n") + 1, "shown chars", len(shown), "widest", max(len(l) for l in shown.split("\n")))

if __name__ == "__main__":
    main()
