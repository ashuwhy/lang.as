#include <cstdint>
#include <iostream>
#include <vector>

static std::vector<int64_t> fill(int64_t n, int64_t seed) {
  std::vector<int64_t> m(n * n);
  int64_t x = seed;
  for (auto &v : m) { x = (x * 37 + 11) % 1000; v = x; }
  return m;
}

static std::vector<int64_t> matmul(const std::vector<int64_t> &a, const std::vector<int64_t> &b, int64_t n) {
  std::vector<int64_t> c(n * n);
  for (int64_t i = 0; i < n; i++)
    for (int64_t j = 0; j < n; j++) {
      int64_t s = 0;
      for (int64_t k = 0; k < n; k++) s += a[i * n + k] * b[k * n + j];
      c[i * n + j] = s;
    }
  return c;
}

int main() {
  const int64_t n = 400;
  auto c = matmul(fill(n, 1), fill(n, 2), n);
  int64_t check = 0;
  for (auto v : c) check = (check + v) % 1000000007;
  std::cout << check << "\n";
}
