# W11: where Tally takes a sales ledger's HSN/SAC from

Synthetic company `BRIDGE PILOT LAB` only. This work writes (a group, three ledgers, HSN/SAC settings on the company and two groups, ten vouchers). Take a fresh backup first and say where it is. Stop wherever time runs out: each later step depends on the earlier ones, so do them in order.

**Stop rule.** Stop and tell us, before going on, if: Tally shows a dialog or warning this list does not mention; a request answer is not a complete envelope with a `COLLECTION` element (or carries a `LINEERROR`); a read-back answer holds no voucher for a voucher you saved (an empty answer for a saved voucher is itself a finding about the request: keep the file, stop, tell us); the company read gets no answer within two minutes (do not send it again; check that Tally still answers the small read `r02-marks.xml` and tell us); a voucher is refused or Tally fills no tax amount; anything differs from the steps. Never use Accept As Is. Do not alter or delete any other master or voucher.

**How to send.** As before: one request at a time, the same `curl.exe` line with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and the printed line for every request. Check each request file against `SHA256.txt` first; if one differs, do not send it. Request files are in `workorder/` except `r16-tax-units.xml` (the earlier set, its hash is in that set's `SHA256.txt`).

**The vouchers.** Each is `BRIDGE Sales`, dated 7-Aug-2026, the walk-in customer, invoice mode, no round off, ONE sales line of 100.00 on the ledger named in the step, with `BRIDGE CGST 2.5%` and `BRIDGE SGST 2.5%` selected and the amounts left as Tally fills them (2.50 each; if Tally fills none, stop and tell us). The NARRATION of voucher k is exactly `W11-0k` (`W11-01` ... `W11-10`); the read-back requests find the vouchers by that narration, so the voucher numbers do not matter, but tell us the number Tally gave. After saving each voucher send its read-back file (`w11-readback-n0k.xml` -> `aw11-readback-n0k.xml`) and screenshot the voucher.

## Setup (reads only)
0. Send `r16-tax-units.xml` -> `aw11-taxunits-0.xml`; `w11-group-sales-all.xml` -> `aw11-group-sales-0.xml`; `w11-company-all.xml` -> `aw11-company-0.xml`.

## Setup (masters)
1. Create the group `BRIDGE Sales Sub` under `Sales Accounts`, nothing else changed.
2. Create the ledger `BRIDGE Svc INH 5%` under `Sales Accounts` and the ledger `BRIDGE Svc INH2 5%` under `BRIDGE Sales Sub`. Both: type of supply Services; taxable, 5%, rate specified in the ledger (as `BRIDGE Svc 998314 5%` has it); HSN/SAC details set to `As per Company/Group` (no own code). Screenshot each ledger's GST and HSN/SAC details.
3. Send `w11-ledger-inh-all.xml` -> `aw11-ledger-inh-0.xml`, `w11-ledger-inh2-all.xml` -> `aw11-ledger-inh2-0.xml`, `w11-group-sub-all.xml` -> `aw11-group-sub-0.xml`.

## Questions, one change each (nothing is set anywhere at the start)
A. Key `W11-01` on `BRIDGE Svc INH 5%` and `W11-02` on `BRIDGE Svc INH2 5%`. (Nothing resolves: expected uncertain; the control.) Read each back.
B. Set the COMPANY-level HSN/SAC to `998341`, applicable from 1-Apr-2026 (Company alter, GST details, HSN/SAC). Screenshot. Send `w11-company-all.xml` -> `aw11-company-1.xml`. Key `W11-03` on INH and `W11-04` on INH2. Read each back.
C. Also set the group `Sales Accounts` to `998342` from 1-Apr-2026 (company stays 998341). Screenshot. Send `w11-group-sales-all.xml` -> `aw11-group-sales-1.xml`. Key `W11-05` on INH and `W11-06` on INH2. Read each back.
D. Also set the group `BRIDGE Sales Sub` to `998343` from 1-Apr-2026. Screenshot. Send `w11-group-sub-all.xml` -> `aw11-group-sub-1.xml`. Key `W11-07` on INH2. Read it back.
E. Give the ledger `BRIDGE Svc INH 5%` its own code `998344` (Specify Details Here, from 1-Apr-2026). Screenshot. Send `w11-ledger-inh-all.xml` -> `aw11-ledger-inh-1.xml`. Key `W11-08` on INH. Read it back.
F. Add a second HSN/SAC row to `BRIDGE Svc INH 5%`: code `998345` applicable from 1-Sep-2026 (after the voucher date). Screenshot. Send `w11-ledger-inh-all.xml` -> `aw11-ledger-inh-2.xml`. Key `W11-09` on INH dated 7-Aug-2026. Read it back.
G. Create the ledger `BRIDGE Svc 4D 5%` under Sales Accounts as in step 2 but with its own code `9983` (four digits), from 1-Apr-2026. Key `W11-10` on it. Read it back. Say what the company's annual turnover threshold setting (in the GST details of the company) reads.

## Screens (look, change nothing)
After each voucher: GSTR-1 for August 2026, whether the voucher is included or uncertain, with Tally's reason if uncertain (the summary screen once per question is enough). After G: the same, and the GST tax analysis screen of `W11-07` and `W11-08` if Tally shows which HSN/SAC it used.

## What to post back
The answers (every file named above), the backup's place and the screenshots in `workorder/` on the same branch, their hashes added to `SHA256.txt`, and for each step what happened, with curl's exit code and printed line, and the voucher numbers Tally gave. Say whether you did anything that differs from this list.

---

*For the reader (not for the lab):* the read-back of an invoice carries, per sales line, `HSNSOURCETYPE`, `HSNLEDGERSOURCE`, `HSNGROUPSOURCE`, `GSTHSNNAME` and `GSTHSNINFERAPPLICABILITY`; on the one voucher the book flagged for a missing HSN/SAC they were empty. These answers show, for ledgers that say "As per Company/Group", which level Tally used, whether a group or company code makes Tally include the voucher, which wins when several are set, whether a row dated after the voucher is ignored, and what Tally does with a four-digit code. The first reads (0, 3) find the names of the fields that carry a group's and a company's HSN/SAC, which no earlier answer holds. The narration filter is itself new: step A's read-backs also test it.
