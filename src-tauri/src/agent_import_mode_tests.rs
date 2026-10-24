//! Captured-body replay; profile metadata mutations exercise admission only.
use super::*;

pub(super) fn licensed_import_probe() -> Vec<ScenarioPlan> {
    let bytes = include_bytes!("../crates/bridge-tally-protocol/tests/fixtures/agent/native-licensed-release-companies.utf16le.xml");
    let xml = String::from_utf16(
        &bytes
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .collect::<Vec<_>>(),
    )
    .unwrap();
    vec![
        ScenarioPlan::new(Fixture::ProductStatus(
            tally_protocol_simulator::ProductStatus::TallyPrime,
        ))
        .with_framing(ResponseFraming::ContentLength),
        ScenarioPlan::new(Fixture::SyntheticXml(xml))
            .with_encoding(WireEncoding::Utf16Le)
            .with_framing(ResponseFraming::ContentLength),
    ]
}

// Change only observed profile fields in memory. These replays prove the
// admission contract, not live compatibility for another release or tier.
pub(super) fn import_profile_probe(fault: &str) -> Vec<ScenarioPlan> {
    let mut plans = licensed_import_probe();
    let xml = plans[1].fixture.body().into_owned();
    let release = "<BRIDGERELEASE TYPE=\"String\">7.1</BRIDGERELEASE>";
    let damaged = match fault {
        "education" => xml.replace(
            "<EDUMODE TYPE=\"Logical\">No</EDUMODE>",
            "<EDUMODE TYPE=\"Logical\">Yes</EDUMODE>",
        ),
        "unknown" => xml.replace(
            "<SILVER TYPE=\"Logical\">Yes</SILVER>",
            "<SILVER TYPE=\"Logical\">No</SILVER>",
        ),
        "product" => xml.replace(
            "<PRODUCTNAME TYPE=\"String\">TallyPrime</PRODUCTNAME>",
            "<PRODUCTNAME TYPE=\"String\">Tally ERP 9</PRODUCTNAME>",
        ),
        "editlog" => xml.replace(
            "<PRODUCTNAME TYPE=\"String\">TallyPrime</PRODUCTNAME>",
            "<PRODUCTNAME TYPE=\"String\">TallyPrime Edit Log</PRODUCTNAME>",
        ),
        "unknown_product" => xml.replace(
            "<PRODUCTNAME TYPE=\"String\">TallyPrime</PRODUCTNAME>",
            "<PRODUCTNAME TYPE=\"String\">UnknownProduct</PRODUCTNAME>",
        ),
        "release_missing" => xml.replace(release, ""),
        "release_unknown" => xml.replace(
            release,
            "<BRIDGERELEASE TYPE=\"String\">unknown</BRIDGERELEASE>",
        ),
        "license_gold" => xml
            .replace(
                "<SILVER TYPE=\"Logical\">Yes</SILVER>",
                "<SILVER TYPE=\"Logical\">No</SILVER>",
            )
            .replace(
                "<GOLD TYPE=\"Logical\">No</GOLD>",
                "<GOLD TYPE=\"Logical\">Yes</GOLD>",
            ),
        "license_ambiguous" => xml.replace(
            "<GOLD TYPE=\"Logical\">No</GOLD>",
            "<GOLD TYPE=\"Logical\">Yes</GOLD>",
        ),
        "release_and_tier_unknown" => xml.replace(release, "").replace(
            "<GOLD TYPE=\"Logical\">No</GOLD>",
            "<GOLD TYPE=\"Logical\">Yes</GOLD>",
        ),
        "none" => xml.clone(),
        _ => panic!("unrecognized profile fault: {fault}"),
    };
    if fault != "none" {
        assert_ne!(damaged, xml, "{fault}");
    }
    plans[1].fixture = Fixture::SyntheticXml(damaged);
    plans
}

