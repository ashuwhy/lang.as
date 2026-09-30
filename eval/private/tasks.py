"""Tasks for the agent evaluation: specification, signatures, reference implementation and
hidden tests. Kept out of the agents' reach until every run is scored; its SHA-256 is committed
in eval/PROTOCOL.md before the first run."""

import random

MIN, MAX = -2**63, 2**63 - 1

# Types: "int", "ints" (array of int), "opt" (optional int).
TASKS = []


def task(name, params, ret, spec):
    def wrap(fn):
        TASKS.append({"name": name, "params": params, "ret": ret, "spec": spec, "ref": fn, "tests": []})
        return fn
    return wrap


def floordiv(a, b):
    return a // b


@task("midpoint", [("lo", "int"), ("hi", "int")], "int", """\
`lo` and `hi` are 64-bit signed integers with lo <= hi; any values in the full 64-bit range are
allowed. Return the midpoint rounded down: the largest integer m such that 2*m <= lo + hi, where
lo + hi is computed exactly (as with unbounded integers). Examples: midpoint(2, 5) = 3,
midpoint(-3, -2) = -3.""")
def midpoint(lo, hi):
    return floordiv(lo + hi, 2)


@task("percent_of", [("amount", "int"), ("bps", "int")], "int", """\
`amount` satisfies 0 <= amount <= 1_000_000_000_000_000 (10^15). `bps` is a number of basis points
with 0 <= bps <= 10_000. Return floor(amount * bps / 10_000), computed exactly. Examples:
percent_of(250, 400) = 10, percent_of(3, 5_000) = 1.""")
def percent_of(amount, bps):
    return amount * bps // 10_000


@task("ring_index", [("head", "int"), ("offset", "int"), ("cap", "int")], "int", """\
A ring buffer has `cap` slots, 1 <= cap <= 1_000_000_000. `head` is a slot index with
0 <= head < cap. `offset` is any 64-bit signed integer (negative offsets move backwards). Return
the slot reached by moving `offset` steps from `head`: the value (head + offset) mod cap in the
range [0, cap), computed exactly. Examples: ring_index(2, 3, 4) = 1, ring_index(0, -1, 4) = 3.""")
def ring_index(head, offset, cap):
    return (head + offset) % cap


@task("lower_bound", [("a", "ints"), ("key", "int")], "int", """\
`a` is an array of 64-bit signed integers sorted in non-decreasing order, with length at most
1_000_000. `key` is any 64-bit signed integer. Return the smallest index i (0-based) such that
a[i] >= key, or the length of `a` if there is no such index. Examples: lower_bound([1, 3, 3, 5], 3)
= 1, lower_bound([1, 3], 9) = 2.""")
def lower_bound(a, key):
    for i, x in enumerate(a):
        if x >= key:
            return i
    return len(a)


@task("max_window_sum", [("a", "ints"), ("k", "int")], "int", """\
`a` is an array with 1 <= length <= 1_000_000 whose elements each satisfy
-1_000_000_000_000 <= a[i] <= 1_000_000_000_000 (-10^12 to 10^12). `k` satisfies
1 <= k <= length of `a`. Return the largest sum of k consecutive elements of `a`. Examples:
max_window_sum([1, -2, 3, 4], 2) = 7, max_window_sum([-5, -1], 1) = -1.""")
def max_window_sum(a, k):
    return max(sum(a[i:i + k]) for i in range(len(a) - k + 1))


@task("rotate_left", [("a", "ints"), ("k", "int")], "ints", """\
`a` is an array of 64-bit signed integers with length at most 1_000_000 (it may be empty). `k`
is any non-negative 64-bit signed integer (0 <= k <= 2^63 - 1). Return a new array: `a` rotated
left by k positions, so that with n = length of `a`, the element at index (i + k) mod n moves to
index i. If `a` is empty, return an empty array. Example: rotate_left([1, 2, 3, 4], 5) =
[2, 3, 4, 1].""")
def rotate_left(a, k):
    if not a:
        return []
    k %= len(a)
    return a[k:] + a[:k]


@task("dedupe_sorted", [("a", "ints")], "ints", """\
`a` is an array of 64-bit signed integers sorted in non-decreasing order, with length at most
1_000_000 (it may be empty). Return the distinct values of `a` in increasing order. Example:
dedupe_sorted([1, 1, 2, 5, 5, 5]) = [1, 2, 5].""")
def dedupe_sorted(a):
    out = []
    for x in a:
        if not out or out[-1] != x:
            out.append(x)
    return out


