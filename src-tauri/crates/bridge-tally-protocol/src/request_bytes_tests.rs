//! These hashes pin the request bytes Bridge sends. Each row is one request
//! builder of this crate, called through its entry point with the fixed
//! synthetic inputs below, and the SHA-256 of the request text it returns. A
//! pull request that changes a row must say which builder changed and why
//! (#2397); every other pull request leaves the table as it is.
//!
//! On a mismatch the test prints every differing row, with the hash it got, in
//! table syntax, so a declared change is a copy and paste. A row with no
//! builder, or a builder with no row, fails as well. Every enum whose variants
//! the table iterates is listed through `every!`, so a new variant does not
//! compile until it is listed, and once listed it is built and needs a row.
//! Some rows hash the same bytes under two labels: they are different entry
//! points reaching one request, each pinned in its own name.
//!
//! The `template` rows are [`ReadOnlyProfileId::template_sha256`], the hash
//! of each profile's UTF-16LE template that `xml_read_profiles_tests` already
//! seals; they are here so this one table covers every rendering.

use std::collections::BTreeSet;

use bridge_tally_primitives::{company_guid::CompanyGuid, TallyDate};

use crate::import_envelope::{render_import_envelope, ImportReport};
use crate::ledger_census::{
    render_ledger_census_slice_request, render_stock_group_census_slice_request,
    render_stock_item_census_slice_request, CensusLimits, CensusPlan, LedgerCensusPlan,
    StockGroups, StockItems,
};
use crate::ledger_guid_part::{LedgerGuid, LedgerGuidPart};
use crate::ledger_scope::LedgerScopeName;
use crate::native_cash_flow::{render_native_cash_flow_request, WholeMonthWindow};
use crate::native_company_features::render_company_features_request;
use crate::native_funds_flow::render_native_funds_flow_request;
use crate::native_godown_summary::{
    render_native_godown_summary_request, render_native_stock_summary_exploded_request,
};
use crate::native_masters::{render_native_masters_request, NativeMasterKind};
use crate::native_month_register::{render_native_month_register_request, MonthRegisterKind};
use crate::native_negative_ledgers::render_native_negative_ledgers_request;
use crate::native_negative_stock::render_native_negative_stock_request;
use crate::native_outstandings::{
    render_company_base_currency_request, render_company_currency_request,
    render_company_currency_request_with_originalname, render_ledger_outstandings_request,
    render_native_bills_request, render_native_group_snapshot_request,
    render_native_ledger_export_request, render_native_ledger_snapshot_request,
    render_native_ledger_snapshot_request_for_ledgers,
    render_native_ledger_snapshot_request_for_parents, render_native_voucher_export_request,
    render_native_voucher_type_export_request, render_party_ledger_master_request,
    render_party_ledger_master_request_for_ledgers, render_party_ledger_master_request_for_parents,
    render_party_ledger_master_request_for_parents_with_gst_details,
    render_party_ledger_master_request_for_parents_with_mailing_details,
    render_party_ledger_master_request_with_gst_details,
    render_party_ledger_master_request_with_mailing_details, NativeBillsReportKind,
    NativeLedgerExportPeriod, NativeLedgerSnapshotPeriod,
};
use crate::native_statement_reports::{
    render_built_in_report_request, render_exploded_stock_summary_request,
    render_native_statement_request, BuiltInReport, NativeStatementKind,
};
use crate::native_stock_query::{render_stock_query_request, MeasuredItemName};
use crate::native_stock_summary::{
    render_company_inventory_flags_request, render_native_stock_summary_request,
};
use crate::native_trial_balance::{
    render_native_trial_balance_request, render_native_trial_balance_request_with_currency,
};
use crate::native_voucher_type_numbering::render_voucher_type_numbering_request;
use crate::outstandings_shared::{
    render_company_book_extent_v2, render_company_ledger_count_request, DateBoundaryProfile,
};
use crate::parent_partition::{ParentObservation, ParentPart, ParentPartition, PartitionLimits};
use crate::xml_read_profiles::{
    compatibility, ReadOnlyProfile, ReadOnlyProfileId, ValidatedCompanyName, ValidatedDateRange,
};

