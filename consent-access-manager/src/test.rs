use soroban_sdk::Env;

use crate::{ConsentAccessManager, ConsentAccessManagerClient};

#[test]
fn contract_registers_and_builds_a_client() {
    let env = Env::default();
    let contract_id = env.register(ConsentAccessManager, ());
    let _client = ConsentAccessManagerClient::new(&env, &contract_id);
}
