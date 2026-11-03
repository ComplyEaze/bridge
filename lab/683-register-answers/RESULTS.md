# Results: row 683-R1, the registers and gst_by_counter_ledger on the several-currency lab book behind a keeping relay, 4 Nov 2026

**All three register reads refused with `bill_allocation_amount_invalid`, not the sheet's expected `voucher_amount_invalid`.** The sheet counts that as a result, not a stop. Every stop rule held: no `import_refused`, no dialog, the gateway answered throughout, and the marks after the row are byte-identical to step 0's. The class-entry window answer's `AMOUNT` and `VATEXPAMOUNT` texts equal both this row's plain-entry answer and #2276's committed fixture, 53 of 53, so by the sheet's Decides **the class-entry answer matches #2276's layout**. The refusal comes from the composite amounts inside `BILLALLOCATIONS.LIST` (8 of them), which the window parse reaches before any register rule. [verified]

Sheet: `lab/683-register-sheet` at 0adb5b76, folder `lab/683-register/`. `SHA256.txt` hashes to 710c39623eef…, and all 5 files it lists are equal. [verified]

Build: merge-queue run 41690252208, `merge_group`, head 75224fe3586875dffe8500824668d20e94fde45e, success. Artifact `unsigned-bundle-smoke-windows-latest`, id 13257808512, zip sha256 73f4612a5c7e656c66f59cefbef81fe36931c5be1093ab9757cb5842e1549ab3, equal to the API digest. `bridge-tally.mcpb` 53b1b52e…a06f, `bridge_mcp.exe` f46fd5bb2830fb3807ae4fedea3c94e9845259f9919d1ed5d8851fc237bb323a and `pdfium.dll` 79d4676b…c118, each equal to the bundle's smoke file. All four were equal before and after the row (`evidence/exe-before.txt`, `evidence/exe-after.txt`). Server 0.7.0. [verified]

Client: `scripts/client.py` of `lab/2074-2214-export-cut-answers` at cc28c8f9, sha256 85752d4a4467baf158d726d0e3685193114d7e1fee610e33376a63950ca77498 (`run/client.py`). Launch: `BRIDGE_TERMS_ACCEPTED=true`, host 127.0.0.1, port 9002 (the relay), one new empty data folder, and no write, batch-post or redaction setting (`run/launch-env.json`; the client logs `BRIDGE_AGENT_ENABLE_WRITES present: False`). One server process for every call, exit code 0. [verified]

## Steps
| step | result |
|---|---|
| 0: book, marks, dates | BRIDGE FOREXGROUP LAB loaded alone with no prompt. `GET /status` on 9001: 200. Marks read raw on 9001: one COMPANY row, GUID 1bb48be5-baba-45b2-90d8-7593244915a9, **V 11, M 214**. Tally's current date: Tuesday, 1-Apr-2025, period 1-Apr-25 to 31-Mar-26. PC's date: 4-Nov-2026. |
| 1: check-relay | 13 PASS, exit 0 (`evidence/check-relay.out`). |
| 2: relay, client | `relay.py 9002 9001` into a new empty folder; client started; `tools/list` answered 32 tools. |
| 3: `list_companies` | One company, this GUID, `identity_state` **`verified_tuple`**. 0.126 s. |
| 4: `vouchers` (control) | **11 vouchers**, state `complete`, profile `agent_vouchers_v1_filters`, snapshot V 11 / M 214. **8 vouchers** have an entry with `foreign` (`amount`, `currency`, `rate`) apart from the rupee `amount`. Window request sha256 986017edb475d126…5cef: **not** the fixture's a105626a…c4f1a, so the answer was not compared with 92e0e054…9369 there (see Decides for the class-entry comparison). 3.152 s. |
| 5: `sales_register` | `isError` true, `code` **`bill_allocation_amount_invalid`**, message "ComplyEaze Bridge refused this operation.", no `cause` field; evidence state `partial`. 3.411 s. |
| 6: `purchase_register` | The same refusal, the same request set, byte-identical answers. 3.363 s. |
| 7: `gst_by_counter_ledger` | The same refusal, the same request set, byte-identical answers. 4.354 s. |
| 8: stop; marks | Client stopped (server exit 0), relay stopped. Marks raw on 9001: **V 11, M 214**, answer byte-identical to step 0's. |

