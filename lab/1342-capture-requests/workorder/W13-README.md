# W13: what the GSTR-1 JSON export holds for an uncertain voucher

Synthetic company `BRIDGE PILOT LAB` only. This work writes: it uses Accept As Is on ONE voucher (`BP/26-27/0005`) and exports GSTR-1 for August 2026 twice. Take a fresh backup first and say where it is. Do it after W12 if W12 has been run (the W12 vouchers `0003`, `0006` may then already be accepted; that does not matter, only the one change below does).

**Stop rule.** Stop and tell us, before going on, if: Tally shows a dialog or warning this list does not mention; a request answer is not a complete envelope with a `COLLECTION` element (or carries a `LINEERROR`); the export dialog offers an option this list does not mention (screenshot it and stop before choosing); anything differs from the steps. Use Accept As Is on `BP/26-27/0005` ONLY. Do not alter or delete any other voucher or master.

**How to send.** As before: one request at a time, the same `curl.exe` line with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and the printed line for every request. Check each request file against `SHA256.txt` first. `r02-marks.xml` is from the earlier set.

## Before
1. Send `r02-marks.xml` -> `aw13-marks-0.xml`; `w13-status-window-aug.xml` -> `aw13-status-window-0.xml`; `w13-status-0005.xml` -> `aw13-status-0005-0.xml`.
2. Screens: GSTR-1 for August 2026: the summary (the counts of included and uncertain, and the B2C Small and any B2B/B2CL totals), and the reason Tally gives for `0005` (screenshot).
3. Export GSTR-1 for August 2026 as JSON for upload (Tally's Export, JSON). Screenshot every dialog and every option it shows, and say which options you chose (choose none that this list does not name; if the dialog offers to include or leave out uncertain transactions, do the first export with Tally's default and say what the default was). Copy the exported file out byte for byte, and save it as `gstr1-aug-2026-a.json`. Say the exact path Tally wrote it to and the file's size.

## One change
4. Use Accept As Is on `BP/26-27/0005` (the same way as in W12: select it in the uncertain list, accept, change no amount). Screenshot the dialogs.
5. Send `r02-marks.xml` -> `aw13-marks-1.xml`; `w13-status-window-aug.xml` -> `aw13-status-window-1.xml`; `w13-status-0005.xml` -> `aw13-status-0005-1.xml`.

## After
6. Export GSTR-1 for August 2026 as JSON again with the same options as step 3, and save the file as `gstr1-aug-2026-b.json`, with its path and size. Screens: the GSTR-1 summary again.
7. If the export dialog had an option about uncertain transactions that you did not choose in step 3, export once more with the other choice and save it as `gstr1-aug-2026-c.json` (say which option). If it had none, say so.

## What to post back
The answers (every request file named above), the three JSON files, the backup's place and the screenshots in `workorder/` on the same branch, their hashes added to `SHA256.txt`, and for each step what happened, with curl's exit code and printed line. Say whether you did anything that differs from this list.

---

*For the reader (not for the lab):* the lab's GSTR-1 screens already show that an uncertain voucher is not in the return view (the ten included rows sum exactly to the screen's taxable value) and that an accepted one is reported at its invoice figure. The JSON export itself has not been read. B2C Small is exported as totals per place of supply and rate, so a single voucher shows only as a change in those totals: exports `a` and `b` differ by exactly `0005`'s taxable value and tax if the export leaves out uncertain vouchers and takes in accepted ones (taxable 1001.00, 25.02 and 25.02, or the figures the status read shows), and they are equal if the export includes uncertain vouchers either way. The totals in `a` are compared with the sums of the included vouchers from the status read.
