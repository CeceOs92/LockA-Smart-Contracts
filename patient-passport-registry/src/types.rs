use soroban_sdk::{contracttype, Address, BytesN};

/// A patient passport's lifecycle state.
///
/// Allowed transitions: `Active -> Suspended`, `Suspended -> Active`,
/// `Active -> Revoked`, `Suspended -> Revoked`. `Revoked` is terminal — no
/// transitions are allowed out of it.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PassportStatus {
    /// The passport is in good standing.
    Active,
    /// The passport has been temporarily suspended.
    Suspended,
    /// The passport has been permanently revoked.
    Revoked,
}

/// Returns whether transitioning a passport's status from `from` to `to` is allowed.
pub fn is_valid_transition(from: &PassportStatus, to: &PassportStatus) -> bool {
    matches!(
        (from, to),
        (PassportStatus::Active, PassportStatus::Suspended)
            | (PassportStatus::Suspended, PassportStatus::Active)
            | (PassportStatus::Active, PassportStatus::Revoked)
            | (PassportStatus::Suspended, PassportStatus::Revoked)
    )
}

/// A patient's LockA Medical Passport identity.
#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Passport {
    /// Unique identifier for the passport.
    pub passport_id: u64,
    /// The Stellar wallet address currently linked to this passport.
    pub patient_wallet_address: Address,
    /// A hash committing to the patient's public identity claims.
    pub public_identity_hash: BytesN<32>,
    /// Ledger timestamp at which the passport was created.
    pub created_at: u64,
    /// The passport's current lifecycle state.
    pub status: PassportStatus,
    /// An optional address authorized to help recover/rotate the passport's wallet key.
    pub recovery_address: Option<Address>,
}

#[cfg(test)]
mod tests {
    use super::{is_valid_transition, PassportStatus};

    #[test]
    fn valid_transitions_are_accepted() {
        assert!(is_valid_transition(&PassportStatus::Active, &PassportStatus::Suspended));
        assert!(is_valid_transition(&PassportStatus::Suspended, &PassportStatus::Active));
        assert!(is_valid_transition(&PassportStatus::Active, &PassportStatus::Revoked));
        assert!(is_valid_transition(&PassportStatus::Suspended, &PassportStatus::Revoked));
    }

    #[test]
    fn invalid_transitions_are_rejected() {
        let statuses = [
            PassportStatus::Active,
            PassportStatus::Suspended,
            PassportStatus::Revoked,
        ];
        let valid = [
            (PassportStatus::Active, PassportStatus::Suspended),
            (PassportStatus::Suspended, PassportStatus::Active),
            (PassportStatus::Active, PassportStatus::Revoked),
            (PassportStatus::Suspended, PassportStatus::Revoked),
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