#[tokio::test]
async fn import_build_requires_an_observed_mode_bracket_and_retains_probe_evidence() {
    let accepted = [
        "none",
        "education",
        "release_missing",
        "release_unknown",
        "license_gold",
        "license_ambiguous",
    ];
    let rejected = ["unknown", "product", "editlog", "unknown_product"];
    for fault in accepted.into_iter().chain(rejected) {
        // Education is admitted by the build's own mode bracket and refused by
        // its numbering read, which reads the same mode first (bridge#724).
        let numbering_refused = fault == "education";
        let admitted = accepted.contains(&fault) && !numbering_refused;
        let plans = if admitted {
            [
                import_profile_probe(fault),
                import_family_cycle_plans()[..16].to_vec(),
                build_numbering_plans_probed(&[], &import_profile_probe(fault)),
                import_family_cycle_plans()[4..10].to_vec(),
                build_preflight_plans(),
                import_profile_probe(fault),
            ]
            .concat()
        } else if numbering_refused {
            [
                import_profile_probe(fault),
                import_family_cycle_plans()[..16].to_vec(),
                import_profile_probe(fault),
            ]
            .concat()
        } else {
            import_profile_probe(fault)
        };
        let responses = plans
            .iter()
            .map(|plan| tally_protocol_simulator::encode(&plan.fixture.body(), plan.encoding))
            .collect::<Vec<_>>();
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = Server::new(super::super::super::Settings {
            endpoint: TallyEndpointConfig {
                host: "127.0.0.1".into(),
                port: simulator.address().port(),
            },
            data_dir: directory.path().into(),
            max_rows: 10,
            max_bytes: 200_000,
            redaction: super::super::super::Redaction::None,
            import_enabled: true,
            posting: crate::agent::Posting::Off,
            master_writes_enabled: false,
            audit_enabled: false,
        });
        let result = server
            .build_import_xml(&serde_json::to_value(captured_catalogue_payload()).unwrap())
            .await;
        let evidence = if admitted {
            let result = result.unwrap();
            assert_eq!(
                result.payload["result"]["live_evidence"],
                json!([{"observation":"synthetic_lab_readback",
                    "report":"docs/agent/ASSESSMENT-2026-09-06.md",
                    "voucher_types":["Journal"]}])
            );
            assert_eq!(
                result.payload["result"]["verification_preflight"],
                json!({
                    "state":"current_window_readable", "from":"20260901", "to":"20260902",
                    "source_rows":0,
                    "paired_source_bytes":responses[25 + BUILD_NUMBERING_PLANS].len() * 2,
                    "response_sha256":sha256_hex(&responses[25 + BUILD_NUMBERING_PLANS])
                })
            );
            let batch = result.payload["result"]["batch_id"].as_str().unwrap();
            assert!(directory
                .path()
                .join("imports")
                .join(format!("{batch}.xml"))
                .is_file());
            assert_eq!(
                result.payload["result"]["observed_profile"]["license_tier"],
                match fault {
                    "license_gold" => json!("gold"),
                    "education" | "license_ambiguous" => Value::Null,
                    _ => json!("silver"),
                },
                "{fault}"
            );
            result.evidence
        } else {
            let error = result.err().unwrap();
            let code = if numbering_refused {
                "masters_education_unqualified"
            } else {
                "import_mode_unqualified"
            };
            assert_eq!(error.code, code, "{fault}");
            assert!(!directory.path().join("imports").exists());
            assert!(!directory.path().join("agent-import-ledger.jsonl").exists());
            *error.evidence.unwrap()
        };
        let observed = simulator.finish().unwrap();
        let join = |a: &str, b: &str| sha256_hex(format!("{a}:{b}").as_bytes());
        let request = |i: usize| observed[i].request_body_sha256.clone();
        let response = |i: usize| sha256_hex(&responses[i]);
        let mut request_hash = join(&request(0), &request(1));
        let mut response_hash = join(&response(0), &response(1));
        let mut bytes = responses[0].len() + responses[1].len();
        if admitted || numbering_refused {
            for i in [2, 7, 13] {
                request_hash = join(&request_hash, &request(i));
                response_hash = join(&response_hash, &response(i));
                bytes += 2 * responses[i].len();
            }
        }
        if admitted {
            assert_eq!(observed.len(), QUALIFIED_BUILD_REQUESTS);
            let (numbering_request, numbering_response, numbering_bytes) =
                numbering_evidence_link(&observed, &responses, 18);
            request_hash = join(&request_hash, &numbering_request);
            response_hash = join(&response_hash, &numbering_response);
            bytes += numbering_bytes;
            for i in [19 + BUILD_NUMBERING_PLANS, 25 + BUILD_NUMBERING_PLANS] {
                request_hash = join(&request_hash, &request(i));
                response_hash = join(&response_hash, &response(i));
                bytes += 2 * responses[i].len();
            }
            let closing = 30 + BUILD_NUMBERING_PLANS;
            request_hash = join(
                &request_hash,
                &join(&request(closing), &request(closing + 1)),
            );
            response_hash = join(
                &response_hash,
                &join(&response(closing), &response(closing + 1)),
            );
            bytes += responses[closing].len() + responses[closing + 1].len();
        } else if numbering_refused {
            // The numbering read's own mode probe, and nothing after it.
            assert_eq!(observed.len(), 20);
            request_hash = join(&request_hash, &join(&request(18), &request(19)));
            response_hash = join(&response_hash, &join(&response(18), &response(19)));
            bytes += responses[18].len() + responses[19].len();
        } else {
            assert_eq!(observed.len(), 2);
        }
        assert_eq!(evidence.request_sha256, request_hash, "{fault}");
        assert_eq!(evidence.response_sha256, response_hash, "{fault}");
        assert_eq!(evidence.bytes, bytes, "{fault}");
    }
}

