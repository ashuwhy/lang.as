n, x = 10_000_000, 7
a = [0] * n
for i in range(n):
    x = (x * 1103 + 12345) % 1_000_003
    a[i] = x
total = 0
for _ in range(20):
    total += sum(a)
print(total)
