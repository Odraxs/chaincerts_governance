#![cfg(test)]

use crate::{option::OptionU64, Wallet, WalletClient};
use soroban_sdk::{testutils::Address as _, vec, Address, Bytes, Env, IntoVal, Vec};

fn create_wallet(e: &Env, owner: &Address) -> WalletClient {
    let wallet = WalletClient::new(e, &e.register_contract(None, Wallet {}));
    wallet.initialize(owner);
    wallet
}

struct ChaincertWalletTest {
    env: Env,
    owner: Address,
    distributor_contract: Address,
    wallet: WalletClient,
    chaincert_id: Bytes,
    organizations: Vec<Bytes>,
    cids: Vec<Bytes>,
}

impl ChaincertWalletTest {
    fn setup() -> Self {
        let env: Env = Default::default();
        let owner = Address::random(&env);
        let distributor_contract = Address::random(&env);
        let wallet = create_wallet(&env, &owner);
        let chaincert_id: Bytes = "CHAINCERT1".into_val(&env);
        let org_id1: Bytes = "ORG1".into_val(&env);
        let org_id2: Bytes = "ORG2".into_val(&env);
        let organizations = vec![&env, org_id1, org_id2];
        let cid1: Bytes = "QmdtyfTYbVS3K9iYqBPjXxn4mbB7aBvEjYGzYWnzRcMrEC".into_val(&env);
        let cids = vec![&env, cid1];

        ChaincertWalletTest {
            env,
            owner,
            distributor_contract,
            wallet,
            chaincert_id,
            organizations,
            cids,
        }
    }
}

#[test]
fn test_successful_execution_of_wallet_capabilities() {
    let test = ChaincertWalletTest::setup();
    let new_chiancert_id = "CHAINCERT2".into_val(&test.env);

    test.wallet
        .add_organization(&test.organizations.get_unchecked(0).unwrap());
    test.wallet
        .add_organization(&test.organizations.get_unchecked(1).unwrap());

    assert_eq!(test.wallet.get_access_control_list().len(), 2);

    test.wallet.deposit_chaincert(
        &test.chaincert_id,
        &test.cids.get_unchecked(0).unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
    );

    test.wallet.deposit_chaincert(
        &new_chiancert_id,
        &test.cids.get_unchecked(0).unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680205831,
        &OptionU64::None,
    );

    assert_eq!(test.wallet.get_chaincerts().len(), 2);

    test.wallet.revoke_chaincert(
        &test.chaincert_id,
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
    );

    test.wallet
        .remove_organization(&test.organizations.get_unchecked(0).unwrap());
    assert_eq!(test.wallet.get_access_control_list().len(), 1);
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn test_initialize_an_already_initialized_wallet() {
    let test = ChaincertWalletTest::setup();
    test.wallet.initialize(&test.owner);
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn test_when_adding_an_already_added_org() {
    let test = ChaincertWalletTest::setup();

    test.wallet
        .add_organization(&test.organizations.get_unchecked(0).unwrap());
    test.wallet
        .add_organization(&test.organizations.get_unchecked(0).unwrap());
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn test_remove_organization_when_not_organizations_already_set() {
    let test = ChaincertWalletTest::setup();
    test.wallet
        .remove_organization(&test.organizations.get_unchecked(0).unwrap());
}

#[test]
#[should_panic(expected = "Status(ContractError(8))")]
fn test_remove_organization_when_organization_not_found() {
    let test = ChaincertWalletTest::setup();
    test.wallet
        .add_organization(&test.organizations.get_unchecked(0).unwrap());
    test.wallet
        .remove_organization(&test.organizations.get_unchecked(1).unwrap());
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_deposit_chaincert_when_organization_is_not_in_the_acl() {
    let test = ChaincertWalletTest::setup();

    test.wallet
        .add_organization(&test.organizations.get_unchecked(0).unwrap());

    test.wallet.deposit_chaincert(
        &test.chaincert_id,
        &test.cids.get(0).unwrap().unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(1).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn test_deposit_chaincert_when_no_organizations_in_the_acl() {
    let test = ChaincertWalletTest::setup();

    test.wallet.deposit_chaincert(
        &test.chaincert_id,
        &test.cids.get(0).unwrap().unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(1).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(9))")]
fn test_deposit_chaincert_chaincert_is_already_in_the_wallet() {
    let test = ChaincertWalletTest::setup();

    test.wallet
        .add_organization(&test.organizations.get_unchecked(0).unwrap());
    test.wallet
        .add_organization(&test.organizations.get_unchecked(1).unwrap());

    test.wallet.deposit_chaincert(
        &test.chaincert_id,
        &test.cids.get_unchecked(0).unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
    );

    test.wallet.deposit_chaincert(
        &test.chaincert_id,
        &test.cids.get_unchecked(0).unwrap(),
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
        &1680105831,
        &OptionU64::Some(1711662757),
    );
}

#[test]
#[should_panic(expected = "Status(ContractError(11))")]
fn test_revoke_chaincert_when_no_chaincerts_in_wallet() {
    let test = ChaincertWalletTest::setup();

    test.wallet
        .add_organization(&test.organizations.get_unchecked(0).unwrap());
    test.wallet.revoke_chaincert(
        &test.chaincert_id,
        &test.distributor_contract,
        &test.organizations.get_unchecked(0).unwrap(),
    )
}

#[test]
#[should_panic(expected = "Status(ContractError(10))")]
fn test_revoke_chaincert_when_chaincert_not_found() {
    let test = ChaincertWalletTest::setup();
    let org1 = test.organizations.get_unchecked(0).unwrap();
    let new_chaincert: Bytes = "CHAINCERT2".into_val(&test.env);

    test.wallet.add_organization(&org1);
    test.wallet.deposit_chaincert(
        &test.chaincert_id,
        &test.cids.get(0).unwrap().unwrap(),
        &test.distributor_contract,
        &org1,
        &1680105831,
        &OptionU64::Some(1711662757),
    );

    test.wallet
        .revoke_chaincert(&new_chaincert, &test.distributor_contract, &org1);
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn test_revoke_chaincert_when_not_authorized_contract_or_organization() {
    let test = ChaincertWalletTest::setup();
    let org1 = test.organizations.get_unchecked(0).unwrap();
    let org2 = test.organizations.get_unchecked(1).unwrap();

    test.wallet.add_organization(&org1);
    test.wallet.deposit_chaincert(
        &test.chaincert_id,
        &test.cids.get(0).unwrap().unwrap(),
        &test.distributor_contract,
        &org1,
        &1680105831,
        &OptionU64::Some(1711662757),
    );

    test.wallet
        .revoke_chaincert(&test.chaincert_id, &test.distributor_contract, &org2);
}

#[test]
#[should_panic(expected = "Status(ContractError(11))")]
fn test_request_chaincerts_when_no_chaincerts_set() {
    let test = ChaincertWalletTest::setup();

    test.wallet.get_chaincerts();
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn test_request_acl_when_no_organizations_set() {
    let test = ChaincertWalletTest::setup();

    test.wallet.get_access_control_list();
}