@task("merge_sorted", [("a", "ints"), ("b", "ints")], "ints", """\
`a` and `b` are arrays of 64-bit signed integers, each sorted in non-decreasing order, each with
length at most 1_000_000 (either may be empty). Return one array containing every element of `a`
and of `b` (keeping duplicates), sorted in non-decreasing order. Example:
merge_sorted([1, 4], [2, 4, 9]) = [1, 2, 4, 4, 9].""")
def merge_sorted(a, b):
    return sorted(a + b)


@task("bucket_counts", [("values", "ints"), ("lo", "int"), ("hi", "int"), ("n", "int")], "ints", """\
`lo` and `hi` satisfy -1_000_000_000_000 <= lo < hi <= 1_000_000_000_000. `n` satisfies
1 <= n <= 1_000. `values` is an array of any 64-bit signed integers with length at most
1_000_000. Split the closed range [lo, hi] into n buckets: a value v with lo <= v <= hi belongs
to bucket floor((v - lo) * n / (hi - lo)), except that v == hi belongs to bucket n - 1. Values
outside [lo, hi] are ignored. Return an array of n counts, where element j is the number of
values in bucket j. Example: bucket_counts([0, 5, 9, 10, 11], 0, 10, 2) = [1, 3].""")
def bucket_counts(values, lo, hi, n):
    out = [0] * n
    for v in values:
        if lo <= v <= hi:
            j = n - 1 if v == hi else (v - lo) * n // (hi - lo)
            out[j] += 1
    return out


@task("exact_sum", [("a", "ints")], "opt", """\
`a` is an array of any 64-bit signed integers with length at most 1_000_000 (it may be empty).
Return the exact mathematical sum of all elements if that sum fits in a 64-bit signed integer,
and no value (none) otherwise. The sum of an empty array is 0. The result depends only on the
exact total, not on the order of the elements. Examples: exact_sum([1, 2]) = 3,
exact_sum([2^63 - 1, 1]) = none.""")
def exact_sum(a):
    s = sum(a)
    return s if MIN <= s <= MAX else None


@task("gcd", [("a", "int"), ("b", "int")], "opt", """\
`a` and `b` are any 64-bit signed integers. Return the greatest common divisor of |a| and |b|,
where gcd(x, 0) = |x| and gcd(0, 0) = 0, or no value (none) if that result does not fit in a
64-bit signed integer. Examples: gcd(12, -18) = 6, gcd(0, 0) = 0.""")
def gcd(a, b):
    import math
    g = math.gcd(a, b)
    return g if g <= MAX else None


@task("isqrt", [("n", "int")], "int", """\
`n` satisfies 0 <= n <= 2^63 - 1. Return the integer square root of n: the largest integer r
such that r * r <= n. Examples: isqrt(15) = 3, isqrt(16) = 4.""")
def isqrt(n):
    import math
    return math.isqrt(n)


# Hidden tests: edge cases chosen from the specification, plus seeded random cases in the domain.
rng = random.Random(20260930)


