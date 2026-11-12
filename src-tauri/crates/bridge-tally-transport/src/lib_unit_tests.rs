use super::*;

#[test]
fn endpoint_identity_normalizes_names_without_collapsing_ip_families() {
    for host in ["localhost", "LOCALHOST", "127.0.0.1"] {
        assert_eq!(
            canonical_loopback_origin(&TallyEndpointConfig {
                host: host.to_owned(),
                port: 9000,
            })
            .expect("valid loopback endpoint"),
            "http://127.0.0.1:9000"
        );
    }
    assert_eq!(
        canonical_loopback_origin(&TallyEndpointConfig {
            host: "::1".to_owned(),
            port: 9000,
        })
        .expect("valid IPv6 loopback endpoint"),
        "http://[::1]:9000"
    );
}

#[test]
fn remote_and_url_shaped_hosts_are_rejected() {
    for host in [
        "",
        "http://localhost",
        "https://localhost",
        "https://127.0.0.1:9000",
        "localhost/path",
        "user@localhost",
        "192.168.1.10",
        "tally.internal",
    ] {
        let error = canonical_loopback_origin(&TallyEndpointConfig {
            host: host.to_owned(),
            port: 9000,
        })
        .expect_err("endpoint must fail closed");
        assert_eq!(error.safe_code(), "endpoint_invalid");
    }
}

#[test]
fn policy_cannot_expand_production_caps_or_deadline() {
    assert_eq!(
        TransportPolicy::default().request_timeout,
        Duration::from_secs(20)
    );
    assert_eq!(
        TransportPolicy::default().xml_response_max_bytes,
        32 * 1024 * 1024
    );
    let expanded = TransportPolicy {
        request_timeout: MAX_REQUEST_TIMEOUT + Duration::from_millis(1),
        ..TransportPolicy::default()
    };
    assert!(matches!(
        expanded.validate(),
        Err(TallyTransportError::PolicyInvalid { .. })
    ));
    let expanded = TransportPolicy {
        xml_response_max_bytes: XML_RESPONSE_MAX_BYTES + 1,
        ..TransportPolicy::default()
    };
    assert!(matches!(
        expanded.validate(),
        Err(TallyTransportError::PolicyInvalid { .. })
    ));
}

#[test]
fn oversized_xml_source_is_rejected_before_wire_encoding() {
    let error = match prepare_tally_xml_request_with_encoder(&"X".repeat(65), 64, |source| {
        panic!(
            "wire encoder must not run for an oversized XML source of {} bytes",
            source.len()
        )
    }) {
        Err(error) => error,
        Ok(prepared) => panic!(
            "oversized XML source unexpectedly produced {} wire bytes",
            prepared.body.len()
        ),
    };
    assert_eq!(error, TallyTransportError::RequestTooLarge { limit: 64 });
}

#[test]
fn xml_request_cap_is_inclusive_for_utf8_source_bytes() {
    let source = "éé";
    assert_eq!(source.len(), 4);
    let prepared = prepare_tally_xml_request(source, source.len())
        .expect("exact source-byte boundary is accepted");
    assert_eq!(prepared.body, encode_tally_xml_request_utf16le(source));
    assert_eq!(prepared.body.len(), 6);
    assert_eq!(
        prepared.body_sha256,
        hex_digest(Sha256::digest(&prepared.body))
    );
}

