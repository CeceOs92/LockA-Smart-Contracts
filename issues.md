# LockA Smart Contracts — Open Source Issue Tracker

Issues drafted for open-source contribution, opened via `gh issue create` against `LockA-Medical-Passport/LockA-Smart-Contracts`.

## consent-access-manager

Build-out of the Consent & Access Manager contract (`consent-access-manager/`), including tests and events.

### 1. consent-access-manager: scaffold Soroban contract crate

**Labels:** good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/2

### Description
Set up the initial Rust/Soroban crate for the Consent & Access Manager contract inside `consent-access-manager/`, matching the layout used by the other LockA contract directories.

### Tasks
- [ ] Add `Cargo.toml` for the `consent-access-manager` crate (crate-type `cdylib` + `lib`, `soroban-sdk` dependency, `dev-dependencies` for testutils)
- [ ] Add `src/lib.rs` with an empty `#[contract]` struct (`ConsentAccessManager`) and `#[contractimpl]` block
- [ ] Add `src/test.rs` (or `src/tests.rs`) module wired up and empty/placeholder test that compiles
- [ ] Ensure `cargo build --target wasm32-unknown-unknown --release` succeeds from inside `consent-access-manager/`
- [ ] Add crate to the workspace root `Cargo.toml` (create the workspace file if it does not exist yet)

### Acceptance Criteria
- `cargo test` and `cargo build --target wasm32-unknown-unknown --release` both succeed inside `consent-access-manager/`.
- No contract logic is implemented yet — this issue only creates the buildable skeleton other issues will build on.

### Location
All work happens inside `consent-access-manager/`.

### 2. consent-access-manager: define RecordScope enum and access scopes

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/3

### Description
Define the `RecordScope` enum representing the granular record categories a patient can grant access to, per the scopes documented in `consent-access-manager/README.md`.

### Tasks
- [ ] Add a `RecordScope` enum with variants: `AllRecords`, `LabResultsOnly`, `PrescriptionsOnly`, `VaccinationRecordsOnly`, `EmergencySummaryOnly`, `InsuranceDataOnly`
- [ ] Derive `Clone`, `Debug`, `PartialEq`, `Eq` and the Soroban `contracttype` macro so it can be stored/passed through contract calls
- [ ] Document each variant with a short doc comment describing what it grants access to
- [ ] Add a basic unit test asserting the enum round-trips through `Env` storage (set/get)

### Acceptance Criteria
- `RecordScope` compiles, is exported from the crate, and is covered by at least one storage round-trip test.

### Location
`consent-access-manager/src/`

### 3. consent-access-manager: define AccessRequest data model and storage keys

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/4

### Description
Define the on-chain data structure for an access grant/request and the storage key scheme used to persist it, based on the fields listed in `consent-access-manager/README.md` (`access_id`, `passport_id`, `provider_id`, `record_scope`, `approved`, `expires_at`, `revoked`, `created_at`).

### Tasks
- [ ] Add an `AccessRequest` struct (`#[contracttype]`) with fields: `access_id: u64`, `passport_id: Address`, `provider_id: Address`, `record_scope: RecordScope`, `approved: bool`, `expires_at: u64`, `revoked: bool`, `created_at: u64`
- [ ] Define a `DataKey` enum (`#[contracttype]`) covering `AccessRequest(u64)`, a per-patient index key, and a monotonic `NextAccessId` counter key
- [ ] Add helper functions `read_access_request`, `write_access_request`, `next_access_id` in a `storage.rs` module
- [ ] Add unit tests for the storage helpers (write then read, counter increments)

### Acceptance Criteria
- Storage helpers compile and are unit tested independently of the public contract API.

### Location
`consent-access-manager/src/`

### 4. consent-access-manager: implement request_access()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/5

### Description
Implement the `request_access()` contract function that lets a registered provider request access to a patient's records for a given `RecordScope` and duration.

### Tasks
- [ ] Add `request_access(env, provider_id: Address, passport_id: Address, record_scope: RecordScope, duration_seconds: u64) -> u64` returning the new `access_id`
- [ ] Require `provider_id.require_auth()` so only the requesting provider can create the request
- [ ] Create an `AccessRequest` with `approved = false`, `revoked = false`, `expires_at = 0` (unset until approved), `created_at = env.ledger().timestamp()`
- [ ] Persist the request and add it to the patient's index of pending/active requests
- [ ] Reject requests with an invalid/zero `duration_seconds` with a descriptive contract error

### Acceptance Criteria
- Unit tests cover: successful request creation, auth failure when not signed by the provider, and rejection of invalid durations.

### Location
`consent-access-manager/src/`

### 5. consent-access-manager: implement approve_access()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/6

### Description
Implement `approve_access()` so a patient can approve a pending access request, activating it for the requested duration.

### Tasks
- [ ] Add `approve_access(env, passport_id: Address, access_id: u64)`
- [ ] Require `passport_id.require_auth()`
- [ ] Load the `AccessRequest`, verify it belongs to `passport_id` and is not already `revoked`
- [ ] Set `approved = true` and compute `expires_at = env.ledger().timestamp() + duration_seconds` (duration carried from the request, see previous issue) or accept an explicit `expires_at` if duration wasn't stored at request time — decide and document which approach is used
- [ ] Persist the updated request
- [ ] Return a contract error if the request does not exist or does not belong to the caller

### Acceptance Criteria
- Unit tests cover: successful approval, auth failure for a non-owning caller, approval of an already-revoked request fails, approval of a non-existent `access_id` fails.

### Location
`consent-access-manager/src/`

### 6. consent-access-manager: implement reject_access()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/7

### Description
Implement `reject_access()` so a patient can reject a pending access request instead of approving it.

