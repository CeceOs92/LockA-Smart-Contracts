use soroban_sdk::Env;

use crate::{ZkCredentialVerifier, ZkCredentialVerifierClient};

#[test]
fn contract_registers_and_builds_a_client() {
    let env = Env::default();
    let contract_id = env.register(ZkCredentialVerifier, ());
    let _client = ZkCredentialVerifierClient::new(&env, &contract_id);
}
