# Results: row 2162-S2, GST status flags straight after an invoice's import, 3 Nov 2026

**All three read-backs are byte-identical.** The read-back sent at once after the import's answer (00s) equals the ones at +5 s and +60 s. Each holds one row, BP/26-27/0046, with included Yes, uncertain No and excluded No, and post-dated, cancelled, optional and GST-overridden all No. Each flag appears once, as a direct child of the voucher. Every EXPECT held. [verified]

Sheet: `lab/2162-s2-sheet` at a0d4c53b, all 7 files equal to `SHA256.txt`. No Bridge build. BRIDGE PILOT LAB (synthetic), loaded alone. Tally's screen shows TallyPrime **Silver**; the release, 7.2, is from `tally_status` on the same tally.exe (PID 1200) earlier in the sitting. The Gateway's current date read 15-Sep-2026, after 1 Aug 2026, so the invoice dated 1 Aug 2026 is not post-dated. The Gateway also showed "(e) Data exceptions exist" beside the company before the row and after the restore; this row does not explain it. [verified]

## Deviations, each agreed before the step
- **The content type.** The sheet's `run.py` sends with `Content-Type: text/xml`. In S1 that header drew `Unknown Request` (59 B, HTTP 200, curl exit 0), and the import is sent once only. So step 3 ran `run-c3.py`: `run.py` with that one header changed to `text/xml; charset=utf-16`, LAB-COMMON C3's. The diff is `evidence/run-c3.diff`:
  ```
  12c12
  <     done = subprocess.run(["curl.exe", "-sS", "--fail-with-body", "-H", "Content-Type: text/xml",
  ---
  >     done = subprocess.run(["curl.exe", "-sS", "--fail-with-body", "-H", "Content-Type: text/xml; charset=utf-16",
  ```
  Before step 3, step 2's answer was checked as an ENVELOPE, not `Unknown Request`. Every other send also went with C3's header (`send.py`). [verified]
- **The marks request.** The sheet names `r02-marks.xml` of `lab/1578-postdated`. That is the folder `lab/1578-postdated/requests/` in branch `lab/1578-postdated-answers` (d4323cea). The file is byte-equal to S1's copy (81892a74…). [verified]
- **Other processes.** Four `bridge_mcp` processes ran under the desktop client as its connectors, idle, as in S1. They took no call. Whether they sent anything to Tally was not measured. [unverified]

## Steps
1. **Marks:** V 68, M 245, byte-identical to S1's starting answer (8a0c4268…). The backup was taken by Tally's screen to a new empty local folder: one file, 90,051 B, sha256 7415004d…, 09:43 local, kept on the lab PC. The Gateway date read 15-Sep-2026. [verified]
2. **Requests:** `gen.py BP/26-27/0046`, run in a copy, wrote `I-0046.xml` and `R-0046.xml` byte-equal to the sheet's. `R-0046.xml` was sent once: an ENVELOPE with no VOUCHER row (3,022 B), so the number was unused. [verified]
3. **The import and read-backs** (`run-c3.py http://127.0.0.1:9001 0046`), with `times-0046.txt` as written:
   ```
   I-0046.xml	a-import-0046.xml	sent +-3.984s	returned +0.000s	curl 0
   R-0046.xml	a-readback-0046-00s.xml	sent +0.000s	returned +0.375s	curl 0
   R-0046.xml	a-readback-0046-05s.xml	sent +5.016s	returned +5.266s	curl 0
   R-0046.xml	a-readback-0046-60s.xml	sent +60.000s	returned +60.157s	curl 0
   ```
   [verified]
4. **Marks:** V 69, M 245. The voucher mark moved by the one import only. [verified]
5. **Restore:** Tally's Restore screen, from that folder. The list showed BRIDGE PILOT LAB at 3-Nov-26 9:43 am. Two prompts came, `Accept ?` (Yes) and the folder-exists prompt naming the company's own number (Replace), and no other. Marks after: **byte-identical to step 1's** (8a0c4268…). [verified]

## The import's answer
`CREATED` 1, `ALTERED` 0, `DELETED` 0, `COMBINED` 0, `IGNORED` 0, `ERRORS` 0, `CANCELLED` 0, `EXCEPTIONS` 0, `LASTVCHID` 45, `LASTMID` 0, no `LINEERROR`. [verified]

## The read-backs
| read-back | bytes | sha256 | rows | `ALTERID` | GUID | `ISCANCELLED` | `ISOPTIONAL` | `ISPOSTDATED` | INCLUDED / UNCERTAIN / EXCLUDED | `ISGSTOVERRIDDEN` |
|---|---:|---|---:|---:|---|---|---|---|---|---|
| 00s | 94,478 | 19799554… | 1 | 69 | …-0000002d | No | No | No | Yes / No / No | No |
| 05s | 94,478 | 19799554… | 1 | 69 | …-0000002d | No | No | No | Yes / No / No | No |
| 60s | 94,478 | 19799554… | 1 | 69 | …-0000002d | No | No | No | Yes / No / No | No |

- The rows were counted by an XML parser, and each flag as a direct child of the voucher, each exactly once. The row is BP/26-27/0046 dated 1 Aug 2026; the GUID begins with the company's. `evidence/check.txt` has every value. [verified]
- 00s equals 05s and 05s equals 60s, byte for byte. [verified]

## Decides
All three are equal. So, as the sheet's Decides line says, the read-back straight after a post reads the flags Tally keeps, item 2 closes, and the post reads without a wait. The code's own read comes later than 00s. [verified for this invoice, this book, this release]

## Kept
- The answers as Tally sent them: the marks, the probe, the import and the three read-backs. None holds the company number.
- `times-0046.txt`, `run-c3.py`, its diff, `send.py`, the send log, the check output and the restore plan.
- On the lab PC only: the backup, the captures and the data-folder listings, listed in `kept-local.txt`.

## The book now
BRIDGE PILOT LAB restored, marks byte-identical to step 1. Shut, no company loaded, one tally.exe on 9001.

**Confidence:** one book, one invoice, one sitting, 7.2: PARTIAL.
