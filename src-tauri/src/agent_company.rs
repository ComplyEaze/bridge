//! Company for the local MCP adapter.
use super::*;
use bridge_tally_core::{CapabilityFeatureId, CapabilityState, EvidenceConfidence};
use bridge_tally_protocol::xml_read_profiles::ShownCompanyName;

impl Server {
    pub(super) async fn status(&self) -> Result<(Value, Evidence), ToolFailure> {
        self.status_for_release(RELEASE_DATE).await
    }

    /// `status` for a build dated `release_date`; the date is a parameter so a test can drive the
    /// build notice's insertion with a fixed date (#2391).
    pub(super) async fn status_for_release(
        &self,
        release_date: &str,
    ) -> Result<(Value, Evidence), ToolFailure> {
        let endpoint = endpoint_origin(&self.settings.endpoint)?;
        let (probe, wire_evidence) = self
            .runtime
            .probe_with_wire_evidence(self.tally_config())
            .await
            .map_err(|error| ToolFailure::from_runtime("status_probe_unavailable", error))?;
        // Product identity comes from the gateway observation, not the optional
        // status page's heuristic banner. Unknown capability stays explicit.
        let observed = probe
            .profile
            .features
            .get(&CapabilityFeatureId::ProductAndMode)
            .is_some_and(|feature| {
                feature.state == CapabilityState::Supported
                    && feature.confidence == EvidenceConfidence::Observed
            });
        let product = if observed {
            probe.profile.product.as_str()
        } else {
            "not_observed"
        };
        let education_mode = match (observed, probe.profile.mode.as_deref()) {
            (true, Some("Education")) => Some(true),
            (true, Some("Licensed")) => Some(false),
            _ => None,
        };
        let refusal_reason = probe
            .profile
            .features
            .get(&CapabilityFeatureId::LoadedCompanies)
            .filter(|feature| feature.state == CapabilityState::Unknown)
            .map(|feature| {
                feature
                    .safe_reason_code
                    .clone()
                    .unwrap_or_else(|| "company_list_not_established".to_string())
            });
        let mut evidence = evidence_from_runtime_read(wire_evidence);
        if refusal_reason.is_some() {
            evidence.state = "partial";
            evidence.reason_code = refusal_reason.clone();
        }
        // One reading of the date for both the field and the notice, so they cannot disagree across midnight.
        let today = tally_host_today();
        let mut result = json!({
            "product": product,
            "release": probe.profile.release,
            "license_tier": probe.profile.license_tier,
            "education_mode": education_mode,
            "endpoint": endpoint,
            "loaded_companies": probe.companies,
            "refusal_reason": refusal_reason,
            // The host date every omitted date defaults to (YYYYMMDD).
            "today": today,
        });
        if let Some(notice) = self.take_build_notice(release_date, &today) {
            result["build_notice"] = json!(notice);
        }
        Ok((result, evidence))
    }

    pub(super) async fn companies(&self) -> Result<(Vec<TallyCompany>, Evidence), ToolFailure> {
        let company_list = self
            .runtime
            .fetch_agent_companies(self.tally_config())
            .await
            .map_err(|error| ToolFailure::from_runtime("company_collection_invalid", error))?;
        let evidence = Evidence {
            request_sha256: sha256_hex(&bridge_tally_protocol::encode_tally_xml_request_utf16le(
                &ReadOnlyProfile::CompanyListV2.render(),
            )),
            response_sha256: company_list.response_sha256,
            bytes: company_list.response_bytes.saturating_mul(2),
            state: "complete",
            read_at: None,
            duration_ms: None,
            reason_code: None,
        };
        Ok((company_list.companies, evidence))
    }

