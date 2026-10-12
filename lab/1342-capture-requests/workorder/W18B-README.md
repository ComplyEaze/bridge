# W18b: the verification-window read that W18 did not take, from the same book state

Synthetic company `BRIDGE PILOT LAB` only. Reads only, on a book restored from a backup; at the end the book is put back as it was. Nothing is keyed, altered, accepted or deleted, and no company feature is changed. No other company may be loaded.

A Sales invoice build sends one read that W18 did not capture: the verification window (`../r15-verification-window.xml`, the window of 1 August 2026). A replay must take every answer from one book state. W19 took the backup `W19-C` before its first write, and W19's first marks equal W18's (`0a2d7f08…`). So restoring `W19-C` gives W18's book, and the marks prove it.

**General STOP rule.** Stop and tell us before going on if: Tally shows any dialog, message or warning that this list does not name; an answer is not a complete envelope with a `COLLECTION` or carries a `LINEERROR`; an answer is larger than 500 KB or takes longer than one minute (do not resend it; check Tally still answers `../r02-marks.xml`); or the marks of step 3 differ from W18's.

**How to send.** As before: one request at a time, `curl.exe` with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and printed line for every request.

## The sitting, in this order
1. Take a backup of `BRIDGE PILOT LAB` as it is now (`W18B-NOW`). Record its path and the SHA-256 of the backup file.
2. Restore `BRIDGE PILOT LAB` from the `W19-C` backup and load it alone. Record that backup's path and SHA-256.
3. `../r02-marks.xml` -> `aw18b-marks-0.xml`. It must be byte-identical to W18's `aw18-marks-0.xml` (`0a2d7f088c55bd6c1d0e827e040b62e88a28f242b7e7388c11259ecce7763a6d`). If not, STOP.
4. `../r15-verification-window.xml` -> `aw18b-verification-window.xml`
5. `../r02-marks.xml` -> `aw18b-marks-1.xml`
6. Restore `BRIDGE PILOT LAB` from `W18B-NOW` and load it alone, then `../r02-marks.xml` -> `aw18b-marks-2.xml`.

## What to post back
The four answers, their hashes added to `SHA256.txt`, both backups' paths and SHA-256, for each request curl's exit code and printed line, every dialog seen and its exact text, and whether you did anything that differs from this list.

---

*For the reader (not for the lab):* #1646's two surviving mutants need a build replayed on `BRIDGE Svc 4D 5%` from one book state. W18 has every read of the build except the verification window. With this answer the replay uses W18's answers and this one, and the provenance names both days, tied by the marks.

Pre-registered expectations (written before any answer):
- (a) `aw18b-marks-0.xml` and `aw18b-marks-1.xml` are byte-identical to `aw18-marks-0.xml`.
- (b) `aw18b-verification-window.xml` carries the vouchers dated 1 August 2026, as the 9 Oct pilot answer of the same request did, apart from what changed on the company between 9 Oct and W18.
- (c) `aw18b-marks-2.xml` equals the marks read just before step 1 by the previous sitting (or, if none, it is recorded).

The opposite of any of these is a finding, not an error.