/// Every variant of a fieldless enum, in the order named: the list the table
/// iterates and an exhaustive `match` are written from the one token list, so
/// a new variant fails to compile until it is named here, and from then on it
/// is iterated and gets a row. A variant named twice is an unreachable pattern
/// and builds two rows under one label, which `assert_matches_table` refuses.
macro_rules! every {
    ($enum:ident: $($variant:ident),+ $(,)?) => {{
        let _exhaustive = |value: $enum| match value {
            $($enum::$variant)|+ => (),
        };
        [$($enum::$variant),+]
    }};
}

/// One synthetic company name holding `&`, `<`, a double quote and non-ASCII
/// text, used by every builder that takes a company.
const COMPANY: &str = "Synthetic & Co <Lab> \"Q\" Ünïcode";

const TABLE: &[(&str, &str)] = &[
    (
        "render_import_envelope/Vouchers",
        "da80dbc474c973548df597c820dcd140d57345313c110d7732a2482f87c9ead8",
    ),
    (
        "render_import_envelope/AllMasters",
        "4b82707366c2b254fcd2dfd7b0c4ed0e334f3bb597942409159cb46812d21112",
    ),
    (
        "render_native_masters_request/VoucherTypes",
        "dedf017f45b240752ac67730fccd50248fb069e51fe39a0353e795c472a3ee9b",
    ),
    (
        "render_native_masters_request/Godowns",
        "4406eb8a721f9feeb3cd3d0470e5aa34b6274355f19f826abc34f2cbf246641e",
    ),
    (
        "render_native_masters_request/Units",
        "f770f30291db013086e095699575da543aaa67ad11430cf8342eb92a0deedea0",
    ),
    (
        "render_native_masters_request/StockGroups",
        "28870e661f3033d826cba42bb0ec5913a577ff414cf0b236f5214571150b3c32",
    ),
    (
        "render_native_masters_request/StockItems",
        "0b9af78e06244f7f6a834216c67d816331cd541afb03a862734f9e633212f3c3",
    ),
    (
        "render_native_masters_request/CostCentres",
        "f07fc23794570bda459fa5f47cc428f28c223ded77ccd3645163b4513e491f39",
    ),
    (
        "render_native_masters_request/CostCategories",
        "2152d00178228ce5857cac6266182b47a635c7b4c01c0b6832ec7490c37aba88",
    ),
    (
        "render_company_book_extent_v2",
        "940379899c14c5297c44fe100d953cb5b34a2d5468252c4445f98d695f495520",
    ),
    (
        "render_company_ledger_count_request",
        "9fbe77ad250b1d555249eb62646cf3c57687e651d6de277c5e5009ccfc0db6c6",
    ),
    (
        "ReadOnlyProfile::CompanyListV2/render",
        "21d038e239e54846a3d6031aaf09a76307a57aa2433074a9304695a24e19b1a1",
    ),
    (
        "ReadOnlyProfile::CompanyBookExtentV2/render",
        "940379899c14c5297c44fe100d953cb5b34a2d5468252c4445f98d695f495520",
    ),
    (
        "ReadOnlyProfile::StandardLedgerIdentityV1/render",
        "06bc9e454e79ceaf55b61591496f47690e2cba8e27237931ee68cd614fd30e5f",
    ),
    (
        "ReadOnlyProfile::StandardLedgerCatalogV1/render",
        "06bc9e454e79ceaf55b61591496f47690e2cba8e27237931ee68cd614fd30e5f",
    ),
    (
        "ReadOnlyProfile::StandardLedgerCatalogV2/render",
        "bbccf1242fbe43609a9de7c0fb303cc00fa0b884034e74bbf13c36a0a343823d",
    ),
    (
        "ReadOnlyProfile::StandardLedgerCatalogV3/render",
        "0c1486f03ab78e91806daceb4b5e69006c31f450297e92297df0a716009a6d4b",
    ),
    (
        "ReadOnlyProfile::AuditCompanyObjectV1/render",
        "e584c6ff83b8d7abc968d84e7f00c1ad852579fb625357910b912777b70b203d",
    ),
    (
        "ReadOnlyProfile::InvoiceCompanyHsnV1/render",
        "b6039810405624cece3d12e0d44f8dbb0f3479b20490fdaacdb61c8abcdb1fc5",
    ),
    (
        "ReadOnlyProfile::AuditLedgersV1/render",
        "1b2628f08e8614f980043abf10a84f8a809dfcee22d2721de3d4875d654e3c6c",
    ),
    (
        "ReadOnlyProfile::AuditVouchersV1/render",
        "1c6b4f498ba8bf2e9765d59a716fc54163cf552c1fb3e56fd82f0dbe598a3303",
    ),
    (
        "ReadOnlyProfile::AuditStockItemsV1/render",
        "a445b9dd100809b3753c18a94a7594e82a268c70858e12d1e8854fc5adb1630e",
    ),
    (
        "compatibility::standard_ledger_identity_request",
        "06bc9e454e79ceaf55b61591496f47690e2cba8e27237931ee68cd614fd30e5f",
    ),
    (
        "compatibility::standard_ledger_catalog_request",
        "06bc9e454e79ceaf55b61591496f47690e2cba8e27237931ee68cd614fd30e5f",
    ),
    (
        "render_company_inventory_flags_request",
        "17117b83acbbb905dd3436fd891f23aa2e14b168415d8e581059f6dc4b8ee924",
    ),
    (
        "render_native_stock_summary_request",
        "7a1e687afce2b4bcfac14eb18213536910d28fb28988078c9c5d5700f8a58fc3",
    ),
    (
        "render_company_features_request",
        "ec75d12177168b93816746cd1c715bc5d3feb5fc9bbc7c5e1a087d988ed5e7f7",
    ),
    (
        "render_ledger_outstandings_request",
        "b86653bdb09ce9c53a8ce2a985451784d6672eb4c024997c12b86a8e80a8d41a",
    ),
    (
        "render_ledger_census_slice_request/(10,20]",
        "5928d6e46ca2cd18abcd70aa6ba9ec073300bb1ee703123ead7f904b26ad035f",
    ),
    (
        "render_stock_item_census_slice_request/(10,20]",
        "a9a167709accd41699a2e3b1ed656aded0886a569f04bc5db75c555035527c2d",
    ),
    (
        "render_stock_group_census_slice_request/(10,20]",
        "aba0c2d0cda86f11c650e733a025e3103733a0c3e62b2ec142f7932e5c5f80fd",
    ),
    (
        "render_native_statement_request/BalanceSheet",
        "158e3fd1bd264aae45e7fe8c74b48a8e16d1e598836bb3156a0315c79b5e7258",
    ),
    (
        "render_native_statement_request/ProfitAndLoss",
        "06042c762461329183295d772c81c0079d3f7ec99505afdf98cd315a6a8fc0cb",
    ),
    (
        "render_built_in_report_request/BalanceSheet",
        "158e3fd1bd264aae45e7fe8c74b48a8e16d1e598836bb3156a0315c79b5e7258",
    ),
    (
        "render_built_in_report_request/ProfitAndLoss",
        "06042c762461329183295d772c81c0079d3f7ec99505afdf98cd315a6a8fc0cb",
    ),
    (
        "render_built_in_report_request/SalesRegister",
        "2eee76837083543d06d79f8f4628507a2bbb3ac8a7adbfc7579d0dfd01d1baaf",
    ),
    (
        "render_built_in_report_request/PurchaseRegister",
        "f2372bd2c9c1fc7125c7ea14dc87dc6ba7be659999fed84b96e5f2faab08c1c7",
    ),
    (
        "render_built_in_report_request/CashFlow",
        "b316436de9a681208b9ca047dc95949066560a6ebf6acdf7b0da4a353d3045ed",
    ),
    (
        "render_built_in_report_request/FundsFlow",
        "76a5f567965c33d72b4b701db89176f1b4c17654fb8690343eeb0d0ff02778c1",
    ),
    (
        "render_built_in_report_request/NegativeLedgers",
        "183a1535cfedccdec53d9c01835b8ec317150b2a699ca6d4bd60f76de227d3cf",
    ),
    (
        "render_built_in_report_request/NegativeStock",
        "2bd46c7b6db46987aa8fd720630b239052db38a27130e174f294e8cc63194277",
    ),
    (
        "render_built_in_report_request/StockSummary",
        "7a1e687afce2b4bcfac14eb18213536910d28fb28988078c9c5d5700f8a58fc3",
    ),
    (
        "render_built_in_report_request/GodownSummary",
        "927fe4d06cf12a466957b78b68d699455a94e52a85e6fae59217eba3ff982472",
    ),
    (
        "render_exploded_stock_summary_request",
        "7c8c168f7ee1f223be8fe63de99fdbd9539085f0282c2f752ef60e99b619fa44",
    ),
    (
        "render_stock_query_request",
        "76814625f9ce11345be0691e81e4cda8fd92657ea7400633f5233fe1ff36413f",
    ),
    (
        "render_native_funds_flow_request",
        "76a5f567965c33d72b4b701db89176f1b4c17654fb8690343eeb0d0ff02778c1",
    ),
    (
        "render_native_cash_flow_request",
        "b316436de9a681208b9ca047dc95949066560a6ebf6acdf7b0da4a353d3045ed",
    ),
    (
        "render_native_month_register_request/Sales",
        "2eee76837083543d06d79f8f4628507a2bbb3ac8a7adbfc7579d0dfd01d1baaf",
    ),
    (
        "render_native_month_register_request/Purchase",
        "f2372bd2c9c1fc7125c7ea14dc87dc6ba7be659999fed84b96e5f2faab08c1c7",
    ),
    (
        "render_native_negative_ledgers_request",
        "183a1535cfedccdec53d9c01835b8ec317150b2a699ca6d4bd60f76de227d3cf",
    ),
    (
        "render_native_godown_summary_request",
        "927fe4d06cf12a466957b78b68d699455a94e52a85e6fae59217eba3ff982472",
    ),
    (
        "render_native_stock_summary_exploded_request",
        "7c8c168f7ee1f223be8fe63de99fdbd9539085f0282c2f752ef60e99b619fa44",
    ),
    (
        "render_native_negative_stock_request",
        "2bd46c7b6db46987aa8fd720630b239052db38a27130e174f294e8cc63194277",
    ),
    (
        "render_native_trial_balance_request",
        "618cfb528eabb396a4ef3fbb1602498b9f87f16a27282e057f28caa2574fee86",
    ),
    (
        "render_native_trial_balance_request_with_currency",
        "34ac990596ef5007ef57b0a9927eadcf50878660546da095fbe23134e3c562b4",
    ),
    (
        "render_native_bills_request/Receivable",
        "0831de23987bea054e4c89e76e1766defb667f71de945bf60604488ae972edb7",
    ),
    (
        "render_native_bills_request/Payable",
        "1115cfb85ab61659c05dc341e336cf82521ce958cd5c08b9da7f5be22d1341c1",
    ),
    (
        "render_native_ledger_snapshot_request",
        "70d69801d89b4bfd321155cf5e438bf77d7e552d38b96e1a6c4fdec9c5b49b1d",
    ),
    (
        "render_native_ledger_snapshot_request_for_parents",
        "96b1eb0bac421aca2c90cd6316bcb3bc7b13273a28d9b28d413eab636d6ab6a7",
    ),
    (
        "render_native_ledger_snapshot_request_for_ledgers",
        "6d9b0549987d72a9a0064fa15fa7a2334158c5ff20d6f60312b87fb82f1c4463",
    ),
    (
        "render_native_ledger_export_request",
        "6f596e07e33902beeba3ab309bb1304bfcd1bfec7cab359ae035c98462a9be4c",
    ),
    (
        "render_party_ledger_master_request",
        "dba11287151e74ff4660aea619ceb3c40006b01fa7d6ca81e259840fdc6048ff",
    ),
    (
        "render_party_ledger_master_request_with_mailing_details",
        "aeb2be5e2bc711f1119ae603984b7e89db4a03e0864909944db68e57f89608dc",
    ),
    (
        "render_party_ledger_master_request_with_gst_details",
        "2835a09ec50c2c59df84dbc91898223b664b7f1fc9fcb87c9a7fffbef865bd3c",
    ),
    (
        "render_party_ledger_master_request_for_parents",
        "51efac428583fe37a4066a63226aacb2db0b888b7205df4e5467ebfc9a0a9375",
    ),
    (
        "render_party_ledger_master_request_for_ledgers",
        "26d1b737971e0fc13806e1a7c32b72c8a247cd51b41d672e5eac08126bedc598",
    ),
    (
        "render_party_ledger_master_request_for_parents_with_mailing_details",
        "3621e3856176b147574ed992d70d0962b76a16ec43c14d9354f5d6836648cd9f",
    ),
    (
        "render_party_ledger_master_request_for_parents_with_gst_details",
        "cb8ff3473d21e57b7ae9aaa119aeb46e6216a154d9f13e3a6be0100c12b98c50",
    ),
    (
        "render_native_voucher_type_export_request",
        "202757a53ef0e75e8b189d88390dc16a8a95f038ea18b21bd733a5121adeae4b",
    ),
    (
        "render_voucher_type_numbering_request",
        "956a186baa0eabd186641bfe85bf37026ba67470a4cb9587ebff0728bd75b7af",
    ),
    (
        "render_native_voucher_export_request",
        "9ba288ce7ed2e4cb30c4fbc17a506162636316f043479c5417625986832bbcdf",
    ),
    (
        "render_native_group_snapshot_request",
        "b4e1b12be94ca92771de60b88ef375eb96b90cad92605a284e9f79e0c5e81d1f",
    ),
    (
        "render_company_currency_request",
        "786570bda7e3a361fcebd3497b87356e2b3a249492e836a0987ac6206eb043ab",
    ),
    (
        "render_company_currency_request_with_originalname",
        "78191c77f509228da7116628fbde1eeb37a16077fd65993ccddd47f5dfccb8a0",
    ),
    (
        "render_company_base_currency_request",
        "aabd1ea8f509571bff8a5df888cdc8bec81de0c91539df65373e4ac47d9a888d",
    ),
    (
        "ReadOnlyProfileId::CompanyListV2/template",
        "9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8",
    ),
    (
        "ReadOnlyProfileId::CompanyBookExtentV2/template",
        "28ffc66bfaee2172ac5de37ab358951ae2439ec2f2fc6b5aeb220624d8d87fdc",
    ),
    (
        "ReadOnlyProfileId::StandardLedgerIdentityV1/template",
        "3f38d58a88c8bb99180290b2fb17a3057d6149f78d81e08ba3d9bfc2d595dd95",
    ),
    (
        "ReadOnlyProfileId::StandardLedgerCatalogV1/template",
        "3f38d58a88c8bb99180290b2fb17a3057d6149f78d81e08ba3d9bfc2d595dd95",
    ),
    (
        "ReadOnlyProfileId::StandardLedgerCatalogV2/template",
        "e93bbf843455db2a4c9f77991c52b16f10b16bbb792af5ecae4909a48216a62e",
    ),
    (
        "ReadOnlyProfileId::StandardLedgerCatalogV3/template",
        "5bdc4b7c5c1baec328934aa5f090efd72ac821956defe63a58a4cc44909d9462",
    ),
    (
        "ReadOnlyProfileId::AuditCompanyObjectV1/template",
        "ff023f06940b80c8e922d8b238772cd77e18f220b2fd7a4be7af667029948acf",
    ),
    (
        "ReadOnlyProfileId::InvoiceCompanyHsnV1/template",
        "bb3efcb756ab15771e58e1daaa543af6bb73b0e32a43a723f0d1ed7d11219703",
    ),
    (
        "ReadOnlyProfileId::AuditLedgersV1/template",
        "788333480a34bd0af0f5d724f2ac5cb8dd4d888c4a8db8e67039583fc295aa5f",
    ),
    (
        "ReadOnlyProfileId::AuditVouchersV1/template",
        "e099da0dcb7426d833e1d2bf1eecc848ed5015847761d803cbb2a8cd6720f8a2",
    ),
    (
        "ReadOnlyProfileId::AuditStockItemsV1/template",
        "18f8280794a476fd3b44f55c63a9f3060b7a523da82331744fe4fc0b0a151337",
    ),
];

