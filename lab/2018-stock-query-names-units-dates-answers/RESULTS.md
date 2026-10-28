# Results: #2018 Stock Query for item names with special characters, an alternate unit and the stock items collection at other period ends, 28 Oct 2026. On TallyPrime 7.2 seven stock items were created by hand, named with `&`, parentheses, an apostrophe, quotation marks, `%`, `/`, `,` and `-`. Tally stored every name as typed. Stock Query selected each one by its name escaped once as XML text, and for the apostrophe and quotation-mark names the raw and the entity-escaped forms gave byte-equal answers. The collection writes each name escaped once, in its attribute and its NAME element alike. The Stock Item Creation screen offered no alternate-unit field, so no alternate unit was set: the 30 Nos item reads `30 Nos`. At 1 Apr 2025 and 1 May 2025 the collection's closing balance equals the Stock Query lines for all five items. The book was restored from its backup and checked

The sheet is `lab/2018-stock-query-names-units-dates` at 24511dd1ec7892ec427340e1d4536f0bf8dbb36a.
- **Sheet checks:** `SHA256.txt` 32 of 32 equal; `gen_requests.py --check` ok (25 requests); `check-request-ids.py requests`: `census: ok 4, skipped 21, FAIL 0`, each skipped request a Data report with ID `Stock Summary` or `Stock Query`.
- **Company:** BRIDGE SHAPE LAB, 100021, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346, `verified_tuple`. Current date 20-Apr-2026 at step 0 and after the restore.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...df44, the same before and after). Writes off; the tool only read.
- **Tally:** TallyPrime 7.2 (Latest), Silver Rental, licensed.
- **P0:** met. Zero Stock Item (-)50.000 Kgs under Raw Chemicals; Soda Ash Light (-)50.000 Kgs under Factory Floor > Raw Chemicals; none of the seven names among the eleven stock items.
- **Marks** (`ledger_masters` compliance, `reused` false every time):
  - M0 / V0 292 / 112, and M0′ / V0′ 292 / 112 after BK1.
  - Unchanged across Part A.
  - M +1 per created item (293 to 299), V 112 throughout.
  - Unchanged across Part W's reads.
  - 292 / 112 after the restore.
- **BK1:** `TDBK1800_100021.001`, 102,253 B, sha256 05f476f74e41efb87adcf53afc7a8c88823e89f22aac6d326a86cbe58ca1ca6b. It is the only backup and stays on the lab PC.
- **Dialogs:** none at any step. The only prompts were the master screens' own `Accept ?` and the restore's two expected prompts. No `Quit ?` prompt appeared after Ctrl+Q on the blank creation screen.

## Part A: the collection at three period ends against Stock Query (read only)
| item | end | collection CLOSINGBALANCE | Stock Query lines (godown / batch / quantity) | equal |
|---|---|---|---|---|
| Caustic Soda Flakes | 1 Apr 2025 | ` 200.000 Kgs` | Main Location / Primary Batch 200.000 Kgs (a-04, 320 B) | yes |
| Caustic Soda Flakes | 1 May 2025 | ` 400.000 Kgs` | Primary Batch 200.000 + B-2504-A 200.000 (a-05, 1,094 B) | yes |
| Soda Ash Light | 1 Apr 2025 | ` 500.000 Kgs` | Main Location / Primary Batch 500.000 Kgs (a-06, 1,340 B) | yes |
| Soda Ash Light | 1 May 2025 | ` 1300.000 Kgs` | Main Location 1350.000 + Factory Floor -50.000 (a-07, 3,152 B) | yes |
| Cleaning Kit A | 1 Apr 2025 | ` 25 Nos` | Primary Batch 25 Nos (a-08, 310 B) | yes |
| Cleaning Kit A | 1 May 2025 | ` 12 Nos` | Primary Batch 25 + K-2504-A -13 (a-09, 1,086 B) | yes |
| Zero Stock Item | 1 Apr 2025 | `0.000 Kgs` (no leading space) | none: the 46 B empty envelope (a-10) | zero against no lines |
| Zero Stock Item | 1 May 2025 | `-50.000 Kgs` | Main Location / Primary Batch -50.000 Kgs (a-11, 848 B) | yes |
| Hydrochloric Acid 33pc | 1 Apr 2025 | ` 150.000 Kgs` | Primary Batch 150.000 Kgs (a-12, 320 B) | yes |
| Hydrochloric Acid 33pc | 1 May 2025 | ` 816.667 Kgs` | 150.000 + H-2504-A 300.000 + Factory Floor H-2504-B 366.667 (a-13, 1,354 B) | yes |

- **Earlier answers:** a-00, a-12, a-10, a-08 and a-14 are byte-equal to the earlier row's answers to the same requests (862c3206..., 6e8d7e5e..., 8d37111f... 46 B, b78d85c0..., 8d37111f... 46 B).
- **a-03 (31 Mar 2026):** 11 rows. Its closings equal the earlier row's Stock Query sums: 400, 1300, 12, -50, 816.667.
- **The collection across the three ends:** a-02 (1 May 2025) and a-03 are byte-equal (15a361aa..., 17,480 B), since these items do not move after May 2025 in this book. a-01 (1 Apr 2025, 17,466 B) differs only in the closings above. Cleaning Kit B, Descaler Concentrate and Label Roll are empty at all three ends.
- **Zero Stock Item:** its OPENINGBALANCE is empty at all three ends. Its closing at 1 Apr 2025 is `0.000 Kgs`, not empty as the sheet expected.
- **Against the committed fixture:** a-03 is not byte-equal to `stock_items_shape_lab_fy_live` (master 7bde5fa6, captured on 7.1; 16,904 B). Every leaf value of the eleven rows is equal. a-03 carries a `LANGUAGEID` 0 on six rows that the fixture does not have.

