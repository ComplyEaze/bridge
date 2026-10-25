# K1 (#685): does a voucher filter on $RemoteID find the voucher ComplyEaze Bridge posted directly under that REMOTEID?
WHY: the only way a directly posted voucher could be corrected in place is a same-REMOTEID upsert, which is safe only if Bridge can first prove the voucher carrying that REMOTEID is the one it means. No read has returned the key so far; this asks Tally to filter by it, and computes what $RemoteID holds, beside a GUID control.
RULES: LAB-COMMON v3 C1 to C11 apply in full. One call at a time, through the built tools only; no request is written, edited or sent by hand. Licensed mode only (ordinary days); Educational mode anywhere = STOP. Every probe answer is recorded whole, with the request and answer files the probe persisted (names and sha256 from `lab-manifest.jsonl`).
COMPANY: BRIDGE KEY PROBE LAB (new, THE COMPANY), loaded alone, books beginning 1 April 2025: a bank ledger BRIDGE Probe Bank (Bank Accounts), an expense ledger BRIDGE Probe Exp (Indirect Expenses); voucher type Payment, automatic numbering; no voucher on D1 (10 April 2025). If a company or data folder of this name exists on the PC, STOP and report it. OWNER STEP: create it; take backup BK1 into a NEW empty folder (name, size, sha256) before step 3 (C5).
STEPS:
1. `list_companies`: exactly BRIDGE KEY PROBE LAB, loaded alone; STOP before any other call otherwise. Record its GUID (set BRIDGE_LAB_TARGET_GUID to it, then restart the exe). `tally_status`: M0, V0.
2. `build_import_xml`: ONE Payment on D1 of 100.00 from BRIDGE Probe Bank to BRIDGE Probe Exp, narration "K1-P", no voucher number. Record the batch id.
3. `post_import` of that batch (OWNER STEP: approve in the native dialog). Record the answer whole. From `<BRIDGE_AGENT_DATA_DIR>/agent-import-ledger.jsonl`, copy the batch's dispatch-intent line and record its `native_remote_id`: that is R. `tally_status`: M1, V1.
4. `verify_import` of the batch: expect `posted_verified`. Record the voucher's `guid` (G), `master_id` and `alter_id`.
5. `lab_voucher_key_probe`, `date` D1, one call at a time, each answer whole:
   P1 `term` guid, `key` G            (control)        expect `vouchers_with_guid` 1, its `guid` G. Record its `remote_id`, `remote_id_equals_key`, `remote_id_equals_guid`.
   P2 `term` remote_id, `key` R       (THE question)   record `vouchers_with_guid` and, if 1, that voucher's `guid`.
   P3 `term` remote_id, `key` G       (discriminator)  record `vouchers_with_guid`.
   P4 `term` remote_id, `key` 00000000-0000-4000-8000-000000000002 (a listed synthetic value no native post mints)   expect 0.
   P5 `term` guid, `key` the company GUID followed by -0fffffff (no such voucher) expect 0.
   In every answer record `rows_without_guid` too.
6. `tally_status`: M2, V2 (equal M1, V1: the probes are reads).
DECIDE (record, then report; the run coordinator decides what follows):
- P2 = 1 with guid G, and P3 = 0: $RemoteID holds the key Bridge sent. Report at once.
- P2 = 0, and P3 = 1 (P1 `remote_id_equals_guid` true): $RemoteID is the voucher's GUID; the key is not findable this way.
- P2 = 0, P3 = 0, and P1 `remote_id` null: $RemoteID returns nothing for a voucher.
- Anything else (P1 not exactly 1 with G, P4 or P5 not 0, a row without a GUID in any answer, M2/V2 not equal M1/V1) is a finding: record it, repeat nothing, report.
PASS = steps 1 to 6 recorded whole, with R, G, the five answers, their request and answer files and sha256.
AFTER: nothing to restore for the reads; the company stays for a later row unless the run coordinator says RESTORE (BK1).
STOP RULES: C7 (any Tally dialog: screenshot, copy its text, press nothing, report); a refusal from any tool (record it whole); step 4 not `posted_verified`; any answer outside the stated shape.
