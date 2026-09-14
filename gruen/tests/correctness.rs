use gruen_experiments::{field::*, poly::*, *};

fn field_checks<B: Prime, E: Extension<B>>() {
    E::validate();
    assert_eq!(std::mem::size_of::<E>(), 16);
    let mut rng = Rng(100);
    for _ in 0..100 {
        let a = E::random(&mut rng);
        let b = E::random(&mut rng);
        let c = E::random(&mut rng);
        assert_eq!(a * (b + c), a * b + a * c);
        assert_eq!((a * b) * c, a * (b * c));
        assert_eq!(a + (-a), E::ZERO);
        if a != E::ZERO {
            assert_eq!(a * a.inverse().unwrap(), E::ONE);
        }
        assert_eq!(a.scale(B::from_u64(3)), a * E::from_u64(3));
        assert_eq!(a.mul_generator(), a * E::generator());
        let x = B::random(&mut rng);
        let y = B::random(&mut rng);
        assert_eq!(
            (x * y).value(),
            ((x.value() as u128 * y.value() as u128) % B::MODULUS as u128) as u64
        );
    }
    for a in [0, 1, 2, B::MODULUS - 2, B::MODULUS - 1] {
        for b in [0, 1, 2, B::MODULUS - 2, B::MODULUS - 1] {
            let (x, y) = (B::from_u64(a), B::from_u64(b));
            assert_eq!(
                (x + y).value(),
                ((a as u128 + b as u128) % B::MODULUS as u128) as u64
            );
            assert_eq!(
                (x * y).value(),
                ((a as u128 * b as u128) % B::MODULUS as u128) as u64
            );
            assert_eq!(
                (x - y).value(),
                ((a as u128 + B::MODULUS as u128 - b as u128) % B::MODULUS as u128) as u64
            );
        }
    }
}

fn transform_checks<B: Prime + Extension<B>, E: Extension<B>>() {
    let mut rng = Rng(12);
    for k in 0..=B::TWO_ADICITY.min(5) {
        let d = Domain::<B>::new(k).unwrap();
        let b = 1 << k;
        let coefficients: Vec<B> = (0..b).map(|_| B::random(&mut rng)).collect();
        let mut values = coefficients.clone();
        d.fft(&mut values, false);
        for (&x, &v) in d.nodes.iter().zip(&values) {
            assert_eq!(eval(&coefficients, x), v);
        }
        let mut restored = values.clone();
        d.fft(&mut restored, true);
        assert_eq!(coefficients, restored);
        d.coset_lde(&mut values);
        for (&x, &v) in d.nodes.iter().zip(&values) {
            assert_eq!(eval(&coefficients, x * d.shift), v);
        }
        d.coset_interpolate(&mut values);
        assert_eq!(coefficients, values);
        for x in [
            E::random(&mut rng),
            E::lift(d.nodes[0]),
            E::lift(d.nodes[b - 1]),
        ] {
            let w = d.lagrange(x);
            assert_eq!(w.iter().copied().fold(E::ZERO, |a, b| a + b), E::ONE);
            let base_values: Vec<_> = d.nodes.iter().map(|&h| eval(&coefficients, h)).collect();
            let got = w
                .iter()
                .zip(base_values)
                .fold(E::ZERO, |a, (&w, v)| a + w.scale(v));
            assert_eq!(
                got,
                eval(
                    &coefficients
                        .iter()
                        .copied()
                        .map(E::lift)
                        .collect::<Vec<_>>(),
                    x
                )
            );
        }
    }
    assert!(Domain::<B>::new(B::TWO_ADICITY + 1).is_err());
}

