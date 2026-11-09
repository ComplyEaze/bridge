//! These hashes pin the request bytes Bridge sends. Each row is one request
//! builder of the app crate, called through its entry point with the fixed
//! synthetic inputs below, and the SHA-256 of the request text it returns. A
//! pull request that changes a row must say which builder changed and why
//! (#2397); every other pull request leaves the tables as they are.
//!
//! On a mismatch the test prints every differing row, with the hash it got, in
//! table syntax, so a declared change is a copy and paste. A row with no
//! builder, or a builder with no row, fails as well. The builders compiled only
//! with the `lab-writes` feature have their own table.
//!
//! The module sits under `agent_import` so that the import builders, which are
//! private to it, are reachable; the protocol crate's builders are pinned in
//! its own `request_bytes_tests`.

use std::collections::BTreeSet;

use bridge_tally_core::TallyDate;
use bridge_tally_protocol::ledger_guid_part::{LedgerGuid, LedgerGuidPart};
use bridge_tally_protocol::ledger_scope::LedgerScopeName;

use super::cancel::{render_native_cancel_xml, RemoteId};
use super::invoice::{
    render_company_registration_request, render_group_hsn_request, render_invoice_number_request,
    render_invoice_readback_request, render_ledger_rates_request_for_ledgers,
    render_voucher_types_request, InvoiceNumber, VoucherTypeGuid,
};
use super::tests::{sentinel_batch_id, sentinel_uuid};
use super::{
    render_import_verification_in_span, render_import_verification_read, render_import_xml,
    render_native_vouchers_as, render_native_vouchers_xml, ImportVoucher, VoucherAction,
};
use crate::agent::read_profiles::{
    render_agent_class_vouchers_in_span, render_agent_company_high_water,
    render_agent_ledger_census, render_agent_ledger_vouchers, render_agent_movement_vouchers,
    render_agent_movement_vouchers_in_span, render_agent_named_vouchers_in_span,
    render_agent_stock_line_vouchers_in_span, render_agent_voucher_census, render_agent_vouchers,
    render_agent_vouchers_in_span, render_gst_status_window, render_gst_uncertain_window,
    GstStatusTypeName, LedgerVoucherFields,
};
use crate::agent::voucher_parse::{EInvoiceFields, NamedVoucherFields, VoucherGstFields};
use crate::agent::voucher_window::AlterIdSpan;
use crate::tally::ledger_create_item::{AdmittedLedgerKind, LedgerCreateItem, MasterName};
use crate::tally::VerifiedCompanyIdentity;

/// One synthetic company name holding `&`, `<`, a double quote and non-ASCII
/// text, used by every builder that takes a company.
const COMPANY: &str = "Synthetic & Co <Lab> \"Q\" Ünïcode";

