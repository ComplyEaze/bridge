# 697-S1 — the audit read's census, live, on BRIDGE PILOT LAB (21 Oct 2026). Synthetic company only. Read only.

About 10 minutes. No ComplyEaze Bridge build, no write, no Tally setting changed. Four request files sent with curl.exe
to port 9001, one at a time, every answer kept byte for byte. Same method, folder rules, PARSE snippet, RULES and STOP
list as item 697-S0A (`lab/697-s0a-run-sheet/WINDOWS-697-S0A.md` on branch `lab/697-s0a-run-sheet`); only the files and steps differ.

**WHAT IT DECIDES.** The tax-audit read (#1727) now always counts its voucher window first with a census request,
even for a small book. The census shape is qualified live for the vouchers tool (protocol reference §11c), but never
for the audit read's window. Step 2 also answers a question the tax-audit tool depends on: does BRIDGE PILOT LAB hold
any voucher in FY 2025-26, the only year the tool's rules cover? (697-S0A read FY 2026-27.)

**FILES** (`C:\bridge-lab\697s1\requests\`; each SHA-256 must equal `SHA256.txt`, else STOP). The census header ID
equals its COLLECTION NAME (`Bridge Agent Voucher Census`). `r01` and `r02` are byte-identical to 697-S0A's.
| file | bytes | what it asks |
|---|---|---|
| r01-company-extent.xml | 2524 | the company list |
| r02-marks.xml | 976 | the company's two high-water marks |
| c01-census-fy2025-26.xml | 1436 | GUID, ALTERID, DATE of every voucher dated 2025-04-01 to 2026-03-31 |
| c02-census-fy2026-27.xml | 1436 | the same for 2026-04-01 to 2027-03-31 |

**STEPS** (answers in `answers\`; log every send as in 697-S0A)
0. Check the four hashes and the Tally version. BRIDGE PILOT LAB loaded alone.
1. Send `r01` (answer `1.xml`). EXPECT exactly one company, BRIDGE PILOT LAB; Education No. Record GUID and books-from.
2. Send `r02` (answer `2.xml`). Record ALTVCHID (M) and ALTMSTID. EXPECT M between 1 and 170.
3. Send `c01` (answer `3.xml`). Record the VOUCHER count under `/ENVELOPE/BODY/DATA/COLLECTION` (a parse error is a failed
   answer, never 0), the bytes and seconds; for each row record DATE and whether GUID and ALTERID are present. EXPECT HTTP
   200, STATUS 1, no LINEERROR. A count of 0 is a valid answer: record "no voucher in FY 2025-26".
4. Send `c02` (answer `4.xml`). The same records. EXPECT the count to be at most M, every row to carry GUID and ALTERID,
   every ALTERID between 1 and M, and every DATE inside the window.
5. Send `r02` again (answer `5.xml`). EXPECT ALTVCHID = M and ALTMSTID unchanged (the census read nothing it altered).

**PASS:** steps 1-5 meet every EXPECT. **REPORT:** the table of steps (HTTP, bytes, seconds, count), M, the FY 2025-26
voucher count with its date range, every answer file, the log and SHA256.txt. Confidence: one run, PARTIAL.

**Not established:** a census divided into AlterID spans (M is far below one census's size); Bridge's own headers and the
identity check around each request; any client book.
