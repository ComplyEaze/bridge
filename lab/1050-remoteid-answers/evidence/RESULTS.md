# 1050-C: can a read reach a voucher by the REMOTEID ComplyEaze Bridge sent? (Windows lab, 12 Oct 2026)

Requests: lab/1050-remoteid-probe 00a8b75b. All five request files matched SHA256.txt. c2–c5 were filled with `fill.ps1`, which replaces only the placeholder (UTF-16LE with BOM); `checkfill.py` confirms each filled file equals its template byte for byte apart from the placeholder.

| Filled file | Bytes | sha256 |
|---|---|---|
| c2-object-name-guid.xml (N1 GUID) | 1124 | f6f9b8bf… |
| c3-object-name-sent.xml (sent REMOTEID) | 1106 | da41009e… |
| c4-object-guid-guid.xml (N1 GUID) | 1124 | f01d1e15… |
| c5-object-guid-sent.xml (sent REMOTEID) | 1106 | 86c5cbd6… |

Full hashes are in `filled/FILLED-SHA256.txt`.

Book: synthetic BRIDGE IDENTITY LAB (100056), loaded alone at every step. No Bridge call, no post, no keying.

Values used:
- N1 GUID: dcac52b6-2168-43fa-be2a-95b1488c11b4-00000002.
- Sent REMOTEID: 5da1d4d5-fc22-424f-9955-776d9c8ee090 (1050-B `sent-remoteid.txt`).

## Steps
1. **I3, then restore I1.** restore-plan.txt was written first.
   - I3 (holds K): 26,492 B, sha256 ed0937c22c24300e7a878f4797dcde9af774cdc051bae920f1a59f4b3b665149, 10:42:26Z.
   - Restore of 1050-B's I1 over the company: the list showed BRIDGE IDENTITY LAB (100056) 12-Oct-26 (3:47 pm) and the Restore Data Path was the Tally data folder, both as planned.
   - Prompts: "Accept ?" Yes; "Company data folder (100056) already exists … Replace / Skip" Replace (`s1-restore-I1-*.jpg`). N1 was then in the book.
2. **N1 reads:** n1-c1 … n1-c5 (10:44:05Z–10:44:13Z).
3. **Restore I3 by the same plan.** The list showed 12-Oct-26 (4:12 pm), as planned, with the same two prompts (`s3-restore-I3-*.jpg`). K was then in the book. Reads k-c1, k-c3, k-c5 (10:45:45Z–10:45:50Z).

No other Tally dialog appeared in the run. Every request was sent once; each got an answer within 1 s.

## Answers (encoding UTF-16LE without BOM; read by structure)

| Answer | HTTP / curl | Bytes | sha256 | Content (parsed) |
|---|---|---|---|---|
| n1-c1.xml | 200 / exit 0 | 42 | e66a1d1d… | `<ENVELOPE></ENVELOPE>`: 0 VOUCHERIDENTITY lines |
| n1-c2.xml | 200 / exit 0 | 184 | 098eea1e… | `<ERRORMSG>Could not find Voucher:dcac52b6-2168-43fa-be2a-95b1488c11b4-00000002!</ERRORMSG>` |
| n1-c3.xml | 200 / exit 0 | 166 | cfd5d32f… | `<ERRORMSG>Could not find Voucher:5da1d4d5-fc22-424f-9955-776d9c8ee090!</ERRORMSG>` |
| n1-c4.xml | 200 / exit 0 | 184 | 098eea1e… | same as n1-c2 (byte-identical) |
| n1-c5.xml | 200 / exit 0 | 166 | cfd5d32f… | same as n1-c3 (byte-identical) |
| k-c1.xml | 200 / exit 0 | 42 | e66a1d1d… | `<ENVELOPE></ENVELOPE>`: 0 lines |
| k-c3.xml | 200 / exit 0 | 166 | cfd5d32f… | "Could not find Voucher:5da1d4d5-…!" |
| k-c5.xml | 200 / exit 0 | 166 | cfd5d32f… | "Could not find Voucher:5da1d4d5-…!" |

## Reading

Every shape failed its own control, so none of these answers says anything about the sent REMOTEID:
- **c1 (the report):** returned no VOUCHERIDENTITY line at all, in both states. The book holds 2 vouchers dated 1–31 July (the seed and N1 or K; 1050-B's window reads returned 2 in the same states). The GUID column printed nothing either, so the report shape itself returned no rows. The `$RemoteID` value of N1 and K is **not measured**. FAILED, not "not found".
- **c2 and c4 (the controls):** an object export of a Voucher with ID TYPE="Name" or TYPE="GUID" set to N1's GUID answers `ERRORMSG Could not find Voucher:<GUID>!` while N1 is in the book. These shapes cannot address a voucher by its GUID on this release, so the c3 and c5 errors (by the sent REMOTEID) prove nothing. FAILED controls.
- **c3 and c5:** the same ERRORMSG for the sent REMOTEID, with N1 in the book and with K. Since their controls failed, this does not show the value is absent.

**Overall:** no request in this set reaches N1 by the sent REMOTEID; none reaches N1 even by its GUID. The question stays open, as unmeasured. The answers are kept as failed.

Not done (not in the steps): any variant request, and any repeat.
