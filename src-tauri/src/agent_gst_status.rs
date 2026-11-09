//! The GSTR-1 status of the vouchers of one type in a date window (R7).
//!
//! Tally keeps, on each voucher, whether its GSTR-1 lists it as included,
//! uncertain or excluded, and whether a person accepted it as it stands. The
//! lab's TallyPrime 7.1 Silver book (10 Oct 2026, synthetic) returned those
//! four fields on a short field list (seven since #1664), with the voucher type selected by its
//! name in the same request, at about 3 KB a voucher (protocol reference
//! §9.17). This tool reads them and says, per voucher, which state it is in.
//! It reads whether the voucher is cancelled, optional or post-dated as well,
//! and decides that first: a cancelled Sales voucher reads the included flag
//! Yes, an optional one reads all three No, and one marked post-dated by hand
//! and dated the day Tally's current date had been set to reads the included
//! flag Yes (lab, 12 Oct 2026, one voucher each).
//! It reads nothing it does not report: Tally's reason for a status (a tax
//! mismatch, a missing HSN/SAC) is on its screen only and no field carries it.
use super::*;
use quick_xml::events::Event;
use std::collections::BTreeMap;

/// The largest voucher mark (an upper bound on the book's vouchers) the status
/// read is made on: the type and the window are decided by a formula that
/// Tally evaluates on every voucher of the book. PROVISIONAL, as for the invoice
/// build: such reads have been timed in seconds on books of a few thousand
/// vouchers and this read on one of about thirty.
const STATUS_MAX_VOUCHER_MARK: u64 = 25_000;

/// The longest window, in days, one read covers.
const STATUS_MAX_WINDOW_DAYS: i64 = 93;

/// The fields a voucher's state is read from: one below a direct child of the
/// voucher is not the voucher's own.
const STATUS_FIELDS: [&str; 9] = [
    "VCHGSTSTATUSISINCLUDED",
    "VCHGSTSTATUSISUNCERTAIN",
    "VCHGSTSTATUSISEXCLUDED",
    "ISGSTOVERRIDDEN",
    "ISCANCELLED",
    "ISOPTIONAL",
    "ISPOSTDATED",
    "ISDELETED",
    "ISDELETEDVCHRETAINED",
];

/// What a voucher's three status flags say. Tally answers exactly one flag Yes
/// for a voucher the return lists; a voucher that is not a GSTR-1 document (the
/// lab's Receipts) answers all three No. Anything else is not read.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum GstStatus {
    Included,
    Uncertain,
    Excluded,
    /// All three flags No: the voucher is not a document of the return.
    NotInReturn,
    /// Tally's `ISCANCELLED` Yes, whatever measured state its three flags state.
    Cancelled,
    /// Tally's `ISOPTIONAL` Yes, whatever measured state its three flags state.
    Optional,
    /// Tally's `ISPOSTDATED` Yes, whatever measured state its three flags state.
    PostDated,
}

/// What the three status flags alone state: the four measured combinations, and
/// never the three states that come from `ISCANCELLED`, `ISOPTIONAL` and
/// `ISPOSTDATED`. A caller that reads the three flags only holds this type, so
/// it cannot be handed a state it did not read.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum FlagStatus {
    Included,
    Uncertain,
    Excluded,
    NotInReturn,
}

impl From<FlagStatus> for GstStatus {
    fn from(flags: FlagStatus) -> Self {
        match flags {
            FlagStatus::Included => GstStatus::Included,
            FlagStatus::Uncertain => GstStatus::Uncertain,
            FlagStatus::Excluded => GstStatus::Excluded,
            FlagStatus::NotInReturn => GstStatus::NotInReturn,
        }
    }
}

/// The state three flags state, or the code of why they state none: absent
/// (`gst_status_not_reported`), or not one of the four measured combinations
/// (`gst_status_unreadable`). A flag that is empty, repeated, spelled another
/// way or missing while another is present is never read as No.
pub(super) fn status_of_flags(
    included: Option<&str>,
    uncertain: Option<&str>,
    excluded: Option<&str>,
) -> Result<FlagStatus, &'static str> {
    match (included, uncertain, excluded) {
        (Some("Yes"), Some("No"), Some("No")) => Ok(FlagStatus::Included),
        (Some("No"), Some("Yes"), Some("No")) => Ok(FlagStatus::Uncertain),
        (Some("No"), Some("No"), Some("Yes")) => Ok(FlagStatus::Excluded),
        (Some("No"), Some("No"), Some("No")) => Ok(FlagStatus::NotInReturn),
        (None, None, None) => Err("gst_status_not_reported"),
        _ => Err("gst_status_unreadable"),
    }
}

