#![no_std]

use soroban_sdk::{contract, contractimpl};

mod storage;
mod types;

pub use storage::DataKey;
pub use types::{is_valid_transition, Passport, PassportStatus};

#[contract]
pub struct PatientPassportRegistry;

#[contractimpl]
impl PatientPassportRegistry {}

#[cfg(test)]
mod test;
