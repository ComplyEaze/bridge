# 2142 drain live (#2142): ComplyEaze Bridge's own drain after a cut post, on its own build, read only, on the dense synthetic book

Not for merge. About 30 minutes. READS ONLY: nothing is created, altered, imported or restored, and no backup is needed (C5 and C6 do not apply). `SHA256.txt` lists every other file here with its bytes; check it before step 0.

## What it decides
The pull request for #2142 makes ComplyEaze Bridge record a post that may have left Tally working (one cut at its limit, among others) beside the port's wire lock. The next post to that port first sends `GET /status` and goes ahead only once three probes in a row, 2 s apart, have answered within their own 5 s deadline, whatever the answer. A probe with no answer in that time starts the count again. After an answer under 1 s the same call probes again; after a slower one, or none, the call is refused as `tally_endpoint_draining`, with `retry_after_s`, having sent no post. A debt clears by itself 10 minutes after the cut request was sent.

Two rows measured the behaviour this rests on with curl (`lab/2142-status-while-busy-answers`, `lab/2142-drain-repeat-answers`). This row runs it through ComplyEaze Bridge's own transport, on the pull request's own build. It must show three things, in order:
1. **The cut, through ComplyEaze Bridge:** `outstandings` at 31 March 2026 is refused when its Bills Receivable request is cut at the 20 s request limit (`native_outstandings_read_failed`, with the cause the pull request names for a cut request). The drain record then reads owed.
2. **A call refused while Tally still works:** `ledger_masters` sent at once is refused as `tally_endpoint_draining`. Its trail shows `/status` sends and no post.
3. **A call admitted after three quick probes:** `ledger_masters`, repeated every `retry_after_s`, is in the end answered. Its trail shows three `/status` sends, then the post, and it answers within the at-rest threshold.

**Expected:** all three, as the curl rows predict: after the cut, Tally's work ran on 8.9 s to 25.4 s. Anything else is a result: report it, and the pull request does not merge on it.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/2142-drain-repeat` at 33d18f5e, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step here on the synthetic company, on the owner's standing word of 27 Oct 2026, and records each step as LAB-COMMON C11 directs.

**THE COMPANY** is `BRIDGE CORPUS DENSE` (synthetic), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`: marks `ALTVCHID` (voucher) 29900 and `ALTMSTID` (master) 328, books from 1 April 2025, Tally's current date 31 March 2026. If it is absent, or step 1's marks differ (record the difference), STOP and report.

Named differences from LAB-COMMON:
1. **No raw request is sent** but C2 c's `GET /status` at step 0 and C10's at the end. Every other request is ComplyEaze Bridge's own, through its tools.
2. **A refused call is a result, not a STOP,** when it is one of the refusals this sheet expects:
   - `outstandings` cut at its limit (step 3);
   - `tally_endpoint_draining` (steps 4 and 5).
   Any other refusal is a STOP.
3. **The drain record** is `<key>.wire.debt` beside the port's wire lock, in the shared profile root (under the user's profile, in the folder the build's `endpoint_wire` names). The operator copies it whole after each step from 2 on, and records its bytes and sha256. That is a file read on the lab PC, not a request.

**Keys.** The Gateway is the only screen used. No key is pressed except the arrows and Enter of Select Company at step 0, and the Company > Shut of C10. **Never Ctrl+Q or Escape at the Gateway of Tally.**

Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` of pull request #2204 at its head `c13a0bd0059ace7d9a4d8e638bdd85144f2596d5`, built by CI run 41013617643 dispatched on that head (the `unsigned-bundle-smoke-windows-latest` artifact, id `12959380456`, 57,442,825 bytes, zip digest `sha256:8657164578579f0d37a828cca2d86cb1b76ad59070b90d0a2cd01e36efa0aa10`). Check the downloaded zip's sha256 against that digest before unpacking; another digest = STOP. The exe's sha256 is recorded at step 0, from that artifact, and again after the last call; a different second sha256 = STOP. A later head of the pull request needs its own build and a new Build line.

Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a new empty `BRIDGE_AGENT_DATA_DIR` (`C:\bridge-lab\2142live\data`, listed first: absent or empty, else STOP), `BRIDGE_TERMS_ACCEPTED=true`, and redaction unset. Writes stay off. `tools/list` must hold `list_companies`, `ledger_masters`, `outstandings` and `egress_log`; any absent = STOP.