[verified]

## What each call sent (`evidence/per-call.txt`, `evidence/exchanges.txt`)
240 exchanges on one relay connection: 136 POST and 104 `GET /status`. Every repeated request had a byte-identical answer, no answer took over 0.156 s, and the client never left early. No Import Data request: `import_refused` 0. [verified]

- **`list_companies`:** 4 exchanges: `BridgeCompanyExtent` ×2 and `GET /status` ×2.
- **`vouchers`:** 44 exchanges.
  - Company and currency reads: `BridgeCompanyExtent` ×10, `BridgeCompanyBookExtentV2` ×4, `BridgeCompanyCurrencies` (two requests) ×2 each, `BridgeCompanyBaseCurrency` ×2.
  - The window: `Bridge Agent Voucher Census` ×2, `Bridge Agent Vouchers` (plain entry window, exchanges 22 and 24) ×2, `Bridge Agent Company High Water` (marks) ×2.
  - `GET /status` ×18.
- **Each register tool:** 64 exchanges with the same requests in all three calls (sales 49–112, purchase 113–176, gst 177–240).
  - Company and currency reads: `BridgeCompanyExtent` ×12, `BridgeCompanyBookExtentV2` ×8, the currency reads ×2 each.
  - The master listing: `List of Ledgers` (master, request b20e80b7…) ×2, `List of Ledgers` (balance, request 71e429b8…) ×2, `List of Groups` ×2.
  - The window: `Bridge Agent Voucher Census` ×2, `Bridge Agent Vouchers` (class-entry window, request ab78fbe5…, exchanges 108/110, 172/174, 236/238) ×2.
  - `GET /status` ×28.
  - **No marks read** (`Bridge Agent Company High Water`) was sent in any register call. The refusal came after the window and before the marks the sheet lists (`agent_register.rs:762-763`).

## Record
**Window answers**, counted with an XML parser (`exch.py`). [verified]

| answer | `VOUCHER` | `AMOUNT` with " @ " (of all `AMOUNT`) | `VATEXPAMOUNT` with " @ " |
|---|---:|---:|---:|
| `vouchers` plain entry window (ex 22, = ex 24) | 11 | 28 (of 36) | 12 |
| register class-entry window (ex 108, = 110, 172, 174, 236, 238) | 11 | 28 (of 36) | 12 |

- **Per voucher**, in both windows, in order: Sales 1, Sales 2, Receipt 1, Journal 1, Sales 3, Receipt 2, Purchase 1, Purchase 2, Payment 1, Payment 2, Receipt 3. Each has 2 `ALLLEDGERENTRIES.LIST` children: 3 Sales, 3 Receipt, 2 Purchase, 2 Payment, 1 Journal, as the sheet says.
- **Where the composites sit** (`compare.py`, `evidence/compare-22-108.txt`):
  - 16 entry `AMOUNT`s;
  - 8 `BILLALLOCATIONS.LIST/AMOUNT`s;
  - 4 `BANKALLOCATIONS.LIST/AMOUNT`s;
  - 12 `VATEXPAMOUNT`s.
- **Composite text shape** (each digit shown as 9): `$999.99 @ ₹ 99/$ = ₹ 9999.99` and its negative `-$… = -₹ …`. There is one form with a four-place rate (`₹ 99.9999/$`). Throughout: symbol, one space, the figure, and one space before ` = `.

**Class-entry answer:** it holds every compute its request asks for, the 10 of `voucher_type_class_computes` (`agent_voucher_type_class.rs:114-127`): `BRIDGEVCHTYPEGUID`, `BRIDGEVCHRESERVEDNAME`, and `BRIDGEVCHIS` + SALES, PURCHASE, PAYMENT, RECEIPT, CONTRA, JOURNAL, DEBITNOTE, CREDITNOTE. Each one is on all 11 vouchers. The plain entry window carries none of them. [verified]

