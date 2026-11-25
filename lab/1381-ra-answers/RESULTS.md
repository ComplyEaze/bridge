# Row 1381-RA: results

Sheet `lab/1381-ra-sheet` at 30f5c820d6601765f3bc680301dfb2f2b2ec3a96 (base master 6c9e6b33b531d0bd23bf6c4c62ba0d01a0aa4a1d). Run 2026-11-25, 20:38Z to 20:43Z, in one sitting, on the Windows lab PC. TallyPrime 7.2 in Educational Mode (title band `TallyPrime EDU` at every screenshot), pid 23160, port 9001. The book `BRIDGE STOCK CREATE LAB` (GUID bac2b5e9…) loaded by Select Company and shut by Shut Company; no other company loaded. Ten reads by `send.py`, each once, in the order M1, C1, C2, I1, I2, G1, G2, U1, U2, M2, from 20:40:48.813Z to 20:41:10.253Z; nothing else sent, keyed or clicked in Tally between M1 and M2 but the C1 screenshot. No write, no backup, no restore, no Bridge build. No stop.

## The ten send lines

| label | request | bytes | answer bytes | answer sha256 | first byte, total (s) | verdict |
|---|---|---|---|---|---|---|
| M1 | `marks.utf16le.xml` | 990 | 3,318 | `ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1` | 0.006539, 0.007573 | SENT-OK, exit 0 |
| C1 | `count.utf16le.xml` | 1,654 | 4,830 | `6552677a10bdb179ae0fc5d61bd5615ecb6aceb37c175bbac0ff5b91d6553979` | 0.004774, 0.005605 | SENT-OK, exit 0 |
| C2 | `count.utf16le.xml` | 1,654 | 4,830 | `6552677a10bdb179ae0fc5d61bd5615ecb6aceb37c175bbac0ff5b91d6553979` | 0.004758, 0.023024 | SENT-OK, exit 0 |
| I1 | `items.utf16le.xml` | 1,330 | 8,270 | `92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4` | 0.064297, 0.065622 | SENT-OK, exit 0 |
| I2 | `items.utf16le.xml` | 1,330 | 8,270 | `92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4` | 0.006223, 0.007448 | SENT-OK, exit 0 |
| G1 | `groups.utf16le.xml` | 1,144 | 4,170 | `b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a` | 0.008803, 0.009841 | SENT-OK, exit 0 |
| G2 | `groups.utf16le.xml` | 1,144 | 4,170 | `b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a` | 0.006619, 0.007563 | SENT-OK, exit 0 |
| U1 | `units.utf16le.xml` | 1,174 | 4,040 | `cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39` | 0.005186, 0.005993 | SENT-OK, exit 0 |
| U2 | `units.utf16le.xml` | 1,174 | 4,040 | `cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39` | 0.002665, 0.003392 | SENT-OK, exit 0 |
| M2 | `marks.utf16le.xml` | 990 | 3,318 | `ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1` | 0.004751, 0.005659 | SENT-OK, exit 0 |

Each: curl exit 0, status 200, CONTENT-LENGTH equal to the answer's bytes. `evidence/send-log.txt` holds the ten lines whole.

## The state and the compute

