# R5: the cost-centre summary of #2149 through the tool: PASS

Build: candidate A, RC COMMIT 026ba163dfcbf75d515a24d0dbbf14c5245534ec, EXE SHA256 576cc0c5aab4e72c43c05799116d76a29429c595a86dd54834d841c551f50d7c (bridge-tally 0.6.1). Company: BRIDGE COST KEY LAB (GUID ddf3a557-704c-4f61-ae8d-46a3380062a1), synthetic, books from 1 April 2025. Reads only; posting off. TallyPrime 7.2 Silver. 30 Oct 2026, 06:57 to 06:59 UTC. Folder `<LAB_DIR>\`.

| EXPECT | result | record |
|---|---|---|
| step 1: exactly THE COMPANY; M0 and V0 | PASS: verified_tuple; M0 215, V0 2 | `answers/001`, `answers/002` |
| step 2: two vouchers; CK Expense with two allocation rows each, the bank entry none; nothing unreadable | PASS | `answers/003` |
| step 3: four buckets in order, each with members CK Expense only, 1 member, 1 voucher, refs complete | PASS: CK North -600, CK South -500, CK Retail (CK Region) -500, CK North East -400; credit 0 each | `answers/004` |
| step 3: category_totals | PASS: Primary Cost Category 2 entries, debit -1500; CK Region 1 entry, debit -500; credit 0 each | `answers/004` |
| step 3: totals, entries_unallocated, vouchers_summarised, entries_counted | PASS: debit -1500 and credit 0; 2 entries, credit 1500; 2; `all_entries` | `answers/004` |
| step 4: with ledger CK Bank | PASS: no buckets, `selected_ledger`, 2 unallocated entries | `answers/005` |
| step 5: marks unchanged | PASS: M 215, V 2 | `answers/006` |

Amounts were compared as numbers. Not covered: masking (`mask_parties`), any other window or book, Gold.
