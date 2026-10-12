# W18: every read a Sales invoice build sends, in one sitting, now that `BRIDGE Svc 4D 5%` exists

Synthetic company `BRIDGE PILOT LAB` only. READS ONLY: nothing is keyed, altered or deleted, no voucher or master is opened for alteration, and no screen is changed. No backup is needed (say so if you take one). Never use Accept As Is. It can run any time; nothing else should touch the company during the sitting.

Every request below has already been answered by Tally on this company. Twelve are the same files, unchanged. One, `w18-number-0040.xml`, is `../r09-number-new-invoice.xml` with only the voucher number changed (`BP/26-27/0010` to `BP/26-27/0040`, nothing else). Check each file against its `SHA256.txt` first: the top folder's for the `r` files; this folder's for `w14-company-object.xml`, `w16-ledger-rates-hsn.xml` and `w18-number-0040.xml`.

**General STOP rule (any step).** Stop and tell us before going on if: Tally shows any dialog, message or warning; an answer is not a complete envelope with a `COLLECTION` (or, for the company object, an `ENVELOPE` with a `BODY`) or carries a `LINEERROR`; an answer is larger than 500 KB or takes longer than one minute (do not resend it; check Tally still answers the small read `r02-marks.xml`).

**How to send.** As before: one request at a time, the same `curl.exe` line with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and printed line for every request.

## The sitting, in this order
1. `../r02-marks.xml` -> `aw18-marks-0.xml` (the book's change marks, first).
2. `../r01-company-extent.xml` -> `aw18-company-extent.xml`
3. `../r03-book-extent.xml` -> `aw18-book-extent.xml`
4. `../r04-currencies.xml` -> `aw18-currencies.xml`
5. `../r05-ledgers-compliance.xml` -> `aw18-ledgers-compliance.xml`
6. `../r06-ledgers-paired.xml` -> `aw18-ledgers-paired.xml`
7. `../r07-groups.xml` -> `aw18-groups.xml`
8. `w16-ledger-rates-hsn.xml` -> `aw18-ledger-rates-hsn.xml`
9. `../r08-voucher-types.xml` -> `aw18-voucher-types.xml`
10. `w18-number-0040.xml` -> `aw18-number-0040.xml`
11. `../r16-tax-units.xml` -> `aw18-tax-units.xml`
12. `w14-company-object.xml` -> `aw18-company-object.xml`
13. `../r13-ledger-catalogue-v2.xml` -> `aw18-ledger-catalogue-v2.xml`
14. `../r02-marks.xml` -> `aw18-marks-1.xml` (the marks again, last).

## What to post back
The fourteen answers, their hashes added to `SHA256.txt`, and for each request curl's exit code and printed line. Say whether you did anything that differs from this list. Screens: none needed.

---

*For the reader (not for the lab):* the HSN/SAC warnings change (#1646) is tested on the lab's rates answer, but no build on the four-digit ledger can be replayed. The 9 Oct compliance listing, paired listing and ledger catalogue predate that ledger, and the replay of `BP/26-27/0010` already pairs 9 Oct answers with 12 Oct ones. This sitting gives one book state for every read of a build, so a build on `BRIDGE Svc 4D 5%` (and the post's re-reads before its approval, which repeat the same requests) can be replayed from it alone. Two mutants of #1646 stay open until that replay runs: the warnings passed into the saved observation, and the call that adds them to the verification result.

Pre-registered expectations (written before any answer): (a) the two marks answers are equal; (b) the compliance listing, the paired listing and the catalogue each carry the eighteen ledgers of W16's rates answer: the eleven of 9 Oct and the seven added since (`BRIDGE Svc 4D 5%`, `BRIDGE Svc 998315 5%`, `BRIDGE Svc INH 5%`, `BRIDGE Svc INH2 5%`, `BRIDGE Purchase Svc 5%`, `BRIDGE Output CGST XML`, `BRIDGE Supplier RJ`), and the groups answer carries `BRIDGE Sales Sub`, which the 9 Oct groups answer does not; (c) the rates answer's row for `BRIDGE Svc 4D 5%` equals W16's (HSN/SAC 9983 from 1 Apr 2026, Specify Details Here; type of supply Services); (d) the number read finds no voucher numbered `BP/26-27/0040` (if it finds one, that is a finding: say so, and the number is changed before any build); (e) the company object carries the company-level code and settings as in W14 and W16. The opposite of any of these is a finding, not an error.
