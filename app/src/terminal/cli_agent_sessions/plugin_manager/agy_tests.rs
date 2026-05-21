use std::fs;

use super::{
    check_installed, compare_versions, installed_version, CliAgentPluginManager,
    AgyPluginManager, MINIMUM_PLUGIN_VERSION,
};

#[test]
fn can_auto_install_is_true() {
    assert!(AgyPluginManager::new(None, None, None).can_auto_install());
}

#[test]
fn minimum_version() {
    assert_eq!(
        AgyPluginManager::new(None, None, None).minimum_plugin_version(),
        "1.0.0"
    );
}

#[test]
fn install_instructions_has_steps() {
    let instructions = AgyPluginManager::new(None, None, None).install_instructions();
    assert!(!instructions.steps.is_empty());
    assert!(!instructions.title.is_empty());
}

#[test]
fn update_instructions_has_steps() {
    let instructions = AgyPluginManager::new(None, None, None).update_instructions();
    assert!(!instructions.steps.is_empty());
    assert!(!instructions.title.is_empty());
}

#[test]
fn installed_when_extension_present() {
    let dir = tempfile::tempdir().unwrap();
    let ext_dir = dir.path().join("agy-warp");
    fs::create_dir_all(&ext_dir).unwrap();

    let json = serde_json::json!({
        "name": "warp",
        "version": "1.0.0",
        "description": "Warp terminal integration for Antigravity CLI"
    });
    fs::write(
        ext_dir.join("agy-extension.json"),
        serde_json::to_string(&json).unwrap(),
    )
    .unwrap();

    assert!(check_installed(dir.path()));
}

#[test]
fn not_installed_when_extension_missing() {
    let dir = tempfile::tempdir().unwrap();
    assert!(!check_installed(dir.path()));
}

#[test]
fn not_installed_when_json_invalid() {
    let dir = tempfile::tempdir().unwrap();
    let ext_dir = dir.path().join("agy-warp");
    fs::create_dir_all(&ext_dir).unwrap();
    fs::write(ext_dir.join("agy-extension.json"), "not json").unwrap();

    assert!(!check_installed(dir.path()));
}

#[test]
fn installed_version_returns_version_when_present() {
    let dir = tempfile::tempdir().unwrap();
    let ext_dir = dir.path().join("agy-warp");
    fs::create_dir_all(&ext_dir).unwrap();

    let json = serde_json::json!({
        "name": "warp",
        "version": "1.5.0"
    });
    fs::write(
        ext_dir.join("agy-extension.json"),
        serde_json::to_string(&json).unwrap(),
    )
    .unwrap();

    assert_eq!(installed_version(dir.path()).as_deref(), Some("1.5.0"));
}

#[test]
fn installed_version_returns_none_when_no_version_field() {
    let dir = tempfile::tempdir().unwrap();
    let ext_dir = dir.path().join("agy-warp");
    fs::create_dir_all(&ext_dir).unwrap();

    let json = serde_json::json!({
        "name": "warp"
    });
    fs::write(
        ext_dir.join("agy-extension.json"),
        serde_json::to_string(&json).unwrap(),
    )
    .unwrap();

    assert_eq!(installed_version(dir.path()), None);
}

#[test]
fn installed_version_returns_none_when_file_missing() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(installed_version(dir.path()), None);
}

#[test]
fn needs_update_logic_true_when_version_outdated() {
    let dir = tempfile::tempdir().unwrap();
    let ext_dir = dir.path().join("agy-warp");
    fs::create_dir_all(&ext_dir).unwrap();

    let json = serde_json::json!({
        "name": "warp",
        "version": "0.9.0"
    });
    fs::write(
        ext_dir.join("agy-extension.json"),
        serde_json::to_string(&json).unwrap(),
    )
    .unwrap();

    let needs_update = match installed_version(dir.path()) {
        Some(v) => compare_versions(&v, MINIMUM_PLUGIN_VERSION).is_lt(),
        None => check_installed(dir.path()),
    };
    assert!(needs_update);
}

#[test]
fn needs_update_logic_false_when_version_current() {
    let dir = tempfile::tempdir().unwrap();
    let ext_dir = dir.path().join("agy-warp");
    fs::create_dir_all(&ext_dir).unwrap();

    let json = serde_json::json!({
        "name": "warp",
        "version": "1.0.0"
    });
    fs::write(
        ext_dir.join("agy-extension.json"),
        serde_json::to_string(&json).unwrap(),
    )
    .unwrap();

    let needs_update = match installed_version(dir.path()) {
        Some(v) => compare_versions(&v, MINIMUM_PLUGIN_VERSION).is_lt(),
        None => check_installed(dir.path()),
    };
    assert!(!needs_update);
}
