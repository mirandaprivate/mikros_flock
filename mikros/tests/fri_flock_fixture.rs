//! Bridge the two encodings of the same connected binary AND/XOR circuit.
//!
//! PLONK stores gate-local L/R/O values; FLOCK stores one constant, the private
//! inputs and gate outputs. This checks logical equivalence without pretending
//! that their witness vectors, transcript domains, or PCS commitments coincide.
#![allow(dead_code)]
#![recursion_limit = "256"]

#[path = "../examples/e2e_plonk_bitwidth/fixture.rs"]
mod bitwidth_fixture;
#[path = "../examples/e2e_plonk/fixture.rs"]
mod fixture;
#[path = "../examples/e2e_plonk/flock_r1cs.rs"]
mod flock_r1cs;
#[path = "../examples/e2e_common/grinding.rs"]
mod grinding;
#[path = "../examples/e2e_plonk/queries.rs"]
mod queries;

use bitwidth_fixture::Circuit as PlonkCircuit;
use fixture::{Circuit as NativeCircuit, Mode};
use flock_field::F128;

/// Map a PLONK wire occurrence to the corresponding compact R1CS variable.
fn native_wire_index(n: usize, wire: usize, gate: usize) -> usize {
    let gadget = gate / 2;
    match (gate & 1, wire) {
        (0, 0) => 1 + gadget,
        (0, 1) => 1 + n / 2 + gadget,
        (1, 0) => n + gate,
        (1, 1) => 1 + ((gadget + 1) & (n / 2 - 1)),
        (_, 2) => n + 1 + gate,
        _ => panic!("wire index"),
    }
}

/// Read a compact variable from the circuit recipe, without allocating tables.
fn native_value(c: &NativeCircuit, index: usize) -> bool {
    let n = c.n();
    let value = if index == 0 {
        return true;
    } else if index <= n / 2 {
        c.word(0, 2 * (index - 1))
    } else if index <= n {
        c.word(1, 2 * (index - n / 2 - 1))
    } else if index <= 2 * n {
        c.word(2, index - n - 1)
    } else {
        F128::ZERO
    };
    value.lo & 1 != 0
}

#[test]
fn bridge_n14_full_gates_inputs_outputs_and_actual_public_csc() {
    for seed in [0, 97, 101, u64::MAX] {
        let native = NativeCircuit::new(14, Mode::Single, seed);
        let plonk = PlonkCircuit::new(14, 1, seed).unwrap();
        let setup = flock_r1cs::Setup::new(&native).unwrap();
        let wires = plonk.witness();
        let [a, b] = flock_r1cs::matrix_images(&native, &setup.witness);
        assert_eq!(native.io, F128::new(plonk.io as u64, 0));
        assert_eq!(setup.witness.len(), wires.len());
        assert!(fixture::bit(&setup.witness, 0));

        for index in 0..4 * native.n() {
            assert_eq!(
                fixture::bit(&setup.witness, index),
                native_value(&native, index)
            );
            // Includes the pinned constant, private-input identity equations,
            // all genuine gates and the zero-padding equations with C = I.
            assert_eq!(
                fixture::bit(&a, index) & fixture::bit(&b, index),
                fixture::bit(&setup.witness, index),
            );
        }
        for gate in 0..native.n() {
            let row = native.n() + 1 + gate;
            let values: [bool; 3] = std::array::from_fn(|wire| {
                let variable = native_wire_index(native.n(), wire, gate);
                let actual = fixture::bit(&setup.witness, variable);
                assert_eq!(actual as u128, plonk.value(wire, gate));
                assert_eq!(
                    actual as u128,
                    plonk.packed_value(&wires, wire * native.n() + gate)
                );
                assert_eq!(actual as u64, native.word(wire, gate).lo & 1);
                actual
            });
            assert_eq!(
                values[2],
                if gate & 1 == 0 {
                    values[0] & values[1]
                } else {
                    values[0] ^ values[1]
                }
            );
            if gate & 1 == 0 {
                assert_eq!(fixture::bit(&a, row), values[0]);
                assert_eq!(fixture::bit(&b, row), values[1]);
            } else {
                assert_eq!(fixture::bit(&a, row), values[0] ^ values[1]);
                assert!(fixture::bit(&b, row));
            }
        }
        assert_eq!(
            native_wire_index(native.n(), 2, native.n() - 1),
            2 * native.n()
        );
        assert_eq!(
            fixture::bit(&setup.witness, 2 * native.n()) as u128,
            plonk.io
        );
        // Equal physical lengths do not imply identical witness contents.
        assert_ne!(setup.witness, wires);
    }
}

