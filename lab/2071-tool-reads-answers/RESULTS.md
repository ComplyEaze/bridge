# Results: #2071 first tool-level read of the forex book, 2 Nov 2026

**No read refused. `trial_balance` and `ledger_masters` read the rupee ledgers and set six aside by name, Profit & Loss A/c among them. `outstandings` answered partial. The row's EXPECT holds.**

Book BRIDGE FOREXGROUP LAB as left, TallyPrime 7.2 Silver, loaded alone. Build master 3e87d00e, run 41474115716, artifact 13156753754, zip digest equal, `bridge_mcp.exe` sha256 15c423e05b8d057d25370ea169809a5e79dc530fb677929d34bc42d5cf96987b, equal to the package's smoke file, before and after. No write setting was on. Marks before and after are the same bytes, sha256 3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba; the tools' snapshots read M 214, V 11.

| call | `isError` | evidence state | result state | rows | seconds |
|---|---|---|---|---|---|
| `trial_balance` 1 to 30 Apr 2025 | false | complete | observed | 2 ledgers, all shown | 0.85 |
| `outstandings` as of 30 Apr 2025 | false | partial | partial | 1 open bill, 1 party | 2.20 |
| `ledger_masters` fields=compliance | false | complete | no state field | 2 items | 1.61 |

[verified]

## Set aside
All three answers name the same six, reason `mixed_currency_movement`, and none kept in another currency:

BRIDGE Export Sales, BRIDGE Import Purchases, BRIDGE Lab Bank, BRIDGE USD Customer, BRIDGE USD Supplier, Profit & Loss A/c

[verified]

## What each answer says
- `trial_balance` keeps BRIDGE INR Customer and Cash, scope `base_currency_ledgers_only`, and its lead says debit and credit totals are not expected to match. Its closing total over the two is -6000. [verified]
- `outstandings` carries `partial_reason` `currency_ledgers_excluded` and `partial_reasons` `["mixed_currency_ledgers_excluded"]`, so the two fields use different words for the same thing. Its open bill is R1 of BRIDGE INR Customer, 3000.00, age 29 days, in the 0 to 30 bucket. Its ageing is on due date. [verified]
- `ledger_masters` lists BRIDGE INR Customer and Cash, `compliance` `paired_party_ledger_master_source`. [verified]

Why six ledgers carry a mixed movement is not read here; the answers give the reason code only. **Confidence:** one book, one Tally, one sitting: PARTIAL.
