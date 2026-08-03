#![no_std]

use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct ZkCredentialVerifier;

#[contractimpl]
impl ZkCredentialVerifier {}

#[cfg(test)]
mod test;
