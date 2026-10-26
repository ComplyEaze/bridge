# Results: 72-reads part B, the tool rows on master 609776ae, TallyPrime 7.2, 26 Oct 2026. B1 to B5 run; B6 not run. B2, B3 and B5 as expected. B1 is as expected except that the screen prints the value with (-) and the tool unsigned. B4 is as expected on TIEOUT B; on SHAPE LAB no item has two dated entries

The sheet is `lab-evidence/72-reads/README.md` and `PART-B-BUILD-609776ae.md` at 1043db35dbd7bf988a64b7386b1824b5f33769d9.

## Build and setting
- Merge-queue run 40366984202, head 609776ae9f5cec48e6563f165480d3fe1adae445, artifact 12670984633:
  - zip sha256 ae1f7267d49d099e3974cb724c1017fb1bf59501508d882718c332db129f2649 = the digest;
  - `.mcpb` cfd109a62004879a6c360d2e31648dfa68ce7733a9a09c22cd03793c005f941b = `archive_sha256` in `mcpb-smoke.json`;
  - `bridge_mcp.exe` 4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44 = `binary_sha256`. It was the same before the first call and after the last.
- **One launch for the whole part** (`client-log.txt`: launched 18:40:11 UTC, ended 18:50:39 UTC). It ran over MCP stdio with `BRIDGE_TERMS_ACCEPTED=true`, host 127.0.0.1, port 9001 and a new empty data folder. `BRIDGE_AGENT_ENABLE_WRITES`, `_ENABLE_IMPORT`, `_MAX_BYTES` and `_REDACTION` were unset.
- `tools/list` (`answers/000-tools-list.json`): server `bridge-tally` **0.6.0**, 29 tools. It includes `negative_stock`, `negative_ledgers`, `profit_and_loss`, `balance_sheet`, `masters` (kind `stock_items` in its enum), `vouchers` and `voucher_presence`.
- TallyPrime 7.2, Silver, not Educational mode.

## Recording
Every call was one at a time, and each answer was saved whole (`answers/NNN-*.json`, with its sha256 in `client-log.txt` and `SHA256.txt`). Each row ran this way:
1. The book was loaded alone from Select Company, by arrow keys and Enter.
2. `list_companies` listed exactly that company.
3. `ledger_masters` was read for the marks before and after the row: first page, no `snapshot_id`, `fields` basic, or compliance on SHAPE LAB. Every pair was equal, with `snapshot.reused` false.
4. The books were shut between companies.

