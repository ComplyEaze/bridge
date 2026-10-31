# Results: 1867-L3 v1.1, `gst_by_counter_ledger` on BRIDGE GST LEDGER LAB as 628-C0 and C0b left it, TallyPrime 7.2 Silver, 27 Oct 2026 (UTC)

The sheet is `lab/1867-l3/WINDOWS-1867-L3.md` at 37afa39e57a403fc5f2a219047f58f0d59281675 (SHA256.txt 2 of 2 equal). This was a read only, with nothing keyed, backed up or restored. The tool was the `bridge_mcp.exe` of pull request 1867's head 796e94f1 (sha256 bf8cdf82e10bf22f693d3c5d6d89bd074b35b2dbf6dc2cbad748ac8609916f46, equal before and after). Steps 3 to 5 went through the read-only recorder of GS1-L1 run 3 at e5dffc2b (SHA256.txt 21 of 21; self-test ok). The company is GUID aae372c5-a894-491c-8843-5532e43d2d2f, number <company number>.

## FINDING
**The duplicate supplier invoice GS1/P/0001 is totalled twice, and the answer flags nothing.** No. 57 and the K1 copy No. 58 carry the same reference and date, and the same bill (BILLID 12, as 628-C0b's reads show). Both are attributed to Purchase - Goods: debit CGST -1800 and SGST -1800, which is 2 × 900 per head. The December answer flags neither voucher: nothing is set aside, and no warning or field marks the repeat.

## The answers (nothing judged at the lab)
- **The marks.** M 228 and V 71 before and after, equal to 628-C0b's close. The calls wrote nothing.
- **The December Day Book** held exactly C0's Purchase 57, K1 Purchase 58, K2 Purchase 60 (Optional), K3 Purchase 59 (cancelled), K4 Journal 5 and K5 Payment 3.
- **December** (`ans-1867l3-dec.json`): state complete, 6 vouchers observed, no refusal.

| counter ledger | group | vouchers | debit_side_tax | credit_side_tax |
|---|---|---|---|---|
| Purchase - Goods | Purchase Accounts | 2 | cgst -1800, sgst_utgst -1800 | none |
| GS1 Rent | Indirect Expenses | 1 | cgst -450, sgst_utgst -450 | none |

The answer also holds: `set_aside` none; `vouchers_cancelled_optional_or_post_dated` 2; `vouchers_without_gst_entry` 1. By count (6 = 2 + 1 + 2 + 1) and from the Day Book:
- **C0's Purchase 57 and its second copy K1 (58) are both attributed** to Purchase - Goods. The same supplier invoice GS1/P/0001 is totalled twice: 2 × 900 per head.
- The K4 Journal's input tax lands on **GS1 Rent**.
- The optional K2 and the cancelled K3 are counted apart and stay out of the totals.
- The Payment K5 has no GST entry.

The answer itself names no voucher per counter ledger. The recorder's voucher-window answers hold every voucher whole for the requester to confirm.
- **The year** (`ans-1867l3-year.json`): state complete, 70 vouchers observed, read whole with no refusal.

| counter ledger | vouchers | debit_side_tax | credit_side_tax |
|---|---|---|---|
| GS1 Professional Fees | 1 | igst -3600 | none |
| GS1 Rent | 53 | cgst -46350, sgst_utgst -46350 | cgst 90, sgst_utgst 90 |
| Purchase - Goods | 3 | cgst -2250, sgst_utgst -2250 | none |

  Set aside, each with its own totals:

  | reason | vouchers |
  |---|---|
  | class_not_admitted | 1 (Credit Note 1) |
  | duty_ledger_without_gst_type | 1 (Journal 2) |
  | gst_on_both_sides | 1 (Journal 3) |
  | no_creditor_party | 3 (Journal 4, Payments 1 and 2) |
  | other_ledgers_present | 2 (Purchases 2 and 3) |
  | several_counter_ledgers | 2 (Purchases 4 and 6) |

  All the set-aside vouchers are November's. Cancelled or optional: 2. Without a GST entry: 1.
- **A registered GSTIN on the December vouchers** (company 08ZZZZZ0000Z1ZQ, party 08ZZZZZ0000Z2ZP) changed no December decision: no December voucher was set aside.
- **The recorder** forwarded 220 requests and refused none. There was no dialog, no timeout and no size refusal.

## This build and the pull request's head
This build is older than the head. It does not admit Direct Expenses or Fixed Assets ledgers as counter ledgers, and its answers carry no `ledger_guid`. December uses only Purchase - Goods and GS1 Rent, which both admit. In the year, the November vouchers set aside as `other_ledgers_present` may read differently on the head.

## Files
- `tool-answers/`: every answer whole, plus `initialize` per launch.
- `recorder/`: the recorder's folder whole, with `manifest.tsv`.
- `client-log.txt`, `notes.txt` (UTC, one line per step).
- `SCREENSHOTS.txt`: the captures by name, bytes and sha256 (they stay on the lab PC).
