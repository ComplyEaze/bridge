# Results: #2027 row R1, the six GST and note elements through the window read, 3 Nov 2026

**Yes. With `include_gst_details`, the window read returns all six elements once each, as direct children of VOUCHER, with the native capture's values. TYPE is `String` for five and `Date` for `VATPARTYTRANSRETURNDATE`, as on 30 Oct. Nothing else changes: the request differs from the plain one by the six words alone, and the answer with the six cut equals the plain answer. No `not_reported`, no refusal, no stall.**

Build: head 1cdc0c41 on fix/2027-voucher-gst-note-fields, run 41556008615, artifact 13195362376, zip digest equal, `bridge_mcp.exe` sha256 d3c5b87ab3b2b0388bf6b88ddd20e33ea4967e7eb50fc57d15e213da3f05872e, equal to the package's smoke file, before and after. BRIDGE NOTES LAB, synthetic, TallyPrime 7.2 Silver, licensed, loaded alone. Posting off, new data folder. The comparison build is the 0.7.0 release, Windows package digest equal to the release's, `bridge_mcp.exe` 48d2f743..., its own process and data folder.

**The relay** is row R0's, sha256 38a2115b... checked first, copied with one addition: each request's and answer's bytes saved beside the log by connection and request number (`mkrelay.py`). Nothing else changed. All four steps and step 5 went over one connection, 114 requests. 0.7.0 used a second connection afterwards. [verified]

## The seed
Backup taken first: 38,476 B, sha256 0556093a..., local. Marks V 4, M 215.

The four vouchers of the capture, by direct XML, one at a time. The first shape, with `ALLLEDGERENTRIES.LIST`, answered `EXCEPTIONS` 1 and created nothing for all four; the marks did not move. With `LEDGERENTRIES.LIST` each answered `CREATED` 1, V 4 to 8. Read back natively with the capture's request: `GSTNATUREOFRETURN`, `VATPARTYTRANSRETURNNUMBER` and `VATPARTYTRANSRETURNDATE` stuck on both notes, with the capture's values. Nothing was keyed. [verified]

Seeded rows set the test up and are no evidence of their own shape.

## Steps
| step | call | seconds | `state` | rows | window request | answer bytes, each of a pair |
|---|---|---:|---|---:|---|---:|
| 1 | `vouchers`, 3 to 6 June | 2.33 | complete | 4 | 1,740 B, plain FETCH | 248,914 |
| 2 | with `include_gst_details` | 2.20 | complete | 4 | 1,964 B, plain + six | 252,224 |
| 3 | with `include_gst_details` and `include_e_invoice` | 2.17 | complete | 4 | 2,012 B, plain + IRN three + six | 253,168 |
| 4 | step 2 again | 2.28 | complete | 4 | step 2's bytes | 252,224 |

No call errored. After each, `GET /status` 200 and Tally's screen at the Gateway with no dialog. Marks V 8, M 215 before step 1 and after step 4. [verified]

Two calls before step 1 never reached Tally: I started the relay without its mode word, so it exited and the build answered `endpoint_unreachable`. Those answers are in `run0/`. The steps above ran in a new process and data folder.

- **Step 1's window request is byte-identical to the 0.7.0 release's** on the same dates.
- **Step 2's differs by the six words alone,** appended to the FETCH after `REFERENCEDATE`.
- **Step 2's answer with the six cut equals step 1's,** parsed, CMPINFO cleared. Every pair of reads is byte-equal, and steps 2 and 4 give the same bytes. [verified]

## The six elements in the step 2 window answer
Each is a direct child of VOUCHER, once, on every voucher:

| voucher | PLACEOFSUPPLY | STATENAME | GSTREGISTRATIONTYPE | GSTNATUREOFRETURN | VATPARTYTRANSRETURNNUMBER | VATPARTYTRANSRETURNDATE |
|---|---|---|---|---|---|---|
| Sales 3-Jun | Maharashtra | Maharashtra | Regular | empty | empty | empty |
| Credit Note 4-Jun | Maharashtra | Maharashtra | Regular | 01-Sales Return | SYN-NOTE-1 | 20250513 |
| Debit Note 5-Jun | Maharashtra | Maharashtra | Regular | 04-Correction in Invoice | SYN-NOTE-2 | 20250515 |
| Purchase 6-Jun | Maharashtra | Maharashtra | Regular | empty | empty | empty |

TYPE `String` on the first five columns and `Date` on the last. Each text equals the native read-back's. The native answer carries no TYPE on these, so TYPE is the window read's own. Step 3's six are equal to step 2's, with IRN, IRNACKNO and IRNACKDATE once each besides. Every date reads YYYYMMDD; `voucher_party_note_date_invalid` did not occur. [verified]

**Against 30 Oct** (`lab/2027-voucher-gst-fields-fetch-answers` at 67039e2b, its four single-day candidate answers): on each of the four vouchers, the six elements are equal in count, TYPE and text. [verified]

**The tool's `voucher_gst`** in steps 2, 3 and 4: `place_of_supply`, `party_state` and `registration_type` present on all four, Maharashtra and Regular. On the notes, `note_reason`, `party_note_number` and `party_note_date` are present with the values above. On the Sales and Purchase those three are `absent`. Step 1 and 0.7.0 return no `voucher_gst`. [verified]

## Step 5, inter-state Sale
Seeded on top: one party, `BRIDGE Customer KA` (Karnataka, Regular, an invented GSTIN), and one ledger, `IGST` (GST, IGST, 5 %), by direct XML, `CREATED` 2. Then one Sales voucher on 7 June, 1,000.00 + IGST 50.00, `CREATED` 1. Marks V 9, M 217. Native read-back: PLACEOFSUPPLY and STATENAME Karnataka.

`vouchers` for 7 June with `include_gst_details`, `include_e_invoice` and `voucher_type` Sales: complete, 1 row, 2.25 s. The window request's FETCH is step 3's. Its filter is the date window alone; the type is applied by the build after the read. `voucher_gst.party_state` Karnataka, `place_of_supply` Karnataka, `registration_type` Regular. On Tally's screen, Party Details shows State Karnataka, GST Registration type Regular, Place of Supply Karnataka: equal. Left by Ctrl+Q with nothing saved. [verified]

## Also compared
The 0.7.0 answer is 2 B longer than step 1's because it ran after step 5's seed. Tally's in-memory counts in CMPINFO read `LEDGER` 35 and `CURRENCY` 5 instead of 28 and 4. Its four rows are as step 1's. [verified]

## Restore
Restored from the backup on Tally's screen, its hash checked first, with the two prompts expected. Catalogue, extent, the native read of 3 to 6 June and the marks are byte-identical to the reads before the seed: V 4, M 215, no voucher in the window, no `BRIDGE Customer KA` or `IGST`. Tally's current date stayed 2-Jun-2025. Company shut, no company loaded, one tally.exe on 9001. [verified]

## Limits
One book, one Tally, one sitting, vouchers seeded by XML. A party registered in another state is seen through Place of Supply only, both read Karnataka; a voucher whose two differ was not made.

**Confidence:** PARTIAL.
