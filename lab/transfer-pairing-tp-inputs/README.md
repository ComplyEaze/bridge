# Transfer pairing live test inputs (synthetic; not for merge)

Two synthetic Union Bank-shaped statements for a live test of the bank transfer pairing (#1392) on a synthetic company. Every account, name, reference and amount is invented.

- `tlab-bank-a.pdf`: statement A (`TLab Bank A`), five transfers out to `TLab Bank B`.
- `tlab-bank-b.pdf`: statement B (`TLab Bank B`), the same transfers in, a second same-day clearing credit, and one transfer in from `TLAB DIRECT`.
- `ROWS.md`: every row as this build's parser reads it (date, amount, reference, `st-` id).
- `make_tp.py`: how the two files were made, from `scripts/generate-bank-statement-fixtures.py`: `python3 -I make_tp.py <repository root> <output folder>`.
- `SHA256.txt`: the hashes to check before use.

Both PDFs open with an empty password.