/// #2142: which ended posts owe a drain, every error named once: those that
/// were sent, or perhaps sent, and not answered owe one; those Tally answered
/// with a whole head, and those that never reached Tally, do not.
#[test]
fn every_transport_error_is_classed_for_the_drain() {
    use TallyTransportError as E;
    for (error, owes) in [
        (E::RequestTimedOut, true),
        (E::RequestFailed, true),
        (
            E::ResponseTooLarge {
                limit: 1,
                declared_by_peer: false,
            },
            true,
        ),
        (E::ResponseTruncated, true),
        (E::ResponseReadFailed, true),
        (E::HttpStatus { status: 500 }, true),
        (
            E::ResponseTooLarge {
                limit: 1,
                declared_by_peer: true,
            },
            true,
        ),
        (E::UnsupportedContentEncoding, true),
        (E::InvalidEncoding { code: "x" }, true),
        (E::EmptyAnswer, true),
        (E::EndpointInvalid { code: "x" }, false),
        (E::PolicyInvalid { code: "x" }, false),
        (E::ClientInitializationFailed, false),
        (E::RequestTooLarge { limit: 1 }, false),
        (E::ConnectionFailed, false),
        (
            E::WireRefused {
                refusal: WireRefusal::Busy,
            },
            false,
        ),
        (E::SendWithdrawn, false),
    ] {
        assert_eq!(drain::owes_drain(&error), owes, "{error:?}");
    }
}

/// #2377: the empty answer's head is exactly the one the lab captured (protocol reference
/// §12a.27), and each of its conditions, missed alone, leaves the head refused as before.
#[test]
fn only_the_captured_empty_head_can_be_the_empty_answer() {
    use reqwest::header::{HeaderMap, HeaderValue, CONNECTION};
    use reqwest::StatusCode;
    fn head(lengths: &[&'static str], content_type: Option<&'static str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONNECTION, HeaderValue::from_static("KEEP-ALIVE"));
        for length in lengths {
            headers.append(CONTENT_LENGTH, HeaderValue::from_static(length));
        }
        if let Some(content_type) = content_type {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static(content_type));
        }
        headers
    }
    assert!(is_empty_answer_head(StatusCode::OK, &head(&["0"], None)));

    // Status: exactly 200.
    for status in [StatusCode::CREATED, StatusCode::NO_CONTENT] {
        assert!(
            !is_empty_answer_head(status, &head(&["0"], None)),
            "{status}"
        );
    }
    // A Content-Type, even one Bridge decodes.
    assert!(!is_empty_answer_head(
        StatusCode::OK,
        &head(&["0"], Some("text/xml; charset=utf-16"))
    ));
    assert!(!is_empty_answer_head(
        StatusCode::OK,
        &head(&["0"], Some(""))
    ));
    // A Content-Type that is sent twice, or that is not text: present, so never the empty answer.
    let mut twice = head(&["0"], Some("text/xml; charset=utf-16"));
    twice.append(
        CONTENT_TYPE,
        HeaderValue::from_static("text/xml; charset=utf-16"),
    );
    assert!(!is_empty_answer_head(StatusCode::OK, &twice));
    let mut not_text = head(&["0"], None);
    not_text.insert(CONTENT_TYPE, HeaderValue::from_bytes(b"text/\xff").unwrap());
    assert!(!is_empty_answer_head(StatusCode::OK, &not_text));
    // Exactly one Content-Length, whose value is the byte 0.
    for lengths in [
        &[][..],
        &["0", "0"],
        &["1"],
        &["00"],
        &[" 0"],
        &["0 "],
        &["+0"],
    ] {
        assert!(
            !is_empty_answer_head(StatusCode::OK, &head(lengths, None)),
            "{lengths:?}"
        );
    }
}

/// #2377: the empty answer is told only after its body was read to the end with no byte. A body
/// that held a byte, or that could not be read, is the refusal a missing type always had.
#[test]
fn the_empty_answer_needs_its_body_read_whole_and_empty() {
    assert_eq!(
        empty_answer_after_body(Some(0)),
        TallyTransportError::EmptyAnswer
    );
    for read in [Some(1), Some(2), None] {
        assert_eq!(
            empty_answer_after_body(read),
            TallyTransportError::InvalidEncoding {
                code: "response_content_type_unsupported"
            },
            "{read:?}"
        );
    }
    // Callers that do not ask for it by name see what they saw before it existed.
    assert_eq!(
        TallyTransportError::EmptyAnswer.to_string(),
        CONTENT_TYPE_MISSING.to_string()
    );
    assert_eq!(
        TallyTransportError::EmptyAnswer.safe_code(),
        CONTENT_TYPE_MISSING.safe_code()
    );
}
