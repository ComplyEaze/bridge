# Row 2377-L, Part B: results

First line: one finding, no stop. **L5 trail**: Tally created the party but holds its name without the trailing space. `present` reads `trimmed`. `r-lo-trail`, sent with the space, answered that party's one bill, -100.00. `tie` fails: `'L5 Trail ' is not in the snapshot`. The other 14 classes were each created `exact`, with their bill `one created`, and answer one bill of -100.00 that ties `equal`. **L7** sums to -150.00 as expected. Its two rows both carry `BILLREF` `On Account` and differ in `BILLDATE`, `BILLDUE`, `BILLOVERDUE`, `BILLVCHNUMBER` and the amounts. The restore was checked: B12 met every EXPECT, so the book is as found at B0. **L8**: each of the three parties answers 250 bills, its count in `top3.txt`. Every write's capture showed no dialog.

Sheet: `lab/2377-sheet-b` at `99c3e8b2` (one commit on `e8c82a5a`), `SHA256.txt` 5 of 5 OK. TallyPrime 7.2 Silver (tally.exe FileVersion 1.1.7.2). Books: BRIDGE BILLTRAIL LAB (sitting 1, writes behind a backup) and BRIDGE CORPUS DENSE (sitting 2, reads only), both synthetic. All times are UTC, 8 Nov 2026. Every number below is read from its file in this folder; `notes.txt` has each step's lines.

| step | result |
|---|---|
| B0 | 08:37:04Z: no company loaded, no dialog. One `tally.exe`, 12672; 9001 LISTENING on 12672; GET /status 200, 51 B. BRIDGE BILLTRAIL LAB loaded alone. Release 7.2 Silver. Current date Monday 1-Sep-2025. Period 1-Apr-25 to 31-Mar-26. |
| B1 | `l2377.py fixed --out req1` exit 0. marks: 3,400 B, one COMPANY row, GUID `9ab5002c-e910-4242-83e4-a27cc490600b`, ALTVCHID 7, ALTMSTID 212: the baseline. books: the same GUID, BOOKSFROM 20250401. `snapshot` and `writes` exit 0 (50 lines, `evidence/b1-writes.txt`). |
| B2 | snap-0331: 10,378 B. `present`: 17 lines, every one `absent`. |
| B3 | Backup `TDBK1800_<company number>.001`, in the new empty folder `C:\lab\2377-b\backup-1`, 33,477 B, sha256 `1ca0ec46...`. Written 08:45:11Z; the Restore list showed 8-Nov-26 (2:15 pm). marks byte-equal to B1's. |
| B4 | w0-income: `CREATED=1 ... ERRORS=0 ... one created lineerrors -`. marks after: ALTVCHID 7, ALTMSTID 213. |
| B5 | Per class, see the table below. |
| B6 (L7) | Three writes, each `one created lineerrors -`. marks after them: ALTVCHID 24, ALTMSTID 229. `lo`: `bills 2 on_account 2 sum_billcl -150.00 other_tags -`. `screen`: row 1, date 20251001, ref On Account, 100.00 Dr pending, due 1-Oct-25, overdue 181. Row 2, date 20260331, ref On Account, 50.00 Dr pending, due -, overdue -. `tie`: `l7 sum_billcl -150.00 ledger_closing -150.00 equal`. |
| B7 | marks: ALTVCHID 24, ALTMSTID 229 (17 vouchers and 17 masters more than the baseline, as written). |
| B8 | 135 answers of B4 to B7 copied; 135 sha256 equal to the send log (`evidence/b8-check.txt`). |
| B9 | Shut by Company, then SHut; no confirmation; no company loaded. |
| B10 | Backup sha256 equal to B3's; its folder holds that one file. `restore-plan.txt` written. The company folder held 21 entries before (`evidence/b10-company-folder.txt`). |
| B11 | Data, Backup & Restore, Restore. Restore Data Path is Tally's data folder; source by Specify Path. The list held BRIDGE BILLTRAIL LAB alone, with its own number and 8-Nov-26 (2:15 pm). The prompts were `Accept ?` (Yes), then `Company data folder (<company number>) already exists ...` (Replace); no other prompt. It was back at Select Company in under a minute. The folder afterwards held 17 entries, all written 09:22:20Z (`evidence/b11-after-company-folder.txt`). Loaded alone; Gateway, no dialog. |
| B12 | marks: 3,400 B, byte-equal to B1's (ALTVCHID 7, ALTMSTID 212). snap-0331: byte-equal to B2's; `present` all 17 `absent`. lo-p1-0331: 5,878 B, byte-equal to Part A's; `bills 6 on_account 1 sum_billcl -26000.00`. All met. |
| B13 | Shut by Company, then SHut. One `tally.exe` 12672 on 9001; GET /status 200, 51 B; no company loaded. Nothing sent after the shut. |
| D0 | No company loaded; one `tally.exe` 12672 on 9001; /status 200, 51 B. BRIDGE CORPUS DENSE loaded alone. Current date Tuesday, 31-Mar-2026, as expected. |
| D1 | `bills.py fixed --out req2` exit 0 (marks 982 B `040ac477...`, ref-receivable 770 B `6520d94e...`). marks: ALTVCHID 29900, ALTMSTID 328: met. |
| D2 | ref-receivable: 13,679,490 B, sha256 `5a406cab...`, first byte 30.27 s, total 30.39 s. `bills.py ref`: `ref ok`, 29900 rows. Its sum stays on the lab PC with the answer. |
| D3 | `top3` exit 0: `parties 120; admitted 120`; top1, top2 and top3 each `bills 250`. `top3.txt` stays on the lab PC. |
| D4 (L8) | lo-top1: 206,974 B, 1.14 s first byte, 1.14 s total. lo-top2: 206,980 B, 0.97 s, 0.97 s. lo-top3: 206,982 B, 1.19 s, 1.19 s. `lo` for each: `bills 250 on_account 0 ... other_tags -`, equal to `top3.txt`'s 250. A capture after each: the Gateway, no dialog. The sums stay on the lab PC with the answers. |
| D5 | Gateway: DENSE alone. marks byte-equal to D1's: ALTVCHID 29900, ALTMSTID 328: met. |
| D6 | Shut by Company, then SHut. One `tally.exe` 12672 on 9001; /status 200, 51 B; no company loaded. Nothing sent after the shut. |

