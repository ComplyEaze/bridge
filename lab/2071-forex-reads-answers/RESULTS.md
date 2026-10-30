# 2071 forex reads: a rupee party's part-settled bill arrives as a plain amount; a dollar purchase bill settled in full at a different rate arrives in the receivable report with an empty amount; the one settled at the same rate leaves both reports (Part of #2071)

Sheet `lab/2071-forex-reads` v3 at 65ec0a23 (README sha256 6f904cff…). Round A was read at ea7ef798 and stands under v3. BRIDGE FOREXGROUP LAB, GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, loaded alone. 30 Oct 2026, 19:04 to 20:17 UTC.

**Tally:** TallyPrime Release 7.2 (Latest), Edition Silver Rental, licensed (About screen). `tally_status`: TallyPrime 7.2, silver, `education_mode` false.

**Build:** the published `mcp-v0.7.0` exe, already on the lab PC from `lab/0.7.0-post-publish`. No download was made.
- `bridge_mcp.exe` sha256 48d2f743…, the same before and after.
- `initialize` reported `bridge-tally` 0.7.0.
- Redaction was off (the default): the launch environment held Terms, host, port and the data folder only.

## Marks and backups

| read | ALTMSTID | ALTVCHID | note |
|---|---:|---:|---|
| m0 | 211 | 4 | = the earlier sitting's |
| hold-check, resume-check | 211 | 4 | byte-equal to m0, before and after the hold between Round A and v3 |
| m1 (after stage 1) | 212 | 6 | |
| m2 (after stage 2) | 214 | 11 | |
| m3 (after the reads and screens) | 214 | 11 | = m2 in both marks; the bytes differ only inside `CMPINFO` |

| backup | taken | file | bytes | sha256 |
|---|---|---|---:|---|
| BK2 | step 1 | TDBK1800_<company number>.001 | 31,038 | d23221e41e8f8a50313044fb6218cb07caa78b8e82f86fcc977b757be9b097f9 |
| BK3 | step 5 | TDBK1800_<company number>.001 | 32,692 | cdcb1a5a527ad7bbf399bbf59c34e6acd23f07902f389b9784e9f4768cec19d6 |
| BK4 | step 8 | TDBK1800_<company number>.001 | 37,618 | 3edd94a4fd477098edb377cfb114b2927c437e4c0eccf95f874a504893a221fd |

The company is left shut, in the BK4 state. No backup was restored.

## Keying (Tally's screen, every voucher dated 1 April 2025)

**Stage 1:**
- **L3** BRIDGE INR Customer, under Sundry Debtors. Bill-by-bill showed Yes.
- **K10** Sales No. 3: BRIDGE INR Customer; BRIDGE Export Sales 5,000.00. No Forex box opened. New Ref R1, 5,000.00 Dr.
- **K11** Receipt No. 2: To BRIDGE INR Customer 2,000.00, Agst Ref R1 2,000.00 Cr; By Cash 2,000.00.

**Stage 2:**
- **L1** BRIDGE USD Supplier, under Sundry Creditors. Bill-by-bill showed Yes.
- **L2** BRIDGE Import Purchases, under Purchase Accounts.
- **K5** Purchase No. 1: $100 @ ₹86, New Ref P1.
- **K6** Purchase No. 2: $100 @ ₹88, New Ref P2.
- **K7** Payment No. 1: By the supplier $100 @ ₹87, Agst Ref P1; To BRIDGE Lab Bank ₹8,700.
- **K8** Payment No. 2: $100 @ ₹88, Agst Ref P2; To BRIDGE Lab Bank ₹8,800.
- **K9** Receipt No. 3: To BRIDGE USD Customer $1,000 @ ₹88, Agst Ref bill 1, $1,000 Cr; By BRIDGE Lab Bank ₹88,000.

**What every keyed voucher showed:**
- No gain or loss line on any voucher.
- No Inventory Allocations box.
- Each bank line's Forex Rate of Exchange and Bank Allocations boxes were passed by Enter.
- In K8 and K9, Tally offered the bank line already filled. It was passed by Enter.

**Pending Bills, as Tally showed them:**
- K8 listed P1 as `$0.00 Dr @ ₹ 86/$ = ₹ 100.00 Dr`.
- K9 listed bill 2 as `$0.00 Cr @ ₹ 86.4125/$ = ₹ 58.75 Cr`. This is a decimal rate, left by the earlier sitting's K2.

## Tool reads (0.7.0)

| round | trial_balance | outstandings | ledger_masters (compliance) |
|---|---|---|---|
| A | answers; ledgers: Cash; `base_currency_ledgers_mixed_excluded` 4 (BRIDGE Export Sales, BRIDGE Lab Bank, BRIDGE USD Customer, Profit & Loss A/c); evidence 58,398 B | `native_outstandings_read_failed`, cause `native_amount_invalid`, `bill_row {payable, 1}`; evidence 168,094 B | answers; items: Cash; the same 4 set aside; evidence 192,618 B |
| B1 | answers; ledgers: BRIDGE INR Customer, Cash; the same 4 set aside; 61,038 B | the same refusal at `{payable, 1}`; 171,502 B | answers; items: BRIDGE INR Customer, Cash; the same 4; 199,446 B |
| B2 | answers; ledgers: BRIDGE INR Customer, Cash; 6 set aside (adds BRIDGE Import Purchases, BRIDGE USD Supplier); 66,922 B | `native_outstandings_read_failed`, `native_amount_invalid`, **`bill_row {receivable, 3}`**; 177,606 B | **refused**: `party_ledger_master_read_failed`, cause **`native_foreign_currency_ledger_balance`**; 100,191 B |

