#![cfg(test)]

use super::{TokenVault, TokenVaultClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup() -> (Env, TokenVaultClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    let contract_id = env.register(TokenVault, ());
    let client = TokenVaultClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &7,
        &String::from_str(&env, "Token Vault"),
        &String::from_str(&env, "TVLT"),
    );

    (env, client, admin, alice, bob)
}

#[test]
fn test_zero_amount_transfer_handled_gracefully() {
    let (_env, client, _admin, alice, bob) = setup();

    // Mint initial tokens to alice
    client.mint(&alice, &1000);
    assert_eq!(client.balance(&alice), 1000);
    assert_eq!(client.balance(&bob), 0);

    // Attempt transfer of 0 tokens from alice to bob
    // As per SEP-41 / token specifications, a zero-amount transfer must be handled gracefully:
    // the call succeeds without reverting, and account balances remain unchanged.
    client.transfer(&alice, &bob, &0);

    // Assert that balances are completely unchanged
    assert_eq!(client.balance(&alice), 1000);
    assert_eq!(client.balance(&bob), 0);
}

#[test]
fn test_zero_amount_transfer_with_zero_balance_handled_gracefully() {
    let (_env, client, _admin, alice, bob) = setup();

    // Alice starts with 0 balance
    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.balance(&bob), 0);

    // Transferring 0 tokens with 0 balance should also succeed gracefully
    client.transfer(&alice, &bob, &0);

    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.balance(&bob), 0);
}

#[test]
fn test_standard_transfer_moves_tokens() {
    let (_env, client, _admin, alice, bob) = setup();

    client.mint(&alice, &500);
    client.transfer(&alice, &bob, &200);

    assert_eq!(client.balance(&alice), 300);
    assert_eq!(client.balance(&bob), 200);
    assert_eq!(client.total_supply(), 500);
}

#[test]
fn test_transfer_insufficient_balance_preserves_state() {
    for initial_balance in [0, 100] {
        let (_env, client, _admin, alice, bob) = setup();
        client.mint(&alice, &initial_balance);
        client.mint(&bob, &7);

        assert_eq!(
            client.try_transfer(&alice, &bob, &(initial_balance + 1)),
            Err(Ok(soroban_sdk::Error::from_contract_error(3)))
        );
        assert_eq!(client.balance(&alice), initial_balance);
        assert_eq!(client.balance(&bob), 7);
        assert_eq!(client.total_supply(), initial_balance + 7);
    }
}

#[test]
fn test_transfer_from_insufficient_balance_preserves_state() {
    for initial_balance in [0, 100] {
        let (env, client, _admin, alice, bob) = setup();
        let spender = Address::generate(&env);
        client.mint(&alice, &initial_balance);
        client.mint(&bob, &7);
        client.approve(&alice, &spender, &200, &0);

        assert_eq!(
            client.try_transfer_from(&spender, &alice, &bob, &(initial_balance + 1)),
            Err(Ok(soroban_sdk::Error::from_contract_error(3)))
        );
        assert_eq!(client.balance(&alice), initial_balance);
        assert_eq!(client.balance(&bob), 7);
        assert_eq!(client.allowance(&alice, &spender), 200);
        assert_eq!(client.total_supply(), initial_balance + 7);
    }
}

#[test]
fn test_burn_insufficient_balance_preserves_state() {
    for initial_balance in [0, 100] {
        let (_env, client, _admin, alice, bob) = setup();
        client.mint(&alice, &initial_balance);
        client.mint(&bob, &7);

        assert_eq!(
            client.try_burn(&alice, &(initial_balance + 1)),
            Err(Ok(soroban_sdk::Error::from_contract_error(3)))
        );
        assert_eq!(client.balance(&alice), initial_balance);
        assert_eq!(client.balance(&bob), 7);
        assert_eq!(client.total_supply(), initial_balance + 7);
    }
}

#[test]
fn test_transfer_entire_balance_succeeds() {
    let (_env, client, _admin, alice, bob) = setup();
    client.mint(&alice, &100);
    client.transfer(&alice, &bob, &100);
    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.balance(&bob), 100);
    assert_eq!(client.total_supply(), 100);
}

#[test]
fn test_transfer_from_entire_balance_succeeds() {
    let (env, client, _admin, alice, bob) = setup();
    let spender = Address::generate(&env);
    client.mint(&alice, &100);
    client.approve(&alice, &spender, &100, &0);
    client.transfer_from(&spender, &alice, &bob, &100);
    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.balance(&bob), 100);
    assert_eq!(client.allowance(&alice, &spender), 0);
    assert_eq!(client.total_supply(), 100);
}

#[test]
fn test_burn_entire_balance_succeeds() {
    let (_env, client, _admin, alice, bob) = setup();
    client.mint(&alice, &100);
    client.mint(&bob, &7);
    client.burn(&alice, &100);
    assert_eq!(client.balance(&alice), 0);
    assert_eq!(client.balance(&bob), 7);
    assert_eq!(client.total_supply(), 7);
}

#[test]
#[should_panic(expected = "amount must not be negative")]
fn test_negative_amount_transfer_panics() {
    let (_env, client, _admin, alice, bob) = setup();

    client.mint(&alice, &100);
    client.transfer(&alice, &bob, &-1);
}