/// The state of a voucher: cancelled, optional or post-dated first, then its
/// three status flags. Such a voucher is so whatever measured state its flags
/// state (the lab's cancelled and post-dated vouchers read included and its
/// optional one reads none), and is unread when they state none: an answer that
/// cannot be read is not trusted for the three flags beside it either. A
/// voucher that reads more than one of the three Yes (never measured), or one
/// of them absent or spelled another way, has no state; all three absent is not
/// reported.
pub(super) fn status_of_voucher(
    cancelled: Option<&str>,
    optional: Option<&str>,
    post_dated: Option<&str>,
    included: Option<&str>,
    uncertain: Option<&str>,
    excluded: Option<&str>,
) -> Result<GstStatus, &'static str> {
    let flags = status_of_flags(included, uncertain, excluded);
    match (cancelled, optional, post_dated) {
        (Some("Yes"), Some("No"), Some("No")) => flags.map(|_| GstStatus::Cancelled),
        (Some("No"), Some("Yes"), Some("No")) => flags.map(|_| GstStatus::Optional),
        (Some("No"), Some("No"), Some("Yes")) => flags.map(|_| GstStatus::PostDated),
        (Some("No"), Some("No"), Some("No")) => flags.map(GstStatus::from),
        (None, None, None) => Err("gst_status_not_reported"),
        _ => Err("gst_status_unreadable"),
    }
}

/// Refuses a state to a voucher Tally marks deleted. `ISDELETED` and
/// `ISDELETEDVCHRETAINED` come back without being asked for (No on every
/// voucher the lab read) and a Yes was never measured, so a voucher that reads
/// one has no state (`gst_status_voucher_deleted`). Both absent is allowed: the
/// request does not name them. One absent beside the other is never No.
pub(super) fn not_deleted(
    deleted: Option<&str>,
    retained: Option<&str>,
) -> Result<(), &'static str> {
    match (deleted, retained) {
        (None, None) | (Some("No"), Some("No")) => Ok(()),
        (Some("No" | "Yes"), Some("No" | "Yes")) => Err("gst_status_voucher_deleted"),
        _ => Err("gst_status_unreadable"),
    }
}

/// Whether a person accepted the voucher as it stands (Accept As Is): `Yes` or
/// `No` only.
pub(super) fn overridden_of(flag: Option<&str>) -> Result<bool, &'static str> {
    match flag {
        Some("Yes") => Ok(true),
        Some("No") => Ok(false),
        None => Err("gst_status_not_reported"),
        Some(_) => Err("gst_status_unreadable"),
    }
}

#[derive(Debug, Default)]
pub(super) struct Node {
    name: String,
    text: String,
    children: Vec<Node>,
}

impl Node {
    pub(super) fn child_texts<'a>(&'a self, name: &'a str) -> impl Iterator<Item = String> + 'a {
        self.children
            .iter()
            .filter(move |child| child.name == name)
            .map(|child| clean(&child.text))
    }

    /// The text of the one direct child named `name`: `Some(None)` when there is
    /// none, `None` when there are several. Nothing below a direct child is read.
    pub(super) fn one(&self, name: &str) -> Option<Option<String>> {
        let mut found = self.child_texts(name);
        let first = found.next();
        match (first, found.next()) {
            (None, _) => Some(None),
            (Some(value), None) => Some(Some(value)),
            (Some(_), Some(_)) => None,
        }
    }

    /// Whether any element below this one, at any depth, is named in `names`.
    fn contains_any(&self, names: &[&str]) -> bool {
        self.children
            .iter()
            .any(|child| names.contains(&child.name.as_str()) || child.contains_any(names))
    }

    /// Whether a status field sits below one of this element's direct
    /// children, at any depth: it is not this element's own.
    fn nests_any(&self, names: &[&str]) -> bool {
        self.children.iter().any(|child| child.contains_any(names))
    }
}

