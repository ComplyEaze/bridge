# Results: row 2162-S3a, a Sales invoice's New Ref to a bill-wise customer with no other bill of that name, 3 Nov 2026

**The invoice stored one New Ref, named by its number, for the party amount, and it stands as the customer's one open bill.** The read-back shows one voucher. Its party leg (`BRIDGE Walk-in BW`, -1051.00) holds one `BILLALLOCATIONS.LIST` with `NAME` BP/26-27/0047, `BILLTYPE` New Ref, `AMOUNT` -1051.00. Bills Receivable shows one bill for that customer: BP/26-27/0047, 1-Aug-26, closing -1051.00. Every EXPECT held but one count.

**NOTE (the one count):** the sheet expects 0 `BILLALLOCATIONS.LIST` elements under every other leg; there are 5. Each non-party leg carries one empty, whitespace-only `<BILLALLOCATIONS.LIST>` element. Counted the other way, allocations with a `NAME` child on non-party legs: 0. The row stopped there and went on to step 6 on the recorded word. Whether the code's check counts these empty elements is for the code's reader. [verified for the counts; the code's reading is not this row's]

Sheet: `lab/2162-s3a-sheet` at d99ef82e, all 9 files equal to `SHA256.txt`, every request file UTF-16LE with a BOM. `M-marks.xml` is byte-equal to S2's marks request. No Bridge build. BRIDGE PILOT LAB (synthetic), loaded alone. Tally's screen shows TallyPrime **Silver**; the release, 7.2, is from `tally_status` on the same tally.exe earlier in the sitting. The Gateway's current date was 15-Sep-2026, after 1 Aug 2026. The Gateway also showed "(e) Data exceptions exist" beside the company, before and after, as in S2. [verified]

How the requests were sent: `run.py` as written, from a copy of the sheet folder with every file hash-equal, URL `http://127.0.0.1:9001`. Each answer was checked by an XML parser after its step (`check.py`, `evidence/check.txt`). One observation on `run.py`: its "Unknown Request" guard decodes an answer as UTF-8 unless the answer has a byte-order mark, and Tally's answers here have none. So an Unknown Request in UTF-16 would not be caught by it. Every answer here parsed as the expected kind, so it did not matter. The script was not changed. Four `bridge_mcp` processes ran idle under the desktop client, as in S1 and S2; whether they sent anything to Tally was not measured. [verified; the processes unverified]

## Steps and answers
| step | file | bytes | sha256 | counts |
|---|---|---:|---|---|
| s1 | M | 3,394 | 8a0c4268… | ALTVCHID 68, ALTMSTID 245 |
| BK | | | | one file, 90,051 B, sha256 4091d87c…, 10:04 local, kept on the lab PC |
| s2 | R | 3,022 | 947fbf36… | 0 `VOUCHER` elements: the number is unused |
| s2 | B | 46 | 8d37111f… | `<ENVELOPE></ENVELOPE>`: 0 `BILLFIXED` groups for `BRIDGE Walk-in BW`, 0 in all |
| s3 | L | 538 | 1b72152a… | CREATED 1, ALTERED 0, ERRORS 0, EXCEPTIONS 0, LASTVCHID 0, no LINEERROR |
| s4 | I | 540 | fdcd8435… | CREATED 1, ALTERED 0, ERRORS 0, EXCEPTIONS 0, LASTVCHID 45, no LINEERROR |
| s5 | R | 95,834 | e8a4c191… | below |
| s5 | B | 510 | 748ddd36… | 1 `BILLFIXED` group for `BRIDGE Walk-in BW`: `BILLREF` BP/26-27/0047, `BILLDATE` 1-Aug-26, `BILLCL` -1051.00; 1 in all |
| s5 | M | 3,396 | 17c5bde5… | ALTVCHID 69, ALTMSTID 246: both above s1's |
| s6 | M | 3,394 | 8a0c4268… | ALTVCHID 68, ALTMSTID 245: **byte-identical to s1's** |

After s3 a capture of Tally's window showed the Gateway, no dialog. [verified]

## s5 R, the read-back
- 1 `VOUCHER`: BP/26-27/0047, dated 20260801, `ALTERID` 69, GUID ending -0000002d.
- `VCHGSTSTATUSISINCLUDED` Yes, `VCHGSTSTATUSISUNCERTAIN` No, `VCHGSTSTATUSISEXCLUDED` No.
- 6 `ALLLEDGERENTRIES.LIST` legs; 1 for `BRIDGE Walk-in BW`, `AMOUNT` -1051.00.
- On that leg, 1 `BILLALLOCATIONS.LIST` direct child: `NAME` BP/26-27/0047, `BILLTYPE` New Ref, `AMOUNT` -1051.00.

[verified]

**Counted both ways, per leg:**

| leg | `BILLALLOCATIONS.LIST` elements | of them whitespace-only | with a `NAME` child | `TAXBILLALLOCATIONS.LIST` elements (all whitespace-only) |
|---|---:|---:|---:|---:|
| BRIDGE Walk-in BW | 1 | 0 | 1 | 1 |
| BRIDGE Svc 998313 5% | 1 | 1 | 0 | 1 |
| BRIDGE Svc 998314 5% | 1 | 1 | 0 | 1 |
| BRIDGE CGST 2.5% | 1 | 1 | 0 | 1 |
| BRIDGE SGST 2.5% | 1 | 1 | 0 | 1 |
| BRIDGE Round Off | 1 | 1 | 0 | 1 |

So: 5 `BILLALLOCATIONS.LIST` elements on non-party legs, all whitespace-only, and 0 allocations with a `NAME` on them. Every leg, the party's included, also carries one empty `TAXBILLALLOCATIONS.LIST`. [verified]

## Restore
Tally's Restore screen, from the backup's folder. The list showed BRIDGE PILOT LAB at 3-Nov-26 10:04 am; it was selected alone and captured before accepting. Two prompts came, `Accept ?` (Yes) and the folder-exists prompt naming the company's own number (Replace), and no other. s6's marks are byte-identical to s1's. [verified]

## Decides
By content, as expected: one New Ref named by the number for the party amount, no allocation with content on any other leg, and Bills Receivable shows it as the customer's one open bill. By the sheet's element count, the non-party legs hold 5 empty `BILLALLOCATIONS.LIST` elements. Which of the two the check at agent_import_invoice.rs:1476 sees decides whether this is the sheet's "As expected" or its "Any other allocation" line. That is left to the code's reader. [hypothesis until then]

## Kept
- Every answer as Tally sent it (none holds the company number, a GSTIN or a PAN shape).
- `run.py`'s logs, the check output, the restore plan, `check.py`.
- On the lab PC only: the backup, the captures and the data-folder listings, listed in `kept-local.txt`.

## The book now
BRIDGE PILOT LAB restored, marks byte-identical to s1. Shut, no company loaded, one tally.exe on 9001.

**Confidence:** one book, one invoice, one sitting, 7.2: PARTIAL.
