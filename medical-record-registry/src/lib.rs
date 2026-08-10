#![no_std]

//! A tamper-evident registry for encrypted medical-record references.
//!
//! Medical files and their storage locations are never written on-chain. This
//! contract stores fixed-size cryptographic hashes of those values instead.

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, Address, BytesN, Env, Symbol, Vec,
};

/// The lifecycle state of a registered medical record.
#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordStatus {
    Active = 0,
    Revoked = 1,
    Archived = 2,
}

/// Immutable record provenance plus its current lifecycle status.
///
/// `encrypted_file_hash` and `storage_pointer_hash` must be 32-byte hashes,
/// not plaintext data or a raw off-chain storage URL.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordMetadata {
    pub record_id: BytesN<32>,
    pub passport_id: BytesN<32>,
    pub provider_id: Address,
    pub record_type: Symbol,
    pub encrypted_file_hash: BytesN<32>,
    pub storage_pointer_hash: BytesN<32>,
    pub issued_at: u64,
    pub status: RecordStatus,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Record(BytesN<32>),
    PatientRecords(BytesN<32>),
    ProviderRecords(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum RegistryError {
    RecordAlreadyExists = 1,
    RecordNotFound = 2,
}

/// Medical-record registry contract.
#[contract]
pub struct MedicalRecordRegistry;

#[contractimpl]
impl MedicalRecordRegistry {
    /// Registers a new encrypted-record reference.
    ///
    /// The issuing provider must authorize this invocation. `record_id` is
    /// immutable: attempting to reuse it returns `RecordAlreadyExists`.
    #[allow(clippy::too_many_arguments)]
    pub fn add_record(
        env: Env,
        record_id: BytesN<32>,
        passport_id: BytesN<32>,
        provider_id: Address,
        record_type: Symbol,
        encrypted_file_hash: BytesN<32>,
        storage_pointer_hash: BytesN<32>,
    ) -> Result<(), RegistryError> {
        provider_id.require_auth();

        let record_key = DataKey::Record(record_id.clone());
        let storage = env.storage().persistent();
        if storage.has(&record_key) {
            return Err(RegistryError::RecordAlreadyExists);
        }

        let record = RecordMetadata {
            record_id: record_id.clone(),
            passport_id: passport_id.clone(),
            provider_id: provider_id.clone(),
            record_type,
            encrypted_file_hash,
            storage_pointer_hash,
            issued_at: env.ledger().timestamp(),
            status: RecordStatus::Active,
        };
        storage.set(&record_key, &record);
        bump_ttl(&env, &record_key);

        let patient_key = DataKey::PatientRecords(passport_id);
        let mut records = storage
            .get::<DataKey, Vec<BytesN<32>>>(&patient_key)
            .unwrap_or(Vec::new(&env));
        records.push_back(record_id.clone());
        storage.set(&patient_key, &records);
        bump_ttl(&env, &patient_key);

        let provider_key = DataKey::ProviderRecords(provider_id);
        let mut records = storage
            .get::<DataKey, Vec<BytesN<32>>>(&provider_key)
            .unwrap_or(Vec::new(&env));
        records.push_back(record_id);
        storage.set(&provider_key, &records);
        bump_ttl(&env, &provider_key);

        Ok(())
    }

    /// Changes the lifecycle state of a record.
    ///
    /// Only the provider that originally registered the record can update its
    /// status. The record hashes and provenance fields remain immutable.
    pub fn update_record_status(
        env: Env,
        record_id: BytesN<32>,
        status: RecordStatus,
    ) -> Result<(), RegistryError> {
        let record_key = DataKey::Record(record_id);
        let storage = env.storage().persistent();
        let mut record = storage
            .get::<DataKey, RecordMetadata>(&record_key)
            .ok_or(RegistryError::RecordNotFound)?;

        record.provider_id.require_auth();
        record.status = status;
        storage.set(&record_key, &record);
        bump_ttl(&env, &record_key);

        Ok(())
    }

    /// Returns whether a supplied encrypted-file hash matches the registered
    /// hash for `record_id`. A missing record returns `false`.
    pub fn verify_record_hash(
        env: Env,
        record_id: BytesN<32>,
        encrypted_file_hash: BytesN<32>,
    ) -> bool {
        let record_key = DataKey::Record(record_id);
        let storage = env.storage().persistent();
        match storage.get::<DataKey, RecordMetadata>(&record_key) {
            Some(record) => {
                bump_ttl(&env, &record_key);
                record.encrypted_file_hash == encrypted_file_hash
            }
            None => false,
        }
    }

    /// Returns the metadata for a record, without any medical-file contents.
    pub fn get_record_metadata(env: Env, record_id: BytesN<32>) -> Option<RecordMetadata> {
        let record_key = DataKey::Record(record_id);
        let storage = env.storage().persistent();
        let record = storage.get::<DataKey, RecordMetadata>(&record_key);
        if record.is_some() {
            bump_ttl(&env, &record_key);
        }
        record
    }

    /// Returns the record identifiers registered to a patient passport.
    ///
    /// Call `get_record_metadata` for each identifier to obtain its metadata.
    pub fn get_patient_records(env: Env, passport_id: BytesN<32>) -> Vec<BytesN<32>> {
        let patient_key = DataKey::PatientRecords(passport_id);
        let storage = env.storage().persistent();
        let records = storage
            .get::<DataKey, Vec<BytesN<32>>>(&patient_key)
            .unwrap_or(Vec::new(&env));
        if !records.is_empty() {
            bump_ttl(&env, &patient_key);
        }
        records
    }

    /// Returns the full metadata set for records issued by a provider.
    ///
    /// This is a read-only audit view. It does not require provider auth
    /// because it returns only on-chain record metadata, not medical contents.
    pub fn get_provider_records(env: Env, provider_id: Address) -> Vec<RecordMetadata> {
        let provider_key = DataKey::ProviderRecords(provider_id);
        let storage = env.storage().persistent();
        let record_ids = storage
            .get::<DataKey, Vec<BytesN<32>>>(&provider_key)
            .unwrap_or(Vec::new(&env));
        if !record_ids.is_empty() {
            bump_ttl(&env, &provider_key);
        }

        let mut records = Vec::new(&env);
        for record_id in record_ids {
            let record_key = DataKey::Record(record_id);
            if let Some(record) = storage.get::<DataKey, RecordMetadata>(&record_key) {
                bump_ttl(&env, &record_key);
                records.push_back(record);
            }
        }
        records
    }
}

// Medical records must outlive normal temporary-storage windows. The network
// caps these values, so the SDK only extends a value when it is below the
// threshold and never beyond the network's configured maximum.
const TTL_THRESHOLD: u32 = 2_592_000;
const TTL_BUMP: u32 = 5_184_000;

fn bump_ttl(env: &Env, key: &DataKey) {
    env.storage()
        .persistent()
        .extend_ttl(key, TTL_THRESHOLD, TTL_BUMP);
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, testutils::Address as _, Env};

    fn hash(env: &Env, value: u8) -> BytesN<32> {
        BytesN::from_array(env, &[value; 32])
    }

    fn add_record(
        client: &MedicalRecordRegistryClient,
        env: &Env,
        record_id_value: u8,
        passport_id_value: u8,
        provider_id: &Address,
        record_type: Symbol,
    ) -> BytesN<32> {
        let record_id = hash(env, record_id_value);
        client.add_record(
            &record_id,
            &hash(env, passport_id_value),
            provider_id,
            &record_type,
            &hash(env, record_id_value + 10),
            &hash(env, record_id_value + 20),
        );
        record_id
    }

    #[test]
    fn registers_verifies_indexes_and_updates_a_record() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(MedicalRecordRegistry, ());
        let client = MedicalRecordRegistryClient::new(&env, &contract_id);

        let record_id = hash(&env, 1);
        let passport_id = hash(&env, 2);
        let encrypted_file_hash = hash(&env, 3);
        let provider_id = Address::generate(&env);

        client.add_record(
            &record_id,
            &passport_id,
            &provider_id,
            &symbol_short!("LAB"),
            &encrypted_file_hash,
            &hash(&env, 4),
        );

        assert!(client.verify_record_hash(&record_id, &encrypted_file_hash));
        assert!(!client.verify_record_hash(&record_id, &hash(&env, 5)));
        assert_eq!(
            client.get_patient_records(&passport_id),
            soroban_sdk::vec![&env, record_id.clone()]
        );

        client.update_record_status(&record_id, &RecordStatus::Archived);
        assert_eq!(
            client.get_record_metadata(&record_id).unwrap().status,
            RecordStatus::Archived
        );
    }

    #[test]
    fn returns_empty_provider_records_for_provider_without_issued_records() {
        let env = Env::default();

        let contract_id = env.register(MedicalRecordRegistry, ());
        let client = MedicalRecordRegistryClient::new(&env, &contract_id);

        let provider_id = Address::generate(&env);

        assert!(client.get_provider_records(&provider_id).is_empty());
    }

    #[test]
    fn returns_provider_records_across_patients_and_record_types() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(MedicalRecordRegistry, ());
        let client = MedicalRecordRegistryClient::new(&env, &contract_id);

        let provider_id = Address::generate(&env);
        let other_provider_id = Address::generate(&env);
        let lab_record_id = add_record(&client, &env, 11, 1, &provider_id, symbol_short!("LAB"));
        let rx_record_id = add_record(&client, &env, 12, 2, &provider_id, symbol_short!("RX"));
        add_record(
            &client,
            &env,
            13,
            3,
            &other_provider_id,
            symbol_short!("IMG"),
        );

        let records = client.get_provider_records(&provider_id);

        assert_eq!(records.len(), 2);
        assert_eq!(records.get(0).unwrap().record_id, lab_record_id);
        assert_eq!(records.get(0).unwrap().passport_id, hash(&env, 1));
        assert_eq!(records.get(0).unwrap().record_type, symbol_short!("LAB"));
        assert_eq!(records.get(1).unwrap().record_id, rx_record_id);
        assert_eq!(records.get(1).unwrap().passport_id, hash(&env, 2));
        assert_eq!(records.get(1).unwrap().record_type, symbol_short!("RX"));
    }
}
