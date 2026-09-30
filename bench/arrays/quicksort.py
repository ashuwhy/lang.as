import sys

sys.setrecursionlimit(10_000)

def quicksort(v, lo, hi):
    if hi - lo < 2:
        return
    pivot, store = v[hi - 1], lo
    for i in range(lo, hi - 1):
        if v[i] < pivot:
            v[i], v[store] = v[store], v[i]
            store += 1
    v[store], v[hi - 1] = v[hi - 1], v[store]
    quicksort(v, lo, store)
    quicksort(v, store + 1, hi)

n, x = 5_000_000, 12345
a = [0] * n
for i in range(n):
    x = (x * 48271) % 1_000_000_007
    a[i] = x
quicksort(a, 0, n)
check = 0
for v in a:
    check = (check * 31 + (v % 1000 + 1000) % 1000) % 1_000_000_007
print(check)
