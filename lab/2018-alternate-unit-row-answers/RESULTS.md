# Results: #2018 alternate units, 28 Oct 2026. On TallyPrime 7.2 the alternate-unit field is behind F12 on the Stock Item Creation screen ("Use Alternate Units"), not on Company Features. With it on, two items were created with 1 Box = 12 Nos. Stock Query, Tally's Stock Query screen, the Stock Summary answer and its screen all write their quantities in the base unit only (`30 Nos`, `36 Nos`). The stock items collection writes the alternate unit into the same text: ` 30 Nos =  3 Box`, ` 36 Nos =  3 Box`, where 30 Nos (2.5 Box) shows as `3 Box`. The F12 setting is not held in the company: it outlived the restore and was set back by hand

The sheet is `lab/2018-alternate-unit-row` at 0210fa5fac8859dd2dfa60efba8b12c9172c24f3.
- **Sheet checks:**
  - `SHA256.txt` 13 of 13 equal.
  - `gen_requests.py --check` ok (6 requests).
  - `check-request-ids.py requests`: `census: ok 1, skipped 5, FAIL 0`. The five skipped are Data reports with ID `Stock Summary` or `Stock Query`.
- **Company:** BRIDGE SHAPE LAB, 100021, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346, `verified_tuple`. Current date 20-Apr-2026 at step 0 and after the restore.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...df44, the same before and after). Writes off; the tool only read.
- **Tally:** TallyPrime 7.2 (Latest), Silver Rental, licensed.
- **P0:** met; neither Alt Unit Probe A nor B existed.
- **Marks** (`ledger_masters` compliance, `reused` false every time):
  - M0 / V0 292 / 112, and M0′ / V0′ 292 / 112 after BK1.
  - 292 / 112 after the F11 look and after the F12 setting (M +0).
  - 293 after A and 294 after B (M +1 each); V 112 throughout.
  - 292 / 112 after the restore, and after the F12 setting was set back.
- **BK1:** `TDBK1800_100021.001`, 102,253 B, sha256 86c2f7b2b1650b55396924c860f1c3b1e2c1b7a29c110ce62484c3b850459b23. It stays on the lab PC.
- **Dialogs:** none. The only prompts were the master screens' `Accept ?` and the restore's two expected prompts. No `Quit ?` after any Ctrl+Q.

## Baseline (step 2, read only)
- a-00 1,370 B (862c3206...), a-02 312 B (53fe972f...) and a-03 46 B (the empty envelope) are as expected.
- **Finding, a-01:**
  - It answered **16,904 B, sha256 7a76a524...**, byte-equal to the committed 7.1 fixture `stock_items_shape_lab_fy_live`. The sheet expected 17,480 B (15a361aa...), from the byte-identical request in the earlier row.
  - The earlier row's a-03 was read before that row's BK1 restore. It carried `LANGUAGEID` 0 on six rows: Caustic Soda Flakes, Cleaning Kit A, Hydrochloric Acid 33pc, Soda Ash Light, Sulphuric Acid 98pc, Zero Stock Item.
  - After that restore the collection answer has none, and every leaf value is equal.
  - On the word that followed, today's a-01 is this row's baseline.

## Where the alternate unit is offered (step 2)
- **F11, Company Features:** the Inventory section has no alternate-units option:
  - Maintain Inventory Yes; Integrate Accounts with Inventory Yes; Enable multiple Price Levels No.
  - Enable Batches Yes; Maintain Expiry Date for Batches Yes.
  - Enable Job Order Processing No; Enable Cost Tracking No; Enable Job Costing No.
  - Use Discount column in invoices Yes; Use separate Actual and Billed Quantity columns No.
  - Show more and show all features were already Yes. Left unchanged with one Ctrl+Q.
- **F12 on Stock Item Creation, "Configuration":** under Inventory Details, **`Use Alternate Units` No**. Only it was set to Yes, accepted with Ctrl+A and no prompt.
- **After the setting:**
  - a-00 and a-02 are byte-equal to the first.
  - a-01 is 17,000 B (e3af8e9c...): the same rows and values, plus a `LANGUAGEID` 0 element now on Carton Box Small's row.

## The two items (step 3)
With the setting on, Stock Item Creation shows `Alternate units` after `Units`, then `where [ ] Box = [ ] Nos`, entered as `1 Box = 12 Nos`. The GST block gains `Reporting UOM (UQC)`, which offers Box and Nos; Nos was kept.

