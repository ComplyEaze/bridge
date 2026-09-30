use super::*;
use bridge_tally_compatibility::SurfacePin;
use tally_protocol_simulator::{Fixture, ScenarioPlan, Simulator, WireEncoding};

const SHA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn config(port: u16) -> LiveRunConfig {
    LiveRunConfig {
        schema_version: CONFIG_SCHEMA_VERSION,
        repository_root: PathBuf::from("."),
        fixture_manifest: PathBuf::from("fixture.json"),
        endpoint_family: LoopbackFamily::Ipv4,
        port,
        product: ProductFamily::TallyPrime,
        release: "7.1".to_string(),
        mode: TallyMode::Education,
        odbc_state: OdbcState::Disabled,
        locale: LocaleProfile::EnglishIndia,
        no_customer_data_attested: true,
    }
}

fn fixture() -> SyntheticFixtureManifest {
    SyntheticFixtureManifest {
        schema_version: FIXTURE_SCHEMA_VERSION,
        fixture_id: "education-small-v1".to_string(),
        dataset_tier: DatasetTier::SyntheticSmall,
        company_marker: "BRIDGE-PR14-SYNTHETIC-019f605f-e6cf-77b2-ac95-31722887a911".to_string(),
        ledger_sentinel: "BRIDGE-LEDGER-019f605f-e6cf-77b2-ac95-31722887a911".to_string(),
        voucher_number_sentinel: "BRIDGE-VOUCHER-019f605f-e6cf-77b2-ac95-31722887a911".to_string(),
        empty_voucher_range: DateWindow {
            from_yyyymmdd: "20260403".to_string(),
            to_yyyymmdd: "20260403".to_string(),
        },
        populated_voucher_range: DateWindow {
            from_yyyymmdd: "20260401".to_string(),
            to_yyyymmdd: "20260402".to_string(),
        },
        minimum_ledger_count: 1,
        maximum_ledger_count: 100,
        minimum_populated_voucher_count: 1,
        maximum_populated_voucher_count: 20,
    }
}

fn metadata() -> RunMetadata {
    RunMetadata {
        observed_at_unix_ms: 1_800_000_000_000,
        bridge_commit_sha: "b".repeat(40),
        working_tree_dirty: true,
        compatibility_surface_sha256: SHA.to_string(),
        executable_sha256: SHA.to_string(),
        cargo_lock_sha256: SHA.to_string(),
        fixture_manifest_sha256: SHA.to_string(),
    }
}

fn receipt() -> LiveCompatibilityReceipt {
    let company = ValidatedCompanyName::new(fixture().company_marker).unwrap();
    let range = ValidatedDateRange::new("20260403", "20260403").unwrap();
    let profile = ReadOnlyProfile::CompanyListV1;
    let mut operations = vec![not_attempted(
        ReadProfileId::XmlCompanyEnumerationV1,
        profile.template_sha256(),
        "endpoint_not_queried",
    )];
    operations.push(not_attempted(
        ReadProfileId::XmlSyntheticFixtureMarkerV1,
        profile.template_sha256(),
        "endpoint_not_queried",
    ));
    append_company_reads_not_attempted(&mut operations, &company, &range);
    seal_receipt(
        &config(9001),
        &fixture(),
        &metadata(),
        operations,
        false,
        false,
        CountBucket::Unknown,
    )
    .unwrap()
}

fn inputs(port: u16, binding: &str, expires_at_unix_ms: i64) -> LiveRunInputs {
    LiveRunInputs {
        config: config(port),
        fixture: fixture(),
        metadata: metadata(),
        repository_root: PathBuf::from("."),
        challenge_phrase: "QUALIFY education-small-v1 test".to_string(),
        consent_binding: binding.to_string(),
        consent_expires_at_unix_ms: expires_at_unix_ms,
    }
}

