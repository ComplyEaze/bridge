# 1605-S1 — what Tally sends for "integrated with inventory", and FY 2025-26 vouchers, on BRIDGE STOCK LAB (21 Oct 2026). Synthetic only. Read only.

About 10 minutes. No Bridge build, no write, no setting changed. Same method, folder rules, PARSE snippet, RULES and STOP
list as item 697-S0A; only the files and steps differ.

**WHAT IT DECIDES.** (a) #1605: the tax-audit engine reads the company object's `ISINTEGRATED` to choose where stock
comes from, and treats a missing value as "not integrated". This records what TallyPrime actually sends for an
inventory book (present? `Yes`?). (b) Whether BRIDGE STOCK LAB holds vouchers in FY 2025-26, so the later live run of the tool (697-S2) can run there if
BRIDGE PILOT LAB has none.

**FILES** (`C:\bridge-lab\1605s1\requests\`; hashes in `SHA256.txt`, else STOP). `r01` is 697-S0A's file; `r02`, `s03`
and `c01` are the 697-S0A / 697-S1 files with only the company name changed to BRIDGE STOCK LAB (same length). The
census header ID equals its COLLECTION NAME.
| file | bytes | what it asks |
|---|---|---|
| r01-company-extent.xml | 2524 | the company list |
| r02-marks-stock-lab.xml | 976 | the two high-water marks |
| s03-company-object-stock-lab.xml | 896 | the company Object export: GUID, NAME, BOOKSFROM, ISINTEGRATED |
| c01-census-fy2025-26-stock-lab.xml | 1436 | GUID, ALTERID, DATE of every voucher dated 2025-04-01 to 2026-03-31 |

**STEPS** (BRIDGE STOCK LAB loaded alone on 9001)
1. Send `r01` (answer `1.xml`). EXPECT exactly one company, BRIDGE STOCK LAB; Education No. Record GUID, books-from.
2. Send `r02` (answer `2.xml`). Record ALTVCHID (M) and ALTMSTID; EXPECT M between 1 and 5,000 (else STOP and report M).
3. Send `s03` (answer `3.xml`). EXPECT one `/ENVELOPE/BODY/DATA/TALLYMESSAGE/COMPANY` with the step-1 GUID. Record EXACTLY
   the `ISINTEGRATED` element as sent (present or absent; its TYPE attribute and text), and the HEADER's fields.
   Also note what the company's own setting shows on screen (Alt+F3 / Company > Alter > "Integrate accounts with inventory"
   — LOOK only, change nothing).
4. Send `c01` (answer `4.xml`). Record the VOUCHER count, the earliest and latest DATE, bytes and seconds.
5. Send `r02` again (answer `5.xml`). EXPECT ALTVCHID = M.

**REPORT:** the table of steps, the ISINTEGRATED element verbatim with the screen's setting, the FY 2025-26 count and
date range, every answer file, the log and SHA256.txt. Confidence: one run, PARTIAL.

**Not established:** a company whose integration was never set (Tally may always send the field), the stock-items part
(a later request), Education mode, any client book.
