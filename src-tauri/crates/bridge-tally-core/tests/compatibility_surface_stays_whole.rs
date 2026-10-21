//! The compatibility surface (`docs/tally/compatibility/compatibility-surface.json`) stays whole:
//! the pin list is well formed, every pinned path is a regular file that no symbolic link leads to,
//! the two required directories are pinned in full, and the eighteen required files are pinned.
//!
//! These checks were made by the `bridge-tally-compatibility` crate's `gate` command and its
//! `SurfacePins::resolve` (removed with the `tools/` workspace, #1745). The pin gate itself is
//! `scripts/check-surface-ack.mjs` and `scripts/merge-gate.sh`, which read the JSON and the diff;
//! they never looked at the files on disk. This test runs in `Tally portable core` on every pull
//! request. On Windows a checked-out symbolic link is a plain file, so the symlink case is
//! exercised on Linux and macOS only.
//!
//! The text below is the record of why the early pins exist, moved here from the
//! constant it was written beside (`MAX_SURFACE_FILES`, now `MAX_PINS`, a parse bound and not a
//! count to maintain); the one test name it cites is corrected to the test that exists.
//! Where it speaks of the "surface digest" or of a raise, read "a pin added": the digest was removed with the crate, and the reason for a pin added now lives in its own
//! `reason` and in the acknowledgement file the pull request adds.
//!
//! A fixed parse bound on the compatibility surface's pin list: a list longer than this is
//! refused as `surface_file_count_invalid`. It is a bound against a runaway or malformed list, **not
//! a count to maintain**: it is deliberately not kept equal to the number of pins, and adding or
//! removing a pin does not touch it. What decides whether a pin may be added or removed is the pin's
//! own `reason` in `compatibility-surface.json` and the acknowledgement file the pull request adds
//! (`docs/release-process.md`, "Acknowledging a change to a pinned file"), which CI enforces.
//!
//! Every file under the Tally migration and report directories is required by a
//! directory rule; `src/` and the protocol crates remain judgment-pinned
//! because their mixed-purpose directories do not have that invariant.
//!
//! The history below was written while this constant tracked the pin count exactly (it was raised
//! by hand with each pin). It is kept as the record of why each early pin exists; where it speaks
//! of a raise, read "a pin added".
//!
//! **Raised eleven times, the first three by branches that did not see each
//! other.** 210 to 211 on master for `src-tauri/src/agent_ledgers.rs`, 211 to
//! 212 for `src-tauri/crates/bridge-tally-core/src/master_binding.rs`, 212 to
//! 216 in a single commit for the voucher-presence engine, its adapter, its
//! admission-contract assertion, and `agent_catalog.rs` -- the last of those
//! taking the slot a paragraph below had already reserved for it by name, which
//! is why the four pins arrive as one raise and not two -- 216 to 217 for
//! `.github/workflows/dependency-security-scheduled.yml`, 217 to 218 for
//! `src-tauri/src/agent_import_identity.rs`, 218 to 232 for fourteen files
//! named individually below, 232 to 238 for six more, and 238 to 239 for
//! `src-tauri/src/agent_import_amend.rs`, 239 to 251 for the
//! bank-statement parser's twelve files, and, after the lowering to 249
//! described below, 249 to 262 for the thirteen test modules of
//! `src-tauri/src/reports`, and 262 to 266 for the four modules carved out of
//! the `bridge-tally-protocol` crate root. Each reason stands; a
//! merge that
//! keeps a raise but loses its pin would pass the gate with behavior silently
//! outside the evidence boundary, which is the failure this constant exists to
//! make loud.
//!
//! `master_binding.rs` decides `validate_masters` results and, through them,
//! import admission. Left unpinned, an edit confined to the matcher would leave
//! the surface digest unchanged and let existing evidence attest behaviour it
//! never covered. That is the deliberate decision the paragraph above requires,
//! and it is one file for one named reason — not headroom.
//!
//! The slot this paragraph once reserved for `agent_catalog.rs` has been taken
//! by it, as intended. The raise to 217 binds
//! `dependency-security-scheduled.yml`, and the named reason is different in
//! kind from the ones above: it is the only workflow that runs unattended on a
//! schedule holding `issues: write`, and the only one of the five whose sibling
//! is pinned while it is not. Left unpinned, an edit that widened its
//! permissions or pointed its audit at a different lockfile would leave the
//! surface digest unchanged. Every other raise here bound a file that decides
//! what Bridge admits; this one binds a file that decides what Bridge is
//! allowed to do to its own repository while nobody is watching.
//!
//! The raise to 218 binds `agent_import_identity.rs`. The marker derivation it
//! holds is shared by the import writer and the presence reader: the writer
//! stamps a marker into a voucher's narration, and the reader identifies that
//! voucher by it. Left unpinned, an edit to that one derivation would change
//! both halves at once and leave the surface digest unchanged, so a receipt
//! would attest an identity rule the evidence never covered. It is one file for
//! one named reason — not headroom.
//!
//! The raise to 232 binds fourteen files at once, which reads like headroom and
//! is not: each is named here with its own reason, and none was chosen to fill
//! space. They were found together (bridge#416) by looking for unpinned
//! production modules declared by pinned ones, then keeping only those whose
//! own body holds a rule about what Bridge posts or prepares for posting, or
//! what may leave the machine. An edit confined to any of them would leave the
//! surface digest unchanged. Each reason says what the file holds, not that it
//! holds all of a guarantee: several guarantees here are shared with pinned
//! files, and a reason that claimed the whole of one would be false.
//!
//! What Bridge posts, or prepares for posting:
//! - `tally/approved_import.rs` -- the operator approval dialog, and which
//!   choice counts as consent (the named post button, or Yes on Windows).
//! - `agent_import_post.rs` -- the MCP post handler, which admits only a
//!   single saved Journal batch, and its part of the refusal to post one batch
//!   twice; `agent_import.rs` holds the admission lock and journal append.
//! - `agent_import_ledger.rs` -- the import journal replay: whether a batch was
//!   dispatched, derived from its dispatch-intent records, and the refusal of a
//!   second dispatch intent for one batch.
//! - `agent_company.rs` -- finding the loaded company whose GUID matches the
//!   request and refusing when none or several do; import admission and the
//!   company-scoped MCP read tools call it.
//! - `agent_import_cash_bank.rs` -- the reserved-group tables deciding which
//!   ledgers may sit on the cash/bank side of a Payment, Receipt or Contra in
//!   an import file Bridge builds.
//! - `bridge-tally-protocol/src/group_ancestry.rs` -- the ancestry walk under
//!   those tables; its other callers were already pinned and it was not.
//! - `agent_import_persistence.rs` -- whether an earlier import publication has
//!   settled, checked every time the import admission lock is taken.
//! - `tally/runtime_control.rs` -- the read retry loop: the attempt limits,
//!   including the single-attempt policy, and which failures may repeat a
//!   request.
//! - `endpoint_coordination.rs` -- the advisory per-user, per-port lease the
//!   shipped post path takes before dispatch, so two of one OS user's Bridge
//!   processes cannot both hold it while posting to one Tally port.
//! - `bridge-tally-transport/src/wire_gate.rs` and `endpoint_wire.rs`
//!   (bridge#697) -- the one-send-at-a-time lock on a Tally port and its wait
//!   budget. They decide whether a send waits, goes ahead or is refused, and,
//!   through the import POST's single try, whether an import is sent at all.
//!   An edit that turned that one try into a wait would pass every other
//!   pinned file.
//!
//! What leaves the machine, and the record of it:
//! - `agent_protocol.rs` -- the MCP response loop, which records an egress
//!   receipt for a tool response before writing it and decides what is sent
//!   when recording fails.
//! - `agent_egress.rs` -- the egress log: a failed append is truncated back, or
//!   reported as `egress_record_rollback_failed` when that fails, and a torn
//!   final row is refused rather than read as evidence.
//! - `agent_delivery.rs` -- the egress receipt record: the fields it carries,
//!   the response hash it commits to, and that only a persisted preparation
//!   yields a write-completion token.
//!
//! The raise to 238 binds six more files from the same bridge#416 search. Five
//! decide what a read tells a caller, or when a sync may move past rows. The
//! sixth, `agent_receipt_fields.rs`, belongs with the egress record above:
//! bridge#416 listed it as borderline rather than with the fourteen, and it
//! describes the response of any tool, not only a read. Same rule: each reason
//! says what the file holds.
//!
//! What a read reports, or lets a sync skip:
//! - `agent_movement.rs` -- the ledger movement tool, and the voucher predicate
//!   it applies after checking the whole window: cancelled, optional and
//!   entryless vouchers are left out of the movement figures.
//! - `agent_movement_math.rs` -- one ledger's movement row: when an opening was
//!   observed, closing is opening plus debit plus credit; when none was, closing
//!   is left empty and the row is marked `partial` with
//!   `opening_balance_not_observed`.
//! - `agent_outstandings.rs` -- the outstandings tool's receivable and payable
//!   totals over open bills, and its ageing buckets (0-30, 31-60, 61-90, over 90
//!   days, and unaged).
//! - `agent_change_parse.rs` -- `checkpoint_advanceable`: true when a page was
//!   not truncated and its highest returned alter id, or the requested
//!   checkpoint when it returned none, reaches the company's high-water mark.
//!
//! The fifth, `agent_changes.rs` (the changed-since tool), left the list when
//! that tool was removed (#1769).
//!
//! And the egress record:
//! - `agent_receipt_fields.rs` -- `released_fields`, which the egress receipt in
//!   `agent_delivery.rs` uses to describe a released tool response by its JSON
//!   key paths rather than its values.
//!
//! The raise to 239 binds `agent_import_amend.rs`. It decides whether a build
//! may reuse an earlier batch's wire identity, which turns an import file from
//! one Tally creates into one Tally applies over vouchers already in the book:
//! the refusal when any build of that batch was posted natively, and the
//! compare-and-swap that admits an amendment only while each voucher is still
//! as a build of that batch wrote it. Left unpinned, an edit confined to it
//! would leave the surface digest unchanged while changing what Bridge prepares
//! to overwrite. It is one file for one named reason — not headroom.
//!
//! The raise to 251 binds twelve files for the bank-statement parser
//! (`parse_bank_statement`), found by the same rule: each holds a rule about
//! what Bridge prepares for posting, or what may leave the machine.
//!
//! What may leave the machine:
//! - `agent_bank_statement.rs` -- the tool keeps every statement row in a
//!   private local file and returns only a counterparty summary with every name
//!   marked for `mask_parties`; it reads the PDF password from an owner-only
//!   file and never returns or writes it. It also decides which local file
//!   `build_import_xml` may build from by `proposals_id`: one this tool
//!   published, unchanged since, by digest.
//! - `bridge-bank-statement/src/pdf.rs` -- the refusal of a password PDFium
//!   cannot carry, without which `pdfium-render` panics with the password's
//!   bytes in the message; and the refusal of a rotated page.
//!
//! What Bridge prepares for posting:
//! - `bridge-bank-statement/Cargo.toml` -- binds the PDFium API version the
//!   parser loads (`pdfium_7881`), beside the pinned manifests of the other
//!   crates.
//! - `bridge-bank-statement/src/pipeline.rs` -- no proposal exists until the
//!   statement binds to the account and its balance chain and totals reproduce.
//! - `bridge-bank-statement/src/money.rs` -- the balance replay and control
//!   totals that decide whether a statement is proven at all.
//! - `bridge-bank-statement/src/parse.rs` -- which account a statement belongs
//!   to, and which printed lines become rows.
//! - `bridge-bank-statement/src/bank.rs` -- which counterparty a row names,
//!   and so which mapping row, and ledger, it reaches.
//! - `bridge-bank-statement/src/geometry.rs` -- the wrap rule that keeps a
//!   12-digit bank reference intact in the narration.
//! - `bridge-bank-statement/src/text.rs` -- the mapping key that decides which
//!   statement spellings reach one ledger, and the loose fold behind the
//!   suspense flag and the self-cancelling-voucher refusal.
//! - `bridge-bank-statement/src/mapping.rs` -- the refusal of an ambiguous
//!   mapping, a sentinel party, and a Contra without a ledger.
//! - `bridge-bank-statement/src/date.rs` -- the voucher date read from the
//!   statement, refusing an impossible one.
//! - `bridge-bank-statement/src/proposals.rs` -- each proposal's voucher type,
//!   legs and sides, the suspense fallback, and the refusal of a row Bridge's
//!   builder could not accept.
//!
//! Not pinned: `bbox.rs`, which reads `pdftotext` captures for tests and is
//! on no production path, and `refusal.rs` and `lib.rs`, which hold no rule.
//!
//! Lowered to 249 when `db/migrations/mod.rs` and `tally/xml_builder.rs` were
//! deleted. Neither was reachable from any binary, test or feature: the first
//! ran a legacy schema no caller opened, the second named import actions no
//! builder used.
//!
//! Lowered by three, to the pin count then in use, when `axal.rs`, `documents.rs` and `export_registry.rs` were
//! deleted with the unfinished document-upload feature (bridge#912). The raise
//! to 232 above names the first two, which no longer exist.
//!
//! The raise to 262 binds thirteen test files because a directory rule requires
//! it, not because of what they decide. Each `src-tauri/src/reports`
//! file kept its tests in an inline `#[cfg(test)]` module, so editing a test
//! re-hashed a production report. Those modules now live beside their parents
//! as `<stem>_tests.rs`, and `REQUIRED_SURFACE_DIRECTORIES` requires every file
//! under that directory to be pinned, test files included. The test files the
//! same extraction moved out of pinned parents in other directories were left
//! unpinned, following bridge#416. Here the directory rule outranks that, and
//! exempting `_tests.rs` from the rule would
//! change what the gate can miss, not merely what it reports. Nothing else is
//! bound by this raise:
//! - `bulk_party_statement_tests.rs`, `outstandings_working_paper_tests.rs`,
//!   `outstandings_working_paper_store_tests.rs`,
//!   `outstandings_working_paper_xlsx_tests.rs`, `party_ledger_master_tests.rs`,
//!   `party_ledger_master_xlsx_tests.rs`, `party_statement_tests.rs`,
//!   `party_statement_pdf_tests.rs`, `party_statement_xlsx_tests.rs`,
//!   `schedule_iii_tests.rs`, `trial_balance_tests.rs`,
//!   `trial_balance_store_tests.rs` and `trial_balance_xlsx_tests.rs` -- the
//!   tests formerly inline in the report file of the same stem.
//!
//! The raise by four binds the modules carved out of the pinned
//! `bridge-tally-protocol` crate root, so the split does not shrink the seal by
//! what it moved:
//! - `text_encoding.rs` -- how every Tally response body becomes text: the
//!   declared-versus-observed encoding check and bounded, strict UTF-8/UTF-16
//!   decoding.
//! - `import_outcome.rs` -- the import response's application status and its
//!   created, altered, deleted, ignored, errors, cancelled and exceptions
//!   counters, from which Bridge reports what a post did.
//! - `standard_ledger_catalog.rs` -- which standard ledgers Bridge reports as
//!   existing and which company a catalogue belongs to; it refuses a ledger name
//!   carrying a bidirectional override or one of the listed invisible characters
//!   that would render one spelling as another, while admitting the zero-width
//!   joiners Indic ledger names need.
//! - `native_ledger_collection.rs` -- ledger source records and party ledger
//!   master fields from the native Ledger collection, bound to the pinned
//!   company: a ledger read fails when no row's GUID carries the company prefix,
//!   counting rather than dropping individual foreign prefixes, and a party
//!   ledger master read fails as a whole unless the response names the pinned
//!   company GUID.
//!
//! The raise to 267 binds `src/persisted-company-profiles.ts`, which moved out
//! of the pinned `src/main.tsx` with no change to its logic. It decides which
//! saved-profile load may write App's company list: only the latest load merges
//! its page, reports an error or ends loading. Merging overlays each profile's
//! saved fields (mirror company id, correlation key, canonical endpoint) onto
//! the matching company, so an older page landing after a newer one could put
//! stale identity fields back on the company the client switcher and evidence
//! drawer select. It is one file for one named reason -- not headroom.
//!
//! Not pinned, and deliberately: files feature-gated out of every shipped build
//! (`agent_lab.rs`), operator filing
//! labels (`client_groups.rs`, `client_group_label_migration.rs` and the
//! `commands/all_clients.rs` commands over them, none of which reads Tally),
//! and dead or declaration-only modules. bridge#416 records the reasoning for the rest.
//!
//! Also not pinned: the four frontend modules bridge#468 moved out of the pinned
//! `src/main.tsx` and `src/MirrorProofScreen.tsx`, where they had been copied.
//! `tally-mirror-contract.ts` holds only types, which are erased at runtime.
//! `display-format.ts`, `tally-capability-evidence.tsx` and
//! `tally-command-error.tsx` present identifiers, capability labels and command
//! errors. The error notice's classification comes from the pinned
//! `tally-error-copy.ts`. None of them decides which book a report or drawer is
//! attributed to, or what Bridge posts or lets leave the machine.
//!
//! `src-tauri/src/agent_import_verification.rs` is `agent_import.rs`'s read-back
//! verification, moved out with only visibility and `super::` paths changed. It
//! decides whether an imported voucher is reported `posted_verified`, which the
//! posting path and the amendment compare-and-swap also rely on.
//!
//! `src-tauri/src/agent_voucher_window.rs` is the pre-flight volume bound for
//! windowed voucher reads (protocol reference §11c). It decides which voucher
//! requests Bridge sends Tally — whether a window goes out whole, in which date
//! and AlterID parts, and which windows are refused unsent — for `vouchers`,
//! `voucher_presence`, `ledger_movement` and the import-verification read.
//! The canonical protocol-reference index is retained for legacy links while
//! bridge#317 moves its content into six separately pinned parts. The cap grows
//! by those six pins; it remains an exact count after the coordinated pin-list change.
//! `src-tauri/src/agent_import_post_location.rs` (bridge#574) confirms which
//! loaded company a native post is aimed at, last before the POST, and reports
//! which companies' voucher marks moved after it.
//! `native_outstandings/ledger_currency.rs` (bridge#551) classifies each
//! ledger's own currency against the base, which decides whether an
//! outstandings read reports any figure.
//! `src-tauri/src/agent_import_ack.rs` (bridge#239) decides whether a person's
//! review of a doubted post is recorded, and what verify_import reports about
//! it beside the unchanged verdict; a defect there could record a review of a
//! voucher state nobody saw.
//! `bridge-tally-protocol/src/gst_registration.rs` (bridge#624) decides which
//! GSTIN ledger_masters reports for a party on a date; a defect there reports a
//! registered supplier as unregistered, or the reverse.
//! `src-tauri/src/agent_voucher_type_class.rs` (bridge#625) decides which
//! vouchers a type filter returns, and which request is refused as ambiguous;
//! a defect there reports a class of vouchers as absent.
//! `bridge-tally-protocol/src/native_statement_reports.rs`,
//! `src-tauri/src/reports/statements.rs` (with its tests, as every file under
//! `src-tauri/src/reports` is required) and `src-tauri/src/agent_statements.rs`
//! (bridge#692) parse Tally's own Balance Sheet and Profit and Loss, derive both
//! statements from the Trial Balance and group tree, and decide which result
//! `profit_and_loss` and `balance_sheet` report as established; a defect there
//! reports a profit that omits a line.
//! `bridge-tally-protocol/src/currency_composite.rs` (bridge#674) decides
//! whether an amount is a foreign-currency composite, which decides whether a
//! `vouchers` read withholds a voucher or refuses its window; a defect there
//! sets aside an amount that should have refused, or refuses a window that
//! should have been read.
//! `src-tauri/src/agent_import_approval.rs` (bridge#725) holds the approval a
//! post redeems across calls: what it is bound to, how long it lasts, that it is
//! spent once before the intent, and when it is revoked; a defect there could
//! post a batch on an approval for something else, or on one withdrawn.
//! `src-tauri/crates/bridge-bank-statement/src/cash.rs` decides which parsed
//! party is a cash withdrawal or deposit, which answers a person may give for
//! one, which need a ledger and which Bridge refuses; a defect there could post
//! a cash line to a ledger nobody chose.
//! `bridge-tally-protocol/src/xml_text.rs` (bridge#832) is the one escaper every
//! request renderer uses, the voucher-import write path included; a defect there
//! changes which ledger or company a posted voucher names, or lets a value
//! break out of the element it belongs to.
//! `bridge-tally-protocol/src/parent_partition.rs` (bridge#679) decides whether a
//! ledger read too large for one request may be split by parent group, and
//! whether the parts together returned every ledger the catalogue named, once;
//! a defect there returns a book with ledgers missing or repeated.
//! `bridge-tally-protocol/src/ledger_census.rs` (bridge#679) renders the request
//! that counts a book's ledgers by AlterID span, and decides whether the slices
//! added up to a count (order, duplicates, span, an empty census); a defect there
//! sizes the next ledger read for fewer ledgers than the book holds, and a
//! request past the response cap is cut off mid-read.
//! `src-tauri/src/request_trail.rs` (bridge#918) builds, field by field, the
//! record of a call's Tally sends that reaches the egress receipt; a defect
//! there puts request or response text, a company name or a row value into a
//! journal the user keeps.
//!
//! Since schema 3 the reason for a pin added from now on lives in that pin's own entry (`reason`
//! in compatibility-surface.json), not as another paragraph here, and the bound below is no longer
//! raised with each pin, so two pull requests that each add a pin do not collide.
//!
//! Compatibility evidence binds the selected-ledger constructor and the native
//! lifecycle implementation, error fallback, and frontend admission points, rather than
//! trusting only their callers.
//!
//! `agent_ledgers.rs` renders the agent ledger reads. It is here rather than left as a
//! judgment pin because a judgment pin can be dropped during a conflict resolution and
//! the gate still returns `compatibility_gate_passed` -- measured, by deleting this very
//! entry and acknowledging the change. A required path cannot be dropped silently, and
//! `removing_a_required_file_pin_is_refused` iterates this list, so adding it
//! here is what covers its omission.
//!
//! The voucher-presence contract's files follow (#840; until then a test in
//! `book_presence_tests.rs` checked them by hand): `book_presence.rs` and
//! `agent_presence.rs` are its production files; `agent_presence_tests.rs` holds the only
//! independent statement of the admission contract, since the adapter reads its bounds from
//! the published schema; and `agent_import_identity.rs` derives the narration marker both the
//! presence reader and the import writer use, so an edit confined to it silently changes what
//! is reported present.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde_json::{json, Value};

