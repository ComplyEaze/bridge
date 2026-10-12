# W21b: one post-dated Sales voucher and one deletion, each read with the status request (for #1655's follow-up)

Synthetic company `BRIDGE PILOT LAB` only. It WRITES: one voucher keyed on screen, then that voucher deleted, then the book restored from a backup. Nothing else is keyed, altered, accepted or cancelled. Never use Accept As Is. Change no company feature: look at the Edit Log setting only, and do not turn it on or off. No other company may be loaded.

Every read uses `w21-status-window-aug-postdated.xml` (from W21; SHA-256 `68aaa000…` in `SHA256.txt`), and `../r02-marks.xml` for the marks.

**General STOP rule.** Stop and tell us before going on if: Tally shows any dialog, message or warning that this list does not name; an answer is not a complete envelope with a `COLLECTION` or carries a `LINEERROR`; an answer is larger than 500 KB or takes longer than one minute (do not resend it; check Tally still answers `../r02-marks.xml`).

**How to send.** As before: one request at a time, `curl.exe` with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and printed line for every request.

## The sitting, in this order
1. Take a backup of `BRIDGE PILOT LAB` (`W21B`). Record its path and the SHA-256 of the backup file.
2. `../r02-marks.xml` -> `aw21b-marks-0.xml`
3. `w21-status-window-aug-postdated.xml` -> `aw21b-a.xml` (the control, answered in W21 as `aw21-b.xml`).
4. Key one `BRIDGE Sales` invoice dated 25 August 2026, the same shape as `BP/26-27/0001` (the same party, ledgers and amounts), numbered `BP/26-27/0045`, and mark it post-dated in the voucher screen before saving (Ctrl+T, or the post-dated option the screen offers). If the screen offers no post-dated option, do not save it: STOP and say what the screen shows. Note any dialog on saving.
5. `w21-status-window-aug-postdated.xml` -> `aw21b-b.xml`
6. Look only: does the company have the Edit Log on? (Say where you looked and what it shows; change nothing.)
7. Delete `BP/26-27/0045` (Alt+D in the voucher, confirm). Note any dialog.
8. `w21-status-window-aug-postdated.xml` -> `aw21b-c.xml`
9. `../r02-marks.xml` -> `aw21b-marks-1.xml`
10. Restore `BRIDGE PILOT LAB` from the `W21B` backup, load it alone, then `../r02-marks.xml` -> `aw21b-marks-2.xml`.

## What to post back
The six answers, their hashes added to `SHA256.txt`, the backup's path and SHA-256, for each request curl's exit code and printed line, the Edit Log finding of step 6, every dialog seen and its exact text, and whether you did anything that differs from this list.

---

*For the reader (not for the lab):* W21 measured the request with `ISPOSTDATED` and saw `ISDELETED` and `ISDELETEDVCHRETAINED` come back unasked, all No, on a book with no post-dated or deleted voucher. This set gives one Yes case for each, if Tally makes one. The book is restored at the end, so later reads compare with W20 and W21.

Pre-registered expectations (written before any answer):
- (a) `aw21b-a.xml` is byte-identical to W21's `aw21-b.xml`.
- (b) `aw21b-b.xml` carries the 32 vouchers of (a), unchanged, and `BP/26-27/0045` with `ISPOSTDATED` Yes once. Its three status flags, `ISCANCELLED`, `ISOPTIONAL`, `ISDELETED` and `ISDELETEDVCHRETAINED` are recorded, not predicted.
- (c) `aw21b-c.xml`: with the Edit Log off, `BP/26-27/0045` is absent and the other 32 read as in (a). With it on, the voucher's presence and its `ISDELETED` and `ISDELETEDVCHRETAINED` are recorded, not predicted.
- (d) `aw21b-marks-2.xml` equals `aw21b-marks-0.xml`.

The opposite of any of these is a finding, not an error.