#[test]
fn fixture_requires_reviewed_sentinels_bounded_counts_and_disjoint_ranges() {
    assert!(validate_fixture(&fixture()).is_ok());
    let mut invalid = fixture();
    invalid.company_marker = "synthetic".to_string();
    assert_eq!(validate_fixture(&invalid), Err(error("fixture_invalid")));
    let mut overlap = fixture();
    overlap.empty_voucher_range = overlap.populated_voucher_range.clone();
    assert_eq!(
        validate_fixture(&overlap),
        Err(error("fixture_ranges_overlap"))
    );
    let mut invalid_bounds = fixture();
    invalid_bounds.maximum_ledger_count = 0;
    assert_eq!(
        validate_fixture(&invalid_bounds),
        Err(error("fixture_invalid"))
    );
}

#[test]
fn dataset_tier_comes_from_the_reviewed_fixture_not_the_local_profile() {
    let mut fixture = fixture();
    fixture.dataset_tier = DatasetTier::SyntheticLarge;
    assert!(validate_fixture(&fixture).is_ok());
    assert_eq!(fixture.dataset_tier, DatasetTier::SyntheticLarge);
}

#[test]
fn unknown_profile_fields_are_rejected_before_network_and_cannot_be_laundered() {
    let mut config = config(9001);
    config.product = ProductFamily::Unknown;
    config.release = "unknown".to_string();
    config.mode = TallyMode::Unknown;
    config.odbc_state = OdbcState::Unknown;
    config.locale = LocaleProfile::Unknown;
    assert_eq!(validate_config(&config), Err(error("config_invalid")));
    let company = ValidatedCompanyName::new(fixture().company_marker).unwrap();
    let range = ValidatedDateRange::new("20260403", "20260403").unwrap();
    let profile = ReadOnlyProfile::CompanyListV1;
    let mut operations = vec![not_attempted(
        ReadProfileId::XmlCompanyEnumerationV1,
        profile.template_sha256(),
        "endpoint_not_queried",
    )];
    operations.push(not_attempted(
        ReadProfileId::XmlSyntheticFixtureMarkerV1,
        profile.template_sha256(),
        "endpoint_not_queried",
    ));
    append_company_reads_not_attempted(&mut operations, &company, &range);
    let receipt = seal_receipt(
        &config,
        &fixture(),
        &metadata(),
        operations,
        false,
        false,
        CountBucket::Unknown,
    )
    .unwrap();
    assert_eq!(receipt.product.authority, EvidenceAuthority::Unknown);
    assert_eq!(receipt.release.confidence, EvidenceConfidence::Unknown);
    assert_eq!(receipt.mode.authority, EvidenceAuthority::Unknown);
}

#[tokio::test]
async fn empty_company_response_stops_after_one_request_and_retains_no_marker() {
    let mut last_failure = "simulator_not_started";
    for _ in 0..5 {
        let simulator = Simulator::spawn(
            ScenarioPlan::new(Fixture::EmptyExport).with_encoding(WireEncoding::Utf16Le),
        )
        .unwrap();
        let config = config(simulator.address().port());
        let transport =
            ReadOnlyTransport::new(ReadLoopback::Ipv4, simulator.address().port()).unwrap();
        let receipt = execute_with_transport(&config, &fixture(), &metadata(), &transport)
            .await
            .unwrap();
        let observed = match simulator.finish() {
            Ok(observed) if observed.method == "POST" && observed.request_processed => observed,
            Ok(_) => {
                last_failure = "simulator_request_not_processed";
                continue;
            }
            Err(_) => {
                last_failure = "simulator_request_unavailable";
                continue;
            }
        };
        assert_eq!(receipt.operations.len(), 5);
        assert_eq!(receipt.operations[0].safe_reason_code.as_deref(), None);
        assert_eq!(receipt.operations[0].outcome, OperationOutcome::Passed);
        assert_eq!(receipt.operations[1].outcome, OperationOutcome::Failed);
        assert!(receipt.operations[2..]
            .iter()
            .all(|operation| operation.outcome == OperationOutcome::NotAttempted));
        assert!(!receipt.fixture_marker_verified);
        assert_eq!(observed.path, "/");
        let text = String::from_utf8(receipt.to_pretty_json().unwrap()).unwrap();
        assert!(!text.contains(&fixture().company_marker));
        return;
    }
    panic!("loopback simulator remained unstable: {last_failure}");
}

