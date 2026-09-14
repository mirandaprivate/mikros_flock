//! Preserve the original cubic messages while factoring equality internally.
use crate::{
    product_tree::Tree,
    profile::{Timings, elapsed},
    sumcheck::{LayerProof, LeafClaim, TreeProof},
};
use flock_core::zerocheck::univariate_skip::SplitEqGhash;
use flock_field::{F128, F256Unreduced};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use std::{mem::swap, time::Instant};

#[derive(Clone, Copy)]
enum Input<'a> {
    Interleaved(&'a [F128]),
    Separate(&'a [F128], &'a [F128]),
}
impl Input<'_> {
    fn len(self) -> usize {
        match self {
            Self::Interleaved(x) => x.len() / 2,
            Self::Separate(a, _) => a.len(),
        }
    }
    #[inline]
    fn at(self, i: usize) -> (F128, F128) {
        match self {
            Self::Interleaved(x) => (x[2 * i], x[2 * i + 1]),
            Self::Separate(a, b) => (a[i], b[i]),
        }
    }
}
fn eq(point: &[F128]) -> SplitEqGhash {
    SplitEqGhash::with_n_hi(point, point.len() / 2)
}
fn add(a: [F128; 3], b: [F128; 3]) -> [F128; 3] {
    std::array::from_fn(|i| a[i] + b[i])
}

fn block_message(input: Input<'_>, start: usize, weights: &[F128]) -> [F128; 3] {
    let mut sums = [F256Unreduced::ZERO; 3];
    #[allow(unused_mut)] // advanced by the cfg-gated four-lane kernel
    let mut cursor = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    ))]
    {
        use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu, f128x4_set, ghash_mul_x4};
        use std::arch::x86_64::_mm512_xor_si512;
        // SAFETY: all intrinsics are compile-time feature gated. The loop
        // reads four in-range pairs and four weights; no lanes are padded.
        unsafe {
            let mut accum = [WideGhashX4::zero(); 3];
            while cursor + 4 <= weights.len() {
                let lo: [(F128, F128); 4] =
                    std::array::from_fn(|j| input.at(2 * (start + cursor + j)));
                let hi: [(F128, F128); 4] =
                    std::array::from_fn(|j| input.at(2 * (start + cursor + j) + 1));
                let a0 = f128x4_set(lo[0].0, lo[1].0, lo[2].0, lo[3].0);
                let b0 = f128x4_set(lo[0].1, lo[1].1, lo[2].1, lo[3].1);
                let a1 = f128x4_set(hi[0].0, hi[1].0, hi[2].0, hi[3].0);
                let b1 = f128x4_set(hi[0].1, hi[1].1, hi[2].1, hi[3].1);
                let w = f128x4_loadu(weights.as_ptr().add(cursor));
                accum[0].mul_acc(ghash_mul_x4(a0, b0), w);
                accum[1].mul_acc(ghash_mul_x4(a1, b1), w);
                accum[2].mul_acc(
                    ghash_mul_x4(_mm512_xor_si512(a0, a1), _mm512_xor_si512(b0, b1)),
                    w,
                );
                cursor += 4;
            }
            for k in 0..3 {
                sums[k] ^= accum[k].fold();
            }
        }
    }
    for (j, &weight) in weights.iter().enumerate().skip(cursor) {
        let (a0, b0) = input.at(2 * (start + j));
        let (a1, b1) = input.at(2 * (start + j) + 1);
        let values = [a0 * b0, a1 * b1, (a0 + a1) * (b0 + b1)];
        for k in 0..3 {
            sums[k] ^= values[k].mul_unreduced(weight);
        }
    }
    sums.map(F256Unreduced::reduce)
}
fn message(input: Input<'_>, weights: &SplitEqGhash) -> [F128; 3] {
    assert_eq!(input.len() / 2, weights.lo.len() * weights.hi.len());
    let compute = |hi: usize| {
        block_message(input, hi * weights.lo.len(), &weights.lo).map(|x| x * weights.hi[hi])
    };
    if input.len() < 1 << 15 {
        (0..weights.hi.len())
            .map(compute)
            .fold([F128::ZERO; 3], add)
    } else {
        (0..weights.hi.len())
            .into_par_iter()
            .map(compute)
            .reduce(|| [F128::ZERO; 3], add)
    }
}

