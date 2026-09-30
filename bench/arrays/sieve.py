n = 50_000_000
composite = bytearray(n)
count = 0
for i in range(2, n):
    if not composite[i]:
        count += 1
        for j in range(i * i, n, i):
            composite[j] = 1
print(count)
