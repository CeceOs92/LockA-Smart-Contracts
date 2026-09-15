use soroban_sdk::contracttype;

/// The granular record categories a patient can grant a provider access to.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecordScope {
    /// Grants access to every record category held for the patient.
    AllRecords,
    /// Grants access only to laboratory results.
    LabResultsOnly,
    /// Grants access only to prescriptions.
    PrescriptionsOnly,
    /// Grants access only to vaccination records.
    VaccinationRecordsOnly,
    /// Grants access only to the patient's emergency summary (e.g. blood type, allergies).
    EmergencySummaryOnly,
    /// Grants access only to insurance-related data.
    InsuranceDataOnly,
}

#[cfg(test)]
mod tests {
    use super::RecordScope;
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn record_scope_round_trips_through_storage() {
        let env = Env::default();
        let contract_id = env.register(crate::ConsentAccessManager, ());

        env.as_contract(&contract_id, || {
            let key = symbol_short!("scope");
            env.storage()
                .persistent()
                .set(&key, &RecordScope::LabResultsOnly);

            let loaded: RecordScope = env.storage().persistent().get(&key).unwrap();
            assert_eq!(loaded, RecordScope::LabResultsOnly);
        });
    }

    #[test]
    fn record_scope_variants_are_distinct() {
        assert_ne!(RecordScope::AllRecords, RecordScope::LabResultsOnly);
        assert_ne!(
            RecordScope::PrescriptionsOnly,
            RecordScope::VaccinationRecordsOnly
        );
        assert_ne!(
            RecordScope::EmergencySummaryOnly,
            RecordScope::InsuranceDataOnly
        );
    }
}
