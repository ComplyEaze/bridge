# W12: what Accept As Is changes in a voucher

Synthetic company `BRIDGE PILOT LAB` only. This work writes: it uses Accept As Is on two vouchers and saves one of them again. Take a fresh backup first and say where it is. Stop wherever time runs out; do the steps in order.

**Stop rule.** Stop and tell us, before going on, if: Tally shows a dialog or warning this list does not mention; a request answer is not a complete envelope with a `COLLECTION` element (or carries a `LINEERROR`); a read for a voucher that exists returns no voucher (keep the file); the GSTR-1 screen shows a reason for a voucher that is not a tax mismatch (screenshot it and stop before Accept As Is on that voucher); anything differs from the steps. Use Accept As Is on `BP/26-27/0003` and `BP/26-27/0006` ONLY. Do not touch `BP/26-27/0002`, which is the control. Do not alter or delete any other voucher or master.

**How to send.** As before: one request at a time, the same `curl.exe` line with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and the printed line for every request. Check each request file against `SHA256.txt` first; if one differs, do not send it. `r02-marks.xml` is from the earlier set (its hash is in that set's `SHA256.txt`).

## Before (nothing changed yet)
1. Send `r02-marks.xml` -> `aw12-marks-0.xml`.
2. For each of the vouchers `0002`, `0003`, `0006`: send `w12-full-<n>.xml` -> `aw12-full-<n>-0.xml` and `w12-status-<n>.xml` -> `aw12-status-<n>-0.xml`.
3. Screens: look at the uncertain list: say whether Tally lets you select more than one voucher at once for Accept As Is (look only; do not do it), and for EVERY uncertain voucher of the book give the reason text Tally shows (say if any voucher shows more than one reason). GSTR-1 for the month of these vouchers: the summary, the list of uncertain transactions, and for `0003` and `0006` the screen that gives the reason Tally shows for each (a tax mismatch is expected: copy the wording into your notes and screenshot it, including the figures Tally shows for expected and modified tax if it shows them).

## Accept As Is on 0003
4. In the uncertain list, select `0003` and use Accept As Is (alter, then accept, as Tally offers it; do not change any amount). Screenshot each dialog. Say exactly which menu or key you used and what Tally asked. Then open the voucher's Edit Log (audit trail) and screenshot what it records for this change.
5. Send `r02-marks.xml` -> `aw12-marks-1.xml`; `w12-full-0003.xml` -> `aw12-full-0003-1.xml`; `w12-status-0003.xml` -> `aw12-status-0003-1.xml`; and, as the control, `w12-full-0002.xml` -> `aw12-full-0002-1.xml`.
6. Screens: GSTR-1 summary and the uncertain list again.

## Accept As Is on 0006
7. Do the same on `0006` (steps 4 to 6 with `0006`, the marks saved as `aw12-marks-2.xml`, the voucher answers as `aw12-full-0006-1.xml` and `aw12-status-0006-1.xml`, and the control `0002` as `aw12-full-0002-2.xml`).

## Save again
8. Open the accepted voucher `0006`, change nothing, and save it. Send `r02-marks.xml` -> `aw12-marks-3.xml`, `w12-full-0006.xml` -> `aw12-full-0006-2.xml`, `w12-status-0006.xml` -> `aw12-status-0006-2.xml`. Screens: GSTR-1 summary and the list. (This tells us whether saving an accepted voucher keeps the acceptance.)

## Edit after accepting (0003 only)
9. Open the accepted voucher `0003` and change ONLY its narration (add `W12`), save. Send `r02-marks.xml` -> `aw12-marks-4.xml`, `w12-full-0003.xml` -> `aw12-full-0003-2.xml`, `w12-status-0003.xml` -> `aw12-status-0003-2.xml`. Screens: GSTR-1 summary and the list (does the voucher stay accepted?).
10. Then change ONE amount of `0003` so its tax is one paisa different again (for example the state tax) and save. Same sends -> `aw12-marks-5.xml`, `aw12-full-0003-3.xml`, `aw12-status-0003-3.xml`. Screens as before. (This tells us whether editing a figure voids the acceptance, which shows what the stored checksum covers.)

## What to post back
The answers (every file named above), the backup's place and the screenshots in `workorder/` on the same branch, their hashes added to `SHA256.txt`, and for each step what happened, with curl's exit code and printed line. Say whether you did anything that differs from this list.

---

*For the reader (not for the lab):* §9.17 measured that Accept As Is alters the voucher, overrides the flag and leaves the amounts unchanged, on one invoice-view voucher. These answers give the whole voucher before and after (every native field), so we can say which fields Accept As Is changes, what the AlterID does, whether the voucher's other fields are untouched, and whether saving the voucher again keeps it. The untouched control (`0002`) shows nothing else moves. The shape found here decides whether a write that Bridge makes can reproduce it; a second request set built from these answers (an import of the changed field alone) comes after this one is read.
