# Row 2569-R: answers

Sheet `ROW.md` of row 2569-R (lab/2569-restore-sheet at `208932f742338b500bf6219629905277ef1e12f6`, folder lab/2569-restore/), run 2026-11-20 to 2026-11-23 on TallyPrime Silver Release 7.2 (Licensed) against the synthetic book BRIDGE EINVOICE LAB (GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`), and restored to BK1 at the end.

## Grade for #2569: INCONCLUSIVE

- **No FAIL.** No box of the new build was read minimised: H2, H3 and both Part B boxes each read `iconic=False`.
- **Part B: PASS on both boxes.** Invoice A's and invoice B's approval boxes under the new build each read W1, W2, W3 and W4 PASS before anyone answered them; the owner answered each **Yes** by mouse within its time, and each invoice posted `posted_verified` with `created` 1.
- **H2: INCONCLUSIVE by the sheet's own wording.** Started with `-Show 7`, the box was not minimised at its first read and its error output holds no `box-fit: restored from minimised` line: the restore path was not exercised.
- **H1 FINDING.** Under the old build, `starth.ps1 -Show 7` also gave a box that was not minimised, so a minimised first show state does not reproduce the box of 19 Nov; the issue's named cause is not shown.
- **Part K: cut by the owner on 22 Nov** (the row was cut to H2, Part B and Part C on the recorded word). K ran to its step 4 (KA built) and posted nothing; its step 5 box was never opened, so the PASS condition on the old build's box is waived, not met.
- **Part C:** step 11 as sent, and the restore read back equal to step 1.

The row's result for #2569 is INCONCLUSIVE, with Part B as positive evidence: the new build's boxes on the 2440-CP launch path opened in front, not minimised, topmost, with focus on No. #2569 stays open.

## Builds

| build | read at the lab |
|---|---|
| OLD | row 2440-CP's copy; zip 60,609,752 bytes, digest, `.mcpb` and exe each equal to the OLD line; exe 35,869,184 bytes, sha256 `c437138a08dc10a8609e42d28104920401b4615daa69f3cdf404f6e37d6323dd`; reports 0.7.0 |
| NEW | the Windows bundle of the master CI run on `8c3fb9f2277966fa98b31bdc775b65c5ea3c29f7`; zip 60,653,959 bytes, sha256 `f8dc02e1f7bbb134e5ecc929c1f567aa265df2d3d1c866fea6636270e52e9472`; `.mcpb` `445d45a6fe18035258f69f04c7538241f19c80a39ad9ad88165b8a7c9f15ac8f`; exe 35,900,928 bytes, sha256 `1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59`, equal to the bundle's `binary_sha256`; reports 0.7.0 |

Each exe was hashed again before each hand run and each launch: always equal to its line.

## Before the sitting

| item | result |
|---|---|
| 1 Licensed | SILVER on screen, no Educational mode |
| 2 the grant | met on 22 Nov, after grant runs (below): the screen-access list shows both exe paths |
| 3 files | `SHA256.txt` ok 23, FAIL 0 at each sheet head; at `208932f7` the same |
| 4 helpers | `boxstate.ps1`, `starth.ps1`, `readwin.ps1` parse with 0 errors; `boxstate.ps1` against tally.exe printed one WINDOW line and the FOREGROUND and FOCUS lines |
| 5 builds | as above |
| 6 go-ahead | the owner's word to finish the #2569 run |

### Grant runs (outside H1 to H3, not graded)

The screen-access grant can name a helper exe only while it runs, and the helper runs only while its box is up. Grant runs started the helper with `starth.ps1 -Show 1` so its box was up when the grant was asked.

- **G-new, first start (20 Nov):** `starth.ps1` stopped with `CreateProcess failed: 123` before any child started (an empty current folder passed for a null one). Its two 0-byte files `g-new-out.txt` and `g-new-err.txt` are kept. The sheet's fix (`208932f7`) passes a real null.
- **g-new2 (20 Nov):** STARTED, the box read normal, in front, topmost, focus on No. The grant asked while it was open came back denied. Nobody answered the box for 600 s; `starth.ps1` printed `NOT EXITED within 600 s (wait result 258)`; the box closed on the owner's No on 22 Nov, so no exit code was captured. `g-new2-out.txt` 0 bytes.
- **g-new3 (22 Nov):** the same box; the grant asked again was denied again; the owner answered No; EXITED code 1, out 0 bytes.
- **With no box open:** the grant could not name the exe (not running); the request was not shown.
- **g-new4:** EXITED code 1 about 3.5 s after the start, before any read; the lab sent no key. Likely a key pressed elsewhere while the box took the focus (No is its default); not confirmed.
- **g-new5:** the owner kept to the mouse only. The box opened at the screen's centre; the owner dragged it to the bottom-right (rect (1370,788,1918,1004)); the grant asked then was **granted** for the exe-new path. The lab answered No: EXITED code 1, out 0 bytes; its error output holds a second `box-fit:` line for the moved place, `fits=true`.
- **G-old:** the same steps with the old build; **granted** for the exe-old path. The dragged box sat partly past the screen's right and bottom edges (rect (1389,838,1937,1054)); the old build checked its fit again there (`fits=false`) and ended the box itself with EXITED code 77, nobody answering. A box that does not fit the screen is outside this row's scope; recorded as an observation, not a row result.

Both grants then held. The two denials each came while a box sat at the screen's centre, topmost; where the grant dialog is drawn could not be seen from the lab.

## Part A (run before Part H, on the recorded word; Part H writes nothing to Tally)

| step | result | grade |
|---|---|---|
| 1 load and read | the book loaded alone; one COMPANY row, the book's GUID, ALTVCHID 3, ALTMSTID 215, equal line for line to row 2440-CP's | PASS |
| 2 backup | BK1: one file, 32,101 bytes, sha256 `6092110e7cdf985fd2ae5c14061aea9297fab55a7bb0f81151692cc29d238b0b`; `--same-row` equal | PASS |
| 3 customer, seeded | `I-seed.xml` CREATED 1, every other counter 0; ALTMSTID 216 | PASS |
| 4 duty ledgers, keyed | Tax type CGST and SGST/UTGST, as the sheet says; ALTMSTID 218 | PASS |
| 5 Sales type, keyed | `BRIDGE CP Sales`, Manual, prevent duplicates Yes; ALTMSTID 220, ALTVCHID 3 | PASS |

## Part H

| run | first read | out, err, exit | grade |
|---|---|---|---|
| H1 OLD `-Show 7` | `iconic=False`, Normal, in front, focus No, topmost, rect (696,444,1244,660) | exit 1; out 0 bytes; err two `box-fit: box=(696,444,1244,660) work=(0,0,1920,1030) fits=true` lines | FINDING: not minimised |
| H2 NEW `-Show 7` | W1 PASS (`iconic=False`, Normal); W2 PASS; W3 PASS (No, `ControlType.Pane`); W4 PASS; rect (696,444,1244,660) | exit 1; out 0 bytes; err one `box-fit: box=(696,444,1244,660) work=(0,0,1920,1030) fits=true` line, no restore line | W1 to W4 PASS; restore line EXPECT not met (the box did not start minimised) |
| H3 NEW `-Show 1` | W1, W3, W4 PASS; W2 True; rect (696,444,1244,660), equal to H2's | exit 1; out 0 bytes; err one `fits=true` line, no restore line | PASS |

H1 and H3 ran before the cut; their files are kept as run. The lab answered each box No by mouse once its reads were taken, with the owner at the screen.

## Part K (cut after step 4)

| step | result |
|---|---|
| 1 step 0 | one tally.exe on 9001; the OLD exe equal to its line; `BRIDGE_AGENT_ENABLE_WRITES` present; `data-k\` new and empty |
| 2 `list_companies` | BRIDGE EINVOICE LAB alone, its GUID equal to `count8.py`'s BOOK |
| 3 MARKS | MK 220, VK 3, equal to `s5r` |
| 4 build KA | `voucher_count` 1, `batch_id` `bridge-674f1e9c-3bc9-4eb5-bf1d-e813aa776059`, no refusal |
| 5, 6 | NOT RUN: the cut. No `post_import` was sent; the launch closed with exit 0 |

## Part B (NEW, one launch)

| step | result | grade |
|---|---|---|
| 6.1 step 0 | one tally.exe on 9001; the NEW exe equal to its line; WRITES present; `data-b\` new and empty; client started as in Part K | PASS |
| 6.2 `tools/list` | 37 tools; `build_import_xml`'s `invoice.e_invoice` requires `irn`, `ack_no`, `ack_date`; `post_import`, `verify_import`; `vouchers` with `include_e_invoice` | PASS |
| 6.3 `list_companies` | BRIDGE EINVOICE LAB alone, `verified_tuple`, GUID equal to BOOK | PASS |
| 6.4 MARKS | M6 220, V6 3, equal to Part K's and `s5r`'s | PASS |
| 6.5 `voucher_types` | `BRIDGE CP Sales`: manual, duplicates prevented, observed | PASS |
| 7 build A | `voucher_count` 1, `batch_id` `bridge-59097e5a-9a7d-4e51-a902-86f7c5dd0ee9`, no HSN warning | PASS |
| 8.1 V8a | 3 | PASS |
| 8.2 A's box | helper `bridge_mcp.exe --confirm-journal`, child of the server; first read 05:44:42Z: `iconic=False` Normal (W1 PASS), in front with the helper's pid (W2 PASS), focus No (W3 PASS), topmost and visible (W4 PASS); no line beginning `E-invoice` | PASS |
| 8.2 answer | the owner's Yes by mouse; call 1 `pending` (retry after 15 s); call 2 `posted_verified`, `created` 1, every other counter 0 | PASS |
| 8.4, 8.5 | Tally at the Gateway, no dialog; `verify_import` A `posted_verified` | PASS |
| 9.1 build B | `voucher_count` 1, `batch_id` `bridge-af17a381-0715-472b-94cc-73e7ea6e255c`, no HSN warning | PASS |
| 9.2 V9a, B's box | V9a 4; first read 05:48:15Z: W1, W2, W3, W4 PASS; `E-invoice IRN: ` and the IRN sent on its own line before the bill allocation, `E-invoice ack: 445577 of 2025-04-03` on the bill allocation's line, as row 2440-CP's sheet correction records | PASS |
| 9.3 | the owner's Yes by mouse; call 2 `posted_verified`, `created` 1, no `invoice_fields` | PASS |
| 9.4 | the Gateway, no dialog; `verify_import` B `posted_verified` | PASS |
| 10 read-back | M 220, V 5; `vouchers` 2025-04-03: the book's three vouchers and `CP/25-26/0101`, `CP/25-26/0102` on `BRIDGE CP Sales`, each with the four entries sent; 0101's e-invoice values absent, 0102's as sent; the launch closed, exit 0 | PASS |

Each box was read as soon as its helper appeared (about 2 s after), before the first call came back `pending`: both boxes were open and unanswered at the read. Recorded as a change of order; the run lead accepted it.

## Part C

| step | result | grade |
|---|---|---|
| 11 what Tally stored | 0101: `IRN`, `IRNACKNO`, `IRNACKDATE` empty; 0102: `IRN` 64 characters equal to the IRN sent, `IRNACKNO` 445577, `IRNACKDATE` 20250403; GST header fields equal on both; M ALTVCHID 5, ALTMSTID 220; the `--diff` list as row 2440-CP's sitting 2 | PASS |
| 12 restore | BK1 rechecked before the shut (32,101 bytes, `6092110e...`); restored (C6) on the recorded RESTORE word; the Restore list showed the book at 20-Nov-26 (10:37 pm) as planned; `Accept ?` Yes, `already exists` Replace, no other prompt; `--same-row` against `s1` equal (ALTVCHID 3, ALTMSTID 215); K-customer 0 LEDGER rows; F-0101: 0 VOUCHER rows | PASS |
| C10 | the book shut; Select Company, none loaded; one tally.exe on 9001; no `bridge_mcp.exe` running | PASS |

After the restore the book's data folder lacks the seven side files it held before, as after this lab's earlier restores.

## Launches

Both launches ran the sheet's `client.py` (byte-equal to row 2440-CP's) as a background shell command of the operator's tool, with no window, stdin from the null device: `python -I -X utf8 client.py <client folder> <exe> <exe sha256>`. The same both times; only the exe, the client folder and the data folder changed. Part K's first `launch-env.json` came out with single backslashes in its path when written through a shell; it was moved aside unused (`setup-aside/`), and each file used was written by a tool that writes the bytes given and read back before its launch: K `648f08a051b3f6eadc416e0b678f5463dfb9c978fed071bc4bf549f6faeb2147`, B `0dda3c95dc4683ff506407c14e0954bf8e263991f21a9e3cb9cc88ddd2ec950b`, 203 bytes each.

## Files

- `h/`: each hand run's `starth.ps1` output, its `-out` and `-err` files, its reads.
- `run-k/`, `run-b/`: each launch's requests, answers (company number redacted), client log, launch file.
- `tally/`: Tally's raw answers (company number masked at equal length) and the read logs.
- `evidence/`: the notes with every time, the count8 outputs, the box reads, the screens as text, the restore plan.
- `scripts/`: the lab's own helpers for this row.
- `kept-local.txt`: what stays on the lab PC (screens, the backup, the builds, data folders, listings, the tool list), by size and sha256.
