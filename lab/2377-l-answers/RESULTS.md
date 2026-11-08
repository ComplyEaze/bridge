# Row 2377-L, Part A: results

First line: S6 (L3) gave a 0-byte answer twice (HTTP 200, RESPSTATUS 1, CONTENT-LENGTH 0, no envelope), with /status healthy between the sends. That is neither shape the row allowed, so it fits no row of "What it decides" and is reported as it is. The kit's `pick` failed at S2 on the first kit and was corrected by the sheet's deviation of 8 Nov (3e865846). S4 (L2) and S5 (L6) were skipped: `picks.txt` says `z none` and `c none`. Every other step met its EXPECT.

Sheet: `lab/2377-sheet` at `3e865846` (one commit on `da5c4cac`), `SHA256.txt` 4 of 4 OK. TallyPrime 7.2 Silver (tally.exe FileVersion 1.1.7.2). Book: BRIDGE BILLTRAIL LAB, synthetic. All times are UTC, 8 Nov 2026. Every number below is read from its file in this folder.

| step | result |
|---|---|
| S0 | 08:02:30Z: no company loaded, no dialog. tasklist exit 0: one `tally.exe`, 12672. netstat exit 0: 9001 LISTENING on 12672 (0.0.0.0 and [::]). GET /status: curl 0, 200, 51 B. Loaded BRIDGE BILLTRAIL LAB alone (Down x11, then Enter once a crop showed the highlighted row read its name). Gateway: period 1-Apr-25 to 31-Mar-26, current date Monday 1-Sep-2025, Silver, no prompt. |
| S1 | `l2377.py fixed --out req`: exit 0; marks 984 B `501c3094`, books 968 B `d2e54607`, lo-p1-0331 1198 B `f560dba1`, lo-p1-0731 1198 B `7c093656`, lo-unknown 1198 B `5ed21185`, lo-case 1198 B `5507e5cd`, lo-space 1200 B `f0990a5a`. marks: 3,400 B, one COMPANY row, GUID `9ab5002c-e910-4242-83e4-a27cc490600b`, ALTVCHID 7, ALTMSTID 212. books: 3,320 B, the same GUID. `l2377.py books`: BOOKSFROM 20250401. |
| S2 | snap-0331 (1292 B `a5d027fd`): 10,378 B `e111b013`. snap-0731 (1292 B `90836c00`): 10,378 B `829ee265`. `pick` on the first kit: "FAIL: a LEDGER with no name" (the CMPINFO count line; see the deviation). On the corrected kit, on the kept snap-0331 answer without a resend: exit 0, "ledgers 6; bill-wise and admitted, other than P1: 0", "z none", "c none". |
| S3 (L1) | lo-p1-0331, first send: 5,878 B `063b3c76`; second send: 5,878 B `063b3c76`, byte-equal. `lo` (each): `bills 6 on_account 1 sum_billcl -26000.00 other_tags -`. `tie` (each): `sum_billcl -26000.00 ledger_closing -26000.00 equal`. lo-p1-0731: 3,926 B `f505ba96`. `lo`: `bills 5 on_account 1 sum_billcl -37000.00 other_tags -`. `tie`: `sum_billcl -37000.00 ledger_closing -37000.00 equal`. `screen`: 6 rows, total pending 26000.00 Dr at 0331; 5 rows, 37000.00 Dr at 0731 (`evidence/s3-lo-screen.txt`). As on 27 Oct at both dates. |
| S4 (L2), S5 (L6) | skipped: none (`picks.txt`: `z none`, `c none`). |
| S6 (L3) | First send, 08:23:20Z: curl 0, HTTP 200, 0 B, CONTENT-LENGTH 0; headers `HTTP/1.1 200 OK`, `RESPSTATUS: 1`, `CONNECTION: KEEP-ALIVE`, `CONTENT-LENGTH: 0`. LAB-COMMON C3 calls an empty body "the request failed": a stop, reported. On the run lead's word: GET /status at 08:27:00Z, curl 0, 200, 51 B, byte-equal to S0's; a capture at 08:27:37Z showed the Gateway, BRIDGE BILLTRAIL LAB alone, no dialog. The repeat at 08:27:47Z: curl 0, HTTP 200, 0 B, CONTENT-LENGTH 0, the same four header lines. The finding: an unknown party name answers HTTP 200, RESPSTATUS 1, CONTENT-LENGTH 0, no envelope, twice, with /status healthy between. `lo` was not run on the 0-byte files. |
| S7 (L4) | lo-case: 5,878 B `063b3c76`; lo-space: 5,878 B `063b3c76`. `lo` (each): `bills 6 on_account 1 sum_billcl -26000.00 other_tags -`. Each is byte-equal to S3's first lo-p1-0331 answer. |
| S8 | 08:28:23Z Gateway: BRIDGE BILLTRAIL LAB alone, no dialog. marks: 3,400 B `0fe9f835`, byte-equal to S1's: the same GUID, ALTVCHID 7, ALTMSTID 212. Equal. |
| S9 | Shut by Tally's top menu: Company, then SHut; the Shut Company list held BRIDGE BILLTRAIL LAB alone, highlighted; one Return. No confirmation appeared. Select Company, no company loaded, no dialog. 08:29:29Z: tasklist exit 0, one `tally.exe`, 12672; netstat exit 0, 9001 LISTENING on 12672; GET /status: curl 0, 200, 51 B. Nothing was sent after the shut. |

## What it decides

- **L1** ties at both dates with the word `equal`. **L4**'s two variants answer with P1's bills, byte-equal to L1's: the row "L4 answers with P1's bills for either variant" is met. Tally folds case and a doubled inner space here; the builder sends only the book's own name, which the design already says. Recorded.
- **L2** and **L6** were not measured: the book holds no such party.
- **L3** is neither an error line nor an empty envelope: it is a 0-byte body with RESPSTATUS 1, twice. It fits no row, and goes to the run lead as it is.

## Files

- `requests/`: the nine request files sent, whole.
- `answers/`: every answer, whole, with its response headers. Two answers are 0 B (S6).
- `evidence/`: the C2 reads (netstat cut to the port-9001 lines), /status bodies and headers, and the script lines of S3 and S7. `picks.txt` is `pick`'s file.
- `sendlog.txt`, `notes.txt`, and the helper scripts `send2377.py`, `c2.ps1`, `marks.py`.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what the pushed copies of the lab's own text changed. `SHA256.txt` lists every file.
