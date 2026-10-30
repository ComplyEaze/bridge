//! A test-only dump of the requests the product itself renders, as files a lab
//! operator can send by hand (#681, #1231, #899). It only formats strings and
//! writes files into a new folder; nothing here opens a connection, so nothing
//! it does can reach Tally. Every file is a request builder's own output,
//! unedited (AGENTS.md: no request is written by hand, and a new shape is not
//! made here: a candidate shape is measured through the dispatch path, with its
//! deadline, size cap and window checks, not by hand). Each is checked before
//! it is written, UTF-16LE with a byte-order mark and no final line break, as
//! the lab sends them.
//!
//! Send one at a time, on a small synthetic book and a window the book is
//! known to fill lightly: an oversized read can occupy the gateway for many
//! minutes (protocol reference 5.2). The windows are for licensed mode; in
//! Education mode only the days 1, 2 and 31 are honoured (2.7).
//!
//! To dump for a named lab company (the folder must be absent or empty):
//! `BRIDGE_DUMP_COMPANY="..." BRIDGE_DUMP_FROM=YYYYMMDD BRIDGE_DUMP_TO=YYYYMMDD
//!  BRIDGE_DUMP_DIR=... cargo test -p bridge --lib request_dump -- --ignored --nocapture`
//! Optional, both or neither: `BRIDGE_DUMP_SPAN_AFTER` and
//! `BRIDGE_DUMP_SPAN_THROUGH` also write the census of that AlterID span. Every
//! census the product sends is spanned (#2145): a book whose mark fits one
//! census is counted in the span from 0 to its mark.
use super::*;
use bridge_tally_protocol::xml_read_profiles::{
    ReadOnlyProfile, ValidatedCompanyName, ValidatedDateRange,
};
use std::fmt::Write as _;
use std::io::Write as _;

/// One request to write: its file name and rendered text.
struct Dumped {
    name: &'static str,
    xml: String,
}

fn day(text: &str) -> Result<bridge_tally_core::TallyDate, String> {
    bridge_tally_core::TallyDate::parse(text).map_err(|_| format!("{text:?} is not a date"))
}

/// The header ID must be the NAME of a collection the request defines, else
/// Tally raises a modal error that blocks the gateway for everyone.
fn id_names_its_collection(xml: &str) -> Result<(), String> {
    let id = xml
        .split("<ID>")
        .nth(1)
        .and_then(|rest| rest.split("</ID>").next())
        .ok_or("no header ID")?;
    let defined = format!("<COLLECTION NAME=\"{id}\"");
    if xml.matches(&defined).count() == 1 {
        Ok(())
    } else {
        Err(format!(
            "header ID {id:?} is not the name of the one collection defined"
        ))
    }
}

/// The product's renderings for one company and window, checked: the shipped
/// voucher read, the tax-audit voucher profile (the baseline of #681) and, when
/// a span is given, the census of that AlterID span.
fn rendered(
    company: &str,
    from: &str,
    to: &str,
    span: Option<(u64, u64)>,
) -> Result<Vec<Dumped>, String> {
    let (from_date, to_date) = (day(from)?, day(to)?);
    let name = ValidatedCompanyName::new(company).map_err(|_| "company_name_invalid")?;
    let window = ValidatedDateRange::new(from, to).map_err(|_| "window_invalid")?;
    let mut all = vec![
        Dumped {
            name: "v01-vouchers-wildcard.xml",
            xml: render_agent_vouchers(company, &from_date, &to_date, None)?,
        },
        Dumped {
            name: "a01-vouchers-audit-profile.xml",
            xml: ReadOnlyProfile::AuditVouchersV1 {
                company: &name,
                window: &window,
            }
            .render(),
        },
    ];
    if let Some((after, through)) = span {
        all.push(Dumped {
            name: "c02-census-span.xml",
            xml: render_agent_voucher_census(
                company,
                &from_date,
                &to_date,
                AlterIdSpan { after, through },
            )?,
        });
    }
    for request in &all {
        id_names_its_collection(&request.xml).map_err(|why| format!("{}: {why}", request.name))?;
    }
    Ok(all)
}

/// Writes the checked requests and a manifest into `dir`, which must be absent
/// or empty. Existing files are never overwritten. Returns the manifest text.
fn write_all(dir: &std::path::Path, requests: &[Dumped]) -> Result<String, String> {
    if dir.exists()
        && std::fs::read_dir(dir)
            .map_err(|e| e.to_string())?
            .next()
            .is_some()
    {
        return Err("the folder is not empty".into());
    }
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let create = |name: &str, bytes: &[u8]| -> Result<(), String> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dir.join(name))
            .map_err(|e| e.to_string())?;
        file.write_all(bytes).map_err(|e| e.to_string())
    };
    let mut manifest = String::new();
    for request in requests {
        let bytes = bridge_tally_protocol::encode_tally_xml_request_utf16le(&request.xml);
        create(request.name, &bytes)?;
        writeln!(
            manifest,
            "{}  {}  {}",
            sha256_hex(&bytes),
            bytes.len(),
            request.name
        )
        .map_err(|e| e.to_string())?;
    }
    create("MANIFEST.txt", manifest.as_bytes())?;
    Ok(manifest)
}

