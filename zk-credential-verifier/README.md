# ZK Credential Verifier

This contract handles Zero-Knowledge Proof-based identity and access control.

**What it does**

It allows a patient to prove something without revealing the full underlying data.

**For example, a patient can prove:**

They are vaccinated.
They are insured.
They are above a required age.
They are eligible for a health program.
They completed a required medical screening.
They have a valid LockA Passport.
They are the rightful owner of a health credential.

without exposing the full medical record.

**Example ZK use cases**
Prove vaccination status without revealing full vaccination history.
Prove insurance eligibility without revealing policy details.
Prove patient identity without exposing all personal information.
Prove access authorization without revealing unrelated records.

**Example data stored**
* `proof_id`
* `passport_id`
* `credential_type`
* `proof_hash`
* `verifier`
* `verified_result`
* `expires_at`
* `Example functions`
* `submit_proof()`
* `verify_proof_attestation()`
* `store_verified_claim()`
* `revoke_claim()`
* `get_verified_claim()`