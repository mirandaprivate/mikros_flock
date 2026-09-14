pub mod batch;
pub mod embedding;
pub mod fields;
pub mod linear_form;
mod multilinear;
pub mod ntt;
pub mod sumcheck;

use ark_ff::{AdditiveGroup, Field};
use ark_std::rand::{distributions::Standard, prelude::Distribution, Rng};
pub use multilinear::{eq_weights, eval_eq, mixed_multilinear_extend, multilinear_extend};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use self::embedding::Embedding;
#[cfg(feature = "parallel")]
use crate::utils::workload_size;
use crate::utils::zip_strict;

pub fn random_vector<F: Field>(mut rng: impl Rng, length: usize) -> Vec<F>
where
    Standard: Distribution<F>,
{
    (0..length).map(|_| rng.gen()).collect::<Vec<F>>()
}

pub fn geometric_sequence<F: Field>(current: F, base: F, length: usize) -> Vec<F> {
    let mut result = Vec::with_capacity(length);
    let mut current = current;
    for _ in 0..length {
        result.push(current);
        current *= base;
    }
    result
}

pub fn dot<F: Field>(a: &[F], b: &[F]) -> F {
    assert_eq!(a.len(), b.len());
    let kernel = batch::get::<F>();
    dot_with_kernel(a, b, kernel.as_deref())
}

/// Share one resolved kernel across a whole operation, including Rayon leaves.
pub(crate) fn dot_with_kernel<F: Field>(
    a: &[F],
    b: &[F],
    kernel: Option<&dyn batch::BatchArithmetic<F>>,
) -> F {
    assert_eq!(a.len(), b.len());
    #[cfg(feature = "parallel")]
    if a.len() > workload_size::<F>() {
        let chunk = workload_size::<F>().max(1);
        return a
            .par_chunks(chunk)
            .zip(b.par_chunks(chunk))
            .map(|(a, b)| dot_with_kernel(a, b, kernel))
            .sum();
    }
    if let Some(kernel) = kernel {
        return kernel.dot(a, b);
    }
    a.iter().zip(b).map(|(&a, &b)| a * b).sum()
}

pub fn tensor_product<F: Field>(a: &[F], b: &[F]) -> Vec<F> {
    let mut result = Vec::with_capacity(a.len() * b.len());
    for &x in a {
        for &y in b {
            result.push(x * y);
        }
    }
    result
}

/// Lift a vector to an embedding.
pub fn lift<M: Embedding>(embedding: &M, source: &[M::Source]) -> Vec<M::Target> {
    #[cfg(not(feature = "parallel"))]
    let result = source.iter().map(|c| embedding.map(*c)).collect();

    #[cfg(feature = "parallel")]
    let result = source.par_iter().map(|c| embedding.map(*c)).collect();

    result
}

pub fn scalar_mul<F: Field>(vector: &mut [F], weight: F) {
    let kernel = batch::get::<F>();
    scale_with_kernel(vector, weight, kernel.as_deref());
}

pub(crate) fn scale_with_kernel<F: Field>(
    vector: &mut [F],
    weight: F,
    kernel: Option<&dyn batch::BatchArithmetic<F>>,
) {
    if let Some(kernel) = kernel {
        kernel.scale(vector, weight);
        return;
    }
    for value in vector.iter_mut() {
        *value *= weight;
    }
}

/// Scalar mul add into new vector.
///
/// Returns r such that r[i] = a[i] + c · b[i].
pub fn scalar_mul_add_new<F: Field>(a: &[F], c: F, b: &[F]) -> Vec<F> {
    zip_strict(a.iter(), b.iter())
        .map(|(a, b)| *a + c * *b)
        .collect::<Vec<F>>()
}

