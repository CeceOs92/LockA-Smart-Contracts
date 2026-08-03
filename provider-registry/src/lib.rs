#![no_std]

use soroban_sdk::{contract, contractimpl};

mod types;

pub use types::{is_valid_transition, ProviderStatus, ProviderType};

#[contract]
pub struct ProviderRegistry;

#[contractimpl]
impl ProviderRegistry {}

#[cfg(test)]
mod test;
