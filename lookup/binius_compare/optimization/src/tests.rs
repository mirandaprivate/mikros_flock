use super::*;
use std::sync::Once;

fn setup() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build_global()
            .unwrap();
        assert!(flock_parallel::init_all_core_pool(1));
    });
}

fn input(width: usize) -> (Vec<u128>, Vec<u128>, Vec<usize>) {
    let n = 1usize << 15;
    let mask = (1u128 << width) - 1;
    let f: Vec<_> = (0..n).map(|i| ((i * 37 + i / 7) as u128) & mask).collect();
    let sigma: Vec<_> = (0..n).map(|i| (i * 3 + 17) & (n - 1)).collect();
    let g = sigma.iter().map(|&i| f[i]).collect();
    (f, g, sigma)
}

#[test]
fn honest_both_and_all_terminal_authentication() {
    setup();
    for width in [1, 3, 8] {
        let (f, g, sigma) = input(width);
        let mut caps = None;
        for system in [System::Mikros, System::Flock] {
            let out = prove(&f, &g, &sigma, width, system, b"test").unwrap();
            let (_, tail) = verify(&out.statement, &out.proof, system, b"test").unwrap();
            assert_eq!(tail, out.transcript_tail);
            let serialized_caps = bincode::serialize(&out.statement).unwrap();
            if let Some(prev) = &caps {
                assert_eq!(prev, &serialized_caps);
            }
            caps = Some(serialized_caps);
            assert!(verify(&out.statement, &out.proof, system, b"other-context").is_err());
            assert!(
                verify(
                    &out.statement,
                    &out.proof,
                    if system == System::Mikros {
                        System::Flock
                    } else {
                        System::Mikros
                    },
                    b"test"
                )
                .is_err()
            );

            let mut p = out.proof.clone();
            p.sources.pop();
            assert!(verify(&out.statement, &p, system, b"test").is_err());
            for side in 0..2 {
                let mut p = out.proof.clone();
                p.sources[side].columns[0] += F128::ONE;
                assert!(verify(&out.statement, &p, system, b"test").is_err());
                let mut p = out.proof.clone();
                p.sources[side].opening.ring_switches[0].s_hat_v[0] += F128::ONE;
                assert!(verify(&out.statement, &p, system, b"test").is_err());
                let mut p = out.proof.clone();
                p.sources[side].opening.ring_switches[0].s_hat_v.pop();
                assert!(verify(&out.statement, &p, system, b"test").is_err());
                let mut p = out.proof.clone();
                p.sources[side].opening.ligerito.final_proof.yr[0] += F128::ONE;
                assert!(verify(&out.statement, &p, system, b"test").is_err());
            }
            if width == 3 {
                let mut p = out.proof.clone();
                p.sources[0].columns[3] = F128::ONE;
                assert!(verify(&out.statement, &p, system, b"test").is_err());
                // Keep the PIOP's weighted sum EXACTLY unchanged; only the
                // randomized bit-MLE/PCS authentication can catch this edit.
                let basis_width = if system == System::Mikros {
                    width + 15
                } else {
                    width
                };
                let basis = SmallField::for_width(basis_width).unwrap().basis();
                let mut p = out.proof.clone();
                p.sources[0].columns[0] += basis[1];
                p.sources[0].columns[1] += basis[0];
                assert!(verify(&out.statement, &p, system, b"test").is_err());
            }
            let mut s = out.statement.clone();
            s.sigma.swap(0, 1);
            assert!(verify(&s, &out.proof, system, b"test").is_err());
            let mut s = out.statement.clone();
            s.commitments.swap(0, 1);
            assert!(verify(&s, &out.proof, system, b"test").is_err());
            let mut s = out.statement.clone();
            s.commitments[0].params.log_inv_rate += 1;
            assert!(verify(&s, &out.proof, system, b"test").is_err());
        }
    }
}

#[test]
fn reject_bad_inputs_and_public_mapping() {
    setup();
    let (f, mut g, mut sigma) = input(8);
    g[0] ^= 1;
    for system in [System::Mikros, System::Flock] {
        assert!(prove(&f, &g, &sigma, 8, system, b"").is_err());
    }
    g[0] ^= 1;
    sigma[0] = sigma[1];
    assert!(prove(&f, &g, &sigma, 8, System::Mikros, b"").is_err());
    assert!(parameters(13, 8).is_err());
    assert!(parameters(20, 65).is_err());
}

#[test]
fn changed_commitment_cannot_reuse_valid_piop_or_openings() {
    setup();
    let (f, g, sigma) = input(8);
    let params = parameters(15, 8).unwrap().0;
    let mut other = f.clone();
    other[7] ^= 1;
    let bits = PackedBits::from_rows(&other, 8).unwrap();
    let (wrong, _) = commit_bits(bits, &params);
    for system in [System::Mikros, System::Flock] {
        let out = prove(&f, &g, &sigma, 8, system, b"bind").unwrap();
        let mut statement = out.statement.clone();
        statement.commitments[0] = wrong.clone();
        assert!(verify(&statement, &out.proof, system, b"bind").is_err());
        let mut p = out.proof.clone();
        match &mut p.piop {
            PiopProof::Mikros { proof, .. } => proof.roots[0] += F128::ONE,
            PiopProof::Flock(proof) => proof.f_eval += F128::ONE,
            PiopProof::Batched { proof, .. } => proof.roots[0] += F128::ONE,
            PiopProof::Packed { proof, .. } => proof.roots[0] += F128::ONE,
        }
        assert!(verify(&out.statement, &p, system, b"bind").is_err());
    }
}

