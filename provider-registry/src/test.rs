use soroban_sdk::Env;

use crate::{ProviderRegistry, ProviderRegistryClient};

#[test]
fn contract_registers_and_builds_a_client() {
    let env = Env::default();
    let contract_id = env.register(ProviderRegistry, ());
    let _client = ProviderRegistryClient::new(&env, &contract_id);
}