#[test]
fn false_customer_attestation_rejects_before_any_post() {
    let simulator = Simulator::spawn(ScenarioPlan::new(Fixture::EmptyExport)).unwrap();
    let mut value = config(simulator.address().port());
    value.no_customer_data_attested = false;
    assert_eq!(validate_config(&value), Err(error("config_invalid")));
    simulator.cancel();
    let observed = simulator.finish().unwrap();
    assert!(!observed.request_processed);
    assert!(observed.cancelled);
}

#[test]
fn consent_is_expiring_and_bound_to_one_loaded_run() {
    let future = now_unix_ms().unwrap() + NETWORK_CONSENT_TTL_MS;
    let run_a = inputs(9000, "a", future);
    let run_b = inputs(9001, "b", future);
    let token_a = confirm_network_challenge(&run_a, "QUALIFY education-small-v1 test").unwrap();
    assert_eq!(
        verify_network_consent(&run_b, &token_a),
        Err(error("network_consent_binding_mismatch"))
    );
    assert_eq!(
        confirm_network_challenge(&run_a, "wrong").err().unwrap(),
        error("network_consent_mismatch")
    );

    let expired = inputs(9000, "expired", 1);
    assert_eq!(
        confirm_network_challenge(&expired, expired.challenge_phrase())
            .err()
            .unwrap(),
        error("network_consent_expired")
    );
}

#[test]
fn live_receipt_paths_are_local_json_and_path_bound() {
    let directory = std::env::temp_dir().join(format!(
        "bridge-live-read-path-test-{}-{}",
        std::process::id(),
        now_unix_ms().unwrap()
    ));
    let local = directory.join(".bridge-live");
    fs::create_dir_all(&local).unwrap();
    let first = local.join("first.json");
    let second = local.join("second.json");
    assert_ne!(
        live_receipt_output_binding(&first).unwrap(),
        live_receipt_output_binding(&second).unwrap()
    );
    assert_eq!(
        live_receipt_output_binding(&directory.join("outside.json")),
        Err(error("receipt_output_outside_local_evidence_root"))
    );
    assert_eq!(
        live_receipt_output_binding(&local.join("receipt.txt")),
        Err(error("receipt_output_invalid"))
    );
    fs::remove_dir(local).unwrap();
    fs::remove_dir(directory).unwrap();
}

#[test]
fn save_requires_exact_receipt_bound_confirmation_and_never_overwrites() {
    let directory = std::env::temp_dir().join(format!(
        "bridge-live-read-save-test-{}-{}",
        std::process::id(),
        now_unix_ms().unwrap()
    ));
    fs::create_dir(&directory).unwrap();
    let output = directory.join("receipt.json");
    assert_eq!(
        save_receipt_no_replace(&output, b"{}", "wrong", "SAVE abc"),
        Err(error("save_consent_mismatch"))
    );
    save_receipt_no_replace(&output, b"{}", "SAVE abc", "SAVE abc").unwrap();
    assert_eq!(fs::read(&output).unwrap(), b"{}");
    assert_eq!(
        save_receipt_no_replace(&output, b"new", "SAVE abc", "SAVE abc"),
        Err(error("receipt_output_exists"))
    );
    fs::remove_file(output).unwrap();
    fs::remove_dir(directory).unwrap();
}

#[test]
fn public_save_consumes_a_repository_bound_target_and_rechecks_no_overwrite() {
    let directory = std::env::temp_dir().join(format!(
        "bridge-live-read-public-save-test-{}-{}",
        std::process::id(),
        now_unix_ms().unwrap()
    ));
    let local = directory.join(".bridge-live");
    fs::create_dir_all(&local).unwrap();
    let output = local.join("receipt.json");
    let mut run = inputs(
        9001,
        "binding",
        now_unix_ms().unwrap() + NETWORK_CONSENT_TTL_MS,
    );
    run.repository_root = directory.clone();
    let target = run.validate_receipt_output(&output).unwrap();
    let overwrite_attempt = run.validate_receipt_output(&output).unwrap();
    let receipt = receipt();
    let bytes = receipt.to_pretty_json().unwrap();
    let phrase = receipt_save_phrase(&receipt, &target).unwrap();
    save_live_receipt_no_replace(target, &bytes, &phrase).unwrap();
    assert_eq!(
        save_live_receipt_no_replace(overwrite_attempt, &bytes, &phrase),
        Err(error("receipt_output_exists"))
    );
    fs::remove_file(output).unwrap();
    fs::remove_dir(local).unwrap();
    fs::remove_dir(directory).unwrap();
}

