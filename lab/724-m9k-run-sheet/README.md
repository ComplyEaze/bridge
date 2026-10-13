# Run sheet and requests for one Windows lab item (#724), 13 Oct 2026

Not for merge. A synthetic lab company only; no ComplyEaze Bridge build is used.

- `WINDOWS-724-M9K.md`: with a voucher type on "Automatic (Manual Override)", does an imported voucher keep the number it carries? And does a back-dated import renumber later-dated Journals under "Renumber Vouchers"? It is a delta on `WINDOWS-724-M9I.md` (branch `lab/724-m9i-run-sheet` at 668cd252), which applies with it.
- `requests/`: its six request files, ready to send, byte for byte (UTF-16LE with a BOM, no line break at the end). The three reads are the files of the M9I run, unchanged.
- The shared rules it carries are `LAB-COMMON.md` on branch `lab/626-1038-1050-run-sheets` at 8c602c15.

The item gives the size and sha256 each request must have, and `SHA256.txt` lists every file here. Check both before a send. A file that writes is sent once.
