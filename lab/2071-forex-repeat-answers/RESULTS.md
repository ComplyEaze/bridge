# Results: #2071 forex repeat, 29 Oct 2026 (second sitting of the forex rerun's step 3 on BRIDGE FOREXGROUP LAB, BUILD M). The three refusals repeat exactly, in code, cause, remediation and `bill_row`. They differ from `lab/2071-forex-rerun-answers` at 0d7eb0bf only in `evidence.bytes`, which is 22 to 28 bytes smaller. A raw read from the same sitting differs in the same way, and the difference is traced to Tally's CMPINFO count header, not to any row. Reads only; the marks are unchanged

The sheet is `lab/2071-forex-rerun` at 52b3b19af43ab3765d8e480ee631f35d266e2976 (step 3 only, then the sheet's own raw marks request).

**Build.** BUILD M is master af4a0dbf7f7d6b9432b2b6f0994c217c69c8279e, run 40671680104.
- `bridge_mcp.exe` 9f73d533dcac8a24d1d33389a7db33cf4055e212f7dc97f8c170f00c08b73fd1, the same at the end.
- Server 0.6.1.
- New data folder; writes off.

**Company.** BRIDGE FOREXGROUP LAB, 100090, 1bb48be5-baba-45b2-90d8-7593244915a9, `verified_tuple`, loaded alone. It holds the rerun's four vouchers. Tally's current date was 1-Apr-2025.

| read | this sitting (02:57Z) | the rerun (0d7eb0bf, 02:16Z) | difference |
|---|---|---|---|
| `trial_balance` 2025-04-01 to 30 | `trial_balance_read_failed`, cause `trial_balance_amount_invalid`, the same remediation; 0.84 s; evidence 54,327 B | the same; 0.76 s; 54,349 B | `evidence.bytes` only (-22) |
| `outstandings` as of 2025-04-30 | `native_outstandings_read_failed`, cause `native_amount_invalid`, `bill_row` {payable, 1}, the same remediation; 1.46 s; evidence 168,094 B | the same; 1.59 s; 168,122 B | `evidence.bytes` only (-28) |
| `ledger_masters` compliance | `party_ledger_master_read_failed`, cause `native_foreign_currency_ledger_balance`, no remediation; 1.30 s; evidence 79,987 B | the same; 1.32 s; 80,013 B | `evidence.bytes` only (-26) |
| `list_companies` | as before | | none (`evidence.bytes` 8,040 both, the same fingerprint) |

The comparison leaves out read times, durations and fingerprints.

## Marks
**The tool cannot read the marks on this book.** `ledger_masters` basic refused twice (before and after the reads):
- `ledger_export_invalid`, cause `company_several_currency_masters`.

This is new to the record. In the rerun the marks were read before any voucher existed (by the compliance read) and by raw request.

The marks were therefore read once, after the reads, with the sheet's own committed request `requests/marks.utf16le.xml` (sha256 5b447d03…):
- 200, 3,402 B = CONTENT-LENGTH;
- **ALTMSTID 211, ALTVCHID 4**, equal to the rerun's M1.

**Where the size difference comes from.** That answer is 2 B smaller than the rerun's M1 (3,404 B). The only differences between the two are in the CMPINFO count lines above the rows:
- LEDGER 13 → 0;
- CURRENCY 4 → 0;
- TAXUNIT 4 → 0;
- VOUCHERNUMBERSERIES 4 → 0;
- VOUCHER 8 → 0.

LAB-COMMON C4 says these lines are not counts of anything read. The tool answers' smaller `evidence.bytes` fit the same header change in each request they fold in. This is inferred; the raw answers of the tool's own reads were not kept.

**Confidence.** The three refusals are now seen in two sittings.

## Files
- `answers/`: every tool answer whole, with UTC times.
- `client-log.txt`.
- `notes.txt` (UTC).
- `raw/`: the marks answer with its headers, its tree, and the diff against the rerun's M1.
- `checks/`: tasklist and status at the start and end.
- `SCREENSHOTS.txt`.