## B5, one line per class

Each class: the party create's `counters`; the bill's `counters`; `lo`; `present` for that class; `tie`.

| class | name | create | bill | `lo` | `present` | `tie` |
|---|---|---|---|---|---|---|
| amp | `L5 Amp & Co` | one created | one created | 1 bill, -100.00 | exact | equal |
| angle | `L5 Angle <A>` | one created | one created | 1 bill, -100.00 | exact | equal |
| dquote | `L5 Quote "Q"` | one created | one created | 1 bill, -100.00 | exact | equal |
| squote | `L5 O'Neil` | one created | one created | 1 bill, -100.00 | exact | equal |
| dollar | `L5 Dollar $1` | one created | one created | 1 bill, -100.00 | exact | equal |
| dollar2 | `L5 $$Name` | one created | one created | 1 bill, -100.00 | exact | equal |
| hash | `L5 Hash #1` | one created | one created | 1 bill, -100.00 | exact | equal |
| colon | `L5 Colon: A` | one created | one created | 1 bill, -100.00 | exact | equal |
| dot | `L5 Dot Co.` | one created | one created | 1 bill, -100.00 | exact | equal |
| dash | `L5 Dash-Co` | one created | one created | 1 bill, -100.00 | exact | equal |
| comma | `L5 Comma, A` | one created | one created | 1 bill, -100.00 | exact | equal |
| paren | `L5 Paren (A)` | one created | one created | 1 bill, -100.00 | exact | equal |
| slash | `L5 Slash A/B` | one created | one created | 1 bill, -100.00 | exact | equal |
| nonascii | `L5 Café` | one created | one created | 1 bill, -100.00 | exact | equal |
| trail | `L5 Trail ` | one created | one created | 1 bill, -100.00 | **trimmed** | **FAIL**: `'L5 Trail ' is not in the snapshot` |

Every `counters` line read `ERRORS=0 ... lineerrors -`. Every `lo` line read `bills 1 on_account 0 sum_billcl -100.00 other_tags -`.

## What it decides

- **L5, 14 classes** (amp, angle, dquote, squote, dollar, dollar2, hash, colon, dot, dash, comma, paren, slash, nonascii) meet the first row. Each party was created `exact`, its bill `one created`, and its answer is one bill that ties `equal`. Each class may enter a separate admitted type for this report, in its own pull request, citing this row.
- **L5, trail** meets the second row: the name is not kept `exact`. Tally holds the party as `L5 Trail`, with the trailing space dropped. The Ledger Outstandings read sent with the space still answered that party's one bill, -100.00. `tie` fails on the spaced name, which no ledger in the snapshot carries. The class stays refused.
- **L7**: the New Ref bill named `On Account` and the On Account amount answer as two rows. Both have `BILLREF` `On Account`. The real bill has `BILLDATE` 1-Oct-25, `BILLDUE` 1-Oct-25, `BILLOVERDUE` 181. The On Account amount has `BILLDATE` 31-Mar-26 (the report date), with `BILLDUE` and `BILLOVERDUE` empty. They also differ in `BILLVCHNUMBER` and amount. `BILLVCHDATE` (1-Oct-25) and `BILLVCHTYPE` (Journal) are the same. Recorded as measured. The run lead decides which row this meets: whether an empty `BILLDUE` is "a field other than `BILLREF`" or falls under "date".
- **L8**: the three parties with the most bills in the Receivable report answer 250 bills each, equal to `top3.txt`. Each read took about 1 s.

## Files

- `requests/sitting1/`: every sitting-1 request sent, whole, except the 17 voucher writes (`w2-bill-<class>` for the 15 classes, `w2-bill-l7` and `w3-onacct-l7`). Each of these carries a voucher REMOTEID that is not in the synthetic registry, so they are listed in `kept-local.txt` by size and sha256. The sheet's `l2377b.py` makes each REMOTEID as uuid5 of `row-2377-b/<tag>`. `l2377b.py writes --out <dir>`, run again on the lab PC, made all 17 byte-equal to the files sent. `requests/sitting2/`: marks and ref-receivable. lo-top1 to lo-top3 are listed in `kept-local.txt`.
- `answers/`: every sitting-1 answer and sitting 2's two marks answers, whole, each with its response headers. The ref-receivable and lo-top answers are listed in `kept-local.txt`.
- `evidence/`: the C2 reads (netstat cut to the port-9001 lines), /status bodies, the script lines, the backup folder listings before and after B3, the company folder listings of B10 and B11, and `b8-check.txt`. The B8 copies are byte-equal to `answers/` and stay on the lab PC.
- `restore-plan.txt`, `sendlog.txt`, `notes.txt`, and the helper scripts.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt` lists what the pushed copies of the lab's own text changed. `kept-local.txt` lists what stays on the lab PC and why. `SHA256.txt` lists every file.
