//! A test-only dump of the requests the product itself renders, as files a lab
//! operator can send by hand (#681, #1231, #899). It only formats strings and
//! writes files into a new folder; nothing here opens a connection, so nothing
//! it does can reach Tally. The files are the product's own builder output
//! (AGENTS.md: no request is written by hand), each checked before it is
//! written, UTF-16LE with a byte-order mark and no final line break, as the lab
//! sends them.
//!
//! To dump for a named lab company (set the folder to a path that does not
//! exist yet or is empty):
//! `BRIDGE_DUMP_COMPANY="..." BRIDGE_DUMP_FROM=YYYYMMDD BRIDGE_DUMP_TO=YYYYMMDD
//!  BRIDGE_DUMP_DIR=... cargo test -p bridge --lib request_dump -- --ignored --nocapture`
//! Optional: `BRIDGE_DUMP_SPAN_AFTER` and `BRIDGE_DUMP_SPAN_THROUGH` also write
//! a census narrowed to that AlterID span.
use super::*;
use std::fmt::Write as _;

/// One request to write: its file name and rendered text.
struct Dumped {
    name: &'static str,
    xml: String,
}

/// The voucher FETCH list `ledger_movement` sends, which names its fields and
/// carries no bill allocations. The allocation variant adds exactly one field.
const WITH_ALLOCATIONS: &str = ",ALLLEDGERENTRIES.BILLALLOCATIONS.*";

fn day(text: &str) -> bridge_tally_core::TallyDate {
    bridge_tally_core::TallyDate::parse(text).unwrap()
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

/// The product's renderings for one company and window, checked.
fn rendered(
    company: &str,
    from: &str,
    to: &str,
    span: Option<(u64, u64)>,
) -> Result<Vec<Dumped>, String> {
    let (from, to) = (day(from), day(to));
    let wildcard = render_agent_vouchers(company, &from, &to, None)?;
    let named = render_agent_movement_vouchers(company, &from, &to)?;
    // The one change of the allocation variant: one more field at the end of
    // the FETCH list. It must apply exactly once, else this refuses.
    let end = "ALLLEDGERENTRIES.ISDEEMEDPOSITIVE</FETCH>";
    if named.matches(end).count() != 1 {
        return Err("the named-field request no longer ends its FETCH as expected".into());
    }
    let with_allocations = named.replacen(
        end,
        &format!("ALLLEDGERENTRIES.ISDEEMEDPOSITIVE{WITH_ALLOCATIONS}</FETCH>"),
        1,
    );
    let mut all = vec![
        Dumped {
            name: "v01-vouchers-wildcard.xml",
            xml: wildcard,
        },
        Dumped {
            name: "v02-vouchers-named-no-allocations.xml",
            xml: named,
        },
        Dumped {
            name: "v03-vouchers-named-with-allocations.xml",
            xml: with_allocations,
        },
        Dumped {
            name: "c01-census-unspanned.xml",
            xml: render_agent_voucher_census(company, &from, &to, None)?,
        },
    ];
    if let Some((after, through)) = span {
        all.push(Dumped {
            name: "c02-census-span.xml",
            xml: render_agent_voucher_census(
                company,
                &from,
                &to,
                Some(AlterIdSpan { after, through }),
            )?,
        });
    }
    for request in &all {
        id_names_its_collection(&request.xml).map_err(|why| format!("{}: {why}", request.name))?;
    }
    Ok(all)
}

/// Writes the checked requests and a manifest into `dir`, which must be absent
/// or empty. Returns the manifest text.
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
    let mut manifest = String::new();
    for request in requests {
        let bytes = bridge_tally_protocol::encode_tally_xml_request_utf16le(&request.xml);
        std::fs::write(dir.join(request.name), &bytes).map_err(|e| e.to_string())?;
        writeln!(
            manifest,
            "{}  {}  {}",
            sha256_hex(&bytes),
            bytes.len(),
            request.name
        )
        .map_err(|e| e.to_string())?;
    }
    std::fs::write(dir.join("MANIFEST.txt"), &manifest).map_err(|e| e.to_string())?;
    Ok(manifest)
}

/// The dump is the product's own output, checked: every header ID names the
/// collection the request defines, the allocation variant differs from the
/// named-field request by exactly the one field, and the census is a census.
#[test]
fn the_dumped_requests_are_checked_and_differ_only_as_stated() {
    let requests = rendered("Synthetic Book", "20260801", "20260802", Some((10, 20))).unwrap();
    let by_name = |name: &str| {
        requests
            .iter()
            .find(|r| r.name == name)
            .unwrap()
            .xml
            .clone()
    };
    let named = by_name("v02-vouchers-named-no-allocations.xml");
    let with = by_name("v03-vouchers-named-with-allocations.xml");
    assert!(!named.contains("BILLALLOCATIONS"));
    assert_eq!(with.replacen(WITH_ALLOCATIONS, "", 1), named);
    assert!(by_name("v01-vouchers-wildcard.xml").contains("ALLLEDGERENTRIES.*"));
    assert!(by_name("c01-census-unspanned.xml").contains("Bridge Agent Voucher Census"));
    assert!(by_name("c02-census-span.xml").contains("$AlterID"));
    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("dump");
    let manifest = write_all(&target, &requests).unwrap();
    assert_eq!(manifest.lines().count(), 5);
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

/// The operator's entry point: ignored unless asked for, and refused when its
/// settings are missing.
#[test]
#[ignore = "writes files for the lab; see the module comment"]
fn request_dump_for_a_named_lab_company() {
    let setting = |name: &str| std::env::var(name).unwrap_or_else(|_| panic!("{name} is not set"));
    let span = match (
        std::env::var("BRIDGE_DUMP_SPAN_AFTER"),
        std::env::var("BRIDGE_DUMP_SPAN_THROUGH"),
    ) {
        (Ok(after), Ok(through)) => Some((after.parse().unwrap(), through.parse().unwrap())),
        _ => None,
    };
    let requests = rendered(
        &setting("BRIDGE_DUMP_COMPANY"),
        &setting("BRIDGE_DUMP_FROM"),
        &setting("BRIDGE_DUMP_TO"),
        span,
    )
    .unwrap();
    let manifest = write_all(std::path::Path::new(&setting("BRIDGE_DUMP_DIR")), &requests).unwrap();
    println!("{manifest}");
}
