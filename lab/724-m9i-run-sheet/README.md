# Run sheet and requests for one Windows lab item (#724), 13 Oct 2026

Not for merge. A synthetic lab company only; no ComplyEaze Bridge build is used.

- `WINDOWS-724-M9I.md`: does a voucher that arrives by import renumber the vouchers already in the book when the type is on "Renumber Vouchers"?
- `requests/`: its six request files, ready to send, byte for byte (UTF-16LE with a BOM, no line break at the end). The three reads are the files of the 13 Oct run on branch `lab/724-m8r-renumber-answers`.
- The shared rules it carries are `LAB-COMMON.md` on branch `lab/626-1038-1050-run-sheets` at 8c602c15.

The item gives the size and sha256 each request must have, and `SHA256.txt` lists every file here. Check both before a send. A file that writes is sent once.