const TABLE: &[(&str, &str)] = &[
    (
        "render_import_verification_read",
        "23f096d4d4db89ab83e41390a3b849909719d697cb17222b76f1e25a6ba71a78",
    ),
    (
        "render_import_verification_in_span/span",
        "38705dc2f6cbad3f899e88ad24be2384c58c039790ab253df5e86e9e5ff522f6",
    ),
    (
        "render_agent_voucher_census",
        "9e0db47a4bc8a28057035315bfd581e2cacfaaf2aa9aa8e7fb9e2fdd71e1b1f2",
    ),
    (
        "render_agent_ledger_census",
        "6070b78bcb469fbaadfb6a8f2dd224f7faf29a4d52b41c65422c0015b1c3be8b",
    ),
    (
        "render_agent_movement_vouchers",
        "2e5186efa43392e7adbf7b2b07b5e29f273fd70298fdcc38209991f8184f8518",
    ),
    (
        "render_agent_movement_vouchers_in_span/span",
        "aa3ca3a6a727bdc77c695267429db7df4eb7797ae776404a3da781a945f8024c",
    ),
    (
        "render_agent_vouchers/no_alter_id",
        "fcf4f82c668c51d703809287a269fea2b60e9ccb08edfd8a1bfebca47828c36f",
    ),
    (
        "render_agent_vouchers/alter_id_2",
        "a947f13d2da14d69ad35bbd32753fd554c23339f4c99f8f4088fbe260ca177a8",
    ),
    (
        "render_agent_vouchers_in_span/span",
        "9fdd8e0ad470cff5d5ec27eaecf35a956e3b24b5e6ae1db7e76366ecf682018b",
    ),
    (
        "render_agent_class_vouchers_in_span/span",
        "cdc64cfced73065db75e12194700e3915019089c54f56a3e33e55a6276a4b2fe",
    ),
    (
        "render_agent_named_vouchers_in_span/span/plain/plain/no_classes",
        "9fdd8e0ad470cff5d5ec27eaecf35a956e3b24b5e6ae1db7e76366ecf682018b",
    ),
    (
        "render_agent_named_vouchers_in_span/span/plain/plain/classes",
        "cdc64cfced73065db75e12194700e3915019089c54f56a3e33e55a6276a4b2fe",
    ),
    (
        "render_agent_named_vouchers_in_span/span/plain/gst/no_classes",
        "eb134f006a8ee6ee4efeb22bde5e20dbbb1f987a56167bc10f49ea521c75ef0d",
    ),
    (
        "render_agent_named_vouchers_in_span/span/plain/gst/classes",
        "57b919417436e638e7ce8bbf6e2975d3d71c1b932915911e23e2e76106f08872",
    ),
    (
        "render_agent_named_vouchers_in_span/span/e_invoice/plain/no_classes",
        "a454b1d5e6373b9847ce9d6f7eef18aac04225c177bc9a7d6183e975dab57662",
    ),
    (
        "render_agent_named_vouchers_in_span/span/e_invoice/plain/classes",
        "7299756b8440019a1f78940425ea26bbdc2c3bed70b84ac290fd414af36f70f4",
    ),
    (
        "render_agent_named_vouchers_in_span/span/e_invoice/gst/no_classes",
        "b5adc54b42b957c27b5022ddbb5d42a70757f204921c0cb6880ab52d57173f98",
    ),
    (
        "render_agent_named_vouchers_in_span/span/e_invoice/gst/classes",
        "bba5c17a71d24818e2c1730c6930bfa32069fb9bef7d3726926973874dc00e5d",
    ),
    (
        "render_agent_stock_line_vouchers_in_span/span",
        "57b8bdf783969fa276c9a5b27f0e65d9979bc9b8688918bcd2612a6056e08da3",
    ),
    (
        "render_agent_ledger_vouchers/Movement",
        "5fa9659617d21b30dcb190a3bcd23a78afe84b9202bd30b05f75243682ee290b",
    ),
    (
        "render_agent_ledger_vouchers/Window",
        "daaced69f714dbf202736d054b8789ff3ba85cdba12220efa28593c9c8f8ac54",
    ),
    (
        "render_gst_status_window",
        "1e944080eeae29d95bc7e9a54233fc776e5b52259aebcb2a491d714b35851a3c",
    ),
    (
        "render_gst_uncertain_window",
        "005f42a1f477ebb755b9b23f5f7f96b45b2308863bf57ba1c4dbf358fce2f2f3",
    ),
    (
        "render_agent_company_high_water",
        "3ebd4c55a69823a2980947db575bdb50dbbfb542b4ddc7507f4ecf2984be8264",
    ),
    (
        "render_voucher_types_request",
        "c0a61f2da122f5200e1c00d35e68375d7d476ba63ab682372d8de7b687820b06",
    ),
    (
        "render_invoice_number_request",
        "bd849e43daa5547b5828b0c029d1c94455167e9b8fe609fa804d580ea56683f6",
    ),
    (
        "render_invoice_readback_request",
        "56a684c7e091a04d0596a82d85d85732ef313c027743f5d330ebb0aea2b9cfdd",
    ),
    (
        "render_company_registration_request",
        "85cdd319bd4922d1437a07f40b37317411ef24e55dd274152b31374ec51d996a",
    ),
    (
        "render_ledger_rates_request_for_ledgers",
        "96b41d40c58d9527da459219d4ea1b020b3c7e6688053ce3de7a42af892cb812",
    ),
    (
        "render_group_hsn_request",
        "86997f134645767eb3a960d4b0d84b96c932d44dc2de9590c5441d73e5198b6b",
    ),
    (
        "render_import_xml",
        "c9e6ba3a7100f093863618851f11ece2fb23cf8071404e3bbd642dbf885e29f9",
    ),
    (
        "render_native_vouchers_xml",
        "649650921f029789f24b944559678a6a708995d74aa66b809da4ce8214b6c29d",
    ),
    (
        "render_native_vouchers_as/Create",
        "649650921f029789f24b944559678a6a708995d74aa66b809da4ce8214b6c29d",
    ),
    (
        "render_native_vouchers_as/Cancel",
        "b118a454ebce68dd3ecf1f85a4ea68cbe9b170bebc6238ec07d04e3294fa4900",
    ),
    (
        "render_native_cancel_xml",
        "b118a454ebce68dd3ecf1f85a4ea68cbe9b170bebc6238ec07d04e3294fa4900",
    ),
    (
        "LedgerCreateItem::render_import/Suspense",
        "aa1eb1f074de236f4fbb0bcd2a6f5e3bdc6378fbd23892203737b4ac3be0fe52",
    ),
];

