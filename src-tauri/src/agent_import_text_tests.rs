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
        assert_eq!(ImportAmount::parse(&value).is_ok(), valid);
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

/// The members of a published character class such as `[\u000A\u000D]`: single code points and
/// `-` ranges, each written `\uXXXX`. Any other spelling panics, so a pattern this cannot read
/// fails the test instead of being approximated.
fn class_members(class: &str) -> std::collections::BTreeSet<char> {
    fn point(text: &str) -> (u32, &str) {
        let digits = text
            .strip_prefix("\\u")
            .and_then(|rest| rest.get(..4))
            .unwrap_or_else(|| panic!("not a \\uXXXX escape: {text}"));
        (u32::from_str_radix(digits, 16).unwrap(), &text[6..])
    }
    let mut rest = class
        .strip_prefix('[')
        .and_then(|inner| inner.strip_suffix(']'))
        .unwrap_or_else(|| panic!("not a bare character class: {class}"));
    let mut members = std::collections::BTreeSet::new();
    while !rest.is_empty() {
        let (first, after) = point(rest);
        let (last, after) = match after.strip_prefix('-') {
            Some(range) => point(range),
            None => (first, after),
        };
        members.extend((first..=last).map(|code| char::from_u32(code).expect("a scalar value")));
        rest = after;
    }
    members
}

/// Each published class is the set of characters the typed boundary refuses, read from the
/// schema's own text: the line terminators the scalar patterns refuse, the controls a ledger
/// name may not hold (a name may end in one CR LF, so CR and LF are left to the line-break
/// pattern), and the characters of a blank name.
#[test]
fn the_published_character_classes_are_the_sets_the_server_refuses() {
    use std::collections::BTreeSet;
    const LINE_BREAKS: [char; 4] = ['\n', '\r', '\u{2028}', '\u{2029}'];
    let schema = voucher_input_schema();
    let voucher = &schema["properties"]["vouchers"]["items"]["properties"];
    let entry = &voucher["entries"]["items"]["properties"];
    let pattern = |value: &Value| value.as_str().expect("a pattern").to_string();
    let every_character = || (0..=char::MAX as u32).filter_map(char::from_u32);

    let terminators: BTreeSet<char> = LINE_BREAKS.into_iter().collect();
    for published in [
        &entry["amount"]["not"]["anyOf"][1]["pattern"],
        &voucher["bridge_txn_id"]["not"]["pattern"],
        &voucher["date"]["not"]["pattern"],
    ] {
        assert_eq!(class_members(&pattern(published)), terminators);
    }
    // The typed boundary refuses each terminator after an otherwise valid value.
    let mut input = captured_catalogue_payload();
    for terminator in LINE_BREAKS {
        assert!(!valid_txn_id(&format!("txn{terminator}")));
        assert!(ImportAmount::parse(&format!("1.00{terminator}")).is_err());
        input.vouchers[0].date = format!("2026-09-01{terminator}");
        assert!(payload_verdict(&input).is_err(), "{terminator:?}");
    }

    let ledger_checks = entry["ledger"]["not"]["anyOf"].as_array().expect("anyOf");
    assert_eq!(ledger_checks.len(), 3);
    let mut ledger_controls: BTreeSet<char> =
        every_character().filter(|c| c.is_control()).collect();
    ledger_controls.remove(&'\n');
    ledger_controls.remove(&'\r');
    assert_eq!(
        class_members(&pattern(&ledger_checks[0]["pattern"])),
        ledger_controls
    );

    // A blank name: Unicode white space that is not a control, and LF and CR.
    let blank = pattern(&ledger_checks[2]["pattern"]);
    let (class, tail) = blank
        .strip_prefix('^')
        .and_then(|rest| rest.split_once(']'))
        .expect("a class between ^ and $");
    assert!(tail.ends_with('$'), "{blank}");
    let mut blank_characters: BTreeSet<char> = every_character()
        .filter(|c| c.is_whitespace() && !c.is_control())
        .collect();
    blank_characters.extend(['\n', '\r']);
    assert_eq!(class_members(&format!("{class}]")), blank_characters);
}