#[tokio::test]
async fn education_build_refuses_an_unsupported_voucher_date_before_any_catalogue_read() {
    let plans = import_profile_probe("education");
    let responses = plans
        .iter()
        .map(|plan| tally_protocol_simulator::encode(&plan.fixture.body(), plan.encoding))
        .collect::<Vec<_>>();
    let simulator = SequenceSimulator::spawn(plans).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let server = Server::new(super::super::super::Settings {
        endpoint: TallyEndpointConfig {
            host: "127.0.0.1".into(),
            port: simulator.address().port(),
        },
        data_dir: directory.path().into(),
        max_rows: 10,
        max_bytes: 200_000,
        redaction: super::super::super::Redaction::None,
        import_enabled: true,
        posting: crate::agent::Posting::Off,
        master_writes_enabled: false,
        audit_enabled: false,
    });
    let mut payload = captured_catalogue_payload();
    payload.vouchers[0].date = "20260915".into();
    let error = match server
        .build_import_xml(&serde_json::to_value(payload).unwrap())
        .await
    {
        Ok(_) => panic!("unsupported Education voucher date was admitted"),
        Err(error) => error,
    };
    assert_eq!(error.code, "education_voucher_date_unsupported");
    let evidence = *error
        .evidence
        .expect("completed Education profile probe is retained on date refusal");
    assert!(!directory.path().join("imports").exists());
    assert!(!directory.path().join("agent-import-ledger.jsonl").exists());
    let observed = simulator.finish().unwrap();
    assert_eq!(observed.len(), 2);
    assert_eq!(
        evidence.request_sha256,
        sha256_hex(
            format!(
                "{}:{}",
                observed[0].request_body_sha256, observed[1].request_body_sha256
            )
            .as_bytes()
        )
    );
    assert_eq!(
        evidence.response_sha256,
        sha256_hex(
            format!(
                "{}:{}",
                sha256_hex(&responses[0]),
                sha256_hex(&responses[1])
            )
            .as_bytes()
        )
    );
    assert_eq!(evidence.bytes, responses[0].len() + responses[1].len());
}

