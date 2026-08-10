# LockA Smart Contracts

Smart contracts for patient identity, provider registry, consent permissions, access logs, and record hash verification.

This repository holds the on-chain layer of LockA Medical Passport — a patient-controlled digital health passport for secure, private, interoperable, and verifiable medical records across Africa. It follows the design of the [LockA EVM/Solidity version](https://github.com/Dannyswiss1/LockA-Medical-Passport-Monorepo) ([live demo](https://locka.remixdapp.eth.limo/)), but is being rebuilt for the **Stellar network using Soroban smart contracts written in Rust**, with **Freighter** as the primary wallet and signing toolkit and **XLM/Stellar accounts** as the native asset and identity layer. Full platform documentation lives in [LockA-Documentation](https://github.com/LockA-Medical-Passport/LockA-Documentation/blob/main/Documentation.md).

## Overview

Stellar is used as the trust, permission, audit, and verification layer for the platform — it does not store raw medical data. Medical records, files, and metadata remain encrypted and off-chain (e.g. IPFS/Filecoin or S3-compatible storage). The contracts in this repository are responsible for:

- Patient identity and passport registration
- Provider registration and verification
- Consent-based access control (grant, limit, expire, revoke)
- Record hash/commitment anchoring for tamper-evident verification
- Device/IoT attestation for verifiable readings
- Audit event emission for access requests, approvals, revocations, and record updates

> **Privacy Rule**
>
> No raw medical record, diagnosis, prescription, lab result, identity document, or personally identifiable medical information is stored on-chain. On-chain data is limited to hashes, commitments, consent state, provider identifiers, revocation status, and event logs.

## Technology Stack

| Layer | Technology / Use |
| --- | --- |
| Blockchain | Stellar network |
| Smart Contracts | Rust + Soroban SDK, compiled to WebAssembly |
| Wallet and Signing | Freighter (browser-based Stellar transaction signing); passkeys/smart wallets planned for easier patient onboarding |
| Chain Interaction | Stellar SDK and Stellar RPC for building/simulating/submitting transactions and generating contract client bindings |
| Native Asset | XLM (Stellar Lumens), with Stellar Assets considered for future billing, health savings, or insurance settlement flows |
| Indexing | Stellar RPC event consumption by an off-chain indexer for consent changes and record update events |
| Testing | Soroban unit and integration tests, plus contract-level end-to-end tests against Stellar testnet |

## Smart Contract Architecture

| Contract | Responsibility |
| --- | --- |
| `PatientIdentityRegistry` | Registers patient passport identifiers, public keys, recovery configuration, and optional identity commitments. |
| `ProviderRegistry` | Registers verified hospitals, clinics, labs, pharmacies, insurers, and their authorized staff accounts. |
| `ConsentAccessControl` | Creates, approves, limits, expires, and revokes provider access permissions. |
| `RecordCommitmentRegistry` | Stores hashes/commitments of encrypted records, record categories, issuer references, and update events. |
| `DeviceAttestationRegistry` | Registers approved medical devices and IoT/wearable data sources for verifiable readings. |
| `AuditEventEmitter` | Emits events for access requests, consent grants, revocations, record additions, provider updates, and device attestations. |

Zero-Knowledge proofs (e.g. vaccination status, insurance eligibility, provider authorization, record integrity) are planned for selective disclosure and privacy-preserving access checks. For the MVP, proof generation/verification can sit in the application layer or a dedicated verifier service, with contracts storing proof commitments, consent states, and verification outcomes.

## Key User Flows (Contract-Relevant)

1. **Patient creates a passport** — a passport identifier and identity commitment are registered via `PatientIdentityRegistry`.
2. **Provider requests access** — a provider requests a record category and duration; the patient approves/limits/rejects, and the decision is recorded by `ConsentAccessControl`.
3. **Provider adds a record** — the record is encrypted and stored off-chain, and its hash/commitment is submitted to `RecordCommitmentRegistry`; an event is emitted for indexing.
4. **Patient revokes access** — revocation is recorded on-chain immediately or via expiry, and providers lose access through the off-chain access layer.

## Repository Structure

This repository corresponds to `locka-contracts` in the wider LockA organization: Soroban contracts, contract tests, deployment scripts, generated client bindings, and Stellar testnet configuration.

## MVP Scope

- Patient identity registration (`PatientIdentityRegistry`)
- Provider registration and verification status (`ProviderRegistry`)
- Consent approval, limitation, expiry, and revocation (`ConsentAccessControl`)
- Record hash/commitment anchoring (`RecordCommitmentRegistry`)
- Audit event emission for indexing (`AuditEventEmitter`)
- Basic ZK proof commitment storage for one or two claims (e.g. vaccination proof or provider authorization proof)

Out of scope for the MVP: full nationwide hospital system replacement, direct integration with every hospital management system, complete insurance claims automation, and large-scale IoT deployment beyond prototype device attestation.