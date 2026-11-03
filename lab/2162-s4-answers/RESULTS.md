# Results: row 2162-S4, a Sales type's guard against a number already in use, with case and leading zeros, 3 Nov 2026

**The guard holds on the Sales type, in the answer, and it ignores case; a dropped leading zero is a different number.**
- **b1**, the same number `BP/26-27/0051`: declined in the answer (`CREATED` 0, `EXCEPTIONS` 1, every other counter 0, no `LINEERROR` text), with no dialog.
- **b2**, `bp/26-27/0051`: declined the same way, answer byte-identical to b1's.
- **b3**, `BP/26-27/051`: **created a voucher**. Tally then held two invoices whose numbers differ only by a leading zero.
- **a, the number read:** it finds the seed under `bp/26-27/0051` and misses it under `BP/26-27/051`. Case is folded the same way by the read, the read-back filter and the guard; the dropped zero is missed by all three.
- **c**, `BP/26-27/0052`: posted, beside the seed. Two invoices that differ only by number are two vouchers.

No dialog appeared after any import. Both restores are byte-identical. [verified]

Sheet: `lab/2162-s4-sheet` at c5f69f97, folder `lab/2162-s4/`. All 19 files equal to `SHA256.txt`, every request file UTF-16LE with a BOM. `run.py` and `M-marks.xml` are byte-equal to S3b's. The files were copied, hash-equal, to a run folder. No Bridge build. BRIDGE PILOT LAB (synthetic), loaded alone. Tally's screen shows TallyPrime **Silver**; the release, 7.2, is from `tally_status` on the same tally.exe earlier today. The Gateway's current date read 15-Sep-2026 throughout, after 1 Aug 2026. "(e) Data exceptions exist" showed beside the company, as in earlier rows. Four `bridge_mcp` processes ran idle under the desktop client, as in S1 to S3b. Whether they sent anything to Tally was not measured. [verified; the processes unverified]

After every import a capture of Tally's window was byte-identical to the Gateway capture taken before the first import (`ca58f95e…`): no dialog. [verified]