#[tokio::test]
async fn import_build_rechecks_captured_catalogue_before_persistence() {
    for fault in ["none", "rename", "identity", "parent", "delete"] {
        let mut plans = qualified_import_cycle_plans()[..QUALIFIED_BUILD_REQUESTS].to_vec();
        if fault != "none" {
            // The catalogue repeat, after the mark and the numbering read.
            for index in [19 + BUILD_NUMBERING_PLANS, 21 + BUILD_NUMBERING_PLANS] {
                let xml = plans[index].fixture.body().into_owned();
                let changed = match fault {
                    "rename" => xml.replace("Bridge Nested Debtor WR4", "Renamed Debtor"),
                    "identity" => xml.replace(
                        "61c6de69-1748-461c-ad3f-162cb949df9f-000000d5",
                        "61c6de69-1748-461c-ad3f-162cb949df9f-000000ff",
                    ),
                    "parent" => xml.replace("Bridge Nested Debtors WR4", "Changed Parent"),
                    "delete" => {
                        let start = xml
                            .find("<LEDGER NAME=\"Bridge Nested Debtor WR4\"")
                            .unwrap();
                        let end =
                            start + xml[start..].find("</LEDGER>").unwrap() + "</LEDGER>".len();
                        format!("{}{}", &xml[..start], &xml[end..])
                    }
                    _ => unreachable!(),
                };
                assert_ne!(changed, xml);
                plans[index].fixture = Fixture::SyntheticXml(changed);
            }
            plans.truncate(24 + BUILD_NUMBERING_PLANS);
        }
        let responses = plans
            .iter()
            .map(|plan| tally_protocol_simulator::encode(&plan.fixture.body(), plan.encoding))
            .collect::<Vec<_>>();
        let simulator = SequenceSimulator::spawn(plans).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let server = Server::new(crate::agent::Settings {
            endpoint: TallyEndpointConfig {
                host: "127.0.0.1".into(),
                port: simulator.address().port(),
            },
            data_dir: directory.path().into(),
            max_rows: 10,
            max_bytes: 200_000,
            redaction: crate::agent::Redaction::None,
            import_enabled: true,
            posting: crate::agent::Posting::Off,
            master_writes_enabled: false,
            audit_enabled: false,
        });
        let result = server
            .build_import_xml(&serde_json::to_value(captured_catalogue_payload()).unwrap())
            .await;
        let evidence = if fault == "none" {
            result.unwrap().evidence
        } else {
            let error = result.err().unwrap();
            assert_eq!(error.code, "import_catalogue_changed", "{fault}");
            assert!(!directory.path().join("imports").exists());
            assert!(!directory.path().join("agent-import-ledger.jsonl").exists());
            *error.evidence.unwrap()
        };
        let observed = simulator.finish().unwrap();
        assert_eq!(
            observed.len(),
            BUILD_NUMBERING_PLANS + if fault == "none" { 32 } else { 24 }
        );
        let join = |a: &str, b: &str| sha256_hex(format!("{a}:{b}").as_bytes());
        let mut request = join(
            &observed[0].request_body_sha256,
            &observed[1].request_body_sha256,
        );
        let mut response = join(&sha256_hex(&responses[0]), &sha256_hex(&responses[1]));
        let mut bytes = responses[0].len() + responses[1].len();
        for i in [2, 7, 13] {
            request = join(&request, &observed[i].request_body_sha256);
            response = join(&response, &sha256_hex(&responses[i]));
            bytes += 2 * responses[i].len();
        }
        let (numbering_request, numbering_response, numbering_bytes) =
            numbering_evidence_link(&observed, &responses, 18);
        request = join(&request, &numbering_request);
        response = join(&response, &numbering_response);
        bytes += numbering_bytes;
        let repeat = 19 + BUILD_NUMBERING_PLANS;
        request = join(&request, &observed[repeat].request_body_sha256);
        response = join(&response, &sha256_hex(&responses[repeat]));
        bytes += 2 * responses[repeat].len();
        assert_eq!(
            observed[7].request_body_sha256,
            observed[repeat].request_body_sha256
        );
        if fault == "none" {
            let preflight = 25 + BUILD_NUMBERING_PLANS;
            let closing = 30 + BUILD_NUMBERING_PLANS;
            request = join(&request, &observed[preflight].request_body_sha256);
            response = join(&response, &sha256_hex(&responses[preflight]));
            bytes += responses[preflight].len() * 2;
            request = join(
                &request,
                &join(
                    &observed[closing].request_body_sha256,
                    &observed[closing + 1].request_body_sha256,
                ),
            );
            response = join(
                &response,
                &join(
                    &sha256_hex(&responses[closing]),
                    &sha256_hex(&responses[closing + 1]),
                ),
            );
            bytes += responses[closing].len() + responses[closing + 1].len();
        }
        assert_eq!(evidence.request_sha256, request);
        assert_eq!(evidence.response_sha256, response);
        assert_eq!(evidence.bytes, bytes);
    }
}
