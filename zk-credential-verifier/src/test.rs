use soroban_sdk::Env;

use crate::{ZkCredentialVerifier, ZkCredentialVerifierClient};

#[test]
fn contract_registers_and_builds_a_client() {
    let env = Env::default();
    let contract_id = env.register(ZkCredentialVerifier, ());
    let _client = ZkCredentialVerifierClient::new(&env, &contract_id);
}

/// Executable evidence for the claims in `docs/bn254-host-functions.md`.
///
/// These tests exist to keep that document honest: they pin the exact method
/// signatures, the point encodings, and the parameter shapes the host accepts.
/// If a `soroban-sdk` bump changes any of it, these stop compiling or fail, and
/// the document gets corrected rather than quietly drifting out of date.
#[cfg(test)]
mod bn254_host_functions {
    extern crate std;

    use soroban_sdk::crypto::bn254::{
        Bn254Fp, Bn254Fr, Bn254G1Affine, Bn254G2Affine, BN254_FP_SERIALIZED_SIZE,
        BN254_G1_SERIALIZED_SIZE, BN254_G2_SERIALIZED_SIZE,
    };
    use soroban_sdk::{vec, BytesN, Env, Symbol, Vec, U256};

    /// BN254 base field modulus `p`, big-endian.
    const FP_MODULUS: [u8; 32] = [
        0x30, 0x64, 0x4e, 0x72, 0xe1, 0x31, 0xa0, 0x29, 0xb8, 0x50, 0x45, 0xb6, 0x81, 0x81, 0x58,
        0x5d, 0x97, 0x81, 0x6a, 0x91, 0x68, 0x71, 0xca, 0x8d, 0x3c, 0x20, 0x8c, 0x16, 0xd8, 0x7c,
        0xfd, 0x47,
    ];

    /// The canonical G1 generator, `(x, y) = (1, 2)`, in the host's
    /// uncompressed `be(X) || be(Y)` encoding.
    fn g1_generator(env: &Env) -> Bn254G1Affine {
        let mut bytes = [0u8; BN254_G1_SERIALIZED_SIZE];
        bytes[31] = 1;
        bytes[63] = 2;
        Bn254G1Affine::from_bytes(BytesN::from_array(env, &bytes))
    }

    /// `-G1 = (1, p - 2)`. Negation in G1 flips only the Y coordinate.
    fn g1_generator_negated(env: &Env) -> Bn254G1Affine {
        let mut y = FP_MODULUS;
        y[31] -= 2;

        let mut bytes = [0u8; BN254_G1_SERIALIZED_SIZE];
        bytes[31] = 1;
        bytes[32..].copy_from_slice(&y);
        Bn254G1Affine::from_bytes(BytesN::from_array(env, &bytes))
    }

    /// The canonical G2 generator. Note the Fp2 limb order the host expects:
    /// each coordinate is `be(c1) || be(c0)` — imaginary part *first*.
    fn g2_generator(env: &Env) -> Bn254G2Affine {
        const X_C1: [u8; 32] =
            hex("198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2");
        const X_C0: [u8; 32] =
            hex("1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed");
        const Y_C1: [u8; 32] =
            hex("090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b");
        const Y_C0: [u8; 32] =
            hex("12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa");

        let mut bytes = [0u8; BN254_G2_SERIALIZED_SIZE];
        bytes[0..32].copy_from_slice(&X_C1);
        bytes[32..64].copy_from_slice(&X_C0);
        bytes[64..96].copy_from_slice(&Y_C1);
        bytes[96..128].copy_from_slice(&Y_C0);
        Bn254G2Affine::from_bytes(BytesN::from_array(env, &bytes))
    }

    /// `const`-evaluable hex decoder, so the generator constants stay readable.
    const fn hex(s: &str) -> [u8; 32] {
        const fn nibble(c: u8) -> u8 {
            match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'f' => c - b'a' + 10,
                _ => panic!("non-hex digit"),
            }
        }

