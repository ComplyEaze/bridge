# Run sheet and requests for one Windows lab item (#724), 14 Oct 2026

Not for merge. A synthetic lab company only; no ComplyEaze Bridge build is used.

- `WINDOWS-724-M9Q.md`: can a raw master import give a Journal voucher type numbered Manual with duplicate numbers prevented, and is a number held in one Journal type matched in its parent, its child and its sibling? Its own rules are written out in full; it names the earlier items only for their request files and for what their runs showed.
- `requests/`: its request files, ready to send, byte for byte (UTF-16LE with a BOM, no line break at the end). Three are master imports of new voucher types; two of the Journal files come in two forms, one for each type that may turn out to be the child, and only one form of each is sent. The three reads are the files of the items before it, unchanged.
- The shared rules it carries are `LAB-COMMON.md` on branch `lab/626-1038-1050-run-sheets` at 8c602c15.

The item gives the size and sha256 each request must have, and `SHA256.txt` lists every file here. Check both before a send. A file that writes is sent once.
