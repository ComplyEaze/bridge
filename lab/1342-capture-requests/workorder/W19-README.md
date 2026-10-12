# W19: the GSTR-1 status read with the party, the amount and the cancelled and optional flags, and what it shows around an accept

Synthetic company `BRIDGE PILOT LAB` only. Steps A and B are READS ONLY. Steps C, D and E key vouchers, create one voucher type, and use Accept As Is on that company, each after a fresh backup taken just before it. Vouchers `BP/26-27/0002` (the control) and `BP/26-27/0008` (reserved) are never opened, altered or accepted.

`w13-status-window-aug.xml` is the request the code renders for this company, the window 1 to 31 August 2026 and the type `BRIDGE Sales`: the code's own renderer gives it byte for byte (SHA-256 `f363e3e7…`), and Tally answered it in W13. `w19-status-window-aug-wide.xml` is that request with ONE change: `,PARTYLEDGERNAME,ISCANCELLED,ISOPTIONAL,AMOUNT` added at the end of its field list. `w19-status-window-aug-wide-two.xml` is the same wide request for the type name `BRIDGE Sales Two` (the code's render of the narrow request for that name, with the same four names added). Check each file against `SHA256.txt` first.

**General STOP rule (any step).** Stop and tell us before going on if: Tally shows any dialog, message or warning you did not expect from the step, and above all one that offers to update the GST status of vouchers for all periods or a period outside August (as in W11 and W12: press Esc, record it, do not choose a period); an answer is not a complete envelope with a `COLLECTION` or carries a `LINEERROR`; an answer is larger than 500 KB or takes longer than one minute (do not resend it; check Tally still answers `../r02-marks.xml`).

**How to send.** As before: one request at a time, the same `curl.exe` line with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and printed line for every request.

## Step A: the control (reads only)
1. `../r02-marks.xml` -> `aw19-marks-0.xml`
2. `w13-status-window-aug.xml` -> `aw19-a.xml` (answered in W13).

## Step B: the wider request (reads only)
3. `w19-status-window-aug-wide.xml` -> `aw19-b-0.xml`
4. Look only: open the invoices `BP/26-27/0001`, `BP/26-27/0002` and `BP/26-27/0016` in display mode (do not alter), and write down the invoice total each shows, and its party.

## Step C: one cancelled and one optional Sales voucher (writes, after a backup)
5. Take a backup (`W19-C`). Key two new `BRIDGE Sales` invoices dated 20 August 2026, the same shape as `BP/26-27/0001`, with numbers `BP/26-27/0041` and `BP/26-27/0042`. Save `0041`, then cancel it (Alt+X). Key `0042` as an optional voucher (Ctrl+L before saving).
6. `w19-status-window-aug-wide.xml` -> `aw19-c.xml`

## Step D: a second Sales-class voucher type (writes, after a backup)
7. Take a backup (`W19-D`). Create the voucher type `BRIDGE Sales Two` of type Sales, with manual numbering. Key one invoice in it dated 21 August 2026, number `BP2/0001`, the same shape as `BP/26-27/0001`.
8. `w19-status-window-aug-wide.xml` -> `aw19-d-sales.xml`
9. `w19-status-window-aug-wide-two.xml` -> `aw19-d-sales-two.xml`

## Step E: two uncertain invoices accepted at once (writes, after a backup)
10. Take a backup (`W19-E`). Key two new `BRIDGE Sales` invoices dated 22 August 2026, numbers `BP/26-27/0043` and `BP/26-27/0044`, the same shape as `BP/26-27/0002` (the CGST line one paisa off the per-line figure, so Tally lists them as uncertain).
11. `w19-status-window-aug-wide.xml` -> `aw19-e-0.xml`
12. Open GSTR-1 for August. In the uncertain list, select `0043` and `0044` with Space (both, nothing else) and use Accept As Is once. Record what the screen asks and shows (screenshots). If the screen cannot accept a selection of two, accept `0043` alone and say so.
13. `w19-status-window-aug-wide.xml` -> `aw19-e-1.xml`
14. `../r02-marks.xml` -> `aw19-marks-1.xml`

## What to post back
Every answer, its hash added to `SHA256.txt`, and for each request curl's exit code and printed line; the three invoice totals and parties of step 4; the screenshots of step 12; the backup names. Say whether you did anything that differs from this list.

---

*For the reader (not for the lab):* the read-only list of uncertain Sales vouchers (Accept As Is, Phase 1) is to be built on the GSTR-1 status read with four field names added. This set measures that request before any code sends it. It also measures the states the list must keep apart: a cancelled and an optional voucher, a second voucher type of the same class, and an accept of a selection. W12 already measured that an amount edit clears an acceptance (the flag back to No, the checksum to 0, uncertain again), so that is not repeated.

Pre-registered expectations (written before any answer): (a) `aw19-a.xml` carries the same vouchers and statuses as W13's `aw13-status-window-1.xml`, apart from anything changed on the company since; (b) `aw19-b-0.xml` carries the same vouchers as `aw19-a.xml`, each with `PARTYLEDGERNAME`, `ISCANCELLED` No, `ISOPTIONAL` No and an `AMOUNT`. Whether `AMOUNT` is the invoice total, the first entry's amount, empty or absent is the finding, compared with step 4; (c) in `aw19-c.xml`, `0041` reads `ISCANCELLED` Yes and `0042` reads `ISOPTIONAL` Yes. Which status flags each carries is the finding; (d) `aw19-d-sales.xml` does not carry `BP2/0001`, and `aw19-d-sales-two.xml` carries only it; (e) `aw19-e-0.xml` reads `0043` and `0044` uncertain and not overridden, and `aw19-e-1.xml` reads both not uncertain and overridden if the selection was accepted at once, with `ALTERID` moved and `AMOUNT` unchanged; (f) `0002` is byte-identical in every answer that carries it, and `0008` too. The opposite of any of these is a finding, not an error.
