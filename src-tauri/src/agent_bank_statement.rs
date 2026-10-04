//! `parse_bank_statement`: a local capability that turns a password-protected
//! bank-statement PDF into voucher proposals. It reads the statement and writes
//! the proposals to a new private file; it never contacts Tally.
//!
//! **What may leave the machine is decided here.** The statement's rows are
//! a client's banking record, and a tool result reaches the AI conversation.
//! So the full proposals — every row's date, amount, bank reference and
//! narration — are written to a private local file, and the result carries
//! only what an operator needs to write the mapping: each counterparty's
//! spelling as printed, its row count, its disposition and whether it reached
//! suspense, and the ledger names to check with `validate_masters`. It carries
//! no amount of its own: only the figures the caller supplied are echoed, and
//! an open cash line's amount (below). Every name in that summary is marked as
//! a party name, so the `mask_parties` redaction preset masks it. The only
//! row-level values returned are an open cash line's: `cash_questions`
//! identifies it by its id, date, amount, party name and movement so a person
//! can answer it (owner ruling b1).
//!
//! **The password never enters the conversation.** It is read from a local
//! owner-only file named by `password_file`, held in a zeroizing buffer, handed
//! to PDFium once and dropped. It is not echoed, logged, persisted or hashed
//! into anything; the egress receipt hashes the arguments, which carry only the
//! file's path. Residual: `pdfium-render` and PDFium each keep a copy that is
//! not zeroised (see `bridge_bank_statement::pdf`).
//!
//! **No identity is minted.** Proposals carry `bridge_txn_id` labels and no
//! REMOTEID or XML. `build_import_xml` builds from the file when given its
//! `proposals_id` and the `sha256` this tool returned
//! ([`load_proposals`], through [`resolve_import_arguments`]); the file's
//! vouchers then pass the same admission as inline vouchers, so nothing here
//! decides what is admitted.

use super::*;
use crate::local_files::local_disk_path::LocalDiskPath;
use bridge_bank_statement::bank::Bank;
use bridge_bank_statement::cash::{CashAnswer, CashAnswerRow, CashAnswers, CashMovement};
use bridge_bank_statement::date::Date;
use bridge_bank_statement::mapping::{Mapping, MappingRow};
use bridge_bank_statement::money::Controls;
use bridge_bank_statement::pdf::{self, MAX_PDF_BYTES};
use bridge_bank_statement::pipeline::{prepare, ParsedStatement, StatementRequest};
use bridge_bank_statement::proposals::StatementRecord;
use bridge_bank_statement::proposals::{format_amount, Disposition};
use bridge_bank_statement::Refusal;
use std::fs;
use std::io::Read;
use std::path::Path;
use zeroize::Zeroizing;

pub(super) const MAX_MAPPING_ROWS: usize = 1000;
pub(super) const MAX_CASH_ANSWERS: usize = 1000;
const MAX_PASSWORD_FILE_BYTES: u64 = 1024;
const MAX_PATH_CHARS: usize = 4096;
const PROPOSALS_DIRECTORY: &str = "bank-statements";
const PROPOSALS_SCHEMA: &str = "bridge.bank_statement.proposals.v1";
/// Larger than any admissible file: 1000 vouchers at the text limits.
const MAX_PROPOSALS_FILE_BYTES: u64 = 16 * 1024 * 1024;

