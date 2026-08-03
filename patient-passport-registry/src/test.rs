use soroban_sdk::Env;

use crate::{PatientPassportRegistry, PatientPassportRegistryClient};

#[test]
fn contract_registers_and_builds_a_client() {
    let env = Env::default();
    let contract_id = env.register(PatientPassportRegistry, ());
    let _client = PatientPassportRegistryClient::new(&env, &contract_id);
}
