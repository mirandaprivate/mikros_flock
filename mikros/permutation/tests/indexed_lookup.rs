use flock_field::F128;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_permutation::{
    Error,
    bits::PackedBits,
    indexed_lookup::{self as lookup, Claims, Proof, PublicSchedule, Statement, Strategy},
};

fn challenger() -> FsChallenger {
    FsChallenger::new(b"indexed-public-lookup-independent-tests-v1")
}
fn fixtures(
    width: usize,
    table: &[u128],
    addresses: &[usize],
) -> (PublicSchedule, PackedBits, PackedBits, Statement) {
    let plan = PublicSchedule::new(table.len(), addresses, width).unwrap();
    let table_bits = PackedBits::from_rows(table, width).unwrap();
    let values = PackedBits::from_rows(
        &addresses.iter().map(|&a| table[a]).collect::<Vec<_>>(),
        width,
    )
    .unwrap();
    let statement = Statement::new(&plan, &values, &table_bits).unwrap();
    (plan, values, table_bits, statement)
}

/// Deliberately scalar and independent of the production split equality table.
fn direct_source_equations(
    claims: &Claims,
    plan: &PublicSchedule,
    values: &PackedBits,
    table: &PackedBits,
) {
    for side in 0..2 {
        let q = &claims.sides[side];
        let mut result = F128::ZERO;
        for (is_table, bits) in [(false, values), (true, table)] {
            let offset = if is_table == (side == 0) {
                plan.segment_len()
            } else {
                0
            };
            for row in 0..bits.rows() {
                let index = offset + row;
                let column = index % q.functional.columns.len();
                let tensor_row = index / q.functional.columns.len();
                let mut weight = q.functional.columns[column];
                for (bit, &r) in q.functional.row_point.iter().enumerate() {
                    weight *= if tensor_row >> bit & 1 == 0 {
                        F128::ONE + r
                    } else {
                        r
                    };
                }
                for bit in 0..bits.width() {
                    if bits.bit(bit * bits.rows() + row) {
                        result += weight * plan.value_basis()[bit];
                    }
                }
            }
        }
        assert_eq!(result, q.value, "source side {side}");
    }
}

fn roundtrip(width: usize, table: &[u128], addresses: &[usize]) {
    let (plan, values, table, statement) = fixtures(width, table, addresses);
    assert!(plan.generator_order() > addresses.len() as u128);
    assert_eq!(plan.tuple_bits(), 2 * width * plan.d());
    assert!((128 / width).is_multiple_of(4 * plan.d()));
    for strategy in [Strategy::Packed, Strategy::FullExtension] {
        let mut p = challenger();
        let output = lookup::prove(&plan, &statement, &values, &table, strategy, &mut p).unwrap();
        let mut v = challenger();
        let (claims, timing) =
            lookup::verify_profiled(&plan, &statement, &output.proof, &mut v).unwrap();
        assert_eq!(output.claims, claims);
        assert_eq!(p.sample_f128(), v.sample_f128());
        assert!(timing.total_ms >= timing.replay_ms);
        claims.check(&values, &table, &plan).unwrap();
        direct_source_equations(&claims, &plan, &values, &table);
        let encoded = bincode::serialize(&output.proof).unwrap();
        let decoded: Proof = bincode::deserialize(&encoded).unwrap();
        assert_eq!(decoded, output.proof);
        assert!(encoded.len() >= output.proof.field_elements() * 16);
        let mut changed = claims.clone();
        changed.sides[0].value += F128::ONE;
        assert!(changed.check(&values, &table, &plan).is_err());
        let mut changed = claims.clone();
        changed.sides[1].value += F128::ONE;
        assert!(changed.check(&values, &table, &plan).is_err());
    }
}

#[test]
fn supported_widths_public_schedule_and_original_sources_roundtrip() {
    for b in [1usize, 2, 4, 8, 16] {
        let mask = (1u128 << b) - 1;
        let table = [mask, 0, mask, 1, 2 & mask];
        roundtrip(
            b,
            &table,
            &[2, 2, 0, 4, 2, 1, 4, 0, 0, 0, 4, 2, 1, 2, 2, 2, 4],
        );
    }
}