pub(super) fn input_schema() -> Value {
    let path = json!({"type":"string","minLength":1,"maxLength":MAX_PATH_CHARS,"pattern":r"\S"});
    let control = json!({"type":"string","minLength":1,"maxLength":64,"pattern":r"\S"});
    let totals_control = json!({"type":"string","minLength":1,"maxLength":64,"pattern":r"\S","description":"Required for sbi and hdfc, whose statements print it. A ubi statement prints no totals: omit both, and every page must then print its Page N of M footer."});
    let ledger = json!({"type":"string","minLength":1,"maxLength":agent_import::MAX_MASTER_NAME_CHARS,"pattern":r"\S"});
    json!({
        "type":"object", "additionalProperties":false,
        "required":["statement_path","password_file","bank","account_label","opening_balance","closing_balance","bank_ledger","suspense_ledger"],
        "properties":{
            "statement_path": path,
            "password_file": path,
            "bank":{"type":"string","enum":["sbi","hdfc","ubi"],"description":"sbi (State Bank of India), hdfc (HDFC Bank) or ubi (Union Bank of India)."},
            "account_label":{"type":"string","minLength":4,"maxLength":64,"pattern":r"\S","description":"A short label carrying at least the last 4 digits of the account, e.g. 'HDFC CA xx4321'. Those digits must end a number on the statement's account-number line. The label is written into each narration."},
            "opening_balance": control,
            "closing_balance": control,
            "total_debits": totals_control.clone(),
            "total_credits": totals_control,
            "bank_ledger": ledger,
            "suspense_ledger": ledger,
            "from":{"type":"string","pattern":"^[0-9]{4}-?[0-9]{2}-?[0-9]{2}$"},
            "to":{"type":"string","pattern":"^[0-9]{4}-?[0-9]{2}-?[0-9]{2}$"},
            "mapping":{
                "type":"array", "maxItems":MAX_MAPPING_ROWS,
                "items":{
                    "type":"object", "additionalProperties":false, "required":["party","ledger"],
                    "properties":{
                        "party":{"type":"string","minLength":1,"maxLength":agent_import::MAX_MASTER_NAME_CHARS,"pattern":r"\S"},
                        "ledger":{"type":"string","maxLength":agent_import::MAX_MASTER_NAME_CHARS},
                        "treatment":{"type":"string","enum":["auto","contra","skip"]}
                    }
                }
            },
            "cash_answers":{
                "type":"array", "maxItems":MAX_CASH_ANSWERS,
                "description":"One answer per cash withdrawal or deposit listed in cash_questions, by its bridge_txn_id. Give the ledger the answer asks for; dont_know takes none and posts the line to suspense_ledger, tagged for the CA.",
                "items":{
                    "type":"object", "additionalProperties":false, "required":["bridge_txn_id","answer"],
                    "properties":{
                        "bridge_txn_id":{"type":"string","minLength":1,"maxLength":64,"pattern":r"\S"},
                        "answer":{"type":"string","enum":["business_cash","owner_use","paid_to_someone","own_cash_box","unbooked_cash_sales","customer_paid_in","owner_brought_in","dont_know"]},
                        "ledger":{"type":"string","maxLength":agent_import::MAX_MASTER_NAME_CHARS}
                    }
                }
            }
        }
    })
}

pub(super) const DESCRIPTION: &str = "Read a local, password-protected SBI, HDFC or Union Bank of India bank-statement PDF and propose one Payment, Receipt or Contra per row (none for a cash line not yet answered), for build_import_xml's voucher shape. The whole run is refused unless the statement's account-number line ends with the digits in account_label, and every row's running balance, the closing balance, and (where the statement prints them) the debit and credit totals reproduce the figures supplied exactly. The password is read from password_file, a local file only its owner can read, and is never returned. Full proposals stay in a private local file; the result is a counterparty summary (spelling as printed, row count, disposition, suspense) for writing `mapping`, and the ledger names to check with validate_masters. A party the mapping does not name, or the parser could not identify, goes to suspense_ledger, tagged UNIDENTIFIED; `skip` omits a transfer already carried by another account's Contra. Only SBI 'ATM WDL' withdrawals and Union Bank 'BY CASH' deposits are recognised as cash. Other cash text is not: where the parser names a party it is an ordinary party, and where it cannot (as for SBI deposits and HDFC cash text) it goes to the UNIDENTIFIED fallback. A recognised cash line is never mapped or defaulted: it is returned in cash_questions with its question and answers, and build_import_xml refuses the proposals (cash_questions_open) until each is answered in cash_answers. Only a dont_know answer posts one to suspense_ledger, tagged \"Bridge: purpose not confirmed; reclassify\"; every line sent to suspense is counted (suspense_rows, and suspense_by_reason by why), and suspense parties are listed first in counterparties, within its bound, with their rows; a line's own date, amount and label stay in the local file. The result carries no amount except the figures the caller supplied, echoed in reconciled (closing_balance, and total_debits and total_credits where given), and an open cash line's amount (below). Arithmetic on the caller's own inputs can still give a row's amount, for example total_debits minus the open cash amounts when one other debit row remains, and when a direction has a single row the caller's own total is that row's amount and counterparties names its party; and a from/to window holding one row shows through rows_in_window that a row falls in it, and through counterparties that row's party and direction; no result can hide what follows from the caller's inputs. An open cash line is the one exception: its cash_questions entry carries its bridge_txn_id, date and amount, its party name (masked when BRIDGE_AGENT_REDACTION is mask_parties) and whether it is a withdrawal or a deposit, so the person can tell which line is asked about. No other value of any row is returned. Every list in the result is bounded by the response size and counts what it left out (cash_questions_omitted, counterparties_omitted, ledgers_to_validate_omitted); cash_questions_open, suspense_rows and skipped count them all. An answer for a row outside from/to is refused (cash_answer_outside_window). An ambiguous mapping is refused, never guessed. Re-run with a corrected mapping: bridge_txn_id labels depend only on the statement row, so they do not change. To build, pass the returned proposals_id and sha256 to build_import_xml as proposals_id and proposals_sha256; to correct a batch already built from an earlier run, add amends_batch_id. Never contacts Tally.";

