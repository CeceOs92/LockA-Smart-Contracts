#![no_std]

use soroban_sdk::{contract, contractimpl};

mod types;

pub use types::{ClaimStatus, SubmissionStatus};

#[contract]
pub struct ZkCredentialVerifier;

#[contractimpl]
impl ZkCredentialVerifier {}

#[cfg(test)]
mod test;
