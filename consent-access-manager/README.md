# Consent & Access Manager

It controls who can access what, for how long, and under what condition.

**What it does**
* Allows providers to request access to patient records.
* Allows patients to approve or reject requests.
* Supports limited access by record type.
* Supports expiry-based access.
* Allows patients to revoke access anytime.
* Creates an on-chain consent trail.

**Example access scopes**
* ALL_RECORDS
* LAB_RESULTS_ONLY
* PRESCRIPTIONS_ONLY
* VACCINATION_RECORDS_ONLY
* EMERGENCY_SUMMARY_ONLY
* INSURANCE_DATA_ONLY

**Example data stored**
* `access_id`
* `passport_id`
* `provider_id`
* `record_scope`
* `approved`
* `expires_at`
* `revoked`
* `created_at`

**Example functions**
* `request_access()`
* `approve_access()`
* `reject_access()`
* `revoke_access()`
* `check_access()`
* `get_active_permissions()`