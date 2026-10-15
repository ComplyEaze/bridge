# ADR 0006: Incremental Tally authority remains evidence gated

Status: Accepted (2026-07-15)

## Decision

Bridge treats a Tally change identifier as reusable only within an exact scope. The scope binds source
lineage, stable company identity and fingerprint, object type, capability-profile version, product,
release, mode, transport, pack schema, query profile, filter hash, and date/overlap policy. Drift in
any field requires a full snapshot.

An incremental cursor can be established only by a verified full-snapshot proof and an immutable,
directly observed capability record proving monotonic per-object identifiers and inclusive lower-bound
behavior plus an explicit source high watermark for that same scope. The generic pack proof is
necessary but insufficient: an immutable establishment receipt must additionally bind the complete
exact scope, durable snapshot plan, source response, coverage manifest, proof hash, and canonical
watermark. Cursors are stored as canonical decimal text, not signed integers or
floating-point numbers. Any future cursor change will require generation compare-and-swap and an
immutable audit event.

The v6 establishment receipt is itself the generation-1 audit event, and the checkpoint head cannot
exist without that receipt. Both are immutable and database triggers reject updates or deletion.
Because no protocol verifier yet authenticates source-response, coverage, and bracketing-watermark
evidence, a separate database gate currently rejects every establishment-receipt insert. Hashing
caller-provided values is not treated as authority. Checkpoint establishment and advancement are not
enabled. A future reviewed migration must replace that gate only when its verifier can atomically
construct the receipt from structured observations. Any future advancement API must atomically
verify the prior generation and exact proof receipt, write the next generation, and append its audit
event before this restriction can be relaxed.

The current implementation deliberately provides policy, exact-scope persistence, and honest readiness
fallback only. It does not expose live AlterID filtering, checkpoint establishment or advancement,
tombstone activation, scheduling, or a public incremental-start command. Portable transition and
deletion-authority receipts have private fields; production callers cannot self-attest `Verified` or
`Observed` authority.

## Split-book `ALTVCHID` collision

**VERIFIED 2026-08-26** on licensed TallyPrime Silver. Captured Company-collection responses
(the evidence recorded in `TALLY_PROTOCOL_REFERENCE.md` §9.11b) established that a year-end
split of `BRIDGE PROBE B SANDBOX` produced two genuinely distinct books:

| COMPANYNUMBER | GUID | BOOKSFROM | ALTVCHID |
| --- | --- | --- | ---: |
| 100005 | `ec4454ae-5c4c-4bfa-b3b0-68182a749689` | 20250401 | 2785 |
| 100014 | `ec4454ae-5c4c-4bfa-b3b0-68182a749689` | 20260401 | 2785 |

The separate per-object `ALTERID` field differed (207 and 435); this observation is specifically
that company-level `ALTVCHID` collided. `ALTVCHID` is therefore a progress counter, not an
identity discriminator. The GUID collision itself is recorded in
`TALLY_PROTOCOL_REFERENCE.md` §9.11b.

Any future incremental scope must key the complete observed book tuple
`(canonical_origin, COMPANYNUMBER, GUID, NAME, BOOKSFROM)`, never GUID alone. In particular,
two split twins must not share a `scope_sha256` or checkpoint head/state; their numeric high
watermarks may legitimately coincide.

The incremental checkpoint collision is latent: no runtime path writes
`tally_incremental_*` checkpoint heads, and Current Core Accounting snapshots are `Partial`, so
they cannot establish incremental authority. The separate live compatibility defect is fail-closed:
the GUID-only `CompanyBookExtentV1` parser rejects a loaded split pair as
`company_identity_ambiguous` before it considers the expected display name. Any path that still
uses that extent bracket, including native outstandings pairing, therefore needs a composite-tuple
profile and parser before it can serve split books. Snapshot source reads use their separate
composite identity brackets; that does not make the GUID-only extent parser safe for other callers.

## Reasons for remaining disabled

- Current Core Accounting snapshots are `Partial`, so they cannot establish incremental authority.
- Existing generic Core proof rows do not bind an object-specific query/filter contract, complete
  numeric AlterID coverage, or an independently observed source high watermark.
- The Education profile has not proved an explicit source high watermark, inclusive-bound semantics,
  empty-feed behavior, reset/regression behavior, cancelled-voucher coverage, or deletion rules.
- A maximum identifier in returned rows is not an acceptable substitute for an explicit source high
  watermark.
- Absence from a delta never means deletion. A tombstone requires a separately observed rule and a
  verified final-state proof.
- Verification must hash and reconcile the complete materialized state after overlay, not the delta
  payload alone.

## Consequences

Bridge may do more full reads than a speculative integration, but it cannot silently lose edits,
invent deletions, or advance a cursor from partial evidence. The legacy name-based `altmastid_cache`
helper is retired from runtime code; the old table remains only for database compatibility.

## Why live AlterID filtering is not exposed (2026-10-16)

The `changed_since` tool, which paged Tally's AlterID change rows for vouchers, ledgers and groups,
was deleted on this date. It had been withheld since it was added (7 September 2026): the tool
list omitted it, and `tool_payload` refused a direct call before it read an argument. So no client
ever received a result from it, and the two defects below were in handler code that never ran for
a caller. They are recorded so that a later re-add does not repeat them.

1. **A legacy argument was the fallback cursor for both marks.** The handler read `alter_id` as the
   fallback for both the voucher mark and the master mark. With marks 500 and 900, `alter_id=500`
   would have started the master feed at 501: masters 1 to 500 never returned, and the result would
   have looked complete. The reverse, `alter_id=900`, put the voucher cursor past its snapshot and
   was refused (`change_checkpoint_exceeds_snapshot`). That argument was not reachable even without the refusal: the published schema had no `alter_id`
   property, so `validate_tool_arguments` refused it as `argument_unknown`. A re-add must not accept a
   legacy single cursor at all, or must refuse it whenever the two marks differ.
2. **Rows were cut by AlterID alone.** Ledger and group rows were merged, sorted by AlterID and
   cut at the page size, and the last AlterID became the next cursor. Two rows of different kinds
   with the same AlterID at a page boundary would lose the second for good. This one was reachable
   from the handler's own logic whenever kinds share AlterID values. Whether they do is not
   settled: on one synthetic book the kinds measured shared one counter, with no duplicate across
   32 rows (partial evidence; the measurement is not recorded in this repository). A re-add must
   page on a key that cannot tie (AlterID plus kind and identity) or read each kind separately.

The feed also read only ledgers and groups, while the master mark covers more. The protocol
reference records that switching a voucher type to Manual numbering moved the company's master mark
(`ALTMSTID`, 224 to 226) with no voucher `ALTERID` changing (TALLY_PROTOCOL_REFERENCE_VOUCHER_WRITES.md,
the numbering-switch record). A feed that stops when the master mark stops moving would therefore
need to read voucher types; this ADR records no other master kind, because no document or test in
this repository shows one.

Nothing here changes the decision above: incremental authority still needs the proofs listed under
"Reasons for remaining disabled".
