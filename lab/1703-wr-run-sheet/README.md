# Run sheet and requests for one Windows lab item (#1703), 14 Oct 2026

Not for merge. A synthetic lab company only; no ComplyEaze Bridge build is used, and nothing is written to Tally.

- `WINDOWS-1703-WR.md`: how long one verification read takes, and how large it is, when a batch's window is widened by 7 days either side, against the same read over one day. Its rules are written out in full.
- `requests/`: its four request files, ready to send, byte for byte (UTF-16LE with a BOM). Two are the company-list and marks reads of an earlier item, unchanged; two are the verification read for one day and for the widened window.

The item gives the size and sha256 each request must have, and `SHA256.txt` lists every file here. Check both before a send.
