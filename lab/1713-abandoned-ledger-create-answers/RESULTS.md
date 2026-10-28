# Results so far: #1713 abandoned ledger create, 28 Oct 2026. On BRIDGE ABANDON LAB (TallyPrime 7.2) a ledger Create sent while the gateway was busy with a 238 MB voucher read was accepted, held about 154 s, and answered 0.48 s after the read's first byte, while the read's body was still streaming. Row L1's gate compares the two curl ends, so it halted there as the sheet requires. L0 to L1 are recorded; L1B to L5 are not run yet. The book is kept for the sitting to resume at L1B once the order gates compare against A's first byte

The sheet is `lab/1713-abandoned-ledger-create`:
- First sitting at f86d4f1a.
- Then at 9a77082a: `read.py marks` accepts an absent ALTVCHID.
- Then at b28ec7967963a15c21b79acdde1bf84bfbd42711: `read.py import` accepts a bare RESPONSE root.

Each fix followed a halt recorded in `notes.txt`. `SHA256.txt` was 18 of 18 equal at each.
- **Tally:** TallyPrime 7.2 (Latest), Silver Rental, licensed, a single currency.
- **Company:** BRIDGE ABANDON LAB, created on Tally's screen at S1 for this sheet. Number 100096, GUID 5f4825c4-5d89-47fa-9872-6397cca3a591. Books from 1 April 2025, INR, Company Features accepted unchanged (every value in `notes.txt`).
- **Requests:** by curl only, with no ComplyEaze Bridge build. Hand sends use LAB-COMMON C3's line; the scripts use their own lines.
- **Owner's yes:** for the writes, recorded in `notes.txt` before S1.

## Setup
| step | what | result |
|---|---|---|
| S0 | scripts parsed from cmd.exe; three dry runs | 0 errors each; the dry-run lines equal the sheet's exactly |
| S1 | company created; MARKS | GUID as above, ALTMSTID 207, ALTVCHID absent (no voucher) |
| S2 | BK0 | `TDBK1800_100096.001`, 23,385 B, sha256 da352c5d... |
| S3 | `s3-bulk-ledgers.xml` | a bare RESPONSE, `import ok CREATED 2`; LEDGERS: Lab Bulk Expense (Indirect Expenses), Lab Bulk Income (Indirect Incomes); ALTMSTID 207 → 209 |
| S4 | `bulk.ps1 -First 1 -Last 96` | `DONE: bulk-001 to bulk-096, 96 files, each CREATED 250`; about 5 to 6.5 s a file; ALTVCHID 24000 |
| M0 | BUSY alone | 119,036,974 B = CONTENT-LENGTH, `busy ok rows 24000`, 21.786 s |
| plan | `read.py plan 119036974 21.786` | `4960 bytes a row; send bulk-097 to bulk-192 (96 files); BUSY then predicted 238073948 bytes, 48000 rows` |
| M0 run | `bulk.ps1 -First 97 -Last 192` | `DONE: bulk-097 to bulk-192, 96 files, each CREATED 250`; ALTVCHID 48000 |
| S5 | BK1 | `TDBK1800_100096.001`, 13,255,159 B, sha256 cf65e098... |
| M1 | BUSY alone | 238,211,294 B = CONTENT-LENGTH (the plan predicted 238,073,948), `busy ok rows 48000`, **186.645 s**: D = 187 s, within 40 to 300, at least 55 (L5 would run). `GET /status` 200 |

Doubling the book from 24,000 to 48,000 Journals took BUSY from 21.8 s to 186.6 s, eight and a half times as long. M1's body began only near the end of that time.

## L0, the control
- MARKS, then `l0-create.xml` with no limit: a bare RESPONSE, `import ok CREATED 1`.
- LEDGERS: `Lab Probe L0` under Suspense A/c, ALTERID 210.
- ALTMSTID 209 → 210: **+1 for one Create**.

## L1: halted at its gate
`row.ps1 -Row L1 -BusySeconds 187 -BusyBytes 238211294 -BusyRows 48000`. Times in UTC, from `times.txt`. Each curl `-w` line is http_code, size_download, size_upload, time_connect, time_starttransfer, time_total.

| send | queued | curl start | curl -w | first byte | end |
|---|---|---|---|---|---|
| A, BUSY, no limit | 20:20:39.006 | 20:20:40.748 | `200 238211294 1826 0.005139 161.567999 163.549734` | about 20:23:22.32 | 20:23:24.426 |
| B, `l1-create.xml`, no limit | 20:20:44.182 | 20:20:48.262 | `200 538 1004 0.022583 154.539004 154.540826` | about 20:23:22.80 | 20:23:22.892 |

- **B's answer:** a bare RESPONSE, `import ok CREATED 1`.
- **netstat at B + 10 s:** both connections ESTABLISHED to Tally's 9001.
- **Measured:** Tally accepted the create's connection while busy with A and held the create about 154 s. It answered B about **0.48 s after A's first byte**, while A's 238 MB body streamed for about 2 s more.
- **The gate:** it compares the two curl ends, so it printed `B ended before A ended` and ended the row, sending nothing more. L1B was not run and L1 was not run again.
- **Reads after the row:**
  - LEDGERS: `Lab Probe L1` under Suspense A/c, ALTERID 211; `Lab Probe L0` still present; 6 ledgers.
  - MARKS: ALTVCHID 48000, ALTMSTID 211 (+1).
  - `GET /status` 200; no dialog at any time.
- **C10:** Tally was closed cleanly, with no company loaded and one `tally.exe` owning 9001. **BRIDGE ABANDON LAB is kept as it is (no restore), and BK1 stands.**

**What this decides so far:** the create sent while the gateway was busy was queued and run. Its answer came after the busy read had been answered, and before that answer finished arriving. Whether an abandoned create is still run (L2 to L5) is **not yet measured**.

**Not covered yet:** L1B to L5; the observation reads; E1.

## Files
- `requests/`: the sheet's 11 requests, as sent.
- `evidence/hand/`: every request sent by hand and its answer with headers; `sends.txt`.
- `evidence/L0/`, `evidence/L1/`: the rows' files. A's answer is listed in `BUSY-ANSWERS.txt`.
- `evidence/bulk/`: `bulk-answers.txt` and every `.sent`, header and `.curl.err` file. The 192 answers are listed in `BULK-ANSWERS-SHA256.txt`.
- `evidence/m0/`, `evidence/m1/`: the BUSY headers. The answers are listed in `BUSY-ANSWERS.txt` by size, sha256 and rows.
- `evidence/starts.txt` and `s0/` (the dry runs).
- `notes.txt` (UTC).
- **Logs on the branch** (`times.txt`, `bulk-answers.txt`, `starts.txt`, the dry runs, `notes.txt`): copies with the lab folder path written `<lab folder>`, the sheet's capital halt word written `HALT`, and its name for who decides written `run owner`.
- **netstat files on the branch:** reduced to their `:9001` lines.
- **`LOGS-ORIGINALS-SHA256.txt`:** the originals of both of the above, which stay on the lab PC.
- **`SCREENSHOTS.txt`:** the captures by sha256. They, the backups and the BUSY and bulk answers stay on the lab PC.
