# Run sheet and requests for one Windows lab item (#697), 15 Oct 2026

Not for merge. A synthetic lab company only; no ComplyEaze Bridge build is used, and nothing is written to Tally.

- `WINDOWS-697-S0A.md`: whether the seven request shapes of the audit read are served by a real Tally, and what the answers hold. Its rules are written out in full.
- `requests/`: the eight request files, ready to send, byte for byte (UTF-16LE with a BOM). Two are the company-list and marks reads of earlier items, unchanged; six were rendered by the audit read's own functions.

The item gives the size each request must have, and `SHA256.txt` lists every file here with its hash. Check both before a send.