/// Independent O(B^2) DFT interpolation and schoolbook polynomial division.
fn quotient_checks<B: Prime + Extension<B>, E: Extension<B>>() {
    for k in 1..=B::TWO_ADICITY.min(4) {
        let n = k + 2;
        let b = 1 << k;
        let trace = Trace::<B>::random(n, 391);
        let tape = Challenges::<E>::deterministic(n, 155);
        let d = Domain::<B>::new(k).unwrap();
        let weights = eq_table(&tape.z[k..]);
        let actual = skip_quotient(&trace, &d, &weights);
        let mut g = vec![E::ZERO; 2 * b - 1];
        for (chunk, &w) in trace.rows.chunks_exact(b).zip(&weights) {
            let mut cols = vec![vec![B::ZERO; b]; 3];
            for (col, coeffs) in cols.iter_mut().enumerate() {
                for (j, coefficient) in coeffs.iter_mut().enumerate() {
                    for (i, row) in chunk.iter().enumerate() {
                        *coefficient =
                            *coefficient + row[col] * d.nodes[i].inverse().unwrap().pow(j as u128);
                    }
                    *coefficient = *coefficient * B::from_u64(b as u64).inverse().unwrap();
                }
            }
            for i in 0..b {
                g[i] = g[i] - w.scale(cols[2][i]);
                for j in 0..b {
                    g[i + j] = g[i + j] + w.scale(cols[0][i] * cols[1][j]);
                }
            }
        }
        let mut expected = vec![E::ZERO; b - 1];
        for i in (b..g.len()).rev() {
            expected[i - b] = g[i];
            g[i - b] = g[i - b] + g[i];
            g[i] = E::ZERO;
        }
        assert!(
            g.iter().all(|&x| x == E::ZERO),
            "valid trace numerator divisible by X^B-1"
        );
        assert_eq!(actual, expected);
    }
}

fn protocol_checks<B: Prime + Extension<B>, E: Extension<B>>() {
    for n in 0..=7 {
        let trace = Trace::<B>::random(n, 900 + n as u64);
        for seed in 0..3 {
            let tape = Challenges::<E>::deterministic(n, 200 + seed);
            let (ordinary, _) = prove(&trace, &tape, Method::Regular).unwrap();
            let (gruen, _) = prove(&trace, &tape, Method::Gruen).unwrap();
            assert_eq!(
                ordinary.rounds, gruen.rounds,
                "Gruen recovery preserves the exact regular proof"
            );
            let mut methods = vec![Method::Regular, Method::Gruen];
            methods.extend(
                (1..=n.min(5))
                    .filter(|&k| validate_skip_domain::<B>(k).is_ok())
                    .map(Method::Skip),
            );
            for method in methods {
                let (proof, _) = prove(&trace, &tape, method).unwrap();
                let trusted = oracle_evaluations(&trace, &tape, method.skip()).unwrap();
                verify::<B, E>(&proof, &tape, n, method.skip(), trusted).unwrap();
                let mut bad = proof.clone();
                bad.terminal[0] = bad.terminal[0] + E::ONE;
                assert!(verify::<B, E>(&bad, &tape, n, method.skip(), trusted).is_err());
                if !proof.rounds.is_empty() {
                    let mut bad = proof.clone();
                    bad.rounds[0][2] = bad.rounds[0][2] + E::ONE;
                    assert!(verify::<B, E>(&bad, &tape, n, method.skip(), trusted).is_err());
                }
                if !proof.quotient.is_empty() {
                    let mut bad = proof.clone();
                    bad.quotient[0] = bad.quotient[0] + E::ONE;
                    assert!(verify::<B, E>(&bad, &tape, n, method.skip(), trusted).is_err());
                }
                let mut bad = proof.clone();
                bad.rounds.push([E::ZERO; 3]);
                assert!(verify::<B, E>(&bad, &tape, n, method.skip(), trusted).is_err());
                let mut invalid = trace.clone();
                invalid.rows[0][2] = invalid.rows[0][2] + B::ONE;
                let (bad, _) = prove(&invalid, &tape, method).unwrap();
                let bad_oracle = oracle_evaluations(&invalid, &tape, method.skip()).unwrap();
                assert!(verify::<B, E>(&bad, &tape, n, method.skip(), bad_oracle).is_err());
            }
        }
    }
    // The reference's barycentric expression needs explicit handling at subgroup nodes.
    let trace = Trace::<B>::random(4, 18);
    for endpoint in [E::ZERO, E::ONE] {
        let tape = Challenges {
            z: vec![endpoint; 4],
            rounds: vec![endpoint; 4],
            skip: E::ONE,
        };
        for method in [
            Method::Regular,
            Method::Gruen,
            Method::Skip(1),
            Method::Skip(kappa_skip_rounds::<B, E>().unwrap()),
        ] {
            let (proof, _) = prove(&trace, &tape, method).unwrap();
            let trusted = oracle_evaluations(&trace, &tape, method.skip()).unwrap();
            verify::<B, E>(&proof, &tape, 4, method.skip(), trusted).unwrap();
        }
    }
    assert!(
        prove(
            &trace,
            &Challenges::<E>::deterministic(4, 0),
            Method::Skip(5)
        )
        .is_err()
    );
    assert!(prove(&trace, &Challenges::<E>::deterministic(3, 0), Method::Gruen).is_err());
    assert!(Trace::<B> { rows: vec![] }.n().is_err());
    assert!(
        Trace::<B> {
            rows: vec![[B::ZERO; 3]; 3]
        }
        .n()
        .is_err()
    );
}