const SCHEMA_VERSION: u64 = 3;
const MAX_PINS: usize = 1024;
const MAX_REASON_CHARS: usize = 500;
const MAX_PATH_BYTES: usize = 240;
const REQUIRED_DIRECTORIES: [&str; 2] = ["src-tauri/src/db/migrations", "src-tauri/src/reports"];
const REQUIRED_FILES: [&str; 18] = [
    "src-tauri/src/agent_catalog.rs",
    "src-tauri/src/agent_desktop_journal.rs",
    "src-tauri/src/agent_ledgers.rs",
    "src-tauri/src/source_draft/lifecycle.rs",
    "src/JournalPostingScreen.tsx",
    "src/ErrorBoundary.tsx",
    "src/NativeLifecycleController.tsx",
    "docs/tally/TALLY_PROTOCOL_REFERENCE.md",
    "docs/tally/TALLY_PROTOCOL_REFERENCE_ENVIRONMENT_AND_REQUESTS.md",
    "docs/tally/TALLY_PROTOCOL_REFERENCE_READS_AND_DATES.md",
    "docs/tally/TALLY_PROTOCOL_REFERENCE_WRITE_RESPONSES_AND_MASTERS.md",
    "docs/tally/TALLY_PROTOCOL_REFERENCE_VOUCHER_WRITES.md",
    "docs/tally/TALLY_PROTOCOL_REFERENCE_COMPANY_IDENTITY_AND_CREATION.md",
    "docs/tally/TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md",
    "src-tauri/crates/bridge-tally-core/src/book_presence.rs",
    "src-tauri/src/agent_presence.rs",
    "src-tauri/src/agent_presence_tests.rs",
    "src-tauri/src/agent_import_identity.rs",
];

