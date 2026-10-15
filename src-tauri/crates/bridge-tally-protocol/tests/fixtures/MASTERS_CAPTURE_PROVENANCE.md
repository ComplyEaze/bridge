# `masters_*` — provenance

Voucher types, godowns, units, stock groups, stock items, cost centres and cost categories, each read as one native collection with the company GUID computed onto every row. These are the fixtures for the `masters` read (#724 step 1 for voucher types). Every file is a live capture, never hand-written.

## Provenance

- **Host / gateway:** TallyPrime **Silver (licensed)**, 7.1, `education_mode=false`, `http://127.0.0.1:9001` (a lab instance).
- **Date:** 2026-09-30, between 19:14:34 and 19:14:37 IST, one request at a time. `tally_status` was healthy before the first request and after every request. The company was confirmed loaded before anything was sent.
- **Book:** `BRIDGE SHAPE LAB`, a synthetic lab company. Its book extent (`ALTMSTID` 289, `ALTVCHID` 111) was read before and after the four requests and was equal.
- **Sender:** a lab capture script, not `bridge_mcp`. It sent the exact request bytes tabled below, one request at a time. Those bytes are the production renderers' own output: `every_request_is_byte_equal_to_its_committed_fixture` (`src/native_masters_tests.rs`) asserts that each renderer's output equals its committed request, so the responses answer the request Bridge sends.
- **Encoding:** responses are **BOM-less UTF-16LE**, exactly as received. Requests are the exact bytes sent: **UTF-16LE with a BOM**. `.gitattributes` marks this tree `-text`.
- **Request shape:**
  - Voucher types: the production `render_native_voucher_type_export_request` text with `ISACTIVE, ISOPTIONAL, NUMBERINGMETHOD` added to its `FETCH`, and the `BRIDGECOMPANYGUID` compute that the group and ledger snapshots already send.
  - Godowns, units and stock groups: a collection of that type (`ISMODIFY="No"`) with `NAME, PARENT, GUID, MASTERID, ALTERID` (godowns and stock groups) or `NAME, GUID, MASTERID, ALTERID, ORIGINALNAME, DECIMALPLACES, ISSIMPLEUNIT` (units), and the same compute.
  - No filter, no dates, no `$$` function other than `$$SysName:XML`.
- **What the captures show:**
  - Every row of every kind carries `BRIDGECOMPANYGUID` equal to the company's GUID.
  - A collection with rows is present as one `COLLECTION` element.
  - `NUMBERINGMETHOD` is a direct child of every voucher-type row. Its values here are `Default` (24), `Automatic` (1) and `Manual` (1).

**Zero-row and single-row answers (2026-09-30, 20:54 IST, the same host).**
- **Book:** `BRIDGE READS LAB`, a synthetic company with no inventory masters. Its book extent (`ALTMSTID` 242, `ALTVCHID` 3) was read before and after, and was equal. `tally_status` was healthy before and after.
- **Requests:** the same request text as the rows below, with only the company name changed. They were not stored again.
- **Units and stock groups:** each answered `STATUS` 1 with one **present but empty** `COLLECTION` element. That is not an absent collection and not an empty envelope.
- **Godowns:** returned one row, the company's default location.

| file | bytes | sha256 | kind | rows |
|---|---|---|---|---|
| `masters_voucher_types_request.utf16le.xml` | 1188 | `4359b5366ea77cb57bbbd72c6c1fee3b2facef7cb6dc1c079492a2988fc628db` | request, voucher types | n/a |
| `masters_voucher_types_shape_lab_live.utf16le.xml` | 48456 | `91c7e56cc3810b8b266ac21b17ca2f08cfce3d0174db16149bcf55a4c9d4e4d1` | voucher types | 26 |
| `masters_godowns_request.utf16le.xml` | 1102 | `a2a63164752bd2c438de7a27ca4cb854cb0bf5b1a60db3723986370140f051f3` | request, godowns | n/a |
| `masters_godowns_shape_lab_live.utf16le.xml` | 5190 | `2ecfd80804b3691776888982187f03401b14ddbf0e48a6a4823dd45b93a5ab9a` | godowns | 2 |
| `masters_units_request.utf16le.xml` | 1160 | `c03da62e706f063bcd88fb2a9d9d5df11ec3347a305fc6de5bf5395d283de7ed` | request, units | n/a |
| `masters_units_shape_lab_live.utf16le.xml` | 6732 | `50c606821031ddfe15740b5a318596cbeb164fff77bad710131dee3c6da52d6e` | units | 4 |
| `masters_stock_groups_request.utf16le.xml` | 1130 | `8f8737ee8f899a2c052d31c57522a9db41a4eeda9162b17ca51b50bf7d602838` | request, stock groups | n/a |
| `masters_stock_groups_shape_lab_live.utf16le.xml` | 6166 | `032564c60fcbd54db5231b87ed20c9fb5b28b116ce146133b15b9841113f4dc8` | stock groups | 3 |
| `masters_units_reads_lab_live.utf16le.xml` | 3000 | `b5fbda77c4df9513a742a884734cbc565cbce1393b7785f4d9d4de597f1d3a15` | units | 0 |
| `masters_stock_groups_reads_lab_live.utf16le.xml` | 2998 | `31f0974839de95aba8015df487b3e37e5ccb125deae1efe2999720024ff729ae` | stock groups | 0 |
| `masters_godowns_reads_lab_live.utf16le.xml` | 4146 | `0d105ecc3a3ade0b2a83ce46f5e0bcdeb0b67c3882b081f3534e5ed15d2e951b` | godowns | 1 |

**Cost centres and cost categories (2026-10-07, about 14:18 IST, licensed TallyPrime 7.1 Silver, `education_mode=false`, `http://127.0.0.1:9001`).** The owner read both books' Cost Centres setting on screen (F11): **No** on both. Five read-only requests, one at a time through a recording relay: on `BRIDGE SHAPE LAB` the Company flags, the cost centres and the cost categories, and on `BRIDGE CORPUS FOREX` the Company flags and the cost centres. Each answered HTTP 200 with an envelope and `STATUS` 1 (three of the five answers are stored here; the two Company-flag answers are not). The request files are the byte-exact request text the capture kit generated (the production renderer's own output, asserted equal by a test), UTF-16LE with a BOM; the responses are BOM-less UTF-16LE as received.
- **Books:** `BRIDGE SHAPE LAB` (flag No, two cost centres and two cost categories) and `BRIDGE CORPUS FOREX` (flag No, no cost centre defined; its request differs from the SHAPE LAB one only by the company name and is not stored again).
- **What the captures show:** a book whose setting reads No still returns its cost centres (`Assembly`, `Trading`: parent the reserved root, category `Business Line`) and its cost categories (`Business Line`: revenue Yes, non-revenue No, affects stock No; `Primary Cost Category`: Yes, Yes, No); a book with none defined answers one present, empty `COLLECTION` (`MSTDEPTYPE` 32). Every row carries `BRIDGECOMPANYGUID`.
- **Not shown by these files:** a larger book, another release (a book whose setting reads Yes is in the next section).

