use gruen_experiments::{field::*, mikros::Interpolation, *};

// Debug encodes the same explicit base-field coordinates and ordered message
// fields in both frozen/candidate types. No floating-point tolerance or digest
// collision is accepted: compare every printed coordinate of the whole proof.
macro_rules! compare_field {
    ($name:ident, $base:ident, $ext:ident) => {
        #[test]
        fn $name() {
            type B = $base;
            type E = $ext;
            type RB = reference::field::$base;
            type RE = reference::field::$ext;
            let k = E::DEGREE.ilog2() as usize;
            for n in [k, k + 1, 13, 16] {
                let trace = Trace::<B>::random(n, 92);
                let rt = reference::Trace::<RB>::random(n, 92);
                assert_eq!(format!("{:?}", trace.rows), format!("{:?}", rt.rows));
                for boundary in [false, true] {
                    let mut tape = Challenges::<E>::deterministic(n, 9);
                    let mut r_tape = reference::Challenges::<RE>::deterministic(n, 9);
                    if boundary {
                        tape.z.fill(E::ONE);
                        tape.rounds.fill(E::ZERO);
                        tape.skip = E::ONE;
                        use reference::field::Field as _;
                        r_tape.z.fill(RE::ONE);
                        r_tape.rounds.fill(RE::ZERO);
                        r_tape.skip = RE::ONE;
                    }
                    let reference_gruen: Vec<_> =
                        [reference::Method::Gruen, reference::Method::Skip(k)]
                            .into_iter()
                            .map(|m| {
                                let (p, _) = reference::prove(&rt, &r_tape, m).unwrap();
                                let oracle =
                                    reference::oracle_evaluations(&rt, &r_tape, m.skip()).unwrap();
                                reference::verify::<RB, RE>(&p, &r_tape, n, m.skip(), oracle)
                                    .unwrap();
                                (format!("{p:?}"), format!("{oracle:?}"))
                            })
                            .collect();
                    let (rp, _) = reference::mikros::prove(
                        &rt,
                        &r_tape,
                        reference::mikros::Interpolation::Reference,
                    )
                    .unwrap();
                    let ro = reference::mikros::oracle_evaluations(&rt, &r_tape).unwrap();
                    reference::mikros::verify::<RB, RE>(&rp, &r_tape, n, ro).unwrap();
                    for threads in [1, 4, 64] {
                        let pool = rayon::ThreadPoolBuilder::new()
                            .num_threads(threads)
                            .build()
                            .unwrap();
                        pool.install(|| {
                            assert_eq!(rayon::current_num_threads(), threads);
                            for (idx, method) in
                                [Method::Gruen, Method::Skip(k)].into_iter().enumerate()
                            {
                                let (p, _) = prove(&trace, &tape, method).unwrap();
                                let oracle =
                                    oracle_evaluations(&trace, &tape, method.skip()).unwrap();
                                verify::<B, E>(&p, &tape, n, method.skip(), oracle).unwrap();
                                assert_eq!(format!("{p:?}"), reference_gruen[idx].0);
                                assert_eq!(format!("{oracle:?}"), reference_gruen[idx].1);
                            }
                            for mode in [Interpolation::Reference, Interpolation::Optimized] {
                                let (p, _) = mikros::prove(&trace, &tape, mode).unwrap();
                                let oracle = mikros::oracle_evaluations(&trace, &tape).unwrap();
                                mikros::verify::<B, E>(&p, &tape, n, oracle).unwrap();
                                assert_eq!(format!("{p:?}"), format!("{rp:?}"));
                                assert_eq!(format!("{oracle:?}"), format!("{ro:?}"));
                            }
                        });
                    }
                }
            }
        }
    };
}
compare_field!(
    mersenne31_parallel_matches_frozen_serial,
    Mersenne31,
    EMersenne31
);
compare_field!(
    goldilocks_parallel_matches_frozen_serial,
    Goldilocks,
    EGoldilocks
);
compare_field!(
    koalabear_parallel_matches_frozen_serial,
    KoalaBear,
    EKoalaBear
);