/// A value as Tally means it: its numeric references to control characters
/// (`&#4;`), which `mark_forbidden_numeric_references` turns into the
/// replacement character and `#4;`, and the whitespace around it, are not
/// part of it.
pub(super) fn clean(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('\u{fffd}') {
        out.push_str(&rest[..at]);
        let after = &rest[at + '\u{fffd}'.len_utf8()..];
        let digits = after
            .strip_prefix('#')
            .map(|tail| tail.bytes().take_while(u8::is_ascii_digit).count());
        match digits {
            Some(count) if count > 0 && after[1 + count..].starts_with(';') => {
                rest = &after[1 + count + 1..];
            }
            _ => {
                out.push('\u{fffd}');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out.trim_matches(|c: char| c.is_whitespace() || c.is_control())
        .to_string()
}

fn parse_tree(xml: &str) -> Result<Node, &'static str> {
    let invalid = "gst_status_read_protocol_invalid";
    let marked = mark_agent_xml(xml);
    let xml = marked.as_ref();
    validate_agent_envelope(xml).map_err(|_| invalid)?;
    let mut reader = quick_xml::Reader::from_str(xml);
    let mut stack: Vec<Node> = vec![Node::default()];
    loop {
        match reader.read_event().map_err(|_| invalid)? {
            Event::Start(start) => stack.push(Node {
                name: start.name().as_ref().to_string(),
                ..Node::default()
            }),
            Event::Empty(empty) => stack.last_mut().ok_or(invalid)?.children.push(Node {
                name: empty.name().as_ref().to_string(),
                ..Node::default()
            }),
            Event::Text(text) => {
                let text = decoded_agent_text(text).map_err(|_| invalid)?;
                stack.last_mut().ok_or(invalid)?.text.push_str(&text);
            }
            Event::GeneralRef(reference) => {
                let text = decoded_agent_reference(reference).map_err(|_| invalid)?;
                stack.last_mut().ok_or(invalid)?.text.push_str(&text);
            }
            Event::CData(text) => stack.last_mut().ok_or(invalid)?.text.push_str(&text),
            Event::End(_) => {
                let node = stack.pop().ok_or(invalid)?;
                stack.last_mut().ok_or(invalid)?.children.push(node);
            }
            Event::Eof => break,
            _ => {}
        }
    }
    let [root] = <[Node; 1]>::try_from(stack).map_err(|_| invalid)?;
    let [envelope] = <[Node; 1]>::try_from(root.children).map_err(|_| invalid)?;
    if envelope.name != "ENVELOPE" {
        return Err(invalid);
    }
    // The status of the answer was checked with the envelope above.
    Ok(envelope)
}

/// One voucher of the answer.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct StatusRow {
    pub(super) voucher_number: String,
    pub(super) date: String,
    pub(super) voucher_type: String,
    pub(super) master_id: String,
    pub(super) alter_id: String,
    /// The state the flags state, or the code of why they state none.
    pub(super) status: Result<GstStatus, &'static str>,
    /// What the three status flags read on a cancelled, optional or post-dated
    /// voucher: returned beside its state and never counted.
    pub(super) status_flags: Option<FlagStatus>,
    pub(super) overridden: Result<bool, &'static str>,
}

/// The vouchers of an answer, each with its status read closed. A voucher that
/// cannot be told apart from another (a field repeated, absent or two rows of
/// one voucher) refuses the whole answer: nothing attributes a status to it. A
/// voucher whose status fields are absent, repeated, nested in another element
/// or spelled another way keeps its row with the code that says so; it is
/// never read as included. `extra` is read from each voucher's node beside its
/// status: it sees the voucher's direct children and whatever lies below
/// them, and is not part of the nesting check, which covers only the fields
/// the status is read from.
pub(super) fn parse_rows_with<T>(
    xml: &str,
    extra: impl Fn(&Node) -> T,
) -> Result<Vec<(StatusRow, T)>, &'static str> {
    let envelope = parse_tree(xml)?;
    let data = envelope
        .children
        .iter()
        .find(|child| child.name == "BODY")
        .and_then(|body| body.children.iter().find(|child| child.name == "DATA"))
        .ok_or("gst_status_read_collection_absent")?;
    let mut collections = data.children.iter().filter(|c| c.name == "COLLECTION");
    let collection = collections
        .next()
        .ok_or("gst_status_read_collection_absent")?;
    // One collection answers the request: a second one, or a voucher outside
    // the collection's direct children, is rows this parse would drop.
    if collections.next().is_some()
        || data.children.iter().any(|c| {
            c.name != "COLLECTION" && (c.name == "VOUCHER" || c.contains_any(&["VOUCHER"]))
        })
        || collection.children.iter().any(|c| {
            c.name != "VOUCHER" && (c.name == "COLLECTION" || c.contains_any(&["VOUCHER"]))
        })
    {
        return Err("gst_status_read_collection_unexpected");
    }
    let mut rows = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for voucher in collection
        .children
        .iter()
        .filter(|child| child.name == "VOUCHER")
    {
        let field = |name: &str| -> Result<String, &'static str> {
            match voucher.one(name) {
                Some(Some(value)) if !value.is_empty() => Ok(value),
                _ => Err("gst_status_read_voucher_unidentified"),
            }
        };
        let master_id = field("MASTERID")?;
        if !seen.insert(master_id.clone()) {
            return Err("gst_status_read_voucher_repeated");
        }
        // A status field nested below a direct child is not the voucher's own.
        let nested = voucher.nests_any(&STATUS_FIELDS);
        // A flag repeated in the voucher leaves every value unread.
        let read = || -> Result<_, &'static str> {
            let flag = |name: &str| voucher.one(name).ok_or("gst_status_unreadable");
            let deleted = (flag("ISDELETED")?, flag("ISDELETEDVCHRETAINED")?);
            let kind = (
                flag("ISCANCELLED")?,
                flag("ISOPTIONAL")?,
                flag("ISPOSTDATED")?,
            );
            let flags = (
                flag("VCHGSTSTATUSISINCLUDED")?,
                flag("VCHGSTSTATUSISUNCERTAIN")?,
                flag("VCHGSTSTATUSISEXCLUDED")?,
            );
            let flags = (flags.0.as_deref(), flags.1.as_deref(), flags.2.as_deref());
            let deleted_check = not_deleted(deleted.0.as_deref(), deleted.1.as_deref());
            let status = deleted_check.and_then(|()| {
                status_of_voucher(
                    kind.0.as_deref(),
                    kind.1.as_deref(),
                    kind.2.as_deref(),
                    flags.0,
                    flags.1,
                    flags.2,
                )
            });
            let status_flags = match status {
                Ok(GstStatus::Cancelled | GstStatus::Optional | GstStatus::PostDated) => {
                    status_of_flags(flags.0, flags.1, flags.2).ok()
                }
                _ => None,
            };
            // A voucher Tally marks deleted has no state, and a Yes on its acceptance flag was
            // never measured on such a voucher: the flag is as unread as the state.
            let overridden = match deleted_check {
                Err(code @ "gst_status_voucher_deleted") => Err(code),
                _ => overridden_of(flag("ISGSTOVERRIDDEN")?.as_deref()),
            };
            Ok((status, status_flags, overridden))
        };
        let (status, status_flags, overridden) = match (nested, read()) {
            (false, Ok(read)) => read,
            _ => (
                Err("gst_status_unreadable"),
                None,
                Err("gst_status_unreadable"),
            ),
        };
        rows.push((
            StatusRow {
                voucher_number: field("VOUCHERNUMBER")?,
                date: field("DATE")?,
                voucher_type: field("VOUCHERTYPENAME")?,
                master_id,
                alter_id: field("ALTERID")?,
                status,
                status_flags,
                overridden,
            },
            extra(voucher),
        ));
    }
    Ok(rows)
}