/// The ledger rule is the five alternatives of one pattern, and the server admits what they
/// leave: one CR LF at the end of a name, and no other CR or LF. Each name below is the one
/// shape that alternative alone is written for.
#[test]
fn a_ledger_name_may_end_in_one_crlf_and_hold_no_other_line_break() {
    let schema = voucher_input_schema();
    let ledger = &schema["properties"]["vouchers"]["items"]["properties"]["entries"]["items"]
        ["properties"]["ledger"];
    assert_eq!(
        ledger["not"]["anyOf"][1]["pattern"]
            .as_str()
            .expect("a pattern")
            .split('|')
            .collect::<Vec<_>>(),
        [
            r"\u000D[^\u000A]",
            r"\u000D$",
            r"[^\u000D]\u000A",
            r"^\u000A",
            r"\u000A[\s\S]",
        ]
    );
    let mut input = captured_catalogue_payload();
    for (name, admitted) in [
        ("A", true),
        ("A\r\n", true),
        ("A\rB", false),
        ("A\r", false),
        ("A\nB", false),
        ("A\n", false),
        ("\nA", false),
        ("A\r\nB", false),
        ("A\r\n\r\n", false),
        ("\r\n", false),
    ] {
        input.vouchers[0].entries[0].ledger = name.into();
        let verdict = payload_verdict(&input);
        assert_eq!(
            verdict,
            if admitted {
                Ok(())
            } else {
                Err("voucher_entry_invalid".into())
            },
            "{name:?}"
        );
    }
}

/// The scalar fields of an entry and a voucher, whole: each is compared as one JSON value, so
/// a changed pattern, limit or guard fails here, and the Rust rules they restate are applied to
/// the values at the edge of each limit.
#[test]
fn the_scalar_fields_publish_exactly_the_patterns_and_limits_the_server_applies() {
    let schema = voucher_input_schema();
    let voucher = &schema["properties"]["vouchers"]["items"]["properties"];
    let entry = &voucher["entries"]["items"]["properties"];
    let line_terminators = r"[\u000A\u000D\u2028\u2029]";
    assert_eq!(
        entry["amount"],
        json!({
            "type": "string", "pattern": r"^[0-9]+\.[0-9]{2}$",
            "maxLength": bridge_tally_core::MAX_EXACT_DECIMAL_BYTES,
            "not": {"anyOf": [{"pattern": r"^0+\.00$"}, {"pattern": line_terminators}]}
        })
    );
    assert_eq!(
        voucher["bridge_txn_id"],
        json!({
            "type": "string", "pattern": "^[A-Za-z0-9_-]{1,64}$",
            "not": {"pattern": line_terminators}
        })
    );
    assert_eq!(
        voucher["date"],
        json!({
            "type": "string", "pattern": r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$",
            "not": {"pattern": line_terminators}
        })
    );
    assert_eq!(entry["side"], json!({"enum": ["Dr", "Cr"]}));
    // The 64-character limit is the server's rule for a transaction label.
    assert!(valid_txn_id(&"t".repeat(64)));
    assert!(!valid_txn_id(&"t".repeat(65)));
    assert!(!valid_txn_id(""));
    // A ledger name is one to 1024 characters, and a voucher number one to 32.
    assert_eq!(entry["ledger"]["minLength"], json!(1));
    assert_eq!(entry["ledger"]["maxLength"], json!(1024));
    assert_eq!(voucher["voucher_number"]["minLength"], json!(1));
    let mut input = captured_catalogue_payload();
    for (name, admitted) in [
        ("", false),
        ("L", true),
        (&"L".repeat(1024), true),
        (&"L".repeat(1025), false),
    ] {
        input.vouchers[0].entries[0].ledger = name.into();
        assert_eq!(
            payload_verdict(&input).is_ok(),
            admitted,
            "{} characters",
            name.len()
        );
    }
    input = captured_catalogue_payload();
    input.vouchers[0].voucher_number = Some(String::new());
    assert_eq!(payload_verdict(&input), Err("voucher_text_invalid".into()));
}

