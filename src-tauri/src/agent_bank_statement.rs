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
//! file named by `password_file` (on Unix it must be owned by this user with no group or other permission bits; on Windows it must be owned by this user or Administrators, with an access list that lets no one but this user, SYSTEM and Administrators read, change or re-grant it), held in a zeroizing buffer, handed
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
use bridge_bank_statement::bank::BankReference;
use bridge_bank_statement::cash::{CashAnswer, CashAnswerRow, CashAnswers, CashMovement};
use bridge_bank_statement::csv_edge;
use bridge_bank_statement::date::Date;
use bridge_bank_statement::mapping::{Mapping, MappingRow};
use bridge_bank_statement::money::Controls;
use bridge_bank_statement::pdf::{self, MAX_PDF_BYTES};
use bridge_bank_statement::pipeline::{prepare_text, ParsedStatement, StatementRequest};
use bridge_bank_statement::proposals::StatementRecord;
use bridge_bank_statement::proposals::{
    format_amount, tie_out_window, Direction, Disposition, VoucherType,
};
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
/// The schema before facts: no direction, reversal flag or coverage.
const PROPOSALS_SCHEMA_V1: &str = "bridge.bank_statement.proposals.v1";
/// The schema this tool writes: v1 plus `coverage` and, on each record, its
/// `direction` and `reversed_same_day`.
const PROPOSALS_SCHEMA: &str = "bridge.bank_statement.proposals.v2";
/// How a proposals file's tie-out edges were worked out: from the statement's
/// rows, by transaction date.
pub(super) const TIE_OUT_BASIS: &str = "statement_rows_by_transaction_date";
/// Larger than any admissible file: 1000 vouchers at the text limits.
const MAX_PROPOSALS_FILE_BYTES: u64 = 16 * 1024 * 1024;

