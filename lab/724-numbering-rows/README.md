# Run sheet for #724: the two numbering rows still to run

Not for merge. LAB-COMMON v3 (C1 to C11) applies in full. One row-pair on a NEW synthetic company, owner steps marked, backup first, restore on the run coordinator's word. Every call is made by the built tool, so no hand-written request is sent. Record the TallyPrime version, the licence mode and the date. The build is named below.

## Build (filled 26 Oct 2026)

The build already staged at the lab as `w1-2306d86d`: the Windows bundle of master `2306d86dfd4c86a3c5f6c19ea9ddff6818c460fb`, Bridge CI run `40077056130` (event `schedule`, conclusion `success`), artifact `12553782422` `unsigned-bundle-smoke-windows-latest` (60,880,528 bytes, zip digest `sha256:74ae2daa851397b71cb59e80373d9478a2da72c74398108e33e34ea83e56eb1e`, expires 2026-10-31 08:16 UTC). Its `bridge_mcp.exe` as the lab recorded it on staging: sha256 `92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524`. Before the first call hash the exe that will run and compare it with that line; any other hash is a STOP. Record the version the tool reports.

RC COMMIT: 2306d86dfd4c86a3c5f6c19ea9ddff6818c460fb  EXE SHA256: 92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524

Run it as the lab runs a staged exe over stdio, with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a NEW empty `BRIDGE_AGENT_DATA_DIR` for the item (kept for the whole item, so the batch journal and the egress log carry over from call to call), `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), and `BRIDGE_AGENT_REDACTION` unset. Posting ON means `BRIDGE_AGENT_ENABLE_WRITES=true`; posting OFF means it is unset and `BRIDGE_AGENT_ENABLE_IMPORT=true` is set instead (`build_import_xml` refuses unless one of the two is set). The approval window opens from the exe's own process and may open behind other windows without coming to the front: the owner looks for it (the taskbar) and answers it by mouse; nothing else is clicked while it is open.

`SHA256.txt` lists every file here.
