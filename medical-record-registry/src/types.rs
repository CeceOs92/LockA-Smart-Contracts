use soroban_sdk::contracttype;

/// The record categories this contract can anchor.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecordType {
    /// A laboratory test result.
    LabResult,
    /// A prescription for medication.
    Prescription,
    /// A clinical diagnosis.
    Diagnosis,
    /// A vaccination record.
    Vaccination,
    /// A surgery report.
    SurgeryReport,
    /// An allergy record.
    AllergyRecord,
    /// An insurance-related record.
    InsuranceRecord,
    /// A general medical summary.
    MedicalSummary,
}

/// A medical record's lifecycle state.
///
/// Allowed transitions: `Active -> Amended`, `Active -> Revoked`, `Amended -> Revoked`.
/// `Revoked` is terminal — no transitions are allowed out of it.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecordStatus {
    /// The record is current and has not been amended or revoked.
    Active,
    /// The record has been superseded/corrected but remains on file.
    Amended,
    /// The record has been permanently revoked and is no longer valid.
    Revoked,
}

/// Returns whether transitioning a record's status from `from` to `to` is allowed.
pub fn is_valid_transition(from: &RecordStatus, to: &RecordStatus) -> bool {
    matches!(
        (from, to),
        (RecordStatus::Active, RecordStatus::Amended)
            | (RecordStatus::Active, RecordStatus::Revoked)
            | (RecordStatus::Amended, RecordStatus::Revoked)
    )
}

#[cfg(test)]
mod tests {
    use super::{is_valid_transition, RecordStatus, RecordType};
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn record_type_round_trips_through_storage() {
        let env = Env::default();
        let contract_id = env.register(crate::MedicalRecordRegistry, ());

        env.as_contract(&contract_id, || {
            let key = symbol_short!("rtype");
            env.storage().persistent().set(&key, &RecordType::LabResult);

            let loaded: RecordType = env.storage().persistent().get(&key).unwrap();
            assert_eq!(loaded, RecordType::LabResult);
        });
    }

    #[test]
    fn valid_transitions_are_accepted() {
        assert!(is_valid_transition(&RecordStatus::Active, &RecordStatus::Amended));
        assert!(is_valid_transition(&RecordStatus::Active, &RecordStatus::Revoked));
        assert!(is_valid_transition(&RecordStatus::Amended, &RecordStatus::Revoked));
    }

    #[test]
    fn invalid_transitions_are_rejected() {
        let statuses = [RecordStatus::Active, RecordStatus::Amended, RecordStatus::Revoked];
        let valid = [
            (RecordStatus::Active, RecordStatus::Amended),
            (RecordStatus::Active, RecordStatus::Revoked),
            (RecordStatus::Amended, RecordStatus::Revoked),
        ];

        for from in &statuses {
            for to in &statuses {
                let is_valid = valid
                    .iter()
                    .any(|(v_from, v_to)| v_from == from && v_to == to);
                assert_eq!(
                    is_valid_transition(from, to),
                    is_valid,
                    "transition {:?} -> {:?} did not match expected validity",
                    from,
                    to
                );
            }
        }
    }
}