/// A voucher's state as a result carries it: the state, or `{"unread": code}`.
pub(super) fn gst_status_item_status(row: &StatusRow) -> Value {
    match row.status {
        Ok(status) => json!(status),
        Err(code) => json!({"unread": code}),
    }
}

fn row_json(row: &StatusRow) -> Value {
    let status = gst_status_item_status(row);
    let overridden = match row.overridden {
        Ok(value) => json!(value),
        Err(code) => json!({"unread": code}),
    };
    let mut item = json!({
        "voucher_number": row.voucher_number,
        "date": row.date,
        "voucher_type": row.voucher_type,
        "master_id": row.master_id,
        "alter_id": row.alter_id,
        "gst_status": status,
        "accepted_as_it_stands": overridden,
    });
    if let Some(flags) = row.status_flags {
        item["status_flags"] = json!(flags);
    }
    item
}

/// The vouchers of a window counted by what each one's flags state, and how many
/// of them are unread. A voucher whose status or whose acceptance flag was not
/// read is unread: the state of the answer is partial then. `unread` is a count
/// within the others, not apart from them.
pub(super) fn status_counts<'a>(
    rows: impl Iterator<Item = &'a StatusRow>,
) -> (BTreeMap<&'static str, u64>, u64) {
    let mut counts = BTreeMap::<&'static str, u64>::new();
    let mut unread = 0_u64;
    for row in rows {
        if row.status.is_err() || row.overridden.is_err() {
            unread += 1;
        }
        match row.status {
            Ok(GstStatus::Included) => *counts.entry("included").or_default() += 1,
            Ok(GstStatus::Uncertain) => *counts.entry("uncertain").or_default() += 1,
            Ok(GstStatus::Excluded) => *counts.entry("excluded").or_default() += 1,
            Ok(GstStatus::NotInReturn) => *counts.entry("not_in_return").or_default() += 1,
            Ok(GstStatus::Cancelled) => *counts.entry("cancelled").or_default() += 1,
            Ok(GstStatus::Optional) => *counts.entry("optional").or_default() += 1,
            Ok(GstStatus::PostDated) => *counts.entry("post_dated").or_default() += 1,
            Err(_) => {}
        }
        if row.overridden == Ok(true) {
            *counts.entry("accepted_as_it_stands").or_default() += 1;
        }
    }
    (counts, unread)
}