def tests_for(name):
    T = []
    if name == "midpoint":
        T = [(0, 0), (0, 1), (2, 5), (-1, 0), (-3, -2), (-5, 4), (MIN, MAX), (MIN, MIN), (MAX, MAX),
             (MAX - 1, MAX), (MIN, MIN + 1), (MIN, 0), (0, MAX), (-7, -7), (MIN + 1, MAX)]
        for _ in range(10):
            a, b = sorted([rng.randint(MIN, MAX), rng.randint(MIN, MAX)])
            T.append((a, b))
    elif name == "percent_of":
        T = [(0, 0), (0, 10_000), (10**15, 10_000), (10**15, 9_999), (10**15 - 1, 9_999), (1, 1),
             (10_000, 1), (9_999, 1), (250, 400), (3, 5_000), (10**15, 1), (922_337_203_685_477, 10_000),
             (999_999_999_999_999, 7_777)]
        for _ in range(10):
            T.append((rng.randint(0, 10**15), rng.randint(0, 10_000)))
    elif name == "ring_index":
        T = [(2, 3, 4), (0, -1, 4), (0, 0, 1), (0, MAX, 1), (0, MIN, 1), (5, MAX, 7), (5, MIN, 7),
             (999_999_999, MAX, 1_000_000_000), (999_999_999, MIN, 1_000_000_000), (0, MIN, 1_000_000_000),
             (3, -3, 10), (3, -4, 10), (0, -10, 10), (9, 1, 10)]
        for _ in range(10):
            cap = rng.randint(1, 10**9)
            T.append((rng.randint(0, cap - 1), rng.randint(MIN, MAX), cap))
    elif name == "lower_bound":
        T = [([], 0), ([], MIN), ([1, 3, 3, 5], 3), ([1, 3], 9), ([1, 3], MIN), ([1, 3], MAX),
             ([MIN, MIN, 0, MAX, MAX], MAX), ([MIN, MIN, 0, MAX, MAX], MIN), ([5], 5), ([5], 6), ([5], 4),
             ([2, 2, 2, 2], 2), ([2, 2, 2, 2], 3)]
        for _ in range(10):
            a = sorted(rng.randint(-50, 50) for _ in range(rng.randint(0, 20)))
            T.append((a, rng.randint(-60, 60)))
    elif name == "max_window_sum":
        E = 10**12
        T = [([1, -2, 3, 4], 2), ([-5, -1], 1), ([-5, -1], 2), ([7], 1), ([E] * 8, 8), ([-E] * 8, 3),
             ([-E, E, -E, E], 2), ([3, -1, -1, 3], 4), ([0, 0, 0], 3), ([-3, -2, -9], 1)]
        for _ in range(10):
            a = [rng.randint(-E, E) for _ in range(rng.randint(1, 20))]
            T.append((a, rng.randint(1, len(a))))
    elif name == "rotate_left":
        T = [([], 0), ([], 5), ([], MAX), ([1, 2, 3, 4], 5), ([1, 2, 3, 4], 0), ([1, 2, 3, 4], 4),
             ([1, 2, 3], MAX), ([7], MAX), ([MIN, MAX], 1), ([1, 2, 3, 4, 5], 2)]
        for _ in range(10):
            T.append(([rng.randint(MIN, MAX) for _ in range(rng.randint(0, 15))], rng.randint(0, MAX)))
    elif name == "dedupe_sorted":
        T = [([],), ([1, 1, 2, 5, 5, 5],), ([MIN, MIN, MAX, MAX],), ([3],), ([4, 4, 4],), ([-2, -1, 0, 1],)]
        for _ in range(10):
            T.append((sorted(rng.randint(-10, 10) for _ in range(rng.randint(0, 20))),))
    elif name == "merge_sorted":
        T = [([], []), ([1, 4], [2, 4, 9]), ([], [3, 3]), ([MIN], [MAX]), ([MAX, MAX], [MIN, MAX]),
             ([1, 2, 3], []), ([5, 5], [5])]
        for _ in range(10):
            T.append((sorted(rng.randint(-20, 20) for _ in range(rng.randint(0, 12))),
                      sorted(rng.randint(-20, 20) for _ in range(rng.randint(0, 12)))))
    elif name == "bucket_counts":
        E = 10**12
        T = [([0, 5, 9, 10, 11], 0, 10, 2), ([], 0, 1, 1), ([MIN, MAX, 0], -E, E, 1000), ([-E, E, 0], -E, E, 1000),
             ([-E, E, 0, E - 1], -E, E, 3), ([0, 1, 2, 3], 0, 3, 3), ([5, 5, 5], 5, 6, 7), ([6], 5, 6, 7),
             ([E - 1, E], E - 1, E, 1000), ([-1], 0, 10, 2), ([10], 0, 10, 1)]
        for _ in range(10):
            lo = rng.randint(-E, E - 1)
            hi = rng.randint(lo + 1, E)
            vals = [rng.randint(lo - 10, hi + 10) for _ in range(rng.randint(0, 15))]
            T.append((vals, lo, hi, rng.randint(1, 1000)))
    elif name == "exact_sum":
        T = [([],), ([1, 2],), ([MAX, 1],), ([MAX, 1, -1],), ([MIN, -1, 1],), ([MIN, -1],), ([MAX, MAX, MIN, MIN],),
             ([MAX, MAX, MIN],), ([MIN, MIN, MAX, MAX, 1],), ([MIN],), ([MAX],), ([MAX, MIN],),
             ([MAX, MAX, MAX, MIN, MIN, MIN, -1],), ([MIN, MIN, MIN, MAX, MAX, MAX, 1, 1],)]
        for _ in range(10):
            T.append(([rng.choice([MIN, MAX, rng.randint(MIN, MAX)]) for _ in range(rng.randint(0, 8))],))
    elif name == "gcd":
        T = [(12, -18), (0, 0), (0, 5), (-5, 0), (MIN, 0), (0, MIN), (MIN, MIN), (MIN, 6), (MIN, MAX), (MAX, MAX),
             (MIN, -2), (1, MIN), (MIN, 2**62), (-(2**62), MIN), (17, 5)]
        for _ in range(10):
            T.append((rng.randint(MIN, MAX), rng.randint(MIN, MAX)))
    elif name == "isqrt":
        T = [(0,), (1,), (2,), (3,), (4,), (15,), (16,), (MAX,), (MAX - 1,), (3037000499**2,), (3037000499**2 - 1,),
             (3037000500**2 - 1 if 3037000500**2 - 1 <= MAX else MAX,), (10**18,), (10**18 - 1,), (2**62,)]
        for _ in range(10):
            T.append((rng.randint(0, MAX),))
    return T


for t in TASKS:
    t["tests"] = tests_for(t["name"])
