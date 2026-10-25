# Run sheet for one Windows lab item (#1832 box clipping and DPI awareness, W1 and W2), 25 Oct 2026

Not for merge. Synthetic text only; no Tally, no key, no display change.

- `WINDOWS-1832-CLIP.md`: the sheet.
- `dump-manifest.ps1` (W1): prints the manifest(s) embedded in `bridge_mcp.exe`. `run-probe.ps1` (W2): opens the real approval box with N lines and prints its rectangle against the work area. Both were written without a PowerShell to test them on.
- `inputs/`: six stdin files (40 to 120 displayed lines). `make-probe-inputs.py` rebuilds them byte for byte.

`SHA256.txt` lists every file here. Check it before the item starts.
