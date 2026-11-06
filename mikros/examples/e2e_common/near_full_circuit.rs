//! A shared public circuit recipe with 256 private input bits and N-257 gates.
//!
//! Compact variables are [1, input_0..input_255, output_0..output_(G-1)].
//! Each gate consumes the previous output and one input; the right-input order
//! is a public permutation within every 256-gate block. The recipe is independent
//! of the witness seed. Copy cycles have closed-form successors, so no N-sized
//! reverse index or per-input occurrence list is needed.

use flock_field::F128;
use rayon::prelude::*;

pub const INPUTS: usize = 256;
pub const RECIPE_DOMAIN: &[u8] = b"near-full-binary-chain-input256-perm73-v1";
const PUBLIC_DOMAIN: u64 = 0x4e46_4352_4354_3031;
const INPUT_DOMAIN: u64 = 0x4e46_494e_5054_3031;

#[inline]
fn mix(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58476D1CE4E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D049BB133111EB);
    value ^ (value >> 31)
}

#[inline]
pub fn row_count(log: usize) -> usize {
    assert!((9..=30).contains(&log), "near-full circuit dimensions");
    1usize << log
}

#[inline]
pub fn gate_count(log: usize) -> usize {
    row_count(log) - INPUTS - 1
}

#[inline]
fn offset(block: usize) -> usize {
    mix(block as u64 ^ PUBLIC_DOMAIN) as usize & (INPUTS - 1)
}

/// The compact variable index (1..=256) used on the right wire.
#[inline]
pub fn right_input(gate: usize) -> usize {
    1 + ((73 * (gate & (INPUTS - 1)) + offset(gate / INPUTS)) & (INPUTS - 1))
}

/// Inverse permutation: a compact input index 1..=256 to its slot in a block.
#[inline]
pub fn right_gate_in_block(input: usize, block: usize) -> usize {
    assert!(
        (1..=INPUTS).contains(&input),
        "near-full compact input index"
    );
    // 73 * 249 = 1 modulo 256. Mask before multiplying to avoid underflow.
    249 * ((input - 1 + INPUTS - offset(block)) & (INPUTS - 1)) & (INPUTS - 1)
}

#[inline]
pub fn first_left_input() -> usize {
    1 + (right_input(0) & (INPUTS - 1))
}

#[inline]
pub fn output_index(gate: usize) -> usize {
    INPUTS + 1 + gate
}

#[inline]
pub fn gate_operands(log: usize, gate: usize) -> [usize; 2] {
    assert!(gate < gate_count(log), "near-full gate index");
    [
        if gate == 0 {
            first_left_input()
        } else {
            output_index(gate - 1)
        },
        right_input(gate),
    ]
}

/// Map a gate-local PLONK wire to the compact R1CS assignment. Padding is zero.
#[inline]
pub fn wire_variable(log: usize, wire: usize, gate: usize) -> Option<usize> {
    assert!(wire < 3 && gate < row_count(log), "near-full wire position");
    if gate >= gate_count(log) {
        return None;
    }
    Some(if wire == 2 {
        output_index(gate)
    } else {
        gate_operands(log, gate)[wire]
    })
}

#[inline]
pub fn selector(log: usize, which: usize, gate: usize) -> bool {
    assert!(
        which < 5 && gate < row_count(log),
        "near-full selector position"
    );
    if gate >= gate_count(log) {
        return which == 3; // qO=1 enforces zero on every padding output.
    }
    match which {
        0 | 1 => gate & 1 != 0,
        2 => gate & 1 == 0,
        3 => true,
        4 => false,
        _ => unreachable!(),
    }
}

/// Successor among all occurrences of the same logical variable.
#[inline]
pub fn sigma(log: usize, index: usize) -> usize {
    let n = row_count(log);
    let gates = gate_count(log);
    assert!(index < 4 * n, "near-full copy position");
    if index >= 3 * n {
        return if index == 3 * n {
            2 * n + gates - 1
        } else {
            index
        };
    }
    let gate = index % n;
    if gate >= gates {
        return index;
    }
    match index / n {
        0 if gate == 0 => n + right_gate_in_block(first_left_input(), 0),
        0 => 2 * n + gate - 1,
        1 => {
            let input = right_input(gate);
            let next_block = gate / INPUTS + 1;
            let next = next_block * INPUTS + right_gate_in_block(input, next_block);
            if next < gates {
                n + next
            } else if input == first_left_input() {
                0 // Insert gate zero's left wire before the first right-wire use.
            } else {
                n + right_gate_in_block(input, 0)
            }
        }
        2 if gate + 1 < gates => gate + 1,
        2 => 3 * n,
        _ => unreachable!(),
    }
}