#[cfg(feature = "lab-writes")]
const LAB_TABLE: &[(&str, &str)] = &[
    (
        "render_agent_lab_inventory_vouchers",
        "498c6b79958ab29f05d9c427392f747416ca9e325156e662603bf3195c3322a0",
    ),
    (
        "render_agent_lab_purchase_identity_vouchers",
        "d2b5b1536cb35596ee77791ec971c1ecd97a463594ea42b47e76ce52104f79a6",
    ),
    (
        "render_master_collection_request/Unit",
        "d694a57846a547ad0069c5061d0f4b7bde65dceedd0a11134f76340502bbb3a8",
    ),
    (
        "render_master_collection_request/Godown",
        "8371fe0020b34d95dd6bfd3e93d493500ab9e186f988f74136ec7e6842f6ee6b",
    ),
    (
        "render_master_collection_request/StockGroup",
        "7f4ec5c68b9b0c2056c4b851d8a45bd6802d0ec191a17dcb26bc1dae8acd83a0",
    ),
    (
        "render_master_collection_request/Group",
        "298feca154c6ee8f8d14f9868c213dfcb910ab94a0ca2e6702759685e582f1b8",
    ),
    (
        "render_master_collection_request/Ledger",
        "fc389bf0b57de4020a48b29e792aaa69722fc134035d29e78bec4afe560ea07e",
    ),
    (
        "render_master_collection_request/StockItem",
        "d614fbf44e44a5b91a00644974453c3f1b1b93464790cfe1a6f01517cfb6151d",
    ),
    (
        "render_voucher_window_request",
        "80c6d6d2d34353e3cc07120ddf1d6317458fde1c9c07f8e82f772290218e092a",
    ),
    (
        "render_lab_master_collection/Unit",
        "752f1751f8f379d9dd40d39065a4f3f322429fcbfaea29f46db652d9c4998069",
    ),
    (
        "render_lab_master_collection/Godown",
        "dc239588dde429dea87a765951c56357aee26f0fc7433d96a5fa095bfb96e732",
    ),
    (
        "render_lab_master_collection/StockGroup",
        "6117e84d6c50258bcdf928bc1d4335541408182918f60af6ab807473d33b3f8a",
    ),
    (
        "render_lab_master_collection/StockItem",
        "b388eb99358beb8f4648d17a72afaaca30d8094d25040f521e1092caccd48a0e",
    ),
    (
        "render_lab_voucher_key_probe/Guid",
        "9cc7155a386f7582cf068f9f82a07b70cfbaabdc0a46f7dc97ce2ccf8956800c",
    ),
    (
        "render_lab_voucher_key_probe/RemoteId",
        "453c5681848bf003baafbd641a5a8ab2a3f4650183c7c69d9d5b05a96f381a73",
    ),
];

fn date(value: &str) -> TallyDate {
    TallyDate::parse(value).expect("a fixed calendar date")
}

/// The financial year every dated builder reads.
fn year() -> (TallyDate, TallyDate) {
    (date("20250401"), date("20260331"))
}

/// The AlterID span `agent_audit_read_tests` uses.
const SPAN: AlterIdSpan = AlterIdSpan {
    after: 0,
    through: 2,
};

fn ledger() -> LedgerScopeName {
    LedgerScopeName::new("DN Party 001").expect("an admitted ledger name")
}