    pub(super) async fn verified_company(
        &self,
        guid: &str,
    ) -> Result<(TallyCompany, VerifiedCompanyIdentity, Evidence), ToolFailure> {
        if guid.trim().is_empty() {
            return Err("company_guid_required".to_string().into());
        }
        let (companies, evidence) = self.companies().await?;
        let result: Result<(TallyCompany, VerifiedCompanyIdentity, Evidence), ToolFailure> =
            async {
                let observed_companies = companies.clone();
                let mut matches = Vec::new();
                for company in companies {
                    let observed_guid = company
                        .guid
                        .as_deref()
                        .filter(|guid| !guid.trim().is_empty())
                        .map(parse_native_company_guid)
                        .transpose()?;
                    if company
                        .guid
                        .as_deref()
                        .is_some_and(|value| value.eq_ignore_ascii_case(guid))
                    {
                        if let Some(observed_guid) = observed_guid {
                            matches.push((company, observed_guid));
                        }
                    }
                }
                if matches.len() != 1 {
                    return Err(if matches.is_empty() {
                        "company_identity_not_found".to_string()
                    } else {
                        "company_identity_ambiguous".to_string()
                    }
                    .into());
                }
                let (company, observed_guid) =
                    matches.into_iter().next().expect("one checked above");
                // Every read of a company passes here. A company whose name a
                // request cannot carry is listed, marked, and refused here,
                // before any request names it (#2433).
                if ValidatedCompanyName::refusal(&company.name).is_some() {
                    return Err(super::failure_code::CompanyReadCode::NameInvalid.into());
                }
                let company_number = company
                    .company_number
                    .clone()
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| "company_identity_incomplete".to_string())?;
                let books_from = company
                    .books_from
                    .clone()
                    .filter(|value| !value.is_empty())
                    .ok_or_else(|| "company_identity_incomplete".to_string())?;
                let identity = VerifiedCompanyIdentity::from_observed_companies(
                    company.name.clone(),
                    observed_guid,
                    company_number,
                    books_from,
                    &observed_companies,
                )
                .map_err(|error| {
                    match error {
                        // The name check above refuses such a name first; this arm
                        // gives the same declared code if that order ever changes.
                        crate::tally::VerifiedCompanyIdentityError::InvalidCompanyName => {
                            super::failure_code::CompanyReadCode::NameInvalid.as_str()
                        }
                        crate::tally::VerifiedCompanyIdentityError::InvalidCompanyNumber => {
                            "company_number_invalid"
                        }
                        crate::tally::VerifiedCompanyIdentityError::InvalidBooksFrom => {
                            "company_books_from_invalid"
                        }
                        crate::tally::VerifiedCompanyIdentityError::Missing => {
                            "company_identity_not_found"
                        }
                        crate::tally::VerifiedCompanyIdentityError::DuplicateTuple => {
                            "company_identity_ambiguous"
                        }
                        crate::tally::VerifiedCompanyIdentityError::DisplayScopeAmbiguous => {
                            "company_display_scope_ambiguous"
                        }
                    }
                    .to_string()
                })?;
                Ok((company, identity, evidence.clone()))
            }
            .await;
        result.map_err(|failure| failure.with_prior_evidence(evidence))
    }
}

pub(super) fn parse_native_company_guid(
    value: &str,
) -> Result<bridge_tally_core::company_guid::CompanyGuid, String> {
    // Native company GUIDs use hyphenated UUID spelling. Case is accepted and
    // normalised; no UUID alias that the native identity bracket rejects is.
    bridge_tally_core::company_guid::CompanyGuid::parse(value)
        .map_err(|_| "company_guid_invalid".to_string())
}

pub(super) fn company_json(company: &TallyCompany, all: &[TallyCompany]) -> Value {
    let guid = company.guid.clone();
    let duplicate_guid = guid.as_deref().is_some_and(|candidate| {
        all.iter()
            .filter(|other| {
                other
                    .guid
                    .as_deref()
                    .is_some_and(|value| value.eq_ignore_ascii_case(candidate))
            })
            .count()
            > 1
    });
    let missing = [
        ("name", Some(company.name.as_str())),
        ("guid", company.guid.as_deref()),
        ("company_number", company.company_number.as_deref()),
        ("books_from", company.books_from.as_deref()),
    ]
    .into_iter()
    .find_map(|(field, value)| {
        value
            .filter(|value| !value.trim().is_empty())
            .is_none()
            .then_some(field)
    });
    let invalid_guid = guid
        .as_deref()
        .is_some_and(|guid| parse_native_company_guid(guid).is_err());
    let invalid_books_from = company
        .books_from
        .as_ref()
        .is_some_and(|value| bridge_tally_core::TallyDate::parse(value.clone()).is_err());
    let invalid_number = company
        .company_number
        .as_deref()
        .is_some_and(|value| !crate::tally::validators::is_valid_company_number(value));
    // The one display rule for a company name, and its two limits, are
    // `ShownCompanyName`'s (#2433, #2449).
    let shown = ShownCompanyName::of(&company.name);
    let name_refusal = shown.refusal();
    let name = shown.into_text();
    let mut row = json!({"name": name, "guid": guid, "company_number": company.company_number, "books_from": company.books_from, "identity_state": if duplicate_guid {"ambiguous_duplicate_guid"} else if missing.is_some() {"incomplete_tuple"} else if invalid_guid {"invalid_guid"} else if invalid_number {"invalid_company_number"} else if invalid_books_from {"invalid_books_from"} else {"verified_tuple"}, "missing_field": missing});
    // Only a marked row carries the key, so every other company block is unchanged.
    if let Some(refusal) = name_refusal {
        row["name_refusal"] = json!(refusal.code());
    }
    row
}

