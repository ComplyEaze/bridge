# #1459 sitting S3c: deviation from the S3 run sheet (6 Nov 2026)

Not for merge. S3c runs the sheet `lab/1459-s3-run-sheet/README.md` at f80fa78a (with its `LAB-COMMON.md` and `SHA256.txt`) exactly as written, with the three changes below and no other. Nothing here has been run.

## The three changes

1. **ROOT.** Every place the sheet names `C:\bridge-lab\s3-1459` (the working folder, `ka.txt`, `run\`, `evidence\`, `bundle\`, the probe's `-Root`, each `.cmd` and `.exit` path, O2's bundle path) reads `C:\bridge-lab\s3b-1459\s3c` instead. It is a new folder; the earlier sittings' folders are not written.
2. **Nonces.** 0c's N1 and N2 are replaced by fresh nonces drawn from a cryptographic source: N5 = `34e7f032` (K1ac, in) and N6 = `d9cb26f9` (K1bc, out). Neither equals an earlier nonce of #1459 S3 or S3b (`341c7fc2`, `afc70663`, `db4823cc`, `6886521a`).
3. **Step labels** carry a `c` suffix, so no earlier evidence file or probe output is overwritten: 0ac, 0bc, K1ac, K1bc, K1cc, 0dc, 0d2c, 0ec, S3-1459-1c to S3-1459-4c, E2c (and E2bc if the sheet's repeat is needed). Evidence names follow: `evidence\0ac-step0.txt`, `evidence\K1ac-in.json`, `run\K1bc-out.cmd`, `evidence\K1bc-out.exit` and so on.

## Unchanged

Everything else: the build, THE COMPANY, the voucher, the owner's acts O1 to O5, every EXPECT and STOP, the rules of LAB-COMMON v3, and the evidence and pre-push rules. If the lab PC blocks or removes a file or a command, that is recorded with its time, the step stops and is reported; no other name, folder, file type or launcher is tried.

## Scope of this sitting

0ac, 0bc, 0c (K1ac, K1bc, K1cc), 0dc and 0d2c; then hold at O1 (or at 0ec if 0dc reads 0.7.0) for the owner.