/// The counts a status result reports, every key present.
pub(super) fn counts_json(counts: &BTreeMap<&'static str, u64>, unread: u64) -> Value {
    let count = |key: &str| counts.get(key).copied().unwrap_or(0);
    json!({
        "included": count("included"),
        "uncertain": count("uncertain"),
        "excluded": count("excluded"),
        "not_in_return": count("not_in_return"),
        "cancelled": count("cancelled"),
        "optional": count("optional"),
        "post_dated": count("post_dated"),
        "unread": unread,
        "accepted_as_it_stands": count("accepted_as_it_stands"),
    })
}

/// The arguments every status read takes, checked before any read is made.
pub(super) struct StatusArguments<'a> {
    pub(super) guid: &'a str,
    pub(super) from: bridge_tally_core::TallyDate,
    pub(super) to: bridge_tally_core::TallyDate,
    pub(super) type_name: read_profiles::GstStatusTypeName,
}

/// The company, its verified identity and what has been read so far, once the
/// book's voucher mark has passed the gate.
pub(super) struct StatusGate {
    pub(super) company: TallyCompany,
    identity: VerifiedCompanyIdentity,
    pub(super) evidence: Evidence,
}

/// The arguments of a status read: `company_guid`, the window and the voucher
/// type's name. Argument refusals cost no read.
pub(super) fn status_arguments(args: &Value) -> Result<StatusArguments<'_>, ToolFailure> {
    let guid = required_string(args, "company_guid")?;
    let from = normalized_date(required_string(args, "from")?)?;
    let to = normalized_date(required_string(args, "to")?)?;
    let Some(days) = window_span_days(&from, &to) else {
        return Err("invalid_date_range".to_string().into());
    };
    let type_name =
        read_profiles::GstStatusTypeName::parse(required_string(args, "voucher_type_name")?)
            .ok_or_else(|| "gst_status_type_name_invalid".to_string())?;
    if days > STATUS_MAX_WINDOW_DAYS {
        return Err("gst_status_window_too_long".to_string().into());
    }
    Ok(StatusArguments {
        guid,
        from,
        to,
        type_name,
    })
}

impl Server {
    /// Verifies the company, then reads its voucher marks: the formula that
    /// selects the type and the window is evaluated on every voucher of the
    /// book, so the voucher mark bounds the read.
    pub(super) async fn status_gate(&self, guid: &str) -> Result<StatusGate, ToolFailure> {
        let (company, identity, prior) = self.verified_company(guid).await?;
        let (marks_xml, marks_evidence) = self
            .post_read(&identity, company_high_water_read(&company.name))
            .await
            .map_err(|failure| failure.with_prior_evidence(prior.clone()))?;
        let evidence = combine_evidence(prior, marks_evidence);
        let (vouchers, _) =
            parse_company_marks(&marks_xml, identity.company_guid()).map_err(|code| {
                ToolFailure::from(code.to_string()).with_prior_evidence(evidence.clone())
            })?;
        if vouchers > STATUS_MAX_VOUCHER_MARK {
            return Err(ToolFailure::from("gst_status_book_too_large".to_string())
                .with_prior_evidence(evidence));
        }
        Ok(StatusGate {
            company,
            identity,
            evidence,
        })
    }