/// The code of the first check that failed, in the names the compatibility crate used.
#[derive(Debug, PartialEq, Eq)]
struct SurfaceFault(&'static str);

fn fault<T>(code: &'static str) -> Result<T, SurfaceFault> {
    Err(SurfaceFault(code))
}

/// A path that names one file the same way as text: not empty, at most 240 bytes, no backslash,
/// relative, normal components only, and equal to its normal form (`a//b`, `a/b/` and `a/./b` name
/// a file the normal form already names, so two spellings of one pin could both be listed).
fn check_path(value: &str) -> Result<(), SurfaceFault> {
    if value.is_empty() || value.len() > MAX_PATH_BYTES || value.contains('\\') {
        return fault("surface_path_invalid");
    }
    let path = Path::new(value);
    if path.is_absolute()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        return fault("surface_path_invalid");
    }
    let normal = path
        .components()
        .map(|part| part.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");
    if normal != value {
        return fault("surface_path_not_normalised");
    }
    Ok(())
}

/// A pinned path must be, and sit under, no symbolic link: reading through one would let a file
/// outside the pin list change what the pin stands for.
fn check_no_symlink(root: &Path, relative: &str) -> Result<(), SurfaceFault> {
    let mut current = root.to_path_buf();
    for component in relative.split('/') {
        current.push(component);
        match fs::symlink_metadata(&current) {
            Err(_) => return fault("surface_file_unavailable"),
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return fault("surface_file_symlink")
            }
            Ok(_) => {}
        }
    }
    Ok(())
}

