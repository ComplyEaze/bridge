# Results: #899 part 2, the census sent as an AlterID span, TallyPrime 7.2, 26 Oct 2026. Book T PASS: the spanned and unspanned census are byte-identical (5 vouchers, AlterIDs 1 to 5). Book O stopped at step 2: its voucher mark is 21, not 20

The sheet is `lab/899-spanned-census-captures` at b73740aa4beda60b0ee73d11c6b9957b549503a3. Its 6 request files and README were checked against `SHA256.txt`: 8/8 OK. Every request was sent with the sheet's command, one at a time; `answers/log.txt` holds each send's UTC time, curl exit, `-w` line and the answer's sha256.

The build for `tally_status` was w1-2306d86d, `bridge_mcp.exe` sha256 92a899a2..., checked before each start. It ran over stdio with host 127.0.0.1, port 9001, a new empty data folder and terms accepted; posting, import and redaction were unset.

## Book T: BRIDGE TRANSFER LAB (100051)
Read on the same load as #1392's whole-day reads (`lab/1392-whole-day-read-answers`), which wrote nothing.

`tally_status` (`answers/t-tally_status.json`): TallyPrime 7.2, silver, `education_mode` false; exactly this company loaded (GUID d6e4c24f-8c7a-4797-93ec-e44bdff60bc9).

| response | bytes | sha256 | result |
|---|---|---|---|
| t-m1 | 3,398 | c412f89ed1fcf9b70d4476d57f7fdea988e3311c23effad29431547953933129 | one COMPANY row: **ALTVCHID " 5"**, **ALTMSTID " 210"** (as sent, with a leading space) |
| t-c1 | 15,050 | c84420220f0b6794bbd3984fe3bfb02c3ed069346d6c7e081a40454616d0905b | 5 VOUCHER: AlterIDs 1, 2, 4 (3 Apr), 3 (4 Apr), 5 (5 Apr), all Contra |
| t-c2 | 15,050 | = t-c1 | 5 VOUCHER, the same |
| t-m2 | 3,398 | = t-m1 | 5 / 210 |

- **Step 6:** `t-c1` and `t-c2` are **byte-identical**.
- **Step 7:** counted with an XML parser (nodes named exactly `VOUCHER` under `ENVELOPE/BODY/DATA/COLLECTION`).
- Marks are equal before and after.

## Book O: BRIDGE OUTSTANDINGS LAB (100028)
`tally_status` (`answers/o-tally_status.json`): TallyPrime 7.2, silver, `education_mode` false; exactly this company loaded (GUID 49f1fbda-ee59-4a4b-aacf-b45fe32402d7). The book was loaded alone, with no question or prompt.

| response | bytes | sha256 | result |
|---|---|---|---|
| o-m1 | 3,408 | ad1a9e1be89e38c5ec9d6da5c207895d4a408d635b9bf9b7b3316d804f12ee7f | one COMPANY row: **ALTVCHID " 21"**, **ALTMSTID " 222"** |

**Stop rule for this book (step 2):** the voucher mark is 21, not the 20 that `o-c2` was rendered for. By the sheet, `o-c1`, `o-c2` and `o-m2` were not sent, and nothing more was sent to this book. The spanned request has to be rendered again for mark 21.

## Close
Each book was shut by the Company menu > Shut, leaving no company loaded. Nothing was written. No dialog appeared.

The evidence folder was zipped on the lab PC (sha256 76f19820afe559d4295cca01963c30280d38f8647fa981360673db2777ea276d); the files are under `answers/` unedited.