**Master listing (each register call):**
- Master answer (ex 82): **8** ledger rows.
- Balance answer (ex 86): **8** ledger rows, every `CURRENCYNAME` `₹`, so **0** rows in a currency other than the base. **6** rows hold " @ " in `OPENINGBALANCE` or `CLOSINGBALANCE`.
- The balance pair was sent on this two-Currency book, as the sheet's code note says (`tally/connection.rs:811-813`).

[verified]

**Tool answers:** every answer is kept whole in `run/answers/` (company number redacted). The three refusals are `{"error": {"code": "bill_allocation_amount_invalid", "message": "ComplyEaze Bridge refused this operation."}}`; there is no `cause`. [verified]

## Why `bill_allocation_amount_invalid`
At 75224fe3, `agent_voucher_parse.rs:917-931` reads a bill allocation that has a `BILLTYPE`: an `AMOUNT` that is not a plain decimal is either withheld (`composites.withholds`) or refused `bill_allocation_amount_invalid`. Under the class-entry window's `CompositePolicy::Refuse` nothing is withheld. A `BILLALLOCATIONS.LIST` inside an entry would close, and be checked, before that entry's own `AMOUNT` is judged. So the first composite bill allocation refuses the window before the entry check that gives `voucher_amount_invalid`. This order is read from the code, not traced in a run. [hypothesis]

## Decides
- **The class-entry answer's entries match #2276's layout.** `compare2276.py` (`evidence/compare-2276.txt`) sets exchange 108 against `vouchers-forex-unspaced-20250401.utf16le.xml` at 75224fe3 (sha256 92e0e054…9369). Result: the same 11 GUIDs, the same 53 `AMOUNT`/`VATEXPAMOUNT` paths, and **53 texts equal, 0 different**. So the register can reuse `read_composite_amount` as captured, and these answers can serve as the register's fixtures. Neither window's request is the fixture's request (1750 B and 3126 B against 860 B). [verified]
- **The refusal is at the bill allocations**, not the entry amounts. Whatever the register does with entry composites, its window parse also has to deal with the 8 composite `BILLALLOCATIONS.LIST/AMOUNT`s, or the read refuses first. [verified for this book; the cause above is a hypothesis]
- **The master listing:** 8 ledger rows in the master answer and 8 in the balance answer, 0 in a non-base currency, 6 with a composite opening or closing. These are the counts the register's classification (`tally/connection.rs:3518`) would work on. Which ledgers it sets aside was not reached: the call refused first. [verified]
- **No dialog anywhere.** [verified]

## Notes
- **Before step 0**, a desktop-control call meant to bring Tally forward started a second tally.exe. That copy never listened on 9001 and loaded no company. I stopped it by PID within a minute, before any request was sent, and there was then one Tally on 9001 again. The company-name text typed meanwhile went to that copy (`evidence/notes.txt`). [verified]
- **The marks request** is the 1038 row's request with only the company name changed. `tools/list` (answer 000) was the first call; it sends nothing to Tally. [verified]

## Kept
- Every relay exchange whole, plus `relay.jsonl`. The 76 `BridgeCompanyExtent` and `BridgeCompanyBookExtentV2` answers carry the company number in `COMPANYNUMBER`. In those, the number is replaced by `redacted` in UTF-16LE, so those answers' `CONTENT-LENGTH` and the `answer_sha256` in `relay.jsonl` refer to the originals, which are listed in `kept-local.txt`. Every other exchange, the window and master-listing answers included, is byte-exact.
- The tool answers (`company_number` redacted, in the JSON and its text), the marks answers with headers, the analysers and their outputs.
- On the lab PC only, by hash: the screens and the originals of changed files.

## The book now
BRIDGE FOREXGROUP LAB, V 11, M 214, unchanged and still loaded, with one tally.exe (PID 1200) on 9001. Relay and Bridge stopped.

**Confidence:** one book, one day, one run, 7.2 Silver: PARTIAL.
