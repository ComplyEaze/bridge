use super::*;

#[test]
fn scalar_schema_limits_match_existing_boundary_admission() {
    let schema = voucher_input_schema();
    let properties = &schema["properties"]["vouchers"]["items"]["properties"];
    let entry = &properties["entries"]["items"]["properties"];
    assert_eq!(
        entry["amount"]["maxLength"],
        bridge_tally_core::MAX_EXACT_DECIMAL_BYTES
    );
    for (value, valid) in [
        ("0.00".to_string(), false),
        ("000.00".to_string(), false),
        ("0.01".to_string(), true),
        ("000.01".to_string(), true),
        (format!("{}.99", "9".repeat(253)), true),
        (format!("{}.99", "9".repeat(254)), false),
    ] {
        assert_eq!(valid_2dp_amount(&value), valid);
    }
    let whitespace: Vec<char> = (0..=char::MAX as u32)
        .filter_map(char::from_u32)
        .filter(|ch| ch.is_whitespace() && !ch.is_control())
        .collect();
    let expected: Vec<char> = [0x20, 0xa0, 0x1680]
        .into_iter()
        .chain(0x2000..=0x200a)
        .chain([0x2028, 0x2029, 0x202f, 0x205f, 0x3000])
        .map(|value| char::from_u32(value).unwrap())
        .collect();
    assert_eq!(
        whitespace, expected,
        "schema whitespace table must track Rust trim semantics"
    );
    for character in whitespace {
        let mut input = captured_catalogue_payload();
        input.vouchers[0].entries[0].ledger = character.to_string();
        assert_eq!(payload_verdict(&input), Err("voucher_entry_invalid".into()));
    }
}

#[test]
fn optional_text_constraints_publish_unicode_lengths_and_reject_controls() {
    let schema = voucher_input_schema();
    let properties = &schema["properties"]["vouchers"]["items"]["properties"];
    for field in ["narration", "reference"] {
        assert_eq!(properties[field]["minLength"], 1);
        assert_eq!(properties[field]["maxLength"], 2000);
        assert_eq!(
            properties[field]["not"]["anyOf"][0]["pattern"],
            r"[\u0000-\u001F\u007F-\u009F]"
        );
        assert_eq!(
            properties[field]["not"]["anyOf"][1]["pattern"],
            r"\[[Bb][Rr][Ii][Dd][Gg][Ee]:"
        );
        let mut input = captured_catalogue_payload();
        for (text, expected) in [
            (None, Ok(())),
            (Some("a".repeat(2000)), Ok(())),
            (Some("क".repeat(2000)), Ok(())),
            (Some("🧾".repeat(2000)), Ok(())),
            (Some(String::new()), Err("voucher_text_invalid".to_string())),
            (
                Some("a".repeat(2001)),
                Err("voucher_text_invalid".to_string()),
            ),
            (
                Some("क".repeat(2001)),
                Err("voucher_text_invalid".to_string()),
            ),
            (
                Some("🧾".repeat(2001)),
                Err("voucher_text_invalid".to_string()),
            ),
        ] {
            match field {
                "narration" => input.vouchers[0].narration = text,
                "reference" => input.vouchers[0].reference = text,
                _ => unreachable!(),
            }
            assert_eq!(payload_verdict(&input), expected, "{field}");
        }
        for codepoint in (0..=0x1f).chain(0x7f..=0x9f) {
            let text = Some(format!("before{}after", char::from_u32(codepoint).unwrap()));
            match field {
                "narration" => input.vouchers[0].narration = text,
                "reference" => input.vouchers[0].reference = text,
                _ => unreachable!(),
            }
            assert_eq!(payload_verdict(&input), Err("voucher_text_invalid".into()));
        }
    }
    assert_eq!(
        properties["voucher_number"]["not"]["pattern"],
        r"[\u0000-\u001F\u007F-\u009F$]|^\s|\s$"
    );
}

