#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,             // Municipal Agriculture Officer (MAO)
    Token,             // Stellar USDC token address
    Farmers(Address),  // Register mapping farmer to eligibility state (bool)
}

#[contract]
pub struct BakaLigtasContract;

#[contractimpl]
impl BakaLigtasContract {
    // Establishes the governing authority and the transactional asset profile
    pub fn initialize(env: Env, admin: Address, token: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract instance already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    // Whitelists a specific registered local farmer based on land classification records
    pub fn register_farmer(env: Env, admin: Address, farmer: Address) {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("Caller is not the authorized official");
        }
        env.storage().instance().set(&DataKey::Farmers(farmer), &true);
    }

    // Distributes a flat emergency micro-subsidy from the pool asset reserves
    pub fn release_subsidy(env: Env, admin: Address, farmer: Address, amount: i128) {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            panic!("Caller is not the authorized official");
        }

        // Evaluate registration state to protect public funds from exploitation
        let is_registered: bool = env
            .storage()
            .instance()
            .get(&DataKey::Farmers(farmer.clone()))
            .unwrap_or(false);
            
        if !is_registered {
            panic!("Farmer wallet address not found in registry");
        }

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let client = token::Client::new(&env, &token_addr);
        
        // Execute the settlement from the contract's reserve to the farmer
        client.transfer(&env.current_contract_address(), &farmer, &amount);

        // Emit standard event tracking for local audit compliance
        env.events().publish(
            (Symbol::new(&env, "subsidy_paid"), farmer),
            amount,
        );
    }
}