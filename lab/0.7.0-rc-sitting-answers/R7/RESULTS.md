# R7: a ledger asked for by its alias (#1688): PASS on the alias, one FINDING on the control

Build: candidate A, RC COMMIT 026ba163dfcbf75d515a24d0dbbf14c5245534ec, EXE SHA256 576cc0c5aab4e72c43c05799116d76a29429c595a86dd54834d841c551f50d7c (bridge-tally 0.6.1). Company: BRIDGE LINE BREAK LAB (GUID 224e7f8c-a0aa-45ce-889a-ad4b7fdbe852), synthetic. Through ComplyEaze Bridge reads only, posting off. The alias was added on Tally's own screen between BK0 and its restore. TallyPrime 7.2 Silver. 30 Oct 2026, 07:00 to 07:08 UTC. Folder `<LAB_DIR>\`.

| step 2 EXPECT (`validate_masters`, `answers/006`) | result |
|---|---|
| `Lab Plain Alias`: `match_state` `missing` or `near_miss`, with `alias_of` `Lab Plain Ledger` | **PASS**: `near_miss`, `alias_of` `Lab Plain Ledger` |
| `Lab Plain Ledger`: `exact`, with no `alias_of` | **PASS**: `exact`, `exact_live_spelling` `Lab Plain Ledger`, no `alias_of` |
| the control `Lab Alias Control Absent`: `missing`, with no `alias_of` | **FINDING**: `near_miss` (reason `master_binding_near_miss`, three candidates by rule `shared_token`: the book's other ledgers whose names share "Lab"), with no `alias_of` |

The FINDING, on the run lead's word: the sheet expected `missing`, but the build's shared-token rule lists the control as a near miss, with three candidates, no `alias_of` and no binding. A near miss never binds a ledger, so it fails safe. The sheet's expectation was wrong, not the build.

Other steps:
- Step 0: THE COMPANY alone; M0 212 / V0 2; L listed; no ledger or group like the alias.
- Step 1: BK0 taken; M0′ 212; the alias added; M 213 = M0′ + 1, V 2, L under its own name.
- Step 3: M 213 / V 2.
- Step 4: BK0 restored; M 212, V 2 and the ledger rows equal to step 0's; L's Alter form shows no alias.

Deviation: step 3's two reads (marks and `egress_log`) were sent in the same batch as step 2, before step 2 was compared with its EXPECT. Both are reads.

Not covered: the twin check through an alias, a build naming the alias, aliases of groups, Gold.