| item | opening typed | sub-screen "Allocations of : <item> for:" | quantity offered | master's opening line | M |
|---|---|---|---|---|---|
| Alt Unit Probe A | `30 Nos` | `30 Nos` | `30 Nos` with `( 3 Box )` beneath | 30 Nos (3 Box), 10.00 per Nos, 300.00 | 293 |
| Alt Unit Probe B | `3 Box` (accepted, no message) | `36 Nos` | `36 Nos` with `( 3 Box )` beneath | 36 Nos (3 Box), 10.00 per Nos, 360.00 | 294 |

`per` offered Nos in each case.

## How the answers write them (steps 4 and 5)
| read | A | B |
|---|---|---|
| Stock Query (a-04, a-05), STQGODOWNQTY | `30 Nos` (Main Location / Primary Batch; 310 B, byte-equal to the earlier row's plain 30 Nos item) | `36 Nos` (Main Location / Primary Batch) |
| collection (a-01), CLOSINGBALANCE and OPENINGBALANCE, raw text | ` 30 Nos =  3 Box` | ` 36 Nos =  3 Box` |
| collection, OPENINGVALUE / CLOSINGVALUE | -300.00 | -360.00 |
| Stock Summary (a-00), DSPCLQTY / DSPCLRATE / DSPCLAMTA | `30 Nos` / `10.00` / `-300.00` | `36 Nos` / `10.00` / `-360.00` |

- **The collection's new rows:** the same element names as every existing row (GUID, PARENT, BASEUNITS, CLOSINGBALANCE, OPENINGBALANCE, OPENINGVALUE, CLOSINGVALUE, LANGUAGENAME.LIST).
  - No element names the alternate unit; it appears only inside the quantity text.
  - PARENT is `&#4; Primary`; BASEUNITS `Nos`; the LANGUAGEID in LANGUAGENAME.LIST is 1033.
- **Against Stock Query:** with the leading space trimmed, each CLOSINGBALANCE is the Stock Query quantity followed by ` =  3 Box`, with two spaces before the number. The base part equals the Stock Query lines. The text is not the one-number-one-unit shape every earlier answer had.
- **Rounding:** 30 Nos is 2.5 Box but is written `3 Box`.
- **Existing items:** the eleven existing rows are unchanged from the step 2 resend; a-02 is unchanged.
- **a-00:** it gains the two items as top-level lines.

## Screens (step 6)
- **Stock Query, 1-4-2025 to 31-3-2026:**
  - A: Closing Balance 30 Nos, Closing value 300.00; Godown/Batch Details Main Location / Primary Batch 30 Nos.
  - B: Closing Balance 36 Nos, 360.00; Main Location / Primary Batch 36 Nos.
- **Stock Summary:** Alt Unit Probe A 30 Nos 10.00 300.00, Alt Unit Probe B 36 Nos 10.00 360.00, Grand Total 66 Nos.
- **Against the answers:** every screen equals its answer and shows no Box text. Only the creation screen and the collection show the alternate unit.

## Restore (step 7)
- **Restore:** BK1 restored by Tally's Restore screen. The list showed BRIDGE SHAPE LAB (100021), 29-Oct-26 (2:03 am) only. Prompts: `Accept ?` Yes; "Company data folder (100021) already exists..." Replace.
- **After loading it alone:**
  - marks 292 / 112;
  - `list_companies` the one company, `verified_tuple`;
  - the Select Stock Item list showed the eleven items, without the probes;
  - a-00 again byte-equal to the first (862c3206...);
  - F11 unchanged.
- **F12 after the restore:** `Use Alternate Units` was **still Yes**, so it is not stored in the company's data. It was set back to the recorded No, under the sheet's grant, with marks 292 / 112 after.
- **C10:** company shut; one `tally.exe` owns 9001; `GET /status` 200; client exit 0, error stream empty.

**Confidence:** two items, PARTIAL until repeated.

**Not covered:**
- a field that names the alternate unit: the fixed FETCH of a-01 asks for none, so "no element" here is not proof that none exists;
- a voucher in an alternate unit;
- another conversion, or a unit kept with decimals in Box;
- a batch-kept alternate-unit item;
- more than one alternate unit;
- the tax-audit book.

## Files
- `requests/`: the six requests, as sent.
- `raw/`: every answer as received with headers.
  - Resends carry a suffix: `-s2after` after the setting, `-s4` in step 4, `-s7` after the restore.
- `send-log.txt`.
- `raw-structure.txt`: every answer read by structure; `readans.py`.
- `answers/`: every tool answer whole, 000 to 013.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256. They, BK1 and the data-folder listings stay on the lab PC.
