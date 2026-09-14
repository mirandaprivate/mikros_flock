use gruen_experiments::{
    Challenges, Trace,
    field::*,
    mikros::{self, Interpolation as I, Setup},
    poly::{eq, eq_table, eval},
};

fn interpolation<B: Prime, E: Extension<B>>() {
    let mut rng = Rng(67);
    for k in [2, 4, 8, 16] {
        let reference = Setup::<B>::new(k, I::Reference).unwrap();
        let fast = Setup::<B>::new(k, I::Optimized).unwrap();
        assert_eq!(reference.basis, fast.basis);
        assert_eq!(reference.quotient_basis, fast.quotient_basis);
        for _ in 0..5 {
            let coefficients = (0..k).map(|_| B::random(&mut rng)).collect::<Vec<_>>();
            let values = (1..=k)
                .map(|x| eval(&coefficients, B::from_u64(x as u64)))
                .collect::<Vec<_>>();
            let mut extra = vec![B::ZERO; k - 1];
            mikros::extrapolate(&values, &mut extra);
            for (j, &v) in extra.iter().enumerate() {
                assert_eq!(v, eval(&coefficients, B::from_u64((k + 1 + j) as u64)));
            }
            let mut xs = vec![E::random(&mut rng), E::ZERO];
            xs.extend((1..=k).map(|x| E::from_u64(x as u64)));
            for x in xs {
                let weights = fast.lagrange(x, I::Optimized);
                assert_eq!(weights, reference.lagrange(x, I::Reference));
                assert_eq!(weights.iter().fold(E::ZERO, |s, &w| s + w), E::ONE);
                let actual = weights
                    .iter()
                    .zip(&values)
                    .fold(E::ZERO, |s, (&w, &v)| s + w.scale(v));
                let expected = coefficients
                    .iter()
                    .rev()
                    .fold(E::ZERO, |s, &c| s * x + E::lift(c));
                assert_eq!(actual, expected);
            }
        }
    }
}
fn quotient<B: Prime + Extension<B>, E: Extension<B>>() {
    let kappa = E::DEGREE;
    let k = kappa.ilog2() as usize;
    let trace = Trace::<B>::random(k + 3, 318);
    let tape = Challenges::<E>::deterministic(k + 3, 191);
    let weights = eq_table(&tape.z[k..]);
    let setup = Setup::<B>::new(kappa, I::Reference).unwrap();
    let expected_helpers = mikros::helpers(&trace, &weights, &setup, I::Reference);
    let fast = Setup::<B>::new(kappa, I::Optimized).unwrap();
    assert_eq!(
        expected_helpers,
        mikros::helpers(&trace, &weights, &fast, I::Optimized)
    );
    // Independently expand A(X)B(X)-interpolate(A(h)B(h)) coefficientwise,
    // then perform monic schoolbook division by product(X-h).
    let mut numerator = vec![E::ZERO; 2 * kappa - 1];
    for (row, &w) in trace.rows.chunks_exact(kappa).zip(&weights) {
        let mut a = vec![B::ZERO; kappa];
        let mut b = a.clone();
        let mut c = a.clone();
        for (j, values) in row.iter().enumerate() {
            for d in 0..kappa {
                a[d] = a[d] + values[0] * setup.basis[j][d];
                b[d] = b[d] + values[1] * setup.basis[j][d];
                c[d] = c[d] + values[0] * values[1] * setup.basis[j][d];
            }
        }
        for i in 0..kappa {
            numerator[i] = numerator[i] - w.scale(c[i]);
            for j in 0..kappa {
                numerator[i + j] = numerator[i + j] + w.scale(a[i] * b[j]);
            }
        }
    }
    let mut q = vec![E::ZERO; kappa - 1];
    for i in (kappa..numerator.len()).rev() {
        let factor = numerator[i];
        q[i - kappa] = factor;
        for j in 0..=kappa {
            numerator[i - kappa + j] = numerator[i - kappa + j] - factor.scale(setup.vanishing[j]);
        }
    }
    assert_eq!(numerator, vec![E::ZERO; 2 * kappa - 1]);
    assert_eq!(q, expected_helpers.1);
}
fn direct_oracle<B: Prime, E: Extension<B>>(trace: &Trace<B>, tape: &Challenges<E>) -> [E; 3] {
    let k = E::DEGREE.ilog2() as usize;
    let mut out = [E::ZERO; 3];
    for (i, row) in trace.rows.iter().enumerate() {
        let mut a = E::ONE;
        let mut original_high = E::ONE;
        let mut c = E::ONE;
        for bit in 0..tape.z.len() {
            let boolean = if i >> bit & 1 == 1 { E::ONE } else { E::ZERO };
            a = a * eq(tape.rounds[bit], boolean);
            c = c * eq(tape.z[bit], boolean);
            if bit >= k {
                original_high = original_high * eq(tape.z[bit], boolean);
            }
        }
        out[0] = out[0] + a.scale(row[0]);
        out[1] = out[1] + (a * original_high).scale(row[1]);
        out[2] = out[2] + c.scale(row[2]);
    }
    out
}
fn protocol<B: Prime + Extension<B>, E: Extension<B>>() {
    let k = E::DEGREE.ilog2() as usize;
    for n in [k, k + 1, 7, 10] {
        for seed in 0..3 {
            let trace = Trace::<B>::random(n, seed);
            let tape = Challenges::<E>::deterministic(n, seed + 61);
            let trusted = mikros::oracle_evaluations(&trace, &tape).unwrap();
            assert_eq!(trusted, direct_oracle(&trace, &tape));
            let (reference, _) = mikros::prove(&trace, &tape, I::Reference).unwrap();
            let (optimized, _) = mikros::prove(&trace, &tape, I::Optimized).unwrap();
            assert_eq!(reference, optimized);
            mikros::verify::<B, E>(&optimized, &tape, n, trusted).unwrap();
            assert_eq!(optimized.field_elements(), 4 * E::DEGREE + 2 * (n - k) + 2);
            let mut wrong_trace = trace.clone();
            wrong_trace.rows[0][2] = wrong_trace.rows[0][2] + B::ONE;
            assert!(mikros::prove(&wrong_trace, &tape, I::Optimized).is_err());
            assert!(
                mikros::verify::<B, E>(
                    &optimized,
                    &tape,
                    n,
                    mikros::oracle_evaluations(&wrong_trace, &tape).unwrap()
                )
                .is_err()
            );
            let reject = |p| assert!(mikros::verify::<B, E>(&p, &tape, n, trusted).is_err());
            for j in 0..E::DEGREE {
                let mut p = optimized.clone();
                p.column_products[j] = p.column_products[j] + E::ONE;
                reject(p);
                let mut p = optimized.clone();
                p.columns_a[j] = p.columns_a[j] + E::ONE;
                reject(p);
                let mut p = optimized.clone();
                p.columns_b[j] = p.columns_b[j] + E::ONE;
                reject(p);
            }
            for j in 0..optimized.quotient.len() {
                let mut p = optimized.clone();
                p.quotient[j] = p.quotient[j] + E::ONE;
                reject(p);
            }
            for i in 0..optimized.rounds.len() {
                for j in 0..2 {
                    let mut p = optimized.clone();
                    p.rounds[i][j] = p.rounds[i][j] + E::ONE;
                    reject(p);
                }
            }
            let mut p = optimized.clone();
            p.c_evaluation = p.c_evaluation + E::ONE;
            reject(p);
            let mut p = optimized.clone();
            p.columns_a.pop();
            reject(p);
            let mut p = optimized.clone();
            p.rounds.push([E::ZERO; 2]);
            reject(p);
            for j in 0..3 {
                let mut bad = trusted;
                bad[j] = bad[j] + E::ONE;
                assert!(mikros::verify::<B, E>(&optimized, &tape, n, bad).is_err());
            }
        }
    }
    let trace = Trace::<B>::random(k + 3, 908);
    let mut tape = Challenges::<E>::deterministic(k + 3, 582);
    tape.z.fill(E::ONE);
    tape.rounds.fill(E::ZERO);
    for node in 1..=E::DEGREE {
        tape.skip = E::from_u64(node as u64);
        let (p, _) = mikros::prove(&trace, &tape, I::Optimized).unwrap();
        let (r, _) = mikros::prove(&trace, &tape, I::Reference).unwrap();
        assert_eq!(p, r);
        mikros::verify::<B, E>(&p, &tape, k + 3, direct_oracle(&trace, &tape)).unwrap();
    }
}
#[test]
fn gold_interpolation() {
    interpolation::<Goldilocks, EGoldilocks>();
}
#[test]
fn koala_interpolation() {
    interpolation::<KoalaBear, EKoalaBear>();
}
#[test]
fn gold_quotient() {
    quotient::<Goldilocks, EGoldilocks>();
}
#[test]
fn koala_quotient() {
    quotient::<KoalaBear, EKoalaBear>();
}
#[test]
fn gold_protocol() {
    protocol::<Goldilocks, EGoldilocks>();
}
#[test]
fn koala_protocol() {
    protocol::<KoalaBear, EKoalaBear>();
}
#[test]
fn koala_quartic_coordinates_match_independent_schoolbook() {
    EKoalaBear::validate();
    let v = EKoalaBear::generator();
    assert_eq!(v.pow(4), EKoalaBear::from_u64(3));
    let coords = |x: EKoalaBear| [x.0.0, x.1.0, x.0.1, x.1.1]; // [1,v,v^2,v^3]
    let mut rng = Rng(2019);
    for _ in 0..100 {
        let a = EKoalaBear::random(&mut rng);
        let b = EKoalaBear::random(&mut rng);
        let aa = coords(a);
        let bb = coords(b);
        let mut c = [KoalaBear::ZERO; 7];
        for i in 0..4 {
            for j in 0..4 {
                c[i + j] = c[i + j] + aa[i] * bb[j];
            }
        }
        for d in (4..7).rev() {
            c[d - 4] = c[d - 4] + c[d] * KoalaBear::from_u64(3);
        }
        assert_eq!(coords(a * b), c[..4]);
    }
}