impl Server {
    pub(super) async fn parse_bank_statement(
        &self,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        let request = OwnedRequest::from_args(args)?;
        let data_dir = self.settings.data_dir.clone();
        let max_bytes = self.settings.max_bytes;
        let result = tokio::task::spawn_blocking(move || run(&request, &data_dir, max_bytes))
            .await
            .map_err(|_| "statement_task_failed".to_string())??;
        let (response_sha256, bytes) = returned_evidence(&result, self.settings.redaction);
        Ok(ToolOutcome {
            evidence: Evidence {
                request_sha256: sha256_hex(b"parse_bank_statement"),
                response_sha256,
                bytes,
                state: "complete",
                read_at: None,
                duration_ms: None,
                reason_code: None,
            },
            payload: json!({"result": result}),
            company_guid: None,
            truncated: false,
        })
    }
}

/// The admitted arguments, owned so the parse can run off the async runtime.
struct OwnedRequest {
    statement_path: LocalDiskPath,
    password_file: LocalDiskPath,
    bank: Bank,
    account_label: String,
    controls: Controls,
    bank_ledger: String,
    suspense_ledger: String,
    date_from: Option<Date>,
    date_to: Option<Date>,
    mapping: Mapping,
    cash_answers: CashAnswers,
}

fn refused(refusal: &Refusal) -> String {
    match refusal.row {
        Some(row) => format!("statement_{}:row_{row}", refusal.category),
        None => format!("statement_{}", refusal.category),
    }
}

/// A file argument, admitted by its text before anything is opened: a path
/// that begins with two separators (a network share, a verbatim or a device
/// path) or is not rooted on a local disk is refused here, so no open is ever
/// attempted on it.
fn local_disk_path(args: &Value, key: &str) -> Result<LocalDiskPath, String> {
    LocalDiskPath::parse(required_string(args, key)?).map_err(|_| format!("argument_invalid:{key}"))
}

fn window_date(args: &Value, key: &str) -> Result<Option<Date>, String> {
    optional_string(args, key)?
        .map(|text| {
            let compact = normalized_date(&text)?;
            Date::parse_iso(&format!(
                "{}-{}-{}",
                &compact[..4],
                &compact[4..6],
                &compact[6..]
            ))
            .ok_or_else(|| format!("argument_invalid:{key}"))
        })
        .transpose()
}

