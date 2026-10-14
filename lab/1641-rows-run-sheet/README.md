# Run sheet for one Windows lab item (#1641), 15 Oct 2026

Not for merge. A synthetic lab company only; Tally is only read, and nothing is written to it.

- `WINDOWS-1641-ROWS.md`: whether the Tally port locks hold between the packaged extension and the desktop app now that each lock is also taken under the user's profile folder. Its rules, stop conditions and restore steps are written out in full.
- `hold-lock.ps1`: takes a Windows file lock the way Bridge does and reports whether it was granted; run once outside the package and once inside it.
- `requests/`: two read-only request files of an earlier item, unchanged (company list; voucher marks).

`SHA256.txt` lists every file here. Check it before the item starts.
