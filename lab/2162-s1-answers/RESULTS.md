# Results: row 2162-S1, `ISPOSTDATED` on a post-dated invoice through the read-back request, 3 Nov 2026

**Yes, once.** R reads one VOUCHER row with `ISPOSTDATED` Yes, once, as a direct child of the voucher, and included Yes, uncertain No, excluded No. The control reads No. Each pair is byte-identical, and R with its `ISPOSTDATED` line and line break removed equals C byte for byte. Every EXPECT held.

Sheet: `lab/2162-s1-sheet` at 413f8b92, every file equal to its `SHA256.txt`. No Bridge build. BRIDGE PILOT LAB, synthetic, loaded alone. **TallyPrime 7.2, Silver**, licensed, as Tally's screen shows. That is the row's one difference from the earlier reads, which ran on 7.1.

## Steps
1. Marks: V 68, M 245, byte-identical to the 13 Oct run's start. Backup taken by Tally's screen, kept on the lab PC. The Gateway's current date read 15-Sep-2026.
2. F2 at the Gateway: current date 10-Aug-2026.
3. Keyed on Tally's screen: BRIDGE Sales, dated 25-Aug-2026, marked Post-Dated with Ctrl+T before saving. Status read Post-Dated. BRIDGE Walk-in; `BRIDGE Svc 998313 5%` 1,000.00; `BRIDGE CGST 2.5%` 25.00 and `BRIDGE SGST 2.5%` 25.00, as Tally computed; total 1,050.00. That is the shape of BP/26-27/0001, read off its screen first. Each tax ledger was chosen by typing until one row was left; no "Too many choices" box came up. Saved with no prompt. Two sub-screens, Dispatch Details and Party Details, were accepted as offered. Marks after: V 69, M 245.
   - **The number.** The number field stayed empty and did not take Enter: this type numbers by hand on this book, so Tally gave no number. I typed `BP/26-27/0045`, the number keyed on 13 Oct and the one in the sheet's `R-0045.xml` and `C-0045.xml`. `gen.py` was not run. The screen and the answers read BP/26-27/0045.
4. Not needed: the number is BP/26-27/0045.
5. R twice, then C twice; 6. the control `R-0010.xml` once.
7. Marks: V 69, M 245. Restored, then marks again: **byte-identical to step 1's**. After the reload the Gateway read 15-Sep-2026, step 1's date, so F2 was not needed.

**How the requests were sent.** The sheet says content type `text/xml`. With bare `text/xml` the first marks request drew `<RESPONSE>Unknown Request, cannot be processed</RESPONSE>`, 59 B; that answer is kept as `marks-0-bare-text-xml`. Every request after it went with LAB-COMMON C3's header, `text/xml; charset=utf-16`, the files as they are, BOM included. [verified]

## Answers
| send | bytes | sha256 | rows | `ISPOSTDATED` direct children | INCLUDED / UNCERTAIN / EXCLUDED | `ISCANCELLED` | `ISOPTIONAL` | `ISGSTOVERRIDDEN` | `ALTERID` |
|---|---:|---|---:|---|---|---|---|---|---:|
| R-0045, twice | 65,050 | 8da0e010… | 1 | 1, Yes | Yes / No / No | No | No | No | 69 |
| C-0045, twice | 64,946 | 1706e6b6… | 1 | 0 | Yes / No / No | No | No | No | 69 |
| R-0010, control | 94,474 | af9cd780… | 1 | 1, No | Yes / No / No | No | No | No | 17 |

Counted by an XML parser. Both rows of R and C are BP/26-27/0045 on 25 August. [verified]

- The R pair and the C pair are each byte-identical. [verified]
- R has exactly one `ISPOSTDATED` line, `<ISPOSTDATED TYPE="Logical">Yes</ISPOSTDATED>`. With that line and its line break removed, R equals C byte for byte, as on 21 Oct. [verified]
- The control equals 21 Oct's answer (`lab/1578-postdated-answers`, `ans-r1-1.xml`) except five counters in the answer's header: `LEDGER`, `CURRENCY`, `TAXUNIT`, `VOUCHERNUMBERSERIES`, `VOUCHER`. That row noted Tally fills these from what its session has loaded. [verified]

## Decides
Per the sheet: Yes once, so #2162 item 1's code can fetch the element and decide the state with `status_of_voucher`, after #1828 item 1.

## Other processes
The sheet says to close every other Bridge process first. Four `bridge_mcp` processes run under the desktop client as its connectors, and they were left running, as on row #2176 R0: a deviation. No call was made to them in this row, and every request was sent by curl. Whether they sent anything to Tally during it was not measured. [unverified]

## The book now
BRIDGE PILOT LAB as at the start, by the marks. Company shut, no company loaded, one tally.exe on 9001.

**Confidence:** one book, one voucher, one sitting, 7.2: PARTIAL.