/// The one-voucher batch of `agent_import_amount_pin_tests`, admitted.
fn voucher() -> ImportVoucher {
    let voucher: ImportVoucher<String> = serde_json::from_value(serde_json::json!({
        "bridge_txn_id":"txn-001","date":"2026-09-01","voucher_type":"Journal",
        "entries":[{"ledger":"Expense","amount":"12.50","side":"Dr"},
                   {"ledger":"Bank","amount":"12.50","side":"Cr"}]}))
    .expect("voucher");
    let date = crate::agent::normalized_date(&voucher.date).unwrap();
    let entries = voucher.parsed_entries().expect("an import amount");
    voucher.admitted(date, entries)
}

/// Every request the app crate builds in a default build, by label, in table
/// order.
fn built_requests() -> Vec<(String, String)> {
    let (from, to) = year();
    let span = Some(SPAN);
    let ledger = ledger();
    let mut rows: Vec<(String, String)> = Vec::new();
    let mut row = |label: &str, request: Result<String, String>| {
        let request = request.unwrap_or_else(|code| panic!("{label} refused: {code}"));
        rows.push((label.to_string(), request));
    };

    row(
        "render_import_verification_read",
        Ok(render_import_verification_read(COMPANY, &from, &to)),
    );
    row(
        "render_import_verification_in_span/span",
        Ok(render_import_verification_in_span(
            COMPANY, &from, &to, span,
        )),
    );
    row(
        "render_agent_voucher_census",
        render_agent_voucher_census(COMPANY, &from, &to, SPAN),
    );
    row(
        "render_agent_ledger_census",
        render_agent_ledger_census(COMPANY, &ledger, &from, &to, SPAN),
    );
    row(
        "render_agent_movement_vouchers",
        render_agent_movement_vouchers(COMPANY, &from, &to),
    );
    row(
        "render_agent_movement_vouchers_in_span/span",
        render_agent_movement_vouchers_in_span(COMPANY, &from, &to, span),
    );
    row(
        "render_agent_vouchers/no_alter_id",
        render_agent_vouchers(COMPANY, &from, &to, None),
    );
    row(
        "render_agent_vouchers/alter_id_2",
        render_agent_vouchers(COMPANY, &from, &to, Some(SPAN.through)),
    );
    row(
        "render_agent_vouchers_in_span/span",
        render_agent_vouchers_in_span(COMPANY, &from, &to, span),
    );
    row(
        "render_agent_class_vouchers_in_span/span",
        render_agent_class_vouchers_in_span(COMPANY, &from, &to, span),
    );
    for e_invoice in [EInvoiceFields::NotAsked, EInvoiceFields::Asked] {
        for gst in [VoucherGstFields::NotAsked, VoucherGstFields::Asked] {
            for classes in [false, true] {
                // Exhaustive, so a new choice fails to compile until it has rows.
                let e = match e_invoice {
                    EInvoiceFields::NotAsked => "plain",
                    EInvoiceFields::Asked => "e_invoice",
                };
                let g = match gst {
                    VoucherGstFields::NotAsked => "plain",
                    VoucherGstFields::Asked => "gst",
                };
                let c = if classes { "classes" } else { "no_classes" };
                row(
                    &format!("render_agent_named_vouchers_in_span/span/{e}/{g}/{c}"),
                    render_agent_named_vouchers_in_span(
                        COMPANY,
                        &from,
                        &to,
                        span,
                        NamedVoucherFields { e_invoice, gst },
                        classes,
                    ),
                );
            }
        }
    }
    row(
        "render_agent_stock_line_vouchers_in_span/span",
        render_agent_stock_line_vouchers_in_span(COMPANY, &from, &to, span),
    );
    for fields in [LedgerVoucherFields::Movement, LedgerVoucherFields::Window] {
        let name = match fields {
            LedgerVoucherFields::Movement => "Movement",
            LedgerVoucherFields::Window => "Window",
        };
        row(
            &format!("render_agent_ledger_vouchers/{name}"),
            render_agent_ledger_vouchers(COMPANY, &ledger, fields, &from, &to),
        );
    }
    let type_name = GstStatusTypeName::parse("BRIDGE Sales").expect("an admitted type name");
    row(
        "render_gst_status_window",
        render_gst_status_window(COMPANY, &from, &to, &type_name),
    );
    row(
        "render_gst_uncertain_window",
        render_gst_uncertain_window(COMPANY, &from, &to, &type_name),
    );
    row(
        "render_agent_company_high_water",
        Ok(render_agent_company_high_water(COMPANY)),
    );

    let number = InvoiceNumber::parse("INV/26-27/0042").expect("an admitted invoice number");
    let type_guid = VoucherTypeGuid::parse(&format!("{}-00000193", sentinel_uuid(0x5)))
        .expect("a hex-and-hyphen GUID");
    let ledgers = LedgerGuidPart::new(
        ["abc-b", "abc-a"].map(|guid| LedgerGuid::parse(guid).expect("a ledger GUID")),
    )
    .expect("a ledger part");
    row(
        "render_voucher_types_request",
        Ok(render_voucher_types_request(COMPANY)),
    );
    row(
        "render_invoice_number_request",
        Ok(render_invoice_number_request(
            COMPANY,
            &number,
            (&from, &to),
        )),
    );
    row(
        "render_invoice_readback_request",
        Ok(render_invoice_readback_request(
            COMPANY,
            &type_guid,
            &number,
            (&from, &to),
        )),
    );
    row(
        "render_company_registration_request",
        Ok(render_company_registration_request(COMPANY)),
    );
    row(
        "render_ledger_rates_request_for_ledgers",
        Ok(render_ledger_rates_request_for_ledgers(
            COMPANY,
            (&from, &to),
            &ledgers,
        )),
    );
    row(
        "render_group_hsn_request",
        Ok(render_group_hsn_request(COMPANY)),
    );

    // Import builders. Their REMOTEIDs and batch id are the sentinel UUIDs of
    // `agent_import_tests`, so the bytes are fixed.
    let voucher = voucher();
    let remote_id = sentinel_uuid(1);
    row(
        "render_import_xml",
        Ok(render_import_xml(
            COMPANY,
            std::slice::from_ref(&voucher),
            &sentinel_batch_id(1),
        )),
    );
    row(
        "render_native_vouchers_xml",
        Ok(render_native_vouchers_xml(
            COMPANY,
            std::iter::once((&voucher, remote_id)),
        )),
    );
    for action in [VoucherAction::Create, VoucherAction::Cancel] {
        let name = match action {
            VoucherAction::Create => "Create",
            VoucherAction::Cancel => "Cancel",
        };
        row(
            &format!("render_native_vouchers_as/{name}"),
            Ok(render_native_vouchers_as(
                action,
                COMPANY,
                std::iter::once((&voucher, remote_id)),
            )),
        );
    }
    let remote = RemoteId::parse(&remote_id.to_string()).expect("a minted REMOTEID");
    row(
        "render_native_cancel_xml",
        render_native_cancel_xml(COMPANY, &voucher, remote)
            .map_err(|refusal| format!("{refusal:?}")),
    );
    let identity = VerifiedCompanyIdentity::test_fixture(COMPANY, crate::synthetic_guid(1));
    let suspense = LedgerCreateItem::new(
        MasterName::new("MLab M1 Suspense").expect("a valid name"),
        AdmittedLedgerKind::Suspense,
    );
    row(
        "LedgerCreateItem::render_import/Suspense",
        Ok(suspense.render_import(&identity).xml().to_string()),
    );

    hash(rows)
}