#[test]
fn maximum_unicode_text_is_preserved_by_import_rendering() {
    let mut input = admitted_captured_payload();
    input.vouchers[0].narration = Some("क".repeat(2000));
    input.vouchers[0].reference = Some("🧾".repeat(2000));
    payload_verdict(&input).unwrap();
    let xml = render_import_xml("WR2 Unicode Lab", &input.vouchers, "text-boundary");
    let mut reader = quick_xml::Reader::from_str(&xml);
    let mut narration = None;
    let mut reference = None;
    loop {
        match reader.read_event().unwrap() {
            quick_xml::events::Event::Start(tag)
                if matches!(tag.name().as_ref(), "NARRATION" | "REFERENCE") =>
            {
                let value = reader.read_text(tag.name()).unwrap().to_string();
                if tag.name().as_ref() == "NARRATION" && narration.is_none() {
                    narration = Some(value);
                } else if tag.name().as_ref() == "REFERENCE" && reference.is_none() {
                    reference = Some(value);
                }
            }
            quick_xml::events::Event::Eof => break,
            _ => {}
        }
    }
    assert_eq!(
        narration.unwrap(),
        format!(
            "{} [BRIDGE:{}]",
            "क".repeat(2000),
            import_identity("text-boundary", "txn-001")
        )
    );
    assert_eq!(reference.unwrap(), "🧾".repeat(2000));
}

/// The texts an assistant reads before it calls say what #1688 added: `validate_masters` names
/// `alias_of` and that it is not a binding, and the ledger field says the twin check counts
/// aliases and names `through_alias`.
#[test]
fn the_ledger_texts_name_alias_of_and_the_alias_twin() {
    let tools = crate::agent::catalog::registered_tool_definitions(true, true);
    let validate = tools
        .as_array()
        .expect("tools")
        .iter()
        .find(|tool| tool["name"] == "validate_masters")
        .and_then(|tool| tool["description"].as_str())
        .expect("validate_masters is in the catalogue");
    assert!(validate.contains("also carries `alias_of`"));
    assert!(validate.contains("the row is still not bound and no alias is ever sent"));
    let schema = voucher_input_schema();
    let ledger = schema["properties"]["vouchers"]["items"]["properties"]["entries"]["items"]
        ["properties"]["ledger"]["description"]
        .as_str()
        .expect("the ledger field has a description");
    assert!(ledger.contains("or to one of its aliases"));
    assert!(ledger.contains("through_alias"));
}

/// #2152: a batch that could be imported or posted says what checks which
/// order. A post after an import is refused by the post's own read of the book
/// (`import_preexisting_identity`); an import after a post is checked by
/// nothing. The posting-off warning and next step say it in the same words.
#[test]
fn the_one_route_sentence_names_the_check_that_exists_and_the_one_that_does_not() {
    const ONE_ROUTE: &str = "Use one route only. Once this batch is posted, do not import this file: nothing checks an import against an earlier post. If this file is imported first, post_import, which Allow voucher posting adds, reads the book before asking for approval and refuses the batch (import_preexisting_identity) if any of its vouchers is already there, matched by its narration tag, by date, voucher type and entries, or for an invoice by its number.";
    let (warnings, next_step) = build_import_guidance(false, None, false, false, false, false);
    assert!(
        warnings[0].as_str().unwrap().ends_with(ONE_ROUTE),
        "{warnings}"
    );
    assert!(next_step.ends_with(ONE_ROUTE), "{next_step}");
    // The stale claim is gone from every text this guidance can return.
    for (writes_enabled, refusal) in [
        (true, None),
        (true, Some("import_post_batch_too_large")),
        (false, None),
        (false, Some("import_post_batch_too_large")),
    ] {
        let (warnings, next_step) =
            build_import_guidance(writes_enabled, refusal, false, false, false, false);
        assert!(
            !warnings
                .to_string()
                .contains("nothing checks an import against a post."),
            "{warnings}"
        );
        assert!(
            !next_step.contains("nothing checks an import against a post."),
            "{next_step}"
        );
    }
}
