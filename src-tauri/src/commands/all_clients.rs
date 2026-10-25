//! Desktop commands for the All Clients screen's local configuration: operator-owned filing
//! labels, their read-only migration plan (in `migration`), and the all-client sort preference.
//! None of them reads Tally; only the migration planner may open the local mirror, and it is the
//! only one outside this file.
use crate::client_groups;
use serde::Deserialize;
use tauri::{AppHandle, Manager};

pub mod migration;

/// Reads operator-owned filing labels from ordinary application configuration.
///
/// The helper deliberately remains display-safe for a missing, empty, corrupt,
/// or unavailable file, while returning a typed degradation reason when one
/// exists. It receives no mirror state, so this command cannot initialise
/// SQLCipher or resolve a keychain key.
#[tauri::command]
pub fn load_client_group_labels(app: AppHandle) -> client_groups::ClientGroupLabelsLoad {
    let Ok(directory) = app.path().app_config_dir() else {
        return client_groups::ClientGroupLabelsLoad {
            labels: client_groups::ClientGroupLabels::new(),
            degradation_reason: Some(client_groups::ClientGroupLabelsDegradationReason::Read),
        };
    };
    client_groups::load_with_degradation(&directory)
}

/// Reads the optional all-client sort preference from ordinary application
/// configuration. Like group labels, it never initialises the Tally mirror.
#[tauri::command]
pub fn load_client_sort_preference(app: AppHandle) -> Option<client_groups::ClientSortPreference> {
    let Ok(directory) = app.path().app_config_dir() else {
        return None;
    };
    client_groups::load_sort_preference(&directory)
}

#[derive(Debug, Deserialize)]
pub struct SaveClientGroupLabelRequest {
    pub company_key: String,
    pub label: String,
}

/// Saves one operator-owned filing label without accessing the Tally mirror.
#[tauri::command]
pub fn save_client_group_label(
    app: AppHandle,
    request: SaveClientGroupLabelRequest,
) -> Result<(), String> {
    if request.company_key.trim().is_empty() {
        return Err(
            "ComplyEaze Bridge could not identify the company for this group label.".to_string(),
        );
    }
    let directory = app.path().app_config_dir().map_err(|_| {
        "ComplyEaze Bridge could not locate its local group-label configuration.".to_string()
    })?;
    client_groups::save_label(&directory, &request.company_key, &request.label)
        .map_err(|_| "ComplyEaze Bridge could not save this group label.".to_string())
}

#[derive(Debug, Deserialize)]
pub struct ReplaceClientGroupLabelsRequest {
    pub labels: client_groups::ClientGroupLabels,
}

/// Atomically persists the one-time local migration from raw GUID keys to
/// composite company keys. It never accesses the Tally mirror.
#[tauri::command]
pub fn replace_client_group_labels(
    app: AppHandle,
    request: ReplaceClientGroupLabelsRequest,
) -> Result<(), String> {
    let directory = app.path().app_config_dir().map_err(|_| {
        "ComplyEaze Bridge could not locate its local group-label configuration.".to_string()
    })?;
    client_groups::replace_labels(&directory, request.labels)
        .map_err(|_| "ComplyEaze Bridge could not migrate local group labels.".to_string())
}

/// Saves the optional all-client sort preference without accessing the Tally mirror.
#[tauri::command]
pub fn save_client_sort_preference(
    app: AppHandle,
    preference: client_groups::ClientSortPreference,
) -> Result<(), String> {
    let directory = app.path().app_config_dir().map_err(|_| {
        "ComplyEaze Bridge could not locate its local client-preference configuration.".to_string()
    })?;
    client_groups::save_sort_preference(&directory, preference)
        .map_err(|_| "ComplyEaze Bridge could not save the all-client sort preference.".to_string())
}
