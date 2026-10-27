# Results: 628-C0 v1.3 and 628-C0b v2.1 on BRIDGE GST LEDGER LAB, TallyPrime 7.2 Silver, 27 Oct 2026 (UTC)

The sheets are `lab/628-c0/WINDOWS-628-C0.md` and `lab/628-c0b/WINDOWS-628-C0B.md` on `lab/1904-purchase-identity-capture` at e1e579c0b6ede0162e91d644bbecacae8f82d96c (SHA256.txt 5 of 5 and 7 of 7 equal; request census ok 2 and ok 4, FAIL 0). Both ran in one sitting on the company GS1-L1 created: GUID aae372c5-a894-491c-8843-5532e43d2d2f, number 100077, books from 1 April 2025. The tool was the `bridge_mcp.exe` of pull request 1867's head 796e94f1 (sha256 bf8cdf82e10bf22f693d3c5d6d89bd074b35b2dbf6dc2cbad748ac8609916f46, equal before the first call and after the last). The recorder was the read-only recording tool of GS1-L1 run 3, at e5dffc2b (SHA256.txt 21 of 21; self-test ok, 10 forwarded and 41 refused). The book keeps every master and voucher keyed here; nothing was restored or deleted.

## 628-C0: FINDING of the first rank
**A clean intra-state Purchase on a company whose GSTIN is the gate's synthetic identifier reads `uncertain`, not `included`.**
- **The company's GSTIN.** On 7.2 the GST registration is a master of its own (Alter > Statutory Masters > GST Registration), not a field on F11. It held State Not Applicable, type Regular and no GSTIN. It was set to Rajasthan, Regular, `08ZZZZZ0000Z1ZQ`; Tally accepted the GSTIN with no message and renamed the registration "Rajasthan Registration" by itself. ALTMSTID moved 225 → 227.
- **The supplier.** `SYN GS1 Supplier B` was created under Sundry Creditors: Regular, Rajasthan, `08ZZZZZ0000Z2ZP`, accepted with no message. ALTMSTID 228.
- **The voucher.** Purchase No. 57 of 2-12-2025 was keyed in Accounting Invoice mode: Supplier Invoice No. GS1/P/0001 of 1-12-2025; Purchase - Goods 10,000.00; Input CGST 900.00 and Input SGST 900.00, both typed (Tally filled no tax amount itself); 11,800.00; New Ref GS1/P/0001. ALTVCHID 64 → 65.
- **Step 6, `gst_status`** (December, Purchase): one voucher, No. 57, `gst_status` **uncertain**, `accepted_as_it_stands` false. Tally's own GST report was not opened.
- **Step 7, recorded `purchase_register`**: complete, one row. Reference GS1/P/0001 of 20251201; party GSTIN 08ZZZZZ0000Z2ZP; New Ref GS1/P/0001; tax by head cgst -900.00 and sgst_utgst -900.00. The recorder forwarded 102 requests and refused none.
- **Step 7b, raw PLPV.** 200, 60,396 B, one VOUCHER. The voucher carries:
  - CMPGSTIN `08ZZZZZ0000Z1ZQ`, CMPGSTREGISTRATIONTYPE Regular, CMPGSTSTATE Rajasthan;
  - PLACEOFSUPPLY and STATENAME Rajasthan, GSTREGISTRATIONTYPE Regular, PARTYGSTIN `08ZZZZZ0000Z2ZP`;
  - ISINVOICE Yes, ISELIGIBLEFORITC Yes, ISGSTOVERRIDDEN No;
  - **VCHGSTSTATUSISUNCERTAIN Yes, ISINCLUDED No**, ISAPPLICABLE Yes, every other VCHGSTSTATUS flag No.

  Both ledger-entry lists hold the same four entries: the party 11800.00 and Purchase - Goods -10000.00, plus the raw tax lines **Input CGST -900.00** and **Input SGST -900.00**, each ISDEEMEDPOSITIVE Yes. The party's bill allocation in this native export carries AMOUNT only, with no NAME or BILLTYPE.
- **gst_status reports Tally's own flag.** As read from the raw PLPV answer, Tally holds `VCHGSTSTATUSISUNCERTAIN` Yes and `VCHGSTSTATUSISINCLUDED` and `VCHGSTSTATUSISEXCLUDED` No on No. 57 itself. Step 6's `uncertain` is that flag as Tally stores it, read from the voucher, not a judgement ComplyEaze Bridge makes.
- **Step 8.** The marks are equal before and after the reads (M 228, V 65), and the raw MARKS equal the tool's.

