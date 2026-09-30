#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int main(void) {
  int64_t n = 10000000, x = 7, total = 0;
  int64_t *a = malloc(n * sizeof(int64_t));
  for (int64_t i = 0; i < n; i++) { x = (x * 1103 + 12345) % 1000003; a[i] = x; }
  for (int r = 0; r < 20; r++) {
    int64_t s = 0;
    for (int64_t i = 0; i < n; i++) s += a[i];
    total += s;
  }
  printf("%lld\n", (long long)total);
  free(a);
}