All windows are 1 April 2025 to 31 March 2026 (the part A requests' window), except where a row names another.

| book (number) | GUID | marks v / m |
|---|---|---|
| BRIDGE NEGSTOCK LAB 2 (100071) | 551f8339-e023-4854-8d16-dac01f47bffc | 3 / 214 |
| BRIDGE SHAPE LAB (100021) | 3a6bd6e1-b835-4bff-89dd-8a6af138c346 | 112 / 292 |
| BRIDGE TIEOUT B (100040) | d64eda55-da7c-4106-a171-3e3cb6cdd331 | 3 / 218 |
| BRIDGE TIEOUT C (100069) | 824929d5-39e3-4ab7-a67d-a4efc503e24d | 3 / 213 |
| BRIDGE TIEOUT D (100075) | 6a9518b2-a88c-417c-a700-40243f9d9119 | 4 / 213 |

## Rows

| row | call | expected | seen | answer |
|---|---|---|---|---|
| B1 | `negative_stock` NEGSTOCK LAB 2, year | one item, -3 Nos, rate 120.00, value 360.00 | `observed`, count 1: Lab Short Item, quantity -3 Nos (read), rate present 120.00, value present 360.00 | 003 |
| B1 | `negative_stock` NEGSTOCK LAB 2, to 31 May 2025 | -5 Nos, rate and value empty | count 1: Lab Short Item -5 Nos, rate `empty`, value `empty` | 004 |
| B1 | `negative_stock` TIEOUT C, year | `not_established` | `not_established`, count 0 | 033 |
| B2 | `negative_ledgers` SHAPE LAB, year | eleven ledgers on both sides in Tally's order | `observed`, count 11 (below) | 008 |
| B2 | `negative_ledgers` TIEOUT B, year | `not_established` | `not_established`, count 0 | 020 |
| B2 | `negative_ledgers` TIEOUT C, year | `not_established` | `not_established`, count 0 | 034 |
| B3 | `profit_and_loss` / `balance_sheet` TIEOUT B, year | `closing_stock` applied (2,100.00), statements established | `closing_stock` applied, value **"-2100.00"** (source `tally_profit_and_loss_less_closing_stock`). P&L gross_result established 500, net_result established 0. Balance sheet: carried P&L established 0. Gate lines matched | 024, 025 |
| B3 | the same, TIEOUT C | `closing_stock` declined, `opening_stock_not_shown_empty`, statements refused as before | declined, `opening_stock_not_shown_empty`; state `not_established` (gross, net and carried: `tally_balance_sheet_differs`, lines Profit & Loss A/c and Current Assets) | 035, 036 |
| B3 | the same, TIEOUT D | as TIEOUT C | as TIEOUT C | 040, 041 |
| B4 | `masters` `stock_items` SHAPE LAB | eleven items, one with two dated HSN and GST entries | 11 items. **No item has two dated entries:** every `hsn` list is `entries []`. Every `gst` list is empty except Soda Ash Light's: one entry from 20250401, Specify Details Here, Taxable, CGST 9 / SGST/UTGST 9 / IGST 18 / Cess absent | 012 |
| B4 | `masters` `stock_items` TIEOUT B | three items with default entries withheld as `tally_default_entry` | 3 items, Lab Widget A, B and C. Each has one HSN entry and one GST entry from 20250401, source "As per Company/Stock Group", with `ineligible_itc_withheld` `tally_default_entry` | 029 |
| B5 | `vouchers` SHAPE LAB, 5 June 2025 | the journal's cost-centre rows (Business Line: Trading -600.00, Assembly -400.00); an entry with none has no such key | The journal (alter 23, narration SHAPELAB-C9-costcentre) has Power Charges -1000.00 with `cost_centre_allocations` Business Line Trading -600.00 and Assembly -400.00. The Chem Supplier 3 entry (1000.00) has no `cost_centre_allocations` key | 016 |
| B6 | `voucher_presence` | | **not run**: the sheet names no ledgers for the proposed voucher, and the run was limited to B1 to B4 (B5 was added as SHAPE LAB was loaded) | |

B2 SHAPE LAB, as the tool returned it (ledger, side, amount): Bank of Baroda CA credit 24270.50; Capital A/c debit 1000000.00; Cess debit 200.00; CGST debit 3600.25; HDFC CC debit 252600.00; IGST debit 1800.00; Shape Buyer 1 credit 1180.00; Shape Buyer 2 credit 8875.00; Shape Buyer 3 credit 9090.00; State Tax debit 3600.25; Working Capital Loan debit 500000.00.

## Screen comparisons (after the call had returned, at Tally's own report, then back to the Gateway)
- **B1, Negative Stock** (Display More Reports > Exception Reports > Negative Stock). It opened at the current period, 1-Apr-25 to 20-Jun-25; 20-Jun-25 is the book's last entry. It showed Lab Short Item **(-)3 Nos, 120.00, (-)360.00**, Grand Total (-)3 Nos (-)360.00. Quantity and rate equal the tool. **The screen prints the value with (-); the tool returns "360.00" unsigned.** That is recorded as a difference. The period was not changed on this screen.
- **B2, Negative Ledgers.** It opened at the current period, 1-Apr-26 to 20-Apr-26, with 12 rows (Profit & Loss A/c 1,37,150.01 Dr among them). The report's period was then set by F2 to 1-4-2025 to 31-3-2026. The screen then shows **the same 11 ledgers in the same order, on the same side, with the same amounts** as the tool. Grand Total Dr 17,61,800.50, Cr 43,415.50. The Gateway's current period was unchanged afterwards.

## Not covered
- B6.
- No screen step was named for B3 to B5.
- The difference on B4 SHAPE LAB is not explained here. The book may have changed since the expectation was written.

Nothing was written or imported; there was no dialog. Screenshots stay on the lab PC. The working notes are `notes.txt`.
