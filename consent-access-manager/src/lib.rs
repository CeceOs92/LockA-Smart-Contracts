#![no_std]

use soroban_sdk::{contract, contractimpl};

mod types;

pub use types::RecordScope;

#[contract]
pub struct ConsentAccessManager;

#[contractimpl]
impl ConsentAccessManager {}

#[cfg(test)]
mod test;
