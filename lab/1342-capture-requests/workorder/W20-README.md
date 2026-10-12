# W20: the GSTR-1 status read with the cancelled and optional flags only (for #1655)

Synthetic company `BRIDGE PILOT LAB` only. READS ONLY: nothing is keyed, altered, accepted or deleted, no voucher or master is opened for alteration, and no screen is changed. No backup is needed (say so if you take one). Never use Accept As Is. It uses the vouchers W19 left: `BP/26-27/0041` (cancelled) and `BP/26-27/0042` (optional).

`w20-status-window-aug-cancel-optional.xml` is the request the code renders for the GSTR-1 status of this company, the window 1 to 31 August 2026 and the type `BRIDGE Sales` (that render is `w13-status-window-aug.xml` byte for byte, SHA-256 `f363e3e7…`), with ONE change: `,ISCANCELLED,ISOPTIONAL` added at the end of its field list. Check each file against `SHA256.txt` first.

**General STOP rule.** Stop and tell us before going on if: Tally shows any dialog, message or warning; an answer is not a complete envelope with a `COLLECTION` or carries a `LINEERROR`; an answer is larger than 500 KB or takes longer than one minute (do not resend it; check Tally still answers `../r02-marks.xml`).

**How to send.** As before: one request at a time, the same `curl.exe` line with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and printed line for every request.

## The sitting, in this order
1. `../r02-marks.xml` -> `aw20-marks-0.xml`
2. `w13-status-window-aug.xml` -> `aw20-a.xml` (the control, answered in W13 and W19).
3. `w20-status-window-aug-cancel-optional.xml` -> `aw20-b.xml`
4. `../r02-marks.xml` -> `aw20-marks-1.xml`

## What to post back
The four answers, their hashes added to `SHA256.txt`, and for each request curl's exit code and printed line. Say whether you did anything that differs from this list.

---

*For the reader (not for the lab):* `gst_status` reads only the three GSTR-1 flags, and W19 showed that a cancelled Sales voucher reads included and an optional one reads none of the three (#1655). The fix adds these two field names to its request. W19 measured the request with them and two more names. This set measures the exact request the fix will send.

Pre-registered expectations (written before any answer): (a) the two marks answers are equal; (b) `aw20-a.xml` carries the same vouchers and flags as W19's `aw19-e-1.xml`; (c) `aw20-b.xml` carries the same vouchers as `aw20-a.xml`, each with `ISCANCELLED` and `ISOPTIONAL` once: Yes and No on `0041`, No and Yes on `0042`, No and No on every other voucher; (d) the three status flags, `ISGSTOVERRIDDEN` and `ALTERID` of each voucher are the same in `aw20-a.xml` and `aw20-b.xml`. The opposite of any of these is a finding, not an error.
