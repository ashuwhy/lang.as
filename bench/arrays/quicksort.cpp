#include <cstdint>
#include <iostream>
#include <utility>
#include <vector>

static void quicksort(std::vector<int64_t> &v, int64_t lo, int64_t hi) {
  if (hi - lo < 2) return;
  int64_t pivot = v[hi - 1], store = lo;
  for (int64_t i = lo; i < hi - 1; i++)
    if (v[i] < pivot) std::swap(v[i], v[store++]);
  std::swap(v[store], v[hi - 1]);
  quicksort(v, lo, store);
  quicksort(v, store + 1, hi);
}

int main() {
  const int64_t n = 5000000;
  std::vector<int64_t> a(n);
  int64_t x = 12345, check = 0;
  for (auto &v : a) { x = (x * 48271) % 1000000007; v = x; }
  quicksort(a, 0, n);
  for (auto v : a) check = (check * 31 + (v % 1000 + 1000) % 1000) % 1000000007;
  std::cout << check << "\n";
}
