#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, Address, Env,
};

#[contract]
pub struct FloodFundQR;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Token,
    Approved(Address),
    Claimed(Address),
}

#[contractimpl]
impl FloodFundQR {

    // Initialize contract with admin and token
    pub fn initialize(env: Env, admin: Address, token: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    // Approve recipient family
    pub fn approve_family(env: Env, admin: Address, family: Address) {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        if admin != stored_admin {
            panic!("unauthorized");
        }

        env.storage()
            .persistent()
            .set(&DataKey::Approved(family), &true);
    }

    // Claim aid payout
    pub fn claim_aid(env: Env, family: Address, amount: i128) {
        family.require_auth();

        let approved = env
            .storage()
            .persistent()
            .get::<_, bool>(&DataKey::Approved(family.clone()))
            .unwrap_or(false);

        if !approved {
            panic!("family not approved");
        }

        let claimed = env
            .storage()
            .persistent()
            .get::<_, bool>(&DataKey::Claimed(family.clone()))
            .unwrap_or(false);

        if claimed {
            panic!("already claimed");
        }

        let token_address: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .unwrap();

        let client = token::Client::new(&env, &token_address);

        client.transfer(
            &env.current_contract_address(),
            &family,
            &amount,
        );

        env.storage()
            .persistent()
            .set(&DataKey::Claimed(family), &true);
    }

    // Check approval status
    pub fn is_approved(env: Env, family: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Approved(family))
            .unwrap_or(false)
    }

    // Check claim status
    pub fn has_claimed(env: Env, family: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Claimed(family))
            .unwrap_or(false)
    }
}