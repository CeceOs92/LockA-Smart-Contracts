#![no_std]

use soroban_sdk::{contract, contractimpl};

mod storage;
mod types;

pub use storage::DataKey;
pub use types::{Device, DeviceCategory};

#[contract]
pub struct DeviceDataAttestation;

#[contractimpl]
impl DeviceDataAttestation {}

#[cfg(test)]
mod test;