pub(super) fn input_schema() -> Value {
    let path = json!({"type":"string","minLength":1,"maxLength":MAX_PATH_CHARS,"pattern":r"\S"});
    let control = json!({"type":"string","minLength":1,"maxLength":64,"pattern":r"\S"});
    let totals_control = json!({"type":"string","minLength":1,"maxLength":64,"pattern":r"\S","description":"Required for sbi and hdfc, whose statements print it. A ubi or bob statement prints no totals: omit both, and every page must then print its Page N of M footer."});
    let ledger = json!({"type":"string","minLength":1,"maxLength":agent_import::MAX_MASTER_NAME_CHARS,"pattern":r"\S"});
    json!({
        "type":"object", "additionalProperties":false,
        "required":["statement_path","password_file","bank","account_label","opening_balance","closing_balance","bank_ledger","suspense_ledger"],
        "properties":{
            "statement_path": path,
            "password_file": path,
            "bank":{"type":"string","enum":["sbi","hdfc","ubi","bob"],"description":"sbi (State Bank of India), hdfc (HDFC Bank), ubi (Union Bank of India) or bob (Bank of Baroda, in the layout measured on two real statements)."},
            "account_label":{"type":"string","minLength":4,"maxLength":64,"pattern":r"\S","description":"A short label carrying at least the last 4 digits of the account, e.g. 'HDFC CA xx4321'. Those digits must end a number on the statement's account-number line. A Bank of Baroda statement in the measured layout prints its number masked (its first and last three digits clear): give the whole number (more than six digits); only those six clear digits are compared. The label is written into each narration."},
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

pub(super) const DESCRIPTION: &str = "Read a local SBI, HDFC, Union Bank of India or Bank of Baroda bank-statement PDF (password-protected; a statement that opens without a password needs password_file to be an empty file) and propose one Payment, Receipt or Contra per row (none for a cash line not yet answered), for build_import_xml's voucher shape. The whole run is refused unless the statement's account-number line ends with the digits in account_label (a Bank of Baroda number is masked: see account_label), and every row's running balance, the closing balance, and (where the statement prints them) the debit and credit totals reproduce the figures supplied exactly. A Union Bank statement, and a Bank of Baroda statement in the measured layout, print no opening or closing balance: take both balances from the books or the bank, since the statement cannot confirm them, and give an overdrawn (Dr) balance as a negative number (-16,141.16). With no printed totals (a Union Bank or Bank of Baroda statement), those two balances and a Page N of M footer on every page are the only checks on which rows were read, and missing rows whose amounts net to zero would not be seen. A Bank of Baroda statement is read only if page 1 prints an IFSC Code with the bank's prefix on the line that carries the words IFSC Code:, and otherwise is refused as statement_bank_not_recognised (the statements print the bank's name only in an image, so the code is the one thing in the text that names the bank). It prints its newest row first (a statement printed the other way round is refused when its dates show it) and cuts a narration at about 50 characters, so a party can end mid-word and two payers whose names agree up to the cut read as one party; rows are numbered oldest first, as the balance replay reads them. Text in render mode 3 (invisible), which a scanner app writes as its OCR layer under the page image, is not read. A PDF from which no figures can be read (a scan, a printed copy or an image has none), or whose text is mostly in that mode with no statement row in the rest, is refused as statement_no_readable_text: ask for the statement downloaded from net banking as a text PDF. A character that draws nothing (Unicode Default_Ignorable_Code_Point or Format, except ZWJ, ZWNJ and the prepended concatenation marks) is not read from the PDF; a soft hyphen is read as a hyphen. The password is read from password_file, a local file, and is never returned. A password file that breaks password_file's ownership and permission rules is refused as statement_password_file_permissions, whose next step says how to make the file private. Full proposals stay in a private local file; the result is a counterparty summary (spelling as printed, row count, disposition, suspense) for writing `mapping`, and the ledger names to check with validate_masters. A party the mapping does not name, or the parser could not identify, goes to suspense_ledger, tagged UNIDENTIFIED; a voucher posted to a mapped ledger carries ` | Statement party: <name>`, the counterparty as the statement printed it, at the end of its narration text (a file imported by hand adds the `[BRIDGE:...]` tag after it; a party containing |, carrying the reserved [BRIDGE: marker or making the narration over 2000 characters is refused, party_not_admissible: skip it or map it to the suspense ledger; the parser's own labels such as BANK CHARGES and a cash movement are not written as a party); `skip` omits a transfer already carried by another account's Contra, and its mapping row must name the other account's ledger, so that Contra can be found; build_import_xml records a skipped row and lists it in a transfers section, by id and date, with its state and what to do, until the other account's Contra is built and verified through ComplyEaze Bridge or, for a skip a person withdrew, a batch holding the row is found posted; a skipped row that a batch found posted also posts, with no withdrawal named, stays listed as posted_against_skip, and a row with an equal amount going the other way the same day on its own statement is listed as reversed_same_day when no Contra with an equal network reference could carry it, because a reversal and a pair of real transfers look alike: it is paired only by an equal network reference, and the person checks in Tally which rows are transfers (a skip with no ledger is refused, skip_without_ledger, and so is one naming bank_ledger itself, skip_ledger_is_statement_bank, or suspense_ledger, skip_ledger_is_suspense). Only SBI 'ATM WDL' withdrawals and Union Bank and Bank of Baroda 'BY CASH' deposits are recognised as cash. Other cash text is not: where the parser names a party it is an ordinary party, and where it cannot (as for SBI deposits and HDFC cash text) it goes to the UNIDENTIFIED fallback. A recognised cash line is never mapped or defaulted: it is returned in cash_questions with its question and answers, and build_import_xml refuses the proposals (cash_questions_open) until each is answered in cash_answers. You may ask the person once about all the open cash lines, listing each line's date and amount (cash_questions carries them, with the movement, a withdrawal or a deposit), then give each line its own answer and ask again about any line the person did not confirm: one answer is never applied to another line for them. Only a dont_know answer posts one to suspense_ledger, tagged \"Bridge: purpose not confirmed; reclassify\"; every line sent to suspense is counted (suspense_rows, and suspense_by_reason by why), and suspense parties are listed first in counterparties, within its bound, with their rows; a line's own date, amount and label stay in the local file. The result carries no amount except the figures the caller supplied, echoed in reconciled (closing_balance, and total_debits and total_credits where given), and an open cash line's amount (below). Arithmetic on the caller's own inputs can still give a row's amount, for example total_debits minus the open cash amounts when one other debit row remains, and when a direction has a single row the caller's own total is that row's amount and counterparties names its party; and a from/to window holding one row shows through rows_in_window that a row falls in it, and through counterparties that row's party and direction; no result can hide what follows from the caller's inputs. An open cash line is the one exception: its cash_questions entry carries its bridge_txn_id, date and amount, its party name (masked when BRIDGE_AGENT_REDACTION is mask_parties) and whether it is a withdrawal or a deposit, so the person can tell which line is asked about. No other value of any row is returned. A file parsed for the whole statement, or for whole calendar months (from the 1st to a month's last day) within its rows, can be checked with statement_tie_out, whose gaps together with the book's own figures give the statement's balance at those edges and its net over those months: totals per calendar month, never a part of a month except the part-months at the statement's own start and end, whose net a month edge's balance gives together with the opening and closing you supplied; though a month in which every other row's amount is already known (one row alone, or one row beside open cash lines) gives that row's amount. statement_tie_out compares no other narrowed window (window_not_whole_calendar_months). Every list in the result is bounded by the response size and counts what it left out (cash_questions_omitted, counterparties_omitted, ledgers_to_validate_omitted); cash_questions_open, suspense_rows and skipped count them all. An answer for a row outside from/to is refused (cash_answer_outside_window). An ambiguous mapping is refused, never guessed. Re-run with a corrected mapping: bridge_txn_id labels depend only on the statement row, so they do not change. build_import_xml builds the whole file, so from and to choose the batch. Import a statement in whole-day batches: cut a batch only between days, at most 50 vouchers (the most one batch post takes), because a same-day row with the same type, ledgers and amounts that an earlier batch posted refuses the whole later batch as import_preexisting_identity; from and to are days, so a window never cuts one, and a day with more than 50 rows does not fit one batch, an open limit: ask the user. With posting on, a saved batch of up to 50 posts after one approval, so same-day twins post together in one batch or through a hand import of the built file. To build, pass the returned proposals_id and sha256 to build_import_xml as proposals_id and proposals_sha256; to correct a batch already built from an earlier run, add amends_batch_id. Never contacts Tally.";

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
            let date = normalized_date(&text)?;
            let compact = date.as_str();
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
/// Refused unless it is a regular file with a single link that is not itself a link (a symbolic link
/// on Unix, a reparse point on Windows), and nobody else can read it. On Unix it must also be owned
/// by this user and carry no group or other permission bits. On Windows its owner must be this user
/// or Administrators, and its access list must let no one but this user, SYSTEM and Administrators
/// read, change or re-grant it.
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
    #[cfg(windows)]
    match crate::local_files::access::private_to_this_user(&file) {
        Ok(Ok(())) => {}
        Ok(Err(_)) => return Err("statement_password_file_permissions".into()),
        Err(_) => return Err("statement_password_file_unreadable".into()),
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

fn read_statement(path: &LocalDiskPath, limit: usize) -> Result<Vec<u8>, String> {
    let file = local_file::open_local_file(path.as_path(), false)
        .map_err(|_| "statement_file_unreadable".to_string())?;
    let limit = limit as u64;
    let mut bytes = Vec::new();
    file.take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "statement_file_unreadable".to_string())?;
    if bytes.len() as u64 > limit {
        return Err("statement_file_too_large".into());
    }
    Ok(bytes)
}

/// Where the bundled PDFium library is: beside this executable. This crate's
/// own unit tests may name another by an absolute path in
/// `BRIDGE_PDFIUM_LIBRARY`; any other build has no such lookup, which
/// `scripts/check-no-test-seam.mjs` checks on the executables the Tauri
/// bundle hook, CI's bundle job and the release workflow produce.
fn pdfium_library() -> Result<PathBuf, String> {
    #[cfg(test)]
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
    let bytes = read_statement(&request.statement_path, MAX_PDF_BYTES)?;
    let text = {
        let password = read_password(&request.password_file)?;
        let engine = pdf::engine(&pdfium_library()?).map_err(|refusal| refused(&refusal))?;
        pdf::extract_pages(engine, &bytes, &password).map_err(|refusal| refused(&refusal))?
    };
    let parsed = prepare_text(
        &text,
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
        "coverage": parsed.coverage.map(|(from, to)| json!({"from": from.iso(), "to": to.iso()})),
        "vouchers": parsed.build.proposals,
        "records": parsed.build.records,
    });
    // Optional, and not part of the schema's required shape: the loader reads
    // fields by name, so a reader of this schema version still reads a file that
    // carries it. An older binary refuses the version itself, so going back to it
    // needs a fresh parse.
    // The statement's own first and last row dates, whether this parse's
    // from/to window left any row out, and where statement_tie_out may compare
    // it with the book. None of it is returned in the summary.
    if let Some((first, last)) = parsed.build.span {
        let tie_out = match tie_out_window(
            &parsed.build,
            &request.controls.opening,
            &request.controls.closing,
            request.date_from,
            request.date_to,
        ) {
            Some(Ok(Ok(edges))) => json!({
                "state": "established",
                "first_date": edges.first().iso(),
                "last_date": edges.last().iso(),
                "opening_balance": format_amount(edges.opening()),
                "closing_balance": format_amount(edges.closing()),
                "basis": TIE_OUT_BASIS,
            }),
            Some(Ok(Err(reason))) => json!({"state": "not_established", "reason": reason.code()}),
            Some(Err(refusal)) => return Err(refused(&refusal)),
            None => Value::Null,
        };
        document["window"] = json!({
            "first_row_date": first.iso(),
            "last_row_date": last.iso(),
            "whole_statement": parsed.build.rows_outside_window == 0,
            "tie_out": tie_out,
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
            // A skipped row posts nothing, but its ledger is the other bank
            // account's, under which its Contra is looked for.
            .chain(
                records
                    .iter()
                    .filter(|record| record.disposition == Disposition::Skipped)
                    .map(|record| record.ledger.as_str()),
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
        "Ask the person about the open cash_questions entries in their own words (once for all of them is fine, listing each line's date and amount), ask again about any line they do not confirm, and re-run with their answers in cash_answers (one per line, by bridge_txn_id, with the ledger the answer asks for). Never answer for them: dont_know is an answer they give, and it posts the line to suspense_ledger for the CA to move. build_import_xml refuses these proposals (cash_questions_open) while any question is open."
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
    /// The two ledgers the parse was given, when the file names both.
    pub(super) statement_ledgers: Option<StatementLedgers>,
    /// What the statement's own file says of its Contras and skipped rows;
    /// `None` for inline vouchers and for a file written before it did.
    pub(super) statement: Option<ResolvedStatement>,
}

/// A statement's Contras, skipped rows and the days it read in full, read from
/// a version 2 proposals file at the boundary, so a later step relies on the
/// types and checks nothing again.
pub(super) struct ResolvedStatement {
    pub(super) bank_ledger: String,
    pub(super) coverage: Option<(bridge_tally_core::TallyDate, bridge_tally_core::TallyDate)>,
    /// One per Contra voucher the file proposes, cash-answer Contras included.
    pub(super) contras: Vec<StatementContra>,
    /// One per row the person skipped as recorded from the other bank's
    /// statement.
    pub(super) skips: Vec<StatementSkip>,
    /// The file proposes no voucher.
    posts_nothing: bool,
}

pub(super) struct StatementContra {
    pub(super) bridge_txn_id: String,
    pub(super) reference: Option<BankReference>,
    pub(super) instrument: Option<String>,
    pub(super) reversed_same_day: bool,
}

pub(super) struct StatementSkip {
    pub(super) bridge_txn_id: String,
    pub(super) date: bridge_tally_core::TallyDate,
    pub(super) amount: String,
    pub(super) direction: Direction,
    pub(super) other_ledger: String,
    pub(super) reference: Option<BankReference>,
    pub(super) instrument: Option<String>,
    pub(super) reversed_same_day: bool,
}

impl ResolvedStatement {
    /// A statement whose every row was skipped: it proposes no voucher and
    /// holds at least one skipped row, and so is recorded without a batch.
    pub(super) fn is_skip_only(&self) -> bool {
        self.posts_nothing && !self.skips.is_empty()
    }

    /// The ledgers the facts name beyond the vouchers': the statement's bank
    /// and each skipped row's other ledger, each once, in name order.
    pub(super) fn ledger_names(&self) -> Vec<String> {
        std::iter::once(&self.bank_ledger)
            .chain(self.skips.iter().map(|skip| &skip.other_ledger))
            .cloned()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}

/// A record of a version 2 file, as far as the facts need it. Only a skipped
/// row reads the fields after the first four.
#[derive(serde::Deserialize)]
struct FileRecord {
    bridge_txn_id: String,
    disposition: Disposition,
    direction: Direction,
    reversed_same_day: bool,
    #[serde(default)]
    date: Option<String>,
    #[serde(default)]
    amount: Option<String>,
    #[serde(default)]
    ledger: Option<String>,
    #[serde(default)]
    reference: Option<String>,
    #[serde(default)]
    instrument: Option<String>,
}

#[derive(serde::Deserialize)]
struct FileCoverage {
    from: String,
    to: String,
}

/// The facts a version 2 file carries. Any part that is missing or not of the
/// shape the parse writes makes the file invalid.
fn read_statement_facts(
    document: &Value,
    records: &[Value],
    posts_nothing: bool,
) -> Result<ResolvedStatement, String> {
    let invalid = || "proposals_file_invalid".to_string();
    let bank_ledger = document["bank_ledger"].as_str().ok_or_else(invalid)?;
    let date = |text: &str| normalized_date(text).map_err(|_| invalid());
    // `coverage` is always written, null when no day was read in full
    let coverage = match document.get("coverage").ok_or_else(invalid)? {
        Value::Null => None,
        value => {
            let window: FileCoverage =
                serde_json::from_value(value.clone()).map_err(|_| invalid())?;
            Some((date(&window.from)?, date(&window.to)?))
        }
    };
    let reference = |text: Option<String>| {
        text.map(|text| BankReference::from_recorded(&text).ok_or_else(invalid))
            .transpose()
    };
    let mut contras = Vec::new();
    let mut skips = Vec::new();
    for record in records {
        let record: FileRecord = serde_json::from_value(record.clone()).map_err(|_| invalid())?;
        match record.disposition {
            Disposition::Voucher(VoucherType::Contra) => contras.push(StatementContra {
                bridge_txn_id: record.bridge_txn_id,
                reference: reference(record.reference)?,
                instrument: record.instrument,
                reversed_same_day: record.reversed_same_day,
            }),
            Disposition::Skipped => skips.push(StatementSkip {
                bridge_txn_id: record.bridge_txn_id,
                date: date(record.date.as_deref().ok_or_else(invalid)?)?,
                amount: record.amount.ok_or_else(invalid)?,
                direction: record.direction,
                other_ledger: record.ledger.ok_or_else(invalid)?,
                reference: reference(record.reference)?,
                instrument: record.instrument,
                reversed_same_day: record.reversed_same_day,
            }),
            Disposition::Voucher(_) | Disposition::NeedsAnswer => {}
        }
    }
    Ok(ResolvedStatement {
        bank_ledger: bank_ledger.to_string(),
        coverage,
        contras,
        skips,
        posts_nothing,
    })
}

/// The bank and suspense ledgers a proposals file was parsed for.
pub(super) struct StatementLedgers {
    pub(super) bank_ledger: String,
    pub(super) suspense_ledger: String,
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
/// `bank-statements/`, a regular file with a single link (owned by this user
/// on Unix), carrying the declared schema and its own id, and hashing to
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
    // a version 1 file is read too: what a caller may build from one is the
    // caller's to decide
    if !matches!(
        document["schema"].as_str(),
        Some(PROPOSALS_SCHEMA | PROPOSALS_SCHEMA_V1)
    ) || document["proposals_id"] != proposals_id
    {
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
            statement_ledgers: None,
            statement: None,
        });
    };
    if object.contains_key("vouchers") {
        return Err("proposals_id_with_vouchers".into());
    }
    let document = load_proposals(data_dir, proposals_id, object.get("proposals_sha256"))?;
    let legacy = document["schema"] == PROPOSALS_SCHEMA_V1;
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
    // A file written before facts existed cannot say which rows were skipped or
    // which Contras it holds. Built anyway, its Contras would be invisible to
    // the transfer checks and its skips unrecorded, so it is parsed again.
    if legacy
        && (records
            .iter()
            .any(|record| record["disposition"] == "skipped")
            || vouchers.as_array().is_some_and(|vouchers| {
                vouchers
                    .iter()
                    .any(|voucher| voucher["voucher_type"] == "Contra")
            }))
    {
        return Err("proposals_legacy".into());
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
    // A file that names neither ledger is not held to a rule about them.
    let statement_ledgers = document["bank_ledger"]
        .as_str()
        .zip(document["suspense_ledger"].as_str())
        .map(|(bank_ledger, suspense_ledger)| StatementLedgers {
            bank_ledger: bank_ledger.to_string(),
            suspense_ledger: suspense_ledger.to_string(),
        });
    let statement = if legacy {
        None
    } else {
        let posts_nothing = vouchers.as_array().is_some_and(Vec::is_empty);
        Some(read_statement_facts(&document, records, posts_nothing)?)
    };
    let mut resolved = object.clone();
    resolved.remove("proposals_id");
    resolved.remove("proposals_sha256");
    resolved.insert("vouchers".into(), vouchers.clone());
    Ok(ResolvedImport {
        args: Value::Object(resolved),
        cash_ledgers,
        statement_ledgers,
        statement,
    })
}

/// Where the opening and closing figures a CSV statement is checked against
/// come from (#1320). The caller says; nothing here can verify it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ControlsSource {
    PriorStatement,
    BankLedgerInTally,
    ThisFile,
}

impl ControlsSource {
    fn parse(text: &str) -> Option<Self> {
        match text {
            "prior_statement" => Some(Self::PriorStatement),
            "bank_ledger_in_tally" => Some(Self::BankLedgerInTally),
            "this_file" => Some(Self::ThisFile),
            _ => None,
        }
    }

    /// Only the file's own first and last balance is not an independent
    /// control: a chain that starts and ends on them always agrees with itself,
    /// so a truncated export would pass.
    fn independence_label(self) -> Option<&'static str> {
        (self == Self::ThisFile).then_some("this_file: derived, not independent")
    }
}

macro_rules! csv_refusals {
    ($($variant:ident = $category:literal),+ $(,)?) => {
        /// The refusal categories of `csv_edge::read_statement`, one variant
        /// each. A category outside this list is not forwarded: it is refused
        /// as unmapped, so a new crate category cannot pass as a known one.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub(super) enum CsvRefusal { $($variant),+ }

        impl CsvRefusal {
            #[cfg(test)]
            pub(super) const ALL: &'static [Self] = &[$(Self::$variant),+];

            pub(super) fn category(self) -> &'static str {
                match self { $(Self::$variant => $category),+ }
            }

            fn from_category(category: &str) -> Option<Self> {
                match category { $($category => Some(Self::$variant),)+ _ => None }
            }
        }
    };
}

csv_refusals! {
    FileTooLarge = "file_too_large",
    EncodingUnsupported = "encoding_unsupported",
    LayoutUnknown = "layout_unknown",
    MalformedRow = "malformed_row",
    SerialBroken = "serial_broken",
    UnparseableDate = "unparseable_date",
    DateOrderBroken = "date_order_broken",
    MalformedAmount = "malformed_amount",
    MalformedBalance = "malformed_balance",
    DirectionUnknown = "direction_unknown",
    ClosingBalanceUnreadable = "closing_balance_unreadable",
    ClosingBalanceDiffers = "closing_balance_differs",
    TrailerMissing = "trailer_missing",
    TrailerUnexpected = "trailer_unexpected",
    EmptyStatement = "empty_statement",
}

/// A file that reads is still refused: whether the real export prints an
/// account number is not established, so nothing can bind the file to an
/// account and nothing is proposed (#1320).
pub(super) const NO_ACCOUNT_NUMBER_LINE: &str = "statement_no_account_number_line";
const CSV_REFUSAL_UNMAPPED: &str = "statement_csv_refusal_unmapped";

pub(super) fn input_schema_csv() -> Value {
    json!({
        "type":"object", "additionalProperties":false,
        "required":["statement_path","controls_source"],
        "properties":{
            "statement_path":{"type":"string","minLength":1,"maxLength":MAX_PATH_CHARS,"pattern":r"\S","description":"The path of the CSV bank statement on this computer."},
            "controls_source":{"type":"string","enum":["prior_statement","bank_ledger_in_tally","this_file"],"description":"Where the opening and closing figures to check the file against come from: the previous statement, the bank ledger in Tally, or this file itself. A Tally ledger built from these same statements counts as this_file. Not verified here."}
        }
    })
}

pub(super) const DESCRIPTION_CSV: &str = "Read a local CSV bank statement in the one layout measured, a private bank's net-banking export (UTF-8, LF line ends, every field double-quoted), and check its shape: the header line, the Sl. No. run, the dates, the amounts, the direction cells and the Closing Balance line, which must equal the last row's balance. Any other file is refused, never guessed: the error code is statement_<category>, with :row_N when a row is at fault (one of fifteen shape categories, such as layout_unknown or closing_balance_differs). controls_source (prior_statement, bank_ledger_in_tally or this_file) says where the figures to check against come from; this_file is derived, not independent, because a chain that starts and ends on the file's own balances always agrees, so a truncated export would pass. This tool proposes nothing yet: whether the export prints an account number is not established, so a file that reads is refused as statement_no_account_number_line (with this_file the refusal's cause reads 'derived, not independent'). No row value is returned.";

/// The admitted arguments of `parse_bank_statement_csv`.
struct CsvRequest {
    statement_path: LocalDiskPath,
    controls_source: ControlsSource,
}

impl CsvRequest {
    fn from_args(args: &Value) -> Result<Self, String> {
        let controls_source = ControlsSource::parse(required_string(args, "controls_source")?)
            .ok_or_else(|| "argument_invalid:controls_source".to_string())?;
        Ok(Self {
            statement_path: local_disk_path(args, "statement_path")?,
            controls_source,
        })
    }
}

fn csv_failure(refusal: &Refusal) -> ToolFailure {
    let code = match (CsvRefusal::from_category(refusal.category), refusal.row) {
        (None, _) => CSV_REFUSAL_UNMAPPED.to_string(),
        (Some(kind), Some(row)) => format!("statement_{}:row_{row}", kind.category()),
        (Some(kind), None) => format!("statement_{}", kind.category()),
    };
    code.into()
}

/// Every run ends in a refusal until the account line exists, so the result is
/// the failure itself, not a `Result` with a success nothing can build.
fn run_csv(request: &CsvRequest) -> ToolFailure {
    let bytes = match read_statement(&request.statement_path, csv_edge::MAX_FILE_BYTES) {
        Ok(bytes) => bytes,
        Err(code) => return code.into(),
    };
    match csv_edge::read_statement(&bytes) {
        Err(refusal) => csv_failure(&refusal),
        Ok(_) => ToolFailure {
            cause: request.controls_source.independence_label(),
            ..NO_ACCOUNT_NUMBER_LINE.to_string().into()
        },
    }
}

impl Server {
    pub(super) async fn parse_bank_statement_csv(
        &self,
        args: &Value,
    ) -> Result<ToolOutcome, ToolFailure> {
        let request = CsvRequest::from_args(args)?;
        Err(tokio::task::spawn_blocking(move || run_csv(&request))
            .await
            .unwrap_or_else(|_| "statement_task_failed".to_string().into()))
    }
}

#[cfg(test)]
#[path = "agent_bank_statement_tests.rs"]
pub(super) mod tests;

#[cfg(test)]
#[path = "agent_bank_statement_csv_tests.rs"]
mod csv_tests;