/// Every request only a `lab-writes` build has, by label, in table order.
#[cfg(feature = "lab-writes")]
fn built_lab_requests() -> Vec<(String, String)> {
    use crate::agent::lab::import::{
        render_master_collection_request, render_voucher_window_request, MasterKind,
    };
    use crate::agent::lab::{
        render_lab_master_collection, render_lab_voucher_key_probe, LabMasterKind, ProbeKey,
        ProbeTerm,
    };
    use crate::agent::read_profiles::{
        render_agent_lab_inventory_vouchers, render_agent_lab_purchase_identity_vouchers,
    };
    use bridge_tally_protocol::native_outstandings::NativeLedgerExportPeriod;
    use bridge_tally_protocol::outstandings_shared::DateBoundaryProfile;

    let (from, to) = year();
    // The book-start period a lab master read carries: both bounds the first day.
    let period = NativeLedgerExportPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        from.clone(),
        from.clone(),
    )
    .expect("a book-start period");
    let mut rows: Vec<(String, String)> = Vec::new();
    let mut row = |label: &str, request: Result<String, String>| {
        let request = request.unwrap_or_else(|code| panic!("{label} refused: {code}"));
        rows.push((label.to_string(), request));
    };

    row(
        "render_agent_lab_inventory_vouchers",
        render_agent_lab_inventory_vouchers(COMPANY, &from, &to),
    );
    row(
        "render_agent_lab_purchase_identity_vouchers",
        render_agent_lab_purchase_identity_vouchers(COMPANY, &from, &to),
    );
    for kind in [
        MasterKind::Unit,
        MasterKind::Godown,
        MasterKind::StockGroup,
        MasterKind::Group,
        MasterKind::Ledger,
        MasterKind::StockItem,
    ] {
        let name = match kind {
            MasterKind::Unit => "Unit",
            MasterKind::Godown => "Godown",
            MasterKind::StockGroup => "StockGroup",
            MasterKind::Group => "Group",
            MasterKind::Ledger => "Ledger",
            MasterKind::StockItem => "StockItem",
        };
        row(
            &format!("render_master_collection_request/{name}"),
            render_master_collection_request(COMPANY, kind, &period),
        );
    }
    row(
        "render_voucher_window_request",
        render_voucher_window_request(COMPANY, &from, &to),
    );
    for kind in [
        LabMasterKind::Unit,
        LabMasterKind::Godown,
        LabMasterKind::StockGroup,
        LabMasterKind::StockItem,
    ] {
        let name = match kind {
            LabMasterKind::Unit => "Unit",
            LabMasterKind::Godown => "Godown",
            LabMasterKind::StockGroup => "StockGroup",
            LabMasterKind::StockItem => "StockItem",
        };
        row(
            &format!("render_lab_master_collection/{name}"),
            render_lab_master_collection(COMPANY, kind, &period),
        );
    }
    // The two keys of the lab module's own probe test.
    for (term, key) in [
        (
            ProbeTerm::Guid,
            "3a6bd6e1-b835-4bff-89dd-8a6af138c346-000000ce",
        ),
        (ProbeTerm::RemoteId, "00000000-0000-4000-8000-000000000002"),
    ] {
        let name = match term {
            ProbeTerm::Guid => "Guid",
            ProbeTerm::RemoteId => "RemoteId",
        };
        let key = ProbeKey::new(key).expect("an admitted probe key");
        row(
            &format!("render_lab_voucher_key_probe/{name}"),
            render_lab_voucher_key_probe(COMPANY, &from, term, &key),
        );
    }

    hash(rows)
}