#[test]
fn singleton_empty_unread_and_nonpower_segments() {
    roundtrip(8, &[7], &[]);
    roundtrip(8, &[7], &[0]);
    roundtrip(4, &[3, 3, 8, 9, 3], &[0, 4, 0]);
    roundtrip(2, &[0, 1, 2], &[2; 17]);
}

#[test]
fn invalid_values_addresses_shapes_and_capacity_rejected() {
    assert!(PublicSchedule::new(0, &[], 8).is_err());
    assert!(PublicSchedule::new(3, &[3], 8).is_err());
    for width in [0, 3, 7, 32, 128] {
        assert!(PublicSchedule::new(2, &[0], width).is_err());
    }
    // d must reserve a whole base-field coordinate for the value, even when
    // a bit-concatenation heuristic would appear to fit in 32 bits.
    assert!(matches!(
        PublicSchedule::new((1 << 16) + 1, &[], 16),
        Err(Error::Capacity)
    ));
    let (plan, values, table, statement) = fixtures(4, &[2, 5, 7], &[0, 1, 0, 2, 0]);
    let wrong = PackedBits::from_rows(&[2, 5, 2, 7, 5], 4).unwrap();
    assert!(
        lookup::prove(
            &plan,
            &statement,
            &wrong,
            &table,
            Strategy::Packed,
            &mut challenger()
        )
        .is_err()
    );
    let wrong_statement = Statement::new(&plan, &wrong, &table).unwrap();
    for strategy in [Strategy::Packed, Strategy::FullExtension] {
        assert!(
            lookup::prove(
                &plan,
                &wrong_statement,
                &wrong,
                &table,
                strategy,
                &mut challenger()
            )
            .is_err()
        );
    }
    assert!(Statement::new(&plan, &PackedBits::from_rows(&[0], 4).unwrap(), &table).is_err());
    let output = lookup::prove(
        &plan,
        &statement,
        &values,
        &table,
        Strategy::Packed,
        &mut challenger(),
    )
    .unwrap();
    let other = PublicSchedule::new(3, &[0, 1, 0, 2, 1], 4).unwrap();
    assert!(lookup::verify(&other, &statement, &output.proof, &mut challenger()).is_err());
    assert!(output.claims.check(&wrong, &table, &plan).is_err());
}

#[test]
fn messages_and_both_terminal_obligations_are_bound() {
    let (plan, values, table, statement) = fixtures(8, &[9, 7, 9], &[0, 1, 2, 1, 1, 0, 2]);
    for strategy in [Strategy::Packed, Strategy::FullExtension] {
        let output = lookup::prove(
            &plan,
            &statement,
            &values,
            &table,
            strategy,
            &mut challenger(),
        )
        .unwrap();
        let rejects = |proof: &Proof| {
            lookup::verify(&plan, &statement, proof, &mut challenger())
                .and_then(|c| c.check(&values, &table, &plan))
                .is_err()
        };
        let mut candidate = output.proof.clone();
        match &mut candidate {
            Proof::Packed(p) => p.roots[0] += F128::ONE,
            Proof::FullExtension(p) => p.roots[0] += F128::ONE,
        }
        assert!(rejects(&candidate));
        let mut candidate = output.proof.clone();
        match &mut candidate {
            Proof::Packed(p) => p.packed_layers[0][1].columns[0] += F128::ONE,
            Proof::FullExtension(p) => p.right.layers.last_mut().unwrap().left += F128::ONE,
        }
        assert!(rejects(&candidate));
        if let Proof::Packed(p) = &output.proof {
            let mut candidate = p.clone();
            candidate.packed_layers[0][0].quotient[0] += F128::ONE;
            assert!(rejects(&Proof::Packed(candidate)));
            let mut candidate = p.clone();
            candidate.packed_layers.pop();
            assert!(rejects(&Proof::Packed(candidate)));
        }
        let mut wrong = statement.clone();
        wrong.table.digest[0] ^= 1;
        assert!(
            lookup::verify(&plan, &wrong, &output.proof, &mut challenger())
                .and_then(|c| c.check(&values, &table, &plan))
                .is_err()
        );
    }
}

