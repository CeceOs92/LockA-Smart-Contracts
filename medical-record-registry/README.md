# Medical Record Registry Contract

This contract registers proofs of medical records.

The actual medical file is not stored on-chain. Instead, the encrypted record is stored off-chain, and the blockchain stores a cryptographic reference.

**What it does**
* Registers a new medical record.
* Links the record to a patient passport.
* Links the record to the provider who issued it.
* Stores the hash of the encrypted record.
* Stores the type of record.
* Allows verification that a record has not been tampered with.

**Example record types**
* LAB_RESULT
* PRESCRIPTION
D* IAGNOSIS
VA* CCINATION
* SURGERY_REPORT
* ALLERGY_RECORD
* INSURANCE_RECORD
* MEDICAL_SUMMARY

**Example data stored**
* `record_id`
* `passport_id`
* `provider_id`
* `record_type`
* `encrypted_file_hash`
* `storage_pointer_hash`
* `issued_at`
* `status`

**Example functions**
* `add_record()`
* `update_record_status()`
* `verify_record_hash()`
* `get_record_metadata()`
* `get_patient_records()`