use soroban_sdk::Env;

use crate::{DeviceDataAttestation, DeviceDataAttestationClient};

#[test]
fn contract_registers_and_builds_a_client() {
    let env = Env::default();
    let contract_id = env.register(DeviceDataAttestation, ());
    let _client = DeviceDataAttestationClient::new(&env, &contract_id);
}