pub fn scalar_mul_add<F: Field>(accumulator: &mut [F], weight: F, vector: &[F]) {
    assert_eq!(accumulator.len(), vector.len());
    let kernel = batch::get::<F>();
    if let Some(kernel) = kernel {
        kernel.axpy(accumulator, weight, vector);
    } else {
        for (accumulator, &value) in accumulator.iter_mut().zip(vector) {
            *accumulator += weight * value;
        }
    }
}

/// Mixed scalar-mul add
///
/// `accumulator[i] += weight * vector[i]`
pub fn mixed_scalar_mul_add<M: Embedding>(
    embedding: &M,
    accumulator: &mut [M::Target],
    weight: M::Target,
    vector: &[M::Source],
) {
    assert_eq!(accumulator.len(), vector.len());
    embedding.mixed_axpy_batch(accumulator, weight, vector);
}

/// Default for an arbitrary embedding, including same-field automorphisms.
pub(crate) fn mixed_scalar_mul_add_generic<M: Embedding>(
    embedding: &M,
    accumulator: &mut [M::Target],
    weight: M::Target,
    vector: &[M::Source],
) {
    assert_eq!(accumulator.len(), vector.len());
    for (accumulator, value) in accumulator.iter_mut().zip(vector) {
        *accumulator += embedding.mixed_mul(weight, *value);
    }
}

pub fn univariate_evaluate<F: Field>(coefficients: &[F], point: F) -> F {
    mixed_univariate_evaluate(&embedding::Identity::new(), coefficients, point)
}

/// Mixed field univariate Horner evaluation.
pub fn mixed_univariate_evaluate<M: Embedding>(
    embedding: &M,
    coefficients: &[M::Source],
    point: M::Target,
) -> M::Target {
    #[cfg(feature = "parallel")]
    if coefficients.len() > workload_size::<M::Source>() {
        let half = coefficients.len() / 2;
        let (low, high) = coefficients.split_at(half);
        let (low, high) = rayon::join(
            || mixed_univariate_evaluate(embedding, low, point),
            || mixed_univariate_evaluate(embedding, high, point),
        );
        return low + high * point.pow([half as u64]);
    }

    let Some(mut acc) = coefficients.last().map(|c| embedding.map(*c)) else {
        return M::Target::ZERO;
    };
    for &c in coefficients.iter().rev().skip(1) {
        acc *= point;
        acc = embedding.mixed_add(acc, c);
    }
    acc
}

pub fn mixed_dot<F: Field, G: Field>(
    embedding: &impl Embedding<Source = F, Target = G>,
    a: &[G],
    b: &[F],
) -> G {
    assert_eq!(a.len(), b.len());
    embedding.mixed_dot_batch(a, b)
}

/// Original mixed dot, without inferring an embedding from field type equality.
pub(crate) fn mixed_dot_generic<M: Embedding>(
    embedding: &M,
    a: &[M::Target],
    b: &[M::Source],
) -> M::Target {
    assert_eq!(a.len(), b.len());

    #[cfg(feature = "parallel")]
    if a.len() > workload_size::<M::Target>() {
        return a
            .par_iter()
            .zip(b)
            .map(|(a, b)| embedding.mixed_mul(*a, *b))
            .sum();
    }

    a.iter()
        .zip(b)
        .map(|(a, b)| embedding.mixed_mul(*a, *b))
        .sum()
}

