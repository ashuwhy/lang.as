#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

static int64_t *fill(int64_t n, int64_t seed) {
  int64_t *m = malloc(n * n * sizeof(int64_t)), x = seed;
  for (int64_t i = 0; i < n * n; i++) { x = (x * 37 + 11) % 1000; m[i] = x; }
  return m;
}

static int64_t *matmul(const int64_t *a, const int64_t *b, int64_t n) {
  int64_t *c = calloc(n * n, sizeof(int64_t));
  for (int64_t i = 0; i < n; i++)
    for (int64_t j = 0; j < n; j++) {
      int64_t s = 0;
      for (int64_t k = 0; k < n; k++) s += a[i * n + k] * b[k * n + j];
      c[i * n + j] = s;
    }
  return c;
}

int main(void) {
  int64_t n = 400;
  int64_t *a = fill(n, 1), *b = fill(n, 2), *c = matmul(a, b, n), check = 0;
  for (int64_t i = 0; i < n * n; i++) check = (check + c[i]) % 1000000007;
  printf("%lld\n", (long long)check);
  free(a); free(b); free(c);
}
