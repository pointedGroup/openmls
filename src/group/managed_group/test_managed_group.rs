use crate::prelude::*;

#[test]
fn test_managed_group_persistence() {
    use std::fs::File;
    use std::path::Path;
    let ciphersuite = &Config::supported_ciphersuites()[0];
    let group_id = GroupId::from_slice(b"Test Group");

    // Define credential bundles
    let alice_credential_bundle =
        CredentialBundle::new("Alice".into(), CredentialType::Basic, ciphersuite.name()).unwrap();

    // Generate KeyPackages
    let alice_key_package_bundle =
        KeyPackageBundle::new(&[ciphersuite.name()], &alice_credential_bundle, vec![]).unwrap();

    // Define the managed group configuration

    let update_policy = UpdatePolicy::default();
    let callbacks = ManagedGroupCallbacks::default();
    let managed_group_config =
        ManagedGroupConfig::new(HandshakeMessageFormat::Plaintext, update_policy, callbacks);

    // === Alice creates a group ===
    let alice_group = ManagedGroup::new(
        &alice_credential_bundle,
        &managed_group_config,
        group_id,
        alice_key_package_bundle,
    )
    .unwrap();

    let path = Path::new("target/test_managed_group_serialization.json");
    let out_file = &mut File::create(&path).expect("Could not create file");
    alice_group
        .save(out_file)
        .expect("Could not write group state to file");

    let in_file = File::open(&path).expect("Could not open file");

    let alice_group_deserialized = ManagedGroup::load(
        in_file,
        &alice_credential_bundle,
        &ManagedGroupCallbacks::default(),
    )
    .expect("Could not deserialize managed group");

    assert_eq!(alice_group, alice_group_deserialized);
}

#[test]
fn test_managed_group_errors() {
    let ciphersuite = &Config::supported_ciphersuites()[0];
    let group_id = GroupId::from_slice(b"Test Group");

    // Define credential bundles
    let alice_credential_bundle =
        CredentialBundle::new("Alice".into(), CredentialType::Basic, ciphersuite.name()).unwrap();

    // Generate KeyPackages
    let alice_key_package_bundle =
        KeyPackageBundle::new(&[ciphersuite.name()], &alice_credential_bundle, vec![]).unwrap();

    // Define the managed group configuration

    let update_policy = UpdatePolicy::default();
    let callbacks = ManagedGroupCallbacks::default();
    let managed_group_config =
        ManagedGroupConfig::new(HandshakeMessageFormat::Plaintext, update_policy, callbacks);

    // === Alice creates a group ===
    let mut alice_group = ManagedGroup::new(
        &alice_credential_bundle,
        &managed_group_config,
        group_id,
        alice_key_package_bundle,
    )
    .unwrap();

    let add_members_error = alice_group
        .add_members(&[])
        .expect_err("Adding an empty list of members did not return an error!");
    let remove_members_error = alice_group
        .remove_members(&[])
        .expect_err("Removing an empty list of members did not return an error!");

    assert_eq!(
        add_members_error,
        ManagedGroupError::EmptyInput(super::EmptyInputError::AddMembers)
    );
    assert_eq!(
        remove_members_error,
        ManagedGroupError::EmptyInput(super::EmptyInputError::RemoveMembers)
    );
}