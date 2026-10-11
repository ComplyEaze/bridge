# W10: a ledger with two rates, a ledger switched off, and one 12% and one 18% bill

After W9. This work writes: it creates four ledgers, alters one of them, and keys two vouchers, all in `BRIDGE PILOT LAB` only. Part A is what we need first; Part B only if time allows. Stop wherever time runs out.

**Stop rule.** Stop and tell us, before going on, if any of these happens: Tally shows a dialog or a warning that this list does not mention; a request answer is not a complete envelope with a `COLLECTION` element (or carries a `LINEERROR`); a voucher is refused or Tally fills no tax amount; a ledger would have to be created in a company other than `BRIDGE PILOT LAB`; anything differs from the steps. Never use Accept As Is. Do not alter or delete any other ledger or voucher.

**Before you start.** Check each request file against `SHA256.txt` first (`Get-FileHash`); if one differs, do not send it. Take a fresh backup of the company and say where it is.

**How to send** (as before). One request at a time, with the same `curl.exe` line and `-w "%{http_code} %{size_download}"`, each answer written to a file with `-o` and never piped. Note curl's exit code and the printed line for every request.

## Part A: a ledger with a rate history, and a ledger switched to "GST not applicable"

1. Create the ledger `BRIDGE Svc Hist` under Sales Accounts, with GST details that hold two rows: from 1-Apr-2026 taxable at 5% (rate specified in the ledger, as `BRIDGE Svc 998314 5%` has it), and from 1-Jun-2026 taxable at 12% (same shape). If Tally refuses the second row, stop. Screenshot the whole GST rate details screen.
2. Create the ledger `BRIDGE Svc NA 5%` under Sales Accounts with the same GST details as `BRIDGE Svc 998314 5%` (taxable, 5%, the rate specified in the ledger). Screenshot its GST details.
3. Send, in this order:

| Send | Save as | Expect, and what to do if not |
| --- | --- | --- |
| `w10-rates-build-window.xml` | `aw10-rates-build.xml` | the ledger listing of the build's own shape (1-Apr-2026 to 5-Aug-2026); `BRIDGE Svc Hist` should list its GST rows. Say how many rows it carries and with which dates and rates |
| `w10-rates-late-window.xml` | `aw10-rates-late.xml` | the same listing for 1-Jul-2026 to 5-Aug-2026 (both rows begin before this window) |
| `w10-rates-early-window.xml` | `aw10-rates-early.xml` | the same listing for 1-Apr-2026 to 1-May-2026 (the second row begins after this window) |
| `w10-ledger-gst-switch-fields.xml` | `aw10-switch-before.xml` | the ledger `BRIDGE Svc NA 5%` with its GST applicability, type of supply and rate rows |

4. Alter `BRIDGE Svc NA 5%`: change its GST applicability to Not Applicable and save. Screenshot the screen before you save, and say what Tally did with the 5% row and whether it asked anything.
5. Send `w10-ledger-gst-switch-fields.xml` again, saved as `aw10-switch-after.xml`, and `w10-rates-build-window.xml` again, saved as `aw10-rates-build-after.xml`. Say whether `BRIDGE Svc NA 5%` still carries its 5% row.

## Part B (optional): one 12% bill and one 18% bill

6. Create `BRIDGE CGST 6%` and `BRIDGE SGST 6%` like `BRIDGE CGST 2.5%` and `BRIDGE SGST 2.5%` (same group, same duty head, rate 6, no rounding), and the sales ledger `BRIDGE Svc 12%` under Sales Accounts (taxable, 12%, rate specified in the ledger). Screenshot each ledger's GST details.
7. Key `BP/26-27/0022`: type `BRIDGE Sales`, dated 6-Aug-2026, the walk-in customer, invoice mode, no round off. One sales line, `BRIDGE Svc 12%`, 100.10. Select `BRIDGE CGST 6%` and `BRIDGE SGST 6%` and leave the amounts exactly as Tally fills them. If Tally fills no amount, stop and tell us. Save it. Screenshot the voucher.
8. Key `BP/26-27/0023` the same way, one sales line 100.10 on the 18% sales ledger that voucher `0009` used, with the 9% central and state tax ledgers that `0009` used. Same rule about the amounts. Save it. Screenshot the voucher.
9. Send `w10-status-window-0806.xml`, saved as `aw10-status-0806.xml`.
10. Screens (look, change nothing): GSTR-1 for August 2026, showing whether `0022` and `0023` are included or uncertain, with Tally's reason if uncertain.

**What to post back.** The answers, the backup's place and the screenshots in a folder `workorder/` on the same branch, their hashes added to `SHA256.txt`, and for each step what happened, with curl's exit code and printed line. Say whether you did anything that differs from this list.

---

*For the reader (not for the lab):* the answers settle three open points of the Sales tax check. (a) Whether a ledger's rate rows come back whole under the window the build sends or are cut by it (Part A, the three listings). (b) Whether a ledger switched off GST still returns its old rate row, and under which field the switch shows (steps 4-5). (c) Whether Tally's own flag holds for a 12% and an 18% single-rate bill with a tax figure that is not a whole paisa (Part B), which decides whether those rates are admitted.