    /// One status window read: the request, its parse and the window and the
    /// type honoured by Tally or the answer refused. The rows come back sorted
    /// by date, number and master id, each with what `extra` read from its
    /// voucher, and with the evidence of the gate's reads and this one.
    pub(super) async fn read_status_window<T>(
        &self,
        gate: &StatusGate,
        arguments: &StatusArguments<'_>,
        request: read_profiles::ReadRequest,
        evidence: Evidence,
        extra: impl Fn(&Node) -> T,
    ) -> Result<(Vec<(StatusRow, T)>, Evidence), ToolFailure> {
        let (xml, read_evidence) = self
            .post_read(&gate.identity, request)
            .await
            .map_err(|failure| failure.with_prior_evidence(evidence.clone()))?;
        let evidence = combine_evidence(evidence, read_evidence);
        let mut rows = parse_rows_with(&xml, extra).map_err(|code| {
            ToolFailure::from(code.to_string()).with_prior_evidence(evidence.clone())
        })?;
        // The window and the type are honoured by Tally or the answer is
        // refused: a voucher outside either is never reported.
        for (row, _) in &rows {
            let day = bridge_tally_core::TallyDate::parse(row.date.as_str()).map_err(|_| {
                ToolFailure::from("gst_status_read_voucher_unidentified".to_string())
                    .with_prior_evidence(evidence.clone())
            })?;
            if day < arguments.from || day > arguments.to {
                return Err(ToolFailure::from("window_not_honoured".to_string())
                    .with_prior_evidence(evidence));
            }
            if row.voucher_type != arguments.type_name.as_str() {
                return Err(
                    ToolFailure::from("gst_status_type_not_honoured".to_string())
                        .with_prior_evidence(evidence),
                );
            }
        }
        rows.sort_by(|(a, _), (b, _)| {
            (&a.date, &a.voucher_number, &a.master_id).cmp(&(
                &b.date,
                &b.voucher_number,
                &b.master_id,
            ))
        });
        Ok((rows, evidence))
    }