#[test]
fn proofs_and_transcript_are_thread_count_independent() {
    let (plan, values, table, statement) = fixtures(8, &[3, 4, 3, 6], &[0, 1, 2, 3, 1, 1, 0]);
    for strategy in [Strategy::Packed, Strategy::FullExtension] {
        let mut reference = None;
        for threads in [1, 3] {
            let result = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| {
                    let mut ch = challenger();
                    let out = lookup::prove(&plan, &statement, &values, &table, strategy, &mut ch)
                        .unwrap();
                    (out.proof, out.claims, ch.sample_f128())
                });
            if let Some(reference) = &reference {
                assert_eq!(&result, reference);
            } else {
                reference = Some(result);
            }
        }
    }
}

#[test]
fn multiplicative_counter_order_is_strictly_greater_than_query_count() {
    let before = PublicSchedule::new(1, &[0; 2], 1).unwrap();
    let boundary = PublicSchedule::new(1, &[0; 3], 1).unwrap();
    assert_eq!(before.position_bits(), 2);
    assert_eq!(before.generator_order(), 3);
    assert_eq!(boundary.position_bits(), 4);
    assert_eq!(boundary.generator_order(), 15);
    // The same boundary in a four-bit base field: order 15 is insufficient
    // for 15 reads, even though every individual value is representable.
    let before = PublicSchedule::new(1, &[0; 14], 4).unwrap();
    let boundary = PublicSchedule::new(1, &[0; 15], 4).unwrap();
    assert_eq!(before.position_bits(), 4);
    assert_eq!(boundary.position_bits(), 8);
    roundtrip(4, &[9], &[0; 15]);
}

#[test]
fn malformed_backend_vectors_return_errors_without_panicking() {
    let (plan, values, table, statement) = fixtures(8, &[9, 7, 9], &[0, 1, 2, 1, 1, 0, 2]);
    for strategy in [Strategy::Packed, Strategy::FullExtension] {
        let output = lookup::prove(
            &plan,
            &statement,
            &values,
            &table,
            strategy,
            &mut challenger(),
        )
        .unwrap();
        let mut candidates = Vec::new();
        match &output.proof {
            Proof::Packed(proof) => {
                for side in 0..2 {
                    let mut p = proof.clone();
                    p.extension[side].layers.clear();
                    candidates.push(Proof::Packed(p));
                    let mut p = proof.clone();
                    p.extension[side].layers.last_mut().unwrap().rounds.clear();
                    candidates.push(Proof::Packed(p));
                    let mut p = proof.clone();
                    p.packed_layers[0][side].columns.clear();
                    candidates.push(Proof::Packed(p));
                    let mut p = proof.clone();
                    p.packed_layers[0][side].quotient.pop();
                    candidates.push(Proof::Packed(p));
                    let mut p = proof.clone();
                    p.packed_layers.last_mut().unwrap()[side]
                        .sumcheck
                        .rounds
                        .clear();
                    candidates.push(Proof::Packed(p));
                }
            }
            Proof::FullExtension(proof) => {
                for side in 0..2 {
                    let mut p = proof.clone();
                    if side == 0 {
                        p.left.layers.clear();
                    } else {
                        p.right.layers.clear();
                    }
                    candidates.push(Proof::FullExtension(p));
                    let mut p = proof.clone();
                    if side == 0 {
                        p.left.layers.last_mut().unwrap().rounds.clear();
                    } else {
                        p.right.layers.last_mut().unwrap().rounds.clear();
                    }
                    candidates.push(Proof::FullExtension(p));
                }
            }
        }
        for candidate in candidates {
            // A panic fails the test, independently of whether the message
            // would have been rejected by a catch-unwind wrapper.
            assert!(lookup::verify(&plan, &statement, &candidate, &mut challenger()).is_err());
        }
    }
}