/// Every file under a required directory, as repository-relative paths joined with `/`. An entry
/// that is neither a file nor a directory is refused.
fn files_under(
    root: &Path,
    directory: &str,
    found: &mut BTreeSet<String>,
) -> Result<(), SurfaceFault> {
    let entries = fs::read_dir(root.join(directory))
        .map_err(|_| SurfaceFault("surface_required_directory_unavailable"))?;
    for entry in entries {
        let entry = entry.map_err(|_| SurfaceFault("surface_required_directory_unavailable"))?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| SurfaceFault("surface_required_directory_unavailable"))?;
        let relative = format!("{directory}/{name}");
        let kind = entry
            .file_type()
            .map_err(|_| SurfaceFault("surface_required_directory_unavailable"))?;
        if kind.is_dir() {
            files_under(root, &relative, found)?;
        } else if kind.is_file() {
            found.insert(relative);
        } else {
            return fault("surface_required_directory_entry_unsupported");
        }
    }
    Ok(())
}

fn check_surface(json_text: &str, root: &Path) -> Result<(), SurfaceFault> {
    let document: Value =
        serde_json::from_str(json_text).map_err(|_| SurfaceFault("artifact_json_invalid"))?;
    let Some(top) = document.as_object() else {
        return fault("surface_schema_unsupported");
    };
    if top
        .keys()
        .any(|key| key != "schema_version" && key != "files")
    {
        return fault("artifact_json_invalid");
    }
    if top.get("schema_version").and_then(Value::as_u64) != Some(SCHEMA_VERSION) {
        return fault("surface_schema_unsupported");
    }
    let Some(files) = top.get("files").and_then(Value::as_array) else {
        return fault("surface_file_count_invalid");
    };
    if files.is_empty() || files.len() > MAX_PINS {
        return fault("surface_file_count_invalid");
    }
    let mut paths: Vec<&str> = Vec::with_capacity(files.len());
    for pin in files {
        let Some(pin) = pin.as_object() else {
            return fault("artifact_json_invalid");
        };
        if pin.keys().any(|key| key != "path" && key != "reason") {
            return fault("artifact_json_invalid");
        }
        let Some(path) = pin.get("path").and_then(Value::as_str) else {
            return fault("surface_path_invalid");
        };
        check_path(path)?;
        if let Some(reason) = pin.get("reason") {
            let Some(reason) = reason.as_str() else {
                return fault("surface_pin_reason_invalid");
            };
            if reason.trim().is_empty()
                || reason.chars().count() > MAX_REASON_CHARS
                || reason.chars().any(char::is_control)
            {
                return fault("surface_pin_reason_invalid");
            }
        }
        if paths.last().is_some_and(|previous| *previous >= path) {
            return fault("surface_files_not_unique_sorted");
        }
        paths.push(path);
    }
    let mut folded = BTreeSet::new();
    if paths
        .iter()
        .any(|path| !folded.insert(path.to_ascii_lowercase()))
    {
        return fault("surface_path_case_collision");
    }
    for path in &paths {
        check_no_symlink(root, path)?;
        let metadata = fs::symlink_metadata(root.join(path))
            .map_err(|_| SurfaceFault("surface_file_unavailable"))?;
        if !metadata.is_file() || fs::File::open(root.join(path)).is_err() {
            return fault("surface_file_unavailable");
        }
    }
    let pinned: BTreeSet<&str> = paths.iter().copied().collect();
    let mut required: BTreeSet<String> = BTreeSet::new();
    for directory in REQUIRED_DIRECTORIES {
        match fs::symlink_metadata(root.join(directory)) {
            Err(_) => return fault("surface_required_directory_unavailable"),
            Ok(metadata) if !metadata.file_type().is_dir() => {
                return fault("surface_required_directory_not_directory")
            }
            Ok(_) => {}
        }
        files_under(root, directory, &mut required)?;
    }
    required.extend(REQUIRED_FILES.iter().map(|path| (*path).to_string()));
    if required.iter().any(|path| !pinned.contains(path.as_str())) {
        return fault("surface_required_directory_file_unpinned");
    }
    Ok(())
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn real_surface() -> String {
    fs::read_to_string(
        repository_root().join("docs/tally/compatibility/compatibility-surface.json"),
    )
    .expect("the surface file is readable")
}

/// The real surface with `edit` applied to its parsed form.
fn real_surface_edited(edit: impl FnOnce(&mut Vec<Value>)) -> String {
    let mut document: Value = serde_json::from_str(&real_surface()).unwrap();
    edit(document["files"].as_array_mut().unwrap());
    serde_json::to_string(&document).unwrap()
}

fn index_of(files: &[Value], path: &str) -> usize {
    files
        .iter()
        .position(|pin| pin["path"] == path)
        .unwrap_or_else(|| panic!("{path} is not pinned"))
}

#[test]
fn the_real_surface_is_whole() {
    assert_eq!(check_surface(&real_surface(), &repository_root()), Ok(()));
}

#[test]
fn removing_a_required_file_pin_is_refused() {
    for required in REQUIRED_FILES {
        let edited = real_surface_edited(|files| {
            let at = index_of(files, required);
            files.remove(at);
        });
        assert_eq!(
            check_surface(&edited, &repository_root()),
            Err(SurfaceFault("surface_required_directory_file_unpinned")),
            "{required} must stay pinned"
        );
    }
}

#[test]
fn every_file_under_a_required_directory_must_be_pinned() {
    for directory in REQUIRED_DIRECTORIES {
        let edited = real_surface_edited(|files| {
            let at = files
                .iter()
                .position(|pin| {
                    pin["path"]
                        .as_str()
                        .is_some_and(|p| p.starts_with(&format!("{directory}/")))
                })
                .unwrap_or_else(|| panic!("nothing pinned under {directory}"));
            files.remove(at);
        });
        assert_eq!(
            check_surface(&edited, &repository_root()),
            Err(SurfaceFault("surface_required_directory_file_unpinned")),
            "{directory}"
        );
    }
}

#[test]
fn a_pin_for_a_missing_path_is_refused() {
    let edited = real_surface_edited(|files| {
        files.push(json!({"path": "zz-no-such-file.txt"}));
    });
    assert_eq!(
        check_surface(&edited, &repository_root()),
        Err(SurfaceFault("surface_file_unavailable"))
    );
}

#[test]
fn a_pinned_directory_is_not_a_pinned_file() {
    let edited = real_surface_edited(|files| {
        files.push(json!({"path": "zz-directory"}));
    });
    let root = Scratch::new("directory-pin");
    fs::create_dir(root.path().join("zz-directory")).unwrap();
    // Not the real tree: the directory exists only here, so the fault is the pin's own.
    let mut copy = ScratchSurface::from_real(&root);
    copy.json = edited;
    assert_eq!(copy.check(), Err(SurfaceFault("surface_file_unavailable")));
}

#[test]
fn the_list_and_each_pin_obey_the_shape_rules() {
    let root = repository_root();
    let pin = |path: &str| json!({"path": path});
    let cases: Vec<(Value, &str)> = vec![
        (
            json!({"schema_version": 2, "files": [pin("a")]}),
            "surface_schema_unsupported",
        ),
        (
            json!({"schema_version": 3, "files": [], }),
            "surface_file_count_invalid",
        ),
        (
            json!({"schema_version": 3, "files": [pin("a")], "extra": 1}),
            "artifact_json_invalid",
        ),
        (
            json!({"schema_version": 3, "files": [{"path": "a", "sha256": "x"}]}),
            "artifact_json_invalid",
        ),
        (
            json!({"schema_version": 3, "files": [pin("a\\b")]}),
            "surface_path_invalid",
        ),
        (
            json!({"schema_version": 3, "files": [pin("/abs")]}),
            "surface_path_invalid",
        ),
        (
            json!({"schema_version": 3, "files": [pin("a/../b")]}),
            "surface_path_invalid",
        ),
        (
            json!({"schema_version": 3, "files": [pin("a/./b")]}),
            "surface_path_not_normalised",
        ),
        (
            json!({"schema_version": 3, "files": [pin("")]}),
            "surface_path_invalid",
        ),
        (
            json!({"schema_version": 3, "files": [pin(&"a".repeat(241))]}),
            "surface_path_invalid",
        ),
        (
            json!({"schema_version": 3, "files": [pin("a//b")]}),
            "surface_path_not_normalised",
        ),
        (
            json!({"schema_version": 3, "files": [pin("a/b/")]}),
            "surface_path_not_normalised",
        ),
        (
            json!({"schema_version": 3, "files": [pin("b"), pin("a")]}),
            "surface_files_not_unique_sorted",
        ),
        (
            json!({"schema_version": 3, "files": [pin("a"), pin("a")]}),
            "surface_files_not_unique_sorted",
        ),
        (
            json!({"schema_version": 3, "files": [pin("A"), pin("a")]}),
            "surface_path_case_collision",
        ),
        (
            json!({"schema_version": 3, "files": [{"path": "a", "reason": " "}]}),
            "surface_pin_reason_invalid",
        ),
        (
            json!({"schema_version": 3, "files": [{"path": "a", "reason": "x".repeat(501)}]}),
            "surface_pin_reason_invalid",
        ),
        (
            json!({"schema_version": 3, "files": [{"path": "a", "reason": "x\ny"}]}),
            "surface_pin_reason_invalid",
        ),
        (
            json!({"schema_version": 3, "files": [{"path": "a", "reason": 7}]}),
            "surface_pin_reason_invalid",
        ),
    ];
    for (document, code) in cases {
        assert_eq!(
            check_surface(&document.to_string(), &root),
            Err(SurfaceFault(code)),
            "{document}"
        );
    }
    let too_many: Vec<Value> = (0..=MAX_PINS).map(|n| pin(&format!("f{n:05}"))).collect();
    assert_eq!(
        check_surface(
            &json!({"schema_version": 3, "files": too_many}).to_string(),
            &root
        ),
        Err(SurfaceFault("surface_file_count_invalid"))
    );
    assert_eq!(
        check_surface("{", &root),
        Err(SurfaceFault("artifact_json_invalid"))
    );
    assert_eq!(
        check_surface("[]", &root),
        Err(SurfaceFault("surface_schema_unsupported"))
    );
}

#[cfg(unix)]
#[test]
fn a_symbolic_link_on_a_pinned_path_is_refused() {
    use std::os::unix::fs::symlink;
    let scratch = Scratch::new("symlink");
    let copy = ScratchSurface::from_real(&scratch);
    let target = copy.pinned_file();
    let moved = scratch.path().join("moved-away");
    fs::rename(scratch.path().join(&target), &moved).unwrap();
    symlink(&moved, scratch.path().join(&target)).unwrap();
    assert_eq!(copy.check(), Err(SurfaceFault("surface_file_symlink")));
    // A link in the middle of the path is refused the same way.
    let scratch = Scratch::new("symlink-dir");
    let copy = ScratchSurface::from_real(&scratch);
    let directory = Path::new(&copy.pinned_file())
        .parent()
        .unwrap()
        .to_path_buf();
    let relocated = scratch.path().join("relocated-directory");
    fs::rename(scratch.path().join(&directory), &relocated).unwrap();
    symlink(&relocated, scratch.path().join(&directory)).unwrap();
    assert_eq!(copy.check(), Err(SurfaceFault("surface_file_symlink")));
}

#[cfg(unix)]
#[test]
fn a_required_directory_that_is_a_symbolic_link_is_refused() {
    use std::os::unix::fs::symlink;
    // With pins under it, the pin check meets the link first.
    let scratch = Scratch::new("required-link");
    let copy = ScratchSurface::from_real(&scratch);
    let directory = scratch.path().join(REQUIRED_DIRECTORIES[1]);
    let relocated = scratch.path().join("relocated-reports");
    fs::rename(&directory, &relocated).unwrap();
    symlink(&relocated, &directory).unwrap();
    assert_eq!(copy.check(), Err(SurfaceFault("surface_file_symlink")));
    // With none pinned under it, the directory rule itself refuses a link.
    let scratch = Scratch::new("required-link-bare");
    let copy = ScratchSurface::without_pins_under(&scratch, REQUIRED_DIRECTORIES[1]);
    let directory = scratch.path().join(REQUIRED_DIRECTORIES[1]);
    let relocated = scratch.path().join("relocated-reports");
    fs::rename(&directory, &relocated).unwrap();
    symlink(&relocated, &directory).unwrap();
    assert_eq!(
        copy.check(),
        Err(SurfaceFault("surface_required_directory_not_directory"))
    );
}

#[test]
fn a_missing_required_directory_is_refused() {
    let scratch = Scratch::new("required-missing");
    let copy = ScratchSurface::without_pins_under(&scratch, REQUIRED_DIRECTORIES[0]);
    fs::remove_dir_all(scratch.path().join(REQUIRED_DIRECTORIES[0])).unwrap();
    assert_eq!(
        copy.check(),
        Err(SurfaceFault("surface_required_directory_unavailable"))
    );
}

#[test]
fn an_unpinned_file_added_under_a_required_directory_is_refused() {
    let scratch = Scratch::new("required-extra");
    let copy = ScratchSurface::from_real(&scratch);
    fs::write(
        scratch
            .path()
            .join(REQUIRED_DIRECTORIES[1])
            .join("zz_new_module.rs"),
        b"// new\n",
    )
    .unwrap();
    assert_eq!(
        copy.check(),
        Err(SurfaceFault("surface_required_directory_file_unpinned"))
    );
}

/// A scratch directory removed on drop (the crate has no temporary-directory dependency).
struct Scratch(PathBuf);

impl Scratch {
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "bridge-surface-{}-{label}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |elapsed| elapsed.as_nanos())
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A copy of the real surface file and of every file it pins, in a scratch directory, so a test
/// can break one thing in a tree that is otherwise whole.
struct ScratchSurface<'a> {
    scratch: &'a Scratch,
    json: String,
    /// Read only by the symbolic-link tests, which exist on Unix only.
    #[cfg(unix)]
    pins: Vec<String>,
}

impl<'a> ScratchSurface<'a> {
    fn from_real(scratch: &'a Scratch) -> Self {
        let json = real_surface();
        let document: Value = serde_json::from_str(&json).unwrap();
        let pins: Vec<String> = document["files"]
            .as_array()
            .unwrap()
            .iter()
            .map(|pin| pin["path"].as_str().unwrap().to_string())
            .collect();
        let real = repository_root();
        let mut copied = BTreeSet::new();
        let mut copy = |relative: &str| {
            if copied.insert(relative.to_string()) {
                let target = scratch.path().join(relative);
                fs::create_dir_all(target.parent().unwrap()).unwrap();
                fs::write(&target, b"pinned\n").unwrap();
            }
        };
        for pin in &pins {
            copy(pin);
        }
        // The required directories hold exactly the pinned files under them, as in the real tree.
        for directory in REQUIRED_DIRECTORIES {
            let mut under = BTreeSet::new();
            files_under(&real, directory, &mut under).unwrap();
            for file in under {
                copy(&file);
            }
        }
        Self {
            scratch,
            json,
            #[cfg(unix)]
            pins,
        }
    }

    /// Like `from_real`, but the surface does not pin anything under `directory`, so the
    /// directory can be removed without a pin failing first.
    fn without_pins_under(scratch: &'a Scratch, directory: &str) -> Self {
        let mut surface = Self::from_real(scratch);
        let mut document: Value = serde_json::from_str(&surface.json).unwrap();
        document["files"].as_array_mut().unwrap().retain(|pin| {
            !pin["path"]
                .as_str()
                .unwrap()
                .starts_with(&format!("{directory}/"))
        });
        surface.json = serde_json::to_string(&document).unwrap();
        surface
    }

    /// A pinned file at least one directory deep.
    #[cfg(unix)]
    fn pinned_file(&self) -> String {
        self.pins
            .iter()
            .find(|pin| pin.contains('/'))
            .unwrap()
            .clone()
    }

    fn check(&self) -> Result<(), SurfaceFault> {
        check_surface(&self.json, self.scratch.path())
    }
}
