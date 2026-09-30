def fill(n, seed):
    m, x = [0] * (n * n), seed
    for i in range(n * n):
        x = (x * 37 + 11) % 1000
        m[i] = x
    return m

def matmul(a, b, n):
    c = [0] * (n * n)
    for i in range(n):
        for j in range(n):
            s = 0
            for k in range(n):
                s += a[i * n + k] * b[k * n + j]
            c[i * n + j] = s
    return c

n = 400
c = matmul(fill(n, 1), fill(n, 2), n)
check = 0
for v in c:
    check = (check + v) % 1000000007
print(check)