## Steps
0. LAB-COMMON step 0 (C2), with no company loaded: the screenshot; the process list and port 9001; `GET /status`; hash the exe; load THE COMPANY alone (arrow keys); a screenshot of the Gateway (EXPECT 31 March 2026). List the drain record's folder: no `.wire.debt` file for port 9001, or one reading `{"state":"clear"}`. Anything else: record it, and report before step 1.
1. Start ComplyEaze Bridge. `tools/list` (saved whole). `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`, `books_from` 20250401.
2. **At rest:** `ledger_masters` (basic), twice, 120 s apart. EXPECT marks 328 and 29900 each time. Record each one's seconds; the threshold is the larger of twice the slower and the slower plus 2 s. Copy the drain record (EXPECT `{"state":"clear"}`).
3. **The cut:** `outstandings` with `as_of` 20260331 and no party. EXPECT the call refused after about 20 s (`native_outstandings_read_failed`, the cause of a request cut at its limit). Record its code, cause, next step and seconds. At once, before anything else, copy the drain record (EXPECT `"state":"owed"`).
4. **Refused while Tally works:** within 5 s of step 3's answer, `ledger_masters` (basic). EXPECT `tally_endpoint_draining` with `retry_after_s`, in at most about 6 s. Record its code, `retry_after_s` and seconds. Then `egress_log` for that call: EXPECT one or more `status` sends and no `post`. Copy the drain record (EXPECT owed, `quick` 0).
5. **Admitted after three quick probes:** after the step's `retry_after_s`, `ledger_masters` (basic) again; repeat after each refusal's `retry_after_s`, at most 20 times or 3 minutes in all, whichever comes first. For each call record its code, `retry_after_s` if any, its seconds, and the drain record after it. EXPECT the last to answer, with marks 328 and 29900, within the threshold of step 2. Then `egress_log` for that call: EXPECT exactly three `status` sends, then one `post`. The drain record after it: EXPECT `{"state":"clear"}`. More than 20 calls or 3 minutes without an answer: STOP.
6. **Repeat, per P6:** wait 120 s, then steps 3, 4 and 5 again.
7. `ledger_masters` (basic): EXPECT 328 and 29900.
8. Stop ComplyEaze Bridge (its stop file), and record its exit code and error stream. Hash the exe.
9. LAB-COMMON C10: the Gateway screenshot; Company > Shut (never Ctrl+Q at the Gateway); Select Company with none loaded; one tally.exe; 9001 LISTENING; `GET /status` 200. No XML request is sent after the shut.

STOP RULES: C7 and C8 of LAB-COMMON; any dialog; a second `tally.exe`; a mark that moves; a refusal this sheet does not expect; step 5's bound; Ctrl+Q or Escape at the Gateway (an incident: press no key, screenshot, STOP); an exe sha256 that is not the one named.

## What the row records
- Steps 3 to 6, as a table for each run: each call, its code, its seconds, its `retry_after_s`, its trail's sends by kind and `held_ms`, and the drain record after it (state, `quick`, `since_ms`, `last_probe_ms`).
- **Reading:**
  - the drain works through ComplyEaze Bridge when step 3 leaves the record owed, step 4 is refused as draining with no post sent, and step 5 ends in an answer after exactly three status sends, within the threshold, with the record clear;
  - both runs must agree.
- **Confidence:** one sitting, PARTIAL. **Not covered:**
  - a connection dropped after sending began, a size limit, or a send abandoned mid-flight, in place of the cut;
  - a second process sending during the drain;
  - other reads, books and Gold;
  - cut points other than 20 s.

## Evidence
- Every tool answer whole, with its size and sha256.
- Each `egress_log` answer whole.
- Each copy of the drain record, with its bytes and sha256.
- The tables above.
- The build's sha256 before and after.
- Screenshots listed by name, size and sha256 (never pushed).
- A notes file with UTC times and one line per step (C11: no roles and no people).
- THE COMPANY's GUID is already in the merge gate's registry. No real figure is recorded beyond the book's own synthetic ones.
