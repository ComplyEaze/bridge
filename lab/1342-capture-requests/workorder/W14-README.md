# W14: where Tally keeps the company-level HSN/SAC, and whether the company's tax-amount tolerance stops the flag on a paisa of rounding

Synthetic company `BRIDGE PILOT LAB` only (or a fresh copy of it; say which). This work writes ONE company setting and re-saves ONE voucher. Take a fresh backup first, say where it is, and say the numbers you see on the GSTR-1 screens before you change anything. Do the steps in order; each step changes one thing. It runs after W13.

**General STOP rule (any step).** Stop and tell us before going on if: Tally shows a dialog or warning that the step does not mention; a request answer is not a complete envelope with a `COLLECTION` (or, for the object read, an `ENVELOPE` with a `BODY`) or carries a `LINEERROR`; a read that must return a voucher returns none; the answer to `w14-company-object.xml` is larger than 500 KB or takes longer than one minute (do not resend it; check Tally still answers the small read `r02-marks.xml`); a number on a GSTR-1 screen changes in a step that says it must not. Never use Accept As Is. Do not alter or delete any other master or voucher.

**How to send.** As before: one request at a time, the same `curl.exe` line with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and printed line for every request. Check each request file against `SHA256.txt` first (`r02-marks.xml` and `w13-status-window-aug.xml` and `w12-status-0002.xml` are from earlier sets: their hashes are in those sets' `SHA256.txt`).

## Step 0 (no Tally; one search)
On the lab PC, search the kept answer `aw11-taxunits-0.xml` for any element whose name contains `HSN`, and report each name with its value (or "none"). STOP: none (this step reads a file only).

## Step 1: reads, nothing changed (the company-level HSN read candidates)
1. `w14-company-object.xml` -> `aw14-company-object-0.xml` (the whole company object; a verified request shape, about 41 KB, 565 tags in an earlier book). It may carry the company-level HSN/SAC and the tolerance. STOP: see the general rule (size, time).
2. `w14-company-fetch.xml` -> `aw14-company-fetch-0.xml` (a short field list: tolerance, HSN applicability, GST classification, and the two HSN and GST detail lists). Say which of those elements came back.
3. `r02-marks.xml` -> `aw14-marks-0.xml`; `w13-status-window-aug.xml` -> `aw14-status-window-0.xml`; `w12-status-0002.xml` -> `aw14-status-0002-0.xml`; `w14-status-0008.xml` -> `aw14-status-0008-0.xml`.
4. Screens: GSTR-1 for August 2026: the summary (counts of included and uncertain, the B2C Small taxable value and tax), and the uncertain list with the reasons. Vouchers `0002` and `0008` carry the same figures (1001.00, 25.03 and 25.02) and are expected to be uncertain for a tax mismatch; if either is not uncertain, STOP and tell us.

## Step 2: the tolerance (one change)
5. Find where the company's tax-amount tolerance is set on screen (the Company's GST details or the F11 features; the field is `CMPTAXAMOUNTTOLERENCE`, 0 now). Screenshot the screen BEFORE you change it. STOP: if no screen offers a tolerance for tax amounts, tell us where you looked and do not change anything else: the step ends here and the rest is not done.
6. Set the tolerance to `0.05` (nothing else on that screen) and save. Screenshot after. STOP: any prompt other than the master-change one.
7. Send `w14-company-object.xml` -> `aw14-company-object-1.xml` and `w14-company-fetch.xml` -> `aw14-company-fetch-1.xml`. They should differ from the "-0" answers in the tolerance (and counters); say what else differs.

## Step 3: the re-check (one change)
8. Open GSTR-1 for August 2026. If Tally shows the master-change dialog ("GST-related details were altered in the masters ..."), choose "C: Current Period" and say what it said and the counts. Screenshot the dialog and the summary.
9. Send `w13-status-window-aug.xml` -> `aw14-status-window-1.xml`, `w12-status-0002.xml` -> `aw14-status-0002-1.xml`, `w14-status-0008.xml` -> `aw14-status-0008-1.xml`, `r02-marks.xml` -> `aw14-marks-1.xml`. Record the GSTR-1 summary: counts and the B2C Small taxable value and tax. Say whether `0002` and `0008` are still uncertain. STOP: any change in a voucher you did not expect to change (compare the two window reads by structure).

## Step 4: re-save one uncertain voucher (one change, to separate "tolerance" from "re-save")
10. Open `BP/26-27/0008`, change nothing, save. `0002` stays untouched (the control).
11. Send `w14-status-0008.xml` -> `aw14-status-0008-2.xml`, `w12-status-0002.xml` -> `aw14-status-0002-2.xml`, `w13-status-window-aug.xml` -> `aw14-status-window-2.xml`, `r02-marks.xml` -> `aw14-marks-2.xml`. Screens: GSTR-1 summary and the uncertain list; record the B2C Small taxable and tax totals before (step 3) and now: the difference is what Tally reports for `0008` (the typed 25.03 and 25.02, or Tally's own figure).

## Step 5: put the setting back (one change)
12. Set the tolerance back to `0`, save, open GSTR-1 (choose "C: Current Period" if asked) and screenshot the summary. Send `w14-company-object.xml` -> `aw14-company-object-2.xml`. STOP: if the tolerance cannot be set back to 0, restore the backup of this step's start and tell us.

## What to post back
All the answers named above, the backup's place, the screenshots, their hashes added to `SHA256.txt`, and for each step what happened with curl's exit code and printed line. Say whether you did anything that differs from this list.

---

*For the reader (not for the lab):* the company read of W11 returned only an empty `HSNAPPLICABILITY` and no company-level code, so a refuse-before-post rule cannot see the company level. The object read returned 565 tags in an earlier book; this asks whether it carries the code. The company read also carries `CMPTAXAMOUNTTOLERENCE` (0). Pre-registered expectations (written before any answer): (a) the object read carries the company-level HSN/SAC as some element with the code and dates (not certain; if it does not, no read of the company's code is known and the refuse rule stays a warning); (b) setting the tolerance changes only that element and counters; (c) the re-check does NOT change the flags of saved vouchers (as in W11-R); (d) the re-save of `0008` makes it included only if the tolerance applies at save, and `0002` stays uncertain; (e) the GSTR-1 totals after the re-save show whether Tally reports the typed figure or its own for a tolerated voucher; the opposite of (c) or (d) is a finding, not an error.