### Tasks
- [ ] Add `reject_access(env, passport_id: Address, access_id: u64)`
- [ ] Require `passport_id.require_auth()`
- [ ] Verify the request belongs to `passport_id`, is not already approved, and is not already revoked/rejected
- [ ] Mark the request as rejected (either a dedicated `rejected: bool` field or by removing it from the active index — pick one approach and document it in the struct's doc comment)
- [ ] Return a contract error for non-existent requests or requests not owned by the caller

### Acceptance Criteria
- Unit tests cover: successful rejection, rejection of an already-approved request fails, auth failure for a non-owning caller.

### Location
`consent-access-manager/src/`

### 7. consent-access-manager: implement revoke_access()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/8

### Description
Implement `revoke_access()` so a patient can revoke a previously approved (active) access grant at any time, per the README's "Allows patients to revoke access anytime" requirement.

### Tasks
- [ ] Add `revoke_access(env, passport_id: Address, access_id: u64)`
- [ ] Require `passport_id.require_auth()`
- [ ] Verify the request belongs to `passport_id`
- [ ] Set `revoked = true` regardless of current `approved`/`expires_at` state
- [ ] Persist the update and remove/mark the entry in the patient's active-permissions index
- [ ] Return a contract error for non-existent requests or requests not owned by the caller

### Acceptance Criteria
- Unit tests cover: revoking an active grant, revoking an already-revoked grant is idempotent or errors clearly (pick one behavior and test it), auth failure for a non-owning caller.

### Location
`consent-access-manager/src/`

### 8. consent-access-manager: implement check_access()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/9

### Description
Implement `check_access()`, the read-only function providers/other contracts call to verify whether a given provider currently has valid access to a patient's records under a specific scope.

### Tasks
- [ ] Add `check_access(env, passport_id: Address, provider_id: Address, record_scope: RecordScope) -> bool`
- [ ] Access is valid when a matching `AccessRequest` exists with `approved == true`, `revoked == false`, `expires_at > env.ledger().timestamp()`, and `record_scope` matches or the stored scope is `AllRecords`
- [ ] No `require_auth()` needed — this is a read-only query any caller (e.g. `medical-record-registry` contract) can invoke
- [ ] Handle the case where no matching request exists (return `false`, do not panic)

### Acceptance Criteria
- Unit tests cover: active + matching scope returns true, expired grant returns false, revoked grant returns false, `AllRecords` grant satisfies a narrower scope query, non-matching scope returns false.

### Location
`consent-access-manager/src/`

### 9. consent-access-manager: implement get_active_permissions()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/10

### Description
Implement `get_active_permissions()` so a patient (or an authorized front end) can list all currently active, non-expired, non-revoked access grants for a passport.

### Tasks
- [ ] Add `get_active_permissions(env, passport_id: Address) -> Vec<AccessRequest>`
- [ ] Read the patient's request index and filter out entries that are not `approved`, are `revoked`, or have `expires_at <= env.ledger().timestamp()`
- [ ] Keep this function read-only (no `require_auth()`), since it only reads state already scoped to the given `passport_id`
- [ ] Consider and document result-size limits (e.g. cap or pagination) if the index can grow large

### Acceptance Criteria
- Unit tests cover: empty result for a patient with no grants, correct filtering of expired/revoked/unapproved entries, correct ordering (documented) for multiple active grants.

### Location
`consent-access-manager/src/`

### 10. consent-access-manager: enforce lazy expiry across all read paths

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/11

### Description
Ensure expiry (`expires_at`) is consistently enforced everywhere access state is read, not just in `check_access()`, so an expired grant can never be reported as active from any entry point.

### Tasks
- [ ] Extract a shared `is_active(request: &AccessRequest, now: u64) -> bool` helper used by both `check_access()` and `get_active_permissions()`
- [ ] Confirm ledger timestamp (`env.ledger().timestamp()`) is the single source of truth for "now" throughout the crate
- [ ] Add unit tests that advance the simulated ledger time (`env.ledger().set_timestamp(...)` in tests) to confirm a grant flips from active to expired at the correct boundary

### Acceptance Criteria
- A single shared helper backs every expiry check in the crate; no duplicated expiry logic exists.
- Tests demonstrate expiry boundary behavior (active at `expires_at - 1`, inactive at `expires_at`).

### Location
`consent-access-manager/src/`

### 11. consent-access-manager: define and emit access-requested event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/12

### Description
Emit a Soroban contract event whenever `request_access()` succeeds, so off-chain indexers/backends can react to new access requests without polling contract state.

### Tasks
- [ ] Define an event topic/schema, e.g. topics `("access", "requested")`, data `(access_id, passport_id, provider_id, record_scope)`
- [ ] Publish the event via `env.events().publish(...)` at the end of `request_access()`
- [ ] Document the event schema (topics + data types) in a `events.rs` module or in the contract's rustdoc
- [ ] Add a test asserting the event is published with the expected topics/data using the Soroban testutils event assertions

### Acceptance Criteria
- Event is emitted on every successful `request_access()` call and covered by a test that inspects `env.events().all()`.

### Location
`consent-access-manager/src/`

### 12. consent-access-manager: define and emit access-approved / access-rejected events

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/13

### Description
Emit contract events for the approval and rejection decisions made in `approve_access()` and `reject_access()`.

### Tasks
- [ ] Define event topics `("access", "approved")` and `("access", "rejected")`, data `(access_id, passport_id, provider_id)`
- [ ] Publish the approved event at the end of `approve_access()`
- [ ] Publish the rejected event at the end of `reject_access()`
- [ ] Document both event schemas alongside the access-requested event from the previous issue
- [ ] Add tests asserting each event is published on the corresponding success path, and NOT published when the call errors out

### Acceptance Criteria
- Both events are emitted only on success, documented, and covered by tests.

### Location
`consent-access-manager/src/`

### 13. consent-access-manager: define and emit access-revoked event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/14

### Description
Emit a contract event whenever `revoke_access()` succeeds, so providers and indexers can be notified promptly that access has been withdrawn — this is the event that ultimately drives real-time enforcement in the off-chain layer.

### Tasks
- [ ] Define event topic `("access", "revoked")`, data `(access_id, passport_id, provider_id, revoked_at)`
- [ ] Publish the event at the end of `revoke_access()`
- [ ] Document the event schema alongside the others from the two prior event issues
- [ ] Add a test asserting the event is published with correct data on successful revocation

### Acceptance Criteria
- Event is emitted on every successful `revoke_access()` call and covered by a test.

### Location
`consent-access-manager/src/`

### 14. consent-access-manager: add contract-level error types

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/15

### Description
Replace ad-hoc `panic!`/`unwrap()` calls introduced by the earlier function-implementation issues with a proper `#[contracterror]` enum, so callers (and tests) get typed, documented failure reasons.

### Tasks
- [ ] Add an `Error` enum (`#[contracterror]`, `#[repr(u32)]`) with variants such as `AccessRequestNotFound`, `NotRequestOwner`, `AlreadyApproved`, `AlreadyRevoked`, `InvalidDuration`
- [ ] Update `request_access`, `approve_access`, `reject_access`, and `revoke_access` to return `Result<_, Error>` using this enum instead of panicking
- [ ] Update existing unit tests to assert on the specific `Error` variant returned instead of just "call panics"
- [ ] Document each error variant with a doc comment explaining when it's returned

### Acceptance Criteria
- No `unwrap()`/`expect()`/bare `panic!` remain in the public contract functions for expected failure cases.
- Every error variant is exercised by at least one test.

### Location
`consent-access-manager/src/`

### 15. consent-access-manager: integration test suite covering the full consent lifecycle

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/16

### Description
Add an end-to-end integration test that exercises the full lifecycle of a consent grant through the public contract API, complementing the unit tests added alongside each individual function.

### Tasks
- [ ] Add `consent-access-manager/tests/` (or extend `src/test.rs`) with a scenario test: provider requests access → patient approves → `check_access` returns true → provider's access appears in `get_active_permissions` → patient revokes → `check_access` returns false
- [ ] Add a second scenario covering rejection: provider requests → patient rejects → `check_access` returns false, request does not appear in active permissions
- [ ] Add a third scenario covering natural expiry: approve with a short duration → advance ledger time past `expires_at` → `check_access` returns false without an explicit revoke
- [ ] Assert the correct sequence of emitted events for each scenario

### Acceptance Criteria
- All three lifecycle scenarios pass under `cargo test` and run as part of CI.

### Location
`consent-access-manager/` (tests may live in `src/test.rs` or a `tests/` directory, whichever matches the convention chosen in the scaffolding issue)

### 16. consent-access-manager: multi-provider and scope-isolation edge case tests

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/17

### Description
Add targeted tests for edge cases that the main lifecycle tests don't cover: multiple providers holding grants for the same patient, and scope isolation between different `RecordScope` values.

### Tasks
- [ ] Test: two different providers each hold an active grant for the same patient; revoking one does not affect the other
- [ ] Test: a provider with `LabResultsOnly` access fails `check_access` for `PrescriptionsOnly` and vice versa
- [ ] Test: a provider with `EmergencySummaryOnly` access does not gain access to `InsuranceDataOnly`
- [ ] Test: re-requesting access for a provider/scope pair that already has an active grant behaves predictably (either creates a second independent grant or is rejected — pick one behavior, document it, and test it)

### Acceptance Criteria
- All new edge-case tests pass and are added to the crate's existing test module/suite.

### Location
`consent-access-manager/src/`

### 17. consent-access-manager: rustdoc and README update with build/test instructions

**Labels:** documentation

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/18

### Description
Update `consent-access-manager/README.md` with concrete build/test instructions and rustdoc coverage for the public contract API, now that the contract is implemented.

### Tasks
- [ ] Add a "Build & Test" section to `consent-access-manager/README.md` with the exact `cargo build --target wasm32-unknown-unknown --release` and `cargo test` commands
- [ ] Document the final public function signatures (`request_access`, `approve_access`, `reject_access`, `revoke_access`, `check_access`, `get_active_permissions`) matching what was actually implemented, replacing the current illustrative-only list
- [ ] Add rustdoc comments (`///`) to every `#[contractimpl]` function explaining parameters, return values, and error conditions
- [ ] Document the emitted event schemas (topics + data) in the README or in a linked `events.rs` doc comment block

### Acceptance Criteria
- `cargo doc` builds without warnings for missing docs on public items (or `#![warn(missing_docs)]` passes clean).
- README accurately reflects the implemented API rather than the original design sketch.

### Location
`consent-access-manager/`

### 18. consent-access-manager: wire up CI workflow for build, test, and lint

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/19

### Description
Add a CI workflow so every PR touching `consent-access-manager/` is automatically built, tested, and linted, preventing regressions as more contracts are added to the monorepo.

### Tasks
- [ ] Add (or extend an existing) GitHub Actions workflow that runs on PRs touching `consent-access-manager/**`
- [ ] Steps: install Rust toolchain + `wasm32-unknown-unknown` target, `cargo build --target wasm32-unknown-unknown --release`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- [ ] Cache Cargo registry/build artifacts to keep CI runs fast
- [ ] Verify the workflow passes on a test PR before merging

### Acceptance Criteria
- CI workflow runs automatically on PRs touching this crate and fails the check if build, tests, clippy, or fmt fail.

### Location
`.github/workflows/` (workflow scoped to `consent-access-manager/`)


## device-data-attestation

Build-out of the Device & Data Attestation contract (`device-data-attestation/`), including tests and events.

### 19. device-data-attestation: scaffold Soroban contract crate

**Labels:** good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/21

### Description
Set up the initial Rust/Soroban crate for the Device & Data Attestation contract inside `device-data-attestation/`, matching the layout used by the other LockA contract directories (see `consent-access-manager/` for the established pattern).

### Tasks
- [ ] Add `Cargo.toml` for the `device-data-attestation` crate (crate-type `cdylib` + `lib`, `soroban-sdk` dependency, `dev-dependencies` for testutils)
- [ ] Add `src/lib.rs` with an empty `#[contract]` struct (`DeviceDataAttestation`) and `#[contractimpl]` block
- [ ] Add `src/test.rs` module wired up with an empty/placeholder test that compiles
- [ ] Ensure `cargo build --target wasm32-unknown-unknown --release` succeeds from inside `device-data-attestation/`
- [ ] Add the crate to the workspace root `Cargo.toml`

### Acceptance Criteria
- `cargo test` and `cargo build --target wasm32-unknown-unknown --release` both succeed inside `device-data-attestation/`.
- No contract logic is implemented yet — this issue only creates the buildable skeleton other issues will build on.

### Location
All work happens inside `device-data-attestation/`.

### 20. device-data-attestation: define DeviceCategory enum

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/22

### Description
Define the `DeviceCategory` enum representing the classes of medical devices/wearables the registry can approve, per the device use cases in the platform documentation (blood pressure monitors, glucose monitors, thermometers/pulse oximeters, cold-chain sensors, clinic equipment, wearables).

### Tasks
- [ ] Add a `DeviceCategory` enum with variants: `BloodPressureMonitor`, `GlucoseMonitor`, `Thermometer`, `PulseOximeter`, `ColdChainSensor`, `ClinicEquipment`, `Wearable`, `Other`
- [ ] Derive the Soroban `contracttype` macro plus `Clone`, `Debug`, `PartialEq`, `Eq` so it can be stored/passed through contract calls
- [ ] Document each variant with a short doc comment describing the kind of device it represents
- [ ] Add a unit test asserting the enum round-trips through `Env` storage (set/get)

### Acceptance Criteria
- `DeviceCategory` compiles, is exported from the crate, and is covered by at least one storage round-trip test.

### Location
`device-data-attestation/src/`

### 21. device-data-attestation: define Device data model and storage keys

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/23

### Description
Define the on-chain data structure representing an approved medical device/IoT source and the storage key scheme used to persist it.

### Tasks
- [ ] Add a `Device` struct (`#[contracttype]`) with fields: `device_id: BytesN<32>`, `owner: Address` (the patient the device is linked to), `issuer: Address` (the provider/organization that registered and vouches for the device), `category: DeviceCategory`, `public_key: BytesN<32>` (used to verify signed readings), `active: bool`, `registered_at: u64`
- [ ] Define a `DataKey` enum (`#[contracttype]`) covering `Device(BytesN<32>)` and a per-patient device index key
- [ ] Add storage helper functions `read_device`, `write_device`, `device_exists` in a `storage.rs` module
- [ ] Add unit tests for the storage helpers (write then read, missing device lookup)

### Acceptance Criteria
- Storage helpers compile and are unit tested independently of the public contract API.

### Location
`device-data-attestation/src/`

### 22. device-data-attestation: define DeviceAttestation data model and storage keys

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/24

### Description
Define the on-chain data structure for a single verifiable reading/attestation submitted by a registered device, plus its storage key scheme.

### Tasks
- [ ] Add a `DeviceAttestation` struct (`#[contracttype]`) with fields: `attestation_id: u64`, `device_id: BytesN<32>`, `passport_id: Address`, `reading_hash: BytesN<32>` (hash/commitment of the encrypted off-chain reading), `recorded_at: u64` (device-reported timestamp), `submitted_at: u64` (ledger timestamp at submission), `issuer_reference: Bytes` (optional pointer/metadata)
- [ ] Define storage keys `Attestation(u64)`, a per-device attestation index, and a per-patient attestation index, plus a monotonic `NextAttestationId` counter key
- [ ] Add storage helper functions `read_attestation`, `write_attestation`, `next_attestation_id` in `storage.rs`
- [ ] Add unit tests for the storage helpers (write then read, counter increments)

### Acceptance Criteria
- Storage helpers compile and are unit tested independently of the public contract API.

### Location
`device-data-attestation/src/`

### 23. device-data-attestation: implement register_device()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/25

### Description
Implement `register_device()` so a verified issuer (provider/organization) can register a new approved device and link it to a patient passport.

### Tasks
- [ ] Add `register_device(env, issuer: Address, device_id: BytesN<32>, owner: Address, category: DeviceCategory, public_key: BytesN<32>)`
- [ ] Require `issuer.require_auth()`
- [ ] Reject registration if `device_id` is already registered (`DeviceAlreadyRegistered` error)
- [ ] Persist the `Device` with `active = true`, `registered_at = env.ledger().timestamp()`
- [ ] Add the device to the owning patient's device index

### Acceptance Criteria
- Unit tests cover: successful registration, auth failure when not signed by the issuer, rejection of a duplicate `device_id`.

### Location
`device-data-attestation/src/`

### 24. device-data-attestation: implement revoke_device() and reactivate_device()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/26

### Description
Implement lifecycle management for a registered device so an issuer can revoke a compromised/decommissioned device, or reactivate one that was mistakenly revoked.

### Tasks
- [ ] Add `revoke_device(env, issuer: Address, device_id: BytesN<32>)` — requires `issuer.require_auth()`, verifies `issuer` matches the device's registered issuer, sets `active = false`
- [ ] Add `reactivate_device(env, issuer: Address, device_id: BytesN<32>)` — same auth/ownership checks, sets `active = true`
- [ ] Return a contract error for a non-existent `device_id` or when the caller is not the registering issuer
- [ ] Persist updates

### Acceptance Criteria
- Unit tests cover: successful revoke, successful reactivate, auth failure for a non-issuer caller, operations on a non-existent device fail with a clear error.

### Location
`device-data-attestation/src/`

### 25. device-data-attestation: implement submit_attestation() with signature verification

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/27

### Description
Implement `submit_attestation()`, the core function that anchors a verifiable, signed device reading on-chain. The device's signature over the reading is verified on-chain using its registered public key, so a reading cannot be forged without the device's private key.

### Tasks
- [ ] Add `submit_attestation(env, device_id: BytesN<32>, passport_id: Address, reading_hash: BytesN<32>, recorded_at: u64, issuer_reference: Bytes, signature: BytesN<64>) -> u64` returning the new `attestation_id`
- [ ] Look up the `Device`, reject if it does not exist (`DeviceNotFound`) or is inactive (`DeviceInactive`)
- [ ] Reject if `passport_id` does not match the device's registered `owner` (`DeviceOwnerMismatch`)
- [ ] Build the signed message (e.g. `device_id || reading_hash || recorded_at`) and verify it against the device's `public_key` using `env.crypto().ed25519_verify(...)`; reject on verification failure (`InvalidSignature`)
- [ ] Persist the `DeviceAttestation` with `submitted_at = env.ledger().timestamp()` and add it to the per-device and per-patient indexes

### Acceptance Criteria
- Unit tests cover: successful attestation with a valid signature, rejection for an unknown device, rejection for an inactive device, rejection for an owner mismatch, rejection for an invalid/forged signature.

### Location
`device-data-attestation/src/`

### 26. device-data-attestation: reject stale and duplicate attestations

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/28

### Description
Harden `submit_attestation()` against replay of old signed readings and duplicate submissions of the same reading, so a captured signed payload can't be resubmitted indefinitely.

### Tasks
- [ ] Reject attestations whose `recorded_at` is further in the past than a configurable freshness window (e.g. reject if `env.ledger().timestamp() - recorded_at > MAX_READING_AGE`), returning a `StaleReading` error
- [ ] Reject attestations whose `recorded_at` is in the future relative to the ledger timestamp
- [ ] Reject a duplicate submission carrying the same `(device_id, reading_hash, recorded_at)` tuple already stored (`DuplicateAttestation` error)
- [ ] Document the chosen freshness window and duplicate-detection approach in a doc comment

### Acceptance Criteria
- Unit tests cover: stale reading rejected, future-dated reading rejected, exact duplicate resubmission rejected, a legitimately fresh unique reading still succeeds.

### Location
`device-data-attestation/src/`

### 27. device-data-attestation: implement get_device() and is_device_active()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/29

### Description
Implement read-only lookup functions for device state, used by front ends and other contracts (e.g. to display device status or gate downstream logic on an active device).

### Tasks
- [ ] Add `get_device(env, device_id: BytesN<32>) -> Option<Device>`
- [ ] Add `is_device_active(env, device_id: BytesN<32>) -> bool` (returns `false` for both inactive and non-existent devices, does not panic)
- [ ] Keep both functions read-only (no `require_auth()`)

### Acceptance Criteria
- Unit tests cover: lookup of an existing device, lookup of a non-existent device returns `None`/`false`, active vs. revoked device status is reflected correctly.

### Location
`device-data-attestation/src/`

### 28. device-data-attestation: implement get_attestations_for_patient()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/30

### Description
Implement `get_attestations_for_patient()` so a patient (or an authorized front end/backend) can list all attestations recorded across all of their registered devices.

### Tasks
- [ ] Add `get_attestations_for_patient(env, passport_id: Address) -> Vec<DeviceAttestation>`
- [ ] Read from the per-patient attestation index built in `submit_attestation()`
- [ ] Keep the function read-only (no `require_auth()`), since results are already scoped to the given `passport_id`
- [ ] Consider and document result-size limits/pagination if the index can grow large

### Acceptance Criteria
- Unit tests cover: empty result for a patient with no attestations, correct aggregation across multiple devices belonging to the same patient.

### Location
`device-data-attestation/src/`

### 29. device-data-attestation: implement get_attestations_for_device()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/31

### Description
Implement `get_attestations_for_device()` so the attestation history of a single device can be audited independently of the owning patient's full record set.

### Tasks
- [ ] Add `get_attestations_for_device(env, device_id: BytesN<32>) -> Vec<DeviceAttestation>`
- [ ] Read from the per-device attestation index built in `submit_attestation()`
- [ ] Keep the function read-only (no `require_auth()`)
- [ ] Return an empty vector for a device with no attestations or an unknown `device_id` (do not panic)

### Acceptance Criteria
- Unit tests cover: empty result for a device with no attestations, correct ordering/content for multiple attestations from the same device, unknown device returns an empty vector.

### Location
`device-data-attestation/src/`

### 30. device-data-attestation: add contract-level error types

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/32

### Description
Consolidate the failure cases introduced across the prior implementation issues into a single, typed `#[contracterror]` enum so callers and tests get documented, stable error codes instead of ad-hoc panics.

### Tasks
- [ ] Add an `Error` enum (`#[contracterror]`, `#[repr(u32)]`) with variants: `DeviceAlreadyRegistered`, `DeviceNotFound`, `DeviceInactive`, `DeviceOwnerMismatch`, `NotDeviceIssuer`, `InvalidSignature`, `StaleReading`, `FutureDatedReading`, `DuplicateAttestation`
- [ ] Update `register_device`, `revoke_device`, `reactivate_device`, and `submit_attestation` to return `Result<_, Error>` using this enum
- [ ] Update existing unit tests to assert on the specific `Error` variant returned
- [ ] Document each error variant with a doc comment explaining when it's returned

### Acceptance Criteria
- No `unwrap()`/`expect()`/bare `panic!` remain in the public contract functions for expected failure cases.
- Every error variant is exercised by at least one test.

### Location
`device-data-attestation/src/`

### 31. device-data-attestation: define and emit device-registered / device-status events

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/33

### Description
Emit Soroban contract events for device lifecycle transitions (registration, revocation, reactivation) so off-chain indexers can maintain an up-to-date device directory without polling.

### Tasks
- [ ] Define event topics `("device", "registered")`, `("device", "revoked")`, `("device", "reactivated")` with data `(device_id, owner, issuer, category)` (status events can omit `category`)
- [ ] Publish the `registered` event at the end of `register_device()`
- [ ] Publish the `revoked`/`reactivated` events at the end of the corresponding lifecycle functions
- [ ] Document the event schemas in an `events.rs` module or rustdoc block
- [ ] Add tests asserting each event is published on its corresponding success path using the Soroban testutils event assertions

### Acceptance Criteria
- All three device lifecycle events are emitted only on success, documented, and covered by tests.

### Location
`device-data-attestation/src/`

### 32. device-data-attestation: define and emit attestation-submitted event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/34

### Description
Emit a contract event whenever `submit_attestation()` succeeds, carrying enough data for an off-chain indexer to anchor the reading commitment without re-reading contract storage.

### Tasks
- [ ] Define event topic `("attestation", "submitted")`, data `(attestation_id, device_id, passport_id, reading_hash, recorded_at)`
- [ ] Publish the event at the end of `submit_attestation()`, only after all validation (signature, freshness, duplicate checks) has passed
- [ ] Document the event schema alongside the device lifecycle events from the previous issue
- [ ] Add a test asserting the event is published with correct data on a successful submission, and NOT published for any rejected submission

### Acceptance Criteria
- Event is emitted on every successful `submit_attestation()` call and covered by a test; rejected submissions never emit it.

### Location
`device-data-attestation/src/`

### 33. device-data-attestation: unit test suite for device lifecycle and authorization

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/35

### Description
Consolidate and extend authorization/lifecycle coverage for device management into a dedicated test module, beyond the individual tests added alongside `register_device`/`revoke_device`/`reactivate_device`.

### Tasks
- [ ] Test: an issuer cannot revoke or reactivate a device registered by a different issuer
- [ ] Test: registering the same `device_id` twice (even by the same issuer) fails with `DeviceAlreadyRegistered`
- [ ] Test: `is_device_active` correctly reflects revoke → reactivate → revoke transitions in sequence
- [ ] Test: a patient (`owner`) cannot call `revoke_device`/`reactivate_device` — only the registering `issuer` can

### Acceptance Criteria
- All new tests pass and are added to the crate's existing test module/suite.

### Location
`device-data-attestation/src/`

### 34. device-data-attestation: integration test suite covering the full attestation lifecycle

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/36

### Description
Add an end-to-end integration test that exercises the full lifecycle through the public contract API: device registration, signed attestation submission, querying by patient and by device, and revocation blocking future submissions.

### Tasks
- [ ] Add `device-data-attestation/tests/` (or extend `src/test.rs`) with a scenario: issuer registers a device for a patient → device submits a validly-signed attestation → attestation appears in both `get_attestations_for_patient` and `get_attestations_for_device`
- [ ] Add a scenario: issuer revokes the device → a subsequent `submit_attestation` call for that device fails with `DeviceInactive`
- [ ] Add a scenario: a forged/invalid signature is rejected and no attestation or event is recorded
- [ ] Assert the correct sequence of emitted events for each scenario, using an Ed25519 test keypair generated via the Soroban testutils crypto helpers

### Acceptance Criteria
- All scenarios pass under `cargo test` and run as part of CI.

### Location
`device-data-attestation/` (tests may live in `src/test.rs` or a `tests/` directory, matching the convention chosen in the scaffolding issue)

### 35. device-data-attestation: rustdoc and README update with build/test instructions

**Labels:** documentation

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/37

### Description
Update `device-data-attestation/README.md` (currently just a title) with a full description of the contract, its data model, and concrete build/test instructions, now that the contract is implemented.

### Tasks
- [ ] Describe the contract's purpose: registering approved medical devices/IoT sources and anchoring verifiable, signed readings on-chain, per the platform's IoT data flow design
- [ ] Add a "Build & Test" section with the exact `cargo build --target wasm32-unknown-unknown --release` and `cargo test` commands
- [ ] Document the final public function signatures (`register_device`, `revoke_device`, `reactivate_device`, `submit_attestation`, `get_device`, `is_device_active`, `get_attestations_for_patient`, `get_attestations_for_device`) matching what was actually implemented
- [ ] Add rustdoc comments (`///`) to every `#[contractimpl]` function explaining parameters, return values, and error conditions
- [ ] Document the emitted event schemas (topics + data) in the README or a linked `events.rs` doc comment block

### Acceptance Criteria
- `cargo doc` builds without warnings for missing docs on public items.
- README accurately reflects the implemented API and data model.

### Location
`device-data-attestation/`

### 36. device-data-attestation: wire up CI workflow for build, test, and lint

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/38

### Description
Add a CI workflow so every PR touching `device-data-attestation/` is automatically built, tested, and linted, matching the CI coverage set up for `consent-access-manager/`.

### Tasks
- [ ] Add (or extend the existing) GitHub Actions workflow to also run on PRs touching `device-data-attestation/**`
- [ ] Steps: install Rust toolchain + `wasm32-unknown-unknown` target, `cargo build --target wasm32-unknown-unknown --release`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- [ ] Reuse Cargo registry/build artifact caching already configured for the workspace
- [ ] Verify the workflow passes on a test PR before merging

### Acceptance Criteria
- CI workflow runs automatically on PRs touching this crate and fails the check if build, tests, clippy, or fmt fail.

### Location
`.github/workflows/` (workflow scoped to `device-data-attestation/`)


## medical-record-registry

Build-out of the Medical Record Registry contract (`medical-record-registry/`), including tests and events.

### 37. medical-record-registry: scaffold Soroban contract crate

**Labels:** good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/39

### Description
Set up the initial Rust/Soroban crate for the Medical Record Registry contract inside `medical-record-registry/`, matching the layout used by `consent-access-manager/` and `device-data-attestation/`.

### Tasks
- [ ] Add `Cargo.toml` for the `medical-record-registry` crate (crate-type `cdylib` + `lib`, `soroban-sdk` dependency, `dev-dependencies` for testutils)
- [ ] Add `src/lib.rs` with an empty `#[contract]` struct (`MedicalRecordRegistry`) and `#[contractimpl]` block
- [ ] Add `src/test.rs` module wired up with an empty/placeholder test that compiles
- [ ] Ensure `cargo build --target wasm32-unknown-unknown --release` succeeds from inside `medical-record-registry/`
- [ ] Add the crate to the workspace root `Cargo.toml`

### Acceptance Criteria
- `cargo test` and `cargo build --target wasm32-unknown-unknown --release` both succeed inside `medical-record-registry/`.
- No contract logic is implemented yet — this issue only creates the buildable skeleton other issues will build on.

### Location
All work happens inside `medical-record-registry/`.

### 38. medical-record-registry: define RecordType enum

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/40

### Description
Define the `RecordType` enum representing the record categories this contract can anchor, per the types listed in `medical-record-registry/README.md`.

### Tasks
- [ ] Add a `RecordType` enum with variants: `LabResult`, `Prescription`, `Diagnosis`, `Vaccination`, `SurgeryReport`, `AllergyRecord`, `InsuranceRecord`, `MedicalSummary`
- [ ] Derive the Soroban `contracttype` macro plus `Clone`, `Debug`, `PartialEq`, `Eq` so it can be stored/passed through contract calls
- [ ] Document each variant with a short doc comment describing what it represents
- [ ] Add a unit test asserting the enum round-trips through `Env` storage (set/get)

### Acceptance Criteria
- `RecordType` compiles, is exported from the crate, and is covered by at least one storage round-trip test.

### Location
`medical-record-registry/src/`

### 39. medical-record-registry: define RecordStatus enum and status transition rules

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/41

### Description
Define the `RecordStatus` enum representing a record's lifecycle state, and document the allowed transitions between states before any function implements them.

### Tasks
- [ ] Add a `RecordStatus` enum with variants: `Active`, `Amended`, `Revoked`
- [ ] Derive the Soroban `contracttype` macro plus `Clone`, `Debug`, `PartialEq`, `Eq`
- [ ] Document the allowed transitions: `Active -> Amended`, `Active -> Revoked`, `Amended -> Revoked`; `Revoked` is terminal (no transitions out of it)
- [ ] Add a `fn is_valid_transition(from: &RecordStatus, to: &RecordStatus) -> bool` helper with unit tests covering every legal and illegal transition pair

### Acceptance Criteria
- `RecordStatus` compiles and is exported from the crate.
- `is_valid_transition` is unit tested for all 9 combinations of `(from, to)` pairs.

### Location
`medical-record-registry/src/`

### 40. medical-record-registry: define MedicalRecord data model and storage keys

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/42

### Description
Define the on-chain data structure for a medical record commitment and the storage key scheme used to persist it, based on the fields listed in `medical-record-registry/README.md` (`record_id`, `passport_id`, `provider_id`, `record_type`, `encrypted_file_hash`, `storage_pointer_hash`, `issued_at`, `status`).

### Tasks
- [ ] Add a `MedicalRecord` struct (`#[contracttype]`) with fields: `record_id: u64`, `passport_id: Address`, `provider_id: Address`, `record_type: RecordType`, `encrypted_file_hash: BytesN<32>`, `storage_pointer_hash: BytesN<32>`, `issued_at: u64`, `status: RecordStatus`
- [ ] Define a `DataKey` enum (`#[contracttype]`) covering `Record(u64)`, a per-patient record index key, a per-provider record index key, and a monotonic `NextRecordId` counter key
- [ ] Add storage helper functions `read_record`, `write_record`, `next_record_id` in a `storage.rs` module
- [ ] Add unit tests for the storage helpers (write then read, counter increments, missing record lookup)

### Acceptance Criteria
- Storage helpers compile and are unit tested independently of the public contract API.

### Location
`medical-record-registry/src/`

### 41. medical-record-registry: implement add_record()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/43

### Description
Implement `add_record()` so a verified provider can register a new medical record commitment, linking it to the patient passport and the issuing provider.

### Tasks
- [ ] Add `add_record(env, provider_id: Address, passport_id: Address, record_type: RecordType, encrypted_file_hash: BytesN<32>, storage_pointer_hash: BytesN<32>) -> u64` returning the new `record_id`
- [ ] Require `provider_id.require_auth()` so only the issuing provider can create the record
- [ ] Persist the `MedicalRecord` with `status = RecordStatus::Active` and `issued_at = env.ledger().timestamp()`
- [ ] Add the record to both the patient's and the provider's record indexes

### Acceptance Criteria
- Unit tests cover: successful record creation, auth failure when not signed by the provider, correct `issued_at`/default `status` values on the stored record.

### Location
`medical-record-registry/src/`

### 42. medical-record-registry: reject duplicate record hashes for the same patient

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/44

### Description
Harden `add_record()` against accidental or malicious re-registration of the exact same encrypted file for a patient, which would otherwise create confusing duplicate entries in the patient's record history.

### Tasks
- [ ] Before persisting, check whether an existing non-revoked record for the same `passport_id` already has the same `encrypted_file_hash`
- [ ] Reject the call with a `DuplicateRecordHash` error if a match is found
- [ ] Allow re-registration of the same hash if the prior matching record's status is `Revoked` (e.g. correcting a mistaken revocation with a fresh record)
- [ ] Document this rule in a doc comment on `add_record`

### Acceptance Criteria
- Unit tests cover: duplicate hash for the same active record is rejected, same hash is accepted again after the prior record was revoked, different patients can share the same hash without conflict.

### Location
`medical-record-registry/src/`

### 43. medical-record-registry: implement update_record_status()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/45

### Description
Implement `update_record_status()` so the issuing provider can amend or revoke a record it previously added, enforcing the transition rules defined earlier.

### Tasks
- [ ] Add `update_record_status(env, provider_id: Address, record_id: u64, new_status: RecordStatus)`
- [ ] Require `provider_id.require_auth()`
- [ ] Load the `MedicalRecord`, verify `provider_id` matches the record's stored `provider_id` (`NotIssuingProvider` error otherwise)
- [ ] Validate the transition using the `is_valid_transition` helper; reject with `InvalidStatusTransition` if illegal (e.g. attempting to change a `Revoked` record)
- [ ] Persist the updated status

### Acceptance Criteria
- Unit tests cover: successful `Active -> Amended`, successful `Active -> Revoked`, successful `Amended -> Revoked`, rejection of any transition out of `Revoked`, auth failure for a non-issuing provider, error for a non-existent `record_id`.

### Location
`medical-record-registry/src/`

### 44. medical-record-registry: implement verify_record_hash()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/46

### Description
Implement `verify_record_hash()`, the read-only function a provider or patient client uses to confirm that a presented encrypted file matches the on-chain commitment and has not been tampered with.

### Tasks
- [ ] Add `verify_record_hash(env, record_id: u64, candidate_hash: BytesN<32>) -> bool`
- [ ] Return `true` only if the record exists and `candidate_hash == record.encrypted_file_hash`
- [ ] Return `false` (not an error) for a non-existent `record_id`, so callers can treat verification failure uniformly
- [ ] No `require_auth()` needed — this is a public read-only integrity check

### Acceptance Criteria
- Unit tests cover: matching hash returns true, mismatched hash returns false, non-existent record returns false without panicking.

### Location
`medical-record-registry/src/`

### 45. medical-record-registry: implement get_record_metadata()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/47

### Description
Implement `get_record_metadata()` so callers can fetch the full stored metadata for a single record by its ID.

### Tasks
- [ ] Add `get_record_metadata(env, record_id: u64) -> Option<MedicalRecord>`
- [ ] Return `None` for a non-existent `record_id` rather than panicking
- [ ] Keep the function read-only (no `require_auth()`)

### Acceptance Criteria
- Unit tests cover: lookup of an existing record returns the exact stored data, lookup of a non-existent record returns `None`.

### Location
`medical-record-registry/src/`

### 46. medical-record-registry: implement get_patient_records()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/48

### Description
Implement `get_patient_records()` so a patient's full record history (across all providers) can be listed in one call, using the per-patient index built in `add_record()`.

### Tasks
- [ ] Add `get_patient_records(env, passport_id: Address) -> Vec<MedicalRecord>`
- [ ] Read from the per-patient record index and resolve each entry to its full `MedicalRecord`
- [ ] Keep the function read-only (no `require_auth()`), since results are already scoped to the given `passport_id`
- [ ] Consider and document result-size limits/pagination if a patient's record count can grow large

### Acceptance Criteria
- Unit tests cover: empty result for a patient with no records, correct aggregation across multiple providers and record types for the same patient, revoked records still appear (status is visible, not hidden) unless explicitly documented otherwise.

### Location
`medical-record-registry/src/`

### 47. medical-record-registry: implement get_provider_records()

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/49

### Description
Implement `get_provider_records()`, complementing `get_patient_records()`, so a provider can audit the full set of records it has issued (useful for provider-side dashboards and compliance review).

### Tasks
- [ ] Add `get_provider_records(env, provider_id: Address) -> Vec<MedicalRecord>`
- [ ] Read from the per-provider record index built in `add_record()`
- [ ] Keep the function read-only (no `require_auth()`)
- [ ] Return an empty vector for a provider with no issued records

### Acceptance Criteria
- Unit tests cover: empty result for a provider with no records, correct aggregation across multiple patients and record types issued by the same provider.

### Location
`medical-record-registry/src/`

### 48. medical-record-registry: add contract-level error types

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/50

### Description
Consolidate the failure cases introduced across the prior implementation issues into a single, typed `#[contracterror]` enum so callers and tests get documented, stable error codes instead of ad-hoc panics.

### Tasks
- [ ] Add an `Error` enum (`#[contracterror]`, `#[repr(u32)]`) with variants: `RecordNotFound`, `NotIssuingProvider`, `InvalidStatusTransition`, `DuplicateRecordHash`
- [ ] Update `add_record` and `update_record_status` to return `Result<_, Error>` using this enum
- [ ] Update existing unit tests to assert on the specific `Error` variant returned instead of a generic panic
- [ ] Document each error variant with a doc comment explaining when it's returned

### Acceptance Criteria
- No `unwrap()`/`expect()`/bare `panic!` remain in the public contract functions for expected failure cases.
- Every error variant is exercised by at least one test.

### Location
`medical-record-registry/src/`

### 49. medical-record-registry: define and emit record-added event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/51

### Description
Emit a Soroban contract event whenever `add_record()` succeeds, so off-chain indexers/backends can react to new record commitments without polling contract state.

### Tasks
- [ ] Define event topics `("record", "added")`, data `(record_id, passport_id, provider_id, record_type)`
- [ ] Publish the event via `env.events().publish(...)` at the end of `add_record()`, only after the duplicate-hash check has passed
- [ ] Document the event schema in an `events.rs` module or in the contract's rustdoc
- [ ] Add a test asserting the event is published with the expected topics/data using the Soroban testutils event assertions

### Acceptance Criteria
- Event is emitted on every successful `add_record()` call and covered by a test.

### Location
`medical-record-registry/src/`

### 50. medical-record-registry: define and emit record-status-updated event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/52

### Description
Emit a contract event whenever `update_record_status()` succeeds, carrying both the previous and new status so downstream systems (e.g. consent-gated record viewers) can react to amendments and revocations.

### Tasks
- [ ] Define event topic `("record", "status_updated")`, data `(record_id, passport_id, provider_id, old_status, new_status)`
- [ ] Publish the event at the end of `update_record_status()`, only after the transition validation has passed
- [ ] Document the event schema alongside the record-added event from the previous issue
- [ ] Add a test asserting the event is published only on successful, valid transitions, and NOT published when the call errors out

### Acceptance Criteria
- Event is emitted on every successful `update_record_status()` call and covered by a test; rejected transitions never emit it.

### Location
`medical-record-registry/src/`

### 51. medical-record-registry: unit test suite for authorization and status transitions

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/53

### Description
Consolidate and extend authorization and lifecycle coverage into a dedicated test module, beyond the individual tests added alongside `add_record`/`update_record_status`.

### Tasks
- [ ] Test: a provider cannot update the status of a record issued by a different provider
- [ ] Test: a patient (`passport_id`) cannot call `update_record_status` — only the issuing `provider_id` can
- [ ] Test: `verify_record_hash` continues to return the correct result after a record has been amended or revoked (status changes do not alter the stored hash)
- [ ] Test: `get_patient_records` and `get_provider_records` stay consistent (same record, same data) after a status update

### Acceptance Criteria
- All new tests pass and are added to the crate's existing test module/suite.

### Location
`medical-record-registry/src/`

### 52. medical-record-registry: integration test suite covering the full record lifecycle

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/54

### Description
Add an end-to-end integration test that exercises the full lifecycle through the public contract API, complementing the unit tests added alongside each individual function.

### Tasks
- [ ] Add `medical-record-registry/tests/` (or extend `src/test.rs`) with a scenario: provider adds a record → record appears in `get_patient_records` and `get_provider_records` → `verify_record_hash` succeeds for the correct hash and fails for a tampered hash
- [ ] Add a scenario: provider amends the record (`Active -> Amended`) → `get_record_metadata` reflects the new status → a subsequent attempt to revoke then re-amend fails once the record is `Revoked`
- [ ] Add a scenario: duplicate registration of the same `encrypted_file_hash` for the same patient is rejected while the original record is still active
- [ ] Assert the correct sequence of emitted events for each scenario

### Acceptance Criteria
- All scenarios pass under `cargo test` and run as part of CI.

### Location
`medical-record-registry/` (tests may live in `src/test.rs` or a `tests/` directory, matching the convention chosen in the scaffolding issue)

### 53. medical-record-registry: rustdoc and README update with build/test instructions

**Labels:** documentation

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/55

### Description
Update `medical-record-registry/README.md` with concrete build/test instructions and rustdoc coverage for the public contract API, now that the contract is implemented. Also fix the malformed bullet list in the current "Example record types" section (`D* IAGNOSIS`, `VA* CCINATION`).

### Tasks
- [ ] Fix the "Example record types" bullet list formatting (`DIAGNOSIS`, `VACCINATION`)
- [ ] Add a "Build & Test" section with the exact `cargo build --target wasm32-unknown-unknown --release` and `cargo test` commands
- [ ] Document the final public function signatures (`add_record`, `update_record_status`, `verify_record_hash`, `get_record_metadata`, `get_patient_records`, `get_provider_records`) matching what was actually implemented, replacing the current illustrative-only list
- [ ] Add rustdoc comments (`///`) to every `#[contractimpl]` function explaining parameters, return values, and error conditions
- [ ] Document the emitted event schemas (topics + data) in the README or a linked `events.rs` doc comment block, and document the `RecordStatus` transition rules

### Acceptance Criteria
- `cargo doc` builds without warnings for missing docs on public items.
- README accurately reflects the implemented API rather than the original design sketch, and its formatting issues are fixed.

### Location
`medical-record-registry/`

### 54. medical-record-registry: wire up CI workflow for build, test, and lint

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/56

### Description
Add a CI workflow so every PR touching `medical-record-registry/` is automatically built, tested, and linted, matching the CI coverage set up for `consent-access-manager/` and `device-data-attestation/`.

### Tasks
- [ ] Add (or extend the existing) GitHub Actions workflow to also run on PRs touching `medical-record-registry/**`
- [ ] Steps: install Rust toolchain + `wasm32-unknown-unknown` target, `cargo build --target wasm32-unknown-unknown --release`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- [ ] Reuse Cargo registry/build artifact caching already configured for the workspace
- [ ] Verify the workflow passes on a test PR before merging

### Acceptance Criteria
- CI workflow runs automatically on PRs touching this crate and fails the check if build, tests, clippy, or fmt fail.

### Location
`.github/workflows/` (workflow scoped to `medical-record-registry/`)


## patient-passport-registry

Build-out of the Patient Passport Registry contract (`patient-passport-registry/`), including tests and events.

### 57. patient-passport-registry: scaffold Soroban contract crate

**Labels:** good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/57

### Description
Set up the initial Rust/Soroban crate for the Patient Passport Registry contract inside `patient-passport-registry/`, matching the layout used by `consent-access-manager/`, `device-data-attestation/`, and `medical-record-registry/`.

### Tasks
- [ ] Add `Cargo.toml` for the `patient-passport-registry` crate (crate-type `cdylib` + `lib`, `soroban-sdk` dependency, `dev-dependencies` for testutils)
- [ ] Add `src/lib.rs` with an empty `#[contract]` struct (`PatientPassportRegistry`) and `#[contractimpl]` block
- [ ] Add `src/test.rs` module wired up with an empty/placeholder test that compiles
- [ ] Ensure `cargo build --target wasm32-unknown-unknown --release` succeeds from inside `patient-passport-registry/`
- [ ] Add the crate to the workspace root `Cargo.toml`

### Acceptance Criteria
- `cargo test` and `cargo build --target wasm32-unknown-unknown --release` both succeed inside `patient-passport-registry/`.
- No contract logic is implemented yet — this issue only creates the buildable skeleton other issues will build on.

### Location
All work happens inside `patient-passport-registry/`.

### 58. patient-passport-registry: define PassportStatus enum and status transition rules

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/58

### Description
Define the `PassportStatus` enum representing a passport's lifecycle state, per `patient-passport-registry/README.md` ("Tracks whether a passport is active, suspended, or revoked"), and document the allowed transitions before any function implements them.

### Tasks
- [ ] Add a `PassportStatus` enum with variants: `Active`, `Suspended`, `Revoked`
- [ ] Derive the Soroban `contracttype` macro plus `Clone`, `Debug`, `PartialEq`, `Eq`
- [ ] Document the allowed transitions: `Active -> Suspended`, `Suspended -> Active`, `Active -> Revoked`, `Suspended -> Revoked`; `Revoked` is terminal (no transitions out of it)
- [ ] Add a `fn is_valid_transition(from: &PassportStatus, to: &PassportStatus) -> bool` helper with unit tests covering every legal and illegal transition pair

### Acceptance Criteria
- `PassportStatus` compiles and is exported from the crate.
- `is_valid_transition` is unit tested for all 9 combinations of `(from, to)` pairs.

### Location
`patient-passport-registry/src/`

### 59. patient-passport-registry: define Passport data model and storage keys

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/59

### Description
Define the on-chain data structure for a patient's Medical Passport identity and the storage key scheme used to persist it, based on the fields listed in `patient-passport-registry/README.md` (`passport_id`, `patient_wallet_address`, `public_identity_hash`, `created_at`, `status`, `recovery_address`).

### Tasks
- [ ] Add a `Passport` struct (`#[contracttype]`) with fields: `passport_id: u64`, `patient_wallet_address: Address`, `public_identity_hash: BytesN<32>`, `created_at: u64`, `status: PassportStatus`, `recovery_address: Option<Address>`
- [ ] Define a `DataKey` enum (`#[contracttype]`) covering `Passport(u64)`, a `WalletIndex(Address)` reverse-lookup key (wallet address -> passport_id), and a monotonic `NextPassportId` counter key
- [ ] Add storage helper functions `read_passport`, `write_passport`, `next_passport_id`, `read_passport_id_by_wallet` in a `storage.rs` module
- [ ] Add unit tests for the storage helpers (write then read, counter increments, missing passport lookup)

### Acceptance Criteria
- Storage helpers compile and are unit tested independently of the public contract API.

### Location
`patient-passport-registry/src/`

### 60. patient-passport-registry: implement register_patient()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/60

### Description
Implement `register_patient()` so a new patient can create their Medical Passport identity, linking their Stellar wallet address to a freshly generated `passport_id`.

### Tasks
- [ ] Add `register_patient(env, patient_wallet_address: Address, public_identity_hash: BytesN<32>, recovery_address: Option<Address>) -> u64` returning the new `passport_id`
- [ ] Require `patient_wallet_address.require_auth()` so only the wallet owner can self-register
- [ ] Persist the `Passport` with `status = PassportStatus::Active` and `created_at = env.ledger().timestamp()`
- [ ] Populate the `WalletIndex` reverse-lookup entry for the wallet address
- [ ] Reject registration if `recovery_address` is equal to `patient_wallet_address` (`RecoveryAddressSameAsWallet` error)

### Acceptance Criteria
- Unit tests cover: successful registration, auth failure when not signed by the wallet owner, rejection of a recovery address identical to the patient wallet, correct default `status`/`created_at` values on the stored passport.

### Location
`patient-passport-registry/src/`

### 61. patient-passport-registry: reject duplicate wallet address registration

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/61

### Description
Harden `register_patient()` so the same Stellar wallet address cannot be linked to more than one passport, keeping the `WalletIndex` reverse lookup unambiguous.

### Tasks
- [ ] Before persisting, check whether `patient_wallet_address` already has an entry in the `WalletIndex`
- [ ] Reject the call with a `WalletAlreadyRegistered` error if a match is found
- [ ] Document that a wallet freed up by key rotation (see the `update_patient_key` issue) should no longer block re-registration under a new passport, and cross-reference that issue
- [ ] Add a unit test asserting a second `register_patient()` call with the same wallet address fails

### Acceptance Criteria
- Duplicate wallet registration is rejected with a clear, typed error and covered by a test.

### Location
`patient-passport-registry/src/`

### 62. patient-passport-registry: implement get_passport()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/62

### Description
Implement `get_passport()` so callers can fetch the full stored passport record by its `passport_id`.

### Tasks
- [ ] Add `get_passport(env, passport_id: u64) -> Option<Passport>`
- [ ] Return `None` for a non-existent `passport_id` rather than panicking
- [ ] Keep the function read-only (no `require_auth()`)

### Acceptance Criteria
- Unit tests cover: lookup of an existing passport returns the exact stored data, lookup of a non-existent passport returns `None`.

### Location
`patient-passport-registry/src/`

### 63. patient-passport-registry: implement get_passport_by_wallet()

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/63

### Description
Implement `get_passport_by_wallet()`, a convenience lookup so callers holding only a Stellar wallet address (e.g. a Freighter-connected front end) can resolve the associated passport without already knowing the `passport_id`.

### Tasks
- [ ] Add `get_passport_by_wallet(env, wallet_address: Address) -> Option<Passport>`
- [ ] Resolve the `passport_id` via the `WalletIndex` reverse lookup, then delegate to the same logic as `get_passport`
- [ ] Return `None` if the wallet address has no associated passport
- [ ] Keep the function read-only (no `require_auth()`)

### Acceptance Criteria
- Unit tests cover: lookup by a registered wallet returns the correct passport, lookup by an unregistered wallet returns `None`.

### Location
`patient-passport-registry/src/`

### 64. patient-passport-registry: implement update_patient_key() for self-service and recovery-based key rotation

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/64

### Description
Implement `update_patient_key()` so a patient can rotate the Stellar wallet address linked to their passport, either by authorizing with their current wallet or, if it's lost, via their designated `recovery_address`.

### Tasks
- [ ] Add `update_patient_key(env, passport_id: u64, caller: Address, new_wallet_address: Address)`
- [ ] Require `caller.require_auth()`
- [ ] Accept the call only if `caller` equals the passport's current `patient_wallet_address` OR the passport's `recovery_address` (`NotAuthorizedForKeyRotation` error otherwise)
- [ ] Reject if `new_wallet_address` is already linked to a different passport (`WalletAlreadyRegistered`)
- [ ] Update `patient_wallet_address`, remove the old `WalletIndex` entry, and add the new one

### Acceptance Criteria
- Unit tests cover: successful rotation authorized by the current wallet, successful rotation authorized by the recovery address, rejection when `caller` is neither the wallet nor the recovery address, rejection when `new_wallet_address` is already taken by another passport.

### Location
`patient-passport-registry/src/`

### 65. patient-passport-registry: implement update_recovery_address()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/65

### Description
Implement `update_recovery_address()` so a patient can set or change the recovery address associated with their passport independently of rotating their primary wallet key.

### Tasks
- [ ] Add `update_recovery_address(env, passport_id: u64, patient_wallet_address: Address, new_recovery_address: Option<Address>)`
- [ ] Require `patient_wallet_address.require_auth()` and verify it matches the passport's stored `patient_wallet_address` (`NotPassportOwner` error otherwise) — the recovery address itself cannot change who the next recovery address is
- [ ] Reject if `new_recovery_address` equals `patient_wallet_address` (`RecoveryAddressSameAsWallet`)
- [ ] Persist the updated `recovery_address` (passing `None` clears it)

### Acceptance Criteria
- Unit tests cover: successful update, successful clearing (`None`), rejection when called by a non-owning wallet, rejection when the new recovery address equals the patient's own wallet.

### Location
`patient-passport-registry/src/`

### 66. patient-passport-registry: implement deactivate_passport()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/66

### Description
Implement `deactivate_passport()` so a patient's passport can be moved to `Suspended` or `Revoked`, using the transition rules defined earlier.

### Tasks
- [ ] Add `deactivate_passport(env, passport_id: u64, caller: Address, new_status: PassportStatus)`
- [ ] Require `caller.require_auth()` and verify `caller` matches the passport's `patient_wallet_address` OR `recovery_address` (`NotAuthorizedForStatusChange` error otherwise)
- [ ] Reject `new_status == PassportStatus::Active` for this function — activation is handled by `reactivate_passport` (see the follow-up issue)
- [ ] Validate the transition using the `is_valid_transition` helper; reject with `InvalidStatusTransition` if illegal (e.g. attempting to change an already-`Revoked` passport)
- [ ] Persist the updated status

### Acceptance Criteria
- Unit tests cover: successful `Active -> Suspended`, successful `Active -> Revoked`, successful `Suspended -> Revoked`, rejection of any transition out of `Revoked`, auth failure for a caller who is neither the wallet nor recovery address, rejection of passing `Active` as `new_status`.

### Location
`patient-passport-registry/src/`

### 67. patient-passport-registry: implement reactivate_passport()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/67

### Description
Implement `reactivate_passport()` so a `Suspended` passport can be restored to `Active`, complementing `deactivate_passport()`.

### Tasks
- [ ] Add `reactivate_passport(env, passport_id: u64, caller: Address)`
- [ ] Require `caller.require_auth()` and verify `caller` matches the passport's `patient_wallet_address` OR `recovery_address`
- [ ] Validate the transition is `Suspended -> Active` using the shared `is_valid_transition` helper; reject with `InvalidStatusTransition` for any other current status (in particular, `Revoked` is terminal and can never be reactivated)
- [ ] Persist the updated status

### Acceptance Criteria
- Unit tests cover: successful `Suspended -> Active`, rejection when the current status is `Active` (no-op transition) or `Revoked` (terminal), auth failure for a non-owning, non-recovery caller.

### Location
`patient-passport-registry/src/`

### 68. patient-passport-registry: block key rotation and recovery updates on non-Active passports

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/68

### Description
Add a cross-cutting guard so a `Suspended` or `Revoked` passport cannot have its wallet key rotated or its recovery address changed — those operations should only be available while the passport is in good standing, closing off a path for an attacker who compromises a recovery address after a passport has already been flagged.

### Tasks
- [ ] Add a shared `require_active(passport: &Passport) -> Result<(), Error>` helper
- [ ] Call it at the top of `update_patient_key()` and `update_recovery_address()`, returning `PassportNotActive` if `status != PassportStatus::Active`
- [ ] Confirm `deactivate_passport()` and `reactivate_passport()` themselves remain callable regardless of current status (subject to their own transition validation), since they are the only way out of a non-Active state
- [ ] Add unit tests asserting `update_patient_key` and `update_recovery_address` both fail with `PassportNotActive` on a `Suspended` passport and on a `Revoked` passport

### Acceptance Criteria
- Neither key rotation nor recovery-address updates can succeed on a non-`Active` passport, verified by tests for both `Suspended` and `Revoked` states.

### Location
`patient-passport-registry/src/`

### 69. patient-passport-registry: add contract-level error types

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/69

### Description
Consolidate the failure cases introduced across the prior implementation issues into a single, typed `#[contracterror]` enum so callers and tests get documented, stable error codes instead of ad-hoc panics.

### Tasks
- [ ] Add an `Error` enum (`#[contracterror]`, `#[repr(u32)]`) with variants: `PassportNotFound`, `WalletAlreadyRegistered`, `RecoveryAddressSameAsWallet`, `NotPassportOwner`, `NotAuthorizedForKeyRotation`, `NotAuthorizedForStatusChange`, `InvalidStatusTransition`, `PassportNotActive`
- [ ] Update `register_patient`, `update_patient_key`, `update_recovery_address`, `deactivate_passport`, and `reactivate_passport` to return `Result<_, Error>` using this enum
- [ ] Update existing unit tests to assert on the specific `Error` variant returned instead of a generic panic
- [ ] Document each error variant with a doc comment explaining when it's returned

### Acceptance Criteria
- No `unwrap()`/`expect()`/bare `panic!` remain in the public contract functions for expected failure cases.
- Every error variant is exercised by at least one test.

### Location
`patient-passport-registry/src/`

### 70. patient-passport-registry: define and emit passport-registered event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/70

### Description
Emit a Soroban contract event whenever `register_patient()` succeeds, so off-chain indexers/backends can react to new passport creations without polling contract state.

### Tasks
- [ ] Define event topics `("passport", "registered")`, data `(passport_id, patient_wallet_address, created_at)`
- [ ] Publish the event via `env.events().publish(...)` at the end of `register_patient()`, only after the duplicate-wallet check has passed
- [ ] Document the event schema in an `events.rs` module or in the contract's rustdoc
- [ ] Add a test asserting the event is published with the expected topics/data using the Soroban testutils event assertions

### Acceptance Criteria
- Event is emitted on every successful `register_patient()` call and covered by a test.

### Location
`patient-passport-registry/src/`

### 71. patient-passport-registry: define and emit key-rotated event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/71

### Description
Emit a contract event whenever `update_patient_key()` succeeds, so wallet-tracking front ends and indexers can update their view of which address currently controls a passport.

### Tasks
- [ ] Define event topic `("passport", "key_rotated")`, data `(passport_id, old_wallet_address, new_wallet_address, rotated_via_recovery: bool)`
- [ ] Publish the event at the end of `update_patient_key()`, only after all validation has passed
- [ ] Document the event schema alongside the passport-registered event from the previous issue
- [ ] Add a test asserting the event is published with correct data for both the self-service and recovery-triggered rotation paths, and is NOT published when the call errors out

### Acceptance Criteria
- Event is emitted on every successful `update_patient_key()` call, with `rotated_via_recovery` set correctly, and covered by tests for both paths.

### Location
`patient-passport-registry/src/`

### 72. patient-passport-registry: define and emit recovery-address-updated event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/72

### Description
Emit a contract event whenever `update_recovery_address()` succeeds, so patients and support tooling can audit changes to who can recover a passport.

### Tasks
- [ ] Define event topic `("passport", "recovery_address_updated")`, data `(passport_id, old_recovery_address, new_recovery_address)`
- [ ] Publish the event at the end of `update_recovery_address()`, only after validation has passed
- [ ] Document the event schema alongside the other passport events
- [ ] Add a test asserting the event is published with correct data, including the case where `new_recovery_address` is `None` (cleared)

### Acceptance Criteria
- Event is emitted on every successful `update_recovery_address()` call and covered by a test, including the clearing case.

### Location
`patient-passport-registry/src/`

### 73. patient-passport-registry: define and emit passport-status-changed event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/73

### Description
Emit a contract event whenever `deactivate_passport()` or `reactivate_passport()` succeeds, carrying both the previous and new status so downstream systems (e.g. provider dashboards checking passport standing) can react immediately.

### Tasks
- [ ] Define event topic `("passport", "status_changed")`, data `(passport_id, old_status, new_status, changed_by: Address)`
- [ ] Publish the event at the end of both `deactivate_passport()` and `reactivate_passport()`, only after transition validation has passed
- [ ] Document the event schema alongside the other passport events
- [ ] Add tests asserting the event is published for both deactivation and reactivation paths, and NOT published for rejected/invalid transitions

### Acceptance Criteria
- Event is emitted on every successful status change and covered by tests for both directions.

### Location
`patient-passport-registry/src/`

### 74. patient-passport-registry: unit test suite for authorization and recovery flows

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/74

### Description
Consolidate and extend authorization coverage into a dedicated test module, focused specifically on the recovery-address trust model, beyond the individual tests added alongside each function.

### Tasks
- [ ] Test: a passport registered with `recovery_address = None` rejects any `update_patient_key` call from an address other than the current wallet
- [ ] Test: after `update_patient_key` is used via the recovery path, the new wallet becomes the only address that can perform subsequent self-service key rotations (the old wallet is no longer authorized)
- [ ] Test: `update_recovery_address` cannot be called by the current `recovery_address` — only by `patient_wallet_address`
- [ ] Test: a caller with no relationship to a passport (neither wallet nor recovery address) is rejected by every state-changing function (`update_patient_key`, `update_recovery_address`, `deactivate_passport`, `reactivate_passport`)

### Acceptance Criteria
- All new tests pass and are added to the crate's existing test module/suite.

### Location
`patient-passport-registry/src/`

### 75. patient-passport-registry: integration test suite covering the full passport lifecycle

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/75

### Description
Add an end-to-end integration test that exercises the full lifecycle through the public contract API, complementing the unit tests added alongside each individual function.

### Tasks
- [ ] Add `patient-passport-registry/tests/` (or extend `src/test.rs`) with a scenario: patient registers a passport with a recovery address → `get_passport` and `get_passport_by_wallet` both resolve it correctly
- [ ] Add a scenario: patient loses their wallet, recovery address rotates the key via `update_patient_key` → old wallet can no longer act on the passport, new wallet can
- [ ] Add a scenario: passport is suspended → key rotation and recovery-address updates are blocked → passport is reactivated → those operations succeed again
- [ ] Add a scenario: passport is revoked → no further status changes, key rotations, or recovery updates succeed
- [ ] Assert the correct sequence of emitted events for each scenario

### Acceptance Criteria
- All scenarios pass under `cargo test` and run as part of CI.

### Location
`patient-passport-registry/` (tests may live in `src/test.rs` or a `tests/` directory, matching the convention chosen in the scaffolding issue)

### 76. patient-passport-registry: rustdoc and README update with build/test instructions

**Labels:** documentation

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/76

### Description
Update `patient-passport-registry/README.md` with concrete build/test instructions and rustdoc coverage for the public contract API, now that the contract is implemented.

### Tasks
- [ ] Add a "Build & Test" section with the exact `cargo build --target wasm32-unknown-unknown --release` and `cargo test` commands
- [ ] Document the final public function signatures (`register_patient`, `update_patient_key`, `update_recovery_address`, `deactivate_passport`, `reactivate_passport`, `get_passport`, `get_passport_by_wallet`) matching what was actually implemented, replacing the current illustrative-only list
- [ ] Add rustdoc comments (`///`) to every `#[contractimpl]` function explaining parameters, return values, and error conditions
- [ ] Document the emitted event schemas (topics + data) in the README or a linked `events.rs` doc comment block, and document the `PassportStatus` transition rules and the recovery-address trust model

### Acceptance Criteria
- `cargo doc` builds without warnings for missing docs on public items.
- README accurately reflects the implemented API rather than the original design sketch.

### Location
`patient-passport-registry/`

### 77. patient-passport-registry: wire up CI workflow for build, test, and lint

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/77

### Description
Add a CI workflow so every PR touching `patient-passport-registry/` is automatically built, tested, and linted, matching the CI coverage set up for the other LockA contract crates.

### Tasks
- [ ] Add (or extend the existing) GitHub Actions workflow to also run on PRs touching `patient-passport-registry/**`
- [ ] Steps: install Rust toolchain + `wasm32-unknown-unknown` target, `cargo build --target wasm32-unknown-unknown --release`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- [ ] Reuse Cargo registry/build artifact caching already configured for the workspace
- [ ] Verify the workflow passes on a test PR before merging

### Acceptance Criteria
- CI workflow runs automatically on PRs touching this crate and fails the check if build, tests, clippy, or fmt fail.

### Location
`.github/workflows/` (workflow scoped to `patient-passport-registry/`)


## provider-registry

Build-out of the Provider Registry contract (`provider-registry/`), including tests and events.

### 78. provider-registry: scaffold Soroban contract crate

**Labels:** good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/78

### Description
Set up the initial Rust/Soroban crate for the Provider Registry contract inside `provider-registry/`, matching the layout used by `consent-access-manager/`, `device-data-attestation/`, `medical-record-registry/`, and `patient-passport-registry/`.

### Tasks
- [ ] Add `Cargo.toml` for the `provider-registry` crate (crate-type `cdylib` + `lib`, `soroban-sdk` dependency, `dev-dependencies` for testutils)
- [ ] Add `src/lib.rs` with an empty `#[contract]` struct (`ProviderRegistry`) and `#[contractimpl]` block
- [ ] Add `src/test.rs` module wired up with an empty/placeholder test that compiles
- [ ] Ensure `cargo build --target wasm32-unknown-unknown --release` succeeds from inside `provider-registry/`
- [ ] Add the crate to the workspace root `Cargo.toml`

### Acceptance Criteria
- `cargo test` and `cargo build --target wasm32-unknown-unknown --release` both succeed inside `provider-registry/`.
- No contract logic is implemented yet — this issue only creates the buildable skeleton other issues will build on.

### Location
All work happens inside `provider-registry/`.

### 79. provider-registry: define ProviderType enum

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/79

### Description
Define the `ProviderType` enum representing the categories of healthcare provider this registry can hold, per `provider-registry/README.md`.

### Tasks
- [ ] Add a `ProviderType` enum with variants: `Hospital`, `Clinic`, `Doctor`, `Laboratory`, `Pharmacy`, `InsuranceCompany`, `PublicHealthAgency`
- [ ] Derive the Soroban `contracttype` macro plus `Clone`, `Debug`, `PartialEq`, `Eq` so it can be stored/passed through contract calls
- [ ] Document each variant with a short doc comment
- [ ] Add a unit test asserting the enum round-trips through `Env` storage (set/get)

### Acceptance Criteria
- `ProviderType` compiles, is exported from the crate, and is covered by at least one storage round-trip test.

### Location
`provider-registry/src/`

### 80. provider-registry: define ProviderStatus enum and status transition rules

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/80

### Description
Define the `ProviderStatus` enum representing a provider's verification lifecycle, and document the allowed transitions before any function implements them.

### Tasks
- [ ] Add a `ProviderStatus` enum with variants: `Pending`, `Verified`, `Suspended`, `Revoked`
- [ ] Derive the Soroban `contracttype` macro plus `Clone`, `Debug`, `PartialEq`, `Eq`
- [ ] Document the allowed transitions: `Pending -> Verified`, `Pending -> Revoked` (rejecting an unverified applicant), `Verified -> Suspended`, `Verified -> Revoked`, `Suspended -> Verified` (reinstatement), `Suspended -> Revoked`; `Revoked` is terminal (no transitions out of it)
- [ ] Add a `fn is_valid_transition(from: &ProviderStatus, to: &ProviderStatus) -> bool` helper with unit tests covering every legal and illegal transition pair

### Acceptance Criteria
- `ProviderStatus` compiles and is exported from the crate.
- `is_valid_transition` is unit tested for all 16 combinations of `(from, to)` pairs.

### Location
`provider-registry/src/`

### 81. provider-registry: define Provider data model and storage keys

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/81

### Description
Define the on-chain data structure for a registered healthcare provider and the storage key scheme used to persist it, based on the fields listed in `provider-registry/README.md` (`provider_id`, `provider_wallet_address`, `provider_type`, `license_hash`, `country`, `status`, `verified_by`).

### Tasks
- [ ] Add a `Provider` struct (`#[contracttype]`) with fields: `provider_id: u64`, `provider_wallet_address: Address`, `provider_type: ProviderType`, `license_hash: BytesN<32>`, `country: Symbol`, `status: ProviderStatus`, `verified_by: Option<Address>`, `registered_at: u64`
- [ ] Define a `DataKey` enum (`#[contracttype]`) covering `Provider(u64)`, a `WalletIndex(Address)` reverse-lookup key (wallet address -> provider_id), and a monotonic `NextProviderId` counter key
- [ ] Add storage helper functions `read_provider`, `write_provider`, `next_provider_id`, `read_provider_id_by_wallet` in a `storage.rs` module
- [ ] Add unit tests for the storage helpers (write then read, counter increments, missing provider lookup)

### Acceptance Criteria
- Storage helpers compile and are unit tested independently of the public contract API.

### Location
`provider-registry/src/`

### 82. provider-registry: implement initialize() and admin storage

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/82

### Description
Provider verification, suspension, and revocation must be gated to a trusted verifying authority (e.g. a LockA medical-board admin address), not to arbitrary callers. Add a one-time contract initialization step that sets this admin.

### Tasks
- [ ] Add `initialize(env, admin: Address)`, callable exactly once
- [ ] Require `admin.require_auth()`
- [ ] Store the admin under a dedicated `DataKey::Admin` storage slot
- [ ] Reject a second call to `initialize()` with an `AlreadyInitialized` error
- [ ] Add a `read_admin(env) -> Address` helper used by every admin-gated function added in later issues; panics or returns `NotInitialized` if the contract has not been initialized yet

### Acceptance Criteria
- Unit tests cover: successful initialization, rejection of a second `initialize()` call, admin-gated helper correctly returns the stored admin.

### Location
`provider-registry/src/`

### 83. provider-registry: implement transfer_admin()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/83

### Description
Implement `transfer_admin()` so the current verifying authority can hand off control of the registry (e.g. organizational handover) without redeploying the contract.

### Tasks
- [ ] Add `transfer_admin(env, current_admin: Address, new_admin: Address)`
- [ ] Require `current_admin.require_auth()` and verify it matches the stored admin (`NotAdmin` error otherwise)
- [ ] Persist `new_admin` as the contract's admin
- [ ] Reject `new_admin == current_admin` as a no-op with a clear error, or document why it's allowed as a harmless no-op — pick one behavior and test it

### Acceptance Criteria
- Unit tests cover: successful transfer, rejection when called by a non-admin address, and the chosen no-op behavior for transferring to the same address.

### Location
`provider-registry/src/`

### 84. provider-registry: implement register_provider()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/84

### Description
Implement `register_provider()` so a healthcare organization can self-register in the directory, starting in `Pending` status until an admin verifies it.

### Tasks
- [ ] Add `register_provider(env, provider_wallet_address: Address, provider_type: ProviderType, license_hash: BytesN<32>, country: Symbol) -> u64` returning the new `provider_id`
- [ ] Require `provider_wallet_address.require_auth()` so only the wallet owner can self-register
- [ ] Persist the `Provider` with `status = ProviderStatus::Pending`, `verified_by = None`, and `registered_at = env.ledger().timestamp()`
- [ ] Populate the `WalletIndex` reverse-lookup entry for the wallet address

### Acceptance Criteria
- Unit tests cover: successful registration, auth failure when not signed by the provider wallet, correct default `status`/`verified_by`/`registered_at` values on the stored provider.

### Location
`provider-registry/src/`

### 85. provider-registry: reject duplicate wallet address registration

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/85

### Description
Harden `register_provider()` so the same Stellar wallet address cannot be linked to more than one provider entry, keeping the `WalletIndex` reverse lookup unambiguous.

### Tasks
- [ ] Before persisting, check whether `provider_wallet_address` already has an entry in the `WalletIndex`
- [ ] Reject the call with a `WalletAlreadyRegistered` error if a match is found
- [ ] Add a unit test asserting a second `register_provider()` call with the same wallet address fails, including when the first registration is `Revoked`

### Acceptance Criteria
- Duplicate wallet registration is rejected with a clear, typed error and covered by a test.

### Location
`provider-registry/src/`

### 86. provider-registry: implement verify_provider()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/86

### Description
Implement `verify_provider()` so the registry admin can approve a `Pending` provider application, marking it as verified and eligible to request patient access elsewhere in the platform.

### Tasks
- [ ] Add `verify_provider(env, admin: Address, provider_id: u64)`
- [ ] Require `admin.require_auth()` and verify it matches the stored contract admin (`NotAdmin` error otherwise)
- [ ] Validate the transition is `Pending -> Verified` using the shared `is_valid_transition` helper (`InvalidStatusTransition` otherwise, e.g. re-verifying an already-verified or revoked provider)
- [ ] Set `status = ProviderStatus::Verified` and `verified_by = Some(admin)`

### Acceptance Criteria
- Unit tests cover: successful verification, rejection when called by a non-admin address, rejection when the provider is not currently `Pending`, correct `verified_by` value on success.

### Location
`provider-registry/src/`

### 87. provider-registry: implement reinstate_provider()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/87

### Description
Implement `reinstate_provider()` so an admin can restore a `Suspended` provider back to `Verified` status, without altering the original `verified_by` record from when it was first approved.

### Tasks
- [ ] Add `reinstate_provider(env, admin: Address, provider_id: u64)`
- [ ] Require `admin.require_auth()` and verify it matches the stored contract admin
- [ ] Validate the transition is `Suspended -> Verified` using the shared `is_valid_transition` helper
- [ ] Set `status = ProviderStatus::Verified`, leaving `verified_by` unchanged

### Acceptance Criteria
- Unit tests cover: successful reinstatement, rejection when called by a non-admin address, rejection when the provider is not currently `Suspended`, `verified_by` is preserved from the original verification.

### Location
`provider-registry/src/`

### 88. provider-registry: implement suspend_provider()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/88

### Description
Implement `suspend_provider()` so an admin can temporarily suspend a `Verified` provider under investigation, without permanently revoking it.

### Tasks
- [ ] Add `suspend_provider(env, admin: Address, provider_id: u64)`
- [ ] Require `admin.require_auth()` and verify it matches the stored contract admin
- [ ] Validate the transition is `Verified -> Suspended` using the shared `is_valid_transition` helper (`InvalidStatusTransition` for a `Pending` or already-`Suspended`/`Revoked` provider)
- [ ] Set `status = ProviderStatus::Suspended`

### Acceptance Criteria
- Unit tests cover: successful suspension, rejection when called by a non-admin address, rejection when the provider is not currently `Verified`.

### Location
`provider-registry/src/`

### 89. provider-registry: implement revoke_provider()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/89

### Description
Implement `revoke_provider()` so an admin can permanently remove a provider's standing, either to reject a bad-faith `Pending` application or to permanently ban a `Verified`/`Suspended` provider caught misusing access.

### Tasks
- [ ] Add `revoke_provider(env, admin: Address, provider_id: u64)`
- [ ] Require `admin.require_auth()` and verify it matches the stored contract admin
- [ ] Validate the transition to `Revoked` is legal from the provider's current status (`Pending`, `Verified`, or `Suspended`) using the shared `is_valid_transition` helper; reject if already `Revoked`
- [ ] Set `status = ProviderStatus::Revoked`

### Acceptance Criteria
- Unit tests cover: successful revocation from each of `Pending`, `Verified`, and `Suspended`, rejection when called by a non-admin address, rejection when the provider is already `Revoked`.

### Location
`provider-registry/src/`

### 90. provider-registry: implement get_provider()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/90

### Description
Implement `get_provider()` so callers can fetch the full stored provider record by its `provider_id`.

### Tasks
- [ ] Add `get_provider(env, provider_id: u64) -> Option<Provider>`
- [ ] Return `None` for a non-existent `provider_id` rather than panicking
- [ ] Keep the function read-only (no `require_auth()`)

### Acceptance Criteria
- Unit tests cover: lookup of an existing provider returns the exact stored data, lookup of a non-existent provider returns `None`.

### Location
`provider-registry/src/`

### 91. provider-registry: implement get_provider_by_wallet()

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/91

### Description
Implement `get_provider_by_wallet()`, a convenience lookup so callers holding only a Stellar wallet address (e.g. a Freighter-connected provider dashboard) can resolve the associated provider record without already knowing the `provider_id`.

### Tasks
- [ ] Add `get_provider_by_wallet(env, wallet_address: Address) -> Option<Provider>`
- [ ] Resolve the `provider_id` via the `WalletIndex` reverse lookup, then delegate to the same logic as `get_provider`
- [ ] Return `None` if the wallet address has no associated provider
- [ ] Keep the function read-only (no `require_auth()`)

### Acceptance Criteria
- Unit tests cover: lookup by a registered wallet returns the correct provider, lookup by an unregistered wallet returns `None`.

### Location
`provider-registry/src/`

### 92. provider-registry: implement is_provider_verified()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/92

### Description
Implement `is_provider_verified()`, a lightweight read-only gate that other contracts (notably `consent-access-manager`) or off-chain services can call before honoring an access request from a given provider, per the README's "Verifies providers before they can request access" requirement.

### Tasks
- [ ] Add `is_provider_verified(env, provider_id: u64) -> bool`
- [ ] Return `true` only if the provider exists and `status == ProviderStatus::Verified`
- [ ] Return `false` (not an error) for a non-existent `provider_id`, so callers can treat the check uniformly
- [ ] No `require_auth()` needed — this is a public read-only check

### Acceptance Criteria
- Unit tests cover: a `Verified` provider returns true, `Pending`/`Suspended`/`Revoked` providers return false, a non-existent `provider_id` returns false without panicking.

### Location
`provider-registry/src/`

### 93. provider-registry: add contract-level error types

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/93

### Description
Consolidate the failure cases introduced across the prior implementation issues into a single, typed `#[contracterror]` enum so callers and tests get documented, stable error codes instead of ad-hoc panics.

### Tasks
- [ ] Add an `Error` enum (`#[contracterror]`, `#[repr(u32)]`) with variants: `NotInitialized`, `AlreadyInitialized`, `NotAdmin`, `ProviderNotFound`, `WalletAlreadyRegistered`, `InvalidStatusTransition`
- [ ] Update `initialize`, `transfer_admin`, `register_provider`, `verify_provider`, `reinstate_provider`, `suspend_provider`, and `revoke_provider` to return `Result<_, Error>` using this enum
- [ ] Update existing unit tests to assert on the specific `Error` variant returned instead of a generic panic
- [ ] Document each error variant with a doc comment explaining when it's returned

### Acceptance Criteria
- No `unwrap()`/`expect()`/bare `panic!` remain in the public contract functions for expected failure cases.
- Every error variant is exercised by at least one test.

### Location
`provider-registry/src/`

### 94. provider-registry: define and emit provider-registered event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/94

### Description
Emit a Soroban contract event whenever `register_provider()` succeeds, so off-chain indexers/backends (and admin review dashboards) can react to new provider applications without polling contract state.

### Tasks
- [ ] Define event topics `("provider", "registered")`, data `(provider_id, provider_wallet_address, provider_type, country)`
- [ ] Publish the event via `env.events().publish(...)` at the end of `register_provider()`, only after the duplicate-wallet check has passed
- [ ] Document the event schema in an `events.rs` module or in the contract's rustdoc
- [ ] Add a test asserting the event is published with the expected topics/data using the Soroban testutils event assertions

### Acceptance Criteria
- Event is emitted on every successful `register_provider()` call and covered by a test.

### Location
`provider-registry/src/`

### 95. provider-registry: define and emit provider-verified event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/95

### Description
Emit a contract event whenever `verify_provider()` succeeds, so downstream systems (including `consent-access-manager` indexers) know a provider is now eligible to request patient access.

### Tasks
- [ ] Define event topic `("provider", "verified")`, data `(provider_id, provider_wallet_address, verified_by)`
- [ ] Publish the event at the end of `verify_provider()`, only after transition validation has passed
- [ ] Document the event schema alongside the provider-registered event from the previous issue
- [ ] Add a test asserting the event is published with correct data on success, and NOT published when the call errors out

### Acceptance Criteria
- Event is emitted on every successful `verify_provider()` call and covered by a test.

### Location
`provider-registry/src/`

### 96. provider-registry: define and emit provider-status-changed event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/96

### Description
Emit a contract event whenever `suspend_provider()`, `reinstate_provider()`, or `revoke_provider()` succeeds, carrying both the previous and new status so provider dashboards and access-control indexers can react immediately to a change in standing.

### Tasks
- [ ] Define event topic `("provider", "status_changed")`, data `(provider_id, old_status, new_status, changed_by: Address)`
- [ ] Publish the event at the end of `suspend_provider()`, `reinstate_provider()`, and `revoke_provider()`, only after transition validation has passed
- [ ] Document the event schema alongside the other provider events
- [ ] Add tests asserting the event is published for suspension, reinstatement, and revocation paths, and NOT published for rejected/invalid transitions

### Acceptance Criteria
- Event is emitted on every successful status change and covered by tests for all three transitions.

### Location
`provider-registry/src/`

### 97. provider-registry: define and emit admin-transferred event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/97

### Description
Emit a contract event whenever `transfer_admin()` succeeds, so any off-chain tooling that trusts the current admin address (e.g. an ops dashboard) can detect a handover immediately.

### Tasks
- [ ] Define event topic `("provider_registry", "admin_transferred")`, data `(old_admin, new_admin)`
- [ ] Publish the event at the end of `transfer_admin()`
- [ ] Document the event schema alongside the other events
- [ ] Add a test asserting the event is published with correct data on a successful transfer

### Acceptance Criteria
- Event is emitted on every successful `transfer_admin()` call and covered by a test.

### Location
`provider-registry/src/`

### 98. provider-registry: unit test suite for admin authorization across all admin-gated functions

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/98

### Description
Consolidate and extend authorization coverage into a dedicated test module, confirming every admin-gated function correctly rejects non-admin callers, including a provider attempting to self-verify.

### Tasks
- [ ] Test: a provider cannot call `verify_provider`, `suspend_provider`, `reinstate_provider`, or `revoke_provider` on its own `provider_id`, even with a valid signature from its own wallet
- [ ] Test: after `transfer_admin()`, the old admin can no longer call any admin-gated function, and the new admin can
- [ ] Test: calling any admin-gated function before `initialize()` has been called fails with `NotInitialized` rather than panicking unpredictably
- [ ] Test: an arbitrary unrelated address (not the admin, not the provider) is rejected by every admin-gated function

### Acceptance Criteria
- All new tests pass and are added to the crate's existing test module/suite.

### Location
`provider-registry/src/`

### 99. provider-registry: integration test suite covering the full provider lifecycle

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/99

### Description
Add an end-to-end integration test that exercises the full lifecycle through the public contract API, complementing the unit tests added alongside each individual function.

### Tasks
- [ ] Add `provider-registry/tests/` (or extend `src/test.rs`) with a scenario: admin initializes the contract → provider registers → `is_provider_verified` returns false → admin verifies the provider → `is_provider_verified` returns true
- [ ] Add a scenario: verified provider is suspended → `is_provider_verified` returns false → admin reinstates it → `is_provider_verified` returns true again
- [ ] Add a scenario: provider is revoked from `Suspended` → no further verification, suspension, or reinstatement succeeds
- [ ] Add a scenario: a `Pending` application is revoked directly (rejected) without ever passing through `Verified`
- [ ] Assert the correct sequence of emitted events for each scenario

### Acceptance Criteria
- All scenarios pass under `cargo test` and run as part of CI.

### Location
`provider-registry/` (tests may live in `src/test.rs` or a `tests/` directory, matching the convention chosen in the scaffolding issue)

### 100. provider-registry: rustdoc and README update with build/test instructions

**Labels:** documentation

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/100

### Description
Update `provider-registry/README.md` with concrete build/test instructions and rustdoc coverage for the public contract API, now that the contract is implemented.

### Tasks
- [ ] Add a "Build & Test" section with the exact `cargo build --target wasm32-unknown-unknown --release` and `cargo test` commands
- [ ] Document the final public function signatures (`initialize`, `transfer_admin`, `register_provider`, `verify_provider`, `reinstate_provider`, `suspend_provider`, `revoke_provider`, `get_provider`, `get_provider_by_wallet`, `is_provider_verified`) matching what was actually implemented, replacing the current illustrative-only list
- [ ] Add rustdoc comments (`///`) to every `#[contractimpl]` function explaining parameters, return values, and error conditions
- [ ] Document the emitted event schemas (topics + data) in the README or a linked `events.rs` doc comment block, and document the `ProviderStatus` transition rules and the admin trust model

### Acceptance Criteria
- `cargo doc` builds without warnings for missing docs on public items.
- README accurately reflects the implemented API rather than the original design sketch.

### Location
`provider-registry/`

### 101. provider-registry: wire up CI workflow for build, test, and lint

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/101

### Description
Add a CI workflow so every PR touching `provider-registry/` is automatically built, tested, and linted, matching the CI coverage set up for the other LockA contract crates.

### Tasks
- [ ] Add (or extend the existing) GitHub Actions workflow to also run on PRs touching `provider-registry/**`
- [ ] Steps: install Rust toolchain + `wasm32-unknown-unknown` target, `cargo build --target wasm32-unknown-unknown --release`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- [ ] Reuse Cargo registry/build artifact caching already configured for the workspace
- [ ] Verify the workflow passes on a test PR before merging

### Acceptance Criteria
- CI workflow runs automatically on PRs touching this crate and fails the check if build, tests, clippy, or fmt fail.

### Location
`.github/workflows/` (workflow scoped to `provider-registry/`)


## zk-credential-verifier

Build-out of the ZK Credential Verifier contract (`zk-credential-verifier/`), including tests and events. Uses Stellar Protocol 25 ("X-Ray") BN254 host functions (`bn254_g1_add`, `bn254_g1_mul`, `bn254_multi_pairing_check`, CAP-0074) for on-chain Groth16 proof verification, and Poseidon2 (CAP-0075) for nullifier derivation.

### 102. zk-credential-verifier: scaffold Soroban contract crate

**Labels:** good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/102

### Description
Set up the initial Rust/Soroban crate for the ZK Credential Verifier contract inside `zk-credential-verifier/`, matching the layout used by the other LockA contract directories.

### Tasks
- [ ] Add `Cargo.toml` for the `zk-credential-verifier` crate (crate-type `cdylib` + `lib`, `soroban-sdk` dependency, `dev-dependencies` for testutils)
- [ ] Add `src/lib.rs` with an empty `#[contract]` struct (`ZkCredentialVerifier`) and `#[contractimpl]` block
- [ ] Add `src/test.rs` module wired up with an empty/placeholder test that compiles
- [ ] Ensure `cargo build --target wasm32-unknown-unknown --release` succeeds from inside `zk-credential-verifier/`
- [ ] Add the crate to the workspace root `Cargo.toml`

### Acceptance Criteria
- `cargo test` and `cargo build --target wasm32-unknown-unknown --release` both succeed inside `zk-credential-verifier/`.
- No contract logic is implemented yet — this issue only creates the buildable skeleton other issues will build on.

### Location
All work happens inside `zk-credential-verifier/`.

### 103. zk-credential-verifier: research spike — confirm Protocol 25 (X-Ray) BN254/Poseidon host function bindings

**Labels:** documentation

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/103

### Description
This contract depends on the BN254 and Poseidon/Poseidon2 host functions introduced in Stellar Protocol 25 ("X-Ray", CAP-0074 and CAP-0075: `bn254_g1_add`, `bn254_g1_mul`, `bn254_multi_pairing_check`, plus the Poseidon/Poseidon2 permutations). As of this writing, the public announcement and upgrade guide describe these at a high level but do not publish exact Rust function signatures. Before any cryptography issue in this epic can be implemented correctly, we need to pin down the real API surface.

### Tasks
- [ ] Identify the minimum `soroban-sdk` (and `stellar-cli` / RPC) version that exposes `Crypto::bn254_*` and `Crypto::poseidon*` bindings in Rust, and confirm it is installable for this workspace
- [ ] Document the exact Rust method signatures for `bn254_g1_add`, `bn254_g1_mul`, `bn254_multi_pairing_check`, and the Poseidon2 permutation call, including expected point/field encodings (compressed vs. uncompressed, byte lengths)
- [ ] Confirm whether BN254 G2 arithmetic (needed for the `B` component of a Groth16 proof and the VK's `beta`, `gamma`, `delta` points) is exposed directly, or whether G2 points must be treated as opaque bytes consumed only by `bn254_multi_pairing_check`
- [ ] Confirm Testnet/Futurenet currently runs Protocol 25 or later, and record how to point local `cargo test`/`soroban-cli` tooling at a compatible environment
- [ ] Write up findings as a short doc (e.g. `zk-credential-verifier/docs/bn254-host-functions.md`) that every later issue in this epic can reference

### Acceptance Criteria
- A written reference document exists in the crate describing the confirmed function signatures and encodings, replacing any placeholder assumptions used in the data-model issues below.
- Any assumption from this spike that later turns out to be wrong is corrected in that same document, not silently worked around.

### Location
`zk-credential-verifier/`

### 104. zk-credential-verifier: define CredentialType enum

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/104

### Description
Define the `CredentialType` enum representing the categories of zero-knowledge claim this contract can verify, per the proof types documented in `zk-credential-verifier/README.md` and the platform documentation's ZK proof catalogue.

### Tasks
- [ ] Add a `CredentialType` enum with variants: `VaccinationStatus`, `InsuranceEligibility`, `AgeThreshold`, `ProgramEligibility`, `MedicalScreeningCompletion`, `PassportValidity`, `CredentialOwnership`
- [ ] Derive the Soroban `contracttype` macro plus `Clone`, `Debug`, `PartialEq`, `Eq` so it can be stored/passed through contract calls
- [ ] Document each variant with a short doc comment describing the real-world claim it represents
- [ ] Add a unit test asserting the enum round-trips through `Env` storage (set/get)

### Acceptance Criteria
- `CredentialType` compiles, is exported from the crate, and is covered by at least one storage round-trip test.

### Location
`zk-credential-verifier/src/`

### 105. zk-credential-verifier: define SubmissionStatus and ClaimStatus enums

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/105

### Description
Define the two status enums used across the proof pipeline: `SubmissionStatus` for a raw proof submission's cryptographic outcome, and `ClaimStatus` for a durable, queryable claim's administrative state.

### Tasks
- [ ] Add a `SubmissionStatus` enum with variants: `Pending`, `Verified`, `Failed` — set once by `verify_proof_attestation` and never changed afterward (a submission's cryptographic result is immutable history)
- [ ] Add a `ClaimStatus` enum with variants: `Active`, `Revoked` — expiry is handled separately via a stored `expires_at` timestamp checked at read time, not as its own status variant (see the later lazy-expiry issue)
- [ ] Derive the Soroban `contracttype` macro plus `Clone`, `Debug`, `PartialEq`, `Eq` on both
- [ ] Add unit tests asserting both enums round-trip through `Env` storage

### Acceptance Criteria
- Both enums compile, are exported from the crate, and are covered by storage round-trip tests.

### Location
`zk-credential-verifier/src/`

### 106. zk-credential-verifier: define Groth16VerifyingKey data model and per-credential-type storage

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/106

### Description
Define the on-chain representation of a Groth16 verifying key over BN254, one of which must be registered per `CredentialType` before proofs of that type can be verified. Field encodings should follow the reference document produced by the research-spike issue.

### Tasks
- [ ] Add a `Groth16VerifyingKey` struct (`#[contracttype]`) with fields: `alpha_g1: BytesN<64>`, `beta_g2: BytesN<128>`, `gamma_g2: BytesN<128>`, `delta_g2: BytesN<128>`, `ic: Vec<BytesN<64>>` (the Groth16 "IC" linear-combination basis, one G1 point per public input plus one constant term), `version: u32` (incremented on rotation)
- [ ] Adjust the byte lengths above if the research-spike issue's findings differ from the assumed uncompressed BN254 point encoding
- [ ] Define a `DataKey::VerifyingKey(CredentialType)` storage key
- [ ] Add storage helper functions `read_verifying_key`, `write_verifying_key` in `storage.rs`
- [ ] Add unit tests for the storage helpers (write then read, missing key lookup for an unregistered `CredentialType`)

### Acceptance Criteria
- Storage helpers compile and are unit tested independently of the public contract API.

### Location
`zk-credential-verifier/src/`

### 107. zk-credential-verifier: define Groth16Proof data model

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/107

### Description
Define the on-chain representation of a single Groth16 proof (the `A`, `B`, `C` group elements) submitted by a client, plus the public inputs the proof is checked against.

### Tasks
- [ ] Add a `Groth16Proof` struct (`#[contracttype]`) with fields: `a: BytesN<64>` (G1), `b: BytesN<128>` (G2), `c: BytesN<64>` (G1)
- [ ] Add a `PublicInputs` type alias or newtype wrapping `Vec<BytesN<32>>` (BN254 scalar field elements, one per public signal the circuit exposes)
- [ ] Adjust byte lengths per the research-spike issue's confirmed encoding if it differs from the assumption above
- [ ] Add a unit test constructing a `Groth16Proof` and `PublicInputs` and asserting they round-trip through `Env` storage/serialization

### Acceptance Criteria
- `Groth16Proof` and `PublicInputs` compile and are exported from the crate, covered by at least one round-trip test.

### Location
`zk-credential-verifier/src/`

### 108. zk-credential-verifier: define ProofSubmission data model and storage keys

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/108

### Description
Define the on-chain record of a single proof submission, tracking its cryptographic verification outcome independently of whether it later becomes a durable claim.

### Tasks
- [ ] Add a `ProofSubmission` struct (`#[contracttype]`) with fields: `submission_id: u64`, `passport_id: Address`, `credential_type: CredentialType`, `proof: Groth16Proof`, `public_inputs: Vec<BytesN<32>>`, `status: SubmissionStatus`, `submitted_at: u64`
- [ ] Define storage keys `Submission(u64)`, a per-patient submission index, and a monotonic `NextSubmissionId` counter key
- [ ] Add storage helper functions `read_submission`, `write_submission`, `next_submission_id` in `storage.rs`
- [ ] Add unit tests for the storage helpers (write then read, counter increments)

### Acceptance Criteria
- Storage helpers compile and are unit tested independently of the public contract API.

### Location
`zk-credential-verifier/src/`

### 109. zk-credential-verifier: define VerifiedClaim data model and storage keys

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/109

### Description
Define the on-chain record of a durable, queryable verified claim, based on the fields listed in `zk-credential-verifier/README.md` (`proof_id`, `passport_id`, `credential_type`, `proof_hash`, `verifier`, `verified_result`, `expires_at`).

### Tasks
- [ ] Add a `VerifiedClaim` struct (`#[contracttype]`) with fields: `claim_id: u64`, `passport_id: Address`, `credential_type: CredentialType`, `proof_hash: BytesN<32>`, `verifier: Address`, `verified_result: bool`, `status: ClaimStatus`, `created_at: u64`, `expires_at: u64`
- [ ] Define storage keys `Claim(u64)`, a per-patient claim index, and a monotonic `NextClaimId` counter key
- [ ] Add storage helper functions `read_claim`, `write_claim`, `next_claim_id` in `storage.rs`
- [ ] Add unit tests for the storage helpers (write then read, counter increments, missing claim lookup)

### Acceptance Criteria
- Storage helpers compile and are unit tested independently of the public contract API.

### Location
`zk-credential-verifier/src/`

### 110. zk-credential-verifier: implement initialize() and admin storage

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/110

### Description
Registering and rotating Groth16 verifying keys, and revoking claims administratively, must be gated to a trusted authority (e.g. whoever audited and trusts the underlying ZK circuits). Add a one-time contract initialization step that sets this admin, mirroring the pattern used in `provider-registry`.

### Tasks
- [ ] Add `initialize(env, admin: Address)`, callable exactly once
- [ ] Require `admin.require_auth()`
- [ ] Store the admin under a dedicated `DataKey::Admin` storage slot
- [ ] Reject a second call to `initialize()` with an `AlreadyInitialized` error
- [ ] Add a `read_admin(env) -> Address` helper used by every admin-gated function added in later issues

### Acceptance Criteria
- Unit tests cover: successful initialization, rejection of a second `initialize()` call, admin-gated helper correctly returns the stored admin.

### Location
`zk-credential-verifier/src/`

### 111. zk-credential-verifier: implement transfer_admin()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/111

### Description
Implement `transfer_admin()` so the current verifying-key authority can hand off control of the registry without redeploying the contract.

### Tasks
- [ ] Add `transfer_admin(env, current_admin: Address, new_admin: Address)`
- [ ] Require `current_admin.require_auth()` and verify it matches the stored admin (`NotAdmin` error otherwise)
- [ ] Persist `new_admin` as the contract's admin
- [ ] Add a unit test covering successful transfer and rejection when called by a non-admin address

### Acceptance Criteria
- Unit tests cover: successful transfer, rejection when called by a non-admin address.

### Location
`zk-credential-verifier/src/`

### 112. zk-credential-verifier: implement register_verifying_key()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/112

### Description
Implement `register_verifying_key()` so the admin can register the Groth16 verifying key for a `CredentialType`'s circuit, or rotate it if the circuit is upgraded (e.g. a bug fix or a new constraint set).

### Tasks
- [ ] Add `register_verifying_key(env, admin: Address, credential_type: CredentialType, verifying_key: Groth16VerifyingKey)`
- [ ] Require `admin.require_auth()` and verify it matches the stored admin
- [ ] On rotation (a key already exists for this `credential_type`), increment `verifying_key.version` rather than allowing the caller to set an arbitrary version, and require the new `ic` vector to be non-empty
- [ ] Persist the verifying key under `DataKey::VerifyingKey(credential_type)`

### Acceptance Criteria
- Unit tests cover: successful first-time registration, successful rotation with correct version increment, rejection when called by a non-admin address, rejection of an empty `ic` vector.

### Location
`zk-credential-verifier/src/`

### 113. zk-credential-verifier: implement get_verifying_key()

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/113

### Description
Implement `get_verifying_key()` so callers (and off-chain tooling generating proofs) can confirm which verifying key version is currently active for a given `CredentialType`.

### Tasks
- [ ] Add `get_verifying_key(env, credential_type: CredentialType) -> Option<Groth16VerifyingKey>`
- [ ] Return `None` for a `CredentialType` that has no registered key rather than panicking
- [ ] Keep the function read-only (no `require_auth()`)

### Acceptance Criteria
- Unit tests cover: lookup of a registered key returns the exact stored data, lookup of an unregistered credential type returns `None`.

### Location
`zk-credential-verifier/src/`

### 114. zk-credential-verifier: implement submit_proof()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/114

### Description
Implement `submit_proof()` so a patient (or a wallet acting on their behalf) can submit a Groth16 proof and its public inputs for a given `CredentialType`, creating a `Pending` `ProofSubmission` that later issues will cryptographically verify.

### Tasks
- [ ] Add `submit_proof(env, passport_id: Address, credential_type: CredentialType, proof: Groth16Proof, public_inputs: Vec<BytesN<32>>) -> u64` returning the new `submission_id`
- [ ] Require `passport_id.require_auth()`
- [ ] Reject submission if no verifying key is registered for `credential_type` (`VerifyingKeyNotFound` error)
- [ ] Reject submission if `public_inputs.len() + 1 != verifying_key.ic.len()` (`PublicInputLengthMismatch` error) — the Groth16 IC basis must have exactly one more element than the number of public inputs
- [ ] Persist the `ProofSubmission` with `status = SubmissionStatus::Pending` and `submitted_at = env.ledger().timestamp()`, and add it to the patient's submission index

### Acceptance Criteria
- Unit tests cover: successful submission, rejection for a `credential_type` with no registered key, rejection for a public-input count that doesn't match the registered `ic` length, auth failure when not signed by `passport_id`.

### Location
`zk-credential-verifier/src/`

### 115. zk-credential-verifier: implement Groth16 public-input linear combination using BN254 G1 host functions

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/115

### Description
Implement the `vk_x` computation required by Groth16 verification: `vk_x = ic[0] + sum(public_input[i] * ic[i+1])`, using the native `bn254_g1_add` and `bn254_g1_mul` host functions confirmed by the research-spike issue. This is the piece that binds the proof to the specific public inputs being claimed.

### Tasks
- [ ] Add a `compute_vk_x(env: &Env, vk: &Groth16VerifyingKey, public_inputs: &Vec<BytesN<32>>) -> BytesN<64>` function
- [ ] Fold over `public_inputs`, accumulating `vk_x` starting from `vk.ic[0]` via `bn254_g1_mul(ic[i+1], public_input[i])` followed by `bn254_g1_add`
- [ ] Return a typed error (`InvalidPublicInput`) if any public input is not a valid BN254 scalar field element (out of range), rather than letting the host function panic
- [ ] Add unit tests using known BN254 test vectors: a linear combination of two known points with known scalars produces the expected resulting point (independent of any full proof verification)

### Acceptance Criteria
- `compute_vk_x` is unit tested against hand-computed/known-correct BN254 test vectors, not just exercised indirectly through a full proof check.

### Location
`zk-credential-verifier/src/`

### 116. zk-credential-verifier: implement verify_proof_attestation() using bn254_multi_pairing_check

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/116

### Description
Implement `verify_proof_attestation()`, the core cryptographic step: checking the Groth16 pairing equation `e(A, B) = e(alpha, beta) * e(vk_x, gamma) * e(C, delta)` on-chain using `bn254_multi_pairing_check`, against a previously submitted `ProofSubmission`.

### Tasks
- [ ] Add `verify_proof_attestation(env, submission_id: u64) -> bool`
- [ ] Load the `ProofSubmission` and its `credential_type`'s `Groth16VerifyingKey`; reject if the submission is not `Pending` (`SubmissionAlreadyProcessed` error) — attestation is a one-time, idempotent-by-design operation
- [ ] Compute `vk_x` via `compute_vk_x` from the previous issue
- [ ] Build the pairing check inputs `[(A, B), (alpha_g1, beta_g2)^-1-equivalent term, (vk_x, gamma_g2), (C, delta_g2)]` per the standard Groth16 verification equation rearranged for a single multi-pairing call, and invoke `bn254_multi_pairing_check`
- [ ] Update the submission's `status` to `Verified` or `Failed` based on the result, and return that boolean
- [ ] Callable by anyone (no `require_auth()`) — verification is a public, deterministic function of already-public on-chain data

### Acceptance Criteria
- Unit tests cover: a valid proof against its correct verifying key and public inputs verifies as `true` and flips status to `Verified`; an invalid/tampered proof verifies as `false` and flips status to `Failed`; calling attestation twice on the same `submission_id` is rejected with `SubmissionAlreadyProcessed`.

### Location
`zk-credential-verifier/src/`

### 117. zk-credential-verifier: unit test suite — Groth16 verification correctness with real test vectors

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/117

### Description
`verify_proof_attestation()` must be validated against genuine Groth16 proofs, not just structurally-shaped byte arrays, to prove the on-chain pairing check logic is actually correct and not merely "doesn't panic."

### Tasks
- [ ] Using Circom + snarkjs (or Noir, per the Interstellar BLS12-381 bridge project, adapted to BN254), author a trivial test circuit (e.g. "I know `x` such that `x^2 == public_y`") and generate a real BN254 Groth16 verifying key and a valid proof for a known witness
- [ ] Commit the verifying key and proof as fixture files (or hardcoded byte constants) under `zk-credential-verifier/src/test_fixtures.rs`
- [ ] Add a test that registers the fixture verifying key, submits the fixture proof with correct public inputs, and asserts `verify_proof_attestation` returns `true`
- [ ] Add a test that mutates a single byte of the fixture proof and asserts verification returns `false`
- [ ] Document the exact `circom`/`snarkjs` (or Noir) commands used to regenerate the fixtures, so they can be reproduced if the test circuit ever needs to change

### Acceptance Criteria
- At least one genuine, independently-generated Groth16 proof passes verification end-to-end inside a Soroban test, proving the pairing-check wiring is cryptographically correct rather than merely type-checking.

### Location
`zk-credential-verifier/src/`

### 118. zk-credential-verifier: unit test suite — reject invalid and tampered proofs

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/118

### Description
Extend attestation test coverage with adversarial cases beyond the single-byte tamper test in the previous issue, confirming the verifier cannot be tricked by structurally valid but semantically wrong inputs.

### Tasks
- [ ] Test: a valid proof for `credential_type` A submitted against the verifying key for `credential_type` B fails verification
- [ ] Test: a valid proof submitted with public inputs altered (e.g. a different claimed `public_y`) than the ones it was actually generated for fails verification
- [ ] Test: a proof reusing a valid `A`/`C` but a `B` from an unrelated proof fails verification
- [ ] Test: submitting a public-input value that is not a valid BN254 scalar field element (e.g. all-`0xFF` bytes, out of field range) is rejected by `compute_vk_x` with `InvalidPublicInput` rather than causing a host function panic

### Acceptance Criteria
- All four adversarial cases are covered by passing tests.

### Location
`zk-credential-verifier/src/`

### 119. zk-credential-verifier: implement Poseidon2-based nullifier derivation and replay protection

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/119

### Description
Use the Poseidon2 permutation host function (CAP-0075, confirmed by the research-spike issue) to derive a nullifier for each verified claim, and track spent nullifiers so the same underlying credential cannot be used to mint duplicate active claims — a standard ZK privacy-pool pattern, adapted here to stop a patient (or a leaked proof) from re-registering the same credential fact as multiple simultaneous claims.

### Tasks
- [ ] Require circuits to expose a nullifier as one of the Groth16 proof's public inputs (document this contract expectation in a doc comment, cross-referencing the research-spike issue's fixture circuit)
- [ ] Add a `DataKey::SpentNullifier(BytesN<32>)` storage key and `is_nullifier_spent` / `mark_nullifier_spent` helpers
- [ ] In `store_verified_claim()` (see the following issue), derive the nullifier hash via the Poseidon2 host function over `(passport_id, credential_type, public_inputs[nullifier_index])` and reject the call with `NullifierAlreadySpent` if it has been seen before
- [ ] Mark the nullifier spent only after all other validation in `store_verified_claim()` succeeds, so a failed attempt does not burn the nullifier

### Acceptance Criteria
- Unit tests cover: first use of a nullifier succeeds, a second `store_verified_claim()` call reusing the same underlying proof/nullifier is rejected with `NullifierAlreadySpent`, and a failed `store_verified_claim()` call (for an unrelated reason) does not consume the nullifier.

### Location
`zk-credential-verifier/src/`

### 120. zk-credential-verifier: implement store_verified_claim()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/120

### Description
Implement `store_verified_claim()`, which promotes a `Verified` `ProofSubmission` into a durable `VerifiedClaim` that downstream consumers (providers, other LockA contracts) can query, with its own expiry and revocation lifecycle independent of the underlying proof submission.

### Tasks
- [ ] Add `store_verified_claim(env, submission_id: u64, verifier: Address, validity_duration: u64) -> u64` returning the new `claim_id`
- [ ] Reject if the submission's `status != SubmissionStatus::Verified` (`SubmissionNotVerified` error) — only successfully attested submissions may become claims
- [ ] Apply the nullifier check/spend from the previous issue
- [ ] Compute `proof_hash` as a hash (SHA-256 or Poseidon2, per the crate's established hashing convention) over the submission's `proof` and `public_inputs`
- [ ] Persist the `VerifiedClaim` with `status = ClaimStatus::Active`, `created_at = env.ledger().timestamp()`, `expires_at = created_at + validity_duration`, and add it to the patient's claim index

### Acceptance Criteria
- Unit tests cover: successful claim creation from a verified submission, rejection when the submission is still `Pending` or is `Failed`, rejection on nullifier reuse, correct `expires_at` computation.

### Location
`zk-credential-verifier/src/`

### 121. zk-credential-verifier: implement revoke_claim()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/121

### Description
Implement `revoke_claim()` so the admin (or, per design decision, the credential's issuing authority) can invalidate a previously stored claim before its natural expiry — e.g. a vaccination record is later found to be fraudulent, or a program eligibility claim needs to be pulled.

### Tasks
- [ ] Add `revoke_claim(env, admin: Address, claim_id: u64)`
- [ ] Require `admin.require_auth()` and verify it matches the stored contract admin
- [ ] Reject if the claim is already `Revoked` (`ClaimAlreadyRevoked` error)
- [ ] Set `status = ClaimStatus::Revoked`

### Acceptance Criteria
- Unit tests cover: successful revocation, rejection when called by a non-admin address, rejection of revoking an already-revoked claim, rejection for a non-existent `claim_id`.

### Location
`zk-credential-verifier/src/`

### 122. zk-credential-verifier: implement get_verified_claim()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/122

### Description
Implement `get_verified_claim()` so callers can fetch the full stored claim record by its `claim_id`.

### Tasks
- [ ] Add `get_verified_claim(env, claim_id: u64) -> Option<VerifiedClaim>`
- [ ] Return `None` for a non-existent `claim_id` rather than panicking
- [ ] Keep the function read-only (no `require_auth()`)

### Acceptance Criteria
- Unit tests cover: lookup of an existing claim returns the exact stored data, lookup of a non-existent claim returns `None`.

### Location
`zk-credential-verifier/src/`

### 123. zk-credential-verifier: implement get_active_claims_for_patient()

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/123

### Description
Implement `get_active_claims_for_patient()` so a provider or front end can list all currently valid (non-expired, non-revoked) claims for a given patient in one call, using the per-patient claim index built in `store_verified_claim()`.

### Tasks
- [ ] Add `get_active_claims_for_patient(env, passport_id: Address) -> Vec<VerifiedClaim>`
- [ ] Filter out claims that are `Revoked` or whose `expires_at <= env.ledger().timestamp()`, reusing the shared `is_active` helper from the following lazy-expiry issue
- [ ] Keep the function read-only (no `require_auth()`), since results are already scoped to the given `passport_id`
- [ ] Consider and document result-size limits/pagination if a patient's claim count can grow large

### Acceptance Criteria
- Unit tests cover: empty result for a patient with no claims, correct filtering of expired/revoked claims, correct inclusion of active claims across multiple credential types.

### Location
`zk-credential-verifier/src/`

### 124. zk-credential-verifier: implement is_claim_valid() and shared lazy-expiry helper

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/124

### Description
Implement `is_claim_valid()`, a lightweight read-only gate other contracts or off-chain services can call before trusting a claim, and extract a shared expiry-check helper so expiry logic is defined exactly once in the crate (mirroring the pattern established in `consent-access-manager`).

### Tasks
- [ ] Add a shared `is_active(claim: &VerifiedClaim, now: u64) -> bool` helper: `claim.status == ClaimStatus::Active && claim.expires_at > now`
- [ ] Add `is_claim_valid(env, claim_id: u64) -> bool` built on top of `is_active`, returning `false` for a non-existent `claim_id`
- [ ] Update `get_active_claims_for_patient` (previous issue) to use the same shared helper
- [ ] Add unit tests that advance the simulated ledger time (`env.ledger().set_timestamp(...)`) to confirm a claim flips from valid to invalid at the correct `expires_at` boundary

### Acceptance Criteria
- A single shared helper backs every expiry/validity check in the crate; no duplicated expiry logic exists.
- Tests demonstrate the expiry boundary (`valid at expires_at - 1`, `invalid at expires_at`) and immediate invalidation on revocation.

### Location
`zk-credential-verifier/src/`

### 125. zk-credential-verifier: add contract-level error types

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/125

### Description
Consolidate the failure cases introduced across the prior implementation issues into a single, typed `#[contracterror]` enum so callers and tests get documented, stable error codes instead of ad-hoc panics.

### Tasks
- [ ] Add an `Error` enum (`#[contracterror]`, `#[repr(u32)]`) with variants: `NotInitialized`, `AlreadyInitialized`, `NotAdmin`, `VerifyingKeyNotFound`, `InvalidVerifyingKey`, `SubmissionNotFound`, `PublicInputLengthMismatch`, `InvalidPublicInput`, `SubmissionAlreadyProcessed`, `SubmissionNotVerified`, `NullifierAlreadySpent`, `ClaimNotFound`, `ClaimAlreadyRevoked`
- [ ] Update every state-changing function added in earlier issues to return `Result<_, Error>` using this enum
- [ ] Update existing unit tests to assert on the specific `Error` variant returned instead of a generic panic
- [ ] Document each error variant with a doc comment explaining when it's returned

### Acceptance Criteria
- No `unwrap()`/`expect()`/bare `panic!` remain in the public contract functions for expected failure cases.
- Every error variant is exercised by at least one test.

### Location
`zk-credential-verifier/src/`

### 126. zk-credential-verifier: define and emit verifying-key-registered event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/126

### Description
Emit a Soroban contract event whenever `register_verifying_key()` succeeds, so off-chain proof-generation tooling can detect when a new circuit version is expected and stop generating proofs against a stale key.

### Tasks
- [ ] Define event topic `("vk", "registered")`, data `(credential_type, version)`
- [ ] Publish the event at the end of `register_verifying_key()`
- [ ] Document the event schema in an `events.rs` module or the contract's rustdoc
- [ ] Add a test asserting the event is published with the expected topics/data on both first-time registration and rotation

### Acceptance Criteria
- Event is emitted on every successful `register_verifying_key()` call and covered by a test.

### Location
`zk-credential-verifier/src/`

### 127. zk-credential-verifier: define and emit proof-submitted event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/127

### Description
Emit a contract event whenever `submit_proof()` succeeds, so an off-chain relayer/indexer can pick up pending submissions and trigger `verify_proof_attestation()` without polling.

### Tasks
- [ ] Define event topic `("proof", "submitted")`, data `(submission_id, passport_id, credential_type)`
- [ ] Publish the event at the end of `submit_proof()`, only after the verifying-key-existence and public-input-length checks have passed
- [ ] Document the event schema alongside the verifying-key-registered event
- [ ] Add a test asserting the event is published with correct data on success

### Acceptance Criteria
- Event is emitted on every successful `submit_proof()` call and covered by a test.

### Location
`zk-credential-verifier/src/`

### 128. zk-credential-verifier: define and emit proof-verified / proof-verification-failed events

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/128

### Description
Emit distinct contract events for the two possible outcomes of `verify_proof_attestation()`, so indexers can distinguish "checked and valid" from "checked and rejected" without re-reading submission state.

### Tasks
- [ ] Define event topics `("proof", "verified")` and `("proof", "verification_failed")`, data `(submission_id, passport_id, credential_type)`
- [ ] Publish the appropriate event at the end of `verify_proof_attestation()` based on the pairing-check result
- [ ] Document both event schemas alongside the other events in this crate
- [ ] Add tests asserting the correct event fires for a valid proof and for an invalid/tampered proof, reusing the fixtures from the Groth16-correctness test issue

### Acceptance Criteria
- Exactly one of the two events fires on every `verify_proof_attestation()` call, matching the returned boolean, and both paths are covered by tests.

### Location
`zk-credential-verifier/src/`

### 129. zk-credential-verifier: define and emit claim-stored event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/129

### Description
Emit a contract event whenever `store_verified_claim()` succeeds, so providers and other LockA contracts (e.g. a future consent-gated viewer checking eligibility) can react to a new active claim.

### Tasks
- [ ] Define event topic `("claim", "stored")`, data `(claim_id, passport_id, credential_type, expires_at)`
- [ ] Publish the event at the end of `store_verified_claim()`, only after the submission-status check and nullifier-spend have both succeeded
- [ ] Document the event schema alongside the other events in this crate
- [ ] Add a test asserting the event is published with correct data on success, and NOT published when the call errors out

### Acceptance Criteria
- Event is emitted on every successful `store_verified_claim()` call and covered by a test.

### Location
`zk-credential-verifier/src/`

### 130. zk-credential-verifier: define and emit claim-revoked event

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/130

### Description
Emit a contract event whenever `revoke_claim()` succeeds, so any party relying on a claim's validity is promptly notified it has been withdrawn.

### Tasks
- [ ] Define event topic `("claim", "revoked")`, data `(claim_id, passport_id, credential_type, revoked_by: Address)`
- [ ] Publish the event at the end of `revoke_claim()`
- [ ] Document the event schema alongside the other events in this crate
- [ ] Add a test asserting the event is published with correct data on successful revocation

### Acceptance Criteria
- Event is emitted on every successful `revoke_claim()` call and covered by a test.

### Location
`zk-credential-verifier/src/`

### 131. zk-credential-verifier: unit test suite for admin authorization across all admin-gated functions

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/131

### Description
Consolidate and extend authorization coverage into a dedicated test module, confirming every admin-gated function correctly rejects non-admin callers, including a patient attempting to register or rotate a verifying key for their own credential type.

### Tasks
- [ ] Test: a patient cannot call `register_verifying_key` or `revoke_claim`, even for their own claim, with a valid signature from their own wallet
- [ ] Test: after `transfer_admin()`, the old admin can no longer call any admin-gated function, and the new admin can
- [ ] Test: calling any admin-gated function before `initialize()` has been called fails with `NotInitialized`
- [ ] Test: an arbitrary unrelated address (not the admin, not the patient) is rejected by every admin-gated function

### Acceptance Criteria
- All new tests pass and are added to the crate's existing test module/suite.

### Location
`zk-credential-verifier/src/`

### 132. zk-credential-verifier: integration test suite covering the full submit-verify-store-revoke lifecycle

**Labels:** enhancement

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/132

### Description
Add an end-to-end integration test that exercises the full lifecycle through the public contract API, using the real fixture proof and verifying key from the Groth16-correctness test issue.

### Tasks
- [ ] Add `zk-credential-verifier/tests/` (or extend `src/test.rs`) with a scenario: admin registers a verifying key → patient submits a valid fixture proof → `verify_proof_attestation` returns true → `store_verified_claim` creates an active claim → `is_claim_valid` and `get_active_claims_for_patient` both reflect it
- [ ] Add a scenario: the same underlying nullifier is submitted a second time (fresh submission, same proof/nullifier) → `store_verified_claim` is rejected with `NullifierAlreadySpent`, and the first claim remains untouched
- [ ] Add a scenario: admin revokes the claim → `is_claim_valid` returns false immediately, before natural expiry
- [ ] Add a scenario: a claim is left to expire naturally (advance ledger time past `expires_at`) → `is_claim_valid` returns false without an explicit revoke
- [ ] Assert the correct sequence of emitted events for each scenario

### Acceptance Criteria
- All scenarios pass under `cargo test` and run as part of CI.

### Location
`zk-credential-verifier/` (tests may live in `src/test.rs` or a `tests/` directory, matching the convention chosen in the scaffolding issue)

### 133. zk-credential-verifier: rustdoc and README update with build/test instructions

**Labels:** documentation

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/133

### Description
Update `zk-credential-verifier/README.md` with concrete build/test instructions and rustdoc coverage for the public contract API, now that the contract is implemented, including how it uses Stellar Protocol 25 (X-Ray)'s BN254 and Poseidon2 host functions.

### Tasks
- [ ] Add a short "How verification works" section explaining the on-chain Groth16-over-BN254 pairing check (`bn254_multi_pairing_check`), referencing CAP-0074, and the Poseidon2-based nullifier scheme, referencing CAP-0075
- [ ] Add a "Build & Test" section with the exact `cargo build --target wasm32-unknown-unknown --release` and `cargo test` commands, plus the minimum `soroban-sdk`/toolchain version confirmed by the research-spike issue
- [ ] Document the final public function signatures (`initialize`, `transfer_admin`, `register_verifying_key`, `get_verifying_key`, `submit_proof`, `verify_proof_attestation`, `store_verified_claim`, `revoke_claim`, `get_verified_claim`, `get_active_claims_for_patient`, `is_claim_valid`) matching what was actually implemented, replacing the current illustrative-only list
- [ ] Add rustdoc comments (`///`) to every `#[contractimpl]` function explaining parameters, return values, and error conditions
- [ ] Document the emitted event schemas (topics + data) in the README or a linked `events.rs` doc comment block

### Acceptance Criteria
- `cargo doc` builds without warnings for missing docs on public items.
- README accurately reflects the implemented API and explains the cryptography in plain terms for contributors unfamiliar with Groth16/BN254.

### Location
`zk-credential-verifier/`

### 134. zk-credential-verifier: wire up CI workflow pinned to a Protocol-25-capable Soroban toolchain

**Labels:** enhancement, good first issue

**Status:** created — https://github.com/LockA-Medical-Passport/LockA-Smart-Contracts/issues/134

### Description
Add a CI workflow so every PR touching `zk-credential-verifier/` is automatically built, tested, and linted, matching the CI coverage set up for the other LockA contract crates — with the added requirement of pinning a toolchain that actually exposes the BN254/Poseidon2 host functions this crate depends on.

### Tasks
- [ ] Add (or extend the existing) GitHub Actions workflow to also run on PRs touching `zk-credential-verifier/**`
- [ ] Pin the Rust toolchain and `soroban-sdk`/`stellar-cli` versions to the ones confirmed by the research-spike issue as supporting Protocol 25 host functions
- [ ] Steps: install Rust toolchain + `wasm32-unknown-unknown` target, `cargo build --target wasm32-unknown-unknown --release`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt --check`
- [ ] Reuse Cargo registry/build artifact caching already configured for the workspace
- [ ] Verify the workflow passes on a test PR before merging

### Acceptance Criteria
- CI workflow runs automatically on PRs touching this crate, uses a pinned Protocol-25-capable toolchain, and fails the check if build, tests, clippy, or fmt fail.

### Location
`.github/workflows/` (workflow scoped to `zk-credential-verifier/`)

