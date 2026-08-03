use soroban_sdk::{contracttype, Address, BytesN, Env};

use crate::types::Device;

/// Storage key scheme used by this contract.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// A single registered device, keyed by its `device_id`.
    Device(BytesN<32>),
    /// The index of device IDs belonging to a given patient.
    PatientDevices(Address),
}

/// Reads a registered device by its `device_id`, if it exists.
pub fn read_device(env: &Env, device_id: &BytesN<32>) -> Option<Device> {
    env.storage()
        .persistent()
        .get(&DataKey::Device(device_id.clone()))
}

/// Persists (or overwrites) a device record under its `device_id`.
pub fn write_device(env: &Env, device: &Device) {
    env.storage()
        .persistent()
        .set(&DataKey::Device(device.device_id.clone()), device);
}

/// Returns whether a device with the given `device_id` is already registered.
pub fn device_exists(env: &Env, device_id: &BytesN<32>) -> bool {
    env.storage().persistent().has(&DataKey::Device(device_id.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DeviceCategory;
    use soroban_sdk::testutils::Address as _;

    fn sample_device(env: &Env) -> Device {
        Device {
            device_id: BytesN::from_array(env, &[7u8; 32]),
            owner: Address::generate(env),
            issuer: Address::generate(env),
            category: DeviceCategory::GlucoseMonitor,
            public_key: BytesN::from_array(env, &[9u8; 32]),
            active: true,
            registered_at: 12_345,
        }
    }

    #[test]
    fn write_then_read_device() {
        let env = Env::default();
        let contract_id = env.register(crate::DeviceDataAttestation, ());

        env.as_contract(&contract_id, || {
            let device = sample_device(&env);
            write_device(&env, &device);

            let loaded = read_device(&env, &device.device_id).unwrap();
            assert_eq!(loaded, device);
            assert!(device_exists(&env, &device.device_id));
        });
    }

    #[test]
    fn missing_device_lookup_returns_none() {
        let env = Env::default();
        let contract_id = env.register(crate::DeviceDataAttestation, ());

        env.as_contract(&contract_id, || {
            let unknown_id = BytesN::from_array(&env, &[0u8; 32]);
            assert!(read_device(&env, &unknown_id).is_none());
            assert!(!device_exists(&env, &unknown_id));
        });
    }
}
