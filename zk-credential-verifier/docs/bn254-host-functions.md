# BN254 and Poseidon host functions: confirmed API surface

Reference for every cryptography issue in the `zk-credential-verifier` epic.
It replaces the placeholder assumptions those issues were written against.

| | |
| --- | --- |
| **Verified against** | `soroban-sdk` 27.0.4 (the workspace pin), `soroban-env-host` 27.0.1 |
| **Verified on** | 2026-08-12 |
| **How** | Reading the vendored SDK source; the executable tests in `src/test.rs::bn254_host_functions`; live queries to Horizon, the Testnet/Futurenet RPCs, and `stellar network settings` |
| **Status** | Confirmed unless a line says otherwise. Anything unverified is called out in [Not yet confirmed](#not-yet-confirmed). |

Every API claim below is pinned by a test in
`src/test.rs::bn254_host_functions`. If an SDK bump changes the surface, those
tests stop compiling or fail — that is the intended trigger for **correcting
this document** rather than working around it in the calling code.

---

## Corrections to the epic's assumptions

The issue that commissioned this spike assumed an API that does not exist as
described. Five things to carry into the rest of the epic:

1. **There is no `Crypto::bn254_g1_add`.** The `bn254_*` names are the *internal*
   host-function symbols (`internal::Env::bn254_g1_add`), not the public Rust
   API. Contracts go through `env.crypto().bn254()`, which returns a `Bn254`
   handle carrying the methods.
2. **The pairing method is `pairing_check`, not `multi_pairing_check`.**
   `bn254_multi_pairing_check` is again the internal symbol.
3. **Poseidon is not on `env.crypto()`.** It sits on `CryptoHazmat`, reachable
   only via `env.crypto_hazmat()` and only when the **`hazmat-crypto`** Cargo
   feature is enabled. Without that feature the type is `pub(crate)` and the
   call does not compile.
4. **G2 arithmetic is not exposed at all** — see [G2 is opaque](#g2-is-opaque).
5. **The network is well past Protocol 25.** Testnet *and* Mainnet both run
   **Protocol 27** (Futurenet is already on 28), so these host functions are
   live in production, not a testnet-only preview.

And the headline result for the epic's feasibility: a four-term Groth16 pairing
check measures **24,385,303 CPU instructions against a 400,000,000
per-transaction budget** — roughly 6%. On-chain verification fits comfortably.
See [Cost and metering](#cost-and-metering).

---

## SDK version requirements

There is no 24.x line; the SDK jumped from 23.x to 25.x to track protocol
numbering. Support arrived in stages:

| Version | What it adds |
| --- | --- |
| **25.0.0** | First release with `crypto::bn254`. Has `g1_add`, `g1_mul`, `pairing_check`, `poseidon_permutation`, `poseidon2_permutation`, and the `Bn254G1Affine` / `Bn254G2Affine` / `Fr` / `Bn254Fp` types. |
| **26.0.0** | Adds `g1_msm`, `g1_is_on_curve`, and the scalar-field arithmetic `fr_add` / `fr_sub` / `fr_mul` / `fr_pow` / `fr_inv`. |
| **27.0.4** | The workspace pin. Renames `Fr` → `Bn254Fr` (the old `Fr` alias still exists but is `#[deprecated]`). |

**Use ≥ 26.0.0, and prefer the workspace's 27.0.4.** 25.x is technically enough
for the three functions the issue named, but it lacks `g1_msm` — and
accumulating a Groth16 proof's public inputs *is* a multi-scalar
multiplication. Doing it with a `g1_mul`/`g1_add` loop on 25.x costs more and
buys nothing.

Nothing needs to be installed: the workspace already resolves to 27.0.4 via
`[workspace.dependencies]`, and the verification tests compile and pass against
it today.

> **Workspace inconsistency, pre-existing.** `medical-record-registry` does not
> use the workspace dependency — it pins `soroban-sdk = "25.3.1"` directly, so
> the lockfile carries both 25.3.2 and 27.0.4. It does no BN254 work, so this
> spike is unaffected, but that crate would hit the 25.x limitations above if it
> ever did. Worth aligning separately.

### Enabling Poseidon

```toml
soroban-sdk = { workspace = true, features = ["hazmat-crypto"] }
```

`zk-credential-verifier/Cargo.toml` currently enables this in
**`[dev-dependencies]` only**, which is enough for the spike's tests. The first
issue that calls Poseidon from contract code (rather than a test) must also add
it to `[dependencies]`.

The feature is named "hazmat" for a reason the SDK is explicit about: these are
raw permutations with no input validation, and misuse is a security bug, not a
compile error. See [Poseidon](#poseidon) below.

---

## BN254

Obtain the handle, then call methods on it:

```rust
let bn254 = env.crypto().bn254();
```

### Signatures

Exactly as they appear in `soroban_sdk::crypto::bn254`:

```rust
// Curve operations
pub fn g1_add(&self, p0: &Bn254G1Affine, p1: &Bn254G1Affine) -> Bn254G1Affine;
pub fn g1_mul(&self, p0: &Bn254G1Affine, scalar: &Bn254Fr) -> Bn254G1Affine;
pub fn g1_msm(&self, vp: Vec<Bn254G1Affine>, vs: Vec<Bn254Fr>) -> Bn254G1Affine;
pub fn g1_is_on_curve(&self, point: &Bn254G1Affine) -> bool;

// Pairing
pub fn pairing_check(&self, vp1: Vec<Bn254G1Affine>, vp2: Vec<Bn254G2Affine>) -> bool;

// Scalar field arithmetic (mod r)
pub fn fr_add(&self, lhs: &Bn254Fr, rhs: &Bn254Fr) -> Bn254Fr;
pub fn fr_sub(&self, lhs: &Bn254Fr, rhs: &Bn254Fr) -> Bn254Fr;
pub fn fr_mul(&self, lhs: &Bn254Fr, rhs: &Bn254Fr) -> Bn254Fr;
pub fn fr_pow(&self, lhs: &Bn254Fr, rhs: u64) -> Bn254Fr;
pub fn fr_inv(&self, lhs: &Bn254Fr) -> Bn254Fr;
```

Note the argument conventions: points are taken **by reference**, but the
vector-taking `g1_msm` and `pairing_check` take their `Vec`s **by value**.

`pairing_check` returns `true` when the product of pairings equals 1 in the
target group. It **panics** (traps, not `false`) if the two vectors have
different lengths or are empty — length agreement is the caller's job.

Operator overloads exist for the common cases and dispatch to the same host
functions: `Add` and `Mul<Bn254Fr>` on `Bn254G1Affine`, and `Neg` on
`Bn254G1Affine` / `Bn254Fp`.

### Encodings

All points are **uncompressed**. There is no compressed encoding on BN254 here.

| Type | Wraps | Size | Layout |
| --- | --- | --- | --- |
| `Bn254Fp` | `BytesN<32>` | 32 B | Big-endian field element, `< p` |
| `Bn254Fr` | `U256` | 32 B | Scalar mod `r` (a `U256`, *not* a byte wrapper) |
| `Bn254G1Affine` | `BytesN<64>` | 64 B | `be(X) ‖ be(Y)` |
| `Bn254G2Affine` | `BytesN<128>` | 128 B | `be(X) ‖ be(Y)`, each an Fp2 element |

Exported as `BN254_FP_SERIALIZED_SIZE` (32), `BN254_G1_SERIALIZED_SIZE` (64),
and `BN254_G2_SERIALIZED_SIZE` (128).

**The Fp2 limb order is the easy thing to get wrong.** Each G2 coordinate is
encoded `be(c1) ‖ be(c0)` — the *imaginary* component first:

```
Bn254G2Affine = be(x.c1) ‖ be(x.c0) ‖ be(y.c1) ‖ be(y.c0)
                └── 32 ──┘└── 32 ──┘└── 32 ──┘└── 32 ──┘
```

This matches Ethereum's `alt_bn128` precompile convention, so proofs and keys
exported by snarkjs/circom in Ethereum byte order transfer directly. It is the
opposite of the `c0`-first order used by some other libraries; a swapped pair
produces a point that is still well-formed but simply not the one intended, so
the failure mode is a silently failing pairing check, not a trap.

Further encoding rules the host enforces:

- The two flag bits (`0x80`, `0x40` of the first byte) must be unset.
- The point at infinity is all-zero bytes (64 for G1, 128 for G2).
- G1 points must be on the curve; **G2 points must be on the curve *and* in the
  correct subgroup.**

`from_bytes` on `Bn254G1Affine` and `Bn254G2Affine` **does not validate** — it
accepts any correctly-sized byte string. Validation happens in the host when
the value reaches a host function, where bad input **traps**. So malformed
proof bytes surface as an unrecoverable panic, not a `false` return. Any
`submit_proof`-style entry point that takes attacker-supplied bytes needs to
treat a trap as a possible outcome. `Bn254Fp::from_bytes` is the exception: it
validates against the modulus eagerly and panics on overflow.

### G2 is opaque

**Answering the spike's third question directly: there is no G2 arithmetic.**

`Bn254G2Affine` has exactly two operations — `from_bytes` and `to_bytes` (plus
`env()`). There is no `g2_add`, no `g2_mul`, no `g2_msm`, no `g2_is_on_curve`,
and no `Neg`. A G2 point is an opaque 128-byte blob whose only destination is
`pairing_check`.

> The SDK's own doc comment on `Bn254G2Affine` claims its bytes are validated
> "when the value is passed to a host function (e.g. `g2_add`, `g2_mul`,
> `pairing`)". **Those first two functions do not exist** — the sentence is a
> copy-paste from the BLS12-381 module, which does expose G2 arithmetic. Don't
> plan around it.

Practical consequence for Groth16: the verification equation must be arranged so
that every negation lands on a **G1** term. The standard form already does —

```
e(-A, B) · e(α, β) · e(vk_x, γ) · e(C, δ) == 1
```

— because `A` is the G1 component of the proof and `-A` is reachable via `Neg`.
The verifying key's `β`, `γ`, `δ` are G2 and are consumed as-is, exactly as
stored. This is a real constraint on the data model: **G2 elements of the VK
never need to be anything but `BytesN<128>`.**

### Mapping to Groth16

For the data-model and verification issues, the primitives compose like this:

| Groth16 step | Primitive |
| --- | --- |
| `vk_x = IC[0] + Σ pubᵢ · IC[i]` | `g1_msm` over `IC[1..]`, then one `g1_add` for `IC[0]` |
| Negate `A` | `-proof_a` (the `Neg` operator, computed in-guest) |
| The four-term check | one `pairing_check` with 4-element `Vec`s |
| Public-input field math | `fr_*`, if inputs need combining before use |

That is the whole surface needed. No G2 arithmetic, no target-group
arithmetic, and no final-exponentiation call is required or available —
`pairing_check` does the product and the comparison to 1 internally.

---

## Poseidon

Both permutations live on `CryptoHazmat`, behind the `hazmat-crypto` feature:

```rust
pub fn poseidon_permutation(
    &self,
    input: &Vec<U256>,
    field: Symbol,                     // "BN254" or "BLS12_381"
    t: u32,                            // state size
    d: u32,                            // S-box degree (5 for both curves)
    rounds_f: u32,                     // full rounds, must be even
    rounds_p: u32,                     // partial rounds
    mds: &Vec<Vec<U256>>,              // t × t MDS matrix
    round_constants: &Vec<Vec<U256>>,  // (rounds_f + rounds_p) × t
) -> Vec<U256>;

pub fn poseidon2_permutation(
    &self,
    input: &Vec<U256>,
    field: Symbol,
    t: u32,                                 // ∈ {2, 3, 4, 8, 12, 16, 20, 24}
    d: u32,
    rounds_f: u32,
    rounds_p: u32,
    mat_internal_diag_m_1: &Vec<U256>,      // length t
    round_constants: &Vec<Vec<U256>>,       // (rounds_f + rounds_p) × t
) -> Vec<U256>;
```

Called as `env.crypto_hazmat().poseidon2_permutation(...)`. The `field`
argument is a `Symbol`, i.e. `Symbol::new(&env, "BN254")`.

Three properties that shape how the epic should use this:

**1. It is a permutation, not a hash.** It maps a `t`-element state to a
`t`-element state. There is no padding, no domain separation, no
sponge construction, and no fixed-length digest. Building a *hash* out of it is
the caller's responsibility.

**2. The host ships no constants.** The MDS matrix (or internal diagonal) and
the full round-constant table come from the caller on every invocation. The
host validates only that the dimensions are self-consistent — it will happily
run a permutation with nonsense constants and return a nonsense-but-deterministic
result. Two contracts using different tables compute different "Poseidon
hashes" of the same input with no error anywhere.

**3. `U256` inputs are silently reduced mod the field order.** Two distinct
`U256` values that reduce to the same field element produce identical output.
Any value derived from untrusted input must be range-checked into `[0, r)`
*before* it reaches the permutation, or the collision is free.

Taken together, these are why the feature is gated behind `hazmat`. **Prefer the
[`soroban-poseidon`](https://github.com/stellar/rs-soroban-poseidon) crate**
(published on crates.io; 27.0.0 is current and version-aligned with the SDK),
which supplies the standard parameter sets and the sponge construction on top.
Reach for the raw permutation only with a specific reason to.

`src/test.rs` exercises both permutations with deliberately arbitrary constants
— confirming the *call shape and dimension rules*, explicitly not a
known-answer test against standard parameters. The two differ only in that
argument: Poseidon1 takes the full `t`-by-`t` `mds` matrix, Poseidon2 takes the
length-`t` `mat_internal_diag_m_1` instead.

---

## Network and tooling

### Protocol status

Confirmed by live query on 2026-08-12:

| Network | Protocol | Source |
| --- | --- | --- |
| **Mainnet** | **27** | Horizon `/ledgers` (ledger 63,921,127) |
| **Testnet** | **27** | Horizon `/ledgers` (ledger 4,106,558) and RPC `getNetwork` |
| **Futurenet** | **28** | Horizon `/ledgers` (ledger 261,489) and RPC `getNetwork` |

All three are past Protocol 25, so BN254 and Poseidon are available everywhere,
including production. The epic can target Mainnet without a protocol caveat.

**Develop against Testnet, not Futurenet.** Testnet is at protocol parity with
Mainnet; Futurenet is a protocol ahead, so it would test against semantics
Mainnet does not yet have.

Testnet RPC software, from `getVersionInfo`: **stellar-rpc 27.1.1**, captive
core 27.1.0. Any RPC serving these host functions needs to be on the 27.x line.

| | Testnet | Futurenet |
| --- | --- | --- |
| RPC | `https://soroban-testnet.stellar.org` | `https://rpc-futurenet.stellar.org` |
| Passphrase | `Test SDF Network ; September 2015` | `Test SDF Future Network ; October 2022` |

### Local testing

`cargo test` needs no network and no configuration. `Env::default()` runs the
bundled `soroban-env-host` 27.0.1, which implements all of these host functions
*and* meters them with the same cost model the network uses:

```bash
cargo test -p zk-credential-verifier
```

### Pointing the CLI at a compatible network

`testnet` and `futurenet` are built-in CLI network names — no setup needed:

```bash
stellar network ls                        # local, futurenet, mainnet, testnet
stellar network health  --network testnet
stellar network info    --network testnet # protocol version, RPC build
stellar network use testnet               # make it the default for later commands
```

Only a non-standard endpoint needs registering:

```bash
stellar network add my-testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

**Upgrade the CLI to 27.x before doing this work.** The version installed here
is 26.0.0, and it is explicitly behind — read-only commands succeed, but
`stellar network settings --network testnet` prints:

> ⚠️ Network protocol version is 27 but the stellar-cli supports 26. The config
> fetched may not represent the complete config settings for the network.
> Upgrade the stellar-cli.

So a 26.x CLI silently returns a *partial* view of protocol-27 state. Basic
health and info calls were verified working, but do not trust it for anything
protocol-sensitive:

```bash
cargo install --locked stellar-cli   # or: brew upgrade stellar-cli
stellar --version
```

---

## Cost and metering

The Soroban cost model is:

```
cost = const_term + ((linear_term × input) >> 7)
```

The `>> 7` matters — linear terms are stored pre-scaled by 128
(`COST_MODEL_LIN_TERM_SCALE_BITS = 7` in `soroban-env-host`). Reading the raw
parameters without unscaling overstates costs by 128×.

Live BN254 CPU parameters, from `stellar network settings --network testnet` on
2026-08-12, indexed by `ContractCostType`:

| Cost type | const | linear (pre-scale) |
| --- | ---: | ---: |
| `Bn254G1Add` (76) | 3,623 | 0 |
| `Bn254G1Mul` (77) | 1,150,435 | 0 |
| `Bn254Pairing` (78) | 5,263,916 | 392,472,814 |
| `Bn254G1CheckPointOnCurve` (72) | 904 | 0 |
| `Bn254G2CheckPointOnCurve` (73) | 2,811 | 0 |
| `Bn254G2CheckPointInSubgroup` (74) | 1,706,052 | 0 |
| `Bn254FrAddSub` (81) | 74 | 0 |
| `Bn254FrMul` (82) | 332 | 0 |
| `Bn254FrInv` (84) | 33,151 | 0 |

Relevant network limits: `tx_max_instructions` **400,000,000**,
`tx_memory_limit` 40 MiB, `tx_max_size_bytes` 132,096.

### Measured: does Groth16 fit?

**Yes, with room to spare.** Measured in the test host — which applies the same
metering — a four-term `pairing_check` in the Groth16 shape costs:

| | |
| --- | ---: |
| 4-term `pairing_check` | **24,385,303** CPU instructions |
| Per-transaction budget | 400,000,000 |
| **Budget consumed** | **~6.1%** |

`four_term_pairing_check_fits_the_transaction_instruction_budget` asserts this
stays under the limit, so a future cost-model change that breaks the assumption
fails the build rather than surfacing as a Testnet error.

Reading the table: the pairing dominates everything else by orders of
magnitude. `g1_msm` over a handful of public inputs and the `fr_*` operations
are rounding errors next to it. The G2 subgroup check (1.7M per point, charged
on each G2 input to the pairing) is the only other line item worth noticing.

Two caveats. This measures the *host function*, not a full contract invocation —
wasm execution, storage reads for the verifying key, and argument
deserialization add to it, and a real `verify_proof` entry point should be
re-measured end to end. And the instruction budget is only one limit; the
verifying key plus proof must also fit `tx_max_size_bytes` and the contract
data entry limits.

---

## Not yet confirmed

Open items, to be resolved by the issues that hit them — and then **written back
into this document**:

- **End-to-end Groth16.** No real proof has been verified. The primitives are
  confirmed individually; the composition is not. The first verification issue
  should carry a known-good snarkjs/circom fixture.
- **Full-invocation cost.** The 24.4M figure is the host function in isolation.
  Wasm execution, verifying-key storage reads, and argument deserialization are
  not included, and a real `verify_proof` should be re-measured end to end —
  ideally against Testnet simulation rather than the test host.
- **Standard Poseidon parameters.** The BN254 round constants and MDS tables
  for the `t` the epic settles on have not been sourced or checked against a
  reference implementation.
- **`soroban-poseidon` API.** Confirmed to exist and to be version-aligned at
  27.0.0; its actual surface was not reviewed.
- **Recovering from a trap.** Confirmed that malformed point bytes trap in the
  host rather than returning an error. Not established whether a contract can
  present that to callers as a typed error instead of an unrecoverable panic —
  relevant to any entry point taking attacker-supplied proof bytes, and worth
  settling before the `submit_proof` data model is fixed.

## Re-verifying this document

```bash
cargo test -p zk-credential-verifier          # pins every API claim above
```

The tests live in `src/test.rs::bn254_host_functions`. When bumping
`soroban-sdk`, run them first: a compile error there is the signal that this
document has drifted and needs editing, not that the tests need silencing.