#[inline]
pub fn input_bit(seed: u64, input: usize) -> bool {
    assert!(
        (1..=INPUTS).contains(&input),
        "near-full compact input index"
    );
    mix((input as u64).wrapping_mul(0xD6E8FEB86659FD93) ^ seed ^ INPUT_DOMAIN) & 1 != 0
}

#[inline]
pub fn compact_value(words: &[F128], index: usize) -> bool {
    let word = words[index / 128];
    if index & 127 < 64 {
        word.lo >> (index & 63) & 1 != 0
    } else {
        word.hi >> (index & 63) & 1 != 0
    }
}

#[inline]
fn set_bit(words: &mut [F128], index: usize, value: bool) {
    if value {
        let word = &mut words[index / 128];
        if index & 127 < 64 {
            word.lo |= 1u64 << (index & 63);
        } else {
            word.hi |= 1u64 << (index & 63);
        }
    }
}

/// Offline O(N) witness generation, packed throughout (one bit per variable).
pub fn compact_witness(log: usize, seed: u64) -> Vec<F128> {
    let mut words = vec![F128::ZERO; row_count(log) / 128];
    set_bit(&mut words, 0, true);
    let inputs: [bool; INPUTS] = std::array::from_fn(|i| input_bit(seed, i + 1));
    for (index, &value) in inputs.iter().enumerate() {
        set_bit(&mut words, index + 1, value);
    }
    let mut previous = inputs[first_left_input() - 1];
    for gate in 0..gate_count(log) {
        let right = inputs[right_input(gate) - 1];
        previous = if gate & 1 == 0 {
            previous & right
        } else {
            previous ^ right
        };
        set_bit(&mut words, output_index(gate), previous);
    }
    words
}

/// Offline O(N) computation of the single public output, using O(256) storage.
pub fn public_output(log: usize, seed: u64) -> bool {
    reference_wire_value(log, seed, 2, gate_count(log) - 1)
}

/// Scalar reference only: a left/output query scans its prefix in O(gate).
/// Bulk witness construction must use compact_witness, never per-gate queries.
pub fn reference_wire_value(log: usize, seed: u64, wire: usize, gate: usize) -> bool {
    let Some(variable) = wire_variable(log, wire, gate) else {
        return false;
    };
    if variable <= INPUTS {
        return input_bit(seed, variable);
    }
    let target = variable - INPUTS - 1;
    let inputs: [bool; INPUTS] = std::array::from_fn(|i| input_bit(seed, i + 1));
    let mut previous = inputs[first_left_input() - 1];
    for g in 0..=target {
        let right = inputs[right_input(g) - 1];
        previous = if g & 1 == 0 {
            previous & right
        } else {
            previous ^ right
        };
    }
    previous
}

