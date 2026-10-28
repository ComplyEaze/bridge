# Results: #485 slice 2, `ledger_movement` for one ledger on the built tool (#2114), 28 Oct 2026. On BRIDGE CORPUS DENSE every scoped read matched its comparison exactly. A year of DN Party 001 (248 vouchers), refused before, was answered `ledger_scoped` with debit and credit equal to the Trial Balance. A name outside the scoped alphabet, and a ledger the scoped read found empty, both fell back to the book's window and said so. Reads only; the marks did not move

The sheet is `lab/485-ledger-movement-row` at 7f8282ea65641d9896328cfcf35306d16a6943e1.
- **Sheet checks:** `SHA256.txt` 3 of 3 equal.
- **Build:** #2114's Windows bundle at a645572c46ac258e7ad4b67fa6a0fad18e2f01f3, Bridge CI run 40733678118, artifact `unsigned-bundle-smoke-windows-latest`.
  - `bridge-tally.mcpb` sha256 ece32b460bfb3974ae7c0e9aa5046ccfe08a087a33e70e226a6f17dff7588929 (15,353,027 B).
  - `bin/x86_64-pc-windows-msvc/bridge_mcp.exe` sha256 4b805994b688c1a05104193e890a161f57273bcd357593c1dc6db10a9bcd35c8 (31,851,520 B), the same before and after.
  - Both equal the run's `mcpb-smoke.json`. The server reports 0.6.1. Redaction off (the default).
- **Tally:** TallyPrime 7.2 (Latest), Silver Rental, licensed. `tally_status`: education_mode false, silver.
- **Company:** BRIDGE CORPUS DENSE, 100012, GUID d45bc1b0-e5e3-4261-b3b2-cce3915f42d3 (part E's), `verified_tuple`, the only company listed.
- **Marks:** ALTVCHID 29900, ALTMSTID 328 at the start and the end (`ledger_masters` snapshot, `reused` false).
- **Calls:** every call answered; none was refused. The largest `evidence.bytes` was 6,873,172 (step 13), under the 40,000,000 bound. No dialog appeared.

## Steps
| step | call | window_scope | vouchers read / rows | figures (opening, debit, credit, closing, touching) | compared with | equal |
|---|---|---|---|---|---|---|
| 2 | trial_balance May 2025 | | 123 ledgers | DN Party 001: -864067.60, -933428.20, empty, -1797495.80; DN Sales: 0.00, empty, 113726661.73, 113726661.73 | | |
| 3 | ledger_movement DN Party 001, May 2025 (7.1 s) | ledger_scoped | 21 / 21 | -864067.60, -933428.2, 0, -1797495.8, 21 | step 2 | yes |
| 4 | trial_balance year | | 123 ledgers | DN Party 001: 0.00, -11815459.60, empty, -11815459.60 | | |
| 5 | ledger_movement DN Party 001, year (13.7 s; no `read_cost` field) | ledger_scoped | 248 / 248 | 0.00, -11815459.6, 0, -11815459.6, 248 | step 4 | yes |
| 6 | ledger_movement DN Sales, 15 May 2025 (16.2 s) | ledger_scoped | 82 / 82 | 0.00, 0, 4520659.59, 4520659.59, 82 | step 7 | yes |
| 7 | ledger_movement every ledger, 15 May 2025 (one page, limit 500) | none (no ledger) | 82 rows observed; 123 ledgers | DN Sales: 0.00, 0, 4520659.59, 4520659.59, 82 | | |
| 8 | ledger_movement `Profit & Loss A/c`, 15 May 2025 | **whole_book**, reason `ledger_name_outside_scoped_alphabet`, with a note | 82 | 143578182.71, 0, 0, 143578182.71, 0 | step 7's row | yes |
| 9 | ledger_movement `dn party 001`, May 2025 | ledger_scoped | 21 / 21 | as step 3; `ledger_match` `case_or_spacing` naming `DN Party 001` | step 3 | yes |

The counts are as part E found: 21 in May, 248 in the year, 82 on the day.

## Steps 10 to 14
- **Step 10, classes:** `vouchers` for 2025-04-01, 2025-05-15 and 2026-03-31, one page each, 82 vouchers each.
  - Every voucher is a Sales voucher with `cancelled` false, `optional` false, `post_dated` false and `is_invoice` false. No ledger line has a zero amount.
  - Each party line carries one bill allocation (New Ref DN-INV-..., 30 days), so these are accounting vouchers with allocations, not invoices.
  - Cancelled, optional, post-dated, invoice (with allocations) and zero-amount line are all **not present** on these days.
- **Step 11, parity sweep on 2025-04-01.** The three days tie at 82; 1 April was taken because 15 May was already compared in steps 6 and 7.
  - The whole-book read gave 123 ledgers, 83 of them touched.
  - Chosen ledgers: the 6 most touched (DN Sales 82, then five parties at 1 each); the 6 fewest but not zero (parties at 1, the same names, since every party has one voucher that day); and Cash. That makes 8 distinct ledgers.
  - Every one equals the whole-book row exactly on opening, debit, credit, closing and `vouchers_touching`.
  - DN Sales and DN Party 002 to 007 were `ledger_scoped`.
  - **Cash** (0 vouchers that day) came back `whole_book` with reason `ledger_scope_empty_not_established`. Its note: "The ledger-scoped read found no vouchers for the named ledger in the period, which is not taken as proof that there are none: the whole book's vouchers for the period were read and checked instead." Its figures equal the whole-book row (0.00, 0, 0, 0, 0).
  - This is the "scoped read that finds nothing" fallback, observed live on this book; the sheet's Not covered list expected it to be covered by tests only.
- **Step 12:** DN Sales is on 82 of 82 vouchers on 2025-04-01. Its scoped read (step 11 #01: 0.00, 0, 180557.03, 180557.03, 82) equals the whole-book row, as step 7 found for 15 May.
- **Step 13:** DN Party 002, the fewest-but-not-zero ledger (alphabetical among the ties), for the year.
  - `ledger_scoped`, 249 vouchers read: 0.00, -11826171.09, 0, -11826171.09.
  - This equals step 4's Trial Balance row (debit -11826171.09, credit empty) exactly.
- **Step 14:** `ledger_masters` (one page, 123 ledgers): no ledger name holds one of the formula words. Recorded "none"; nothing was sent.
- **Step 15:** `tally_status` the same as at step 0; marks 29900 / 328, equal.

**Confidence:** one sitting, PARTIAL until repeated.

**Not covered:**
- a day too heavy for one scoped read;
- the five voucher classes, absent from the compared days, so covered by the tool's tests only;
- a formula-word ledger name, none on this book, so covered by the tool's tests only.

## Files
- `answers/`: every tool answer whole, 000 to 027, named `NNN-t<step>-<tool>`.
- `answers-index.txt`: per answer, the file size and sha256, tool, arguments, send time and seconds, `isError`, `window_scope` without its note, `ledger_match`, `voucher_rows_observed`, `evidence.bytes`, state, reason_code and `response_sha256`.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256; they stay on the lab PC.
