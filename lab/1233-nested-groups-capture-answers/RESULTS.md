# Results: #1233 item quantities inside nested stock groups against Tally's Stock Summary, on master 609776ae and TallyPrime 7.2, 27 Oct 2026. The sheet ran; every comparison is equal

The sheet is `lab/1233-nested-groups-capture/README.md` at 23c676dda4ba99027304d84adfe7d83c400ee710 (SHA256.txt: 7 of 7 equal). Request-ID check: `census: ok 2, skipped 1, FAIL 0` before step 2 and `ok 3, skipped 1, FAIL 0` with `n3-company-flags`.

## Build and setting
- `bridge_mcp.exe` of master 609776ae (run 40366984202, artifact 12670984633), sha256 4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44, before the first call and after the last. It reports `bridge-tally` 0.6.0. `tools/list` holds 29 tools, among them `list_companies`, `ledger_masters`, `masters` and `stock_summary`.
- One launch over MCP stdio, 11:04:47 to 12:22:33 UTC (`client-log.txt`). The settings were `BRIDGE_TERMS_ACCEPTED` true, host 127.0.0.1, port 9001 and a new data folder; writes, import, max bytes and redaction were unset. The server's error stream stayed empty.
- TallyPrime 7.2, Silver.
- The company: BRIDGE NESTED STOCK LAB, created at step 1, books from 1 April 2025, inventory on, GST off. Company number 100086, GUID 4b6094e2-d6db-46b6-a4ad-90c283cd1b52. It is kept on the lab PC.

## The book (keyed on Tally's screens, each master captured before its accept)
- Units `Nos` (0 decimals) and `Kgs` (3 decimals).
- Stock groups Lab Group One (Primary), Lab Group Two (in One), Lab Group Three (in Two). "Should quantities of items be added" was offered No for the group under Primary and Yes for each group under a group; each was kept as offered.
- Eight items exactly as the sheet's table. C2's opening quantity `-0.750` was taken, shown as (-)0.750 Kgs, and the screen computed (-)45.00. B3 has no opening.
- SJ1 (15 Sep 2025) and SJ2 (15 Apr 2026) were each keyed as 5 Nos from Lab Depth3 Pin into Lab Depth2 Washer, at the rates the screen offered (0.50 and 1.00).
  - SJ2's date was first refused: "Date cannot be above the current period (15-Sep-25)".
  - To key it, the company's period was set to 1-4-2025..31-3-2027 and its current date to 15-4-2026. SJ2 was keyed, and both were then set back to 1-4-2025..31-3-2026 and 15-9-2025.
  - The marks were read after each change and did not move (219 / 1 before SJ2; 219 / 2 after).
  - SJ2 took number 1 of the new year.
- Marks: M0 206 and V0 null; after the masters M1 219, V1 null; after SJ1 V 1; after SJ2 M2 219, V2 2.
- BK0: `TDBK1800_100086.001`, 28,269 bytes, sha256 f22cd944a626be0d4297a4bc52b7b302dc35db7b0b46b1a6b81b6a3b4cedefea. It stays on the lab PC.

## The product's reads (step 5)
- `stock_summary` as of 20260331, one page:
  - `result.state` `value_total_matched`;
  - `item_count_cross_check` `matched`, with rows 8 and tally_count 8;
  - `tie_out` matched, total -447.50;
  - `totals.partial` true, `empty_closing_value_count` 1 (B3).
- Items:
  - A1, A2, B1, B2, C1 and C2 have `closing.quantity.state` `inside_stock_group` and no amount;
  - B3 has `none_sent`;
  - R1, under Primary beside a group, is **`agreed`** at 10 Nos.
- `masters`:
  - stock_groups: the three groups with their parents as keyed;
  - stock_items: eight items with parents as keyed;
  - units: Kgs with 3 decimals and Nos with 0, both simple.
- Marks were unchanged by every read.

## The raw reads (step 6)
- `n3-a` (before the masters): NUMSTOCKITEMS 0, ALTVCHID absent, ALTMSTID 206.
- `n3-b` and `n3-c`: ALTMSTID 219 and ALTVCHID 2, equal to the marks after BK0. **NUMSTOCKITEMS 8**, so Tally's item count includes items two and three levels down.
- `n1-a`: 8 STOCKITEM rows, every depth, with parents as keyed (`raw/n1-a-table.txt`). B3's CLOSINGBALANCE and CLOSINGVALUE are empty. `n1-b` is byte-identical to `n1-a`.
- `n2-a`: the plain Stock Summary has 2 lines. They are `Lab Group One` (amount -397.50, no quantity) and `Lab Root Bolt` (10 Nos, 5.00, -50.00). No line carries an item inside a group.

## Screen truth (step 7)
- The report opens at 1-Apr-25 to 31-Mar-26, so the period was not changed.
- On F12, "Format of Report" was set from Condensed to Detailed, and "Expand all levels in Detailed format" from No to Yes.
- The screen was transcribed twice, a minute apart (`screen-transcription-T1.txt`, `-T2.txt`), and the two agree line for line.
- Both settings were then set back and the report closed.

## Comparison (step 8, `comparison-step8.txt`)
- For every item with a quantity, n1-a's CLOSINGBALANCE equals both transcriptions in number and unit. The values agree too, taking the wire's negative sign for stock held.
- The group sums (every depth below each group) equal the groups' screen lines: Three (-)27.50, Two 107.50, One 397.50.
- **C1 and B1 read 35 Nos** in n1-a, on the screen and in the tool's values, not 30 or 40, so the collection honours the 31 March 2026 period end.
- Against the finding's conditions in the sheet:
  - `value_total_matched`;
  - all eight items listed with their parents;
  - NUMSTOCKITEMS 8;
  - B3 empty;
  - the nested quantities equal both transcriptions;
  - C1 and B1 at 35 Nos;
  - no `stock_summary_item_count_differs`.

  Every one holds.

## Files
- `answers/`: every tool answer whole.
- `raw/`: every raw answer with its headers and the send log (size, CONTENT-LENGTH, sha256), plus the two parsed tables.
- `n3-company-flags.utf16le.xml`: the filled request, 1,654 bytes, sha256 bf5197c0ddac03788a1de135b1b99315b2fe612f6bc858e7e88307ae6b1b2640.
- The two transcriptions, the comparison and `notes.txt` (UTC times).
- `SCREENSHOTS.txt`: the screen captures by name, bytes and sha256. The captures stay on the lab PC.
