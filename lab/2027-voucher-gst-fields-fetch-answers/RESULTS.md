# 2027 step 2: voucher GST fields by FETCH, the shipped `vouchers` request and the same request with six words added (Part of #2027)

Sheet `lab/2027-voucher-gst-fields-fetch` at 7be9ae07. Build: master 609776ae, `bridge_mcp.exe` sha256 4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44, the same before and after. Company: BRIDGE NOTES LAB (synthetic), restored from BK1 at the end. TallyPrime 7.2 Silver. 30 Oct 2026, 10:15 to 10:50 UTC. One sitting, one voucher per kind, so the result is PARTIAL until it is repeated.

## What was keyed
On Tally's screens, by hand, on the run lead's grant:
- One ledger, `BRIDGE IGST`, under Duties & Taxes, with Type of Duty/Tax GST and Tax type IGST. Every other field was left on its default; the percentage of calculation was offered as 0 % and kept.
- Five vouchers:

| day | voucher | what was typed |
|---|---|---|
| 3 June | SV2, Sales | 1,000.00 + CGST 25.00 + SGST 25.00; bill SYN-BILL-S2 |
| 4 June | CN2, Credit Note | original invoice SYN-ORIG-1 / 12-5-2025; reason 01-Sales Return; buyer's note SYN-NOTE-1 / 13-5-2025; 400.00 + CGST 10.00 + SGST 10.00; bill SYN-BILL-C2 |
| 5 June | DN2, Debit Note | original invoice SYN-ORIG-2 / 14-5-2025; reason 04-Correction in Invoice; supplier's note SYN-NOTE-2 / 15-5-2025; 400.00 + CGST 10.00 + SGST 10.00; bill SYN-BILL-D2 |
| 6 June | PV2, Purchase | supplier invoice SYN-SUP-1 / 16-5-2025; 1,000.00 + CGST 25.00 + SGST 25.00; bill SYN-BILL-P2 |
| 7 June | SV3, Sales, inter-state | Place of Supply set to Karnataka on Party Details (the field took the value); 1,000.00 + BRIDGE IGST, which Tally computed as 50.00; bill SYN-BILL-S3 |

Every mark read as expected:

| after | M | V |
|---|---:|---:|
| step 1 (M0, V0) | 215 | 4 |
| BK1 | 215 | 4 |
| the ledger | 216 | 4 |
| SV2 | 216 | 5 |
| CN2 | 216 | 6 |
| DN2 | 216 | 7 |
| PV2 | 216 | 8 |
| SV3 | 216 | 9 |
| step 9 (M1, V1) | 216 | 9 |
| the restore | 215 | 4 |

## Results
1. **The candidate carries all six fields on every voucher.** Each is a direct child of `ENVELOPE/BODY/DATA/COLLECTION/VOUCHER`, exactly once, with the typed value.
   - CN2: `GSTNATUREOFRETURN` "01-Sales Return", `VATPARTYTRANSRETURNNUMBER` "SYN-NOTE-1", `VATPARTYTRANSRETURNDATE` "20250513".
   - DN2: "04-Correction in Invoice", "SYN-NOTE-2", "20250515".
   - SV2, PV2 and SV3: the three note fields are present but empty (an open and a close tag, with no text).
   - Every voucher reads `GSTREGISTRATIONTYPE` "Regular".
   - `PLACEOFSUPPLY` and `STATENAME` read "Maharashtra" on SV2, CN2, DN2 and PV2.
   - Attributes: `TYPE="String"`, except `VATPARTYTRANSRETURNDATE`, which is `TYPE="Date"`.
   - No value has a leading or trailing space.
2. **The six words added only those elements.** For each of the five days, the candidate with the six elements cut out equals the control:
   - The size check holds: control + 2 × characters cut = candidate. The characters cut were 395, 428, 437, 395 and 393.
   - All three layers read equal on all five days: (a) byte for byte; (b) byte for byte with `CMPINFO` cut; (c) the parsed trees, ignoring the six.
   - None of the six appears as a voucher child in any control.
   - The two reads of the 4 June control are byte-equal, so two reads of the same day can be equal.
3. **SV3 tells Place of Supply apart by value.** `PLACEOFSUPPLY` reads "Karnataka" where the company's and the party's state is Maharashtra. `STATENAME` reads "Maharashtra", the party's state, so it stays attributed by element name only (PARTIAL).

The built tool (`vouchers`, 3 to 7 June) returned 5 rows, one per day, matching the controls. Its answer is kept as the control of what the tool returns today. Every raw answer is UTF-16LE, with no byte-order mark, CRLF line ends and one space of indent per level, as on 28 Oct. Every raw read answered 200 in under 0.4 s, and every candidate was followed by a capture of the Gateway with no dialog and `GET /status` 200.

## Restore and close
- BK1 (38,476 B, sha256 b9f239b3…) was restored by Replace, on the run lead's standing RESTORE word.
- After the restore: M 215 / V 4, no `BRIDGE IGST`, and the Day Book for 3 to 7 June is empty.
- Tally's current date was set back to 2-Jun-2025 (D0) before the restore.
- C10: the company was shut, one `tally.exe` owns 9001, and `GET /status` returned 200.

## Notes and deviations
- **The company's GUID is not in the merge gate's synthetic registry.** The 28 Oct notes said it was; on master it is not. The run lead confirmed the book is a lab book. The GUID appears in the tool answers and raw answers, but not in this file or the notes.
- The ledger's first name keying dropped its space ("BRIDGEIGST"). It was corrected on the same screen before saving, with nothing saved in between.
- The Place of Supply sub-screen of SV3, after Karnataka was keyed, was read on the desktop view but not window-captured. The answers record the value.
- On CN2, the bill name Tally offered was typed over without being read.
- Not covered (the sheet's list): an unregistered party, an export, a note without an original invoice, a party registered in another state, Gold, another release.
