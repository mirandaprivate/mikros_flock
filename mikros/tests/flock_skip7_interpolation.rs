//! Independent polynomial checks for the 128-point Flock skip domain.
//! This file can be linked directly against the frozen benchmark's rlib.

use flock_core::{
    field::{F8, F128, phi8, test_rng::Rng},
    zerocheck::multilinear::{interpolate_at_z_combined, interpolate_at_z_on_lambda},
};

fn horner(coefficients: &[F128], point: F128) -> F128 {
    coefficients
        .iter()
        .rev()
        .fold(F128::ZERO, |value, &coefficient| {
            value * point + coefficient
        })
}

fn nodes_and_outside_points(rng: &mut Rng) -> Vec<F128> {
    let mut points: Vec<_> = (0..=u8::MAX).map(|i| phi8(F8(i))).collect();
    // The first 256 points exhaust S and Lambda. Rejection sampling ensures
    // that each additional point exercises interpolation outside GF(256).
    while points.len() < 256 + 16 {
        let point = rng.f128();
        if !points.contains(&point) {
            points.push(point);
        }
    }
    points
}

#[test]
fn skip7_lambda_interpolates_degree127_at_all_nodes_and_outside_points() {
    let mut rng = Rng::new(0x7157_0127);
    let mut coefficients = rng.f128_vec(128);
    coefficients[127] = rng.nonzero();
    let points = nodes_and_outside_points(&mut rng);
    let on_lambda: Vec<_> = points[128..256]
        .iter()
        .map(|&point| horner(&coefficients, point))
        .collect();
    assert!(
        on_lambda
            .iter()
            .any(|&value| value != F128::ZERO && value != F128::ONE)
    );

    for (index, &point) in points.iter().enumerate() {
        let expected = horner(&coefficients, point);
        let actual = interpolate_at_z_on_lambda(&on_lambda, 7, point);
        assert_eq!(
            actual, expected,
            "degree127: point index={index}, point={point:?}"
        );
    }
}

#[test]
fn skip7_combined_interpolates_degree254_vanishing_on_all_s_nodes() {
    let mut rng = Rng::new(0x7157_0254);
    let mut quotient = rng.f128_vec(127);
    quotient[126] = rng.nonzero();
    let points = nodes_and_outside_points(&mut rng);
    let s = &points[..128];
    // Directly evaluate P(X) = product_{s in S}(X - s) * Q(X).
    // Its degree is exactly 128 + 126 = 254 and it vanishes on all of S.
    // In characteristic two subtraction equals addition. Neither this
    // reference nor Horner uses the production interpolation weights.
    let polynomial = |point: F128| {
        s.iter()
            .fold(F128::ONE, |value, &node| value * (point + node))
            * horner(&quotient, point)
    };
    let on_lambda: Vec<_> = points[128..256]
        .iter()
        .map(|&point| polynomial(point))
        .collect();
    assert!(
        on_lambda
            .iter()
            .any(|&value| value != F128::ZERO && value != F128::ONE)
    );

    for (index, &point) in points.iter().enumerate() {
        let expected = polynomial(point);
        if index < 128 {
            assert_eq!(expected, F128::ZERO, "vanishing polynomial at S[{index}]");
        }
        let actual = interpolate_at_z_combined(&on_lambda, 7, point);
        assert_eq!(
            actual, expected,
            "degree254: point index={index}, point={point:?}"
        );
    }
}
