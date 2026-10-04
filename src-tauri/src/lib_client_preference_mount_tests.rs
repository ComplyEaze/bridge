/// The All Clients commands over filing labels and the sort preference never open the Tally
/// mirror or the keychain. The whole of `commands/all_clients.rs` is checked, not spans found by
/// function name, so a command added anywhere in it is covered (#839). The one command that may
/// open the mirror, the label-migration planner, lives in its own module, which is checked to be
/// where the mirror is taken.
#[test]
fn client_preference_commands_are_mirror_and_keychain_free() {
    let commands = include_str!("commands/all_clients.rs");
    let migration = include_str!("commands/all_clients/migration.rs");

    for command in [
        "load_client_group_labels",
        "save_client_group_label",
        "replace_client_group_labels",
        "load_client_sort_preference",
        "save_client_sort_preference",
    ] {
        assert!(
            commands.contains(&format!("pub fn {command}(")),
            "{command} is no longer in commands/all_clients.rs"
        );
    }
    assert!(commands.contains("app_config_dir"));
    assert!(!commands.contains("LazyTallyMirror"));
    assert!(!commands.contains("keyring"));
    assert!(migration.contains("pub async fn prepare_client_group_label_migration("));
    assert!(migration.contains("State<'_, crate::LazyTallyMirror>"));
}
