# Run sheet and requests for Windows item 1390-1 (one hand import of two suspense-tagged lines; pull request 1390), 15 Oct 2026

Not for merge. Synthetic company only (BRIDGE IDENTITY LAB). One build: pull request 1390 at head 902c86ff0dda46f1d0b2ff0d12b869cf7f631094, the `.mcpb` route (pinned in the sheet). No native post, no approval dialog, no ledger created.
- `WINDOWS-1390-1.md`: the run sheet in two parts (Part A a person's 30-minute sitting, Part B the operator's unattended run); LAB-COMMON v3 on branch lab/626-1038-1050-run-sheets applies. The larger sheet `sheet-1390` is superseded by this one and its later items.
- `requests/1390-1/`: the request files, UTF-16LE with a BOM, no line break at the end; `statement/`, `hc-narration.txt`, `hu-narration.txt`, `to-utf16.py`, `read-answer.py`, `check-request-ids.py`: the inputs and helpers the sheet names.
- Check every file against `SHA256.txt` before use (PowerShell: `Get-FileHash -Algorithm SHA256 FILE`; macOS: `shasum -a 256 -c SHA256.txt`), then `python -I check-request-ids.py requests`: EXPECT `census: ok 5, skipped 0, FAIL 0`.
- A file that WRITES is sent once; the sheet says which.