    pub(super) async fn gst_status(&self, args: &Value) -> Result<ToolOutcome, ToolFailure> {
        let arguments = status_arguments(args)?;
        let (from, to, type_name) = (&arguments.from, &arguments.to, &arguments.type_name);
        let offset = arg_usize(args, "offset", 0)?;
        let limit =
            arg_positive_usize(args, "limit", self.settings.max_rows)?.min(self.settings.max_rows);
        let gate = self.status_gate(arguments.guid).await?;
        let request = gst_status_read(&gate.company.name, from, to, type_name).map_err(|code| {
            ToolFailure::from(code.to_string()).with_prior_evidence(gate.evidence.clone())
        })?;
        let (rows, mut evidence) = self
            .read_status_window(&gate, &arguments, request, gate.evidence.clone(), |_| ())
            .await?;
        let rows: Vec<StatusRow> = rows.into_iter().map(|(row, ())| row).collect();
        let (counts, unread) = status_counts(rows.iter());
        let company = gate.company;
        let guid = arguments.guid;
        let total = rows.len();
        let page: Vec<Value> = rows.iter().skip(offset).take(limit).map(row_json).collect();
        let next_offset = (offset + page.len() < total).then_some(offset + page.len());
        let state = if unread == 0 {
            "complete"
        } else {
            evidence.state = "partial";
            evidence.reason_code = Some("gst_status_voucher_unread".to_string());
            "partial"
        };
        let mut result = json!({
            "state": state,
            "basis": "tally_voucher_gst_status_fields_one_voucher_type_one_window",
            "window": {"from": from.as_str(), "to": to.as_str()},
            "voucher_type_name": type_name.as_str(),
            "total": total,
            "offset": offset,
            "counts": counts_json(&counts, unread),
            "items": page,
            "limitations": [
                "These are the status flags Tally holds on each voucher when it is read. Tally keeps the status a voucher was given when it was saved; on the lab's book, correcting a master afterwards did not change the status of a voucher already saved (one voucher, cause unverified)",
                "The reason Tally gives for an uncertain voucher is on its screen only; no field carries it, so none is returned. The one reason measured on the lab's book is a tax figure that did not match Tally's own figure",
                "included means exactly one flag Yes; it counts a voucher a person accepted as it stands as well (see accepted_as_it_stands), so included is not 'clean'. unread is a count within the other counts, not apart from them: a voucher whose acceptance flag was unread is counted under its status and under unread, and the answer is partial",
                "not_in_return means all three flags No. The lab's Receipts read so (with the request before it fetched the cancelled, optional and post-dated flags; the request as it is now is measured on Sales vouchers only), which supports 'Tally does not list the voucher in the return' for Receipts only; it is not measured for Sales in a company without GST or for any other type, and the lab's keyed Purchase invoices read uncertain",
                "excluded has not been observed on the lab's book; it rests on the field's name",
                "accepted_as_it_stands is Tally's flag ISGSTOVERRIDDEN. It was seen to go from No to Yes once, by Accept As Is; whether other actions set it is not measured",
                "A voucher whose flags, or whose cancelled, optional or post-dated flag, are absent, repeated, nested in another element or spelled another way is unread, never included",
                "cancelled means Tally's ISCANCELLED reads Yes and optional that its ISOPTIONAL reads Yes. Each is decided before the three status flags and is never counted as included, uncertain, excluded or not_in_return: on the lab's book a cancelled Sales voucher read the included flag Yes and an optional one read all three No (one voucher each). Whether Tally lists a cancelled voucher anywhere in the return, such as among the documents issued, is not measured",
                "post_dated means Tally's ISPOSTDATED reads Yes, which is not the same as a date still ahead: on the lab's book the flag read Yes on a Sales voucher marked post-dated by hand (Ctrl+T) and dated the day Tally's current date (the date set with F2, not the calendar's) had been set to, and that voucher read the included flag Yes (one voucher). It is decided before the three status flags, as cancelled and optional are, and is never counted as included, uncertain, excluded or not_in_return. Whether Tally's return lists a post-dated voucher, while Tally's current date is before the voucher's date or once it has reached it, is not measured, and ComplyEaze Bridge states neither: look at the return in Tally. A voucher that reads more than one of cancelled, optional and post-dated Yes is unread",
                "status_flags, on a cancelled, optional or post_dated voucher only, is what Tally's three status flags read on it (included, uncertain, excluded or not_in_return). It is returned so that nothing read is hidden, and it is not counted: it does not say that the return lists the voucher",
                "A voucher whose ISDELETED or ISDELETEDVCHRETAINED flag reads Yes is unread (gst_status_voucher_deleted), and so is whether it was accepted as it stands, which is not counted. Tally returns the two flags without being asked, No on every voucher the lab read; a Yes was never measured, and what ISDELETEDVCHRETAINED means rests on its name. On the lab's book, where no Edit Log was seen to be on, one deleted voucher left the answer altogether. Both flags absent is allowed; one absent beside the other is unread",
                "A total of 0 may mean no voucher of that name in the window or a type name that matched nothing; the name is not checked against the book's voucher types, take it from masters",
                "Each page is a new read of Tally: a voucher saved between two pages can shift the offsets, so a page may repeat or skip one. Read all the pages in one sitting, or narrow the window",
                "Measured on one synthetic book of TallyPrime 7.1 Silver, for the unregistered buyer of its Sales invoices; a registered buyer, other releases and Gold are not measured",
                "The voucher type is selected by its exact name in the request (a name holding & < > ' or a quote is refused); a type of the same class under another name is not read",
                "The selection formula is evaluated on every voucher of the book, so a book whose voucher mark is above 25,000 is refused (provisional: timed only on a book of about thirty vouchers); the size of an answer is not otherwise bounded",
            ],
        });
        if let Some(next) = next_offset {
            result["next_offset"] = json!(next);
        }
        Ok(ToolOutcome {
            payload: json!({
                "company": company_json(&company, std::slice::from_ref(&company)),
                "result": result,
            }),
            evidence,
            company_guid: Some(guid.to_string()),
            truncated: next_offset.is_some(),
        })
    }
}

#[cfg(test)]
#[path = "agent_gst_status_tests.rs"]
mod tests;
