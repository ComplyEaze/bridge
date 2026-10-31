//! Structural payload admission; live identity, masters and balance remain runtime checks.
use super::{
    LIVE_QUALIFIED_VOUCHER_TYPES, MAX_MASTER_NAMES, MAX_MASTER_NAME_CHARS, MAX_TEXT_CHARS,
    MAX_VOUCHERS,
};
use serde_json::{json, Value};

const CONTROLS: &str = r"[\u0000-\u001F\u007F-\u009F]";
// ECMAScript's `$` can match before these terminal characters. ASCII-only
// scalar patterns additionally refuse them anywhere in the value.
const LINE_TERMINATORS: &str = r"[\u000A\u000D\u2028\u2029]";
const RESERVED_MARKER: &str = r"\[[Bb][Rr][Ii][Dd][Gg][Ee]:";
// A ledger name may end in CR and LF, as some books store one (bridge#626):
// every other control character is refused, and so is a line break with text
// after it. The server admits only one trailing CR LF, and only when a live
// ledger holds exactly those bytes.
const LEDGER_CONTROLS: &str = r"[\u0000-\u0009\u000B\u000C\u000E-\u001F\u007F-\u009F]";
// Any CR or LF other than one CR LF ending the name, without lookaround so
// that any ECMA-262 or RE2-style validator accepts it: a CR followed by
// anything but LF, a CR at the end, an LF after anything but CR, an LF first,
// or an LF with anything after it.
const INTERIOR_LINE_BREAK: &str = r"\u000D[^\u000A]|\u000D$|[^\u000D]\u000A|^\u000A|\u000A[\s\S]";
// Non-control members of Unicode White_Space, matching Rust str::trim, plus
// LF and CR: a ledger name may now end in a line break, so a name of nothing
// else must still read as blank (bridge#626).
const BLANK_LEDGER: &str =
    r"^[\u000A\u000D\u0020\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000]*$";