impl OwnedRequest {
    fn from_args(args: &Value) -> Result<Self, String> {
        let schema = input_schema();
        if let Some(mapping) = args.get("mapping") {
            catalog::validate_against_schema(mapping, &schema["properties"]["mapping"], "mapping")?;
        }
        if let Some(answers) = args.get("cash_answers") {
            catalog::validate_against_schema(
                answers,
                &schema["properties"]["cash_answers"],
                "cash_answers",
            )?;
        }
        let bank = Bank::from_name(required_string(args, "bank")?)
            .ok_or_else(|| "argument_invalid:bank".to_string())?;
        let total_debits = optional_string(args, "total_debits")?;
        let total_credits = optional_string(args, "total_credits")?;
        let controls = Controls::parse_optional(
            required_string(args, "opening_balance")?,
            required_string(args, "closing_balance")?,
            total_debits.as_deref(),
            total_credits.as_deref(),
        )
        .map_err(|refusal| refused(&refusal))?;
        if bank.prints_totals() && controls.debits.is_none() {
            return Err("statement_control_totals_required".into());
        }
        let date_from = window_date(args, "from")?;
        let date_to = window_date(args, "to")?;
        if let (Some(from), Some(to)) = (date_from, date_to) {
            if from > to {
                return Err("statement_reversed_date_window".into());
            }
        }
        let rows = args
            .get("mapping")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .enumerate()
            .map(|(index, row)| MappingRow {
                origin: format!("mapping[{index}]"),
                party: row["party"].as_str().unwrap_or_default().to_string(),
                ledger: row["ledger"].as_str().unwrap_or_default().to_string(),
                treatment: row["treatment"].as_str().map(str::to_string),
            });
        let mapping = Mapping::from_rows(rows).map_err(|refusal| refused(&refusal))?;
        let answers = args
            .get("cash_answers")
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or_default()
            .iter()
            .enumerate()
            .map(|(index, row)| CashAnswerRow {
                origin: format!("cash_answers[{index}]"),
                bridge_txn_id: row["bridge_txn_id"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                answer: row["answer"].as_str().unwrap_or_default().to_string(),
                ledger: row["ledger"].as_str().map(str::to_string),
            });
        let cash_answers = CashAnswers::from_rows(answers).map_err(|refusal| refused(&refusal))?;
        Ok(Self {
            statement_path: local_disk_path(args, "statement_path")?,
            password_file: local_disk_path(args, "password_file")?,
            bank,
            account_label: required_string(args, "account_label")?.to_string(),
            controls,
            bank_ledger: required_string(args, "bank_ledger")?.to_string(),
            suspense_ledger: required_string(args, "suspense_ledger")?.to_string(),
            date_from,
            date_to,
            mapping,
            cash_answers,
        })
    }
}

/// The password file's contents, less one trailing line ending.
///
/// Refused unless it is a regular file owned by this user with a single link
/// and, on Unix, no group or other permission bits: a password that anyone
/// else can read is not one this tool should be trusted to keep.
fn read_password(path: &LocalDiskPath) -> Result<Zeroizing<String>, String> {
    let file = local_file::open_local_file(path.as_path(), false)
        .map_err(|_| "statement_password_file_unreadable".to_string())?;
    let metadata = file
        .metadata()
        .map_err(|_| "statement_password_file_unreadable".to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if metadata.permissions().mode() & 0o077 != 0 {
            return Err("statement_password_file_permissions".into());
        }
    }
    if metadata.len() > MAX_PASSWORD_FILE_BYTES {
        return Err("statement_password_file_too_large".into());
    }
    let mut bytes = Zeroizing::new(Vec::with_capacity(MAX_PASSWORD_FILE_BYTES as usize));
    file.take(MAX_PASSWORD_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "statement_password_file_unreadable".to_string())?;
    if bytes.len() as u64 > MAX_PASSWORD_FILE_BYTES {
        return Err("statement_password_file_too_large".into());
    }
    for ending in [&b"\r\n"[..], &b"\n"[..]] {
        if bytes.ends_with(ending) {
            let keep = bytes.len() - ending.len();
            bytes.truncate(keep);
            break;
        }
    }
    let text =
        std::str::from_utf8(&bytes).map_err(|_| "statement_password_file_not_utf8".to_string())?;
    Ok(Zeroizing::new(text.to_string()))
}

fn read_statement(path: &LocalDiskPath) -> Result<Vec<u8>, String> {
    let file = local_file::open_local_file(path.as_path(), false)
        .map_err(|_| "statement_file_unreadable".to_string())?;
    let limit = MAX_PDF_BYTES as u64;
    let mut bytes = Vec::new();
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "statement_file_unreadable".to_string())?;
    if bytes.len() as u64 > limit {
        return Err("statement_file_too_large".into());
    }
    Ok(bytes)
}

/// Where the bundled PDFium library is: beside this executable, unless
/// `BRIDGE_PDFIUM_LIBRARY` names an absolute path to it.
fn pdfium_library() -> Result<PathBuf, String> {
    if let Some(path) = env::var_os("BRIDGE_PDFIUM_LIBRARY") {
        let path = PathBuf::from(path);
        return if path.is_absolute() {
            Ok(path)
        } else {
            Err("statement_pdf_engine_setting_invalid".into())
        };
    }
    let executable =
        env::current_exe().map_err(|_| "statement_pdf_engine_unavailable".to_string())?;
    let directory = executable
        .parent()
        .ok_or_else(|| "statement_pdf_engine_unavailable".to_string())?;
    Ok(pdf::platform_library_path(directory))
}

fn run(request: &OwnedRequest, data_dir: &Path, max_bytes: usize) -> Result<Value, String> {
    // Everything that can be refused without the PDF was refused already.
    let bytes = read_statement(&request.statement_path)?;
    let pages = {
        let password = read_password(&request.password_file)?;
        let engine = pdf::engine(&pdfium_library()?).map_err(|refusal| refused(&refusal))?;
        pdf::extract_pages(engine, &bytes, &password).map_err(|refusal| refused(&refusal))?
    };
    let parsed = prepare(
        &pages,
        &StatementRequest {
            bank: request.bank,
            account_label: &request.account_label,
            controls: &request.controls,
            bank_ledger: &request.bank_ledger,
            suspense_ledger: &request.suspense_ledger,
            mapping: &request.mapping,
            cash_answers: &request.cash_answers,
            date_from: request.date_from,
            date_to: request.date_to,
        },
    )
    .map_err(|refusal| refused(&refusal))?;
    let statement_sha256 = sha256_hex(&bytes);
    let (proposals_id, file_sha256) = persist(data_dir, request, &parsed, &statement_sha256)?;
    Ok(summary(
        request,
        &parsed,
        &proposals_id,
        &file_sha256,
        max_bytes,
    ))
}

