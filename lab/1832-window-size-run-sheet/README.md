# Run sheet for one Windows lab item (#1832 box size), 23 Oct 2026

Not for merge. Synthetic text only; no Tally, no posting.

- `WINDOWS-1832-BOX.md`: the sheet (build, settings, cases, pass and fail).
- `run-box.ps1`: opens the real approval box with prepared text and records its window rectangle against the work area. Untested on Windows when written; run `-DryRun` first.
- `inputs/`: ten stdin files for the box's child mode (Windows and Mac wording; real and worst-case synthetic text).
- `previews/`: the three real previews the product builds (one voucher 24 lines; a batch of 10 at 39 and 40 lines).
- `make-real-inputs.py`, `make-synthetic-inputs.py`: regenerate `inputs/` byte for byte.

`SHA256.txt` lists every file here. Check it before the item starts.
