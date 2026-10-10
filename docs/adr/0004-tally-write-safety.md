# ADR 0004: Keep Tally writes disabled behind a controlled sandbox

Status: accepted for the safety contract; transport dispatch is not yet
enabled.

**Amended 2026-09-21 (Deviation, 2026-09-08: #236).** The line above is
superseded in part: transport dispatch *is* enabled for one path,
`post_import`, which shipped without citing this ADR.
The text below is kept as written; read the **Amendment** at the end before
relying on it as a description of current behavior.

## Decision

Writes require an observed runtime capability, explicit operator opt-in, a
synthetic company during initial validation, backup guidance acknowledgement,
small batches, exact validation and preview commitments, approval evidence, a
durable idempotency reservation, parser-derived Tally counters, and a strict
company-bound read-after-write verification.

The first live write has a deliberately narrower rule. Before write capability
can honestly be recorded as observed, Bridge may prepare one sealed synthetic
ledger-canary *candidate* only after a fresh, GUID-bound company check and a
local fixture enrollment that records the operator's disposable-company and
backup acknowledgements. Candidate status is not general write authorization,
does not enable arbitrary payloads or batch writes, and never upgrades the
Capability Passport. Only a parser-derived import receipt together with the
exact, company-bound readback can create observed write evidence. Until then,
including after a timeout, support remains unknown.

The lifecycle is draft, validate, preview, approve, arm, send, parse, verify,
and then verified/partial/failed/outcome-unknown. A timeout or connection loss
after bytes may have been sent is outcome-unknown and is never retried
automatically. Bridge does not call an action rollback unless a compensating
Tally operation has been implemented and verified.

Wire bytes, canonical intended state, the import response, canonical readback
state, and identity coverage use distinct domain-separated commitments. Import
and readback evidence is opaque and parser-derived; callers cannot assert
counts, identity-presence booleans, or a payload hash as proof of observed Tally
state. Raw line-error text is reduced to ordered, domain-separated digests and
is never retained by the portable contract.

The first qualification profile is ledger-only, limited to create/alter, and
requires a RemoteID-bound preflight. Alter intent declares the exact before and
after state and no-op alters are rejected. A parsed receipt must match exact
create/alter counts (with zero deletes) before an exact applied verdict is
possible. A lost response remains outcome-unknown even when a later readback
matches before or after state; that observation may aid investigation but is
not promoted without import-result evidence. Automatic retry is always false.

## Consequences

The network-free contract can be reviewed and tested before dispatch is
introduced. It commits to private deterministic import bytes but exposes no
public byte getter or transport adapter, and every prepared write remains
ineligible for dispatch. Legacy durable rows created by the earlier
caller-attested recovery contract remain readable but cannot be promoted to a
success/recovery terminal state. A later migration must persist the opaque
derived commitments separately before runtime wiring is considered. Fixture
enrollment is local, explicit, and revocable; it stores only durable local
commitments and safe status. It is not proof that a company is disposable, is
not a Tally write, and is insufficient to arm a generic write path.

## Amendment — Deviation, 2026-09-08: #236

### What happened

On 2026-09-08, #236 (`e1ea3c86`) added `post_import`, which posts one saved
Journal to Tally over the XML gateway after a native operator approval. #238
(`ea6a4219`) added a desktop action that uses the same service. Neither change
cited this ADR or edited it. So from that date this ADR's status line ("transport
dispatch is not yet enabled") and its Consequences ("every prepared write
remains ineligible for dispatch") were false. This amendment records the
deviation. The owner chose to document it rather than withdraw the path. The
amendment changes no code.

The owner accepted #236's scope on 2026-09-08 as a "quiet-company" first
version: the approval tells the operator to pause other edits and imports and
to keep the company and Tally mode unchanged until Bridge finishes. Support
for concurrent external changes is open as #239.

### Scope of the dispatching path

Evidence base: master `4f050da`. Re-checked at `529bc12b` (for #572, #576, #577, #578 and
#579): the "What it can send", aiming and gates bullets, rows 2, 6, 12 and 14, and
follow-ups 3, 9, 10 and 12. The code diff since `4f050da` leaves the other rows accurate. Code paths are relative to `src-tauri/src/` unless another
root is shown (`packaging/`, `src-tauri/tests/`, `src-tauri/crates/`, `src-tauri/src/db/`).

- **Entry points.** The MCP tool `post_import` (`agent.rs`, `tool_payload`)
  and the desktop "post a saved Journal" action
  (`agent_desktop_journal.rs`, `DesktopJournalService::post`) both call
  `post_import_checked` (`agent_import_post.rs`).
- **What it can send.** Exactly one voucher per call, of type Journal (and,
  since the 2026-09-22 amendment below, Payment, Receipt or Contra through the
  MCP tool; the desktop action stays Journal-only), from a
  batch Bridge built and saved earlier under the `BatchV1` identity scheme.
  Enforced in `admit_saved_voucher_integrity` under a `PostScope`: one voucher,
  of an admitted type, amendments refused (`import_post_amendment_requires_file_import`). For a
  batch not yet dispatched, before any request to Tally, the post path reads
  the batch file Bridge persisted at build time
  (`Server::read_persisted_import_xml`: the leaf is not followed if it is a
  symlink, and it must be a regular, single-link file, owned by the current
  user on Unix, size-capped) and requires it to equal,
  byte for byte, the XML the journal record renders (#578). A difference
  refuses with `import_batch_changed`; a missing file refuses with
  `import_persisted_file_unavailable`. The native post re-renders the same
  vouchers from that record, differing only in a fresh private REMOTEID, so
  every accounting field it sends is one the check covered. This proves
  that the record and the saved file agree. It is not proof that Bridge
  wrote them, because anyone able to write both files in the private data
  directory can make them agree. The native approval, which renders the same
  record, remains the effective gate. Tally numbers the voucher
  (`require_native_numbering`). The rendered review must fit a native message
  box (`admit_fresh_saved_voucher`: at most 1,600 characters, 24 lines, and
  100 characters per line).
- **Nothing else answers the approval (#583).** Unit tests of this crate drive
  the whole post, through the `post_import` tool call to the simulated POST,
  by scripting the approval with `approved_import::test_seam`. It is compiled
  only under bare `#[cfg(test)]`; in every other build the approval is
  exactly the native dialog (`use confirm as approve`). No feature,
  environment variable or runtime flag enables it. `tests/approval_seam_gate.rs`
  holds the source, build configuration and workflows to that, and
  `scripts/check-no-test-seam.mjs` fails a build whose shipped executable
  holds the seam's marker. It runs as Tauri's `beforeBundleCommand`, in
  `package-mcpb.mjs`, and as required CI steps, with positive controls on the
  debug and release unit-test executables.
- **What it cannot send.** Masters, voucher types other than Journal, Payment,
  Receipt and Contra, multi-voucher batches, alters and deletes. No
  caller-supplied XML reaches the gateway: the request is re-rendered from the
  saved batch (`render_native_voucher_xml`, `agent_import.rs`).
- **How the write is aimed.** The import request names the company only by
  name, through `SVCURRENTCOMPANY` (`render_import_envelope`,
  `agent_import.rs`). It carries no GUID. TALLY_PROTOCOL_REFERENCE §9.11d
  records that a mismatched name was observed to post into whichever company
  Tally had loaded, reporting CREATED=1 and no error. On this path, what stands
  in for aiming is the in-queue recheck: the GUID-bound company identity,
  last checked immediately before the POST, and, earlier in the same queued
  operation, a unique name among the loaded companies, trimmed and compared
  ignoring ASCII case (`post_approved_import`; `require_unique_company_scope`,
  `tally/approved_import.rs`). That recheck, plus the quiet-company assumption,
  is what keeps the write aimed. A change in the gap between the recheck and
  the POST is not prevented: for example, the company being renamed so that
  the request's name no longer matches it. (§9.11d: a name that did not match
  the loaded company was observed to post into it, while a named company that
  exists but is not loaded was observed to fail closed.) The
  GUID-bound readback would then report the voucher as not found, giving
  `reconciliation_required`. The misdirected voucher would stay in the other
  company, with no rollback, and reconciliation reads only the intended company.
  Nor can Bridge delete it over XML: see row 14 and #579.
- **Gates.** For the CLI, `BRIDGE_AGENT_ENABLE_WRITES=true`, otherwise
  `import_posting_disabled` (`agent.rs`) and the tool is absent from discovery
  (`agent_catalog.rs`). In the MCPB, the "Allow Journal posting" setting (renamed "Allow voucher posting (Journal, Payment, Receipt, Contra)" by the 2026-09-22 amendment)
  defaulted to **on** from #236 and defaults to **off** since #577
  (`packaging/mcpb/manifest.json`, `enable_writes`). The
  desktop service sets `writes_enabled: true` unconditionally
  (`agent_desktop_journal.rs`, `Settings::for_desktop_journal`). The
  environment flag gates only the MCP tool. A server run with
  `BRIDGE_AGENT_ENABLE_IMPORT=true` and writes off still builds Journals
  (`agent.rs`), and the MCPB sets that flag unconditionally
  (`packaging/mcpb/manifest.json`, `mcp_config.env`) and no data directory.
  So a default MCPB install prepares Journals, with posting off, into the
  default data directory, which the desktop also uses when its own environment
  names no other; the desktop can post them. Default-off removes the model's
  `post_import`, not posting: the desktop's pick-then-post flow and the
  native approval (the enforced gate) still stand between it and Tally. An install from before #577 may still have "on" saved
  (`docs/agent/INSTALL.md`).
- **A separate, non-shipping dispatch path.** `TallyRuntime::post_lab_import`
  (`tally/runtime.rs`) posts XML without approval or the journal. It is
  compiled only with the `lab-writes` Cargo feature, which no CI workflow
  enables (`tests/lab_writes_ci_gate.rs`). At runtime it also requires
  `BRIDGE_LAB_WRITES=1`, port 9001 and a matching target and deny-list GUID
  set, but not `BRIDGE_AGENT_ENABLE_WRITES`. It posts masters and
  multi-voucher batches. The CI gate test scans only `.github/workflows`. The
  lab path is outside this amendment's map. It is noted so that "one
  dispatching path" is not read as "one dispatch call site".

### Clause-by-clause map

"Met" means the code enforces the clause on the path above. "Partly" and
"Not met" name exactly what is missing. The map describes code. The only live
evidence is #236's own report that a desktop build posted a
human-approved synthetic Journal and reconciled it without a resend. That
report was not re-run for this amendment. #236 records live posts only through
the desktop caller. It states that interactive Windows approval and live
Gold/Education posting were untested.

| # | ADR 0004 requirement | Status | How #236 meets it, or what is missing |
|---|---|---|---|
| 1 | Observed runtime capability | Partly | `qualified_import_profile` (`agent_import.rs`) requires freshly observed TallyPrime in Licensed or Education mode, with `ProductAndMode` Supported/Observed. This is product/mode capability, not observed *write* capability: nothing requires a prior observed write, and the Capability Passport is neither read nor upgraded. |
| 2 | Explicit operator opt-in | Partly | The CLI is opt-in (above). The MCPB exposed posting to the model by default until #577 made "Allow Journal posting" (now "Allow voucher posting …") off by default (an earlier install may still have "on" saved); with it on, the per-post native approval (row 7) is the only further opt-in. Journals the MCPB prepares with posting off can still be posted from the desktop (see Gates). In the desktop, pick, review, then post (`commands.rs`: `desktop_pick_journal_for_review`, `desktop_post_reviewed_journal`) is a UI convention: the post IPC takes a batch id, digest and company GUID from the webview and is not bound to an earlier pick. The native dialog is the enforcing gate there too. |
| 3 | Synthetic company during initial validation | Not met | Nothing restricts posting to a synthetic or enrolled company. Any company that passes identity admission is eligible. |
| 4 | Backup guidance acknowledgement | Not met | The approval preview (`admit_fresh_saved_voucher`) and dialog (`tally/approved_import.rs`, `show_review`) contain no backup guidance and record no acknowledgement. |
| 5 | Small batches | Met | Exactly one voucher per call (`admit_saved_voucher_integrity`), under a review-size cap. Stricter than the ADR requires. |
| 6 | Exact validation and preview commitments | Partly | Validation is exact. Before approval, the saved batch is re-hashed and compared byte for byte with the saved file (#578), and the company tuple, the date against `BOOKSFROM`, duplicate absence, and every ledger (exact match, bound by identity through `bind_selected`) are checked. After approval, inside the endpoint queue, the company identity and unique name scope, the date against the observed mode's boundary, ledger identity and duplicate absence are checked again (`post_approved_import`, `recheck_import_admission`), and for a Payment, Receipt or Contra every leg's cash/bank classification, both before approval and in the queue (2026-09-22 amendment); `BOOKSFROM` is not. The preview is rendered from the same saved batch, and `ApprovedImport` carries the XML rendered before approval (the operator sees the preview, not the XML). But the preview text is not committed: no digest of what the operator saw is persisted. |
| 7 | Approval evidence | Partly | Approval runs in a separate native-dialog subprocess, so the model never supplies an approval boolean (`approved_import.rs`: `confirm`, `run_confirmation`). Decline and a 120 s timeout both refuse. On Windows only `IDYES` counts, but interactive Windows approval is untested (#236). No approval record is written: the durable intent (row 8) implies approval but carries no approval time or preview digest. |
| 8 | Durable idempotency reservation | Met, by a different mechanism | Before the POST, `StatusRecord::dispatch_native` is appended and `sync_data`-flushed to `agent-import-ledger.jsonl`, under the admission lock, after a check that the batch was not already dispatched or changed (`post_import_checked`, the `before_dispatch` closure). A cross-process lease covers intent, the POST, the response append and readback (`dispatch_lease::acquire`). A dispatched batch is only ever reconciled, never re-sent. This is not the `tally_import_idempotency_state` or outbox tables of migration `0003_tally_safe_writes.sql`, which `post_import` does not use. |
| 9 | Parser-derived Tally counters | Met | `parse_import_outcome` (`bridge-tally-protocol`, `import_outcome.rs`). A clean response requires every counter to be reported, and exactly created 1, altered 0, deleted 0, with zero ignored, errors, cancelled, exceptions and line errors (`is_clean_success_for(1, 0, 0)`), and an application status other than failure (`import_outcome_is_clean`). |
| 10 | Strict company-bound read-after-write verification | Met, for detection | Readback is mandatory (`verify_import_after_current_dispatch`). It is bound to the verified company GUID and compares accounting entries, not the response. `posted_verified` requires both a clean response and a verified readback (`finalize_current_dispatch`). It detects a write that did not land in the intended company, but it cannot prevent one or find where it went (see "How the write is aimed"). |
| 11 | First-write canary rule: one sealed synthetic ledger-canary candidate, fixture enrollment with disposable-company and backup acknowledgements, no Passport upgrade | Not met | The first dispatched write was a Journal, not a ledger canary. `post_import` does not consult the write-fixture enrollment (`commands.rs`, `enroll_tally_write_fixture`), which exists but gates nothing on this path. The one part that holds: no Passport upgrade. |
| 12 | Lifecycle, outcome-unknown after bytes may have been sent, no automatic retry, no rollback | Met, with different terminal names | draft (`build_import_xml`) → validate → preview → approve (native) → arm (lease plus durable intent) → send (`ReadRetryPolicy::SINGLE_ATTEMPT`) → parse → verify. Terminal states are `posted_verified` and `reconciliation_required`. The ADR's separate `partial` and `failed` verdicts are folded into `reconciliation_required`. An error from `post_approved_import` maps to `import_dispatch_outcome_unknown`, except its fourteen named admission refusals (`import_bank_classification_changed` and `import_post_admission_inconsistent` since the 2026-09-22 amendment, `post_company_scope_changed` and `post_company_scope_unconfirmed` since #574, `import_multi_currency_unsupported` and `import_base_currency_undetermined` since #551, `post_masters_moved` and `post_masters_unconfirmed` since #239, `post_catalogue_unreadable` since #641, for a queue catalogue re-read that does not parse as the target's catalogue, whose `cause` names why, and `group_export_invalid` since #717, for a queue group re-read that does not parse, whose `cause` is the one the read before approval names) and an in-queue unobserved product/mode boundary, which keep their own codes (the latter `financial_read_profile_unqualified`), including errors *before* the intent was recorded, where `attempt_recorded: false` says no attempt exists. Since #656, any other failure inside the queue before the intent (a queue read failing in transport, by pair drift, at an identity bracket or to parse) refuses as `post_queue_read_failed`, with a data-free `cause` where the failure has a typed one. The runtime's general overrides still apply there, whatever the operation: a withdrawn call reports `request_cancelled`, and an Education switch caught at a read's identity bracket reports `window_part_boundary_unsupported_in_education`. The catch-all still covers failures outside that block: queue admission before its first read (lease, session, circuit, deadline), a cancellation part-way through it, and everything from `before_dispatch` onwards. The checks made under the admission lock just before the intent append keep their own codes since #711 (`import_batch_not_found`, `import_already_attempted`, `import_remote_id_reused`, `import_batch_changed`), as does an approval that can no longer be spent there since #791 (`import_approval_revoked`): nothing is recorded or sent by this post, and `attempt_recorded` reports what the journal shows. The catch-all still covers taking that lock and reading the journal under it, the append itself and everything after it. Failures earlier in `post_import_checked` keep their own codes. Current-dispatch responses carry `resent: false` and `automatic_retry: false`, and reconciliation responses carry `resent: false`, but `reconciliation_failure_payload` carries neither. No code path re-sends. No rollback is claimed or implemented. Since #582 the REMOTEID a native post sends is recorded in its dispatch intent before the POST, and a native post was deleted by it live (#579); no delete or rollback tool exists. |
| 13 | Domain-separated commitments for wire bytes, canonical intended state, import response, canonical readback state and identity coverage; opaque parser-derived evidence; line-error text reduced to digests | Partly | Import response: the parser's `response_sha256` is domain-separated (`import_outcome.rs`, `domain_sha256`) and is stored inside `DispatchResponse.outcome`. Line errors: kept only as ordered domain-separated digests. Counts: parser-derived only, so a caller cannot assert them. Wire bytes: `DispatchResponse.request_sha256` and `response_sha256` are plain SHA-256 over the wire bytes, not domain-separated. Canonical intended state: the batch `sha256` is plain SHA-256 over the rendered XML. Canonical readback state: only the read evidence's response digest exists. Identity coverage: no commitment exists. |
| 14 | First qualification profile: ledger-only create/alter, RemoteID-bound preflight, exact counts, a lost response stays unknown | Partly | Counts are exact (row 9). A lost response is never promoted: without a clean persisted response, the verdict stays `reconciliation_required` even if a later readback matches (`finalize_current_dispatch`, `finalize_previous_attempt_reconciliation`). But the profile is voucher create, not ledger create/alter. The preflight is not RemoteID-bound. Each post uses a fresh random `REMOTEID`, so a public file's REMOTEID is never upserted. Since the batch-posting slice A, a `REMOTEID` any dispatch intent in the journal already records is refused as `import_remote_id_reused` before any Tally request, and checked again under the exclusive lock as the intent is written (a match there refuses as `import_remote_id_reused` since #711, with no intent and no POST). The journal does not refuse, on read, an intent whose `REMOTEID` another already carries: that would stop every batch's reads. So a journal already holding one twice (a restored old copy, say) reads normally and is not flagged; what holds is that no post sends a `REMOTEID` any intent records. Tally may have seen any recorded `REMOTEID`, and a resend with a changed payload undoes a person's cancel or delete (protocol reference §9.3). A voucher export shows Tally's own GUID as its REMOTEID, and a delete keyed by that GUID was refused in the 21-Sep lab ("Voucher does not exist!"). Since #582 the REMOTEID each native post sends is recorded in its dispatch intent before the POST, and a delete by that recorded value succeeded live (#579); no delete or locate tool exists yet. Duplicate absence is checked over the batch's date window, by the `[BRIDGE:…]` narration attribution or an accounting-content fingerprint (`verify_batch`, `agent_import_verification.rs`), twice, inside the queue (`require_absent_verification_result`, `recheck_import_admission`). |
| 15 | Consequences: private deterministic import bytes; no public byte getter or transport adapter; every prepared write ineligible for dispatch; legacy caller-attested rows not promotable; a migration persisting opaque derived commitments before runtime wiring; fixture enrollment local, explicit, revocable | Partly | No public byte getter: `ApprovedImport::xml` is `pub(super)`. Not met: the native bytes are not deterministic (a fresh `Uuid::new_v4()` REMOTEID on each render). Not met: a transport path now exists (`post_approved_import`), and prepared Journals are eligible for dispatch. Not met: no migration preceded the wiring; the path persists to the JSONL journal instead. Not applicable: the legacy caller-attested rows belong to the database contract, which this path does not use. Fixture enrollment is local, explicit and revocable, and irrelevant to this path (row 11). |

### Follow-ups

These are listed and not fixed here. Each one is either an implementation change,
or an owner decision to amend the requirement instead:

1. **Synthetic-company and canary gating (rows 3, 11).** Either require write-fixture enrollment
   before `post_import`, or record the owner's decision that a single
   human-approved Journal on any admitted company replaces the canary rule.
2. **Backup acknowledgement (row 4)** in the approval, or an explicit decision to drop it.
3. **Default opt-in (row 2).** The MCPB default of posting-on is resolved by #577 (now off),
   except for installs that kept an earlier saved "on". Open: the desktop service's
   unconditional `writes_enabled`, which can post Journals the MCPB prepared with posting off.
4. **Approval evidence (rows 6, 7).** Persist a digest of the approved preview, and the approval
   time, in the dispatch intent.
5. **Domain-separated request and response commitments (row 13).**
6. **Three write contracts.** The migration `0003` outbox and idempotency tables, the
   canary reservation, dispatch-attempt and final-verdict tables of migrations `0015`–`0021`
   (`db/tally_mirror.rs`), and the `agent-import-ledger.jsonl` journal. Only the journal has a
   runtime caller. Decide which one is authoritative, and retire or connect the others.
7. **Write capability in the Passport (row 1).** Decide whether a verified post
   should become observed write evidence, and under what rule.
8. **Concurrent external changes, #239. Narrowed.** Tally offers no conditional import, so
   nothing binds a POST to the masters Bridge checked. The queue now reads every loaded company's
   change marks as its binding reads begin (just before the catalogue re-read) and compares the
   target's master mark (`ALTMSTID`) with the one in the aim snapshot sent last before the POST;
   any change refuses with `post_masters_moved` (`post_masters_unconfirmed` if the comparison
   cannot be made), before the intent. A ledger rename made through the gateway is measured to move
   `ALTMSTID` by 1, and master creates move it too (TALLY_PROTOCOL_REFERENCE §11c.5, §10); only
   gateway changes were measured (§11c.4). Not yet measured: a regroup, an edit made in Tally's own
   screens (the likelier concurrent writer), and whether posting a voucher moves `ALTMSTID` (if it
   does, a busy book refuses more often; it fails closed). A change that does not move the mark is
   not caught. Any change that does refuses, including unrelated ones; the operator re-runs. During the approval wait, a ledger renamed and a new one created under
   its old name is refused by the catalogue binding's (name, GUID) check (`import_masters_changed`).
   Between build and post, which can be days apart, the build now records each named ledger with
   the GUID its catalogue read bound it to. Before approval, a post refuses any ledger that now
   resolves to another GUID (`import_masters_changed_since_build`, naming it). A batch saved
   before that record existed refuses with `import_batch_predates_ledger_binding` and must be
   rebuilt. The record is trusted local state, as in follow-up 10: its hash covers the rendered
   file, not these GUIDs. Open: the one round trip from the aim snapshot to the POST, and a check
   after the POST.
9. **Aiming by name (scope, "How the write is aimed"), #574. Narrowed.** Live on licensed 7.1
   Silver (2026-09-21/22): a name matching no loaded company fails closed over the gateway
   whichever company is selected; a rename between build and post is refused at admission; a
   post into another company lacking one of the voucher's ledgers is rejected by Tally. What
   remains is a rename, in the moments before the POST, to another loaded company's exact name
   whose ledgers all overlap. The queue now reads every loaded company's change marks as its last
   Tally request before the POST and refuses unless exactly one company has the target's GUID
   and name and none shares its name (`post_company_scope_changed`, or
   `post_company_scope_unconfirmed` if that read fails). It reads them again right after the
   POST, once its response is journaled, and reports which companies' voucher marks moved
   (`post_location` in the result), with the target's own step against Tally's `CREATED`
   (`target_voucher_step`). Each gateway create, alter or cancel moves the mark by one (protocol
   reference §11c.5), so a larger step means either the post itself altered or cancelled
   vouchers, or another voucher in the target changed meanwhile; the step alone cannot say
   which. Screen edits were also seen to move it by one each (PARTIAL), and a change
   that leaves the mark still is not ruled out, so a matching step does not prove that nothing
   else changed. A multi-voucher import stepped by its count in lab scripts (PARTIAL); through
   Bridge's own post path that is UNVERIFIED, and a batch post must measure it before gating on
   it. The step is reported only: a single post stands on its readback. This flags a possibly
   misdirected post; concurrent writers on a shared book can make it ambiguous, and it cannot prevent one, because Tally cannot bind an
   import to a GUID. The interval between that snapshot and the POST, local work only, stays accepted with
   #239. Locating the voucher inside another company, and removing it, are not built.
10. **Batch record integrity (scope, "What it can send"), #575. Resolved by #578.** The post
    path now re-checks the saved file's bytes, as the desktop review already did, and both use one
    reader. Residual, by decision: the record and file are trusted local state against anyone who
    can write both in the private data directory; the native approval is the gate for that case.
11. **Test the call-time refusal.** No test references `import_posting_disabled` (`agent.rs`); only
    the tool's absence from discovery is tested.
12. **Record the REMOTEID a native post sends, #579. Resolved by #582.** The dispatch intent
    records it before the POST, and a native post was deleted by it live (#579, 2026-09-22).
    Open: a delete or locate tool built on it, under the requirements listed on #579.
13. **Multi-currency books, #551.** A voucher's amounts are plain base-currency figures, and a
    foreign-currency ledger's balance can read as a plain amount too, so only the ledger's own
    currency tells them apart. Among several Currency masters, only the outstandings reads
    identify the base (bridge#551, TALLY_PROTOCOL_REFERENCE §9.10a.2); the write path does not
    compare a leg's currency with it yet. Until it does, every post reads the company's Currency
    masters before approval and again inside the queue, in the same identity brackets as the catalogue, and is
    refused unless there is exactly one (`import_multi_currency_unsupported`, naming the masters
    seen, or `import_base_currency_undetermined` when the response parses to no master or does not
    parse, as a master without a name does not). A transport failure of that read is reported as any failed read
    before approval, and as `post_queue_read_failed` with `attempt_recorded: false` in the
    queue (#656), as for the catalogue re-read. That a one-master book holds every ledger in the base is
    an inference, not a measurement (TALLY_PROTOCOL_REFERENCE §8.2d).
    A master added during the few requests between the queue's read and the POST is not caught,
    the same window as the catalogue re-read (#239). This refuses every post
    into a book that defines a second currency, even one whose legs are all in the base. When
    the write path uses the identified base, each leg's own `CURRENCYNAME` is compared with it instead.
14. **Amendments can overwrite work done in Tally, #239. Named, not closed.** An amendment
    (`amends_batch_id`) is refused unless each voucher it alters is still in the book as a build of
    that batch wrote it, in these fields only: the date, a bank voucher's effective date when Tally
    returns one, the voucher type, the voucher number when the batch set one, each entry's ledger,
    amount and side, and the narration. Two gaps remain, and each needs its own fix:
    - **Fields not compared.** A voucher's reference, its bill-wise and cost-centre allocations,
      and which ledger Tally records as its party are not fetched by the verification read, so the
      field comparison cannot see an edit to them. An in-place alteration replaces a
      voucher's entries rather than merging them (TALLY_PROTOCOL_REFERENCE §9.3, measured over the
      gateway), and an amendment's entries carry no allocations, so allocations made in Tally,
      including those Bridge's own build advice asks for after a Payment or Receipt import, are
      expected to be lost. That loss, and what happens to a reference, are not measured directly.
      Since #239's baseline change, the build also admits a voucher only when its `ALTERID` equals
      one Bridge recorded the first time it verified a build the book still matches; otherwise it
      refuses (`voucher_altered_since_verified`, or `voucher_never_verified` when no such build has
      a record). A voucher's `ALTERID` advances on every alteration (§9.3, measured over the
      gateway; an edit in Tally's own screens is not yet measured), so equality with any recorded
      value means nothing has altered the voucher since that reading. The values are kept beside the
      proof, not in the journal: in `<batch>.baseline.json` as builds before #911 wrote it (rewritten
      as it grew), and from #911 in one file per verification that adds a voucher,
      `<batch>.baseline.<n>.json`, written once and never replaced. That catches an edit to any field made after the first verification,
      on the premise above. An edit made between the import and the first
      verification becomes part of the baseline and is not caught; every amendable batch was imported
      by hand, since a batch Bridge posted cannot be amended, so the build asks for a verify right
      after each import. Batches verified before this change have no record; their first
      verification after it becomes the baseline, so an edit made before that is not caught for
      them. The refusal says so plainly: "Verifying now records this voucher exactly as it stands
      in Tally, including any changes made since Bridge built it. Check the voucher in Tally first;
      if someone has edited it, correct it there instead of amending." Any alteration refuses,
      including one that is not a content edit (for example a bank
      reconciliation date set in Tally), which is the right answer when the amendment would
      replace the voucher's entries.
    - **The build-to-import window.** The comparison runs when the amendment is built. The import
      is done by hand through Tally's Import menu, and Bridge refuses to post an amendment
      (`import_post_amendment_requires_file_import`), so nothing re-checks the vouchers just before
      they are altered, and an edit made in between is overwritten without warning. Only Bridge
      posting amendments through its own queue, with the check run last before the POST as the
      #574 aim check is, would close this gap.

    The build result's warnings and next step, the tool and schema descriptions, and the refusal
    text state these limits and ask for a prompt import.

## Amendment — owner decision, 2026-09-22: direct voucher posting

`post_import` may post one voucher of type **Journal, Payment, Receipt or
Contra**. Every safeguard of the Journal path applies to all four:

- one voucher per call from a batch Bridge built and saved;
- the saved file equal to its record (#578);
- a fresh REMOTEID recorded before dispatch (#582);
- complete identity admission, re-checked inside the endpoint queue
  before the POST (#574);
- each named ledger bound to its GUID at build, and checked against it before
  approval (#239, follow-up 8);
- a company with exactly one Currency master, checked before approval and
  again inside the queue (#551, follow-up 13);
- the native approval;
- a single attempt with no automatic retry;
- parser-derived counters requiring a clean create;
- mandatory company-bound readback.

Payment, Receipt and Contra add one requirement. Every leg is classified
again, by the same rules the build applied (`cash_bank_refusals`), from the
ledgers' current parents (the catalogue) and a fresh read of the group
collection: once before approval, and again after approval inside the queued
operation, where the group read sits beside the catalogue read in the same
identity brackets. If any leg no longer passes, the post is refused before any
import request and before any dispatch intent (`import_bank_classification_changed`).
A ledger a bank cash answer named as cash in hand is held to Cash-in-Hand at the
same two points (#815): the build records it, and a post refuses with
`cash_ledger_not_cash_in_hand` when it now reaches another group. The leg
classification cannot see that move, since a cash ledger under Bank Accounts is
still money. A batch recorded before that record refuses with
`import_batch_predates_cash_ledger_record` and is rebuilt. Releases 0.3.0 to 0.4.2 read
neither this record nor the bill-wise one below, so a saved batch carries its ledger binding
under a name they do not read: they refuse it as `import_batch_predates_ledger_binding`
rather than post it without those checks. A ledger that keeps bills in
Tally takes an entry On Account when the voucher carries no bill allocation, so the build
reads each named ledger's `ISBILLWISEON` from the same ledger-list read it already makes
(§12a.15), refuses until each such party is approved, and records the approved ledgers; the
post and the queue admission read the flag again and refuse `import_bill_wise_changed` for a
ledger that became bill-wise since. A batch recorded before that record refuses with
`import_batch_predates_bill_wise_record` and is rebuilt. The approval is the assistant's word,
not proof that a person said yes; the native dialog marks each approved ledger On Account from
the build's record, on each of its entries for one voucher and on its totals line for a batch
(#1234).
The queued re-read is not the last request before the POST: the mode and
company re-admission and the two duplicate-absence reads follow it, because
duplicate absence stays the final source check. A regroup in Tally during those
few requests is refused by the master-mark comparison (follow-up 8) only if it moves
the company's `ALTMSTID`, which is not yet measured for a regroup; until then the
quiet-company assumption (#239) covers it. The catalogue binding alone cannot see
this: it compares each ledger's name and GUID, not its parent, so a ledger or a
group re-parented after the build would otherwise go unnoticed. A bank voucher
without its group read, or a Journal with one, is refused as a wiring fault
(`import_post_admission_inconsistent`).

The approval names the type in its first line ("Create ONE Payment in …") and
states which side had to be bank or cash; the dialog title and button are
type-neutral ("approve one voucher", "Post voucher", or for a batch "approve N
vouchers", "Post N vouchers", #746). A Journal carries no group
request. (Since #574 every post, a Journal included, also reads the all-company
marks last before the POST and once after it.) Known limits of the preview:
it lists entries in the saved order while the posted XML puts debits first, and
it does not say which ledger becomes the voucher's party (Tally 7.1 Silver read
the bank ledger back as the party on Payments and Receipts). A preview over the
native dialog's caps (24 lines, 100 characters a line, 1,600 characters) is
refused, never truncated; a bank voucher fits at most six entries (an
assistant's post, the route a bank voucher takes, adds the line that says when
the post happens; a text without that line fits seven). A
one-voucher preview ends with the voucher's own text: its reference and
narration are the last two lines, quoted as they will be posted, under a line
saying that no line below it is an entry, a total or an instruction, and the
date line says where they are. Every line the preview writes itself, the On
Account sentence included, stands above that line (#1234).

**Scope.** The desktop "post a saved Journal" action stays Journal-only
(`PostScope::JournalOnly`); its review was built for one Journal. So does the
desktop's read-only reconcile: a bank voucher posted from the assistant is
reconciled through `verify_import` or `post_import` with the same batch, not from
the desktop. The MCPB
setting is renamed "Allow voucher posting (Journal, Payment, Receipt, Contra)"
and still defaults to **off** (#577). Rows 5, 6, 12 and 14 of the map extend to
these types unchanged; row 6 gains the classification recheck, and row 12's
named admission refusals gain `import_bank_classification_changed` and
`import_post_admission_inconsistent`. On the MCP path, a batch that is not one
voucher of an admitted type now refuses with `import_post_requires_one_voucher`
(previously `import_post_requires_one_journal`, which the desktop path keeps). Follow-ups
1–4 stay accepted-open for all four types, as for Journals. Multi-entry bank
vouchers post only as far as the build admits them (bridge#466).

**Evidence.** Bridge-built two-entry Payment, Receipt and Contra files imported
and verified on licensed TallyPrime 7.1 Gold, 2026-09-10 (reference §9.13); a
Bridge-built three-entry Receipt imported over the gateway and verified
`posted_verified` on licensed 7.1 Silver, 2026-09-22 (#466); a native Journal
post deleted by its recorded REMOTEID (#579, live-qualification comment of
2026-09-22). A native post of a Payment,
Receipt or Contra has not yet been observed live. The code paths are covered by
simulator tests through the `post_import` tool call (#583 seam), including a
ledger and, separately, a group re-parented after approval.

## Amendment — 2026-09-23: concurrent writers (#239)

**What Tally gives.** Tally offers no conditional import: nothing binds a POST to a company GUID,
a master version or an `ALTERID`, and an import succeeds by name whatever changed since Bridge
looked. There is no mutation-time witness either. So Bridge can only check before the POST and
read after it; it cannot make the two atomic. One policy applies to every edition, with no Gold
gate (owner, 21 September): what matters is a second writer, not the licence tier.

**What is checked before anything is sent.** Each refuses before a post's dispatch intent, or, for an amendment, before its file is written, so nothing is posted.

| Check | Where | Evidence |
|---|---|---|
| The target company's name and GUID, on the last Tally request before the POST | #574, follow-up 9 | Simulator; the multi-company snapshot shape is measured, not captured |
| Exactly one Currency master | #613, follow-up 13 | Live: refused before approval on a two-master synthetic book, 2026-09-23 (#613, live-check comment) |
| The target's master AlterID (`ALTMSTID`) unchanged from just before the queue's catalogue re-read to that last request | #615, follow-up 8 | Simulator; a gateway rename moves `ALTMSTID` (§11c.5) |
| Each ledger's (name, GUID) unchanged from the build to the approval and on to the queue | #616, follow-up 8 | Simulator |
| An amendment's vouchers unchanged since Bridge first verified them, by fields and by `ALTERID`, at build (Bridge never posts an amendment) | #620, follow-up 14 | Simulator; a gateway alteration advances `ALTERID` (§9.3) |

**The windows that remain.** Each is named where an operator or an agent reads it.
1. **Between the last request and the POST.** Only local work runs there: the recheck and the
   durable intent. A change made in Tally during it is not seen before the POST; it is posted
   into, and only the check after the POST (window 3) can flag it.
2. **Changes that do not move `ALTMSTID`.** Whether a regroup, or any edit made in Tally's own
   screens rather than through the gateway, moves the mark is not yet measured. A change that
   does not move it is missed by the check in the queue and by the check after the POST alike.
3. **After the POST.** The readback compares ledgers by name, and a posted voucher's ledger lines
   carry no ledger GUID over XML (measured 2026-09-23 on one Bridge-posted Journal; #239,
   R-2 measurement comment). So unless the target's master mark is proven unmoved between the
   snapshot before the POST and the one after it, Bridge reads the ledger catalogue again and
   resolves each approved ledger's name to its GUID. A snapshot after the POST that is lost or
   unreadable counts as moved. If any ledger now resolves to another GUID
   (`masters_after_post: posted_under_changed_masters`), or the check cannot be completed
   (`masters_after_post_unconfirmed`), the post is reported `reconciliation_required`, never
   `posted_verified`, with a message that the voucher is in Tally, should be reviewed there, and
   must not be rebuilt. A ledger renamed after the POST that gives its approved name back to its
   approved GUID cannot be told apart from the one posted into. The per-voucher readback status
   still says what the name comparison found; only the dispatch state carries the doubt.

   The check is recorded beside the batch's proof, without a lock. Before anything can be sent,
   Bridge marks it pending, and refuses the post if that mark cannot be written; after the POST,
   the verdict replaces the mark. A crash before the check finishes, a check that cannot read the
   catalogue, a cancel during it, or a verdict that cannot be written leaves the mark pending,
   which every later `verify_import` or reconcile reads as a doubt. The first of those that finds
   the vouchers finishes the check, against the ledger identities recorded at build (#616), and
   records its verdict. An observed doubt is also written to a file of its own that nothing
   removes or replaces (since #911 it is written once, and a later doubt keeps the first), and readers check it first, so no later verdict,
   pending mark or race can clear it; the readback compares by name and cannot either. It stays even after a person
   corrects the voucher in Tally, and while a doubt or a pending check stands, the batch is no
   baseline for an amendment. Bridge has no way to clear it. Follow-up: an explicit operator
   acknowledgement through the native approval dialog ("I reviewed this voucher in Tally"),
   recording who and when; never a clear an agent can call.

   Residuals, stated rather than closed:
   - a pending check finished by a later readback, perhaps days later, lengthens the window in
     which a ledger swapped away and back goes unseen;
   - the pending mark is renamed into place and its directory is not synced, so a power loss can
     lose it, or a doubt file, whose dispatch record survived, and the batch then reads as its
     remaining record says, or as one dispatched before these records existed. Since #911 a
     finished verdict and a doubt are each written once to a file of their own
     (`*.masters_verdict.json`, `*.masters_doubt.json`), never replaced, and their directory is
     synced; one that exists but cannot be read reads as pending, which no later verdict can then
     replace, so the batch stays in doubt;
   - an observed doubt whose own file cannot be written is kept only by its verdict record (the
     verdict file, or for a batch step the check record, which a losing second post's pending
     mark can replace); if neither can be written, it is lost. Since #722 that verdict carries `doubt_record: unavailable`, and a
     doubt the check record holds without its own file, whether its write failed or the file
     was lost later, is refused for review (`ack_doubt_record_unavailable`) and reported as
     `operator_review` `doubt_record_unavailable`, rather than read as no doubt. A review
     recorded before the file went missing reads stale. The same holds for a batch's step
     doubt;
   - a readback that finishes a pending check in the moment before the post's own doubt lands
     can report the voucher verified once; later reads, and any amendment, see the doubt;
   - a losing second post's pending mark can replace a clean verdict, and the next readback
     then checks the catalogue as it is by then, so a legitimate later change to a ledger can
     leave a lasting doubt: failing closed, but with no clear in Bridge.
4. **Amendments.** The window between the build and the manual import, and between that import
   and its first verification (follow-up 14). Only amendments posted through Bridge's own queue
   would close the first.
5. **Not #239.** A person entering the same transaction by hand is a duplicate, not a concurrent
   change. It belongs to the bank-voucher duplicate check.

The approval still asks the operator to pause other edits and imports while Bridge posts. With
these checks, that is advice that narrows the remaining windows, not the only guard.

**Evidence since the 2026-09-22 amendment.** Native posts through `post_import` of a Payment, a
Receipt, a Contra and a three-entry Receipt were observed live on licensed 7.1 Silver on
2026-09-22, on a synthetic company: each read back `posted_verified` and was then deleted by its
recorded REMOTEID (#600, live-qualification comment of 2026-09-22). That three-entry Receipt was a
native post, distinct from #466's gateway import of a Bridge-built file. The 2026-09-22
amendment's "has not yet been observed live" is superseded.

## Amendment — batch posting behind a setting (slice D2a)

`post_import` may post **2 to 50** vouchers of one saved batch in one import, only when
`BRIDGE_AGENT_ENABLE_BATCH_POST` is on together with posting. It is off by default. It is a
command-line setting, not in the MCPB extension, until a live batch post through Bridge is
proved. With it off, a batch of more than one voucher refuses exactly as before
(`import_post_requires_one_voucher`), and a one-voucher post is unchanged.

**Every safeguard applies to every voucher** (slice D1):
- each voucher's type and numbering;
- the Education boundary for every voucher's date, in the queue too;
- duplicate absence for all N, before approval and in the queue;
- one fresh REMOTEID per voucher, none recorded by any earlier intent, all recorded in the one
  dispatch intent before the POST;
- the ledger bindings, currency and bank-leg checks for every leg.

**Approval.** The native dialog (#665's nonce-bound approval) shows a summary, not a listing:
- the company;
- the count by type and the date range;
- one line per ledger with its debit and credit totals and entry count;
- the grand totals;
- "Money in by Receipt vouchers" (their debits) and "Money out by Payment vouchers" (their
  credits), which the types fix as cash/bank and the build and queue enforce;
- a Contra line (net zero) and, when the batch holds Journals, a line saying they may also move
  cash/bank ledgers;
- the standing cautions.

It says, in a line of its own, that each voucher's date, narration and reference are not shown. Its budget is its own: 40 lines, 3,200 characters, 100
a line, and 7,000 UTF-8 bytes (under the dialog's 8,000). A summary over it is refused
(`import_review_too_large`), never cut, and the caller posts the batch in parts. A 40-line native
dialog has not yet been shown on Windows or macOS (UNVERIFIED).

**Rejected for the money line:**
- A single "bank and cash net" computed after the catalogue and group reads was rejected. It
  moves the approval text after two more reads, and adds a group read to Journal-only batches.
  The per-type lines are exact without it.
- Recording the build's leg classification in the batch record was rejected. It adds a field
  whose only reader is this line.

**Outcome.** A batch is `posted_verified` only when all of these hold:
- Tally reports CREATED N and nothing else;
- the readback verifies all N;
- the masters check finds no doubt;
- the target's voucher mark moved by exactly CREATED (`post_location.target_voucher_step`).

The step verdict is durable. It is recorded in the check record beside the masters verdict,
pending before the POST (the finished masters verdict has its own file since #911), and an
observed mismatch is kept in its own file that nothing removes. So no later readback can lose it, and neither verdict masks the other. If that file
cannot be written, the check record keeps the mismatch, marked `doubt_record: unavailable`
(#722), as it keeps a masters doubt (the residuals of #239's window 3, "After the POST").
A mismatch, or a verdict never recorded, reads `reconciliation_required` with
`batch_step_unconfirmed`. On a multi-user book a person's edit during the post trips it; that
is accepted, and loud.

**Limits.**
- A doubted batch keeps its verdict. `acknowledge_post_review` records that a person reviewed
  it, as for one voucher: from a summary of every voucher as read back, for one named doubt
  (`masters` or `batch_step`), binding that doubt's bytes and every voucher's GUID, MASTERID,
  ALTERID and fingerprint, in batch order. A review covers only the doubt it names; an edit to
  any voucher makes it stale, naming the voucher. It is refused unless all N read back. It
  changes no verdict. A `batch_step` review attests only the batch's own vouchers, not that
  nothing else in the company changed. A doubt the check record holds without its own file
  has no bytes to bind, so it is refused (`ack_doubt_record_unavailable`, #722), named or
  chosen, before any recorded review of that kind is consulted: such a review is stale, so the
  refusal names the missing file rather than the stale record (#770). A pending check that the
  review's own read finishes as a doubt whose file cannot be written is refused the same way,
  after that read. Beside another observed doubt it needs a name, unless both are held only by
  the check record, which is refused with no name needed; a check still pending or unreadable
  is not observed when the choice is made, which is before the read that can finish it. If a read
  finishes it as a second doubt (the review's first read, its read after the dialog, or another
  call's read while the dialog is open), an unnamed review is refused as ambiguous after its
  next read, before any record (#756); a named one goes on, since the person named what they review.
- The N-voucher step is PARTIAL on raw-gateway lab scripts (protocol reference §11c.5).
  Through Bridge's own post path it is UNVERIFIED until the lab proof (slice D3).
- The desktop stays single-voucher `JournalOnly`.

**Rollback.**
- **Back to slice D1's build.** D1 never records a step verdict. A batch whose step this build
  recorded as matched stays clean there. Any other batch reads `reconciliation_required`
  (`batch_step_unconfirmed`), and so does one whose verdict D1's own pending re-check drops.
- **Back to a build older than D1.** That build refuses a journal holding a batch's dispatch
  intent (`deny_unknown_fields`), so every import and verification stops, loudly, until a newer
  build is back.
- **Back to a build before batch reviews (slice D2b).** That build reads a batch's
  `masters_ack.json` (version 2) as unreadable, and ignores `batch_step_ack.json`. Both are
  display-only, so no verdict changes.
- **Back to a build before #722.** That build ignores `doubt_record` and reads a doubt the check
  record holds without its own file as no doubt again (`ack_no_observed_doubt`, and
  `operator_review` null). The verdict itself is unchanged.
- Do not edit the journal or the check records by hand to get around any of these.

## Amendment — 2026-10-02: native posts carry no narration tag

- **Change:** a native post (`post_import`, and the desktop Journal post)
  writes the narration without `[BRIDGE:…]` (owner decision, 2026-09-28) and
  is attributed by the post-span basis of ADR 0018 (amended 2026-10-02). The
  hand-import file keeps the tag.
- **New journal state:** the dispatch intent records the pre-POST voucher
  mark, and one post-span verdict record per batch follows the response. An
  older connector refuses such a journal; the incompatibility starts at the
  first post made with this build.
- **Rollback:** do not downgrade after posting with this build. Before the
  first such post a downgrade is harmless.
- **New refusal:** `post_mark_unrecorded`, under the admission lock before the
  intent: nothing recorded or sent, and the approval withdrawn.
- **New readback statuses:** `bound_not_in_window` and `book_rolled_back`.
  Neither is ever absence.

## Amendment — owner decision, 2026-10-08: the order of the GST Sales slices after the invoice core

The owner approved this order on 8 Oct 2026: lab groundwork first, then each guard as a refusal inside the Sales build. Nothing below is built yet. It fixes the order of the work that follows the Sales invoice core (#1342, a draft at this change, with its own amendment "GST invoices, Sales first" and the owed list there and in its protocol-reference section), so that each slice's pull request can cite one place. A slice that is later dropped or reordered says so here with a dated note. Every safeguard of the earlier amendments applies to every slice, and no slice below switches Sales on. Where a slice names a decision still to be taken, that decision is made, and recorded, before the slice is built.

**Where the guards go.** Each check below is to be a refusal inside the Sales build, so a build that would fail it is refused on every call. It is not a separate read tool, because a check a caller must remember to run does not guard a build that skips it (AGENTS.md P2, P7).

**Before any slice: lab groundwork, no code.** None of the slices can be proven until these hold.

- A synthetic lab company whose own GSTIN Tally accepts as valid. On the 7 Oct rehearsal's company, whose invented GSTIN carries another state's code, Tally's GSTR-1 counted all four vouchers as uncertain and gave the company's registration details as invalid or not specified; the cause was not isolated, and a status check proves nothing on such a company. Its GSTIN stays within the synthetic family that test fixtures may carry.
- An unregistered buyer keyed by hand on screen, then read natively. This gives the fields Tally itself writes for such a buyer.
- A Gold edition the lab can reach, so that the Sales slices are also measured on the edition a client's book may use. The 7 Oct Sales rehearsal ran on Silver; Sales on Gold rests on hand imports only. This is a measurement prerequisite, not a release gate: the concurrent-writers amendment keeps one policy for every edition.

**Slice 1 — refuse an incomplete company registration.** The build is to read the company's own GST registration in force on the invoice date: GSTIN, registration type, state and applicable-from. It is to refuse a company whose registration is absent, starts after the invoice date, is not Regular, or any of whose fields Tally does not return. A missing field is refused, never read as Regular. More than one registration in force on the date is refused until how one is chosen is decided. This closes the owed item "the company's own registration on the invoice date is not read". Today the core reads only the company's state.

**Slice 2 — GST status and the buyer block in the Sales read-back.** The read-back is to read each posted invoice's GST status (Tally's included and uncertain flags) and the buyer and consignee fields the build wrote. `posted_verified` is to be kept only when the invoice reads as included and not uncertain, and those fields equal what the build wrote. A status that reads uncertain, or that the read does not return ("not established"), keeps the invoice out of `posted_verified` and counts as GST-uncertain for slice 4. Equality with what the build wrote does not show the names are right: the build writes the ledger name, because the mailing name is not read, and that read stays owed in the core's list. The slice's lab evidence compares a posted invoice, field by field, with a hand-keyed invoice of the same shape on the same company. Not measured: whether the status fields come back in a field-listed read, or only in a native one.

**Slice 3 — refuse a sales ledger whose GST details do not fit its leg.** The build is to read each sales ledger's GST rate, HSN/SAC and taxability as of the invoice date. It is to refuse a ledger that is not taxable, a ledger without an HSN/SAC, and a leg whose ledger rate differs from the slab the invoice's tax amounts imply. This closes the blended-bill item owed in the core's list. It is also to read the rate of each tax leg's duty ledger, where the book gives one, and refuse a tax leg whose ledger rate is not that slab's share for its head (how Tally states a duty ledger's rate is not yet measured): the core reads no duty ledger's rate either, so a bill at one rate posted to another rate's duty ledgers passes whenever its amounts fit a slab, and a bill with lines at two rates can pass on one pair of heads as a single blended rate. One decision comes first: what a sales ledger, or a duty ledger, with no rate of its own means.

**Slice 4 — a stop latch per company, with a release.** A Sales batch for a company, keyed by the company's GUID, that ends `reconciliation_required`, `posted_divergent` or GST-uncertain (as slice 2 defines it) is to block every further Sales post for that company. The latch lives in the import journal, so it survives a restart. Its release is a person's recorded act, naming the batch, in a native dialog the model cannot answer. Today's acknowledgement step (`acknowledge_post_review`) is not that release: it admits only a review of a doubt beside vouchers that all read back verified, and it unblocks nothing. Slice 4 is to extend it, or add a release beside it, for every state that latches. The release owed in the core's list is to ship in this slice: the core's number control refuses a build, on a book whose voucher mark is not 0, for a company that has sent an invoice but holds none verified posted (`invoice_number_control_unavailable`), which never clears when Tally declined that invoice or it was deleted, so a latch shipped without the release would add a second lock with no key. Decided in the slice's pull request: whether a declined or lost-answer batch also sets the latch, and whether a later verification that reaches `posted_verified` lifts the latch by itself.

**After slice 4 (listed for order only).** These come after slice 4: switching Sales on once the owed items of the core's list are done; several invoices per approval, after batch posting's own proof (#1090); invoices from a CSV file in a template ComplyEaze Bridge defines, through the proposals route, generalised beyond bank statements; and two rates on one invoice, after measurement.

**Rejected while the order was drafted:**

| Rejected | Why |
| --- | --- |
| A standalone readiness read tool | A check a caller can skip does not guard the build. |
| Creating masters through the approval window | It would ship a second dispatch path into the release (AGENTS.md P8). |
| An xlsx reader | It needs a new dependency, and the bank-statement crate already reads CSV. |
| A reader for one billing layout | One layout is not a product shape. |
| Rollback inside ComplyEaze Bridge | No compensating Tally operation is implemented and verified (Decision, above). |

**Not measured.** None of this has been measured against Tally: the native shape of a company's GST registration, the native shape of a sales ledger's dated GST rows, the transport of the GST status fields, and where GSTR-1 places a posted invoice.

## Amendment — owner decision, 2026-10-10: master writes through the one approved path (dated deviation)

The owner approved this direction on 10 Oct 2026: "yes to the direction for today, improve later". It reverses one row of the 8 Oct amendment's rejected table, "Creating masters through the approval window". The reason given there, a second dispatch path into the release, stays true of any second path. This amendment adds none. Nothing below is built yet. Every slice ships as a draft behind a setting that is off by default, and is measured on a lab company before it is switched on.

**What changes.** Production may send three new write classes, each only through `post_approved_import`, the single dispatch path, with its queue, admission reads, all-company marks aim, wire lock, durable intent and native approval:

- creating ledgers;
- altering a ledger's GST, registration and state blocks, and renaming a ledger;
- creating and altering voucher types.

The request becomes a two-variant type, vouchers as today or a master batch. Each variant brings its own admission reads, approval binding, intent payload and verification. The two variants share no defaulted behaviour.

**What still cannot be sent.**

- Deletes of any master.
- Openings.
- Stock-in-Hand ledgers.
- TDS and TCS ledgers where those features are on.
- Caller-supplied XML. Requests are rendered by Bridge.
- Any class not named above.

**Rules that carry over unchanged.**

- Every write is approved in the native dialog. A master dialog lists one master per item and refuses rather than cuts at the existing caps, so a large class is approved in parts.
- The company is pinned by its full identity tuple. The company name in the import request is never treated as a guard.
- Success is decided by reading back, never by Tally's counters. Counters and the company's master mark (`ALTMSTID`) are doubt signals only; one master write can move the mark by more than one.
- An outcome unknown after bytes were sent is never re-sent. It is reconciled by reading back.

**The dated deviation.** The 3 Oct design for automatic ledger creation chose a name-only Create followed by an Alter, subject to a lab run. That run has not been made, and a ledger's GST duty head is settable at Create but not reliably by Alter. This amendment instead creates a ledger with one direct Create, under four conditions:

1. **Before.** An absence check runs inside the queue, over every ledger and group name and alias.
2. **Before.** The company's master mark must be unchanged from the binding read to the last read before the request.
3. **After.** Each created name must resolve to a master that did not exist before. A name that resolves to an existing master is reported for reconciliation, never as created.
4. **Owner's acceptance.** The owner accepts the residual window between the last read and the request, which no read can close.

**List fields.** An Alter replaces a whole list, as measured on one book. So a list is sent as exported, with any new dated entry added, under a check that the ledger is unchanged since it was read. The result is verified by comparing the whole ledger before and after.

**Not measured.** All of these remain unmeasured:

- How Tally resolves a new name against an existing alias, a case or space variant, or a group of the same name.
- Whether a ledger's registration history is appended or replaced by an Alter.
- Any write of a sales ledger's GST rate and HSN lists.
- Every voucher-type write.
- The class-batch dialog seen live.
- The master mark's step for a list alter or a voucher type.

Each slice names the lab measurement it waits on.