#[tokio::test]
async fn an_education_run_refuses_the_ledger_and_voucher_reads_before_sending_them() {
    let company = ValidatedCompanyName::new("Synthetic Co").unwrap();
    let range = ValidatedDateRange::new("20260401", "20260430").unwrap();
    let simulator = tally_protocol_simulator::SequenceSimulator::spawn(vec![ScenarioPlan::new(
        Fixture::EmptyExport,
    )
    .with_encoding(WireEncoding::Utf16Le)])
    .unwrap();
    let port = simulator.address().port();
    let education = read_transport(&config(port)).unwrap();
    for profile in [
        ReadOnlyProfile::LedgersV1 { company: &company },
        ReadOnlyProfile::VouchersV2 {
            company: &company,
            range: &range,
        },
    ] {
        let refused = education.send(profile).await.unwrap_err();
        assert_eq!(refused.safe_code(), "education_report_family_unsupported");
    }
    assert_eq!(simulator.received(), 0);
    let mut licensed = config(port);
    licensed.mode = TallyMode::Licensed;
    let sent = read_transport(&licensed)
        .unwrap()
        .send(ReadOnlyProfile::LedgersV1 { company: &company })
        .await;
    assert!(sent.is_ok(), "{sent:?}");
    assert_eq!(simulator.finish().unwrap().len(), 1);
}

fn git_in(directory: &Path, arguments: &[&str]) {
    let status = git_command(directory)
        .args(["-c", "user.name=t", "-c", "user.email=t@example.invalid"])
        .args(arguments)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .unwrap();
    assert!(status.success(), "git {arguments:?} failed");
}

fn status_is_clean(directory: &Path) -> bool {
    git_output(directory, &["status", "--porcelain"])
        .unwrap()
        .is_empty()
}

/// A throwaway repository with `a.rs` ("one\n") and `[b].rs` committed, removed on drop.
struct Repo(PathBuf);

impl Repo {
    fn new(label: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "bridge-live-read-drift-{label}-{}-{}",
            std::process::id(),
            now_unix_ms().unwrap()
        ));
        fs::create_dir(&directory).unwrap();
        git_in(&directory, &["init", "-q"]);
        fs::write(directory.join("a.rs"), b"one\n").unwrap();
        fs::write(directory.join("[b].rs"), b"two\n").unwrap();
        fs::write(directory.join("b.rs"), b"three\n").unwrap();
        git_in(&directory, &["add", "."]);
        git_in(&directory, &["commit", "-q", "-m", "base"]);
        Self(directory)
    }

    fn check(&self, paths: &[&str]) -> Result<(), LiveReadError> {
        refuse_drifted_paths(&self.0, paths, "surface_changed")
    }
}

impl Drop for Repo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

const DRIFT: Result<(), LiveReadError> = Err(LiveReadError {
    code: "surface_changed",
});

#[test]
fn clean_committed_pins_pass_and_an_edit_of_any_kind_is_refused() {
    let repo = Repo::new("edit");
    assert_eq!(repo.check(&["a.rs", "[b].rs"]), Ok(()));
    // Pathspecs are literal: "[b].rs" must not stand for "b.rs".
    fs::write(repo.0.join("b.rs"), b"changed\n").unwrap();
    assert_eq!(repo.check(&["[b].rs"]), Ok(()));
    fs::write(repo.0.join("a.rs"), b"edited\n").unwrap();
    assert_eq!(repo.check(&["a.rs", "[b].rs"]), DRIFT);
    // A staged edit counts too.
    git_in(&repo.0, &["add", "a.rs"]);
    assert_eq!(repo.check(&["a.rs"]), DRIFT);
}

