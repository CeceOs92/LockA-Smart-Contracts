//! Persistence for [`AccessRequest`] values: the storage key scheme and the
//! helper functions used to read and write them.

use soroban_sdk::{contracttype, Address, Env, Vec};

use crate::RecordScope;

/// An access grant/request from a provider for a patient's records.
///
/// Rejection and revocation are both recorded as dedicated boolean fields
/// (`rejected`, `revoked`) on the request itself rather than by removing the
/// `access_id` from [`DataKey::PatientIndex`]. The patient index therefore
/// always lists every request ever made for a passport, active or not, which
/// keeps it usable as a full consent trail; callers that need only
/// currently-valid grants (e.g. [`crate::ConsentAccessManager::check_access`])
/// filter on these fields instead.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessRequest {
    pub access_id: u64,
    pub passport_id: Address,
    pub provider_id: Address,
    pub record_scope: RecordScope,
    /// The access window the provider asked for, in seconds. Recorded at
    /// request time so approval can derive `expires_at` from the duration the
    /// patient actually saw when consenting.
    pub duration_seconds: u64,
    pub approved: bool,
    /// Ledger timestamp at which the grant lapses. `0` until approval, since
    /// an unapproved request has no access window yet.
    pub expires_at: u64,
    pub revoked: bool,
    /// Set by `reject_access` when the patient declines a pending request.
    /// Mutually exclusive with `approved` in normal operation: the contract
    /// refuses to approve a rejected request and refuses to reject an
    /// approved one.
    pub rejected: bool,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone)]
pub(crate) enum DataKey {
    AccessRequest(u64),
    PatientIndex(Address),
    NextAccessId,
}

// Access requests must outlive normal temporary-storage windows. The network
// caps these values, so the SDK only extends a value when it is below the
// threshold and never beyond the network's configured maximum.
const TTL_THRESHOLD: u32 = 2_592_000;
const TTL_BUMP: u32 = 5_184_000;

fn bump_ttl(env: &Env, key: &DataKey) {
    env.storage()
        .persistent()
        .extend_ttl(key, TTL_THRESHOLD, TTL_BUMP);
}

/// Returns the next unused access-request identifier, advancing the counter.
///
/// Identifiers are allocated starting at 1.
pub(crate) fn next_access_id(env: &Env) -> u64 {
    let key = DataKey::NextAccessId;
    let storage = env.storage().persistent();

    let next = storage.get::<DataKey, u64>(&key).unwrap_or(0) + 1;
    storage.set(&key, &next);
    bump_ttl(env, &key);

    next
}

/// Persists `request`, indexing it under its patient passport for lookup.
pub(crate) fn write_access_request(env: &Env, request: &AccessRequest) {
    let storage = env.storage().persistent();

    let request_key = DataKey::AccessRequest(request.access_id);
    storage.set(&request_key, request);
    bump_ttl(env, &request_key);

    let index_key = DataKey::PatientIndex(request.passport_id.clone());
    let mut index = storage
        .get::<DataKey, Vec<u64>>(&index_key)
        .unwrap_or(Vec::new(env));
    if !index.contains(request.access_id) {
        index.push_back(request.access_id);
    }
    storage.set(&index_key, &index);
    bump_ttl(env, &index_key);
}

/// Returns the access request stored under `access_id`, if any.
pub(crate) fn read_access_request(env: &Env, access_id: u64) -> Option<AccessRequest> {
    let request_key = DataKey::AccessRequest(access_id);
    let storage = env.storage().persistent();

    let request = storage.get::<DataKey, AccessRequest>(&request_key);
    if request.is_some() {
        bump_ttl(env, &request_key);
    }
    request
}

/// Returns every `access_id` ever recorded for `passport_id`, in the order
/// the requests were created. Includes rejected, revoked, and expired
/// requests; callers that need only currently-valid grants must filter the
/// loaded [`AccessRequest`] values themselves.
pub(crate) fn read_patient_index(env: &Env, passport_id: &Address) -> Vec<u64> {
    let index_key = DataKey::PatientIndex(passport_id.clone());
    let storage = env.storage().persistent();

    let index = storage
        .get::<DataKey, Vec<u64>>(&index_key)
        .unwrap_or(Vec::new(env));
    if !index.is_empty() {
        bump_ttl(env, &index_key);
    }
    index
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ConsentAccessManager;
    use soroban_sdk::testutils::Address as _;

    fn sample_request(env: &Env, access_id: u64) -> AccessRequest {
        AccessRequest {
            access_id,
            passport_id: Address::generate(env),
            provider_id: Address::generate(env),
            record_scope: RecordScope::LabResultsOnly,
            duration_seconds: 3_600,
            approved: false,
            expires_at: 0,
            revoked: false,
            rejected: false,
            created_at: 0,
        }
    }

    #[test]
    fn writes_then_reads_an_access_request() {
        let env = Env::default();
        let contract_id = env.register(ConsentAccessManager, ());

        env.as_contract(&contract_id, || {
            let request = sample_request(&env, 1);
            write_access_request(&env, &request);

            assert_eq!(read_access_request(&env, 1), Some(request));
        });
    }

    #[test]
    fn read_access_request_returns_none_for_unknown_id() {
        let env = Env::default();
        let contract_id = env.register(ConsentAccessManager, ());

        env.as_contract(&contract_id, || {
            assert_eq!(read_access_request(&env, 42), None);
        });
    }

    #[test]
    fn next_access_id_increments_monotonically() {
        let env = Env::default();
        let contract_id = env.register(ConsentAccessManager, ());

        env.as_contract(&contract_id, || {
            assert_eq!(next_access_id(&env), 1);
            assert_eq!(next_access_id(&env), 2);
            assert_eq!(next_access_id(&env), 3);
        });
    }
}
