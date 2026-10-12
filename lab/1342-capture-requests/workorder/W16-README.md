# W16: the exact ledger, group and company reads the HSN/SAC check before a Sales post will send

Synthetic company `BRIDGE PILOT LAB` only. READS ONLY: nothing is keyed, altered or deleted, and no screen is changed. No backup is needed (say so if you take one). It can run any time after W15.

Each new request below is an answered request with ONE change: field names added to its field list. Each is sent right after the answered request it was derived from (the control), so a failure of the new one cannot be confused with a gateway or book problem.

**General STOP rule (any step).** Stop and tell us before going on if: Tally shows any dialog, message or warning; an answer is not a complete envelope with a `COLLECTION` (or, for the object read, an `ENVELOPE` with a `BODY`) or carries a `LINEERROR`; a control answer has no rows; an answer is larger than 500 KB or takes longer than one minute (do not resend it; check Tally still answers the small read `r02-marks.xml`). Never use Accept As Is. Do not open any voucher or master for alteration.

**How to send.** As before: one request at a time, the same `curl.exe` line with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and printed line for every request. Check each request file against `SHA256.txt` first (`w7-ledgers-rates.xml` and `w14-company-object.xml` are in this folder's `SHA256.txt` from earlier sets; `r07-groups.xml` is in the top-level `SHA256.txt`).

## Step 1: ledgers (control, then the new field list)
1. `w7-ledgers-rates.xml` -> `aw16-ledgers-rates-0.xml` (control: answered on 10 Oct; about 51 KB; every ledger with its GST rate rows).
2. `w16-ledger-rates-hsn.xml` -> `aw16-ledgers-rates-hsn-0.xml` (the same request with `HSNDETAILS.LIST, GSTTYPEOFSUPPLY, GSTAPPLICABLE` added to the field list, nothing else changed).

## Step 2: groups (control, then the new field list)
3. `r07-groups.xml` (top folder) -> `aw16-groups-0.xml` (control: answered on 10 Oct; every group with its parent).
4. `w16-groups-hsn.xml` -> `aw16-groups-hsn-0.xml` (the same request with `HSNDETAILS.LIST` added to the field list, nothing else changed).

## Step 3: the company, in the same book state
5. `w14-company-object.xml` -> `aw16-company-object-0.xml` (answered in W14; about 42 KB).
6. `r02-marks.xml` (top folder) -> `aw16-marks-0.xml` (the book's change marks after the reads; they should equal the marks before step 1 if you read them; say if you did).

## What to post back
The six answers, their hashes added to `SHA256.txt`, and for each request curl's exit code and printed line. Screens: none needed. Say the HSN/SAC codes now set on the company, the group `Sales Accounts`, the sub-group `BRIDGE Sales Sub` and the ledgers `BRIDGE Svc INH 5%` and `BRIDGE Svc INH2 5%` as the masters' GST screens show them (look only; do not change them), so the answers can be checked against the screens. Say whether you did anything that differs from this list.

---

*For the reader (not for the lab):* the HSN/SAC check before a Sales post resolves each sales ledger's code ledger -> group chain -> company, as measured in W11 and W14. The build already sends the ledger rates read and the group listing; this set measures the two with the HSN fields added, so the requests the code sends are requests Tally has answered, byte for byte. The company object read is W14's, unchanged. Pre-registered expectations (written before any answer): (a) the ledger answer has the same ledgers as the control, each now with `HSNDETAILS.LIST` rows as the W9 and W11 all-fields reads showed them (`APPLICABLEFROM`, `HSNCODE`, `HSN`, `HSNCLASSIFICATIONNAME`, `SRCOFHSNDETAILS`), `GSTAPPLICABLE` (with Tally's control-character prefix) and `GSTTYPEOFSUPPLY` on the GST sales ledgers; (b) the group answer has the same groups as the control, with `HSNDETAILS.LIST` on the groups that carry a code; a group without one carries an empty or placeholder row, or none (which of these is the finding); (c) the company object carries `HSNDETAILS.LIST` and `CMPGSTDETAILS.LIST` as in W14; (d) the marks do not move. The opposite of any of these is a finding, not an error.