fn date(value: &str) -> TallyDate {
    TallyDate::parse(value).expect("a fixed calendar date")
}

/// The synthetic company GUID `company_name_escaping_tests` uses.
fn company_guid() -> CompanyGuid {
    CompanyGuid::parse("3a6bd6e1-b835-4bff-89dd-8a6af138c346").expect("a GUID")
}

/// One parent part naming two parents, as `native_outstandings` request tests
/// build it.
fn parent_part() -> ParentPart {
    let rows = [
        ("l0", "g0", ParentObservation::from(Some("Sundry Debtors"))),
        ("l1", "g1", ParentObservation::from(Some("Duties & Taxes"))),
    ];
    let partition = ParentPartition::plan(
        rows,
        PartitionLimits {
            max_ledgers_per_part: 100,
            max_parents_per_part: 100,
            max_parts: 1,
            max_complement_formula_bytes: 10_000,
        },
    )
    .expect("a one-part plan");
    partition.parts()[0].clone()
}

/// The three-ledger part of the `native_outstandings` ledger-part test.
fn ledger_part() -> LedgerGuidPart {
    let company = "6b43e498-430c-4d5c-bfef-d32e2ab93c85";
    LedgerGuidPart::new(
        ["000000d8", "000000d0", "000000d5"]
            .map(|suffix| LedgerGuid::parse(&format!("{company}-{suffix}")).unwrap()),
    )
    .expect("a ledger part")
}