fn hash(rows: Vec<(String, String)>) -> Vec<(String, String)> {
    rows.into_iter()
        .map(|(label, request)| (label, bridge_tally_protocol::sha256_hex(request.as_bytes())))
        .collect()
}

/// Fails unless every built row has a table row of the same label and hash and
/// the reverse. Prints every differing row in table syntax first.
fn assert_matches_table(table: &[(&str, &str)], built: &[(String, String)]) {
    let mut labels = BTreeSet::new();
    for (label, _) in table {
        assert!(labels.insert(*label), "the table lists {label:?} twice");
    }
    let mut built_labels = BTreeSet::new();
    for (label, _) in built {
        assert!(
            built_labels.insert(label.as_str()),
            "two requests are built under the label {label:?}"
        );
    }
    let differing: Vec<String> = built
        .iter()
        .filter(|(label, actual)| {
            !table
                .iter()
                .any(|(row, expected)| row == label && expected == actual)
        })
        .map(|(label, actual)| format!("    ({label:?}, {actual:?}),"))
        .collect();
    let unbuilt: Vec<&str> = table
        .iter()
        .map(|(label, _)| *label)
        .filter(|label| !built_labels.contains(label))
        .collect();
    if !differing.is_empty() || !unbuilt.is_empty() {
        panic!(
            "request bytes changed (#2397): {} row(s) differ or are new, {} row(s) have no builder.\n\
             Rows as built now:\n{}\nTable rows with no builder: {unbuilt:?}",
            differing.len(),
            unbuilt.len(),
            differing.join("\n"),
        );
    }
    assert_eq!(built.len(), table.len());
}

#[test]
fn request_bytes_of_every_app_builder_match_the_pinned_table() {
    assert_matches_table(TABLE, &built_requests());
}

#[cfg(feature = "lab-writes")]
#[test]
fn request_bytes_of_every_lab_builder_match_the_pinned_table() {
    assert_matches_table(LAB_TABLE, &built_lab_requests());
}
