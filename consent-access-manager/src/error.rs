//! Typed failure reasons returned by the public contract API.

use soroban_sdk::contracterror;

/// Errors returned by [`ConsentAccessManager`](crate::ConsentAccessManager).
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// The requested access duration is not usable: a request must be granted
    /// for a non-zero number of seconds, otherwise it would expire the moment
    /// the patient approved it.
    InvalidDuration = 1,
    /// No [`AccessRequest`](crate::AccessRequest) is stored under the given
    /// `access_id`.
    RequestNotFound = 2,
    /// The `access_id` exists, but its `passport_id` does not match the
    /// caller, so the caller has no right to act on it.
    NotOwner = 3,
    /// The request has already been revoked, so it can no longer be approved
    /// or rejected.
    AlreadyRevoked = 4,
    /// The request has already been approved, so it can no longer be
    /// rejected.
    AlreadyApproved = 5,
    /// The request has already been rejected, so it can no longer be
    /// approved or rejected again.
    AlreadyRejected = 6,
}
