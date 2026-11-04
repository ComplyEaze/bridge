# 1379 Part R: what the synthetic inventory book holds for an item Sales invoice: results

**First line: PASS (Part R ran).** The gate held, every call answered (r1 to r8, each on one page, none refused), and the marks did not move: m-b is byte-identical to m-a, M 292, V 4. **The verdict for v2: W1 and W2 cannot be built on this book as it stands.** It has 3 godowns, not one. Batch-wise is on for the company. No stock item returns a GST rate or an HSN, and the sales ledger's GST details hold no rate or taxability. The customer's bill-wise setting is not returned by these tools. The fields v2 would have to key, or the conditions the next sheet must decide, are listed below. (verified)

Sheet `lab/1379-sheet` at 93aa5f81 (SHA256.txt 5 of 5 equal, by sha256 and bytes). LAB-COMMON v3. THE COMPANY: BRIDGE STOCK LAB, GUID 1edb9b05-8d35-4c0c-9959-dce655730463. TallyPrime 7.2 Silver. Each step and stamp is in `notes.txt` (UTC). Every name from the book is in `evidence/r-table.txt` only, read from the JSON by key.

## The build, step 0 and the gate

- **THE BUILD.** No build on the lab PC was master at or after 5680ba48, so the row stopped before the first call and reported. On the recorded word, it used master **87ee283452264c1c8e9921c0c07be1ec2b27875c**, whose ancestor is 5680ba48: the Windows bundle artifact of its Bridge CI run, which concluded success.
  - The zip is 59,336,009 B, sha256 91162124d6eb789be191fa3183797c8c3f02143f0f8043059776503886a40ff7, equal to the artifact's digest. It was checked before unpacking.
  - `bridge_mcp.exe` sha256 ace7cdb0b79a6bc6d91229d7530c29badc2b6ea8c9a93acf7ea8e2e08ad05928, at the start and the end. This equals the CI smoke file's `binary_sha256`. bridge-tally 0.7.0, 32 tools. (verified)
- **Request-ID check:** `census: ok 1, skipped 0, FAIL 0` (= EXPECT). (verified)
- **Step 0:** the Tally window first: Select Company, no company loaded, no dialog. One tally.exe owns 9001; GET /status 200.
  - THE COMPANY was loaded alone. Current Date Monday, 21-Apr-2025; period 1-Apr-25 to 31-Mar-26; last entry 21-Apr-25.
  - The Gateway marks the company "(e)" and reads "(e) Data exceptions exist" at its foot. This is a note on the Gateway, not a dialog; it is recorded only. (verified)
