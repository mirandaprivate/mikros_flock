//! Bounded resident storage for legacy and packed tree-reuse candidates.
//!
//! Each slot is independent of the other slot and of Flock's scratch pool.
//! Only completed, initialized tree layers are returned. A hit reuses the
//! allocation, never the old mathematical values: producers must overwrite
//! every byte/element they will subsequently read. The pool uses exact logical
//! lengths and charges allocation capacity, including unused spare capacity.
//! Rejected returns and [`clear_all`] release memory synchronously.

use flock_field::F128;
use serde::Serialize;
use std::{
    mem::{ManuallyDrop, MaybeUninit, size_of},
    sync::Mutex,
};

const BUDGET_BYTES: u64 = 32 * 1024 * 1024 * 1024;
const MIN_BUFFER_BYTES: usize = 1024 * 1024;
const MAX_BUFFERS: usize = 32;

/// Current occupancy and cumulative allocation-reuse counters for one slot.
/// Counters are reset by [`clear_all`]; save a snapshot before clearing when
/// reporting a complete batch and its separate teardown time.
#[derive(Clone, Debug, Serialize)]
pub struct Stats {
    pub budget_bytes: u64,
    pub min_buffer_bytes: usize,
    pub max_buffers: usize,
    pub retained_bytes: u64,
    pub retained_buffers: usize,
    pub retained_u8_buffers: usize,
    pub retained_f128_buffers: usize,
    pub hits: u64,
    pub misses: u64,
    /// Logical bytes that the producer will overwrite after a successful take.
    pub reused_bytes: u64,
    pub returned_buffers: u64,
    pub rejected_buffers: u64,
}

enum Buffer {
    Bytes(Vec<u8>),
    Fields(Vec<F128>),
}

impl Buffer {
    fn capacity_bytes(&self) -> u64 {
        match self {
            Self::Bytes(v) => v.capacity() as u64,
            Self::Fields(v) => (v.capacity() * size_of::<F128>()) as u64,
        }
    }

    fn len_bytes(&self) -> usize {
        match self {
            Self::Bytes(v) => v.len(),
            Self::Fields(v) => v.len() * size_of::<F128>(),
        }
    }
}

struct Pool {
    buffers: Vec<Buffer>,
    budget_bytes: u64,
    min_buffer_bytes: usize,
    max_buffers: usize,
    retained_bytes: u64,
    hits: u64,
    misses: u64,
    reused_bytes: u64,
    returned_buffers: u64,
    rejected_buffers: u64,
}

impl Pool {
    const fn new(budget_bytes: u64, min_buffer_bytes: usize, max_buffers: usize) -> Self {
        Self {
            buffers: Vec::new(),
            budget_bytes,
            min_buffer_bytes,
            max_buffers,
            retained_bytes: 0,
            hits: 0,
            misses: 0,
            reused_bytes: 0,
            returned_buffers: 0,
            rejected_buffers: 0,
        }
    }

    fn take(&mut self, n: usize, fields: bool) -> Option<Buffer> {
        // Vec's allocation invariant ensures these products fit for any
        // existing buffer. Requests also check before attempting allocation.
        let bytes = n
            .checked_mul(if fields { size_of::<F128>() } else { 1 })
            .expect("tree buffer length overflow");
        let found = self.buffers.iter().position(|b| match b {
            Buffer::Bytes(v) => !fields && v.len() == n,
            Buffer::Fields(v) => fields && v.len() == n,
        });
        if let Some(index) = found {
            let buffer = self.buffers.swap_remove(index);
            self.retained_bytes -= buffer.capacity_bytes();
            self.hits += 1;
            self.reused_bytes += bytes as u64;
            Some(buffer)
        } else {
            self.misses += 1;
            None
        }
    }

    /// A rejected buffer stays owned by the caller and is dropped after the
    /// mutex guard is released, within the same synchronous return operation.
    fn give(&mut self, buffer: Buffer) -> Option<Buffer> {
        let bytes = buffer.capacity_bytes();
        if buffer.len_bytes() < self.min_buffer_bytes
            || bytes == 0
            || self.buffers.len() >= self.max_buffers
            || bytes > self.budget_bytes.saturating_sub(self.retained_bytes)
        {
            self.rejected_buffers += 1;
            return Some(buffer);
        }
        self.retained_bytes += bytes;
        self.returned_buffers += 1;
        self.buffers.push(buffer);
        None
    }