#[test]
fn an_index_flag_cannot_hide_an_edit() {
    for flag in ["--assume-unchanged", "--skip-worktree"] {
        let repo = Repo::new("flag");
        git_in(&repo.0, &["update-index", flag, "a.rs"]);
        fs::write(repo.0.join("a.rs"), b"edited\n").unwrap();
        assert!(
            status_is_clean(&repo.0),
            "premise: git status is fooled by {flag}"
        );
        assert_eq!(repo.check(&["a.rs"]), DRIFT, "{flag}");
    }
}

#[test]
fn a_line_ending_rule_after_git_add_cannot_hide_an_edit() {
    let repo = Repo::new("eol");
    fs::write(repo.0.join(".git/info/attributes"), b"a.rs text eol=lf\n").unwrap();
    fs::write(repo.0.join("a.rs"), b"one\r\n").unwrap();
    git_in(&repo.0, &["add", "a.rs"]);
    assert!(
        status_is_clean(&repo.0),
        "premise: the CRLF edit normalises to the committed blob"
    );
    assert_eq!(repo.check(&["a.rs"]), DRIFT);
}

#[cfg(unix)]
#[test]
fn a_clean_filter_after_git_add_cannot_hide_an_edit() {
    let repo = Repo::new("filter");
    fs::write(
        repo.0.join(".git/info/attributes"),
        b"a.rs filter=restore\n",
    )
    .unwrap();
    git_in(
        &repo.0,
        &["config", "filter.restore.clean", "sed s/edited/one/"],
    );
    fs::write(repo.0.join("a.rs"), b"edited\n").unwrap();
    git_in(&repo.0, &["add", "a.rs"]);
    assert!(
        status_is_clean(&repo.0),
        "premise: the filter maps the edit back to the committed blob"
    );
    assert_eq!(repo.check(&["a.rs"]), DRIFT);
}

#[test]
fn a_stale_stat_cache_cannot_hide_a_same_size_edit() {
    let repo = Repo::new("stat");
    git_in(&repo.0, &["config", "core.checkStat", "minimal"]);
    // An old mtime, so the index entry is not "racily clean" (git re-reads those regardless).
    let modified = std::time::SystemTime::now() - std::time::Duration::from_secs(3600);
    let set_modified = |when| {
        fs::OpenOptions::new()
            .write(true)
            .open(repo.0.join("a.rs"))
            .unwrap()
            .set_modified(when)
            .unwrap();
    };
    set_modified(modified);
    git_in(&repo.0, &["add", "a.rs"]);
    fs::write(repo.0.join("a.rs"), b"two\n").unwrap();
    set_modified(modified);
    assert!(
        status_is_clean(&repo.0),
        "premise: same size and mtime, so git status is fooled"
    );
    assert_eq!(repo.check(&["a.rs"]), DRIFT);
}

#[test]
fn an_untracked_ignored_or_uncommitted_pin_is_refused() {
    let repo = Repo::new("untracked");
    fs::write(repo.0.join("untracked.rs"), b"new\n").unwrap();
    assert_eq!(repo.check(&["untracked.rs"]), DRIFT);
    fs::write(repo.0.join(".gitignore"), b"ignored.rs\n").unwrap();
    fs::write(repo.0.join("ignored.rs"), b"new\n").unwrap();
    assert_eq!(repo.check(&["ignored.rs"]), DRIFT);
    // Added to the index but not committed: not at HEAD.
    git_in(&repo.0, &["add", "untracked.rs"]);
    assert_eq!(repo.check(&["untracked.rs"]), DRIFT);
    // A committed pin that is gone from disk is an error, never a clean answer.
    fs::remove_file(repo.0.join("a.rs")).unwrap();
    assert!(repo.check(&["a.rs"]).is_err());
}

#[test]
fn a_pin_spelled_in_another_case_is_refused() {
    // On a case-insensitive filesystem the file would still be read; git does not know the name.
    let repo = Repo::new("case");
    assert_eq!(repo.check(&["A.rs"]), DRIFT);
}