- **m-a and m-b** (C3: 200, 4,894 B = CONTENT-LENGTH, under the 8,192 bound; C4: one COMPANY row, THE COMPANY's GUID). The two answers are byte-identical (6b68c769…), and also equal to step 0's marks read. (verified)

| field | m-a | m-b |
|---|---|---|
| ALTMSTID | 292 | 292 |
| ALTVCHID | 4 | 4 |
| ISINVENTORYON | Yes | Yes |
| ISBATCHWISEON | Yes | Yes |
| ISINTEGRATED | Yes | Yes |
| NUMGODOWNS | 3 | 3 |
| NUMSTOCKITEMS | 55 | 55 |
| NUMUNITS | 2 | 2 |
| NUMVOUCHERTYPES | 33 | 33 |
| BOOKSFROM | 20250401 | 20250401 |
| LASTVOUCHERDATE | 20250421 | 20250421 |

The values of 29 Oct (ALTMSTID 292, ALTVCHID 4, NUMSTOCKITEMS 55) are unchanged. (verified)

## The calls (shapes and counts)

| call | arguments (and THE COMPANY's GUID) | answer | rows |
|---|---|---|---|
| r1 | `list_companies` `{}` | answered; exactly THE COMPANY, its GUID, `verified_tuple` (= EXPECT) | 1 |
| r2 | `masters` godowns | answered, state observed | 3 of 3 |
| r3 | `masters` units | answered | 2 of 2 |
| r4 | `masters` stock_items | answered | 55 of 55 |
| r5 | `masters` voucher_types | answered | 24 of 24 |
| r6 | `ledger_masters` Sales Accounts, ancestry, compliance, GST details | answered; no subgroup ledger excluded | 1 of 1 |
| r7 | `ledger_masters` Sundry Debtors, ancestry, compliance, GST details | answered; no subgroup ledger excluded | 1 of 1 |
| r8 | `ledger_masters` Duties & Taxes, ancestry, compliance, GST details | answered; no subgroup ledger excluded | 2 of 2 |

No answer said more rows follow, so no call was paged. No call returned an error or a refusal. (verified)

## Record by parser (names in `evidence/r-table.txt`)

- **r2:** 3 godowns, each under Primary. Tally's default, Main Location, is one of them. **Not exactly one.** (verified)
- **r3:** 2 units, both simple. One has 3 decimal places and the other 0. (verified)
- **r4:** 55 stock items. Each carries name, parent, GUID, ids, `gst` and `hsn`, and `inherited_from_group_or_company` `not_read`. **No base unit is returned.** Every item's `gst` and `hsn` read `observation` `entries` with an empty list.
  - Counts: **with a GST rate 0, with an HSN 0, with neither 55.**
  - Under 3 parents: 51 items, 2 and 2. (verified)
- **r5:** 24 voucher types. **One** is of the Sales class, active, `numbering_method` `default`, with `series_numbering` automatic, `auto_retain`, duplicates not prevented. (verified)
- **r6:** 1 sales ledger: tax_type Others, not a tax ledger. Its GST details hold one entry from 20250401, source "As per Company/Group", ineligible_itc true.
  - `taxability` null; one state entry ("Any"); every rate (CGST, SGST/UTGST, IGST, Cess, State Cess) `absent`; no slab rates.
  - Whether it affects stock is **not returned**. (verified)
- **r7:** 1 customer ledger: registration type null, no GST registration entries, GSTIN status `not_reported`, state null. Its bill-wise setting is **not returned**. (verified)
- **r8:** 2 tax ledgers, both tax_type GST, neither with GST detail entries of its own.
  - One `gst_duty_head` recognized `cgst` (raw CGST); the other recognized `sgst_utgst` (raw SGST/UTGST).
  - **No ledger reads State Tax.** (verified)

## The verdict for v2

W1 and W2 cannot be built on this book as it is.

| condition | on this book |
|---|---|
| one godown | **no**: 3 godowns |
| no batch-wise items | **no, at the company**: ISBATCHWISEON Yes. Per item: not returned by r4 |
| a Sales-class voucher type | yes: 1 |
| a customer ledger not bill-wise | **not established**: bill-wise not returned |
| CGST and State Tax ledgers | **CGST yes; State Tax no**: the second tax ledger's head is SGST/UTGST |
| one stock item with GST details set | **no**: 0 of 55 with a rate or an HSN |
| one sales ledger with GST details set | **no**: taxability null, every rate absent |
| for W2, items differing from the ledger in exactly one attribute | **no**: nothing to differ from |

**What v2 would have to key, or decide, as fields** (each a master change after C5's backup, or a choice of another book):
- **Godowns:** W1's "one godown" is not met. Either choose a book with one, or name the godown the line uses (`GODOWNNAME`) and drop the condition. Deleting godowns is not a field change.
- **Batches:** company F11 "Maintain batch-wise details" (now Yes), or each chosen item's "Maintain in batches" (not returned here; read on the item's screen).
- **Stock item, each one used (W1; W2's control and W2a to W2c):**
  - GST applicability;
  - HSN/SAC details and HSN/SAC;
  - GST rate details: taxability and rate;
  - type of supply;
  - base unit (not returned by `masters`; read on the item's screen).
- **Sales ledger:** GST details: taxability, HSN/SAC, the rate (CGST, SGST/UTGST, IGST), type of supply; and "Inventory values are affected" (not returned).
- **Customer ledger:**
  - "Maintain balances bill-by-bill" (not returned; read on the screen, and set to No if Yes);
  - State;
  - Registration type, and a GSTIN if the row needs one.
- **Tax ledgers:** the second ledger's Tax type (SGST/UTGST now) only if v2 needs a State Tax head; otherwise v2 names SGST/UTGST.

The company's own GST settings were not read by Part R, so v2 also reads them. (verified)

## Kept on the lab PC

No captured byte is altered. A captured file is pushed whole unless it holds the company number or a run of 10 or more digits. Two places are exempt: inside a whole 64-hex or 40-hex hash value, and the whole value of a VOUCHERKEY element. Each kept file is listed in `kept-local.txt` by size and sha256, with its reason. Screens stay here too, listed in `SCREENSHOTS.txt`. (verified)
- **The eight tool answers** r1 to r8 (001 to 008) carry the company number in their company fields, and so do **the three marks answers** (s0-marks, m-a, m-b; one sha256, 6b68c769…). All of them stay here whole. Their values are given above, and pushed in `evidence/r-table.txt` (the parser's table), `evidence/*-c4.txt` (C4 of each marks answer) and `evidence/tools-log.txt`. The tools list (000), each marks answer's headers, the send log and the server's own logs are pushed. (verified)

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.
