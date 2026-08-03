#![no_std]

use soroban_sdk::{contract, contractimpl};

mod types;

pub use types::{is_valid_transition, RecordStatus, RecordType};

#[contract]
pub struct MedicalRecordRegistry;

#[contractimpl]
impl MedicalRecordRegistry {}

#[cfg(test)]
mod test;
