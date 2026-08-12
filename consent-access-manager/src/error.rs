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
}
