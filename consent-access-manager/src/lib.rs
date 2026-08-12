#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

mod error;
mod storage;
mod types;

pub use error::Error;
pub use storage::AccessRequest;
pub use types::RecordScope;

#[contract]
pub struct ConsentAccessManager;

#[contractimpl]
impl ConsentAccessManager {
    /// Records a provider's request for access to a patient's records.
    ///
    /// The request starts unapproved and unexpired: `approved` and `revoked`
    /// are `false`, and `expires_at` stays `0` until the patient approves it
    /// and the requested `duration_seconds` window begins. Returns the
    /// `access_id` allocated to the new request, which the patient uses to
    /// approve or reject it.
    ///
    /// Only `provider_id` can create the request. Requests for a zero
    /// duration are rejected with [`Error::InvalidDuration`].
    pub fn request_access(
        env: Env,
        provider_id: Address,
        passport_id: Address,
        record_scope: RecordScope,
        duration_seconds: u64,
    ) -> Result<u64, Error> {
        provider_id.require_auth();

        if duration_seconds == 0 {
            return Err(Error::InvalidDuration);
        }

        let access_id = storage::next_access_id(&env);
        storage::write_access_request(
            &env,
            &AccessRequest {
                access_id,
                passport_id,
                provider_id,
                record_scope,
                duration_seconds,
                approved: false,
                expires_at: 0,
                revoked: false,
                created_at: env.ledger().timestamp(),
            },
        );

        Ok(access_id)
    }
}

#[cfg(test)]
mod test;