/// What the schema requires, closes and counts is what the server's own parse and admission
/// do: a field is listed as required exactly when the parse fails without it, an unknown key
/// is refused where the schema closes the object, and the counts are the edges of admission.
#[test]
fn the_schema_requires_closes_and_counts_what_the_server_does() {
    let schema = voucher_input_schema();
    let top = &schema["properties"];
    let voucher = &top["vouchers"]["items"];
    let entry = &voucher["properties"]["entries"]["items"];
    for object in [
        &schema,
        voucher,
        entry,
        &top["on_account_approvals"]["items"],
    ] {
        assert_eq!(object["additionalProperties"], json!(false));
    }
    assert_eq!(schema["required"], json!(["company_guid"]));
    assert_eq!(
        voucher["required"],
        json!(["bridge_txn_id", "date", "voucher_type", "entries"])
    );
    assert_eq!(entry["required"], json!(["ledger", "amount", "side"]));
    assert_eq!(top["company_guid"]["minLength"], json!(1));

    let valid = serde_json::to_value(captured_catalogue_payload()).unwrap();
    assert!(parse_payload(&valid).is_ok());
    fn whole(input: &mut Value) -> &mut Value {
        input
    }
    fn first_voucher(input: &mut Value) -> &mut Value {
        &mut input["vouchers"][0]
    }
    fn first_entry(input: &mut Value) -> &mut Value {
        &mut input["vouchers"][0]["entries"][0]
    }
    type Locate = fn(&mut Value) -> &mut Value;
    let levels: [(&Value, Locate); 3] = [
        (&schema, whole),
        (voucher, first_voucher),
        (entry, first_entry),
    ];
    for (published, locate) in levels {
        for key in published["properties"]
            .as_object()
            .expect("properties")
            .keys()
        {
            // `vouchers` is optional beside `proposals_id`, which the payload does not carry.
            let mut input = valid.clone();
            if key == "vouchers"
                || locate(&mut input)
                    .as_object_mut()
                    .expect("an object")
                    .remove(key)
                    .is_none()
            {
                continue;
            }
            let required = published["required"]
                .as_array()
                .expect("required")
                .contains(&json!(key));
            assert_eq!(parse_payload(&input).is_err(), required, "{key} removed");
        }
        let mut input = valid.clone();
        locate(&mut input)["colour"] = json!("red");
        assert!(parse_payload(&input).is_err(), "an unknown key");
    }

    // The counts: one to a thousand vouchers, two or more entries, at most a hundred approvals.
    assert_eq!(top["vouchers"]["minItems"], json!(1));
    assert_eq!(top["vouchers"]["maxItems"], json!(1000));
    assert_eq!(voucher["properties"]["entries"]["minItems"], json!(2));
    assert_eq!(top["on_account_approvals"]["maxItems"], json!(100));
    let sample = captured_catalogue_payload().vouchers[0].clone();
    let batch_of = |count: usize| {
        let mut input = captured_catalogue_payload();
        input.vouchers = (0..count)
            .map(|number| {
                let mut voucher = sample.clone();
                voucher.bridge_txn_id = format!("txn-{number}");
                voucher
            })
            .collect();
        input
    };
    assert_eq!(
        payload_verdict(&batch_of(0)),
        Err("voucher_count_invalid".into())
    );
    assert_eq!(payload_verdict(&batch_of(1)), Ok(()));
    assert_eq!(payload_verdict(&batch_of(1000)), Ok(()));
    assert_eq!(
        payload_verdict(&batch_of(1001)),
        Err("voucher_count_invalid".into())
    );
    let mut input = batch_of(1);
    input.vouchers[0].entries.truncate(1);
    assert_eq!(
        payload_verdict(&input),
        Err("voucher_entries_too_few".into())
    );

    // The identifiers of a proposals file are fixed in width, and the skips are a closed list.
    assert_eq!(top["proposals_id"]["minLength"], json!(46));
    assert_eq!(top["proposals_id"]["maxLength"], json!(46));
    assert_eq!(top["proposals_sha256"]["minLength"], json!(64));
    assert_eq!(top["proposals_sha256"]["maxLength"], json!(64));
    let skips = &top[transfers::WITHDRAW_SKIPS_KEY];
    assert_eq!(skips["type"], json!("array"));
    assert_eq!(skips["uniqueItems"], json!(true));
    assert_eq!(skips["maxItems"], json!(1000));
    assert_eq!(
        skips["items"],
        json!({"type": "string", "pattern": crate::agent::catalog::STATEMENT_ROW_ID_PATTERN})
    );
}