pub(in crate::agent) fn voucher_input_schema() -> Value {
    let text = json!({
        "type":"string", "minLength":1, "maxLength":MAX_TEXT_CHARS,
        "not":{"anyOf":[{"pattern":CONTROLS},{"pattern":RESERVED_MARKER}]},
        "description":"Nonempty text without control characters. The reserved [BRIDGE: marker is forbidden in every ASCII case, including XML entity-encoded spellings."
    });
    let ledger = json!({
        "type":"string", "minLength":1, "maxLength":MAX_MASTER_NAME_CHARS,
        "not":{"anyOf":[{"pattern":LEDGER_CONTROLS},{"pattern":INTERIOR_LINE_BREAK},{"pattern":BLANK_LEDGER}]},
        "description":"The ledger's exact live name. It may end in one CR LF only when the live ledger's stored name does, as validate_masters reports it in exact_live_spelling; no other control character is accepted. A name that folds equal to another live ledger's name or to one of its aliases (case, spacing, dashes, slashes or quotes, a trailing line break) is refused as ledger_has_folded_twin; a ledger in the refusal only because of an alias carries through_alias."
    });
    let entry = json!({
        "type":"object", "additionalProperties":false,
        "required":["ledger","amount","side"],
        "properties":{
            "ledger":ledger,
            "amount":{
                "type":"string", "pattern":r"^[0-9]+\.[0-9]{2}$",
                "maxLength":bridge_tally_core::MAX_EXACT_DECIMAL_BYTES,
                "not":{"anyOf":[{"pattern":r"^0+\.00$"},{"pattern":LINE_TERMINATORS}]}
            },
            "side":{"enum":["Dr","Cr"]}
        }
    });
    let voucher = json!({
        "type":"object", "additionalProperties":false,
        "required":["bridge_txn_id","date","voucher_type","entries"],
        "properties":{
            "bridge_txn_id":{
                "type":"string", "pattern":"^[A-Za-z0-9_-]{1,64}$",
                "not":{"pattern":LINE_TERMINATORS}
            },
            "date":{
                "type":"string", "pattern":r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$",
                "not":{"pattern":LINE_TERMINATORS}
            },
            "voucher_type":{
                "enum":LIVE_QUALIFIED_VOUCHER_TYPES,
                "description":"Journal takes any balanced set of entries and may carry voucher_number and reference. Payment, Receipt and Contra take two or more entries (for more than two, one three-entry Receipt built by ComplyEaze Bridge has been imported over the gateway and verified; no multi-entry Payment or Contra has been, and none of the three, including that Receipt, through Tally's Import menu), at least one debit and one credit and no ledger on both sides, and neither of those fields, and are refused unless every money-side entry names a ledger whose live group ancestry reaches Bank Accounts or Cash-in-Hand: every credit on a Payment, every debit on a Receipt, every leg on a Contra. Every remaining leg of a Payment or Receipt must be established as holding no money: any money group there means the voucher is really a Contra, and a ledger whose group ancestry cannot be resolved is refused as well. These rules span fields and live masters, so the server enforces them after admission rather than here."
            },
            "narration":text, "reference":text,
            "voucher_number":{
                "type":"string", "minLength":1, "maxLength":32,
                "not":{"pattern":r"[\u0000-\u001F\u007F-\u009F$]|^\s|\s$"}
            },
            "entries":{"type":"array", "minItems":2, "items":entry}
        }
    });
    json!({
        "type":"object", "additionalProperties":false,
        "required":["company_guid"],
        "properties":{
            "company_guid":{"type":"string","minLength":1,"description":crate::agent::catalog::COMPANY_GUID_DESCRIPTION},
            "amends_batch_id":{
                "type":"string",
                "pattern":crate::agent::catalog::BRIDGE_BATCH_ID_PATTERN,
                "description":"Correct vouchers of a batch ComplyEaze Bridge built on this computer, in place. Name that batch (or an earlier amendment of it); every bridge_txn_id must be one that batch or one of its amendments built, with the same voucher_type, voucher_number and reference (a changed, added or removed reference is refused, import_amend_voucher_reference_changed: the import could not be confirmed; change it in Tally after the amendment is imported and verified). The file reuses that batch's REMOTEIDs, so a file import alters the vouchers instead of duplicating them. Refused if any build of that batch was posted natively, and refused unless each named voucher is still in the book as a build of that batch wrote it in the fields compared (date, a bank voucher's effective date when Tally returns one, type, number when set, entries' ledger, amount and side, narration) — checked during this build only, not at import. A reference, bill-wise or cost-centre allocations and the party ledger are not compared, and allocations made in Tally are expected to be lost when the file is imported (not measured directly)."
            },
            "proposals_id":{
                "type":"string", "minLength":46, "maxLength":46,
                "description":"Build from a proposals file parse_bank_statement wrote, instead of `vouchers`. Its vouchers are admitted exactly as inline ones would be. Requires proposals_sha256."
            },
            "proposals_sha256":{
                "type":"string", "minLength":64, "maxLength":64,
                "description":"The sha256 parse_bank_statement returned for that proposals file; the build is refused if the file no longer has it."
            },
            super::transfers::WITHDRAW_SKIPS_KEY:{
                "type":"array", "uniqueItems":true, "maxItems":MAX_VOUCHERS,
                "description":"Statement rows (the bridge_txn_id of a parse_bank_statement proposal, st- followed by the row's date and hash) that a statement of this company skipped as carried by the other account's Contra, and that this build posts anyway as a voucher of any type. A build that posts a row such a skip names is refused as skip_row_paired_elsewhere unless this names it, because posting it books its own bank and, if the other account's Contra exists, books the transfer twice. The refusal reads only a skip held by a statement of only skipped rows or by a batch sent to Tally or found posted, not one in a batch that has only been built. Name a row only when the person confirms the other account's statement does not carry it; never on their behalf. Each row must be a skip ComplyEaze Bridge holds for this company (in a batch, whether or not it was sent, or in a statement of only skipped rows) and a voucher of this build, or the build is refused as withdraw_skip_invalid. A withdrawal is refused while exactly one verified Contra matches the row and nothing else claims that Contra. Where more than one verified Contra matches, the match is also claimed by another row or booked twice, the amount reverses the same day and the match is by date, or the only match carries a different reference and is verified, ComplyEaze Bridge cannot tell which entry the row belongs to: the withdrawal is then the person's explicit choice, and the batch of every matching Contra is listed on the row in the transfers section. While the only match is not yet verified, the withdrawal is refused and the step says to verify that entry first. So is one with any match in a batch not yet verified, whatever the row's state: a person can confirm only what Tally holds. Such a withdrawal is refused as withdraw_skip_paired. The batch records the names. A withdrawn skip is held by the new batch until it is found posted, and is retired once every batch found posted that holds the row has a build in its lineage that recorded the withdrawal, sent or found posted and built after the newest record of the skip (a later build of the skipping statement records it again); otherwise it stays open as posted_against_skip.",
                "items":{"type":"string", "pattern":crate::agent::catalog::STATEMENT_ROW_ID_PATTERN}
            },
            super::bill_wise::APPROVALS_KEY:{
                "type":"array", "maxItems":MAX_MASTER_NAMES,
                "description":"One entry per bill-wise party the person agreed may receive this batch's entries On Account, copied from a bill_wise_party_unapproved refusal of the same batch: its party_digest, which alone says which party is approved (ledger may be copied beside it, and is not read: with party names masked it is a masked name). Leave it out on the first build. A digest that is not one of this batch's parties' digests, or one given twice, is refused as on_account_approval_invalid (cause digest_differs, duplicate or malformed): nothing is accepted and dropped. The digest ties an approval to this exact batch, company, endpoint and ledger; it does not prove that a person said yes.",
                "items":{
                    "type":"object", "additionalProperties":false,
                    "required":["party_digest"],
                    "properties":{
                        "ledger":{"type":"string"},
                        "party_digest":{"type":"string","pattern":"^[0-9a-f]{64}$"}
                    }
                }
            },
            "vouchers":{
                "type":"array", "minItems":1, "maxItems":MAX_VOUCHERS,
                "description":"Exactly one of vouchers or proposals_id. At most 100 distinct ledger names across the batch; repeated ledgers do not reduce the 1000-voucher limit.",
                "items":voucher
            }
        }
    })
}
