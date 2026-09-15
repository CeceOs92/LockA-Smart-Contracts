// Not yet called from the contract's public methods; exercised via the tests below
// until PatientPassportRegistry's read/write flows are wired up.
#![allow(dead_code)]

use soroban_sdk::{contracttype, Address, Env};

use crate::types::Passport;

/// Storage key scheme used by this contract.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// A single registered passport, keyed by its `passport_id`.
    Passport(u64),
    /// Reverse lookup from a wallet address to the `passport_id` it controls.
    WalletIndex(Address),
    /// Monotonic counter used to allocate the next `passport_id`.
    NextPassportId,
}

/// Reads a registered passport by its `passport_id`, if it exists.
pub fn read_passport(env: &Env, passport_id: u64) -> Option<Passport> {
    env.storage()
        .persistent()
        .get(&DataKey::Passport(passport_id))
}

/// Persists (or overwrites) a passport record under its `passport_id`.
pub fn write_passport(env: &Env, passport: &Passport) {
    env.storage()
        .persistent()
        .set(&DataKey::Passport(passport.passport_id), passport);
}

/// Allocates and returns the next unused `passport_id`, starting at `1`.
pub fn next_passport_id(env: &Env) -> u64 {
    let next = env
        .storage()
        .instance()
        .get::<_, u64>(&DataKey::NextPassportId)
        .unwrap_or(0)
        + 1;
    env.storage()
        .instance()
        .set(&DataKey::NextPassportId, &next);
    next
}

/// Resolves a wallet address to its associated `passport_id`, if one exists.
pub fn read_passport_id_by_wallet(env: &Env, wallet: &Address) -> Option<u64> {
    env.storage()
        .persistent()
        .get(&DataKey::WalletIndex(wallet.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PassportStatus;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::BytesN;

    fn sample_passport(env: &Env, passport_id: u64, wallet: &Address) -> Passport {
        Passport {
            passport_id,
            patient_wallet_address: wallet.clone(),
            public_identity_hash: BytesN::from_array(env, &[3u8; 32]),
            created_at: 42,
            status: PassportStatus::Active,
            recovery_address: None,
        }
    }

    #[test]
    fn write_then_read_passport() {
        let env = Env::default();
        let contract_id = env.register(crate::PatientPassportRegistry, ());

        env.as_contract(&contract_id, || {
            let wallet = Address::generate(&env);
            let passport = sample_passport(&env, 1, &wallet);
            write_passport(&env, &passport);

            let loaded = read_passport(&env, passport.passport_id).unwrap();
            assert_eq!(loaded, passport);
        });
    }

    #[test]
    fn missing_passport_lookup_returns_none() {
        let env = Env::default();
        let contract_id = env.register(crate::PatientPassportRegistry, ());

        env.as_contract(&contract_id, || {
            assert!(read_passport(&env, 999).is_none());
        });
    }

    #[test]
    fn next_passport_id_increments() {
        let env = Env::default();
        let contract_id = env.register(crate::PatientPassportRegistry, ());

        env.as_contract(&contract_id, || {
            assert_eq!(next_passport_id(&env), 1);
            assert_eq!(next_passport_id(&env), 2);
            assert_eq!(next_passport_id(&env), 3);
        });
    }

    #[test]
    fn read_passport_id_by_wallet_resolves_indexed_wallet() {
        let env = Env::default();
        let contract_id = env.register(crate::PatientPassportRegistry, ());

        env.as_contract(&contract_id, || {
            let wallet = Address::generate(&env);
            env.storage()
                .persistent()
                .set(&DataKey::WalletIndex(wallet.clone()), &7u64);

            assert_eq!(read_passport_id_by_wallet(&env, &wallet), Some(7));
        });
    }

    #[test]
    fn read_passport_id_by_wallet_returns_none_for_unindexed_wallet() {
        let env = Env::default();
        let contract_id = env.register(crate::PatientPassportRegistry, ());

        env.as_contract(&contract_id, || {
            let wallet = Address::generate(&env);
            assert!(read_passport_id_by_wallet(&env, &wallet).is_none());
        });
    }
}
