# L4b (#1787, #1909): the not-loaded refusal again, on a build that names its remedy. READ-ONLY.

Not for merge. `WINDOWS-72-READS.md` in this folder applies, except its Build block: this row runs on the build named below, not on 2306d86d. One read, synthetic BRIDGE companies only, no write and no import setting.

**WHY.** On 25 Oct (`lab/72-read-regression-answers`, its L4 section) the read of a company that exists but is not loaded was refused `company_identity_not_found` with the general message only: no `cause` and no `remediation`. Pull request #1909 (merged since) gives that code its own remediation. This row checks it on TallyPrime 7.2. The code itself is not expected to change.

**BUILD.** Bridge CI run 40362694215 on master `fd6f7c0d032ec51f9d9de9c80e8bcd33d930ad77` (it holds #1909), artifact `unsigned-bundle-smoke-windows-latest` id 12668144368, zip digest `sha256:0a06f07251060a6f9b6469c8e4a2df63198c540e64aaf329c6e64704e364baa5`, `bridge_mcp.exe` sha256 `63641231787492201133443cb1a887705c7137f495a9205ffd33c2d0068f7484`, `.mcpb` sha256 `9572c5941b7045c374aaf3bb903ac073481b8e203b146ad14caf527da7fe3230`. Hash the exe before the call and after it; another hash = STOP. Over stdio, `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_REDACTION` unset, a new empty `BRIDGE_AGENT_DATA_DIR`, and `BRIDGE_AGENT_ENABLE_WRITES` and `BRIDGE_AGENT_ENABLE_IMPORT` unset. Record the version the tool list reports.

**STEPS.**
1. As L4: load one BRIDGE company alone (N2, as on 25 Oct, or another synthetic one; name it). `list_companies`: EXPECT exactly that company; anything else = STOP.
2. `ledger_masters` with `args\l4-ledger-masters-not-loaded.json`, its placeholder replaced by the GUID of another BRIDGE company that exists on the PC and is not loaded (N1 on 25 Oct). Save the answer whole.
3. EXPECT: refused, `result.error.code` `company_identity_not_found`, and `result.error.remediation` present, beginning "The company list Tally returned held no company with this company_guid, so nothing from its books was returned." and naming `list_companies`. No rows. Record whether a `cause` is present (not expected).
4. `egress_log`: record this call's `response_prepared` line whole (data only).

**PASS** = the code as expected and the remediation present with that opening sentence. **FAIL** (a finding) = no remediation on this build, another code, or any row returned. **STOP** on any dialog, a second company loaded, or Education mode.

**CLOSE.** Shut the company (no company loaded, as found). Hand-back: the answer whole with its sha256, the egress line, the exe hash at the start and the end, the `tally.exe` version, UTC times; on a new branch `lab/72-l4b-answers` with its own `SHA256.txt`, after the pre-push scan and the run coordinator's yes.
