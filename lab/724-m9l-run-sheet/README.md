# Run sheet and requests for one Windows lab item (#724), 13 Oct 2026

Not for merge. A synthetic lab company only; no ComplyEaze Bridge build is used.

- `WINDOWS-724-M9L.md`: what does Tally do with an imported voucher that carries a number, under Manual numbering with duplicates allowed and under "Automatic (Manual Override)", when the number is new, when another voucher already holds it, and when the voucher is back-dated? Its own rules are written out in full; it names the earlier items only for their request files and for what their runs showed.
- `requests/`: its nine request files, ready to send, byte for byte (UTF-16LE with a BOM, no line break at the end). The three reads are the files of the earlier runs on this book, unchanged.
- The shared rules it carries are `LAB-COMMON.md` on branch `lab/626-1038-1050-run-sheets` at 8c602c15.

The item gives the size and sha256 each request must have, and `SHA256.txt` lists every file here. Check both before a send. A file that writes is sent once.