#[cfg(test)]
#[path = "agent_status_tests.rs"]
mod status_tests;

#[cfg(test)]
#[path = "agent_status_identity_tests.rs"]
mod status_identity_tests;

#[cfg(test)]
#[path = "agent_company_identity_tests.rs"]
mod identity_tests;

#[cfg(test)]
#[path = "agent_company_tuple_tests.rs"]
mod tuple_tests;

impl Server {
    /// The build notice for this session's first status read, if the build is old enough, else None.
    /// The session's one chance is spent only when a notice is produced, so a read on a day with no
    /// notice leaves it for a later read; it is spent when the notice is built, so a response that
    /// is then withheld (too large, cancelled) loses it for the session, which is accepted (#2391).
    pub(super) fn take_build_notice(&self, release_date: &str, today: &str) -> Option<String> {
        let notice = build_age_notice(release_date, today)?;
        (!self
            .build_notice_sent
            .swap(true, std::sync::atomic::Ordering::SeqCst))
        .then_some(notice)
    }
}

/// The one line `tally_status` adds, once per session, when this build's release date is more than
/// [`BUILD_NOTICE_AFTER_DAYS`] days before `today` (YYYYMMDD, the computer's date). It states the
/// two dates and where releases are listed and claims nothing else: it cannot know that a newer
/// release exists, and a wrong computer clock shows in the line itself. It makes no request. An
/// unreadable date on either side gives no notice (#2391).
pub(super) fn build_age_notice(release_date: &str, today: &str) -> Option<String> {
    let released = chrono::NaiveDate::parse_from_str(release_date, "%Y-%m-%d").ok()?;
    let today = chrono::NaiveDate::parse_from_str(today, "%Y%m%d").ok()?;
    ((today - released).num_days() > BUILD_NOTICE_AFTER_DAYS).then(|| {
        format!(
            "This build is dated {}; this computer's date is {}. Releases are listed at https://github.com/ComplyEaze/bridge/releases .",
            released.format("%Y-%m-%d"),
            today.format("%Y-%m-%d")
        )
    })
}

#[cfg(test)]
mod build_age_tests {
    use super::build_age_notice;

    #[test]
    fn the_notice_starts_after_sixty_days_and_not_before() {
        assert_eq!(build_age_notice("2026-10-30", "20261128"), None, "29 days");
        assert_eq!(build_age_notice("2026-10-30", "20261228"), None, "59 days");
        assert_eq!(build_age_notice("2026-10-30", "20261229"), None, "60 days");
        assert!(
            build_age_notice("2026-10-30", "20261230").is_some(),
            "61 days"
        );
        assert_eq!(
            build_age_notice("2026-10-30", "20260101"),
            None,
            "a clock before the release"
        );
        assert_eq!(
            build_age_notice("2026-10-30", "20261230").as_deref(),
            Some("This build is dated 2026-10-30; this computer's date is 2026-12-30. Releases are listed at https://github.com/ComplyEaze/bridge/releases ."),
            "states the two dates and claims nothing else"
        );
    }

    #[test]
    fn an_unreadable_date_gives_no_notice() {
        for (release, today) in [
            ("", "20270101"),
            ("2026-10", "20270101"),
            ("2026-10-30", ""),
            ("2026-10-30", "2027-01-01"),
            ("20261030", "20270101"),
        ] {
            assert_eq!(
                build_age_notice(release, today),
                None,
                "{release:?} {today:?}"
            );
        }
    }

    #[test]
    fn the_compiled_in_date_is_a_real_date() {
        // An empty date (no release-date.txt at build time) would silently turn the notice off.
        assert!(chrono::NaiveDate::parse_from_str(super::RELEASE_DATE, "%Y-%m-%d").is_ok());
    }
}

#[cfg(test)]
#[path = "agent_company_name_tests.rs"]
mod name_tests;
