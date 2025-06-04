/*
    Example Soroban SEP41 Fungible Token
       Using OpenZeppelin Libraries

        .------------------------.
    }=>/  __------------------__  \
      /       Soroban SEP41        \
     }=>  Stellar ♥️ OpenZeppelin    )
      \   __     v.0.0.1      __   /
    }=>\    ------------------    /
        '------------------------'
                  `-----'
*/

use openzeppelin_fungible_token::{
    self as fungible, burnable::FungibleBurnable, mintable::FungibleMintable, FungibleToken,
};

use soroban_sdk::{
    contract, contracterror, contractimpl, panic_with_error, symbol_short, Address, Env, String,
    Symbol,
};

pub const OWNER: Symbol = symbol_short!("OWNER");
pub const CAP: Symbol = symbol_short!("CAP");

#[contract]
pub struct MyCoinContract;

#[contracterror]
pub enum MyCoinContractError {
    MaxSupplyExceeded = 1,
}

#[contractimpl]
impl MyCoinContract {
    pub fn __constructor(e: &Env, owner: Address, cap: i128) {
        fungible::metadata::set_metadata(
            e,
            18,
            String::from_str(e, "Adiccello"),
            String::from_str(e, "ADL"),
        );
        e.storage().instance().set(&OWNER, &owner);
        e.storage().instance().set(&CAP, &cap);
    }
}

#[contractimpl]
impl FungibleToken for MyCoinContract {
    fn total_supply(e: &Env) -> i128 {
        fungible::total_supply(e)
    }

    fn balance(e: &Env, account: Address) -> i128 {
        fungible::balance(e, &account)
    }

    fn allowance(e: &Env, owner: Address, spender: Address) -> i128 {
        fungible::allowance(e, &owner, &spender)
    }

    fn transfer(e: &Env, from: Address, to: Address, amount: i128) {
        fungible::transfer(e, &from, &to, amount);
    }

    fn transfer_from(e: &Env, spender: Address, from: Address, to: Address, amount: i128) {
        fungible::transfer_from(e, &spender, &from, &to, amount);
    }

    fn approve(e: &Env, owner: Address, spender: Address, amount: i128, live_until_ledger: u32) {
        fungible::approve(e, &owner, &spender, amount, live_until_ledger);
    }

    fn decimals(e: &Env) -> u32 {
        fungible::metadata::decimals(e)
    }

    fn name(e: &Env) -> String {
        fungible::metadata::name(e)
    }

    fn symbol(e: &Env) -> String {
        fungible::metadata::symbol(e)
    }
}

#[contractimpl]
impl FungibleBurnable for MyCoinContract {
    fn burn(e: &Env, from: Address, amount: i128) {
        fungible::burnable::burn(e, &from, amount)
    }

    fn burn_from(e: &Env, spender: Address, from: Address, amount: i128) {
        fungible::burnable::burn_from(e, &spender, &from, amount)
    }
}

#[contractimpl]
impl FungibleMintable for MyCoinContract {
    fn mint(e: &Env, account: Address, amount: i128) {
        let owner: Address = e
            .storage()
            .instance()
            .get(&OWNER)
            .expect("owner should be set");
        owner.require_auth();
        let cap: i128 = e.storage().instance().get(&CAP).expect("cap should be set");
        let current_total_supply = fungible::total_supply(e);
        if current_total_supply + amount > cap {
            panic_with_error!(e, MyCoinContractError::MaxSupplyExceeded);
        }
        fungible::mintable::mint(e, &account, amount);
    }
}