## Part W: seven items created by hand, then read
Every created item was made the same way:
- Under Primary, Units Nos, Maintain in batches No.
- GST fields as offered.
- Opening quantity at rate 10.00 in godown Main Location.

The godown allocation sub-screen was titled "Allocations of : <name> for: <qty> Nos", with columns Godown, Quantity, Rate, per and Amount. It closed by itself once the amount was entered.

| item | Stored as typed | Stock Query, form F1 | form F2 (entities) | collection name, as received |
|---|---|---|---|---|
| W5 `Alt Unit Item` | yes | 1 line, `30 Nos` (w-08) | | `Alt Unit Item` |
| W7 `Dash-Item 7` | yes | 1 line, `17 Nos` (w-10) | | `Dash-Item 7` |
| W2 `Gauge (Test) 50L` | yes | 1 line, `12 Nos` (w-03) | | `Gauge (Test) 50L` |
| W3 `Owner's Item` | yes | 1 line, `13 Nos` (w-04) | `Owner&apos;s Item`, byte-equal answer (w-05) | `Owner&apos;s Item` |
| W6 `Mix 5% / Set, Large` | yes | 1 line, `16 Nos` (w-09) | | `Mix 5% / Set, Large` |
| W1 `Kit & Co Item` | yes (sent as `Kit &amp; Co Item`) | 1 line, `11 Nos` (w-02) | | `Kit &amp; Co Item` |
| W4 `Quote "Q" Item` | yes | 1 line, `14 Nos` (w-06) | `Quote &quot;Q&quot; Item`, byte-equal answer (w-07) | `Quote &quot;Q&quot; Item` |

- **Stock Query answers:** each is 310 B with one line: Main Location / Primary Batch / the quantity. None has a purchase or sale group.
- **The collection (w-01, 27,118 B, 18 rows):**
  - Each name is written escaped once, the same in the `STOCKITEM NAME` attribute and in `LANGUAGENAME.LIST/NAME.LIST/NAME`. The quotation marks are escaped in the attribute.
  - Each CLOSINGBALANCE is the Stock Query quantity with the leading space, BASEUNITS `Nos`.
  - The eleven existing rows are equal to a-03's.
  - The seven new rows' PARENT is `&#4; Primary`, a reference C4 drops before parsing. With it dropped, w-01 parses.
- **Alternate unit:**
  - The screen offered no Alternate units field: after Units it went straight to Maintain in batches. F12 was not used, so no Box conversion was set.
  - W5's quantity text is `30 Nos` in Stock Query, the collection and Tally's Stock Query screen.
  - W5's collection row has the same element names as W1's (GUID, PARENT, BASEUNITS, CLOSINGBALANCE, OPENINGBALANCE, OPENINGVALUE, CLOSINGVALUE, LANGUAGENAME.LIST), so the collection gives no alternate-unit signal.
  - What an alternate unit does to the quantity text is **not established** by this row.
- **Step 7 (view only):** Tally's Stock Query screen for Alt Unit Item, period 1-4-2025 to 31-3-2026, shows Closing Balance 30 Nos and Godown/Batch Details Main Location / Primary Batch 30 Nos. This equals the answer.
- **Typing:** on the first attempt the name field showed `AltUnitItem`, because the screen-control typing dropped the spaces. It was retyped over before anything was saved, and every name was read on screen before Enter.

## Restore (step 8)
- **Restore:** BK1 restored by Tally's Restore screen. The list showed BRIDGE SHAPE LAB (100021), 29-Oct-26 (0:31 am) only. Prompts: `Accept ?` Yes; "Company data folder (100021) already exists..." Replace.
- **After loading it alone:**
  - marks 292 / 112;
  - `list_companies` the one company, `verified_tuple`;
  - the Select Stock Item list showed the eleven original items and none of the seven;
  - a-00 sent again was byte-equal to the first (862c3206...);
  - current date 20-Apr-2026.
- **C10:** company shut; one `tally.exe` owns 9001; `GET /status` 200.

**Confidence:** one observation per name, PARTIAL until repeated.

**Not covered:**
- an alternate unit actually set (the creation screen offered no field without F12);
- other punctuation, names with leading or trailing spaces, other lengths;
- a batch-kept item with such a name;
- an alternate-unit voucher;
- a book that is not batch-wise;
- the tax-audit book.

## Files
- `requests/`: the 25 requests as sent.
- `raw/`: every answer as received with its headers, a-00-stock-summary-s8 being step 8's resend.
- `send-log.txt`: each send's exit, status, bytes, CONTENT-LENGTH and sha256.
- `answers/`: every tool answer whole, 000 to 016.
- `client-log.txt`.
- `stockquery-structure.txt` and `compare-output.txt`: step 6's readings by structure.
- `readans.py`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the 64 captures by sha256; they, BK1 and the data-folder listings stay on the lab PC.
