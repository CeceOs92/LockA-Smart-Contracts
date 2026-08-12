use soroban_sdk::testutils::{
    Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger, MockAuth, MockAuthInvoke,
};
use soroban_sdk::{Address, Env, IntoVal, InvokeError, Symbol, Vec};

use crate::storage::{read_access_request, DataKey};
use crate::{ConsentAccessManager, ConsentAccessManagerClient, Error, RecordScope};

const ONE_DAY: u64 = 86_400;
const REQUESTED_AT: u64 = 1_700_000_000;

struct Fixture {
    env: Env,
    contract_id: Address,
    provider_id: Address,
    passport_id: Address,
}

impl Fixture {
    fn new() -> Self {
        let env = Env::default();
        env.ledger().set_timestamp(REQUESTED_AT);

        let contract_id = env.register(ConsentAccessManager, ());
        let provider_id = Address::generate(&env);
        let passport_id = Address::generate(&env);

        Fixture {
            env,
            contract_id,
            provider_id,
            passport_id,
        }
    }

    fn client(&self) -> ConsentAccessManagerClient<'_> {
        ConsentAccessManagerClient::new(&self.env, &self.contract_id)
    }

    fn stored_request(&self, access_id: u64) -> Option<crate::AccessRequest> {
        self.env.as_contract(&self.contract_id, || {
            read_access_request(&self.env, access_id)
        })
    }

    fn patient_index(&self) -> Vec<u64> {
        self.env.as_contract(&self.contract_id, || {
            self.env
                .storage()
                .persistent()
                .get::<DataKey, Vec<u64>>(&DataKey::PatientIndex(self.passport_id.clone()))
                .unwrap_or(Vec::new(&self.env))
        })
    }
}

#[test]
fn contract_registers_and_builds_a_client() {
    let env = Env::default();
    let contract_id = env.register(ConsentAccessManager, ());
    let _client = ConsentAccessManagerClient::new(&env, &contract_id);
}

#[test]
fn request_access_creates_a_pending_request() {
    let fixture = Fixture::new();
    fixture.env.mock_all_auths();

    let access_id = fixture.client().request_access(
        &fixture.provider_id,
        &fixture.passport_id,
        &RecordScope::LabResultsOnly,
        &ONE_DAY,
    );

    assert_eq!(access_id, 1);

    let request = fixture.stored_request(access_id).unwrap();
    assert_eq!(request.access_id, 1);
    assert_eq!(request.provider_id, fixture.provider_id);
    assert_eq!(request.passport_id, fixture.passport_id);
    assert_eq!(request.record_scope, RecordScope::LabResultsOnly);
    assert_eq!(request.duration_seconds, ONE_DAY);
    assert!(!request.approved);
    assert!(!request.revoked);
    assert_eq!(request.expires_at, 0, "expiry is unset until approval");
    assert_eq!(request.created_at, REQUESTED_AT);
}

#[test]
fn request_access_is_authorized_by_the_requesting_provider() {
    let fixture = Fixture::new();
    fixture.env.mock_all_auths();

    fixture.client().request_access(
        &fixture.provider_id,
        &fixture.passport_id,
        &RecordScope::AllRecords,
        &ONE_DAY,
    );

    assert_eq!(
        fixture.env.auths(),
        [(
            fixture.provider_id.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    fixture.contract_id.clone(),
                    Symbol::new(&fixture.env, "request_access"),
                    (
                        fixture.provider_id.clone(),
                        fixture.passport_id.clone(),
                        RecordScope::AllRecords,
                        ONE_DAY,
                    )
                        .into_val(&fixture.env),
                )),
                sub_invocations: [].into(),
            }
        )]
    );
}

#[test]
fn request_access_fails_without_the_providers_authorization() {
    let fixture = Fixture::new();
    let impostor = Address::generate(&fixture.env);

    // Authorization from anybody other than `provider_id` must not satisfy the
    // provider's `require_auth`.
    fixture.env.mock_auths(&[MockAuth {
        address: &impostor,
        invoke: &MockAuthInvoke {
            contract: &fixture.contract_id,
            fn_name: "request_access",
            args: (
                fixture.provider_id.clone(),
                fixture.passport_id.clone(),
                RecordScope::AllRecords,
                ONE_DAY,
            )
                .into_val(&fixture.env),
            sub_invokes: &[],
        },
    }]);

    let result = fixture.client().try_request_access(
        &fixture.provider_id,
        &fixture.passport_id,
        &RecordScope::AllRecords,
        &ONE_DAY,
    );

    assert_eq!(result, Err(Err(InvokeError::Abort)));
    assert!(
        fixture.stored_request(1).is_none(),
        "an unauthorized call must not persist a request"
    );
}

#[test]
fn request_access_rejects_a_zero_duration() {
    let fixture = Fixture::new();
    fixture.env.mock_all_auths();

    let result = fixture.client().try_request_access(
        &fixture.provider_id,
        &fixture.passport_id,
        &RecordScope::AllRecords,
        &0,
    );

    assert_eq!(result, Err(Ok(Error::InvalidDuration)));
    assert!(
        fixture.stored_request(1).is_none(),
        "a rejected request must not consume an access id"
    );
}

#[test]
fn request_access_allocates_sequential_ids_and_indexes_them_by_patient() {
    let fixture = Fixture::new();
    fixture.env.mock_all_auths();

    let client = fixture.client();
    let first = client.request_access(
        &fixture.provider_id,
        &fixture.passport_id,
        &RecordScope::LabResultsOnly,
        &ONE_DAY,
    );
    let second = client.request_access(
        &Address::generate(&fixture.env),
        &fixture.passport_id,
        &RecordScope::PrescriptionsOnly,
        &(7 * ONE_DAY),
    );

    assert_eq!((first, second), (1, 2));
    assert_eq!(
        fixture.patient_index(),
        soroban_sdk::vec![&fixture.env, 1, 2]
    );
}

#[test]
fn request_access_keeps_each_patients_index_separate() {
    let fixture = Fixture::new();
    fixture.env.mock_all_auths();

    let other_passport_id = Address::generate(&fixture.env);
    let client = fixture.client();
    client.request_access(
        &fixture.provider_id,
        &fixture.passport_id,
        &RecordScope::LabResultsOnly,
        &ONE_DAY,
    );
    client.request_access(
        &fixture.provider_id,
        &other_passport_id,
        &RecordScope::LabResultsOnly,
        &ONE_DAY,
    );

    assert_eq!(fixture.patient_index(), soroban_sdk::vec![&fixture.env, 1]);
}
