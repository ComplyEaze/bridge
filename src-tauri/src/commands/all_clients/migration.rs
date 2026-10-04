//! The All Clients screen's read-only label-migration planner (#839). It is the one command over
//! operator filing labels that may open the local mirror, so it lives in its own module: the rest
//! of `commands/all_clients.rs` can then be checked as a whole to touch neither the mirror nor the
//! keychain (`lib_client_preference_mount_tests.rs`).
use crate::client_group_label_migration::{
    classify_client_group_label_migration, ClientGroupLabelMigrationPlan,
};
use crate::client_groups;
use serde::Serialize;
use tauri::{AppHandle, Manager, State};

/// A safe, typed failure returned by the read-only label-migration planner.
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum ClientGroupLabelMigrationPreparationError {
    ConfigurationUnavailable,
    LabelsUnavailable,
    MirrorUnavailable,
    PersistedProfilesUnavailable,
}

pub(in crate::commands) fn load_client_group_labels_for_migration(
    directory: &std::path::Path,
) -> Result<client_groups::ClientGroupLabels, ClientGroupLabelMigrationPreparationError> {
    client_groups::try_load(directory)
        .map_err(|_| ClientGroupLabelMigrationPreparationError::LabelsUnavailable)
}

pub(in crate::commands) async fn prepare_client_group_label_migration_from_labels<F, Fut>(
    labels: client_groups::ClientGroupLabels,
    open_mirror_and_load_profiles: F,
) -> Result<ClientGroupLabelMigrationPlan, ClientGroupLabelMigrationPreparationError>
where
    F: FnOnce(Vec<String>) -> Fut,
    Fut: std::future::Future<
        Output = Result<
            Vec<crate::db::tally_mirror::ClientGroupLabelMigrationProfile>,
            ClientGroupLabelMigrationPreparationError,
        >,
    >,
{
    if labels.is_empty() {
        return Ok(classify_client_group_label_migration(&labels, &[]));
    }

    let raw_guids = labels.keys().cloned().collect::<Vec<_>>();
    let profiles = open_mirror_and_load_profiles(raw_guids).await?;
    Ok(classify_client_group_label_migration(&labels, &profiles))
}

/// Explicitly prepares a read-only migration plan. Unlike ordinary label
/// reads and saves, this operator-requested command may initialise the mirror
/// to inspect durable observed-company history; it never calls the label
/// writer. It fails closed if the v1 label file cannot be read, so a phase-2
/// consumer can never treat unread local input as an empty migration. An
/// absent v1 label file returns an empty plan before the mirror/keychain path.
#[tauri::command]
pub async fn prepare_client_group_label_migration(
    app: AppHandle,
    mirror: State<'_, crate::LazyTallyMirror>,
) -> Result<ClientGroupLabelMigrationPlan, ClientGroupLabelMigrationPreparationError> {
    let labels = app
        .path()
        .app_config_dir()
        .map_err(|_| ClientGroupLabelMigrationPreparationError::ConfigurationUnavailable)
        .and_then(|directory| load_client_group_labels_for_migration(&directory))?;
    prepare_client_group_label_migration_from_labels(labels, |raw_guids| async move {
        let mirror = mirror
            .get()
            .await
            .map_err(|_| ClientGroupLabelMigrationPreparationError::MirrorUnavailable)?;
        mirror
            .persisted_company_profiles_for_client_group_label_migration(&raw_guids)
            .await
            .map_err(|_| ClientGroupLabelMigrationPreparationError::PersistedProfilesUnavailable)
    })
    .await
}
