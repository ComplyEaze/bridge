# Run sheet for #685: can Tally find a directly posted voucher by the REMOTEID ComplyEaze Bridge sent?

Not for merge. LAB-COMMON v3 (C1 to C11) applies in full. One row, K1, on a NEW synthetic company, owner steps marked. Reads only, apart from the one native post that makes the voucher the reads look for. Record the TallyPrime version and edition, the licence mode and the date.

## Why

A voucher ComplyEaze Bridge posts directly carries a private REMOTEID, recorded in Bridge's journal, and no narration marker. A read returns the voucher's GUID where the REMOTEID attribute stands, never the key that was sent (protocol reference 9.3). Whether such a voucher could ever be corrected in place depends on whether Tally can find it by that key. K1 asks it once, through a reviewed read builder, with controls on both sides.

## Build

The draft pull request that adds `lab_voucher_key_probe` (Part of #685), at the head named when the item is routed, built with the `lab-writes` feature on the lab PC (no CI artifact carries that feature): `cargo build --locked --release --manifest-path src-tauri/Cargo.toml -p bridge --features lab-writes`. Record the commit, and the sha256 of the exe, before the first call. If the head differs from the one named when the item is routed, STOP before the first call and ask the run coordinator which build runs.

Run it as the lab runs a staged exe over stdio, with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a NEW empty `BRIDGE_AGENT_DATA_DIR` for the item, `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), `BRIDGE_AGENT_REDACTION` unset, `BRIDGE_AGENT_ENABLE_IMPORT=true`, posting ON as the lab's standing setup has it for a native post (step 3 only needs it), `BRIDGE_LAB_WRITES=1`, and `BRIDGE_LAB_TARGET_GUID` set to the new company's GUID from step 1 (the company is loaded alone, so no deny list is needed). The probe persists every request and answer under `<BRIDGE_AGENT_DATA_DIR>/lab/`, named by sha256, with `lab-manifest.jsonl`.

`K1-request-template.xml` is the probe's request with its four variables as placeholders (COMPANY, DAY, METHOD, KEY); the unit test `the_key_probe_request_has_the_term_and_key_as_its_only_variables` pins the rendered bytes. Its header ID equals its collection name. It is not sent as a file: every request goes through the tool.

`SHA256.txt` lists every file here.