    fn stats(&self) -> Stats {
        let retained_u8_buffers = self
            .buffers
            .iter()
            .filter(|b| matches!(b, Buffer::Bytes(_)))
            .count();
        Stats {
            budget_bytes: self.budget_bytes,
            min_buffer_bytes: self.min_buffer_bytes,
            max_buffers: self.max_buffers,
            retained_bytes: self.retained_bytes,
            retained_buffers: self.buffers.len(),
            retained_u8_buffers,
            retained_f128_buffers: self.buffers.len() - retained_u8_buffers,
            hits: self.hits,
            misses: self.misses,
            reused_bytes: self.reused_bytes,
            returned_buffers: self.returned_buffers,
            rejected_buffers: self.rejected_buffers,
        }
    }

    fn drain_and_reset(&mut self) -> Vec<Buffer> {
        let fresh = Self::new(self.budget_bytes, self.min_buffer_bytes, self.max_buffers);
        let old = std::mem::replace(self, fresh);
        old.buffers
    }
}

static POOLS: [Mutex<Pool>; 3] = [
    Mutex::new(Pool::new(BUDGET_BYTES, MIN_BUFFER_BYTES, MAX_BUFFERS)),
    Mutex::new(Pool::new(BUDGET_BYTES, MIN_BUFFER_BYTES, MAX_BUFFERS)),
    Mutex::new(Pool::new(BUDGET_BYTES, MIN_BUFFER_BYTES, MAX_BUFFERS)),
];

/// Snapshot slot 0 (Recycled), 1 (Combined), or 2 (revised packed protocol).
/// Panics for an unknown slot.
pub fn stats(slot: usize) -> Stats {
    POOLS[slot].lock().unwrap().stats()
}

/// Synchronously release all slots and reset their counters. This does not
/// clear Flock's separate scratch pool. A benchmark wanting total teardown
/// must clear that pool too. Call with no proof in flight; checked-out buffers
/// are owned by their prover and cannot be reclaimed here.
pub fn clear_all() {
    for pool in &POOLS {
        let buffers = pool.lock().unwrap().drain_and_reset();
        // Drop outside the mutex, but before clear_all returns.
        drop(buffers);
    }
}

/// Returned bytes are initialized but may contain arbitrary previous data.
/// Tree producers overwrite the complete logical length, including padding;
/// sub-byte `|=` producers must instead request zeroed fresh storage.
pub(crate) fn take_u8(slot: usize, n: usize) -> Vec<u8> {
    let buffer = POOLS[slot].lock().unwrap().take(n, false);
    match buffer {
        Some(Buffer::Bytes(v)) => v,
        Some(Buffer::Fields(_)) => unreachable!("typed pool take"),
        None => vec![0; n],
    }
}

pub(crate) fn give_u8(slot: usize, values: Vec<u8>) {
    let rejected = POOLS[slot].lock().unwrap().give(Buffer::Bytes(values));
    drop(rejected);
}

/// Transfer an initialized allocation into typed output slots. No old field
/// values are relied on by a producer. If filling panics, dropping these
/// MaybeUninit slots is sound even when only a prefix was written.
fn uninit_slots(values: Vec<F128>) -> Vec<MaybeUninit<F128>> {
    let mut values = ManuallyDrop::new(values);
    // SAFETY: F128 and MaybeUninit<F128> have identical size and alignment.
    // Every live element is already initialized; the conversion only relaxes
    // that requirement. Length and capacity are preserved, ownership transfers
    // exactly once, and the caller must initialize every slot before converting
    // back to Vec<F128>. No u8 allocation is ever cast to a field allocation.
    unsafe { Vec::from_raw_parts(values.as_mut_ptr().cast(), values.len(), values.capacity()) }
}

pub(crate) fn take_f128(slot: usize, n: usize) -> Vec<MaybeUninit<F128>> {
    let buffer = POOLS[slot].lock().unwrap().take(n, true);
    match buffer {
        Some(Buffer::Fields(v)) => uninit_slots(v),
        Some(Buffer::Bytes(_)) => unreachable!("typed pool take"),
        None => crate::kernels::uninit_f128(n),
    }
}

