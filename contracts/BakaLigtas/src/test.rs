#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

fn setup_test_env(env: &Env) -> (Address, Address, Address, BakaLigtasContractClient, token::Client<'static>) {
    let admin = Address::generate(env);
    let farmer = Address::generate(env);
    
    // Deploy a mock Stellar asset contract representing USDC
    let token_admin = Address::generate(env);
    let token_contract_id = env.register_stellar_asset_contract(token_admin.clone());
    let token_client = token::Client::new(env, &token_contract_id);

    // Deploy our custom Soroban protocol instance
    let contract_id = env.register_contract(None, BakaLigtasContract);
    let client = BakaLigtasContractClient::new(env, &contract_id);

    // Pre-fund the contract account to simulate government reserve allocations
    token_client.mint(&contract_id, &100000);

    (admin, farmer, token_contract_id, client, token_client)
}

#[test]
fn test_happy_path_disbursement() {
    let env = Env::default();
    env.mock_all_auths();
    let (admin, farmer, token_id, client, token_client) = setup_test_env(&env);

    client.initialize(&admin, &token_id);
    client.register_farmer(&admin, &farmer);
    
    // Execute a standard distribution payout
    client.release_subsidy(&admin, &farmer, &15000);

    // Assert funds reached target
    assert_eq!(token_client.balance(&farmer), 15000);
    assert_eq!(token_client.balance(&client.address), 85000);
}

#[test]
#[should_panic(expected = "Farmer wallet address not found in registry")]
fn test_unregistered_farmer_failure() {
    let env = Env::default();
    env.mock_all_auths();
    let (admin, farmer, token_id, client, _) = setup_test_env(&env);

    client.initialize(&admin, &token_id);
    // Explicit bypass: skip client.register_farmer verification step
    
    client.release_subsidy(&admin, &farmer, &15000);
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    env.mock_all_auths();
    let (admin, farmer, token_id, client, _) = setup_test_env(&env);

    client.initialize(&admin, &token_id);
    client.register_farmer(&admin, &farmer);

    // Interrogate storage tree to confirm state survival
    let is_active = env.as_contract(&client.address, || {
        env.storage().instance().get(&DataKey::Farmers(farmer.clone())).unwrap_or(false)
    });
    assert!(is_active);
}

#[test]
#[should_panic(expected = "Caller is not the authorized official")]
fn test_unauthorized_registration_rejection() {
    let env = Env::default();
    env.mock_all_auths();
    let (admin, farmer, token_id, client, _) = setup_test_env(&env);

    client.initialize(&admin, &token_id);
    
    let unauthorized_actor = Address::generate(&env);
    client.register_farmer(&unauthorized_actor, &farmer);
}

#[test]
#[should_panic(expected = "Contract instance already initialized")]
fn test_prevent_double_initialization() {
    let env = Env::default();
    env.mock_all_auths();
    let (admin, _, token_id, client, _) = setup_test_env(&env);

    client.initialize(&admin, &token_id);
    // Attempting to overwrite keys throws an explicit error state exception
    client.initialize(&admin, &token_id);
}