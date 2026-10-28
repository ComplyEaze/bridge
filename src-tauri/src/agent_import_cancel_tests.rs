use super::super::render_native_vouchers_xml;
use super::*;

/// The cancel rendering of `vouchers` is their native Create rendering with
/// only each voucher's `ACTION` changed: a byte comparison, after checking the
/// Create rendering carries the attribute exactly once per voucher. Returns
/// the cancel rendering.
pub(in crate::agent) fn assert_cancel_changes_only_the_action(
    company: &str,
    vouchers: &[(&ImportVoucher, Uuid)],
) -> String {
    let create = render_native_vouchers_xml(company, vouchers.iter().copied());
    let cancel = render_native_cancel_xml(
        company,
        vouchers
            .iter()
            .map(|(voucher, id)| (*voucher, RemoteId(*id))),
    );
    assert_eq!(
        create.matches(" ACTION=\"Create\" ").count(),
        vouchers.len()
    );
    assert!(!create.contains("ACTION=\"Cancel\""));
    let expected = create.replace(" ACTION=\"Create\" ", " ACTION=\"Cancel\" ");
    assert_eq!(cancel.as_bytes(), expected.as_bytes());
    cancel
}

#[test]
fn a_remote_id_is_admitted_only_in_the_form_a_native_post_mints() {
    let minted = Uuid::new_v4();
    let text = minted.to_string();
    assert_eq!(RemoteId::parse(&text), Ok(RemoteId(minted)));
    let refused = Err("import_remote_id_not_native");
    for other in [
        String::new(),
        text.to_uppercase(),
        minted.simple().to_string(),
        minted.braced().to_string(),
        minted.urn().to_string(),
        format!(" {text}"),
        format!("{text}\""),
        format!("{text}\" ACTION=\"Delete"),
        Uuid::nil().to_string(),
        // Version 1 and version 7 UUIDs: well formed, but not what a post mints.
        "c232ab00-9414-11ec-b3c8-9f6bdeced846".to_string(),
        "017f22e2-79b0-7cc3-98c4-dc0c0c07398f".to_string(),
        // Version 4 with a variant a v4 generator never sets.
        format!("{}c{}", &text[..19], &text[20..]),
    ] {
        assert_eq!(RemoteId::parse(&other), refused, "{other}");
    }
}
