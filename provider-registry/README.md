# Provider Registry

This contract manages verified healthcare providers.

**A provider could be:**

* Hospital
* Clinic
* Doctor
* Laboratory
* Pharmacy
* Insurance company
* Public health agency

**What it does**
* Registers healthcare providers.
* Verifies providers before they can request access.
* Assigns roles to providers.
* Allows suspension or revocation of bad actors.

**Example data stored**
* `provider_id`
* `provider_wallet_address`
* `provider_type`
* `license_hash`
* `country`
* `status`
* `verified_by`

**Example functions**
* `register_provider()`
* `verify_provider()`
* `suspend_provider()`
* `revoke_provider()`
* `get_provider()`