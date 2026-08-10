use soroban_sdk::{contracttype, Address, BytesN};

/// The classes of medical devices and wearables the registry can approve.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeviceCategory {
    /// A cuff-based or wrist-based blood pressure monitor.
    BloodPressureMonitor,
    /// A blood glucose meter or continuous glucose monitor.
    GlucoseMonitor,
    /// A digital or smart thermometer.
    Thermometer,
    /// A pulse oximeter measuring blood oxygen saturation.
    PulseOximeter,
    /// A temperature sensor used to monitor cold-chain storage (e.g. vaccines).
    ColdChainSensor,
    /// Clinic or hospital equipment reporting maintenance/calibration data.
    ClinicEquipment,
    /// A general-purpose wearable (e.g. activity, heart rate, sleep tracker).
    Wearable,
    /// Any device category not covered by the variants above.
    Other,
}

/// An approved medical device or IoT/wearable data source registered on-chain.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    /// Unique identifier for the device.
    pub device_id: BytesN<32>,
    /// The patient this device is linked to.
    pub owner: Address,
    /// The provider/organization that registered and vouches for the device.
    pub issuer: Address,
    /// The device's category.
    pub category: DeviceCategory,
    /// Public key used to verify signed readings submitted by this device.
    pub public_key: BytesN<32>,
    /// Whether the device is currently active and allowed to submit attestations.
    pub active: bool,
    /// Ledger timestamp at which the device was registered.
    pub registered_at: u64,
}

#[cfg(test)]
mod tests {
    use super::DeviceCategory;
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn device_category_round_trips_through_storage() {
        let env = Env::default();
        let contract_id = env.register(crate::DeviceDataAttestation, ());

        env.as_contract(&contract_id, || {
            let key = symbol_short!("category");
            env.storage()
                .persistent()
                .set(&key, &DeviceCategory::PulseOximeter);

            let loaded: DeviceCategory = env.storage().persistent().get(&key).unwrap();
            assert_eq!(loaded, DeviceCategory::PulseOximeter);
        });
    }
}