/// A parse result's hash and size as the caller receives it: redacted by the
/// same function, for the same tool, that redacts the response, so the two
/// cannot differ.
fn returned_evidence(result: &Value, redaction: Redaction) -> (String, usize) {
    let returned = redact_tool_response(
        "parse_bank_statement",
        json!({ "result": result }),
        redaction,
    )["result"]
        .take();
    let bytes = serde_json::to_vec(&returned).map_or(0, |bytes| bytes.len());
    (sha256_json(&returned), bytes)
}

fn account_last4(account_number: &str) -> String {
    let characters: Vec<char> = account_number.chars().collect();
    characters[characters.len().saturating_sub(4)..]
        .iter()
        .collect()
}

fn persist(
    data_dir: &Path,
    request: &OwnedRequest,
    parsed: &ParsedStatement,
    statement_sha256: &str,
) -> Result<(String, String), String> {
    let directory = data_dir.join(PROPOSALS_DIRECTORY);
    ensure_private_directory(&directory)
        .map_err(|_| "statement_proposals_directory_unavailable".to_string())?;
    let proposals_id = format!("statement-{}", uuid::Uuid::new_v4());
    let mut document = json!({
        "schema": PROPOSALS_SCHEMA,
        "proposals_id": proposals_id,
        "created_at": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
        "bank": request.bank.name(),
        "account_last4": account_last4(&parsed.account_number),
        "statement_sha256": statement_sha256,
        "bank_ledger": request.bank_ledger,
        "suspense_ledger": request.suspense_ledger,
        "controls": {
            "opening_balance": request.controls.opening.as_str(),
            "closing_balance": request.controls.closing.as_str(),
            "total_debits": request.controls.debits.as_ref().map(|value| value.as_str()),
            "total_credits": request.controls.credits.as_ref().map(|value| value.as_str()),
        },
        "vouchers": parsed.build.proposals,
        "records": parsed.build.records,
    });
    // Optional, and not part of the schema's required shape: the loader reads
    // fields by name, so an older binary still reads a file that carries it.
    // The statement's own first and last row dates, and whether this parse's
    // from/to window left any row out. None of it is returned in the summary.
    if let Some((first, last)) = parsed.build.span {
        document["window"] = json!({
            "first_row_date": first.iso(),
            "last_row_date": last.iso(),
            "whole_statement": parsed.build.rows_outside_window == 0,
        });
    }
    let bytes = serde_json::to_vec_pretty(&document)
        .map_err(|_| "statement_proposals_serialization_failed".to_string())?;
    let path = directory.join(format!("{proposals_id}.json"));
    let staged = directory.join(format!(".{proposals_id}.json.partial"));
    // A private write, then a rename: a reader never sees a partial file under
    // the published name.
    let written = (|| {
        let mut file = local_file::open_local_file(&staged, true)
            .map_err(|_| "statement_proposals_write_failed".to_string())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            file.set_permissions(fs::Permissions::from_mode(0o600))
                .map_err(|_| "statement_proposals_write_failed".to_string())?;
        }
        std::io::Write::write_all(&mut file, &bytes)
            .and_then(|()| file.sync_all())
            .map_err(|_| "statement_proposals_write_failed".to_string())?;
        fs::rename(&staged, &path).map_err(|_| "statement_proposals_write_failed".to_string())
    })();
    if let Err(error) = written {
        let _ = fs::remove_file(&staged);
        return Err(error);
    }
    Ok((proposals_id, sha256_hex(&bytes)))
}