fn fold_block(input: Input<'_>, base: usize, a: &mut [F128], b: &mut [F128], r: F128) {
    match input {
        Input::Separate(x, y) => {
            flock_field::f128_slice::fold_pairs(x, base, a, r);
            flock_field::f128_slice::fold_pairs(y, base, b, r);
        }
        Input::Interleaved(src) => crate::kernels::fold_interleaved(src, base, a, b, r),
    }
}
fn fold_message(
    input: Input<'_>,
    a: &mut [F128],
    b: &mut [F128],
    r: F128,
    next: Option<&SplitEqGhash>,
) -> Option<[F128; 3]> {
    assert_eq!(input.len() / 2, a.len());
    assert_eq!(a.len(), b.len());
    if let Some(weights) = next {
        let chunk = 2 * weights.lo.len();
        assert_eq!(a.len(), chunk * weights.hi.len());
        let compute = |(hi, (a, b)): (usize, (&mut [F128], &mut [F128]))| {
            fold_block(input, hi * chunk, a, b, r);
            block_message(Input::Separate(a, b), 0, &weights.lo).map(|x| x * weights.hi[hi])
        };
        let result = if input.len() < 1 << 15 {
            a.chunks_mut(chunk)
                .zip(b.chunks_mut(chunk))
                .enumerate()
                .map(compute)
                .fold([F128::ZERO; 3], add)
        } else {
            a.par_chunks_mut(chunk)
                .zip(b.par_chunks_mut(chunk))
                .enumerate()
                .map(compute)
                .reduce(|| [F128::ZERO; 3], add)
        };
        Some(result)
    } else {
        fold_block(input, 0, a, b, r);
        None
    }
}

fn cubic(core: [F128; 3], parent: F128, prefix: F128) -> [F128; 3] {
    let [g0, g1, ginf] = core;
    [
        prefix * (F128::ONE + parent) * g0,
        prefix * (parent * ginf + g0 + g1),
        prefix * ginf,
    ]
}
fn evaluate([g0, g2, g3]: [F128; 3], claim: F128, t: F128) -> F128 {
    g0 + t * (claim + g2 + g3 + t * (g2 + t * g3))
}

pub(crate) fn prove<C: Challenger>(
    tree: &impl Tree,
    c: &mut C,
    p: &mut Timings,
) -> (TreeProof, LeafClaim) {
    let mut point = Vec::new();
    let mut claim = tree.root();
    let mut layers = Vec::new();
    let capacity = if tree.log_len() > 1 {
        1 << (tree.log_len() - 2)
    } else {
        0
    };
    // Reuse Flock's scratch pool. Only the prefixes written by fold_message
    // are ever read; untouched capacity is never inspected.
    let mut cur = std::array::from_fn::<_, 2, _>(|_| flock_core::scratch::take_f128(capacity));
    let mut next = std::array::from_fn::<_, 2, _>(|_| flock_core::scratch::take_f128(capacity));
    for level in (0..tree.log_len()).rev() {
        let t = Instant::now();
        let children = tree.layer(level);
        p.layer_materialize_ms += elapsed(t);
        let k = point.len();
        let h = 1usize << k;
        let mut rounds = Vec::with_capacity(k);
        let mut q = Vec::with_capacity(k);
        let mut prefix = F128::ONE;
        let mut pending = if k > 0 {
            let t = Instant::now();
            let w = eq(&point[1..]);
            p.equality_ms += elapsed(t);
            let t = Instant::now();
            let v = message(Input::Interleaved(&children), &w);
            p.messages_ms += elapsed(t);
            Some(v)
        } else {
            None
        };
        let mut len = 0;
        for i in 0..k {
            let g = cubic(pending.take().unwrap(), point[i], prefix);
            c.observe_f128_slice(&g);
            let r = c.sample_f128();
            claim = evaluate(g, claim, r);
            rounds.push(g);
            q.push(r);
            prefix *= F128::ONE + point[i] + r;
            let t = Instant::now();
            let w = (i + 1 < k).then(|| eq(&point[i + 2..]));
            p.equality_ms += elapsed(t);
            let t = Instant::now();
            if i == 0 {
                len = h / 2;
                let [a, b] = &mut cur;
                pending = fold_message(
                    Input::Interleaved(&children),
                    &mut a[..len],
                    &mut b[..len],
                    r,
                    w.as_ref(),
                );
            } else {
                let input = Input::Separate(&cur[0][..len], &cur[1][..len]);
                len /= 2;
                let [a, b] = &mut next;
                pending = fold_message(input, &mut a[..len], &mut b[..len], r, w.as_ref());
                swap(&mut cur, &mut next);
            }
            p.fold_and_message_ms += elapsed(t);
        }
        let (a, b) = if k == 0 {
            (children[0], children[1])
        } else {
            (cur[0][0], cur[1][0])
        };
        debug_assert_eq!(claim, prefix * a * b);
        c.observe_f128_slice(&[a, b]);
        let rho = c.sample_f128();
        claim = a + rho * (a + b);
        point = vec![rho];
        point.extend(q);
        layers.push(LayerProof {
            rounds,
            left: a,
            right: b,
        });
    }
    for buffer in cur.into_iter().chain(next) {
        flock_core::scratch::give_f128(buffer);
    }
    (
        TreeProof { layers },
        LeafClaim {
            point,
            value: claim,
        },
    )
}
