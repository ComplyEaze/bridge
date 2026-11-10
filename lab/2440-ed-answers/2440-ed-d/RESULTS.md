# Row 2440-ED-D: s1 and s2 of row 2440-ED added as captured, their one GSTIN value being a synthetic family member already in master's public tree; the derive script refused, as recorded

Part of #2162 (#2440, the e-invoice read-back). File work only: no request was sent, Tally was not started and no book was opened. The sheet was `lab/2440-ed-derive-sheet` (a3058adb), its three files equal to its `SHA256.txt`.

## What this commit adds

- `answers/a-s1-E1-einvoice-day.xml` and `answers/a-s2-E1-einvoice-day.xml`: row 2440-ED's s1 and s2, **captured, byte for byte** as Tally answered them on 10 Nov 2026 (127,966 B each, byte-equal, sha256 `f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0`, the size and hash row 2440-ED's `kept-local.txt` recorded). Their rows are added to `SHA256.txt`.
- This file.

Nothing else on the branch changes. Row 2440-ED's `kept-local.txt` and `RESULTS.md` are left as pushed; their lines saying s1 and s2 stay on the lab PC are superseded by this commit for those two files only.

## Why they may go as captured

Each holds 2 GSTIN-shaped values, 1 distinct. Read on the lab PC after the stop below, printing results only:
- the value is byte-equal to the synthetic family GSTIN for state 29 that the run lead named;
- that value already appears in master's public tree: `git grep -c -F` at origin/master `dfba177cf35f2a7a71d29937fb28d625e7e8c63a` gives files=6, lines=16;
- s2's values are the same as s1's.

So it is synthetic by construction. Row 2440-ED's stage kept s1 and s2 local because its allow-list admitted only two other family GSTINs (states 08 and 27). On the run lead's word of 11 Nov, this commit's stage check admits this one value, for this commit only, and applies every other keep-local rule of row 2440-ED's stage unchanged: the second PAN part, each book's company number, the start-up company's number, the gate word, any other GSTIN shape, and digit runs of 10 or more outside a VOUCHERKEY value, a UUID or a hash. Both answers pass. The check script stays on the lab PC (it names lab paths): `check-2440edd.py`, 9f1ec4355313d2c86f5603ec361a4bcb6f24aa7296e0ce8d50da9b2db4e0e80f.

A captured answer is better evidence for #2440's fixture than a derived one, so no derived copy was made.

## Deviations

- **The derive script refused, and wrote nothing.** The sheet's one command was run first, at 2026-11-10T20:41:56.430Z to 20:41:56.602Z, exit 1:

  ```
  python -I derive-gstin.py <row 2440-ED folder>/a-s1-E1-einvoice-day.xml f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0 2 a-s1-E1-einvoice-day.derived.xml derive-report.txt
  ```

  Its whole output:

  ```
  REFUSED: the input holds 0 GSTIN-shaped values outside the family, not 2
  ```

  That was the sheet's stop. `derive-gstin.py` as run: sha256 `221455066068a8a3b339adf09c56ce0c2b688f5004aaec4fc97579a3d11c114a`, equal to the sheet's `SHA256.txt`. The script skips values already in the family, and both of s1's are, so it found none to replace. No `derived/` folder is pushed. The run lead then chose to add s1 and s2 as captured, on the condition above, which was checked first.
- **The input's path.** The sheet names `<row 2440-ED folder>/answers/a-s1-E1-einvoice-day.xml`; on the lab PC s1 sits in row 2440-ED's folder itself. The script was given that path, on the run lead's word, and checked the file's sha256 itself before anything else.

## What this does not say

Whether #2440's parse accepts s1; the reads themselves are row 2440-ED's, described in its `RESULTS.md`.
