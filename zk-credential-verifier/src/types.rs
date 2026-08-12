use soroban_sdk::contracttype;

/// The zero-knowledge credential categories this contract can verify.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CredentialType {
    /// Proves that a patient meets a vaccination requirement without exposing full history.
    VaccinationStatus,
    /// Proves insurance eligibility without exposing policy details.
    InsuranceEligibility,
    /// Proves the patient is above a required age threshold.
    AgeThreshold,
    /// Proves eligibility for a health program or benefit.
    ProgramEligibility,
    /// Proves completion of a medical screening requirement.
    MedicalScreeningCompletion,
    /// Proves that a LockA Medical Passport is currently valid.
    PassportValidity,
    /// Proves ownership of a health credential without revealing the credential contents.
    CredentialOwnership,
}

#[cfg(test)]
mod tests {
    use super::CredentialType;
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn credential_type_round_trips_through_storage() {
        let env = Env::default();
        let contract_id = env.register(crate::ZkCredentialVerifier, ());

        env.as_contract(&contract_id, || {
            let key = symbol_short!("ctype");
            env.storage()
                .persistent()
                .set(&key, &CredentialType::ProgramEligibility);

            let loaded: CredentialType = env.storage().persistent().get(&key).unwrap();
            assert_eq!(loaded, CredentialType::ProgramEligibility);
        });
    }
}
