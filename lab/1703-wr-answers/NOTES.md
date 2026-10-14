# 1703-WR answers: the cost of one widened verification read (#1703), 15 Oct 2026

Not for merge. Synthetic company BRIDGE PILOT LAB only. Read only: nothing was written to Tally and no Tally
setting was changed. Item description and requests: branch lab/1703-wr-run-sheet at ad488247 (WINDOWS-1703-WR.md),
followed as written. Confidence: one run, PARTIAL.

## Setup
- The four request files here match the sheet's sizes and sha256, and every line of the sheet's SHA256.txt
  matched before the first send.
- TallyPrime 7.1 Silver, Education mode No, one tally.exe owning port 9001, BRIDGE PILOT LAB loaded alone
  (books from 2026-04-01, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85).
- Before any request was sent, starting Tally's window from the desktop-control tool opened a second tally.exe
  that did not own port 9001. In that copy the demo company was loaded by a keystroke, then BRIDGE PILOT LAB.
  Both were shut and that copy quit; nothing was entered or saved in it. BRIDGE PILOT LAB was then loaded alone
  in the port-9001 Tally, and every request below went there. One tally.exe was running for all sends.
- Each send is the sheet's curl command verbatim, one at a time, with no time or size limit. send-log.txt has
  one line per request.

## Results
| step | request | HTTP | bytes | seconds | VOUCHER rows | answer sha256 |
|---|---|---|---|---|---|---|
| 2a | r15 (2026-08-01) | 200 | 87,884 | 0.224 | 11 | b42a98cd... |
| 2b | r15 (2026-08-01) | 200 | 87,884 | 0.130 | 11 | b42a98cd... |
| 3a | w01 (2026-07-25 to 2026-08-08) | 200 | 250,716 | 0.102 | 35 | 994def1d... |
| 3b | w01 (2026-07-25 to 2026-08-08) | 200 | 250,716 | 0.060 | 35 | 994def1d... |

Every answer: HTTP 200, STATUS 1, no LINEERROR, parsed without error, under 16 MiB, every POST under 20 s.
The two answers of each step are byte-identical. /status between the sends of a step answered HTTP 200 (51 B).

- Step 0b (r01): exactly one company, BRIDGE PILOT LAB; Education No; Silver Yes; books from 2026-04-01.
- Step 1 (r02): one COMPANY row with that GUID; ALTVCHID (M) 68, ALTMSTID 245. M is within 1 to 170, so Bridge
  sends the window as one request with no census, as this item reproduces.
- Step 2: 11 vouchers, all dated 2026-08-01, all of type BRIDGE Sales, 53 ledger lines.
- Step 3: 35 vouchers (at least step 2's 11), all inside the window: 11 on 08-01, 7 on 08-02, 4 on 08-03,
  2 on 08-04, 1 on 08-05, 10 on 08-07; none from 07-25 to 07-31. By type: BRIDGE Sales 28, BRIDGE Sales Z1 1,
  Receipt 4, Purchase 2. 146 ledger lines. None cancelled, none optional. Not an envelope-only answer.
- Step 4 (r02 again): ALTVCHID 68 and ALTMSTID 245, unchanged; answer byte-identical to step 1.
- Step 5 (r01 again): the same single company and GUID; answer byte-identical to step 0b.

## Cost per POST, widened against one day
- Bytes: 250,716 against 87,884, that is +162,832 bytes (2.85 times).
- Voucher rows: 35 against 11 (+24); ledger lines 146 against 53.
- Seconds: every POST took under 0.25 s. The widened read was not slower in this run (0.102 and 0.060 s
  against 0.224 and 0.130 s). The first voucher read of the session was the slowest, so at this size the time
  difference is within run-to-run variation and is not measured.

## Retire estimate (about 4 times the step-3 POST plus the wrapper)
- About 1,002,864 bytes, 140 voucher rows and 0.24 to 0.41 s for the four widened POSTs on this book.
- Wrapper, as measured here: each company-list read 4,000 bytes and under 0.02 s, each /status 51 bytes and
  under 0.01 s.
- Every widened POST stayed far under Bridge's 20-second limit (largest 0.102 s), so no part would be halved.

## Files
- requests/: the four request files, byte for byte (UTF-16LE with a BOM).
- answers/: every answer (<step>.xml, UTF-16LE as Tally sent it), its headers (<step>.headers.txt) and the
  two /status bodies (<step>-status.txt).
- send-log.txt: one line per request.
- SHA256.txt: sha256 and size of every other file in this folder.
- Kept on the lab PC and not on this branch, by sha256 and bytes:
  - 265105b150c09d1b452c16ecbffb90c7cfee41a2895e333b0721d11a0a744ca4  43862  screenshot 01, step 0a (second copy, before it was quit)
  - 64d8040edacda2cd19d9ad809b11b9cf4b101ad8dfc73f81f8bb61fd7db06931  43192  screenshot 02, step 0a (port-9001 Tally, company loaded alone)

## Not established
- A book above the single-request size (M over 170, where Bridge counts the window first).
- Education mode's outward rounding.
- Bridge's own HTTP headers: curl's User-Agent and Accept differ from Bridge's; the body is the same.
- Timings under load, on a larger book, or on a cold Tally.