fn summary(
    request: &OwnedRequest,
    parsed: &ParsedStatement,
    proposals_id: &str,
    file_sha256: &str,
    max_bytes: usize,
) -> Value {
    let records = &parsed.build.records;
    let skipped = records
        .iter()
        .filter(|record| record.disposition == Disposition::Skipped)
        .count();
    // Suspense parties first (most rows first within each, then by name and
    // disposition, so the order says nothing of amounts): the unmapped and unidentified, but
    // also parties mapped to the suspense ledger and dont_know cash lines,
    // which the group cannot tell apart. Mapping some of those listed and
    // re-running brings others into view; parties kept in suspense stay at the
    // head.
    let mut groups = parsed.counterparties.iter().collect::<Vec<_>>();
    // The key is unique per group (a group is one party's spelling, one
    // disposition, suspense or not), so the order is total.
    groups.sort_by(|left, right| {
        (
            !left.suspense,
            std::cmp::Reverse(left.rows),
            &left.party,
            &left.disposition,
        )
            .cmp(&(
                !right.suspense,
                std::cmp::Reverse(right.rows),
                &right.party,
                &right.disposition,
            ))
    });
    let counterparties: Vec<Value> = groups
        .into_iter()
        .map(|group| {
            json!({
                "party": party_name(group.party.clone()),
                "also_printed_as": group.also_printed_as.iter().cloned().map(party_name).collect::<Vec<_>>(),
                "ledger": party_name(group.ledger.clone()),
                "disposition": group.disposition,
                "suspense": group.suspense,
                "rows": group.rows,
            })
        })
        .collect();
    let mut ledgers: Vec<&str> =
        std::iter::once(request.bank_ledger.as_str())
            .chain(
                parsed.build.proposals.iter().flat_map(|proposal| {
                    proposal.entries.iter().map(|entry| entry.ledger.as_str())
                }),
            )
            .collect();
    ledgers.sort_unstable();
    ledgers.dedup();
    // Row-level content (a line's date, amount or label) stays in the local
    // file, except the four fields an open cash question carries (see
    // cash_questions); no amount is returned but the caller's own. Suspense
    // lines are counted by reason, and each suspense party is in
    // counterparties with its rows. Every list grows with the
    // statement, so each is bounded, with what was left out counted. The MCP
    // frame carries the result twice, so the lists together take under half:
    // a quarter for the cash questions, an eighth for the counterparties
    // (suspense parties first) and a sixteenth for the ledgers.
    let open = records
        .iter()
        .filter(|record| record.disposition == Disposition::NeedsAnswer)
        .count();
    let mut budget = max_bytes / 4;
    let (cash_questions, cash_questions_omitted) = bounded(cash_questions(records), &mut budget);
    let (counterparties, counterparties_omitted) = bounded(counterparties, &mut (max_bytes / 8));
    let (ledgers_to_validate, ledgers_to_validate_omitted) = bounded(
        ledgers
            .into_iter()
            .map(|name| json!(party_name(name.to_string())))
            .collect(),
        &mut (max_bytes / 16),
    );
    let next_step = if open == 0 {
        "Write mapping from counterparties and re-run until no row needs a ledger it should not reach (counterparties lists suspense parties first, most rows first: unmapped and unidentified parties, parties mapped to the suspense ledger, and dont_know cash lines. When counterparties_omitted is above zero, mapping some of those listed and re-running brings others into view, unless the listed ones are parties kept in suspense. Otherwise raise BRIDGE_AGENT_MAX_BYTES); check ledgers_to_validate with validate_masters. Then call build_import_xml with company_guid, proposals_id and proposals_sha256 set to this proposals_id and sha256. To correct a batch already built from an earlier run of this statement, also pass amends_batch_id; never rebuild it as a new batch."
    } else {
        "Ask the person each cash_questions entry in its own words, and re-run with their answers in cash_answers (by bridge_txn_id, with the ledger the answer asks for). Never answer for them: dont_know is an answer they give, and it posts the line to suspense_ledger for the CA to move. build_import_xml refuses these proposals (cash_questions_open) while any question is open."
    };
    json!({
        "proposals_id": proposals_id,
        "sha256": file_sha256,
        "bank": request.bank.name(),
        "account_last4": account_last4(&parsed.account_number),
        "statement_rows": parsed.statement_rows,
        "rows_in_window": records.len(),
        "vouchers": parsed.build.proposals.len(),
        "skipped": skipped,
        "suspense_rows": records.iter().filter(|record| record.suspense).count(),
        "suspense_by_reason": suspense_by_reason(records),
        "cash_questions_open": open,
        "cash_questions": cash_questions,
        "cash_questions_omitted": cash_questions_omitted,
        // Only the caller's own figures are echoed. Totals the caller did not
        // give (a layout that prints none) are not returned.
        "reconciled": {
            "running_balance_every_row": true,
            "closing_balance": format_amount(&request.controls.closing),
            "total_debits": request.controls.debits.as_ref().map(format_amount),
            "total_credits": request.controls.credits.as_ref().map(format_amount),
            "totals_match_statement": request.controls.debits.is_some(),
        },
        "counterparties": counterparties,
        "counterparties_omitted": counterparties_omitted,
        "ledgers_to_validate": ledgers_to_validate,
        "ledgers_to_validate_omitted": ledgers_to_validate_omitted,
        "next_step": next_step,
    })
}

