# Results: #683 forex currency reads, 29 Oct 2026, BRIDGE FOREXGROUP LAB on TallyPrime 7.2. The book holds two Currency masters, `$` and `₹`. On 7.2 the rupee master's `NAME` and `ORIGINALNAME` are both `₹` (one character, U+20B9), and the company's own `CURRENCYNAME` is that same `₹`. Reads only; the marks did not move

The sheet is `lab/683-forex-currency-reads` at 553bb89eaf5731ae9e4c2b9b69c412d35804a744.
- `SHA256.txt` matched 8 of 8. It lists hashes only; the request sizes were 966, 1,004, 976 and 986 B.
- `check-request-ids.py requests` printed `census: ok 4, skipped 0, FAIL 0`.

**Tally.** TallyPrime Release 7.2 (Latest), Silver Rental, licensed (About screen). `GET /status` 200, 51 B.

**Company.** BRIDGE FOREXGROUP LAB, 100090, loaded alone. Current date 1-Apr-2025.

**Go-ahead.** From the run order of 29 Oct.

**Requests.** Every request was sent by C3's line, one at a time, with a size bound of 8,192 B. After each of the three NEW SHAPE reads, Tally's whole window was captured at once (no dialog) and then `GET /status` answered 200.

| step | request (sha256) | sent → answered (UTC) | status, bytes = CONTENT-LENGTH | answer sha256 | parses |
|---|---|---|---|---|---|
| m0 | marks (5b447d03…) | 04:25:47.246 → 47.719 | 200, 3,402 | c43336b1b9364cac67845f6fcdba531d2e9d698ede6b79b4d0924a365da27a3a | yes |
| r1 | currency-masters (df262192…) | 04:25:47.826 → 47.949 | 200, 3,796 | f5a1414a195194ff4f8b46f4c65cae0e0f7567143c71c6a2c465532334b3439a | yes |
| r2 | currency-masters-originalname (0f10c96e…) | 04:25:48.649 → 48.751 | 200, 4,000 | 3f0bfbcbb3bc0def49de13a7ac918e5d20d6a1d6885aa0e17635eb5af694aaf0 | yes |
| r3 | company-base-currency (8f620533…) | 04:25:48.922 → 49.030 | 200, 3,434 | 52c33f519c9e5ee390d44ef30223ad46f8b3bbbe2c71e02a5c995e0672043df3 | yes |
| m1 | marks (5b447d03…) | 04:25:49.216 → 49.366 | 200, 3,402 | c43336b1… (the same bytes as m0) | yes |

## What Tally sent (read by an XML parser)
**r1 (2 CURRENCY rows)**

| NAME | length, code points | MAILINGNAME | DECIMALPLACES |
|---|---|---|---|
| `$` | 1, U+0024 | `USD` | `' 2'` |
| `₹` | 1, U+20B9 | `INR` | `' 2'` |

**r2 (the same 2 rows, with ORIGINALNAME)**

| NAME | MAILINGNAME | DECIMALPLACES | ORIGINALNAME | length, code points |
|---|---|---|---|---|
| `$` | `USD` | `' 2'` | `$` | 1, U+0024 |
| `₹` | `INR` | `' 2'` | `₹` | 1, U+20B9 |

**r3 (1 COMPANY row, the company loaded alone)**
- NAME `BRIDGE FOREXGROUP LAB`;
- GUID 1bb48be5-baba-45b2-90d8-7593244915a9;
- CURRENCYNAME `₹` (1 character, U+20B9).

**Marks.** m0 and m1 both read ALTMSTID 211 and ALTVCHID 4; the two answers are the same bytes.

## For the requester's comparison
- On 7.2, the company's `CURRENCYNAME` equals both the rupee master's `NAME` and its `ORIGINALNAME`. The two fields do not differ for this master.
- The ledger snapshot and Trial Balance of `lab/2071-forex-rerun-answers` show `CURRENCYNAME` `₹` on every ledger row. That is the same character as the rupee master's `NAME` here.
- The lab did not run `identify_base`.

**Confidence:** one sitting, PARTIAL until repeated.

## Files
- `raw/`: `m0`, `r1`, `r2`, `r3` and `m1`, each `.response` byte for byte with its `.headers.txt`.
- `readings.txt`: the parser's lines.
- `times.txt` and `send-log.txt`.
- `checks/`: step 0 and C10 checks.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256. They stay on the lab PC.