/// Independent divided-difference interpolation, without the production
/// cardinal-polynomial matrices or FFT.
fn newton_interpolate<B: Prime>(nodes: &[B], values: &[B]) -> Vec<B> {
    let mut differences = values.to_vec();
    for degree in 1..nodes.len() {
        for i in (degree..nodes.len()).rev() {
            differences[i] = (differences[i] - differences[i - 1])
                * (nodes[i] - nodes[i - degree]).inverse().unwrap();
        }
    }
    let mut out = vec![B::ZERO; nodes.len()];
    let mut basis = vec![B::ONE];
    for i in 0..nodes.len() {
        for (j, &c) in basis.iter().enumerate() {
            out[j] = out[j] + c * differences[i];
        }
        let mut next = vec![B::ZERO; basis.len() + 1];
        for (j, &c) in basis.iter().enumerate() {
            next[j] = next[j] - c * nodes[i];
            next[j + 1] = next[j + 1] + c;
        }
        basis = next;
    }
    out
}

fn kappa_checks<B: Prime + Extension<B>, E: Extension<B>>() {
    let k = kappa_skip_rounds::<B, E>().unwrap();
    let b = E::DEGREE;
    assert_eq!(1usize << k, b);
    let domain = SkipDomain::<B>::new(k).unwrap();
    assert_eq!(domain.nodes().len(), b);
    assert_eq!(
        domain.label(),
        if k <= B::TWO_ADICITY {
            "radix2_coset"
        } else {
            "general_lagrange"
        }
    );
    let samples: Vec<_> = match &domain {
        SkipDomain::Radix2(d) => d.nodes.iter().map(|&h| h * d.shift).collect(),
        SkipDomain::Lagrange(_) => (b..2 * b).map(|i| B::from_u64(i as u64)).collect(),
    };
    let mut rng = Rng(888);
    let coefficients: Vec<B> = (0..b).map(|_| B::random(&mut rng)).collect();
    let values: Vec<_> = domain
        .nodes()
        .iter()
        .map(|&h| eval(&coefficients, h))
        .collect();
    let mut got = values.clone();
    domain.evaluate(&mut got, &mut vec![B::ZERO; b]);
    for (&x, &v) in samples.iter().zip(&got) {
        assert_eq!(v, eval(&coefficients, x));
    }
    let ext_coefficients: Vec<_> = (0..b).map(|_| E::random(&mut rng)).collect();
    let mut ext_values: Vec<_> = samples
        .iter()
        .map(|&s| eval(&ext_coefficients, E::lift(s)))
        .collect();
    domain.interpolate(&mut ext_values);
    assert_eq!(ext_values, ext_coefficients);
    for x in [
        E::random(&mut rng),
        E::lift(domain.nodes()[0]),
        E::lift(domain.nodes()[b - 1]),
    ] {
        let lagrange = domain.lagrange(x);
        let evaluation = lagrange
            .iter()
            .zip(&values)
            .fold(E::ZERO, |acc, (&w, &v)| acc + w.scale(v));
        assert_eq!(
            evaluation,
            eval(
                &coefficients
                    .iter()
                    .copied()
                    .map(E::lift)
                    .collect::<Vec<_>>(),
                x
            )
        );
        assert_eq!(
            skip_vanishing::<B, E>(k, x),
            domain
                .nodes()
                .iter()
                .fold(E::ONE, |acc, &h| acc * (x - E::lift(h)))
        );
    }

    let n = k + 2;
    let trace = Trace::<B>::random(n, 42);
    let tape = Challenges::<E>::deterministic(n, 77);
    let weights = eq_table(&tape.z[k..]);
    let actual = skip_quotient_on_domain(&trace, &domain, &weights);
    let mut numerator = vec![E::ZERO; 2 * b - 1];
    for (chunk, &w) in trace.rows.chunks_exact(b).zip(&weights) {
        let cols: Vec<_> = (0..3)
            .map(|j| {
                newton_interpolate(
                    domain.nodes(),
                    &chunk.iter().map(|row| row[j]).collect::<Vec<_>>(),
                )
            })
            .collect();
        for i in 0..b {
            numerator[i] = numerator[i] - w.scale(cols[2][i]);
            for j in 0..b {
                numerator[i + j] = numerator[i + j] + w.scale(cols[0][i] * cols[1][j]);
            }
        }
    }
    let mut psi = vec![B::ONE];
    for &h in domain.nodes() {
        let mut next = vec![B::ZERO; psi.len() + 1];
        for (i, &c) in psi.iter().enumerate() {
            next[i] = next[i] - h * c;
            next[i + 1] = next[i + 1] + c;
        }
        psi = next;
    }
    let mut expected = vec![E::ZERO; b - 1];
    for i in (b..numerator.len()).rev() {
        let q = numerator[i];
        expected[i - b] = q;
        for j in 0..=b {
            numerator[i - b + j] = numerator[i - b + j] - q.scale(psi[j]);
        }
    }
    assert!(numerator.iter().all(|&x| x == E::ZERO));
    assert_eq!(actual, expected);
    let (proof, _) = prove(&trace, &tape, Method::Skip(k)).unwrap();
    assert_eq!(proof.rounds.len(), n - k);
    assert_eq!(proof.quotient.len(), b - 1);
    let trusted = oracle_evaluations(&trace, &tape, k).unwrap();
    verify::<B, E>(&proof, &tape, n, k, trusted).unwrap();
    assert!(verify::<B, E>(&proof, &tape, n, k - 1, trusted).is_err());
}