/// Every open cash line, with its question and every answer it may take, so
/// the agent asks exactly these and invents none. The line's bridge_txn_id,
/// date and amount, and its party name (masked under mask_parties), are the
/// only row-level fields the result carries (owner ruling b1): a person cannot
/// answer for a line they cannot identify. Nothing else of the line is listed
/// here, and an answered line is not listed at all.
fn cash_questions(records: &[StatementRecord]) -> Vec<Value> {
    records
        .iter()
        .filter(|record| record.disposition == Disposition::NeedsAnswer)
        .filter_map(|record| {
            let (question, answers) = record.cash_movement?.question();
            Some(json!({
                "bridge_txn_id": record.bridge_txn_id,
                "date": record.date,
                "amount": record.amount,
                "printed_as": party_name(record.party.clone()),
                "movement": record.cash_movement,
                "question": question,
                "answers": answers.iter().map(|answer| json!({
                    "answer": answer.as_str(),
                    "text": answer.text(),
                    "ledger_needed": answer.ledger_prompt(),
                    "not_built": answer.not_built(),
                })).collect::<Vec<_>>(),
            }))
        })
        .collect()
}

/// How many lines went to the suspense ledger, by why: a person answered
/// dont_know for a cash line, or a party was unmapped, unidentified or mapped
/// to the suspense ledger. Counts only: a line's own detail stays local.
fn suspense_by_reason(records: &[StatementRecord]) -> Value {
    let suspense = records.iter().filter(|record| record.suspense);
    let answered = suspense
        .clone()
        .filter(|record| record.cash_answer.is_some())
        .count();
    json!({
        "cash_purpose_not_confirmed": answered,
        "party_unmapped_or_mapped_to_suspense": suspense.count() - answered,
    })
}

/// A ledger a person named in answering a bank cash line, which the build
/// checks against the book's groups. A cash-in-hand answer's ledger must reach
/// Cash-in-Hand: a bank ledger there would move the cash bank to bank. No
/// answer but dont_know may name a ledger under the Suspense group: only a
/// dont_know line posted there is tagged "purpose not confirmed" and counted.
pub(super) struct AnsweredCashLedger {
    pub(super) bridge_txn_id: String,
    pub(super) ledger: String,
    pub(super) cash_in_hand: bool,
}

/// `build_import_xml`'s arguments, and what the proposals file requires of the
/// book beyond them.
pub(super) struct ResolvedImport {
    pub(super) args: Value,
    pub(super) cash_ledgers: Vec<AnsweredCashLedger>,
}

/// The voucher type and side an answer puts its named ledger on, for an
/// answer that names one and is built.
fn answered_entry(answer: CashAnswer) -> Option<(&'static str, &'static str)> {
    match answer {
        CashAnswer::BusinessCash => Some(("Contra", "Dr")),
        CashAnswer::OwnerUse => Some(("Payment", "Dr")),
        CashAnswer::CustomerPaidIn | CashAnswer::OwnerBroughtIn => Some(("Receipt", "Cr")),
        CashAnswer::PaidToSomeone
        | CashAnswer::OwnCashBox
        | CashAnswer::UnbookedCashSales
        | CashAnswer::DontKnow => None,
    }
}

/// `rows` that fit `budget` bytes of JSON, in order, and how many did not.
/// A row that does not fit is counted as omitted, never cut, and later
/// smaller rows may still be kept, so the kept rows are not always a prefix.
pub(super) fn bounded(rows: Vec<Value>, budget: &mut usize) -> (Vec<Value>, usize) {
    let total = rows.len();
    let kept = rows
        .into_iter()
        .filter(|row| {
            let cost = serde_json::to_string(row).map_or(usize::MAX, |text| text.len());
            let affordable = cost <= *budget;
            if affordable {
                *budget -= cost;
            }
            affordable
        })
        .collect::<Vec<_>>();
    let omitted = total - kept.len();
    (kept, omitted)
}

/// A proposals file this tool published, read and checked, for every caller that
/// reads one (`build_import_xml` and `statement_tie_out`).
///
/// It is named by a well-formed `proposals_id`, under the data directory's
/// `bank-statements/`, a regular file owned by this user with a single link,
/// carrying the declared schema and its own id, and hashing to
/// `proposals_sha256` -- the digest the parse returned. A file edited or
/// replaced since is refused (`proposals_changed`), so what a caller reads is
/// exactly what the summary described.
pub(super) fn load_proposals(
    data_dir: &Path,
    proposals_id: &Value,
    proposals_sha256: Option<&Value>,
) -> Result<Value, String> {
    let proposals_id = proposals_id
        .as_str()
        .filter(|id| {
            id.strip_prefix("statement-")
                .is_some_and(catalog::is_uuid_v4_lowercase)
        })
        .ok_or_else(|| "argument_invalid:proposals_id".to_string())?;
    let expected_sha256 = proposals_sha256
        .ok_or_else(|| "proposals_sha256_required".to_string())?
        .as_str()
        .filter(|digest| {
            digest.len() == 64
                && digest
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        })
        .ok_or_else(|| "argument_invalid:proposals_sha256".to_string())?;
    let path = data_dir
        .join(PROPOSALS_DIRECTORY)
        .join(format!("{proposals_id}.json"));
    let file = local_file::open_local_file(&path, false).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            "proposals_not_found".to_string()
        } else {
            "proposals_file_unreadable".to_string()
        }
    })?;
    let mut bytes = Vec::new();
    file.take(MAX_PROPOSALS_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "proposals_file_unreadable".to_string())?;
    if bytes.len() as u64 > MAX_PROPOSALS_FILE_BYTES {
        return Err("proposals_file_too_large".into());
    }
    if sha256_hex(&bytes) != expected_sha256 {
        return Err("proposals_changed".into());
    }
    let document: Value =
        serde_json::from_slice(&bytes).map_err(|_| "proposals_file_invalid".to_string())?;
    if document["schema"] != PROPOSALS_SCHEMA || document["proposals_id"] != proposals_id {
        return Err("proposals_file_invalid".into());
    }
    Ok(document)
}