/// Accept only completed initialized field vectors. Producers retain the
/// existing initialize-all-then-commit boundary before calling this function.
pub(crate) fn give_f128(slot: usize, values: Vec<F128>) {
    let rejected = POOLS[slot].lock().unwrap().give(Buffer::Fields(values));
    drop(rejected);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_length_and_type_are_required_and_capacity_is_charged() {
        let mut pool = Pool::new(128, 1, 8);
        let mut bytes = Vec::with_capacity(40);
        bytes.extend_from_slice(&[7; 8]);
        let byte_capacity = bytes.capacity() as u64;
        let pointer = bytes.as_ptr();
        assert!(pool.give(Buffer::Bytes(bytes)).is_none());
        assert_eq!(pool.stats().retained_bytes, byte_capacity);
        assert!(pool.take(7, false).is_none());
        assert!(pool.take(8, true).is_none());
        let Buffer::Bytes(bytes) = pool.take(8, false).unwrap() else {
            panic!("wrong buffer type");
        };
        assert_eq!(bytes.as_ptr(), pointer);
        assert_eq!(bytes, [7; 8]);
        let stats = pool.stats();
        assert_eq!((stats.hits, stats.misses, stats.reused_bytes), (1, 2, 8));
        assert_eq!(stats.retained_bytes, 0);
    }

    #[test]
    fn shared_byte_budget_minimum_and_count_limit_are_enforced() {
        let mut pool = Pool::new(64, 8, 2);
        assert!(pool.give(Buffer::Bytes(vec![0; 7])).is_some());
        assert!(pool.give(Buffer::Bytes(vec![0; 16])).is_none());
        assert!(pool.give(Buffer::Fields(vec![F128::ONE; 2])).is_none());
        assert!(pool.give(Buffer::Bytes(vec![0; 8])).is_some());
        let stats = pool.stats();
        assert_eq!(stats.retained_bytes, 48);
        assert_eq!(stats.retained_buffers, 2);
        assert_eq!(
            (stats.retained_u8_buffers, stats.retained_f128_buffers),
            (1, 1)
        );
        drop(pool.take(16, false));
        // Count now allows another buffer, but its capacity exceeds the
        // remaining byte budget even though the logical length would fit.
        let mut oversized = Vec::with_capacity(48);
        oversized.resize(8, 0);
        assert!(pool.give(Buffer::Bytes(oversized)).is_some());
        assert_eq!(pool.stats().retained_bytes, 32);
        assert_eq!(pool.stats().rejected_buffers, 3);
    }

    #[test]
    fn field_round_trip_preserves_allocation_and_requires_overwrite() {
        let mut pool = Pool::new(256, 1, 4);
        let original = vec![F128::ONE; 7];
        let pointer = original.as_ptr();
        let capacity = original.capacity();
        assert!(pool.give(Buffer::Fields(original)).is_none());
        let Buffer::Fields(fields) = pool.take(7, true).unwrap() else {
            panic!("wrong buffer type");
        };
        let mut slots = uninit_slots(fields);
        assert_eq!(slots.as_ptr().cast::<F128>(), pointer);
        assert_eq!(slots.capacity(), capacity);
        for slot in &mut slots {
            slot.write(F128::ZERO);
        }
        // SAFETY: every slot was initialized immediately above.
        let initialized = unsafe { crate::kernels::initialized_f128(slots) };
        assert_eq!(initialized, vec![F128::ZERO; 7]);
        assert_eq!(pool.stats().reused_bytes, 7 * 16);
        assert!(pool.give(Buffer::Fields(initialized)).is_none());
        let Buffer::Fields(fields) = pool.take(7, true).unwrap() else {
            panic!("wrong buffer type");
        };
        let mut interrupted = uninit_slots(fields);
        interrupted[0].write(F128::ONE);
        // A partially filled output can be dropped without committing it.
        drop(interrupted);
        assert_eq!(pool.stats().retained_buffers, 0);
    }

    #[test]
    fn pools_are_isolated_and_clear_resets_accounting() {
        let mut first = Pool::new(64, 1, 4);
        let mut second = Pool::new(64, 1, 4);
        assert!(first.give(Buffer::Bytes(vec![3; 16])).is_none());
        assert!(second.take(16, false).is_none());
        assert_eq!(first.stats().retained_bytes, 16);
        let drained = first.drain_and_reset();
        assert_eq!(drained.len(), 1);
        drop(drained);
        let stats = first.stats();
        assert_eq!(stats.retained_buffers, 0);
        assert_eq!(stats.retained_bytes, 0);
        assert_eq!(stats.returned_buffers, 0);
        assert_eq!(stats.budget_bytes, 64);
        assert_eq!(second.stats().misses, 1);
    }
}