| file | bytes | sha256 | kind | rows |
|---|---|---|---|---|
| `masters_cost_centres_request.utf16le.xml` | 1150 | `7e4aec0f8f19f0dee68083422e5be5f029a58a77eb900e30452a39c5b4db749f` | request, cost centres | n/a |
| `masters_cost_centres_shape_lab_flag_no_live.utf16le.xml` | 5296 | `f2966af3109cc74e60fdd13e3680d8ed6d75565d8a81d8c260478a8584fe12c9` | cost centres | 2 |
| `masters_cost_categories_request.utf16le.xml` | 1232 | `c79688503edd6b897159774e16e0573c2fd869112dd6b702972d7859b6c4a582` | request, cost categories | n/a |
| `masters_cost_categories_shape_lab_live.utf16le.xml` | 5782 | `e61714960d2c365c93805366c2021703eb885891be684b8c879d1721056c9b7b` | cost categories | 2 |
| `masters_cost_centres_corpus_forex_empty_live.utf16le.xml` | 2996 | `9ec9b8fbbfc810101a9cfe8a4a8b1a82df383be0c678400cf3e5ab82d954909a` | cost centres | 0 |

### Cost centres and categories on a book with the setting at Yes (captured 2026-10-07 in a separate sitting)

A third synthetic book, the parity lab, whose Cost Centres setting read Yes (the Company flag `ISCOSTCENTRESON`, equal to the owner's screen), captured through the same collection requests as the files above (these two answers carry no release or licence; the status read of that sitting, taken through the same relay just before them and not committed, recorded product TallyPrime, release 7.1, licence Silver, Education mode off). The fixtures are **scrubbed copies**: the company GUID prefix is replaced with a synthetic one and the three lab centre names with `Parity CC A`, `Parity CC A1` and `Parity CC B`; the structure and every other byte of the answers are as received (BOM-less UTF-16LE, `&#4;` kept in `PARENT`). The originals are 6,806 and 4,458 bytes.

| file | bytes | sha256 | content |
| --- | --- | --- | --- |
| `masters_cost_centres_parity_flag_yes_live.utf16le.xml` | 6848 | `2a8fe692c63da913919ec5e8440a2da5571b2f7d5a371cb13d1288b6a0ca9186` | CostCentre, 3 rows: two at the top level, one under another centre, default category |
| `masters_cost_categories_parity_flag_yes_live.utf16le.xml` | 4458 | `a07fd9d365134a5023e17b5aa5e7f3c762a27d9dd807ae12ff433edaf2863376` | CostCategory, 1 row, the predefined Primary Cost Category (Yes, Yes, No) |

### Stock items with their dated HSN and GST lists (captured 2026-10-14, 21:14 to 21:29 UTC)

Licensed TallyPrime **Silver**, 7.1, on Windows (`http://127.0.0.1:9001`, a lab instance), the synthetic book `BRIDGE SHAPE LAB`, loaded alone, one request at a time. The sender was a lab capture script, not `bridge_mcp`: it sent the exact request bytes tabled below, as read-only Export requests. The request is the production renderer's own output, asserted equal by `every_request_is_byte_equal_to_its_committed_fixture`. The responses are **BOM-less UTF-16LE**, exactly as received; the request is UTF-16LE with a BOM. The `COLLECTION` of every answer carries `ISMSTDEPTYPE="Yes"` and `MSTDEPTYPE="512"`.

The three answers are the same request sent at three points of one sitting: before anything was created (the baseline), after one stock item was created in Tally's own screen (Under `Raw Chemicals`, unit Kgs), and after that item had two dated HSN entries and two dated GST entries keyed. The sitting ended by restoring the book from a backup taken before the item was created; after the restore the baseline answer came back byte for byte. Counts below were taken by parsing the XML, not by searching the text.

- **Baseline:** 11 items; every item carries an empty placeholder (a whitespace-only element) of both lists, except `Soda Ash Light`, which carries one GST entry (`Taxable`, IGST 18 with CGST and SGST/UTGST 9, source `Specify Details Here`) and an empty HSN placeholder. The largest row is 1,936 UTF-16 units.
- **Item created:** 12 items. On its first save Tally added an HSN entry and a GST entry of its own to the new item, both dated 1 April 2025, source `As per Company/Stock Group`, with no code and no rate (the GST entry has no `TAXABILITY`, carries `GSTINELIGIBLEITC` Yes, and no `GSTRATE` on any of its five heads). The row is 2,238 units.
- **Two dates:** 12 items. The item carries two HSN entries (`28362010`, description `Disodium Carbonate, Dense`, from 20250401; `28362090`, `Other`, from 20250922) and two GST entries (Taxable, IGST 18 from 20250401 with `GSTINELIGIBLEITC` Yes; Taxable, IGST 5 from 20250922 with no such element). Rates arrive as text with a leading space (` 9`, ` 18`, ` 2.50`), the state and the Cess valuation type carry the reserved character `&#4;`, and the State Cess head has no `GSTRATE`. Inside a row the lists come as every `GSTDETAILS.LIST`, then every `HSNDETAILS.LIST`, then `LANGUAGENAME.LIST`. The row is 4,373 UTF-16 units, line ends kept; it is the largest captured.
- **Input-tax-credit element:** the screen's `Eligible for Input Tax Credit` read No on the first GST row and Yes on the second; those readings are not in any fixture, only in `lab/hsn-masters-answers/NOTES.md` ("Other observations") and `lab/hsn-masters-answers/check-lines.txt` (steps 9 and 10), commit `46dc0a98246ac399e294cadb7c4b43fe64e1506d` of this repository.
- **Not shown by these files:** a GST state row other than `Any`, slab rates, a code keyed on a stock group reaching an item that defers to it (`lab/seeding-answers/answers/ans-s06b.xml` and `ans-s05c.xml`, commit `8a0c9e55ce249c93c6b35add00e5aa220c7676c6` of this repository, not stored here, showed the group's four other items still carried empty lists; no item deferring to a coded group was observed), another release, a larger book, a zero-row stock-item answer.

| file | bytes | sha256 | kind | rows |
|---|---|---|---|---|
| `masters_stock_items_request.utf16le.xml` | 1192 | `c90cfcc238cf308b58fc160e6ecd78affa691729a3dbc2ff963f4d777e82d075` | request, stock items | n/a |
| `masters_stock_items_shape_lab_baseline_live.utf16le.xml` | 19450 | `7038532ed86f7b924fe537d42b241eab93576d471787627e42f96837833c8a92` | stock items, baseline | 11 |
| `masters_stock_items_shape_lab_item_created_live.utf16le.xml` | 23938 | `bdfffd330d47fcc2acc97eb40ded1ff262f86bd385c831a5b7b1b107bbcedd74` | stock items, item as first saved | 12 |
| `masters_stock_items_shape_lab_two_dates_live.utf16le.xml` | 28208 | `6a4fd0b871763bad43db92cc98cbf0e0c48fcda441b84fbe20664d753df5d653` | stock items, two dated entries per list | 12 |