/// `build_import_xml`'s arguments with a proposals file resolved into
/// `vouchers`, or the arguments unchanged when none is named.
///
/// The file must be one [`load_proposals`] admits, so the batch is exactly what
/// the parse's summary described.
pub(super) fn resolve_import_arguments(
    data_dir: &Path,
    args: &Value,
) -> Result<ResolvedImport, String> {
    let Some(object) = args.as_object() else {
        return Err("argument_schema_invalid".into());
    };
    let Some(proposals_id) = object.get("proposals_id") else {
        if object.contains_key("proposals_sha256") {
            return Err("proposals_id_required".into());
        }
        if !object.contains_key("vouchers") {
            return Err("vouchers_required".into());
        }
        return Ok(ResolvedImport {
            args: args.clone(),
            cash_ledgers: Vec::new(),
        });
    };
    if object.contains_key("vouchers") {
        return Err("proposals_id_with_vouchers".into());
    }
    let document = load_proposals(data_dir, proposals_id, object.get("proposals_sha256"))?;
    let vouchers = document
        .get("vouchers")
        .filter(|vouchers| vouchers.is_array())
        .ok_or_else(|| "proposals_file_invalid".to_string())?;
    // A cash line nobody answered has no voucher; building the rest would
    // leave it out of the books silently. A file written before cash lines
    // were asked holds them with no answer recorded, and one written by an
    // earlier build may record an answer this version no longer offers: each
    // is refused the same way.
    let records = document["records"]
        .as_array()
        .ok_or_else(|| "proposals_file_invalid".to_string())?;
    if records.iter().any(|record| {
        record["disposition"] == "needs_answer"
            || (record["party"]
                .as_str()
                .and_then(CashMovement::of_party)
                .is_some()
                && record["cash_answer"]
                    .as_str()
                    .and_then(CashAnswer::parse)
                    .is_none())
    }) {
        return Err("cash_questions_open".into());
    }
    // Every ledger a person named in an answer, bound to the voucher actually
    // built: that bridge_txn_id's voucher has the type the answer makes, with
    // the named ledger on the side the answer puts it.
    let mut cash_ledgers = Vec::new();
    for record in records {
        let Some(answer) = record["cash_answer"].as_str().and_then(CashAnswer::parse) else {
            continue;
        };
        let Some((voucher_type, side)) = answered_entry(answer) else {
            continue;
        };
        let need = AnsweredCashLedger {
            bridge_txn_id: record["bridge_txn_id"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            ledger: record["ledger"].as_str().unwrap_or_default().to_string(),
            cash_in_hand: answer.names_cash_in_hand(),
        };
        let built = vouchers.as_array().is_some_and(|vouchers| {
            vouchers.iter().any(|voucher| {
                voucher["bridge_txn_id"] == need.bridge_txn_id.as_str()
                    && voucher["voucher_type"] == voucher_type
                    && voucher["entries"].as_array().is_some_and(|entries| {
                        entries.iter().any(|entry| {
                            entry["side"] == side && entry["ledger"] == need.ledger.as_str()
                        })
                    })
            })
        });
        if !built {
            return Err("proposals_file_invalid".into());
        }
        cash_ledgers.push(need);
    }
    let mut resolved = object.clone();
    resolved.remove("proposals_id");
    resolved.remove("proposals_sha256");
    resolved.insert("vouchers".into(), vouchers.clone());
    Ok(ResolvedImport {
        args: Value::Object(resolved),
        cash_ledgers,
    })
}

#[cfg(test)]
#[path = "agent_bank_statement_tests.rs"]
pub(super) mod tests;
