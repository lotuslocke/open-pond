#[cfg(test)]
use crate::crypto::AuthKey;
#[cfg(test)]
use serial_test::serial;

// Helper function to setup initial authentication key
#[cfg(test)]
fn setup_auth() {
    std::fs::create_dir("private").unwrap();
}

// Helper function to teardown authentication key
#[cfg(test)]
fn teardown_auth() {
    std::fs::remove_file("private/auth-keypair").unwrap();
    std::fs::remove_dir("private").unwrap();
}

#[test]
#[serial]
fn load_success_generation() {
    setup_auth();

    let auth_key = AuthKey::load();

    assert!(auth_key.is_ok());

    teardown_auth();
}

#[test]
#[serial]
fn load_success_from_file() {
    setup_auth();

    let gen_key = AuthKey::load().unwrap();
    let loaded_key = AuthKey::load().unwrap();

    assert_eq!(gen_key, loaded_key);

    teardown_auth();
}

#[test]
#[serial]
fn load_success_no_private_directory() {
    let auth_key = AuthKey::load();

    assert!(auth_key.is_ok());

    teardown_auth();
}

#[test]
#[serial]
fn sign_and_verify_success() {
    setup_auth();

    let key = AuthKey::load().unwrap();
    let message = vec![1, 2, 3];

    let signature = key.sign(&message).unwrap();
    let pub_key = key.get_public();
    let is_valid = AuthKey::verify(&message, signature, pub_key).unwrap();

    assert!(is_valid);

    teardown_auth();
}

#[test]
#[serial]
fn sign_and_verify_invalid_signature() {
    setup_auth();

    let key = AuthKey::load().unwrap();
    let message = vec![1, 2, 3];

    let mut signature = key.sign(&message).unwrap();
    let pub_key = key.get_public();

    signature[0] = signature[0] + 1;
    let is_valid = AuthKey::verify(&message, signature, pub_key).unwrap();

    assert!(!is_valid);

    teardown_auth();
}
