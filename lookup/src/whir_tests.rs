use crate::{
    field::from_raw,
    whir_field::{Bit, Ghash},
    whir_pcs::{Parameters, Pcs},
    whir_rs,
};
use ark_ff::{AdditiveGroup, Field, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, EmptyFlags};
use flock_field::F128;
fn next(s: &mut u128) -> u128 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}
#[test]
fn binary_ark_field_and_canonical_codec() {
    assert_eq!(Ghash::characteristic(), &[2]);
    assert_eq!(Ghash::extension_degree(), 128);
    assert_eq!(Ghash(1u128 << 127) * Ghash(2), Ghash(0x87));
    for n in 0..10u64 {
        let b = Bit::from(n);
        assert_eq!(b.into_bigint().0[0], n & 1);
        assert_eq!(b + b, Bit::ZERO);
        assert_eq!(b.sqrt(), Some(b));
        assert_eq!(Ghash::from(n), Ghash((n & 1) as u128));
    }
    let mut seed = 0xf125126ff721891273198273129_u128;
    for _ in 0..100 {
        let a = Ghash(next(&mut seed));
        let b = Ghash(next(&mut seed));
        let c = Ghash(next(&mut seed));
        assert_eq!(a * (b + c), a * b + a * c);
        assert_eq!(a * a, a.square());
        assert_eq!(a * a.inverse().unwrap(), Ghash::ONE);
        assert_eq!(a.sqrt().unwrap().square(), a);
        assert_eq!(a.pow([0, 0, 1]), a);
        assert_eq!(
            Ghash::from_base_prime_field_elems(a.to_base_prime_field_elements()),
            Some(a)
        );
        let mut bytes = Vec::new();
        a.serialize_compressed(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 16);
        assert_eq!(Ghash::deserialize_compressed(bytes.as_slice()).unwrap(), a);
        assert_eq!(
            Ghash::from_random_bytes_with_flags::<EmptyFlags>(&bytes)
                .unwrap()
                .0,
            a
        );
        assert!(Ghash::deserialize_compressed(&bytes[..15]).is_err());
    }
}
#[test]
fn additive_rs_matches_independent_horner() {
    let encoder = whir_rs::register();
    let mut seed = 711237;
    for log in 0..=8 {
        let n = 1 << log;
        for len in [0, 1, n / 2, n] {
            let coefficients = (0..len).map(|_| Ghash(next(&mut seed))).collect::<Vec<_>>();
            let actual = encoder.evaluate(&coefficients, n);
            for (i, &value) in actual.iter().enumerate() {
                assert_eq!(
                    value,
                    coefficients
                        .iter()
                        .rev()
                        .fold(Ghash::ZERO, |v, &c| v * Ghash(i as u128) + c),
                    "log={log} len={len} i={i}"
                );
            }
        }
    }
}
#[test]
fn whir_authenticates_full_extension_covectors() {
    let mut seed = 7865283225;
    for log in [1, 3, 6, 10] {
        let pcs = Pcs::new(Parameters::diagnostic(log)).unwrap();
        let values = (0..1 << log)
            .map(|_| from_raw(next(&mut seed)))
            .collect::<Vec<_>>();
        let weights = (0..3)
            .map(|_| {
                (0..1 << log)
                    .map(|_| from_raw(next(&mut seed)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let claims = weights
            .iter()
            .map(|w| {
                w.iter()
                    .zip(&values)
                    .fold(F128::ZERO, |s, (&w, &v)| s + w * v)
            })
            .collect::<Vec<_>>();
        let data = pcs.commit(values).unwrap();
        let commitment = data.commitment().clone();
        let proof = pcs.open(&data, &weights, &claims, b"context").unwrap();
        drop(data);
        let verifier = Pcs::new(Parameters::diagnostic(log)).unwrap();
        verifier
            .verify(&commitment, &weights, &claims, &proof, b"context")
            .unwrap();
        assert!(verifier.diagnostic_security_bits(3).unwrap() >= 96.);
        assert!(
            verifier
                .verify(&commitment, &weights, &claims, &proof, b"wrong")
                .is_err()
        );
        let mut bad = claims.clone();
        bad[0] += F128::ONE;
        assert!(
            verifier
                .verify(&commitment, &weights, &bad, &proof, b"context")
                .is_err()
        );
        let mut extra = proof.clone();
        extra.push(0);
        assert!(
            verifier
                .verify(&commitment, &weights, &claims, &extra, b"context")
                .is_err()
        );
        let mut changed = proof.clone();
        let end = changed.len() - 1;
        changed[end] ^= 1;
        assert!(
            verifier
                .verify(&commitment, &weights, &claims, &changed, b"context")
                .is_err()
        );
        assert!(
            verifier
                .verify(
                    &commitment,
                    &weights,
                    &claims,
                    &proof[..proof.len() - 1],
                    b"context"
                )
                .is_err()
        );
    }
}

#[test]
fn standard_whir_odd_field_regression() {
    // The Standard-mode half removal must preserve the sibling's odd-field
    // protocol too. Exercise its original RS backend with a nonzero claim.
    use whir::{
        algebra::{
            embedding::Identity,
            fields::Field64_2 as E,
            linear_form::{Covector, LinearForm},
        },
        buffer::Buffer,
        parameters::ProtocolParameters,
        protocols::{params::DecodingRegime, whir::Config},
        transcript::{DomainSeparator, Proof, ProverState, VerifierState, codecs::Empty},
    };
    let config = Config::<Identity<E>>::new(
        16,
        &ProtocolParameters {
            decoding_regime: DecodingRegime::Unique,
            starting_log_inv_rate: 1,
            initial_folding_factor: 2,
            folding_factor: 2,
            security_level: 96,
            pow_bits: 0,
            batch_size: 1,
            hash_id: whir::hash::BLAKE3,
        },
    );
    let entries = (1u64..=16).map(E::from).collect::<Vec<_>>();
    let value = entries.iter().copied().sum::<E>();
    assert_ne!(value, E::ZERO);
    let vector = Buffer::from(entries);
    let ds = DomainSeparator::protocol(&config).instance(&Empty);
    let mut prover = ProverState::new_std(&ds);
    let witness = config.commit(&mut prover, &[&vector]);
    let _ = config.prove(
        &mut prover,
        &[&vector],
        vec![&witness],
        vec![Box::new(Covector::new(vec![E::ONE; 16]))],
        Buffer::from(vec![value]),
    );
    let p = prover.proof();
    let proof = Proof::from_raw_parts(p.narg_string, p.hints);
    let mut verifier = VerifierState::new_std(&ds, &proof);
    let commitment = config.receive_commitment(&mut verifier).unwrap();
    let final_claim = config
        .verify(&mut verifier, &[&commitment], &[value])
        .unwrap();
    let form = Covector::new(vec![E::ONE; 16]);
    final_claim.verify([&form as &dyn LinearForm<E>]).unwrap();
    verifier.check_eof().unwrap();
}
