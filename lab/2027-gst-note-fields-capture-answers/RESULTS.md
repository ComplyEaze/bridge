# Results: #2027 note fields, 28 Oct 2026. On TallyPrime 7.2 the native voucher read carries every field #2027 asks for at the voucher's top level:
- Place of Supply, state and registration type;
- the note's original invoice number and date in `REFERENCE` / `REFERENCEDATE`;
- the reason in `GSTNATUREOFRETURN`;
- the buyer's or supplier's note number and date in `VATPARTYTRANSRETURNNUMBER` / `VATPARTYTRANSRETURNDATE`.

The shipped request returns the reference pair, the party GSTIN and the bill names, but not the other fields. Adding fifteen voucher-level lists to the native request changed nothing in Tally's answer. The book was restored from BK1

The sheet is `lab/2027-gst-note-fields-capture` at 4cde7eb3adb752b4ca1494840a3415ec571aca38.
- **Sheet checks:** `SHA256.txt` 13 of 13; `gen_requests.py --check` ok; census ok 6.
- **Company:** BRIDGE NOTES LAB, 100084, GUID d0b6b9e8-9f2e-4863-b0f1-feac353fd9b0. This GUID is already in the merge gate's synthetic-UUID registry.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...df44, the same before and after).
- **Tally:** TallyPrime 7.2, Silver, Licensed.
- **Marks:** 215 / 4 at start and after BK1; 215 / 8 after the four vouchers and unchanged across the reads; 215 / 4 after the restore.
- **BK1:** `TDBK1800_100084.001`, 38,476 B, sha256 e264a4cafef9d97dd275e36cd6025cc37dc18e8e7cfb1bb5881c120308db3f87.
- **Folder deviation:** an earlier sitting already used the sheet's folder name, so this sitting's data and backup folders are new ones under a separate folder.

## What was keyed (step 2), each saved with no message
| voucher | screens and typed values | V after |
|---|---|---|
| SV2 Sales 3-Jun-25 | BRIDGE Customer A; Svc Sales 5% 1,000.00, CGST/SGST 25.00 each (computed); Party Details taken from the party (Maharashtra, Regular, its GSTIN, POS Maharashtra); bill New Ref (the default) SYN-BILL-S2 | 5 |
| CN2 Credit Note 4-Jun-25 | Receipt Details > Original Invoice No. SYN-ORIG-1, Date 12-5-2025; 400.00 + 10.00 + 10.00; "Provide GST details" Yes (already selected) > Additional Details : Local Sales - Taxable: Reason 01-Sales Return, Buyer's Debit Note No. SYN-NOTE-1, Date 13-5-2025; bill defaulted to Agst Ref, New Ref SYN-BILL-C2 | 6 |
| DN2 Debit Note 5-Jun-25 | Dispatch Details > Original Invoice No. SYN-ORIG-2, Date 14-5-2025; Additional Details : Local Purchase - Taxable: Reason 04-Correction in Invoice, Supplier's Debit/Credit Note No. SYN-NOTE-2, Date 15-5-2025; bill defaulted to Agst Ref, New Ref SYN-BILL-D2 | 7 |
| PV2 Purchase 6-Jun-25 | Supplier Invoice No. SYN-SUP-1, Date 16-5-2025; 1,000.00 + 25.00 + 25.00; no "Provide GST details" line; bill New Ref, name pre-filled "SYN-SUP-1", typed SYN-BILL-P2 | 8 |

Tally's current date followed the voucher dates (to 6-Jun-2025). It was set back to 2-Jun-2025 at the Gateway before the shut.

## Where each field sits (step 4)
| field (#2027) | native read (`PLPVA`, both answers) | shipped `Bridge Agent Vouchers` (a07) |
|---|---|---|
| Place of Supply | `/PLACEOFSUPPLY` Maharashtra (all four), PARTIAL (not keyed) | absent |
| party State | `/STATENAME` Maharashtra; also `CONSIGNEESTATENAME`, `CMPGSTSTATE` | absent |
| GST registration type | `/GSTREGISTRATIONTYPE` Regular | absent |
| party GSTIN | `/PARTYGSTIN` | `/PARTYGSTIN` |
| original invoice no. (notes) | **`/REFERENCE`** SYN-ORIG-1 / SYN-ORIG-2 (`ORIGINALVCHNUMBER` empty) | `/REFERENCE` |
| original invoice date (notes) | **`/REFERENCEDATE`** 20250512 / 20250514 (`ORIGINALVCHDATE` empty) | `/REFERENCEDATE` |
| reason for issuing note | **`/GSTNATUREOFRETURN`** "01-Sales Return" / "04-Correction in Invoice" (empty on SV2 and PV2) | absent |
| buyer's / supplier's note no. | **`/VATPARTYTRANSRETURNNUMBER`** SYN-NOTE-1 / SYN-NOTE-2 | absent |
| buyer's / supplier's note date | **`/VATPARTYTRANSRETURNDATE`** 20250513 / 20250515 | absent |
| supplier invoice no. and date (control) | `/REFERENCE` SYN-SUP-1, `/REFERENCEDATE` 20250516 | the same |
| bill names | not present (each party entry's `BILLALLOCATIONS.LIST` holds one child, no bill name) | `/ALLLEDGERENTRIES.LIST/BILLALLOCATIONS.LIST/NAME` |

- **The fifteen added lists:** the candidate request (the native read plus fifteen `NATIVEMETHOD` lines) answered byte-for-byte equal to the committed native read: 232,288 B, sha256 5086cafe....
  - Every one of the fifteen lists is an empty placeholder on every voucher in both reads, so each is "not expanded or empty: undetermined".
  - No #2027 field needs them: all were found at top level.
- **a07 ledger entries:**
  - Sales and purchase ledgers: `GSTOVRDNTAXABILITY` Taxable, `GSTOVRDNTYPEOFSUPPLY` Services, `GSTOVRDNISREVCHARGEAPPL` Not Applicable, `GSTCLASS` Not Applicable.
  - Also on those ledgers: `GSTOVRDNNATURE` and `GSTHSNNAME` empty, and five `RATEDETAILS.LIST` (CGST 2.50, SGST/UTGST 2.50, IGST 5, Cess Not Applicable, and one more).
  - Party and tax ledger entries carry one empty `RATEDETAILS.LIST`.
- **Counts:** each native answer 4 rows (3 to 6 June, one per day); each a07 answer 1 row.

## Restore (step 8, on the recorded RESTORE word)
- BK1 was restored by C6 with the two expected prompts (Accept ? Yes; folder 100084 exists: Replace).
- Marks after: 215 / 4. list_companies shows exactly the company. The Day Book for 3 to 6 June 2025 is empty.

**Confidence:** one voucher per kind, PARTIAL until repeated. Place of Supply, state and registration type equal the company's own, so they are attributed by element name only.

## Files
- `answers/` (tool, 000 to 010).
- `raw/`: native1, native2-lists, a07 x4, each with headers.
- `analyse.py` and `analysis.txt`.
- `send-log.txt`.
- `client-log.txt`.
- `restore-plan.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures and BK1 by sha256; they stay on the lab PC.
