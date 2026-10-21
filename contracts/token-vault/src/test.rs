#![cfg(test)]

use super::{TokenVault, TokenVaultClient, MAX_SUPPLY};
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
#[should_panic(expected = "insufficient balance")]
fn test_transfer_more_than_balance_panics() {
    let (_env, client, _admin, alice, bob) = setup();

    client.mint(&alice, &100);
    client.transfer(&alice, &bob, &101);
}

#[test]
#[should_panic(expected = "amount must not be negative")]
fn test_negative_amount_transfer_panics() {
    let (_env, client, _admin, alice, bob) = setup();

    client.mint(&alice, &100);
    client.transfer(&alice, &bob, &-1);
}

#[test]
fn test_mint_up_to_max_supply_succeeds() {
    let (_env, client, _admin, alice, bob) = setup();

    // Minting exactly up to MAX_SUPPLY is allowed: the cap is inclusive.
    client.mint(&alice, &MAX_SUPPLY);
    assert_eq!(client.total_supply(), MAX_SUPPLY);
    assert_eq!(client.balance(&alice), MAX_SUPPLY);

    // A zero-value mint while already at the cap stays a graceful no-op.
    client.mint(&bob, &0);
    assert_eq!(client.total_supply(), MAX_SUPPLY);
    assert_eq!(client.balance(&bob), 0);
}

#[test]
#[should_panic(expected = "supply overflow")]
fn test_mint_beyond_max_supply_panics() {
    let (_env, client, _admin, alice, bob) = setup();

    // Fill total supply to the inclusive cap first, so the next mint is the
    // first amount that would push supply beyond MAX_SUPPLY.
    client.mint(&alice, &MAX_SUPPLY);
    assert_eq!(client.total_supply(), MAX_SUPPLY);

    // Minting 1 token past the cap must revert on the checked-add supply
    // boundary without crediting bob or corrupting total supply.
    client.mint(&bob, &1);
}