#[test]
fn bridge_n14_full_selectors_sigma_and_raw_word_layout() {
    for seed in [0, 97, 101] {
        let native = NativeCircuit::new(14, Mode::Single, seed);
        let plonk = PlonkCircuit::new(14, 1, seed).unwrap();
        let old_wires = native.witness();
        let new_wires = plonk.witness();
        let old_public = native.public();
        let new_public = plonk.public();
        // The old PLONK fixture and the current b=1 PLONK fixture have exactly
        // the same raw bit layouts, unlike the compact native R1CS witness.
        assert_eq!(old_wires, new_wires);
        assert_eq!(old_public, new_public);
        assert_eq!(native.public_blocks(), plonk.public_blocks());
        assert_eq!(native.copy_len(), plonk.copy_len());
        assert_eq!(native.live_len(), plonk.live_len());
        for index in 0..native.copy_len() {
            assert_eq!(native.sigma(index), plonk.sigma(index));
            assert_eq!(plonk.sigma(plonk.sigma(index)), index);
        }
        for gate in 0..native.n() {
            for selector in 0..5 {
                let value = NativeCircuit::selector(selector, gate);
                assert_eq!(value, PlonkCircuit::selector(selector, gate));
                assert_eq!(
                    fixture::bit(&new_public, selector * native.n() + gate),
                    value
                );
            }
            for wire in 0..3 {
                let label = plonk.sigma(wire * native.n() + gate);
                for bit in 0..plonk.h() {
                    assert_eq!(
                        fixture::bit(
                            &new_public,
                            (5 + wire * plonk.h() + bit) * native.n() + gate
                        ),
                        label >> bit & 1 != 0
                    );
                }
            }
        }
        assert!(new_wires[3 * plonk.wire_words()..]
            .iter()
            .all(|&x| x == F128::ZERO));
        // These are protocol-specific transcript identities, not the shared
        // logical-circuit fingerprint used by a cross-system comparison.
        assert_ne!(native.digest(), plonk.digest());
    }
}

#[test]
#[cfg(target_pointer_width = "64")]
fn bridge_n26_n30_boundary_sampling_without_allocating_witnesses() {
    for log_gates in [26, 30] {
        for seed in [0, 97, 101, u64::MAX] {
            let native = NativeCircuit::new(log_gates, Mode::Single, seed);
            let plonk = PlonkCircuit::new(log_gates, 1, seed).unwrap();
            let n = native.n();
            assert_eq!(plonk.n(), n);
            assert_eq!(native.io, F128::new(plonk.io as u64, 0));
            assert_eq!(plonk.witness_words(), n / 32);
            assert_eq!(plonk.wire_words(), native.wire_words());
            assert_eq!(plonk.public_words(), n);
            assert_eq!(plonk.public_words(), native.public_words());
            let mut gates = vec![0, 1, 63, 64, 127, 128, n / 2 - 1, n / 2, n - 2, n - 1];
            let mut state = seed ^ 0xa62e_5d79_c130_4b8f;
            for _ in 0..32 {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                gates.push((state as usize) & (n - 1));
            }
            for gate in gates {
                for wire in 0..3 {
                    let index = native_wire_index(n, wire, gate);
                    assert!(index <= 2 * n);
                    assert_eq!(
                        native_value(&native, index) as u128,
                        plonk.value(wire, gate)
                    );
                    assert_eq!(
                        (native.word(wire, gate).lo & 1) as u128,
                        plonk.value(wire, gate)
                    );
                    assert_eq!(native.sigma(wire * n + gate), plonk.sigma(wire * n + gate));
                }
                for selector in 0..5 {
                    assert_eq!(
                        NativeCircuit::selector(selector, gate),
                        PlonkCircuit::selector(selector, gate)
                    );
                }
            }
            for index in [0, n - 1, n, 2 * n, 3 * n - 1, 3 * n, 3 * n + 1, 4 * n - 1] {
                assert_eq!(native.sigma(index), plonk.sigma(index));
                assert!(plonk.sigma(index) < 4 * n);
            }
            assert_eq!(plonk.sigma(3 * n - 1), 3 * n);
            assert_eq!(native_wire_index(n, 1, n - 1), 1);
        }
    }
}