/// Both span bounds or neither: one alone is refused, never dropped.
fn span_from(after: Option<String>, through: Option<String>) -> Result<Option<(u64, u64)>, String> {
    match (after, through) {
        (None, None) => Ok(None),
        (Some(after), Some(through)) => Ok(Some((
            after.parse().map_err(|_| "span after is not a number")?,
            through
                .parse()
                .map_err(|_| "span through is not a number")?,
        ))),
        _ => Err("set both BRIDGE_DUMP_SPAN_AFTER and BRIDGE_DUMP_SPAN_THROUGH, or neither".into()),
    }
}

/// The dump is the product's own output, checked: every header ID names the
/// collection the request defines, the audit profile carries no allocation
/// path (it is the baseline), the census is a census, and the files are
/// BOM-prefixed UTF-16LE.
#[test]
fn the_dumped_requests_are_checked_and_unedited() {
    let requests = rendered("Synthetic Book", "20260801", "20260802", Some((10, 20))).unwrap();
    let by_name = |name: &str| {
        requests
            .iter()
            .find(|r| r.name == name)
            .unwrap()
            .xml
            .clone()
    };
    assert_eq!(
        by_name("v01-vouchers-wildcard.xml"),
        render_agent_vouchers(
            "Synthetic Book",
            &day("20260801").unwrap(),
            &day("20260802").unwrap(),
            None
        )
        .unwrap()
    );
    assert!(by_name("v01-vouchers-wildcard.xml").contains("ALLLEDGERENTRIES.*"));
    assert!(!by_name("a01-vouchers-audit-profile.xml").contains("BILLALLOCATIONS"));
    assert!(by_name("c02-census-span.xml").contains("Bridge Agent Voucher Census"));
    assert!(by_name("c02-census-span.xml").contains("$AlterID &gt; 10 AND $AlterID &lt;= 20"));
    // No census without its span: the product sends none (#2145).
    assert_eq!(requests.len(), 3);
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("dump");
    let manifest = write_all(&target, &requests).unwrap();
    assert_eq!(manifest.lines().count(), 3);
    let bytes = std::fs::read(target.join("v01-vouchers-wildcard.xml")).unwrap();
    assert_eq!(
        &bytes[..2],
        &[0xFF, 0xFE],
        "UTF-16LE with a byte-order mark"
    );
    assert!(!bytes.ends_with(&[0x0A, 0x00]), "no final line break");
    // A folder that already holds a file is refused, never written into.
    assert_eq!(
        write_all(&target, &requests),
        Err("the folder is not empty".to_string())
    );
}

/// A header ID that is not a defined collection's name is refused.
#[test]
fn a_request_whose_id_names_no_collection_is_refused() {
    let broken = "<ENVELOPE><HEADER><ID>One</ID></HEADER><BODY><COLLECTION NAME=\"Two\" ISMODIFY=\"No\"></COLLECTION></BODY></ENVELOPE>";
    assert!(id_names_its_collection(broken).is_err());
    assert!(id_names_its_collection(&broken.replace("One", "Two")).is_ok());
}

/// A span with one bound is refused; a bad date or name is an error, not a panic.
#[test]
fn bad_settings_are_refused_not_dropped() {
    assert_eq!(span_from(None, None), Ok(None));
    assert_eq!(
        span_from(Some("5".into()), Some("9".into())),
        Ok(Some((5, 9)))
    );
    assert!(span_from(Some("5".into()), None).is_err());
    assert!(span_from(None, Some("9".into())).is_err());
    assert!(span_from(Some("x".into()), Some("9".into())).is_err());
    assert!(rendered("Synthetic Book", "2026-08-01", "20260802", None).is_err());
    assert!(rendered("", "20260801", "20260802", None).is_err());
}

/// The operator's entry point: ignored unless asked for, and refused when its
/// settings are missing.
#[test]
#[ignore = "writes files for the lab; see the module comment"]
fn request_dump_for_a_named_lab_company() {
    let setting = |name: &str| std::env::var(name).unwrap_or_else(|_| panic!("{name} is not set"));
    let span = span_from(
        std::env::var("BRIDGE_DUMP_SPAN_AFTER").ok(),
        std::env::var("BRIDGE_DUMP_SPAN_THROUGH").ok(),
    )
    .unwrap_or_else(|why| panic!("{why}"));
    let requests = rendered(
        &setting("BRIDGE_DUMP_COMPANY"),
        &setting("BRIDGE_DUMP_FROM"),
        &setting("BRIDGE_DUMP_TO"),
        span,
    )
    .unwrap_or_else(|why| panic!("{why}"));
    let manifest = write_all(std::path::Path::new(&setting("BRIDGE_DUMP_DIR")), &requests)
        .unwrap_or_else(|why| panic!("{why}"));
    println!("{manifest}");
}
