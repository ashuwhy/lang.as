#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

static void quicksort(int64_t *v, int64_t lo, int64_t hi) {
  if (hi - lo < 2) return;
  int64_t pivot = v[hi - 1], store = lo;
  for (int64_t i = lo; i < hi - 1; i++)
    if (v[i] < pivot) { int64_t t = v[i]; v[i] = v[store]; v[store] = t; store++; }
  int64_t t = v[store]; v[store] = v[hi - 1]; v[hi - 1] = t;
  quicksort(v, lo, store);
  quicksort(v, store + 1, hi);
}

int main(void) {
  int64_t n = 5000000, x = 12345, check = 0;
  int64_t *a = malloc(n * sizeof(int64_t));
  for (int64_t i = 0; i < n; i++) { x = (x * 48271) % 1000000007; a[i] = x; }
  quicksort(a, 0, n);
  for (int64_t i = 0; i < n; i++) check = (check * 31 + (a[i] % 1000 + 1000) % 1000) % 1000000007;
  printf("%lld\n", (long long)check);
  free(a);
}