/// Compute `accumulator[i] += sum_j scalars[j] * points[j]^i`.
///
/// In-domain NTT points can lie in the prime subfield even when the committed
/// code, covector, and transcript challenges live in the full extension. Check
/// this once, outside the coefficient loops, and specialize only multiplication
/// by those points. Genuine extension points retain the generic arithmetic.
pub fn geometric_accumulate<F: Field>(accumulator: &mut [F], scalars: Vec<F>, points: &[F]) {
    #[cfg(test)]
    if FORCE_GENERIC_GEOMETRIC.with(std::cell::Cell::get) {
        return geometric_accumulate_generic(accumulator, scalars, points);
    }

    // Resolve once before recursion; a declining hook must leave the existing
    // accumulator untouched. Preserve the old zip behavior for unequal slices.
    if scalars.len() == points.len() {
        if let Some(kernel) = batch::get::<F>() {
            if kernel.geometric_accumulate(accumulator, &scalars, points) {
                return;
            }
        }
    }

    let base_points: Option<Vec<F::BasePrimeField>> = points
        .iter()
        .map(|point| {
            let mut coefficients = point.to_base_prime_field_elements();
            let first = coefficients.next()?;
            coefficients
                .all(|coefficient| coefficient == F::BasePrimeField::ZERO)
                .then_some(first)
        })
        .collect();
    match base_points {
        Some(base_points) => geometric_accumulate_base(accumulator, scalars, &base_points),
        None => geometric_accumulate_generic(accumulator, scalars, points),
    }
}

fn geometric_accumulate_base<F: Field>(
    accumulator: &mut [F],
    mut scalars: Vec<F>,
    points: &[F::BasePrimeField],
) {
    #[cfg(feature = "parallel")]
    if accumulator.len() > workload_size::<F>() {
        let half = accumulator.len() / 2;
        let (low, high) = accumulator.split_at_mut(half);
        let scalars_high = scalars
            .iter()
            .zip(points)
            .map(|(s, x)| s.mul_by_base_prime_field(&x.pow([half as u64])))
            .collect();
        rayon::join(
            || geometric_accumulate_base(low, scalars, points),
            || geometric_accumulate_base(high, scalars_high, points),
        );
        return;
    }

    for entry in accumulator {
        for (scalar, point) in scalars.iter_mut().zip(points) {
            *entry += *scalar;
            *scalar = scalar.mul_by_base_prime_field(point);
        }
    }
}

// Preserve the original arithmetic for extension-valued points and differential
// tests. Its loop order and parallel recursion are unchanged.
fn geometric_accumulate_generic<F: Field>(
    accumulator: &mut [F],
    mut scalars: Vec<F>,
    points: &[F],
) {
    #[cfg(feature = "parallel")]
    if accumulator.len() > workload_size::<F>() {
        let half = accumulator.len() / 2;
        let (low, high) = accumulator.split_at_mut(half);
        let scalars_high = scalars
            .iter()
            .zip(points)
            .map(|(s, x)| *s * x.pow([half as u64]))
            .collect();
        rayon::join(
            || geometric_accumulate_generic(low, scalars, points),
            || geometric_accumulate_generic(high, scalars_high, points),
        );
        return;
    }

    for entry in accumulator {
        for (scalar, point) in scalars.iter_mut().zip(points) {
            *entry += *scalar;
            *scalar *= *point; // TODO: Skip on last
        }
    }
}