#[cfg(unix)]
#[test]
fn a_committed_symlink_is_refused() {
    let repo = Repo::new("symlink");
    std::os::unix::fs::symlink("a.rs", repo.0.join("link.rs")).unwrap();
    git_in(&repo.0, &["add", "link.rs"]);
    git_in(&repo.0, &["commit", "-q", "-m", "link"]);
    assert_eq!(repo.check(&["link.rs"]), DRIFT);
}

#[test]
fn a_tree_that_is_not_the_repository_git_answers_for_is_refused() {
    let repo = Repo::new("nested");
    // An archive extract: same file names, no .git of its own, inside another repository.
    let nested = repo.0.join("extract");
    fs::create_dir(&nested).unwrap();
    fs::write(nested.join("a.rs"), b"one\n").unwrap();
    assert_eq!(
        refuse_drifted_paths(&nested, &["a.rs"], "surface_changed"),
        DRIFT
    );
    // A directory that is no repository at all is a failed query.
    let outside = std::env::temp_dir().join(format!(
        "bridge-live-read-drift-none-{}-{}",
        std::process::id(),
        now_unix_ms().unwrap()
    ));
    fs::create_dir(&outside).unwrap();
    let failed = refuse_drifted_paths(&outside, &["a.rs"], "surface_changed");
    fs::remove_dir_all(&outside).unwrap();
    assert!(failed.is_err());
}

#[test]
fn an_inherited_git_environment_cannot_redirect_the_check() {
    use std::sync::Mutex;
    static ENVIRONMENT: Mutex<()> = Mutex::new(());
    let _guard = ENVIRONMENT
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let repo = Repo::new("env-checkout");
    let elsewhere = Repo::new("env-elsewhere");
    fs::write(elsewhere.0.join("a.rs"), b"edited\n").unwrap();
    // The variables point at a checkout whose pinned file differs; the check must ignore them.
    std::env::set_var("GIT_DIR", elsewhere.0.join(".git"));
    std::env::set_var("GIT_WORK_TREE", &elsewhere.0);
    std::env::set_var("GIT_INDEX_FILE", elsewhere.0.join(".git/index"));
    let clean_checkout = repo.check(&["a.rs"]);
    fs::write(repo.0.join("a.rs"), b"edited\n").unwrap();
    let edited_checkout = repo.check(&["a.rs"]);
    for name in ["GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE"] {
        std::env::remove_var(name);
    }
    assert_eq!(clean_checkout, Ok(()));
    assert_eq!(edited_checkout, DRIFT);
}

#[test]
fn every_git_call_runs_without_the_redirecting_environment() {
    let command = git_command(Path::new("."));
    let removed: Vec<_> = command
        .get_envs()
        .filter(|(_, value)| value.is_none())
        .map(|(name, _)| name.to_string_lossy().into_owned())
        .collect();
    for name in GIT_REDIRECTING_ENVIRONMENT {
        assert!(
            removed.iter().any(|removed| removed == name),
            "{name} not removed"
        );
    }
}

#[test]
fn the_pin_list_itself_is_drift_checked() {
    let pins = SurfacePins {
        schema_version: 3,
        files: vec![SurfacePin {
            path: "a.rs".to_string(),
            reason: None,
        }],
    };
    assert_eq!(paths_to_guard(&pins), vec!["a.rs", SURFACE_RELATIVE_PATH]);
    // The pin list is checked like any pin: an uncommitted edit that drops an entry refuses.
    let repo = Repo::new("pinlist");
    let directory = repo.0.join("docs/tally/compatibility");
    fs::create_dir_all(&directory).unwrap();
    let list = directory.join("compatibility-surface.json");
    fs::write(
        &list,
        b"{\"schema_version\":3,\"files\":[{\"path\":\"a.rs\"}]}\n",
    )
    .unwrap();
    git_in(&repo.0, &["add", "."]);
    git_in(&repo.0, &["commit", "-q", "-m", "list"]);
    let paths = paths_to_guard(&pins);
    assert_eq!(repo.check(&paths), Ok(()));
    fs::write(&list, b"{\"schema_version\":3,\"files\":[]}\n").unwrap();
    assert_eq!(repo.check(&paths), DRIFT);
}
