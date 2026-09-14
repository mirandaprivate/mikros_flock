"""Independent Python integer/polynomial checks; no FLINT dependency."""
import json

PROFILES = [
    ("F241", 241, 16, 7),
    ("F61441", 61441, 8, 11),
    ("KoalaBear", 2130706433, 4, 3),
    ("Goldilocks", 18446744069414584321, 2, 7),
]


def prime64(n):
    if n < 2:
        return False
    for p in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % p == 0:
            return n == p
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    # Deterministic Miller-Rabin bases for unsigned 64-bit integers.
    for a in (2, 325, 9375, 28178, 450775, 9780504, 1795265022):
        if a % n == 0:
            continue
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def trim(a):
    while a and a[-1] == 0:
        a.pop()
    return a


def rem(a, b, p):
    a = trim(a[:])
    b = trim(b[:])
    inv = pow(b[-1], -1, p)
    while len(a) >= len(b):
        offset = len(a) - len(b)
        scale = a[-1] * inv % p
        for j, value in enumerate(b):
            a[offset + j] = (a[offset + j] - scale * value) % p
        trim(a)
    return a


def mulmod(a, b, modulus, p):
    c = [0] * (len(a) + len(b) - 1)
    for i, ai in enumerate(a):
        for j, bj in enumerate(b):
            c[i + j] = (c[i + j] + ai * bj) % p
    return rem(c, modulus, p)


def powmod(a, n, modulus, p):
    out = [1]
    while n:
        if n & 1:
            out = mulmod(out, a, modulus, p)
        a = mulmod(a, a, modulus, p)
        n >>= 1
    return out


def gcd(a, b, p):
    while b:
        a, b = b, rem(a, b, p)
    inv = pow(a[-1], -1, p)
    return [ai * inv % p for ai in a]


def verify():
    results = []
    for name, p, degree, nonresidue in PROFILES:
        assert prime64(p), name
        assert degree & (degree - 1) == 0
        modulus = [(-nonresidue) % p] + [0] * (degree - 1) + [1]
        x = [0, 1]
        assert powmod(x, p**degree, modulus, p) == x, name
        halfway = powmod(x, p ** (degree // 2), modulus, p)
        halfway += [0] * max(0, 2 - len(halfway))
        halfway[1] = (halfway[1] - 1) % p
        assert gcd(trim(halfway), modulus, p) == [1], name
        results.append({
            "name": name, "p": str(p), "degree": degree,
            "polynomial": f"X^{degree}-{nonresidue}",
            "prime64": True, "rabin_irreducible": True,
        })
    return {"passed": True, "method": "Python arbitrary-precision Miller-Rabin and Rabin polynomial test", "fields": results}


if __name__ == "__main__":
    print(json.dumps(verify(), indent=2))
