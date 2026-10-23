#!/usr/bin/env python3
"""Lab kit (not product code): stdin files for the approval dialog's child mode from REAL previews.
Run: python3 -I make-real-inputs.py OUTDIR  (OUTDIR new or empty).
`previews/*.txt` are the exact previews the product builds (dumped by a temporary test from the repository's own test
helpers, synthetic data only: 'Synthetic Accounts', 'Party N', zero GUIDs). This writes, for each, the stdin form the
parent writes: `<nonce>\\n<count>\\n<answer text wrapped at 100>\\n\\n<preview>` (approved_import.rs displayed_text), for
the Windows wording and the Mac wording. No final newline."""
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
CASES = [("real-1", 1, "real-one-voucher-24-lines.txt"), ("real-10-39", 10, "real-batch-10-39-lines.txt"), ("real-10-40", 10, "real-batch-10-40-lines.txt")]
def main():
    if len(sys.argv) != 2:
        sys.exit("usage: make-real-inputs.py OUTDIR")
    out = sys.argv[1]
    if os.path.exists(out) and os.listdir(out):
        sys.exit("refused: OUTDIR must be new or empty")
    os.makedirs(out, exist_ok=True)
    here = os.path.dirname(os.path.abspath(__file__))
    for platform in ("win", "mac"):
        for name, count, fname in CASES:
            preview = open(os.path.join(here, "previews", fname), encoding="utf-8", newline="").read()
            shown = wrapped(answer(platform, count)) + "\n\n" + preview
            with open(os.path.join(out, f"{platform}-{name}.txt"), "w", newline="\n", encoding="utf-8") as f:
                f.write(f"{NONCE}\n{count}\n{shown}")
            print(f"{platform}-{name}", "shown lines", shown.count("\n") + 1, "widest", max(len(l) for l in shown.split("\n")))
if __name__ == "__main__":
    main()
