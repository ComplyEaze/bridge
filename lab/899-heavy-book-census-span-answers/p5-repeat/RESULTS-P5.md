# Results: #899, the S1 repeat (P5) on the heavy client book (THE BOOK), 29 Oct 2026, second sitting. **S1 (250 ids) answered in 192.8 s, against 487.0 s in the first sitting: still about 9.6 times the 20 s request limit. With the `UDF` prefix declared, the answer parses: 39 rows by structure.**

| read | request bytes | curl exit, status | answer bytes = CONTENT-LENGTH | time_connect | time_starttransfer | time_total | answer sha256 |
|---|---|---|---|---|---|---|---|
| m0 (marks) | 1,026 | 0, 200 | 3,458 = 3,458 | 0.003 s | 0.009 s | 0.018 s | a4fc75b3d5287a7f1417f8e01d9d586956ddac3ebb5ed3275a82856762ab748b |
| s1 (census, width 250) | 1,588 | 0, 200 | 139,664 = 139,664 | 0.005 s | 192.770 s | 192.772 s | 40dd377132559bc402f1ed980adcdab8c4d17ce0a267a20b2b9d045a24a73023 |
| m1 (marks) | 1,026 | 0, 200 | 3,458 = 3,458 | 0.001 s | 0.003 s | 0.005 s | a4fc75b3d5287a7f1417f8e01d9d586956ddac3ebb5ed3275a82856762ab748b |

**The two sittings side by side (S1, the same 250-id span from V0/8, the same window):**

| | first sitting (09:10Z) | P5 repeat (12:50Z) |
|---|---|---|
| time to first byte | 487.039 s | 192.770 s |
| time_total | 487.042 s | 192.772 s |
| answer | 139,664 B, sha256 40dd3771... | 139,664 B, the same sha256 (byte-equal) |
| census_read | did not parse (unbound prefix `UDF:`) | exit 0 with the prefix declared |
| marks | M1 = M0 | M1 = M0, and both equal to the first sitting's (same sha256) |

**Reading the answer by structure** (`census_read_udf.py`: the sheet's `census_read.py` with one change, `xmlns:UDF` declared on ENVELOPE before parsing; rows are still the VOUCHER elements under ENVELOPE/BODY/DATA/COLLECTION):
- `census ok bytes 139664 rows 39 rows_missing_a_field 0 rows_outside_span 0 repeated_guids 0 min_alterid_offset 208 max_alterid_offset 250`.
- **39 rows**, not the first sitting's text count of 40 (that count, `<VOUCHER[ >]`, also caught one tag that is not a row).
- **Bytes per row:** 139,664 / 39 = 3,581 B on the wire (UTF-16LE), **87.4 % of 4,096**; about 1,791 B in UTF-8. This includes the fixed envelope (one span only, so no marginal figure).
- **Rows per id:** 39 / 250 = 0.156.
- **Seconds per row:** 192.77 s / 39 = 4.94 s per row, or 0.77 s per id of the span.
- **At these rates:** a full 8,192-row span would be about 29.3 MB, 87 % of the 33,554,432-byte cap. The time is dominated by the first byte, not the rows, so a rate per row says little about a wider span.

**By the sheet's reading:** "S1 above 20 s: the product cannot read this book in 250 ids" holds again. The ladder stopped after S1; S2, S3 and `plan` were not run. "S3's marginal bytes per row" cannot be read; the S1 figure, 3,581 B per row with the envelope, sits between 3.4 KB and 4.0 KB ("the headroom is thinner than documented"), on one span.

**The time moved 2.5 times between two sittings on byte-equal answers with equal marks:** the first byte, not the size, carries the cost, and it varies with Tally's state (the first sitting's book had been loaded by the owner just before; this one was loaded by the lab and its load took about 90 s). One more sitting would say whether 190 to 490 s is the range.

Reads only. The marks M1 equal M0 (and equal the first sitting's). THE BOOK was the only company loaded. Its data path was set back to the lab's default data folder at the end.

**Who acted.** The lab, on the owner's word for this row (reads only on THE BOOK), under the owner's standing word of 27 Oct, with the owner at the PC. In this sitting Tally loaded THE BOOK with no login prompt; in an attempt earlier the same day it asked for a login, and the lab left that box without logging in (no credential was typed or recorded).

**Not measured:** S2, S3 and any other width; a narrower date window; an unspanned read (the sheet has no such step); the product's own parser on this answer; a third sitting.

**Confidence:** two sittings, one span, PARTIAL.

## Files (in `p5-repeat/`)
`RESULTS-P5.md`; `notes.txt` (UTC); `evidence/times.txt` (curl's fields, CONTENT-LENGTH, sha256, the census_read line, each `/status`); the step-0 and close checks (netstat cut to the port-9001 lines; `substitutions.txt`); `scripts/census_read_udf.py`; `SCREENSHOTS.txt`; `SHA256.txt`. The requests, answers, `company.txt`, `dates.txt` and the local marks and span files stay on the lab PC until step 9.