/// Map the compact assignment once to three packed PLONK columns. The fourth
/// column is zero storage; the public-output copy endpoint is supplied by io.
pub fn plonk_witness(log: usize, seed: u64) -> Vec<F128> {
    let compact = compact_witness(log, seed);
    let m = row_count(log) / 128;
    let mut words = vec![F128::ZERO; (4 * m).max(1 << 15)];
    words[..3 * m]
        .par_chunks_mut(m)
        .enumerate()
        .for_each(|(wire, block)| {
            block.par_iter_mut().enumerate().for_each(|(word, dst)| {
                for bit in 0..128 {
                    let gate = word * 128 + bit;
                    if wire_variable(log, wire, gate)
                        .is_some_and(|index| compact_value(&compact, index))
                    {
                        if bit < 64 {
                            dst.lo |= 1u64 << bit;
                        } else {
                            dst.hi |= 1u64 << (bit - 64);
                        }
                    }
                }
            });
        });
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_block_uses_each_input_once_and_inverse_is_exact() {
        assert_eq!(73 * 249 % INPUTS, 1);
        assert_ne!(first_left_input(), right_input(0));
        for block in [0, 1, 2, 63, (1usize << 22) - 2] {
            let mut seen = [false; INPUTS];
            for slot in 0..INPUTS {
                let input = right_input(block * INPUTS + slot);
                assert!(!seen[input - 1]);
                seen[input - 1] = true;
                assert_eq!(right_gate_in_block(input, block), slot);
            }
            assert!(seen.into_iter().all(|bit| bit));
        }
    }

    #[test]
    fn packed_witness_matches_independent_indexed_interpreter() {
        for log in [9, 10, 14] {
            for seed in [0, 97, u64::MAX] {
                let n = row_count(log);
                let gates = gate_count(log);
                let mut values = vec![false; n];
                values[0] = true;
                for input in 1..=INPUTS {
                    values[input] = input_bit(seed, input);
                }
                for gate in 0..gates {
                    let [left, right] = gate_operands(log, gate);
                    assert!(left < output_index(gate) && right < output_index(gate));
                    assert_ne!(left, right);
                    values[output_index(gate)] = if gate & 1 == 0 {
                        values[left] && values[right]
                    } else {
                        values[left] != values[right]
                    };
                }
                let packed = compact_witness(log, seed);
                for (index, &expected) in values.iter().enumerate() {
                    assert_eq!(compact_value(&packed, index), expected);
                }
                assert_eq!(public_output(log, seed), values[n - 1]);
                let plonk = plonk_witness(log, seed);
                for gate in 0..n {
                    let wires: [bool; 3] = std::array::from_fn(|wire| {
                        let value = compact_value(&plonk, wire * n + gate);
                        assert_eq!(
                            value,
                            wire_variable(log, wire, gate).is_some_and(|i| values[i])
                        );
                        value
                    });
                    let q = |which| selector(log, which, gate);
                    assert!(
                        !((q(0) & wires[0])
                            ^ (q(1) & wires[1])
                            ^ (q(2) & wires[0] & wires[1])
                            ^ (q(3) & wires[2])
                            ^ q(4))
                    );
                }
            }
        }
    }

    #[test]
    fn sigma_is_exactly_the_logical_occurrence_cycles() {
        for log in [9, 10, 14] {
            let n = row_count(log);
            let gates = gate_count(log);
            let mut groups = vec![Vec::new(); n];
            for wire in 0..3 {
                for gate in 0..gates {
                    groups[wire_variable(log, wire, gate).unwrap()].push(wire * n + gate);
                }
            }
            groups[n - 1].push(3 * n);
            let mut seen = vec![false; 4 * n];
            for index in 0..4 * n {
                let next = sigma(log, index);
                assert!(!seen[next]);
                seen[next] = true;
                if index < 3 * n && index % n >= gates || index > 3 * n {
                    assert_eq!(next, index);
                }
            }
            for group in groups.iter().filter(|group| !group.is_empty()) {
                let start = group[0];
                let mut position = start;
                let mut visited = Vec::new();
                loop {
                    assert!(group.contains(&position));
                    assert!(!visited.contains(&position));
                    visited.push(position);
                    position = sigma(log, position);
                    if position == start {
                        break;
                    }
                }
                assert_eq!(visited.len(), group.len());
            }
        }
    }

    #[test]
    fn billion_position_boundaries_require_no_assignment() {
        let log = 30;
        let n = row_count(log);
        let gates = gate_count(log);
        assert_eq!(gates, (1usize << 30) - 257);
        assert_eq!(output_index(gates - 1), n - 1);
        for gate in [0, 1, 254, 255, 256, gates - 2, gates - 1] {
            let [left, right] = gate_operands(log, gate);
            assert!(left < output_index(gate) && right < output_index(gate));
            for wire in 0..3 {
                assert!(sigma(log, wire * n + gate) < 4 * n);
            }
        }
        assert_eq!(sigma(log, 3 * n), 2 * n + gates - 1);
        assert_eq!(sigma(log, 2 * n + gates - 1), 3 * n);
        for index in [
            gates,
            n - 1,
            n + gates,
            2 * n - 1,
            2 * n + gates,
            3 * n - 1,
            4 * n - 1,
        ] {
            assert_eq!(sigma(log, index), index);
        }
    }
}