/// The census limits of `ledger_census_tests`; the second slice of a plan to
/// 25 is `(10, 20]`.
const CENSUS_LIMITS: CensusLimits = CensusLimits {
    slice_width: 10,
    max_slices: 5,
};

/// Every request this crate builds, by label, in table order.
fn built_requests() -> Vec<(String, String)> {
    let company = ValidatedCompanyName::new(COMPANY).expect("an admitted company name");
    let (from, to) = (date("20250401"), date("20260331"));
    let snapshot = NativeLedgerSnapshotPeriod::new(
        DateBoundaryProfile::ModeAgnostic,
        from.clone(),
        to.clone(),
    )
    .expect("a snapshot period");
    let export =
        NativeLedgerExportPeriod::new(DateBoundaryProfile::ModeAgnostic, from.clone(), to.clone())
            .expect("an export period");
    let range = ValidatedDateRange::new("20250401", "20260331").expect("a date range");
    let months = WholeMonthWindow::new(DateBoundaryProfile::ModeAgnostic, from.clone(), to.clone())
        .expect("a twelve-month window");
    let parents = parent_part();
    let ledgers = ledger_part();
    let party = LedgerScopeName::new("Lab Party P1").expect("an admitted ledger name");
    let item = MeasuredItemName::new("Kit & Co Item").expect("an admitted item name");
    let guid = company_guid();

    let mut rows: Vec<(String, String)> = Vec::new();
    let mut row = |label: String, request: String| rows.push((label, request));

    for report in every!(ImportReport: Vouchers, AllMasters) {
        let name = match report {
            ImportReport::Vouchers => "Vouchers",
            ImportReport::AllMasters => "AllMasters",
        };
        row(
            format!("render_import_envelope/{name}"),
            render_import_envelope(COMPANY, report, "<TALLYMESSAGE/>"),
        );
    }
    // Not `NativeMasterKind::ALL`: nothing makes that list name every kind.
    for kind in every!(NativeMasterKind:
        VoucherTypes,
        Godowns,
        Units,
        StockGroups,
        StockItems,
        CostCentres,
        CostCategories,
    ) {
        row(
            format!("render_native_masters_request/{kind:?}"),
            render_native_masters_request(kind, COMPANY),
        );
    }
    // The same request as the `ReadOnlyProfile::CompanyBookExtentV2` row.
    row(
        "render_company_book_extent_v2".into(),
        render_company_book_extent_v2(COMPANY),
    );
    row(
        "render_company_ledger_count_request".into(),
        render_company_ledger_count_request(COMPANY),
    );

    let profile_ids = every!(ReadOnlyProfileId:
        CompanyListV2,
        CompanyBookExtentV2,
        StandardLedgerIdentityV1,
        StandardLedgerCatalogV1,
        StandardLedgerCatalogV2,
        StandardLedgerCatalogV3,
        AuditCompanyObjectV1,
        InvoiceCompanyHsnV1,
        AuditLedgersV1,
        AuditVouchersV1,
        AuditStockItemsV1,
    );
    for id in profile_ids {
        // The profile each id names; `every!` puts each id in the list.
        let profile = match id {
            // The control row: discovery names no company.
            ReadOnlyProfileId::CompanyListV2 => ReadOnlyProfile::CompanyListV2,
            ReadOnlyProfileId::CompanyBookExtentV2 => {
                ReadOnlyProfile::CompanyBookExtentV2 { company: &company }
            }
            // V1 identity and V1 catalogue render one request (and template).
            ReadOnlyProfileId::StandardLedgerIdentityV1 => {
                ReadOnlyProfile::StandardLedgerIdentityV1 { company: &company }
            }
            ReadOnlyProfileId::StandardLedgerCatalogV1 => {
                ReadOnlyProfile::StandardLedgerCatalogV1 { company: &company }
            }
            ReadOnlyProfileId::StandardLedgerCatalogV2 => {
                ReadOnlyProfile::StandardLedgerCatalogV2 { company: &company }
            }
            ReadOnlyProfileId::StandardLedgerCatalogV3 => {
                ReadOnlyProfile::StandardLedgerCatalogV3 { company: &company }
            }
            ReadOnlyProfileId::AuditCompanyObjectV1 => {
                ReadOnlyProfile::AuditCompanyObjectV1 { company: &company }
            }
            ReadOnlyProfileId::InvoiceCompanyHsnV1 => {
                ReadOnlyProfile::InvoiceCompanyHsnV1 { company: &company }
            }
            ReadOnlyProfileId::AuditLedgersV1 => ReadOnlyProfile::AuditLedgersV1 {
                company: &company,
                period: &range,
            },
            ReadOnlyProfileId::AuditVouchersV1 => ReadOnlyProfile::AuditVouchersV1 {
                company: &company,
                window: &range,
            },
            ReadOnlyProfileId::AuditStockItemsV1 => ReadOnlyProfile::AuditStockItemsV1 {
                company: &company,
                period: &range,
            },
        };
        assert_eq!(profile.id(), id);
        row(format!("ReadOnlyProfile::{id:?}/render"), profile.render());
    }
    // The two compatibility entry points reach the same request as the
    // StandardLedgerIdentityV1 and StandardLedgerCatalogV1 rows.
    row(
        "compatibility::standard_ledger_identity_request".into(),
        compatibility::standard_ledger_identity_request(COMPANY),
    );
    row(
        "compatibility::standard_ledger_catalog_request".into(),
        compatibility::standard_ledger_catalog_request(COMPANY),
    );

    row(
        "render_company_inventory_flags_request".into(),
        render_company_inventory_flags_request(COMPANY, &guid),
    );
    row(
        "render_native_stock_summary_request".into(),
        render_native_stock_summary_request(&company, &snapshot),
    );
    row(
        "render_company_features_request".into(),
        render_company_features_request(COMPANY, &guid),
    );
    row(
        "render_ledger_outstandings_request".into(),
        render_ledger_outstandings_request(COMPANY, &party, &to),
    );

    let ledger_plan = LedgerCensusPlan::new(25, CENSUS_LIMITS).expect("a census plan");
    row(
        "render_ledger_census_slice_request/(10,20]".into(),
        render_ledger_census_slice_request(COMPANY, &ledger_plan.slices()[1]),
    );
    let item_plan = CensusPlan::<StockItems>::new(25, CENSUS_LIMITS).expect("a census plan");
    row(
        "render_stock_item_census_slice_request/(10,20]".into(),
        render_stock_item_census_slice_request(COMPANY, &item_plan.slices()[1]),
    );
    let group_plan = CensusPlan::<StockGroups>::new(25, CENSUS_LIMITS).expect("a census plan");
    row(
        "render_stock_group_census_slice_request/(10,20]".into(),
        render_stock_group_census_slice_request(COMPANY, &group_plan.slices()[1]),
    );

    for kind in every!(NativeStatementKind: BalanceSheet, ProfitAndLoss) {
        let name = match kind {
            NativeStatementKind::BalanceSheet => "BalanceSheet",
            NativeStatementKind::ProfitAndLoss => "ProfitAndLoss",
        };
        row(
            format!("render_native_statement_request/{name}"),
            render_native_statement_request(kind, &company, &snapshot),
        );
    }
    // Each public report builder below (statement, stock summary, funds and
    // cash flow, month register, negative ledgers and stock, godown summary)
    // reaches the same request as its row here: different entry points, one
    // request. Not `BuiltInReport::ALL`: nothing makes that list name every report.
    for report in every!(BuiltInReport:
        BalanceSheet,
        ProfitAndLoss,
        SalesRegister,
        PurchaseRegister,
        CashFlow,
        FundsFlow,
        NegativeLedgers,
        NegativeStock,
        StockSummary,
        GodownSummary,
    ) {
        row(
            format!("render_built_in_report_request/{report:?}"),
            render_built_in_report_request(report, &company, &snapshot),
        );
    }
    // The same request as `render_native_stock_summary_exploded_request` below.
    row(
        "render_exploded_stock_summary_request".into(),
        render_exploded_stock_summary_request(&company, &snapshot),
    );
    row(
        "render_stock_query_request".into(),
        render_stock_query_request(&company, &item, &snapshot),
    );
    row(
        "render_native_funds_flow_request".into(),
        render_native_funds_flow_request(&company, &months),
    );
    row(
        "render_native_cash_flow_request".into(),
        render_native_cash_flow_request(&company, &months),
    );
    for kind in every!(MonthRegisterKind: Sales, Purchase) {
        let name = match kind {
            MonthRegisterKind::Sales => "Sales",
            MonthRegisterKind::Purchase => "Purchase",
        };
        row(
            format!("render_native_month_register_request/{name}"),
            render_native_month_register_request(kind, &company, &months),
        );
    }
    row(
        "render_native_negative_ledgers_request".into(),
        render_native_negative_ledgers_request(&company, &snapshot),
    );
    row(
        "render_native_godown_summary_request".into(),
        render_native_godown_summary_request(&company, &snapshot),
    );
    row(
        "render_native_stock_summary_exploded_request".into(),
        render_native_stock_summary_exploded_request(&company, &snapshot),
    );
    row(
        "render_native_negative_stock_request".into(),
        render_native_negative_stock_request(&company, &snapshot),
    );

    row(
        "render_native_trial_balance_request".into(),
        render_native_trial_balance_request(&company, &snapshot),
    );
    row(
        "render_native_trial_balance_request_with_currency".into(),
        render_native_trial_balance_request_with_currency(&company, &snapshot),
    );

    for kind in every!(NativeBillsReportKind: Receivable, Payable) {
        let name = match kind {
            NativeBillsReportKind::Receivable => "Receivable",
            NativeBillsReportKind::Payable => "Payable",
        };
        row(
            format!("render_native_bills_request/{name}"),
            render_native_bills_request(kind, &company, &from, &to),
        );
    }
    row(
        "render_native_ledger_snapshot_request".into(),
        render_native_ledger_snapshot_request(&company, &snapshot),
    );
    row(
        "render_native_ledger_snapshot_request_for_parents".into(),
        render_native_ledger_snapshot_request_for_parents(&company, &snapshot, &parents),
    );
    row(
        "render_native_ledger_snapshot_request_for_ledgers".into(),
        render_native_ledger_snapshot_request_for_ledgers(&company, &snapshot, &ledgers),
    );
    row(
        "render_native_ledger_export_request".into(),
        render_native_ledger_export_request(&company, &export),
    );
    row(
        "render_party_ledger_master_request".into(),
        render_party_ledger_master_request(&company, &export),
    );
    row(
        "render_party_ledger_master_request_with_mailing_details".into(),
        render_party_ledger_master_request_with_mailing_details(&company, &export),
    );
    row(
        "render_party_ledger_master_request_with_gst_details".into(),
        render_party_ledger_master_request_with_gst_details(&company, &export),
    );
    row(
        "render_party_ledger_master_request_for_parents".into(),
        render_party_ledger_master_request_for_parents(&company, &export, &parents),
    );
    row(
        "render_party_ledger_master_request_for_ledgers".into(),
        render_party_ledger_master_request_for_ledgers(&company, &export, &ledgers),
    );
    row(
        "render_party_ledger_master_request_for_parents_with_mailing_details".into(),
        render_party_ledger_master_request_for_parents_with_mailing_details(
            &company, &export, &parents,
        ),
    );
    row(
        "render_party_ledger_master_request_for_parents_with_gst_details".into(),
        render_party_ledger_master_request_for_parents_with_gst_details(
            &company, &export, &parents,
        ),
    );
    row(
        "render_native_voucher_type_export_request".into(),
        render_native_voucher_type_export_request(&company),
    );
    row(
        "render_voucher_type_numbering_request".into(),
        render_voucher_type_numbering_request(&company),
    );
    row(
        "render_native_voucher_export_request".into(),
        render_native_voucher_export_request(&company, &from, &to),
    );
    row(
        "render_native_group_snapshot_request".into(),
        render_native_group_snapshot_request(&company),
    );
    row(
        "render_company_currency_request".into(),
        render_company_currency_request(&company),
    );
    row(
        "render_company_currency_request_with_originalname".into(),
        render_company_currency_request_with_originalname(&company),
    );
    row(
        "render_company_base_currency_request".into(),
        render_company_base_currency_request(&company),
    );

    let mut hashed: Vec<(String, String)> = rows
        .into_iter()
        .map(|(label, request)| (label, crate::sha256_hex(request.as_bytes())))
        .collect();
    for id in profile_ids {
        hashed.push((
            format!("ReadOnlyProfileId::{id:?}/template"),
            id.template_sha256(),
        ));
    }
    hashed
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
fn request_bytes_of_every_builder_match_the_pinned_table() {
    assert_matches_table(TABLE, &built_requests());
}
