"""Independent finite-field checks for the proposed claim reductions.

This is an algebra experiment, not a native PCS proof test or a benchmark.
GF(2^128) uses x^128+x^7+x^2+x+1. Root counts use GF(16),
with x^4+x+1, so every pair of block challenges can be enumerated.
"""
import json
import random


def mul(a, b, bits=128, reduction=0x87):
    out, top, mask = 0, 1 << (bits - 1), (1 << bits) - 1
    while b:
        if b & 1:
            out ^= a
        b >>= 1
        high = a & top
        a = (a << 1) & mask
        if high:
            a ^= reduction
    return out


def xor(values):
    result = 0
    for x in values:
        result ^= x
    return result


def eq(point, block, multiply=mul):
    out = 1
    for i, coordinate in enumerate(point):
        out = multiply(out, coordinate if block & (1 << i) else 1 ^ coordinate)
    return out


rng = random.Random(20260915)
tests = 0
for _ in range(32):
    # Exact scalar-map classification: c_j=lambda*beta_j gives a packed-word dot.
    words = [rng.getrandbits(128) for _ in range(4)]
    point = [rng.getrandbits(128) for _ in range(2)]
    scale = rng.getrandbits(128)
    columns = [mul(scale, 1 << j) for j in range(128)]
    bit_query = xor(mul(eq(point, i), xor(columns[j] for j in range(128) if word >> j & 1))
                    for i, word in enumerate(words))
    direct = mul(scale, xor(mul(eq(point, i), word) for i, word in enumerate(words)))
    assert bit_query == direct
    tests += 1

    # Copy scalar values already include the old tree-prefix weights.
    values = [rng.getrandbits(128) for _ in range(3)] + [0]
    prefix = [rng.getrandbits(128) for _ in range(2)]
    for fresh in ([0, 0], [1, 1], [rng.getrandbits(128) for _ in range(2)],
                  [1 ^ prefix[0], 1 ^ prefix[1]]):
        weighted = [mul(eq(prefix, j), values[j]) for j in range(4)]
        target = xor(mul(eq(fresh, j), weighted[j]) for j in range(3))
        product_pairs = [[mul(1 ^ p, 1 ^ s), mul(p, s)] for p, s in zip(prefix, fresh)]
        actual = xor(mul(values[j], mul(product_pairs[0][j & 1], product_pairs[1][j >> 1]))
                     for j in range(4))
        assert target == actual
        tests += 1

# Check the degree-two random-combination bound, including a malicious padding error.
small_mul = lambda a, b: mul(a, b, 4, 3)
error_cases = [[0, 0, 0, 1], [1, 0, 0, 0], [1, 1, 1, 1]]
error_cases += [[rng.randrange(16) for _ in range(4)] for _ in range(64)]
maximum_roots = 0
for errors in error_cases:
    if not any(errors):
        continue
    roots = sum(xor(small_mul(eq((s0, s1), j, small_mul), errors[j]) for j in range(4)) == 0
                for s0 in range(16) for s1 in range(16))
    assert roots <= 2 * 16  # Schwartz-Zippel upper bound for a nonzero degree<=2 polynomial.
    maximum_roots = max(maximum_roots, roots)
    tests += 1

# Basis-packing with extension-field coefficients is not injective.
beta0, beta1 = 1, 2
assert mul(beta0, beta1) ^ mul(beta1, beta0) == 0
assert beta0 != 0 and beta1 != 0
print(json.dumps({'algebra_checks': tests, 'gf16_maximum_roots': maximum_roots,
                  'gf16_challenge_pairs': 256, 'native_pcs_proofs_tested': 0,
                  'performance_measurements': 0}, indent=2))
