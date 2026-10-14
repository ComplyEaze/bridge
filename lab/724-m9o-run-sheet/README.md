# Run sheet and requests for one Windows lab item (#724), 14 Oct 2026

Not for merge. A synthetic lab company only; no ComplyEaze Bridge build is used.

- `WINDOWS-724-M9O.md`: under Manual numbering with duplicates prevented, how does Tally decide that an imported voucher's number is already held: across voucher types, by letter case, with a leading zero, and for a back-dated voucher? Its own rules are written out in full; it names the earlier items only for their request files and for what their runs showed.
- `requests/`: its nine request files, ready to send, byte for byte (UTF-16LE with a BOM, no line break at the end). The three reads are the files of the item before it, unchanged: this item runs on the same book.
- The shared rules it carries are `LAB-COMMON.md` on branch `lab/626-1038-1050-run-sheets` at 8c602c15.

The item gives the size and sha256 each request must have, and `SHA256.txt` lists every file here. Check both before a send. A file that writes is sent once.
