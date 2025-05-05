#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, log, symbol_short, Symbol};

// Health data structure
#[contracttype]
#[derive(Clone)]
pub struct HealthData {
    pub age: u32,
    pub height_cm: u32,
    pub weight_kg: u32,
    pub notes: String,
}

// Storage key
#[contracttype]
pub enum DataKey {
    Record(Address),
}

#[contract]
pub struct SimpleHealthTracker;

#[contractimpl]
impl SimpleHealthTracker {
    // Add or update health data
    pub fn upsert_health(env: Env, user: Address, age: u32, height_cm: u32, weight_kg: u32, notes: String) {
        user.require_auth();
        let data = HealthData {
            age,
            height_cm,
            weight_kg,
            notes,
        };
        env.storage().instance().set(&DataKey::Record(user.clone()), &data);
        log!(&env, "Health data updated for {}", user);
    }

    // Get health data for user
    pub fn get_health(env: Env, user: Address) -> Option<HealthData> {
        env.storage().instance().get(&DataKey::Record(user))
    }

    // Remove health record (for privacy)
    pub fn delete_health(env: Env, user: Address) {
        user.require_auth();
        env.storage().instance().remove(&DataKey::Record(user.clone()));
        log!(&env, "Health data removed for {}", user);
    }
}
