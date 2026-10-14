//! DIAGNOSTIC ONLY (cx-f4thx, never lands): can a fleet step read the router's
//! Keychain install secret? The router mints keys through this load.
#[cfg(target_os = "macos")]
#[test]
fn keychain_probe_router_install_secret() {
    use cmux_coderouter::InstallSecretStore;
    let store = cmux_coderouter::KeychainInstallSecretStore::new("default");
    let result = store.load();
    eprintln!("KEYCHAIN PROBE: {:?}", result.as_ref().map(|_| "ok").map_err(|e| format!("{e:#}")));
    assert!(result.is_ok(), "KEYCHAIN PROBE FAILED: {:#}", result.err().unwrap());
}