- In every answer, `foreign_currency_ledgers_excluded` was 0, and no answer carried `partial` or `partial_reasons`.
- The remediation texts are kept whole in `answers/`.
- Rounds A and B1 match the sheet's EXPECT lines.

## Raw reads (curl, from `sheet\`)
- **Every raw answer:**
  - each answered 200 with CONTENT-LENGTH equal to its size;
  - each was under its tool answer's `evidence.bytes`;
  - sizes and sha256 are in `evidence/log.txt`.
- **Round A against `lab/2071-forex-rerun-answers`:**
  - `bills-receivable` and `bills-payable` are byte-identical;
  - `tb-currency` and both ledger snapshots differ only inside `CMPINFO` (its counters of objects in memory);
  - `group-snapshot` has no earlier answer (54,280 B, the same in all three rounds);
  - the two ledger snapshots are byte-identical to each other in every round.

## Step 10: Tally's raw answers, parsed (`evidence/parsed-a.txt`, `parsed-b1.txt`, `parsed-b2.txt`)

**Bills reports:**

| round | receivable rows | payable rows |
|---|---|---|
| A | 1: bill 1, BRIDGE USD Customer, BILLCL `-87000000.00` | 1: bill 2, BRIDGE USD Customer, BILLCL **empty** |
| B1 | 1: **R1**, BRIDGE INR Customer, BILLCL `-3000.00`; 2: bill 1, `-87000000.00` | 1: bill 2, BILLCL **empty** |
| B2 | 1: R1, `-3000.00`; 2: bill 1, `-87912000.00`; 3: **P1, BRIDGE USD Supplier, BILLCL empty** | 1: bill 2, BILLCL **empty** |

- Every row's BILLDATE and BILLDUE are `1-Apr-25`, and BILLOVERDUE is `29`.
- **R1:** in the receivable report as a plain amount, in B1 and B2.
- **P1:** settled in full at a different rate. It arrives in the **receivable** report, with an empty BILLCL.
- **P2:** settled in full at the same rate. It is in neither report.
- **Bill 2:** in the payable report with an empty BILLCL, in every round.
- **Bill 1:** part-settled at a different rate. It stays a plain amount.

**Ledger balances in B2** (ledger snapshot, as sent):
- BRIDGE USD Supplier's CLOSINGBALANCE is the composite `$0.00 @ ₹ /$ = ₹ 0.00`, with an empty rate, not a plain amount.
- Profit & Loss A/c is `$999900.00 @ ₹ 86.0058/$ = ₹ 85997241.25`, a **decimal rate** in a ledger balance.
- BRIDGE USD Customer is `-$999000.00 @ ₹ 88/$ = -₹ 87912000.00`.
- BRIDGE Lab Bank is `-$900.00 @ ₹ 88/$ = -₹ 79200.00`.
- BRIDGE INR Customer is `-3000.00`.

**Trial Balance in B2** also carries decimal rates in its totals:
- BRIDGE USD Customer's DEBITTOTALS is `-$1000100.00 @ ₹ 87.9999/$ = -₹ 88008700.00`.
- BRIDGE USD Supplier's CREDITTOTALS is `$200.00 @ ₹ 87.50/$ = ₹ 17500.00`.

**Every rate keyed in this sheet was a whole number.** The decimal rates above are Tally's own averages.

**Group snapshot:** 28 group rows in every round. Each row's NAME, PARENT and RESERVEDNAME are in the parsed files.

## Screens (step 11; captures stay on the lab PC)
- **BRIDGE USD Supplier, 1 to 30 April 2025:** the two purchases and two payments, plus `Unadjusted Forex Gain/Loss` on 30-Apr-25, Credit 100.00. Totals 17,500.00 Dr and 17,500.00 Cr; the closing balance is blank.
- **BRIDGE USD Customer:** `Unadjusted Forex Gain/Loss` Debit 20,00,058.75; closing balance 8,79,12,000.00.
- **BRIDGE INR Customer:** Sales 3 5,000.00 Dr, Receipt 2 2,000.00 Cr; closing balance 3,000.00; no forex line.
- **Bills Receivable as of 30-Apr-25:** R1 3,000.00; bill 1 8,79,12,000.00; P1 with no pending amount.
- **Bills Payable:** bill 2 with no pending amount.

## Deviations
- **The hold between Round A and v3.** Round A was read under v1, then the run was held before any keying. In that hold, the Create > List of Masters picker was opened and closed by Escape before any ledger screen opened. The book was shut while another item used another book. At the resume, FOREXGROUP was loaded alone, and resume-check was byte-equal to m0.
- **The supplier's pending bills** were recorded from the K8 Pending Bills list, not asked for by the sheet.
- **The evidence zip** is listed in `kept-local.txt` with its sha256.

Not measured: a fixed build's reads (BK4 is kept for them); a rate with decimals keyed by hand; any period other than April 2025.
