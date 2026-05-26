#![cfg(test)]

use soroban_sdk::{token, Address, Env};

use crate::{FloodFundQR, FloodFundQRClient};

mod tests {
    use super::*;

    // Test 1: Happy path
    #[test]
    fn test_claim_success() {
        let env = Env::default();

        let admin = Address::generate(&env);
        let family = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_contract =
            env.register_stellar_asset_contract(token_admin.clone());

        let token_client =
            token::Client::new(&env, &token_contract.address());

        let contract_id = env.register(FloodFundQR, ());
        let client = FloodFundQRClient::new(&env, &contract_id);

        client.initialize(&admin, &token_contract.address());

        token_client.mint(&contract_id, &1000);

        client.approve_family(&admin, &family);

        client.claim_aid(&family, &100);

        assert_eq!(token_client.balance(&family), 100);
    }

    // Test 2: Edge case
    #[test]
    #[should_panic(expected = "family not approved")]
    fn test_unapproved_claim() {
        let env = Env::default();

        let admin = Address::generate(&env);
        let family = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_contract =
            env.register_stellar_asset_contract(token_admin.clone());

        let contract_id = env.register(FloodFundQR, ());
        let client = FloodFundQRClient::new(&env, &contract_id);

        client.initialize(&admin, &token_contract.address());

        client.claim_aid(&family, &100);
    }

    // Test 3: State verification
    #[test]
    fn test_claim_state_saved() {
        let env = Env::default();

        let admin = Address::generate(&env);
        let family = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_contract =
            env.register_stellar_asset_contract(token_admin.clone());

        let token_client =
            token::Client::new(&env, &token_contract.address());

        let contract_id = env.register(FloodFundQR, ());
        let client = FloodFundQRClient::new(&env, &contract_id);

        client.initialize(&admin, &token_contract.address());

        token_client.mint(&contract_id, &1000);

        client.approve_family(&admin, &family);

        client.claim_aid(&family, &100);

        assert_eq!(client.has_claimed(&family), true);
    }

    // Test 4: Double claim prevention
    #[test]
    #[should_panic(expected = "already claimed")]
    fn test_double_claim() {
        let env = Env::default();

        let admin = Address::generate(&env);
        let family = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_contract =
            env.register_stellar_asset_contract(token_admin.clone());

        let token_client =
            token::Client::new(&env, &token_contract.address());

        let contract_id = env.register(FloodFundQR, ());
        let client = FloodFundQRClient::new(&env, &contract_id);

        client.initialize(&admin, &token_contract.address());

        token_client.mint(&contract_id, &1000);

        client.approve_family(&admin, &family);

        client.claim_aid(&family, &100);

        client.claim_aid(&family, &100);
    }

    // Test 5: Approval verification
    #[test]
    fn test_approval_status() {
        let env = Env::default();

        let admin = Address::generate(&env);
        let family = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_contract =
            env.register_stellar_asset_contract(token_admin.clone());

        let contract_id = env.register(FloodFundQR, ());
        let client = FloodFundQRClient::new(&env, &contract_id);

        client.initialize(&admin, &token_contract.address());

        client.approve_family(&admin, &family);

        assert_eq!(client.is_approved(&family), true);
    }
}