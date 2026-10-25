# Results: LR5, `audit_test_run` on TallyPrime 7.2 (25 Oct 2026). READ-ONLY. Confidence: one run, PARTIAL

The sheet is `lab-evidence/697-0.7.0-requests` @ f306dca6 (`SHA256.txt` 7/7 OK).

**Build:** the staged Windows bundle of master 2306d86d. The artifact zip digest was checked before unpacking (74ae2daa…e56eb1e), and `bridge_mcp.exe` is 92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524 (serverInfo `bridge-tally` 0.6.0).

**Tally:** TallyPrime 7.2 (Latest), Silver, `tally.exe` 1.1.7.2, Education mode No (the company list's `EduMode` read No); port 9001. One company was loaded alone at every send.

**How each file ran:** as the bundle's `bridge_mcp` over stdio, one request line at a time, waiting for each answer before the next. Each had a fresh empty `BRIDGE_AGENT_DATA_DIR` and the environment of the sheet, section 2:
- `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_TERMS_ACCEPTED=true`;
- A, B and C with `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true`;
- D with it unset;
- E with it set and `BRIDGE_AGENT_REDACTION=mask_parties`.

No other `BRIDGE_` variable was set, and `BRIDGE_AGENT_REDACTION` unset reads `none` (docs/agent/README.md). The filled files are in `filled/`: the TEMPLATEs with `<GUID>` replaced, every other byte kept.

## Deviations, each agreed with the run's review before or as it happened
1. **Bank groups.** In BRIDGE SHAPE LAB the bank ledger `HDFC CC` sits under `Bank OD A/c` (under Loans (Liability)), not under `Bank Accounts`. The templates were kept exactly, with `bank_groups` ["Bank Accounts"], so `HDFC CC` is outside the test. The tool's own limitation line says the same: "a ledger outside them is not counted, and whether an overdraft group is a bank leg is a judgement for the user's accountant." The other ledgers sit as follows:
   - `Cash` under Cash-in-Hand;
   - `Bank of Baroda CA` under Bank Accounts;
   - `Shape Reparent Probe 20260922` under Bank Accounts (no transaction in the year).
2. **Order.**
   - A, then D and E on SHAPE LAB, then B1 and B2, then C on BRIDGE PILOT LAB.
   - Step 2, the screen totals, came after the tool files; it is independent of Bridge.
   - The book was known to hold cash and bank vouchers in the year, so step 2's "vacuous" stop could not apply.
3. **Masking.** In the committed copies of `a-list`, `a-list-pilot`, `b1` and `b2` (`*.masked.out`), the value of `company_number` is replaced by `<company number>`; nothing else is changed. The unmasked files stay on the lab PC, with sizes and sha256 below.

## Steps and answers
| File | Company (GUID) | Wall-clock | Result |
|---|---|---|---|
| A `a-list` | BRIDGE SHAPE LAB (3a6bd6e1-b835-4bff-89dd-8a6af138c346) | 0.2 s | `tools/list` 28 tools, `audit_test_run` once; `list_companies` one company |
| D `d-setting-off` | SHAPE LAB | 0.2 s | `tools/list` 27 tools, `audit_test_run` absent; id 30 refused **`audit_tool_disabled`**; no Tally request (no request trail, evidence bytes 0) |
| E `e-redaction-on` | SHAPE LAB | 0.2 s | id 40 refused **`audit_redaction_unsupported`**; no Tally request |
| B1 `b1-shape-lab-h1` | SHAPE LAB | 7.8 s (id 11: 7.6 s) | id 11 `completed_with_failed_book_checks`; 37 Tally requests, 0 failed |
| B2 `b2-shape-lab` | SHAPE LAB | 14.6 s | id 10 `completed_with_failed_book_checks` (37 requests); ids 12 to 16 below |
| A `a-list-pilot` (GUID for C) | BRIDGE PILOT LAB (6b43e498-430c-4d5c-bfef-d32e2ab93c85) | 0.3 s | one company |
| C `c-pilot-lab` | PILOT LAB | 5.7 s (id 20: 5.5 s) | id 20 refused **`C5-books-from`**, after a read of 34 requests |

**B1 gates before B2:**
- id 11 was not refused;
- the send took 7.8 s, under 120 s;
- no Tally dialog appeared.

The id 18 egress log lists only HTTP posts (37, 0 failed) and status checks. It carries no request-type field, so it cannot label each post an Export; it shows no other kind. The build's only write path, import, was off in this environment.

### Known answer from Tally's own screens (step 2; SHAPE LAB; Display > Account Books, Exception Reports; period set per report only)
| Screen | 1-4-2025 to 31-3-2026 | 1-4-2025 to 30-9-2025 |
|---|---|---|
| `Cash` (the only ledger under Cash-in-Hand with transactions), Ledger Monthly Summary | Debit 37,000.00, Credit 21,065.00 | Debit 37,000.00, Credit 20,177.00 (April 25,000 / 18,977; June 12,000 / 1,200; March credit 888) |
| `Bank of Baroda CA` (Bank Accounts) | Debit 28,464.50, Credit 52,735.00 | the same (April 28,464.50 / 36,435.00; June credit 16,300.00) |
| Contra Register | 4 vouchers, all April 2025, none cancelled: HDFC CC Dr 1,100.00; Cash Dr 20,000.00; HDFC CC Dr 1,500.00; Bank of Baroda CA Dr 15,000.00 | the same |
| Optional vouchers | 1: 2-Jun-25 Payment, Power Charges Dr 1,100.00 (its credit is `Cash`: the item's raw voucher read, lab/698 read of the same book) | same |
| Cancelled vouchers | 1: 3-Jul-25 Purchase (no amounts) | same |
| Post-dated vouchers | 1: 4-Jun-25 Payment, Power Charges Dr 1,300.00 (its credit is `Bank of Baroda CA`, same source) | same |

Tally's ledger totals leave the optional voucher out and include the post-dated one. The Contra credits fall as follows:
- **Cash:** 15,000 (the Bank of Baroda CA Contra).
- **Bank of Baroda CA:** 22,600 (20,000 to Cash, and 1,100 and 1,500 to HDFC CC).

So the known answer, in rupees, is:

| Figure | FY from the screens | Tool id 10 (FY) | H1 from the screens | Tool id 11 (H1) |
|---|---|---|---|---|
| cash_receipts | 37,000 − 20,000 = 17,000.00 | 1,700,000 paise | 17,000.00 | 1,700,000 |
| cash_payments | 21,065 − 15,000 = 6,065.00 | 606,500 | 20,177 − 15,000 = 5,177.00 | 517,700 |
| bank_receipts | 28,464.50 − 15,000 = 13,464.50 | 1,346,450 | 13,464.50 | 1,346,450 |
| bank_payments | 52,735 − 22,600 − 1,300 (post-dated) = 28,835.00 | 2,883,500 | 28,835.00 | 2,883,500 |
| cash_share_receipts | 17,000 / 30,464.50 = 55.80% | 5580 bp | 55.80% | 5580 |
| cash_share_payments | 6,065 / 34,900 = 17.38% | 1738 bp | 5,177 / 34,012 = 15.22% | 1522 |
| applicable_threshold | shares above 5%: Rs 1 crore | 1,000,000,000 paise | Rs 1 crore | 1,000,000,000 |

**Every figure equals the known answer, for the full year and for H1 (C5, C15).**
- `book_checks.evaluated` lists 8 codes (ID-1, MAP-0, MAP-1, POP-0, POP-1, POP-2, POP-3, POP-5); `failed` lists POP-1 count 2 and POP-3 count 2, in both runs.
- The population note reads "optional, cancelled and post-dated vouchers excluded; Contra excluded".
- No non-Contra voucher with both a cash and a bank line was found in the raw voucher read of 1 April to 30 June 2025. That window holds every Bank of Baroda CA movement and every cash movement of the year except March's 888 credit.

### Refusals (B2)
| id | Expected | Seen | Tally requests |
|---|---|---|---|
| 12 (`Cash-in-Hand X`) | `BIND-GROUP-UNKNOWN` | `BIND-GROUP-UNKNOWN` (after the read, evidence bytes 2,862,960) | 37 |
| 13 (overlap) | `audit_groups_overlap` | `audit_groups_overlap` | none |
| 14 (`cash_payments_40a3`) | `audit_test_not_admitted` | **`argument_invalid:test`**: FINDING, a different code; still refused before any request | none |
| 15 (FY 2024-25) | `audit_period_outside_ay_2026_27` | same | none |
| 16 | `audit_period_outside_ay_2026_27` | same | none |

Egress (id 17, and the data folder's `agent-egress.jsonl`): Tally requests only for ids 10 and 12 (37 each, 0 failed), none for 13 to 16. Observation: the `response_prepared` line records no `error` code for id 12 or for id 14, although both answers were refusals. It does record one for 13, 15 and 16.

### C9b, PILOT LAB, FY 2025-26 (books from 1 Apr 2026, 0 vouchers in that year)
- id 20 was refused with the engine rule code **`C5-books-from`**, after 34 Tally requests (0 failed).
- By request and response size, the trail shows the identity, marks, company object, groups, ledgers, trial balance and voucher types reads. It also shows two small requests (1,436→3,022 and 1,740→3,022 bytes), likely the census and one voucher read of zero rows.
- Not `completed`, so not the finding the sheet names. Step 7b was not run (it creates a company).

### C16 leak check (step 6), on the answers of ids 10 to 16
The search covered every ledger, group and party name, voucher number and voucher GUID of the book, taken from the item's raw reads of the same book. It found none:
- the only word matches are the tool's own wording ("Cash receipts…", `cash_44ab`) and `test_version` "1";
- the company GUID appears once each in ids 10 and 11, in the company block the call echoes, with the company name and number.

### C18
- Each data folder holds only `agent-egress.jsonl` and `terms-acceptance.jsonl` after its file. The folders were absent before; each was made new for its file.
- The book's voucher and master marks read 112 / 292 before this item (the item lab/698's reads) and again after every LR5 step (one MARKS read, sha256 c71056a0…, identical bytes).

## Files (sha256 of the answers as received; `*.masked.out` in this commit)
| File | Bytes | sha256 |
|---|---|---|
| a-list.out | 146,558 | 6b4d17e9fafdc1e26321e9bb23c9cf8d346b38f976d59edb56a496246d841aeb |
| a-list-pilot.out | 146,558 | 3551ef18463617639faa2553aeba627338e06c47ab81092bb85556212f44964c |
| b1-shape-lab-h1.out | 22,198 | 1e42f9d65f20dd7de87e9784e1528fca9fde49a704b9f2efda69cf430a4528b1 |
| b2-shape-lab.out | 48,461 | d55bc4c4ad25dc46d28f8119915aa3514947fd5087adfc517c4efd6914854933 |
| c-pilot-lab.out | 14,811 | 90683a57bf7c88a6b05bc174d9fe5da7c2b70db529ffabc1b70b5707535a7ee1 |
| d-setting-off.out | 143,966 | 670c27032d0035edc73ef7dffc74a29bc8114664e4a87abd6dd2a480976d6511 |
| e-redaction-on.out | 3,067 | 34506fe28ced3666e4a01a6fa294331b2e081a8ca0677e4010e03a2d4690250b |
| every `.err` | 0 | e3b0c442… |

## Close
- BRIDGE SHAPE LAB was shut. Tally is at Select Company with no company loaded.
- Tally's current period and date (1-Apr-26 to 31-Mar-27; 20-Apr-2026) are as found. The report periods were set inside each report only.
- One `tally.exe` owns 9001.

## Not run
- 7b (creates a company).
- S1 (a write).
- C10 to C13 and C19 live (not planned).
