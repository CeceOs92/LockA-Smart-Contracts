# Patient Passport Registry Contract

This contract creates and manages each patient’s LockA Medical Passport identity.

It does not store medical details. It stores the patient’s blockchain identity and passport reference.

**What it does**
Registers a patient.
Creates a unique Medical Passport ID.
Links the patient’s Stellar wallet address to their passport.
Supports account recovery or key rotation.
Tracks whether a passport is active, suspended, or revoked.

**Example data stored**
`passport_id`
`patient_wallet_address`
`public_identity_hash`
`created_at`
`status`
`recovery_address`

**Example functions**
`register_patient()`
`update_patient_key()`
`deactivate_passport()`
`get_passport()`