#![cfg(test)]

extern crate std;

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::contract::{MyCoinContract, MyCoinContractClient};

fn create_client<'a>(e: &Env, owner: &Address, initial_supply: i128) -> MyCoinContractClient<'a> {
    let address = e.register(MyCoinContract, (owner, initial_supply));
    MyCoinContractClient::new(e, &address)
}

#[test]
fn initial_state() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let client = create_client(&e, &owner, 1000);
    e.mock_all_auths();
    client.mint(&owner, &1000);
    assert_eq!(client.total_supply(), 1000);
    assert_eq!(client.balance(&owner), 1000);
    assert_eq!(client.symbol(), String::from_str(&e, "MC"));
    assert_eq!(client.name(), String::from_str(&e, "My Coin"));
    assert_eq!(client.decimals(), 18);
}

#[test]
fn transfer_works() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let recipient = Address::generate(&e);
    let client = create_client(&e, &owner, 1000);
    e.mock_all_auths();
    client.mint(&owner, &1000);
    client.transfer(&owner, &recipient, &100);
    assert_eq!(client.balance(&owner), 900);
    assert_eq!(client.balance(&recipient), 100);
}

#[test]
fn transfer_from_works() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let spender = Address::generate(&e);
    let recipient = Address::generate(&e);
    let client = create_client(&e, &owner, 1000);
    e.mock_all_auths();
    client.mint(&owner, &1000);
    client.approve(&owner, &spender, &200, &100);
    client.transfer_from(&spender, &owner, &recipient, &200);
    assert_eq!(client.balance(&owner), 800);
    assert_eq!(client.balance(&recipient), 200);
}

#[test]
fn mint_works() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let client = create_client(&e, &owner, 1500);
    e.mock_all_auths();
    client.mint(&owner, &1000);
    client.mint(&owner, &500);
    assert_eq!(client.total_supply(), 1500);
    assert_eq!(client.balance(&owner), 1500);
}

#[test]
fn burn_works() {
    let e = Env::default();
    let owner = Address::generate(&e);
    let client = create_client(&e, &owner, 1000);
    e.mock_all_auths();
    client.mint(&owner, &1000);
    client.burn(&owner, &200);
    assert_eq!(client.total_supply(), 800);
    assert_eq!(client.balance(&owner), 800);
}