## 628-C0b: the copies and what each read sees
| voucher | keyed | Tally on screen | `purchase_register` | `gst_status` |
|---|---|---|---|---|
| C0 No. 57, 2-Dec | as above | — | row, New Ref GS1/P/0001 | uncertain |
| K1 No. 58, 3-Dec | a second copy of GS1/P/0001 dated 1-12-2025 | at the bill name: **"'GS1/P/0001' has been used earlier!"**, "Press any key to continue" (its only choice); **saved** | row, same reference, date and GSTIN; bill **Agst Ref** GS1/P/0001 | uncertain |
| K2 No. 60, 4-Dec | GS1/P/0002, marked Optional (Ctrl+L, "Status: Optional") | no message | row, optional true | optional (flags not_in_return) |
| K3 No. 59, 5-Dec | GS1/P/0003, saved, then cancelled (Alt+X, "Cancel ? Yes or No": Yes; no reason field asked) | Day Book "(cancelled)" | side list: Purchase without a Duties & Taxes entry, cancelled true | cancelled (flags included) |
| K4 Journal 5, 6-Dec | Rent 5,000 + CGST 450 + SGST 450 / Supplier B 5,900, New Ref GS1/J/0004 | no supplier invoice or reference field offered | side list: other types touching Duties & Taxes | — |
| K5 Payment 3, 8-Dec | Supplier B 11,800 Agst Ref GS1/P/0001 / Cash | no reference field offered; "Negative Cash! (-)12,230.00", continued once | in no list | — |

- **K1 was saved as Agst Ref, not New Ref.** It was keyed as New Ref named GS1/P/0001, and the warning's one key kept the name. The saved voucher's party allocation reads **Agst Ref GS1/P/0001**, with C0's bill date 20251202 and BILLID 12 (R1, R4 and the register). Tally attached the second copy to C0's bill. K5's Pending Bills list therefore shows **one** row, GS1/P/0001 of 2-Dec-25 at 23,600.00 Cr (C0 and K1 together), beside GS1/J/0004 at 5,900.00 Cr. The optional K2 and the cancelled K3 are not listed.
- **The marks.** Each voucher moved ALTVCHID by 1, and so did **the cancel of K3** (68 → 69). From C0B to step 8: V 65 → 71; M 228 throughout. The calls and reads wrote nothing: M 228 and V 71 at step 11, and the raw MARKS read the same.
- **The year register** (step 9b): complete, 60 rows, 70 vouchers observed, other types 8, Purchases without Duties & Taxes 1 (K3). Neither register call was refused. The recorder forwarded 216 requests and refused none.

## The raw read shapes (C0b step 10b)
| read | answer | vouchers | bytes per voucher | the recorder's voucher window, same vouchers |
|---|---|---|---|---|
| R1 narrow, December | 56,250 B, 0.81 s | 6 | 9,375 | 291,376 B, the same 6 |
| R3 native, December | 327,668 B, 0.37 s | 6 | 54,611 | — |
| R4 narrow, year | 693,816 B, 0.46 s | 70 | 9,912 | 2,420,424 + 1,853,676 B, the same 70 |
- **R1 and R4** carry each party allocation's NAME, BILLTYPE, BILLDATE and BILLID.
- **R3 (native)** carries AMOUNT only on every party allocation: the reference's §9.19 gap, reproduced on 7.2. Its `LEDGERENTRIES.LIST` holds nothing that `ALLLEDGERENTRIES.LIST` lacks, and the Journal and the Payment have no `LEDGERENTRIES.LIST`.
- **The cancelled K3** keeps its REFERENCE in every read, but no party name, no GSTIN and one ledger entry.
- No read had a LINEERROR, and there was no dialog before or after any send.

## Files
- `c0/` and `c0b/`, each with:
  - `tool-answers/`: every tool answer whole, plus the `initialize` answer of each launch;
  - `answers/`: the raw reads with their response headers;
  - `recorder/`: the recorder's recording folder whole, with `manifest.tsv`;
  - `notes.txt`: one line per step in UTC, with every message's words.
- `client-log.txt` and `send-log.txt` cover both items. `SCREENSHOTS.txt` lists the captures and the two backups (C00, C0B) by name, bytes and sha256; they stay on the lab PC.

## Not measured
Tally's GST report for December was not opened, because opening a GST return view can raise the voucher mark. So the reason Tally gives for `uncertain` is not read here. The voucher's raw fields above are recorded so that it can be read from the evidence.
