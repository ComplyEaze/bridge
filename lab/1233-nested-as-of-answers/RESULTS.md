# Results: #1233, nested stock quantities at two dates inside the year, BRIDGE NESTED STOCK LAB, 29 Oct 2026. **Every condition of the finding holds.**
- `a1` and `b1` each list all eight items with their parents.
- Every nested quantity equals both transcriptions of Tally's expanded Stock Summary at its date.
- C1 and B1 read 40 and 30 Nos at 31 Aug, and 35 and 35 Nos at 30 Sep.
- Each report's amount sum equals its date's `CLOSINGVALUE` sum.

Reads only; the marks did not move; every setting was put back.

The sheet is `lab/1233-nested-as-of` at c42cfeff54972eaf587e11878f58767c741370cd. `SHA256.txt` matched 9 of 9, and `check-request-ids.py` printed `census: ok 3, skipped 2, FAIL 0`.

**Who acted.** The lab, on the owner's standing word of 27 Oct, on the synthetic company. No `bridge_mcp` call was made; the reads were sent by curl under C3.

**One difference from the sheet's note (recorded, not a stop rule).** The Gateway opened with the period 1-Apr-26 to 31-Mar-27 and the current date 15-Apr-2026 (last entry 15-Apr-26). The sheet noted 1-4-2025 to 31-3-2026 and 15-9-2025 from the capture sitting. The book's marks are unchanged (ALTMSTID 219, ALTVCHID 2, 8 items), and every request names its own dates.

## The reads
Every read answered 200, with bytes equal to CONTENT-LENGTH, curl exit 0 and within its bound.

| read | request | bytes | sha256 | what it held |
|---|---|---|---|---|
| n3-a | company flags | 4,918 | 847d99be… | one COMPANY row, THE COMPANY's GUID; ALTMSTID 219, ALTVCHID 2, NUMSTOCKITEMS 8 |
| a1 | items to 20250831 | 13,952 | 1f012bcb… | 8 STOCKITEM rows |
| a2 | Stock Summary to 20250831 | 948 | 0ab20c15… | 2 lines |
| b1 | items to 20250930 | 13,952 | 53aa65c8… | 8 STOCKITEM rows |
| b2 | Stock Summary to 20250930 | 948 | 338b20a3… | 2 lines |
| a1r | items to 20250831 again | 13,952 | 1f012bcb… | byte-identical to a1 |
| n3-b | company flags again | 4,918 | 847d99be… | byte-identical to n3-a: the marks did not move |

The rows, the report lines and the two amount sums are in `evidence/parsed-tables.txt`. Each report's `DSPCLAMTA` sum equals its date's `CLOSINGVALUE` sum: **yes** at 31 Aug, **yes** at 30 Sep.

## Screen truth and the comparison
- **Setting up.** Stock Summary opened at 1-Apr-26 to 15-Apr-26. Under F12 the BEFORE values were Format of Report **Condensed** and Expand all levels **No**; they were set to Detailed and Yes.
- **Readings.** Each date was transcribed twice, a minute apart, and the two readings agree at both dates.
- **Putting back.** Afterwards the period and both settings were set back and captured, and the Gateway showed its step 0 period and date.

| item | parent | 31 Aug: row / readings | 30 Sep: row / readings |
|---|---|---|---|
| Lab Depth1 Nut | Lab Group One | 20 Nos / 20, 20 | 20 Nos / 20, 20 |
| Lab Depth1 Powder | Lab Group One | 2.500 Kgs / 2.500, 2.500 | 2.500 Kgs / 2.500, 2.500 |
| Lab Depth2 Resin | Lab Group Two | 1.250 Kgs / 1.250, 1.250 | 1.250 Kgs / 1.250, 1.250 |
| Lab Depth2 Spare | Lab Group Two | empty / not on the screen | empty / not on the screen |
| Lab Depth2 Washer (B1) | Lab Group Two | **30 Nos** / 30, 30 | **35 Nos** / 35, 35 |
| Lab Depth3 Oil | Lab Group Three | -0.750 Kgs / (-)0.750, (-)0.750 | -0.750 Kgs / (-)0.750, (-)0.750 |
| Lab Depth3 Pin (C1) | Lab Group Three | **40 Nos** / 40, 40 | **35 Nos** / 35, 35 |
| Lab Root Bolt | Primary | 10 Nos / 10, 10 | 10 Nos / 10, 10 |

Lab Depth2 Spare has no quantity in the row, and the report hides zero items (its setting is No), so it is not on the screen at either date.

## Reading
- **Nested items follow the date.** The item collection follows `SVTODATE` inside the year for items at depths one to three, not only under the root. SJ1's two items read their quantity before SJ1 at 31 Aug and after it at 30 Sep, and every nested quantity agrees with Tally's own expanded screen. By the sheet's finding, the date change **may** return nested quantities at a date inside the year on this evidence.
- **Not measured:**
  - a window across two financial years;
  - a date before the books begin;
  - sales or purchase vouchers, godowns, batches;
  - another book; TallyPrime 7.1.

**Confidence:** one sitting, PARTIAL until repeated.

## Files
- `evidence/`:
  - every raw answer with its headers, curl's printed fields and its error stream;
  - `send-log.txt`, `parsed-tables.txt`, `comparison.txt`;
  - the four transcriptions, with the quantity column only: the rate and value columns are in the copies kept on the lab PC, by sha256 in `substitutions.txt`;
  - the step 0 and C10 checks.
- `scripts/send.ps1` and `scripts/parse.py`.
- `substitutions.txt`, `notes.txt` (UTC), and `SCREENSHOTS.txt` (the captures, kept on the lab PC).
