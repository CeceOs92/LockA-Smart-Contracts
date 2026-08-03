use soroban_sdk::Env;

use crate::{MedicalRecordRegistry, MedicalRecordRegistryClient};

#[test]
fn contract_registers_and_builds_a_client() {
    let env = Env::default();
    let contract_id = env.register(MedicalRecordRegistry, ());
    let _client = MedicalRecordRegistryClient::new(&env, &contract_id);
}