#[test]
fn fixed_kappa_parameters() {
    assert_eq!(kappa_skip_rounds::<F251, E251>().unwrap(), 4);
    assert_eq!(kappa_skip_rounds::<F65521, E65521>().unwrap(), 3);
    assert_eq!(kappa_skip_rounds::<Mersenne31, EMersenne31>().unwrap(), 2);
    assert_eq!(kappa_skip_rounds::<Goldilocks, EGoldilocks>().unwrap(), 1);
    assert_eq!(kappa_skip_rounds::<KoalaBear, EKoalaBear>().unwrap(), 2);
    assert!(SkipDomain::<F251>::new(7).is_err());
    assert!(SkipDomain::<Mersenne31>::new(9).is_err());
}

macro_rules! suite {
    ($module:ident, $b:ty, $e:ty) => {
        mod $module {
            use super::*;
            #[test]
            fn fields() {
                field_checks::<$b, $e>();
            }
            #[test]
            fn transforms() {
                transform_checks::<$b, $e>();
            }
            #[test]
            fn quotient() {
                quotient_checks::<$b, $e>();
            }
            #[test]
            fn protocol() {
                protocol_checks::<$b, $e>();
            }
            #[test]
            fn kappa() {
                kappa_checks::<$b, $e>();
            }
        }
    };
}
suite!(f251, F251, E251);
suite!(f65521, F65521, E65521);
suite!(m31, Mersenne31, EMersenne31);
suite!(goldilocks, Goldilocks, EGoldilocks);
suite!(koalabear, KoalaBear, EKoalaBear);
