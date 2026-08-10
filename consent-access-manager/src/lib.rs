#![no_std]

//! Consent & Access Manager: controls which providers may access which
//! categories of a patient's medical records, for how long, and under what
//! conditions.

use soroban_sdk::{contract, contracttype};

/// The granular record categories a patient can grant a provider access to.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordScope {
    /// Grants access to the patient's entire medical record.
    AllRecords = 0,
    /// Grants access to laboratory results only.
    LabResultsOnly = 1,
    /// Grants access to prescription records only.
    PrescriptionsOnly = 2,
    /// Grants access to vaccination records only.
    VaccinationRecordsOnly = 3,
    /// Grants access to the emergency summary only.
    EmergencySummaryOnly = 4,
    /// Grants access to insurance data only.
    InsuranceDataOnly = 5,
}

/// Consent & access manager contract.
#[contract]
pub struct ConsentAccessManager;

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn record_scope_round_trips_through_storage() {
        let env = Env::default();
        let contract_id = env.register(ConsentAccessManager, ());

        env.as_contract(&contract_id, || {
            let key = symbol_short!("SCOPE");
            let storage = env.storage().persistent();

            storage.set(&key, &RecordScope::LabResultsOnly);
            let scope: RecordScope = storage.get(&key).unwrap();

            assert_eq!(scope, RecordScope::LabResultsOnly);
        });
    }
}
