# Synthetic write-canary fixture gate

This gate records a local, revocable operator attestation for a future synthetic
write-canary. It does not construct XML, call Tally, write to Tally, change the
Capability Passport, or establish write support.

## Enrollment prerequisites

1. Use a dedicated disposable synthetic company. An existing demo company is
   not automatically eligible: the operator attestation is a gate, not proof
   that the company is disposable.
2. Do not use customer, personal, or production data.
3. Before any later canary, create an offline backup, record how to restore it,
   and verify the restore path against a separate copy. If this is not possible,
   do not acknowledge the backup guidance and do not proceed.
4. Persist the selected GUID-bearing company scope, then obtain a separate fresh
   Probe review for the local enrollment. A review consumed by setup save cannot
   be reused.

## Local effects and revocation

Enrollment stores only commitment hashes, attestation flags, and local event
timestamps. It does not store fixture content, company names, GUIDs, backup
locations, or free text in the enrollment evidence tables. The UI must continue
to report `write capability: Unknown`.

No canary payload, coordinator, dispatch path or command exists in ComplyEaze
Bridge. The canary tables created by migrations `0015` to `0021` remain in the
local database, with no reader or writer. Enrollment is local, explicit and
revocable, and it gates nothing: ADR 0004 row 11 records the canary rule as not
met.

Revocation appends a local `operator_revoked` event. It changes the local
candidate gate only and never alters Tally. A revoked fixture requires a new
fresh review and a new complete attestation before it can be enrolled again.
