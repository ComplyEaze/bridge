# Results: #1233, stock to a voucher's own date on BRIDGE TIEOUT D, 29 Oct 2026. **Both sources include the voucher dated on `SVTODATE`.**
- The item collection (`t1`) and the plain Stock Summary (`t2`) to 20251015 both read Lab Gadget **17 Nos**.
- Tally's own Stock Summary for 1-Apr-25 to 15-Oct-25 shows the same, twice. That includes the sale dated 15 Oct 2025, the book's last entry. 22 Nos would have meant it was left out.

Reads only; the marks did not move.

The sheet is `lab/1233-tieout-d-end-date` at c0faf7d9b40d8f2c36b4e3e760a373e95c24c6c6. `SHA256.txt` matched 6 of 6, and `check-request-ids.py` printed `census: ok 2, skipped 1, FAIL 0`.

**Who acted.** The lab, on the owner's standing word of 27 Oct, on the synthetic company. No `bridge_mcp` call was made; the reads were sent by curl under C3.

## The reads
Every read answered 200, with bytes equal to CONTENT-LENGTH and curl exit 0.

| read | request | bytes | sha256 | what it held |
|---|---|---|---|---|
| m-a | marks | 4,886 | 84e0a5be… | one COMPANY row, THE COMPANY's GUID; ALTMSTID 213, ALTVCHID 4, NUMSTOCKITEMS 1 |
| t1 | items to 20251015 | 4,356 | e1ff3aa1… | 1 STOCKITEM: Lab Gadget, parent Primary, CLOSINGBALANCE **17 Nos** |
| t2 | Stock Summary to 20251015 | 510 | 5bc8266d… | 1 line: Lab Gadget, DSPCLQTY **17 Nos** |
| t1r | items again | 4,356 | e1ff3aa1… | byte-identical to t1 |
| m-b | marks again | 4,886 | 84e0a5be… | byte-identical to m-a: the marks did not move |

`t2`'s amount equals `t1`'s closing value (exact decimals): yes. The figures are in `evidence/parsed-tables.txt`.

## Screen truth and the comparison
- **Stock Summary.** It opened with the period 1-Apr-25 to 15-Oct-25, and Alt+F2 set the same period. Two readings a minute apart both show Lab Gadget **17 Nos**.
- **Day Book for 15-Oct-25.** It lists one voucher: Sales, number 2, party Customer.
  - The item's quantity on it is not shown in that view, and the voucher was not opened (Enter opens its Alter screen).
- **Stock Item Vouchers (an added read, display only).** For Lab Gadget, 1-Oct-25 to 15-Oct-25, two readings a minute apart agree: opening 22 Nos; the one voucher, Sales no. 2 of 15-Oct-25, outwards **5 Nos**; closing 17 Nos. The 5 Nos change between 30 Sep and 15 Oct is that end-date voucher (`evidence/stock-item-vouchers-1.txt`, `-2.txt`, quantity columns only).
- **The Gateway** showed its step 0 period, date and last entry afterwards.

| source | quantity |
|---|---|
| `t1` CLOSINGBALANCE | 17 Nos |
| `t2` DSPCLQTY | 17 Nos |
| transcription 1 | 17 Nos |
| transcription 2 | 17 Nos |

## Reading
- **The end date is inclusive in both sources.** Both the item collection and the plain Stock Summary include a voucher dated on their end date, in agreement with Tally's own screen. By the sheet, the month-end change may state that the end date is inclusive.
- **Not measured:**
  - a month-end date with a voucher on it;
  - more than one voucher on the end date;
  - a Stock Journal or a purchase on the end date;
  - another book; TallyPrime 7.1.

**Confidence:** one sitting, PARTIAL until repeated.

## Files
- `evidence/`:
  - every raw answer with its headers, curl's printed fields and its error stream;
  - `send-log.txt`, `parsed-tables.txt`, `comparison.txt`, `daybook-20251015.txt`, `stock-item-vouchers-1.txt` and `-2.txt`;
  - the two transcriptions, with the quantity column only: the rate and value are in the copies kept on the lab PC, by sha256 in `substitutions.txt`;
  - the step 0 and C10 checks.
- `scripts/send.ps1` and `scripts/parse.py`.
- `substitutions.txt`, `notes.txt` (UTC), and `SCREENSHOTS.txt` (the captures, kept on the lab PC).