## Steps and answers
| step | file | bytes | sha256 | counts |
|---|---|---:|---|---|
| s1 | M | 3,394 | 8a0c4268… | ALTVCHID 68, ALTMSTID 245 |
| BK | | | | one file, 90,051 B, sha256 d7a801e5…, 14:23 local, kept on the lab PC |
| s2 | N-0051, N-lc0051, N-z051, N-0052, R-0051, R-0052 | 3,022 each | 947fbf36… | 0 rows / 0 `VOUCHER` elements each: the numbers are unused |
| s3 | I-0051 (seed) | 540 | fdcd8435… | CREATED 1, ALTERED 0, ERRORS 0, EXCEPTIONS 0, LASTVCHID 45 |
| s3r | R-0051 | 94,478 | 928dbe08… | 1 voucher: `BP/26-27/0051`, 20260801, ALTERID 69, GUID …0000002d |
| s3r | N-0051 | 5,608 | cf5d48ac… | 1 row: `BP/26-27/0051`, 20260801, BRIDGE Sales, ISCANCELLED No, BRIDGEVCHISSALES Yes |
| s3r | M | 3,396 | 8feeba3e… | 69 / 245 (ALTVCHID above s1's) |
| s4 (a) | N-lc0051 | 5,608 | cf5d48ac… | **1 row**, `BP/26-27/0051` (the seed); byte-identical to s3r's N-0051 |
| s4 (a) | N-z051 | 3,024 | a36e6e43… | **0 rows** |
| s5 (b1) | I-0051b | 538 | 09a713e2… | **CREATED 0**, ALTERED 0, ERRORS 0, **EXCEPTIONS 1**, LASTVCHID 0, no LINEERROR |
| s5r | R-0051, N-0051, M | 94,478 / 5,608 / 3,396 | ed230775… / ca5387db… / 4010a771… | R: 1 voucher (the seed, ALTERID 69); N: 1 row; M 69 / 245 |
| s6 (b2) | I-lc0051 | 538 | 09a713e2… | **CREATED 0, EXCEPTIONS 1**: byte-identical to b1's answer |
| s6r | R-0051, R-lc0051, N-0051, N-lc0051, M | 94,478 ×2 / 5,608 ×2 / 3,396 | 4df285e0… (both R) / 5bff18b6… (both N) / 61561b61… | each R: 1 voucher, the seed; each N: 1 row, the seed; M 69 / 245 |
| s7 (b3) | I-z051 | 540 | 32e7a29b… | **CREATED 1**, ALTERED 0, ERRORS 0, EXCEPTIONS 0, LASTVCHID 46 |
| s7r | R-0051 | 94,478 | 0a7b7758… | 1 voucher: the seed (ALTERID 69) |
| s7r | R-z051 | 94,474 | f8bb3e17… | 1 voucher: `BP/26-27/051`, 20260801, ALTERID 70, GUID …0000002e |
| s7r | N-0051 / N-z051 | 5,608 / 5,606 | c59518d5… / 801e4fa8… | 1 row each: `BP/26-27/0051` / `BP/26-27/051`, both BRIDGE Sales, BRIDGEVCHISSALES Yes |
| s7r | M | 3,396 | 348d416e… | 70 / 245 |
| restore 1, rs1 | M | 3,394 | 8a0c4268… | 68 / 245: **byte-identical to s1's** |
| s3-2 (seed again) | I-0051 | 540 | fdcd8435… | CREATED 1, LASTVCHID 45: byte-identical to s3's answer |
| s3r-2 | R-0051, N-0051, M | 94,478 / 5,608 / 3,396 | 928dbe08… / cf5d48ac… / 8feeba3e… | byte-identical to s3r's three answers |
| s8 (c) | I-0052 | 540 | 32e7a29b… | CREATED 1, ERRORS 0, EXCEPTIONS 0, LASTVCHID 46 |
| s8r | R-0052 | 94,478 | a16f44b1… | 1 voucher: `BP/26-27/0052`, 20260801, ALTERID 70, GUID …0000002e |
| s8r | R-0051 | 94,478 | c87c07d5… | 1 voucher: the seed, ALTERID 69 |
| s8r | N-0052 | 5,608 | 35093aa5… | 1 row: `BP/26-27/0052`, BRIDGE Sales, BRIDGEVCHISSALES Yes |
| s8r | M | 3,396 | 7a5fe6a0… | 70 / 245 |
| s9 | M | 3,394 | 8a0c4268… | 68 / 245: **byte-identical to s1's** |

[verified]

- Repeat reads of the same request differ only in CMPINFO header counters (`LEDGER`, `TAXUNIT`, `VOUCHERNUMBERSERIES`), as in earlier rows; their rows are equal. For s3r against s5r this is checked line by line in `evidence/check.txt`. [verified]
- The answers to b3's create (s7) and c's create (s8) are byte-identical: each is CREATED 1, LASTVCHID 46, on a book holding only the seed. [verified]

## How the row ran, against the sheet
- **After b3 created a voucher:** as the sheet says, BK was restored, M checked equal to s1's, and the seed sent again before c. To keep run.py from overwriting s3's answer files, the second seed ran under the labels `s3-2` and `s3r-2`, and the post-restore marks under `rs1`. The sheet names no labels for that pass. [verified]
- **Restores:** both from Tally's Restore screen, from the backup's folder, the list showing BRIDGE PILOT LAB at 3-Nov-26 2:23 pm, selected alone and captured before accepting. Each brought two prompts, `Accept ?` (Yes) and the folder-exists prompt naming the company's own number (Replace), and no other. [verified]

## Decides
- **b1 declined in the answer:** the guard holds on the Sales type `BRIDGE Sales`, in-band, with no dialog. ADR item 12's first sub-item is measured. [verified]
- **a against b2 and b3:**
  - **Case:** the number read finds the variant (s4: 1 row), so the build refuses `bp/26-27/0051` before sending. The guard also declines it (b2). Safe both ways. [verified]
  - **Leading zero:** the number read misses `BP/26-27/051` (s4: 0 rows), and the guard does not decline it: b3 created a second invoice. Tally then holds `BP/26-27/0051` and `BP/26-27/051` as two Sales invoices on the same date and figures. By the sheet's Decides line, whether GST allows that is decided before any code. [verified for what Tally did; the GST question not this row's]
- **c created and R-0051 still holds 1:** two invoices that differ only by number are two vouchers in Tally, as the identity check assumes. [verified]

## Kept
- Every answer as Tally sent it (none holds the company number, a GSTIN or a PAN shape).
- `run.py`'s logs, the check output, the restore plan, `check.py`.
- On the lab PC only: the backup, the captures and the data-folder listings, listed in `kept-local.txt`.

## The book now
BRIDGE PILOT LAB restored, marks byte-identical to s1. Shut, no company loaded, one tally.exe on 9001.

**Confidence:** one book, one Sales type, one sitting, 7.2: PARTIAL.
