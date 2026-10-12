# Masters M1 lab sitting, phases 0-2 (12 Oct 2026)

Run sheet: lab/masters-m1-requests @ dd5135804c40f0f809dd804ec129543edf153a04 (README.md sha256 c87cd300f3539525198af9ef92cc44a5590f978964b5ee7db59c06eec587c7e7). The 21 request files were checked against SHA256.txt (21/21 OK). Only the four phase-2 reads were sent. No writes; phases 3-4 were not run.

## Phase 0
- TallyPrime 7.1, Silver, Licensed, not Education mode.
- Tally's own Connectivity settings: acts as Both, ODBC Yes, port 9001.
- Loaded at the check: one company (since shut, see phase 1).

## Phase 1 (keyed on screen)
- **Company:**
  - BRIDGE MASTERS LAB, number 100060.
  - Rajasthan, books from 1-Apr-2026, no inventory.
  - GST Regular, Monthly, with the run sheet's company GSTIN (shown here as the same-length mask ZZZZZZZZZZZZZZZ).
  - The only company loaded.
- **Ledgers.** All six are under Sundry Debtors, bill-by-bill No, no opening balance:
  - MLab Exact Target. Regular, with the run sheet's party GSTIN (mask ZZZZZZZZZZZZZZZ).
  - MLab Alias Target. Aliases MLab Alias Name and MLab Alias Two.
  - MLab Case Target.
  - MLab Space Target.
  - MLab Dash-Target.
  - MLab Slash/Target.
  - Tally pre-filled Registration type Regular on each party form. It was left that way, with GSTIN blank, on the five ledgers other than MLab Exact Target.
- **Group:** MLab Group Target under Sundry Debtors, alias MLab Group Alias.
- **Backup B0:**
  - File TDBK1800_100060.001, 24735 bytes, written 2026-10-12T14:16:25Z.
  - sha256 9ca37bd80d695f2fd93b8055a59566bfdf1f2b4a3b581a5b16c3092eb792994d.
  - Stays on the lab PC.
- **Screenshots:** the alias screens and the others stay on the lab PC. They are listed by hash in SHA256-screenshots.txt.

## Phase 2 (step id `nr`, sent in order, one at a time)
| Request | curl exit | HTTP | bytes | Rows under ENVELOPE/BODY/DATA/COLLECTION |
|---|---|---|---|---|
| R-MARKS | 0 | 200 | 3308 | COMPANY 1 |
| R-V2 | 0 | 200 | 12910 | LEDGER 8 |
| R-V3 | 0 | 200 | 13610 | LEDGER 8 |
| R-GRP | 0 | 200 | 56068 | GROUP 29 |

All four answers have Content-Type `text/xml; charset=utf-16` and were decoded by that charset.

- **R-MARKS:**
  - One COMPANY row: BRIDGE MASTERS LAB, GUID f107e523-3d6b-4516-bd24-5b1a14498fa6.
  - ALTMSTID 216.
  - **ALTVCHID is absent from the row,** although the request FETCHes it. The company has 0 vouchers, and CMPINFO VOUCHER is 0.
- **MLab Alias Target, NAME list in document order:**
  - R-V2: MLab Alias Target, MLab Alias Name, MLab Alias Two.
  - R-V3: MLab Alias Target, MLab Alias Name, MLab Alias Two.
- **R-V3 ALTERID:** yes, on 8 of 8 LEDGER rows.
  - Cash 32, Profit & Loss A/c 31.
  - MLab Exact Target 210, MLab Alias Target 211, MLab Case Target 212, MLab Space Target 213, MLab Dash-Target 214, MLab Slash/Target 215.
  - R-V2's rows carry no ALTERID.
- **MLab Group Target in R-GRP:**
  - NAME.LIST: MLab Group Target, MLab Group Alias.
  - GUID f107e523-3d6b-4516-bd24-5b1a14498fa6-000000d4.
  - ALTERID 216.
- **Parsing note:**
  - R-V2 and R-V3 each carry one `&#4;` character reference, in Profit & Loss A/c's PARENT ("&#4; Primary").
  - R-GRP carries raw 0x03 characters and 30 `&#4;` references.
  - A strict XML parser refuses these. The answers are published byte-for-byte as received.
  - phase2-parse.py replaces those characters with "?" in memory before parsing. phase2-parse.txt is its output.
- Tally's window showed no dialog after the reads (p2-tally-after-reads.jpg, by hash).

No STOP condition was met: one COMPANY row, ALTERID on every R-V3 ledger row, no transport failure, no dialog.

## Files
- nr-*.response and nr-*.headers: the answers and headers exactly as saved. SHA256-evidence.txt lists them.
- log.txt: the sitting log.
- send-read.sh: the send helper (it checks the hash, then runs the run sheet's curl line).
- phase2-parse.py and phase2-parse.txt: the element parse.
- SHA256-screenshots.txt and SHA256-backup.txt: hashes only.

## Masking
In log.txt and this report, the run sheet's two synthetic GSTINs (company and party) are written as the same-length mask ZZZZZZZZZZZZZZZ. The XML answers contain no GSTIN and are unchanged. The published log.txt also drops one scheduling note from its first line. log.txt: lab copy sha256 ab7efc4cd0e88bb32efc3e1dd9f438847b9a46c8c24b9c25b08af077914564d2, published copy sha256 a3ee4f08be2c0213e49b14d3e0946f6dcab977ed904fc634c7931526ba719a2b.