#[test]
fn legacy_modes_equal_original_full_pcs_bytes() {
    setup();
    let (f, g, sigma) = input(3);
    let old = baseline_adapter::prove(
        &f,
        &g,
        &sigma,
        3,
        baseline_adapter::System::Mikros,
        b"golden",
    )
    .unwrap();
    let expected = bincode::serialize(&old.proof).unwrap();
    for mode in [
        Mode::Reference,
        Mode::FullExtension,
        Mode::Direct,
        Mode::Lookup,
        Mode::Fused,
    ] {
        let new = prove_with_mode(&f, &g, &sigma, 3, System::Mikros, b"golden", mode).unwrap();
        assert_eq!(
            bincode::serialize(&new.proof).unwrap(),
            expected,
            "{mode:?}"
        );
        assert_eq!(new.transcript_tail, old.transcript_tail);
    }
}

#[test]
fn bit_bridge_and_public_tags_equal_scalar_for_all_widths() {
    setup();
    for log in [3, 6, 7, 10, 15] {
        for width in [1, 3, 8, 20, 32, 64] {
            let n = 1usize << log;
            let bits = PackedBits::from_fn_parallel(n, width, |i| {
                ((i as u128 * 0x912487a7) ^ ((i as u128 + 1) << 31)) & ((1u128 << width) - 1)
            })
            .unwrap();
            let point: Vec<_> = (0..log)
                .map(|i| F128::new((i * 17 + 11) as u64, (i * 31 + 1) as u64))
                .collect();
            assert_eq!(
                bridge_fast::column_evals(&bits, &point),
                column_evals(&bits, &point)
            );
        }
    }
    let (f, g, sigma) = input(8);
    let out = prove_with_mode(&f, &g, &sigma, 8, System::Mikros, b"bridge", Mode::Batched).unwrap();
    let fast = prove_with_mode(&f, &g, &sigma, 8, System::Mikros, b"bridge", Mode::Bridge).unwrap();
    assert_eq!(
        bincode::serialize(&out.proof).unwrap(),
        bincode::serialize(&fast.proof).unwrap()
    );
    assert_eq!(out.transcript_tail, fast.transcript_tail);
}

#[test]
fn packed_quotient_pcs_authenticates_every_terminal() {
    setup();
    for width in [1, 3, 8, 32, 64] {
        let (f, g, sigma) = input(width);
        let out = prove_with_mode(
            &f,
            &g,
            &sigma,
            width,
            System::Mikros,
            b"packed-pcs",
            Mode::Packed,
        )
        .unwrap();
        let decoded: Proof =
            bincode::deserialize(&bincode::serialize(&out.proof).unwrap()).unwrap();
        let (_, tail) = verify(&out.statement, &decoded, System::Mikros, b"packed-pcs").unwrap();
        assert_eq!(tail, out.transcript_tail);
        assert!(verify(&out.statement, &decoded, System::Mikros, b"other").is_err());
        for side in 0..2 {
            let mut bad = decoded.clone();
            bad.sources[side].columns[0] += F128::ONE;
            assert!(verify(&out.statement, &bad, System::Mikros, b"packed-pcs").is_err());
            let mut bad = decoded.clone();
            bad.sources[side].opening.ring_switches[0].s_hat_v[0] += F128::ONE;
            assert!(verify(&out.statement, &bad, System::Mikros, b"packed-pcs").is_err());
            let mut bad = decoded.clone();
            bad.sources[side].opening.ligerito.final_proof.yr[0] += F128::ONE;
            assert!(verify(&out.statement, &bad, System::Mikros, b"packed-pcs").is_err());
            if width > 1 {
                let basis = SmallField::for_width(width + 15).unwrap().basis();
                let mut bad = decoded.clone();
                bad.sources[side].columns[0] += basis[1];
                bad.sources[side].columns[1] += basis[0];
                assert!(verify(&out.statement, &bad, System::Mikros, b"packed-pcs").is_err());
            }
        }
        let mut changed = out.statement.clone();
        changed.sigma.swap(0, 1);
        assert!(verify(&changed, &decoded, System::Mikros, b"packed-pcs").is_err());
        let mut bad = decoded.clone();
        bad.sources.pop();
        assert!(verify(&out.statement, &bad, System::Mikros, b"packed-pcs").is_err());
        if width == 3 {
            let mut bad = decoded.clone();
            bad.sources[0].columns[3] = F128::ONE;
            assert!(verify(&out.statement, &bad, System::Mikros, b"packed-pcs").is_err());
        }
    }
}