#[test]
fn near_full_bridge_independent_boolean_evaluation_and_copy_cycles() {
    use fixture::near_full_circuit as recipe;
    for seed in [0, 97, 101] {
        let native = NativeCircuit::new_near_full(14, Mode::Single, seed);
        let plonk = PlonkCircuit::new_near_full(14, 1, seed).unwrap();
        let n = plonk.n();
        let gates = n - 257;
        assert_eq!(native.r1cs_rows(), n);
        assert_eq!(native.genuine_gate_count(), gates);
        assert_eq!(plonk.genuine_gate_count(), gates);
        let setup = flock_r1cs::Setup::new(&native).unwrap();
        let wires = plonk.witness();
        let public = plonk.public();
        let [a, b] = flock_r1cs::matrix_images(&native, &setup.witness);

        // Interpret the public circuit on the committed input bits, independently
        // of either fixture's witness generator and without scalar prefix replay.
        let mut assignment = vec![false; n];
        assignment[0] = true;
        for input in 1..=256 {
            assignment[input] = fixture::bit(&setup.witness, input);
        }
        for gate in 0..gates {
            let [left, right] = recipe::gate_operands(14, gate);
            assert!((1..=256).contains(&right));
            if gate == 0 {
                assert!((1..=256).contains(&left));
                assert_ne!(left, right);
            } else {
                assert_eq!(left, 256 + gate);
            }
            let row = 257 + gate;
            assert!(left < row && right < row);
            let l = assignment[left];
            let r = assignment[right];
            let output = if gate & 1 == 0 { l & r } else { l ^ r };
            assignment[row] = output;
            for (wire, expected) in [l, r, output].into_iter().enumerate() {
                assert_eq!(
                    plonk.packed_value(&wires, wire * n + gate),
                    expected as u128
                );
            }
            assert_eq!(fixture::bit(&a, row), if gate & 1 == 0 { l } else { l ^ r });
            assert_eq!(fixture::bit(&b, row), if gate & 1 == 0 { r } else { true });
        }
        for row in 0..n {
            assert_eq!(fixture::bit(&setup.witness, row), assignment[row]);
            assert_eq!(
                fixture::bit(&a, row) & fixture::bit(&b, row),
                assignment[row]
            );
        }
        assert_eq!(native.io, F128::new(assignment[n - 1] as u64, 0));
        assert_eq!(plonk.io, assignment[n - 1] as u128);
        assert_eq!(native.witness(), wires);
        assert_eq!(native.public(), public);

        for gate in 0..n {
            for selector in 0..5 {
                let expected = match selector {
                    0 | 1 => gate < gates && gate & 1 != 0,
                    2 => gate < gates && gate & 1 == 0,
                    3 => true,
                    _ => false,
                };
                assert_eq!(fixture::bit(&public, selector * n + gate), expected);
            }
            if gate >= gates {
                for wire in 0..3 {
                    assert_eq!(plonk.packed_value(&wires, wire * n + gate), 0);
                }
            }
        }
        let variable = |index: usize| {
            if index == 3 * n {
                Some(n - 1)
            } else if index < 3 * n && index % n < gates {
                let gate = index % n;
                let operands = recipe::gate_operands(14, gate);
                Some(match index / n {
                    0 => operands[0],
                    1 => operands[1],
                    2 => 257 + gate,
                    _ => unreachable!(),
                })
            } else {
                None
            }
        };
        let mut seen = vec![false; 4 * n];
        let mut has_long_cycle = false;
        for index in 0..4 * n {
            let next = plonk.sigma(index);
            assert_eq!(native.sigma(index), next);
            assert!(next < 4 * n && !seen[next]);
            seen[next] = true;
            assert_eq!(variable(index), variable(next));
            if variable(index).is_none() {
                assert_eq!(index, next);
            }
            has_long_cycle |= plonk.sigma(next) != index;
        }
        assert!(has_long_cycle);
        assert_eq!(plonk.sigma(3 * n), 2 * n + gates - 1);
        assert_eq!(plonk.sigma(2 * n + gates - 1), 3 * n);
    }
}

#[test]
#[cfg(target_pointer_width = "64")]
fn near_full_n30_geometry_and_index_boundaries_without_allocating_witness() {
    use fixture::near_full_circuit as recipe;
    let n = 1usize << 30;
    let gates = n - 257;
    assert_eq!(recipe::INPUTS, 256);
    assert_eq!(recipe::gate_count(30), gates);
    assert_eq!(recipe::row_count(30), n);
    assert_eq!(gates + 256 + 1, n);
    for gate in [0, 1, 254, 255, 256, gates / 2, gates - 2, gates - 1] {
        let [left, right] = recipe::gate_operands(30, gate);
        assert!(left < 257 + gate && right < 257 + gate);
        assert!((1..=256).contains(&right));
        assert_eq!(recipe::output_index(gate), 257 + gate);
    }
    for index in [
        0,
        gates - 1,
        gates,
        n - 1,
        n,
        2 * n,
        2 * n + gates - 1,
        3 * n,
        4 * n - 1,
    ] {
        let next = recipe::sigma(30, index);
        assert!(next < 4 * n);
        assert!(u32::try_from(next).is_ok());
    }
    assert_eq!(recipe::sigma(30, 3 * n), 2 * n + gates - 1);
    assert_eq!(recipe::sigma(30, 4 * n - 1), 4 * n - 1);
}
