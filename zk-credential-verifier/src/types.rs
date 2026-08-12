use soroban_sdk::contracttype;

/// The cryptographic outcome of a raw proof submission.
///
/// Set once by `verify_proof_attestation` and never changed afterward — a
/// submission's cryptographic result is immutable history. This is distinct
/// from [`ClaimStatus`], which tracks the administrative state of the
/// durable claim a verified submission produces.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubmissionStatus {
    /// The submission has been recorded but not yet verified.
    Pending,
    /// The proof was checked against its verifying key and holds.
    Verified,
    /// The proof was checked against its verifying key and does not hold.
    Failed,
}

/// The administrative state of a durable, queryable verified claim.
///
/// Expiry is handled separately via a stored `expires_at` timestamp checked
/// at read time, not as its own variant here.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClaimStatus {
    /// The claim is in good standing.
    Active,
    /// The claim has been revoked by the patient or an authorized party.
    Revoked,
}

#[cfg(test)]
mod tests {
    use super::{ClaimStatus, SubmissionStatus};
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn submission_status_round_trips_through_storage() {
        let env = Env::default();
        let contract_id = env.register(crate::ZkCredentialVerifier, ());

        env.as_contract(&contract_id, || {
            let key = symbol_short!("sub_st");
            env.storage()
                .persistent()
                .set(&key, &SubmissionStatus::Verified);

            let loaded: SubmissionStatus = env.storage().persistent().get(&key).unwrap();
            assert_eq!(loaded, SubmissionStatus::Verified);
        });
    }

    #[test]
    fn submission_status_variants_are_distinct() {
        assert_ne!(SubmissionStatus::Pending, SubmissionStatus::Verified);
        assert_ne!(SubmissionStatus::Verified, SubmissionStatus::Failed);
        assert_ne!(SubmissionStatus::Pending, SubmissionStatus::Failed);
    }

    #[test]
    fn claim_status_round_trips_through_storage() {
        let env = Env::default();
        let contract_id = env.register(crate::ZkCredentialVerifier, ());

        env.as_contract(&contract_id, || {
            let key = symbol_short!("claim_st");
            env.storage().persistent().set(&key, &ClaimStatus::Revoked);

            let loaded: ClaimStatus = env.storage().persistent().get(&key).unwrap();
            assert_eq!(loaded, ClaimStatus::Revoked);
        });
    }

    #[test]
    fn claim_status_variants_are_distinct() {
        assert_ne!(ClaimStatus::Active, ClaimStatus::Revoked);
    }
}