#[cfg(test)]
thread_local! {
    // Only top-level dispatch reads this flag. Recursive Rayon work already
    // follows the selected implementation and needs no thread-local state.
    static FORCE_GENERIC_GEOMETRIC: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
mod geometric_tests {
    use super::*;
    use crate::{
        algebra::{
            embedding::Identity,
            fields::{Field64, Field64_2},
            linear_form::{Evaluate, LinearForm, MultilinearExtension},
        },
        buffer::Buffer,
        hash,
        parameters::ProtocolParameters,
        protocols::{params::DecodingRegime, whir::Config},
        transcript::{codecs::Empty, DomainSeparator, Proof, ProverState, VerifierState},
    };

    type E = Field64_2;

    fn element(index: usize) -> E {
        E::new(
            Field64::from((index as u64) * 13 + 5),
            Field64::from((index as u64) * 7 + 1),
        )
    }

    fn compare_with_generic(points: &[E], length: usize, zeros: bool) {
        let scalars = (0..points.len())
            .map(|i| if zeros { E::ZERO } else { element(i + 3) })
            .collect::<Vec<_>>();
        let mut actual = (0..length).map(element).collect::<Vec<_>>();
        let mut expected = actual.clone();
        geometric_accumulate_generic(&mut expected, scalars.clone(), points);
        geometric_accumulate(&mut actual, scalars, points);
        assert_eq!(
            actual,
            expected,
            "length={length}, points={}, zero={zeros}",
            points.len()
        );
    }

    #[test]
    fn base_point_geometric_matches_generic_including_parallel_tails() {
        let generator = ntt::generator::<E>(1 << 15);
        assert_eq!(generator.c1, Field64::ZERO);
        let points = [E::ZERO, E::ONE, generator, generator.pow([27]), -E::ONE];
        for length in [0, 1, 31, 2048, 8193] {
            compare_with_generic(&points, length, false);
            compare_with_generic(&points, length, true);
            compare_with_generic(&points[..1], length, false);
            compare_with_generic(&[], length, false);
        }
        let many_points = (0..232)
            .map(|i| generator.pow([i * 97]))
            .collect::<Vec<_>>();
        compare_with_generic(&many_points, 8193, false);
    }

    #[test]
    fn extension_point_geometric_retains_generic_semantics() {
        let points = [E::ZERO, E::ONE, element(7), element(11)];
        assert_ne!(points[2].c1, Field64::ZERO);
        for length in [0, 1, 31, 2048, 8193] {
            compare_with_generic(&points, length, false);
            compare_with_generic(&points[2..3], length, false);
            compare_with_generic(&points, length, true);
        }
    }

    fn complete_whir_proof() -> Proof {
        let config = Config::<Identity<E>>::new(
            1 << 10,
            &ProtocolParameters {
                decoding_regime: DecodingRegime::Unique,
                starting_log_inv_rate: 1,
                initial_folding_factor: 2,
                folding_factor: 2,
                security_level: 32,
                pow_bits: 0,
                batch_size: 1,
                hash_id: hash::BLAKE3,
            },
        );
        let table = (0..1 << 10).map(element).collect::<Vec<_>>();
        let linear_forms = [
            MultilinearExtension::new((0..10).map(|i| element(i + 23)).collect()),
            MultilinearExtension::new((0..10).map(|i| E::from((i % 2) as u64)).collect()),
        ];
        let values = linear_forms
            .iter()
            .map(|l| l.evaluate(&Identity::new(), &table))
            .collect::<Vec<_>>();
        let ds = DomainSeparator::protocol(&config)
            .session(&"geometric-numeric-regression")
            .instance(&Empty);
        let mut prover = ProverState::new_std(&ds);
        let table = Buffer::from(table);
        let witness = config.commit(&mut prover, &[&table]);
        let forms = linear_forms
            .iter()
            .cloned()
            .map(|l| Box::new(l) as Box<dyn LinearForm<E>>)
            .collect();
        let _pending = config.prove(
            &mut prover,
            &[&table],
            vec![&witness],
            forms,
            Buffer::from(values.clone()),
        );
        let proof = prover.proof();
        let mut verifier = VerifierState::new_std(&ds, &proof);
        let commitment = config.receive_commitment(&mut verifier).unwrap();
        let final_claim = config
            .verify(&mut verifier, &[&commitment], &values)
            .unwrap();
        final_claim
            .verify(linear_forms.iter().map(|l| l as &dyn LinearForm<E>))
            .unwrap();
        verifier.check_eof().unwrap();
        proof
    }

    #[test]
    fn full_whir_proof_bytes_match_generic_geometric_reference() {
        struct Reset(bool);
        impl Drop for Reset {
            fn drop(&mut self) {
                FORCE_GENERIC_GEOMETRIC.with(|flag| flag.set(self.0));
            }
        }
        let reference = {
            let _reset = Reset(FORCE_GENERIC_GEOMETRIC.with(|flag| flag.replace(true)));
            complete_whir_proof()
        };
        let optimized = complete_whir_proof();
        assert_eq!(optimized.narg_string, reference.narg_string);
        assert_eq!(optimized.hints, reference.hints);
    }
}