- `measure.py state` after M1 (`evidence/m1-state.txt`): one COMPANY row, GUID bac2b5e9… (the book's), ALTMSTID 210, ALTVCHID absent: `BOOK STATE MET`, exit 0.
- `measure.py all` (`evidence/measure-all.txt`, whole there): every label read; C1 and C2 one COMPANY row each, the book's GUID, `NUMSTOCKITEMS reported 1 (as received ' 1')`; I1 and I2 one STOCKITEM row, `SC Sibling Goods`, bound to the book; G1 and G2 one STOCKGROUP row, `SC Goods Group`, bound; U1 and U2 one UNIT row, `Nos`, bound; every pair byte-equal; the four item-against-count lines equal; M1 and M2 equal; M1, I1, G1 and U1 byte-equal to the stock-create-lab captures; `ALL 10 ANSWERS READ`, exit 0.

## The questions

- **Q1: yes, for this book in this sitting.** C1 and C2 each answer one company row with `NUMSTOCKITEMS` reported as 1 (the text as received is a space and the digit); I1 and I2 each answer one STOCKITEM row bound to the book; each item read's count equals each count read's (four lines, all equal).
- **Q2: yes.** C1 and C2, I1 and I2, G1 and G2, U1 and U2: each pair byte-equal.
- **Q3: yes.** M1 and M2: ALTMSTID 210, ALTVCHID absent in both; byte-equal.
- **Q4: yes, byte for byte.** M1, I1, G1 and U1 in Educational Mode are byte-equal to the answers lab 1381-O recorded for this book in Silver (the stock-create-lab captures on master: marks 3,318 B, items 8,270 B, groups 4,170 B, units 4,040 B). The title band read `TallyPrime EDU` at all six screenshots; the Gateway showed the book as the one company, No Vouchers Entered, period 1-Apr-25 to 31-Mar-26, current date Tuesday, 1-Apr-2025, as lab 1381-O recorded.

Each answer is one book, one release, one sitting, in Educational Mode: partial, and recorded so.

## Steps and EXPECT

| step | EXPECT | met |
|---|---|---|
| 0 | `made`, exit 0; fetch exit 0 (head 30f5c820 as the GO names); copy `exit 0, 0`; `--fresh` four `OK`, exit 0; `present 6c9e6b33…`; fixtures `exit 0, 0`; the six self-checks exit 0; no `bridge_mcp.exe`; one `tally.exe`, pid 23160; `:9001` LISTENING on that pid and 40 LISTENING lines in all; the screenshot: Select Company, no company loaded, EDU, no dialog; GET `/status` status 200, 51 bytes, exit 0 | met (deviation 2) |
| 1 | the book the one row left in the filter; the Gateway with the book as the one company, EDU, no dialog; period 1-Apr-25 to 31-Mar-26 and Tuesday 1-Apr-2025 | met |
| 2 | M1 `VERDICT SENT-OK`, exit 0; `BOOK STATE MET`, exit 0 | met |
| 3 | C1 and C2 `SENT-OK`, exit 0; the screenshot after C1 the Gateway as at step 1, EDU, no dialog | met |
| 4 | I1, I2, G1, G2, U1, U2 each `SENT-OK`, exit 0 | met |
| 5 | M2 `SENT-OK`, exit 0; the screenshot the Gateway as at step 1, EDU, no dialog | met |
| 6 | `ALL 10 ANSWERS READ`, exit 0 | met |
| 7 | the shut by Shut Company; Select Company with no company loaded, EDU; the same `tally.exe` pid 23160 owning `:9001`; GET `/status` status 200, exit 0 | met: no renewal dialog and no License screen came, so no licence key or click was made; `/status` status 200, 51 bytes, exit 0; 40 LISTENING lines |
| 8 | `check_sheet.py` three `OK`, exit 0; the listing exit 0 | met at 20:43Z: three `OK`, exit 0; the listing 114 files under `local/`, exit 0 |

The keys. The load: one click in the Select Company filter box; `BRIDGE`, then a space and `STOCK` (three rows left), then a space and `CREATE` (one row left); Enter. The shut: one click on the Gateway's left panel; Alt+K; one click on SHut; Enter on the one row.

## Deviations

1. The screenshots were saved as `.jpg` (the screen tool's format) under the sheet's names: `s0-window.jpg`, `s1-select.jpg`, `s1-gateway.jpg`, `s3-after-c1.jpg`, `s5-after-m2.jpg`, `s7-select.jpg`.
2. R3 has step 0 begin with the screenshot; the sheet's step 0 puts it after the process and port lines and before GET `/status`. It was taken in the sheet's order (20:38:54Z, after the port lines at 20:38:51Z); nothing had been sent to Tally before it.
3. The notes end at step 7: step 8's listing hashes them, so the step 8 line is here (20:43Z) and not in the notes. At step 7 one further `tasklist` for `bridge_mcp.exe` was typed and printed, not filed: none running.

## Observations, not findings

- Tally's filter box showed the typed text without its spaces (`BRIDGESTOCK`, then `BRIDGESTOCKCREATE`); the rows it left were as the sheet expects.
- C2's total time (0.023 s) is about four times C1's; every other read ended within 0.066 s, I1 the slowest first byte (0.064 s).
- `/status` at the close answered with the same 51 bytes as at step 0 (`TallyPrime Server is Running`).

## Not covered

As the sheet's Not covered: a master changed between bind and aim, a book with no stock item, a licensed edition for the count read, a Bridge build, other books, releases and sittings, and transport failure.

## The record

`evidence/` (every helper's output with its exit code, the send log, the state and compute, the two `/status` captures, `s8-listing.txt` and the notes), `answers/` (M1, M2, I1, I2, G1, G2, U1 and U2 exactly as received, and all ten headers and transport records). Kept on the lab PC, listed in `kept-local.txt` by size and sha256: the answers of C1 and C2 (they carry the book's company number), the screenshots, the `tasklist` and `netstat` output, the clone's fixtures in `local/gen/`, the self-tests' scratch, the stage and scan helpers and the commit message. Text files had lab and clone paths and the gate's desk words replaced; `substitutions.txt` counts every replacement.