        let src = s.as_bytes();
        assert!(src.len() == 64, "expected 32 bytes of hex");

        let mut out = [0u8; 32];
        let mut i = 0;
        while i < 32 {
            out[i] = (nibble(src[2 * i]) << 4) | nibble(src[2 * i + 1]);
            i += 1;
        }
        out
    }

    fn fr(env: &Env, value: u32) -> Bn254Fr {
        Bn254Fr::from_u256(U256::from_u32(env, value))
    }

    #[test]
    fn serialized_sizes_are_uncompressed_and_ethereum_shaped() {
        assert_eq!(BN254_FP_SERIALIZED_SIZE, 32);
        assert_eq!(BN254_G1_SERIALIZED_SIZE, 64);
        assert_eq!(BN254_G2_SERIALIZED_SIZE, 128);
    }

    #[test]
    fn g1_add_and_g1_mul_agree_on_the_doubled_generator() {
        let env = Env::default();
        let bn254 = env.crypto().bn254();
        let g = g1_generator(&env);

        let doubled = bn254.g1_add(&g, &g);
        let scaled = bn254.g1_mul(&g, &fr(&env, 2));

        assert_eq!(doubled.to_bytes(), scaled.to_bytes());
        assert!(bn254.g1_is_on_curve(&doubled));
    }

    #[test]
    fn g1_msm_matches_repeated_add_and_mul() {
        let env = Env::default();
        let bn254 = env.crypto().bn254();
        let g = g1_generator(&env);

        // 3*G + 5*G, the shape used to accumulate Groth16 public inputs.
        let msm = bn254.g1_msm(
            vec![&env, g.clone(), g.clone()],
            vec![&env, fr(&env, 3), fr(&env, 5)],
        );
        let expected = bn254.g1_mul(&g, &fr(&env, 8));

        assert_eq!(msm.to_bytes(), expected.to_bytes());
    }

    #[test]
    fn pairing_check_accepts_a_cancelling_pair_and_rejects_a_single_one() {
        let env = Env::default();
        let bn254 = env.crypto().bn254();
        let g1 = g1_generator(&env);
        let neg_g1 = g1_generator_negated(&env);
        let g2 = g2_generator(&env);

        // e(G1, G2) * e(-G1, G2) == 1
        assert!(bn254.pairing_check(
            vec![&env, g1.clone(), neg_g1],
            vec![&env, g2.clone(), g2.clone()],
        ));

        // e(G1, G2) alone is not the identity.
        assert!(!bn254.pairing_check(vec![&env, g1], vec![&env, g2]));
    }

    #[test]
    fn scalar_field_arithmetic_is_available_on_the_host() {
        let env = Env::default();
        let bn254 = env.crypto().bn254();

        let seven = bn254.fr_add(&fr(&env, 3), &fr(&env, 4));
        assert_eq!(seven.to_u256(), U256::from_u32(&env, 7));
        assert_eq!(
            bn254.fr_sub(&seven, &fr(&env, 5)).to_u256(),
            U256::from_u32(&env, 2)
        );
        assert_eq!(
            bn254.fr_mul(&fr(&env, 6), &fr(&env, 7)).to_u256(),
            U256::from_u32(&env, 42)
        );
        assert_eq!(
            bn254.fr_pow(&fr(&env, 2), 10).to_u256(),
            U256::from_u32(&env, 1024)
        );
        // inv(x) * x == 1
        assert_eq!(
            bn254
                .fr_mul(&bn254.fr_inv(&fr(&env, 9)), &fr(&env, 9))
                .to_u256(),
            U256::from_u32(&env, 1)
        );
    }

    /// Confirms the *call shape* of the Poseidon2 permutation: the host takes
    /// the full parameter set from the caller and validates only the
    /// dimensions, so the round constants below are arbitrary rather than the
    /// standard BN254 table. See the doc for why that matters.
    #[test]
    fn poseidon2_permutation_takes_caller_supplied_parameters() {
        let env = Env::default();

        const T: u32 = 3;
        const ROUNDS_F: u32 = 8;
        const ROUNDS_P: u32 = 4;

        let input = vec![
            &env,
            U256::from_u32(&env, 1),
            U256::from_u32(&env, 2),
            U256::from_u32(&env, 3),
        ];
        let mat_internal_diag_m_1 = vec![
            &env,
            U256::from_u32(&env, 1),
            U256::from_u32(&env, 2),
            U256::from_u32(&env, 3),
        ];

        let mut round_constants: Vec<Vec<U256>> = Vec::new(&env);
        for round in 0..(ROUNDS_F + ROUNDS_P) {
            let mut row: Vec<U256> = Vec::new(&env);
            for slot in 0..T {
                row.push_back(U256::from_u32(&env, round * T + slot + 1));
            }
            round_constants.push_back(row);
        }

        // Poseidon lives on `CryptoHazmat`, reachable only when the
        // `hazmat-crypto` feature is enabled — not on `env.crypto()`.
        let output = env.crypto_hazmat().poseidon2_permutation(
            &input,
            Symbol::new(&env, "BN254"),
            T,
            5, // S-box degree
            ROUNDS_F,
            ROUNDS_P,
            &mat_internal_diag_m_1,
            &round_constants,
        );

        assert_eq!(output.len(), T, "permutation is state-size preserving");
        assert_ne!(output, input, "permutation actually mixed the state");
    }

    /// Groth16 needs `-A`, and only G1 can be negated — so the negation has to
    /// be arranged to fall on a G1 term. `Neg` is computed in-guest as
    /// `(X, p - Y)`, which this cross-checks against the hand-built encoding.
    #[test]
    fn g1_negation_is_available_as_an_operator() {
        let env = Env::default();
        let bn254 = env.crypto().bn254();
        let g = g1_generator(&env);

        assert_eq!(
            (-g.clone()).to_bytes(),
            g1_generator_negated(&env).to_bytes()
        );
        // G + (-G) is the point at infinity: 64 zero bytes.
        assert_eq!(
            bn254.g1_add(&g, &-g.clone()).to_bytes(),
            BytesN::from_array(&env, &[0u8; BN254_G1_SERIALIZED_SIZE])
        );
    }

    /// The G2 type is a byte wrapper with no arithmetic of its own; it only
    /// travels into `pairing_check`. Round-tripping is all it supports.
    #[test]
    fn g2_points_are_opaque_bytes() {
        let env = Env::default();
        let g2 = g2_generator(&env);

        let round_tripped = Bn254G2Affine::from_bytes(g2.to_bytes());
        assert_eq!(round_tripped.to_bytes(), g2.to_bytes());
        assert_eq!(g2.to_bytes().to_array().len(), BN254_G2_SERIALIZED_SIZE);
    }

    /// Poseidon1 takes a full `t`-by-`t` MDS matrix where Poseidon2 takes only
    /// the internal diagonal. Same caveat as the Poseidon2 test: the constants
    /// are arbitrary, so this pins the call shape, not a known-answer vector.
    #[test]
    fn poseidon1_permutation_takes_a_full_mds_matrix() {
        let env = Env::default();

        const T: u32 = 3;
        const ROUNDS_F: u32 = 8;
        const ROUNDS_P: u32 = 4;

        let input = vec![
            &env,
            U256::from_u32(&env, 1),
            U256::from_u32(&env, 2),
            U256::from_u32(&env, 3),
        ];

        let mut mds: Vec<Vec<U256>> = Vec::new(&env);
        for row_index in 0..T {
            let mut row: Vec<U256> = Vec::new(&env);
            for col_index in 0..T {
                row.push_back(U256::from_u32(&env, row_index * T + col_index + 1));
            }
            mds.push_back(row);
        }

        let mut round_constants: Vec<Vec<U256>> = Vec::new(&env);
        for round in 0..(ROUNDS_F + ROUNDS_P) {
            let mut row: Vec<U256> = Vec::new(&env);
            for slot in 0..T {
                row.push_back(U256::from_u32(&env, round * T + slot + 1));
            }
            round_constants.push_back(row);
        }

        let output = env.crypto_hazmat().poseidon_permutation(
            &input,
            Symbol::new(&env, "BN254"),
            T,
            5,
            ROUNDS_F,
            ROUNDS_P,
            &mds,
            &round_constants,
        );

        assert_eq!(output.len(), T);
        assert_ne!(output, input);
    }

    /// Mismatched vector lengths trap rather than returning `false` — the
    /// caller owns the length invariant.
    #[test]
    #[should_panic]
    fn pairing_check_traps_on_mismatched_vector_lengths() {
        let env = Env::default();
        let g1 = g1_generator(&env);
        let g2 = g2_generator(&env);

        env.crypto()
            .bn254()
            .pairing_check(vec![&env, g1.clone(), g1], vec![&env, g2]);
    }

    /// Empty inputs trap too, so "no terms" is not a silently-true check.
    #[test]
    #[should_panic]
    fn pairing_check_traps_on_empty_vectors() {
        let env = Env::default();

        env.crypto()
            .bn254()
            .pairing_check(Vec::new(&env), Vec::new(&env));
    }

    /// `from_bytes` accepts any 64 bytes; the host rejects them on use. This is
    /// the failure mode a `submit_proof` entry point has to expect from
    /// attacker-supplied bytes: an unrecoverable trap, not a `false`.
    #[test]
    #[should_panic]
    fn malformed_g1_bytes_trap_in_the_host_not_at_construction() {
        let env = Env::default();

        // Constructing is fine — all flag bits set, not a curve point.
        let garbage = Bn254G1Affine::from_bytes(BytesN::from_array(
            &env,
            &[0xffu8; BN254_G1_SERIALIZED_SIZE],
        ));

        // Using it is not.
        env.crypto().bn254().g1_is_on_curve(&garbage);
    }

    /// `Bn254Fp::from_bytes` is the one constructor that *does* validate
    /// eagerly: it panics on a value at or above the field modulus.
    #[test]
    #[should_panic(expected = "Bn254: Invalid Fp")]
    fn fp_from_bytes_rejects_values_at_or_above_the_modulus() {
        let env = Env::default();
        Bn254Fp::from_bytes(BytesN::from_array(&env, &FP_MODULUS));
    }

    /// Measures the dominant cost of Groth16 verification against the real
    /// metering model, answering the epic's "does it fit in one transaction?"
    /// question. Testnet's `tx_max_instructions` is 400,000,000; the four-term
    /// pairing check is the expensive part by a wide margin.
    #[test]
    fn four_term_pairing_check_fits_the_transaction_instruction_budget() {
        /// `contract_compute_v0.tx_max_instructions`, read from Testnet on
        /// 2026-08-12 via `stellar network settings --network testnet`.
        const TX_MAX_INSTRUCTIONS: u64 = 400_000_000;

        let env = Env::default();
        let bn254 = env.crypto().bn254();
        let g1 = g1_generator(&env);
        let neg_g1 = g1_generator_negated(&env);
        let g2 = g2_generator(&env);

        let mut budget = env.cost_estimate().budget();
        budget.reset_default();

        // Four terms, the shape of e(-A,B)·e(α,β)·e(vk_x,γ)·e(C,δ).
        let holds = bn254.pairing_check(
            vec![&env, g1.clone(), neg_g1.clone(), g1, neg_g1],
            vec![&env, g2.clone(), g2.clone(), g2.clone(), g2],
        );
        assert!(holds);

        let cpu = env.cost_estimate().budget().cpu_instruction_cost();
        std::println!("4-term BN254 pairing_check: {cpu} CPU instructions");

        assert!(
            cpu < TX_MAX_INSTRUCTIONS,
            "4-term pairing check cost {cpu} exceeds the {TX_MAX_INSTRUCTIONS} \
             per-transaction instruction budget"
        );
    }
}
