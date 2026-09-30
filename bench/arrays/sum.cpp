#include <cstdint>
#include <iostream>
#include <vector>

int main() {
  const int64_t n = 10000000;
  std::vector<int64_t> a(n);
  int64_t x = 7, total = 0;
  for (auto &v : a) { x = (x * 1103 + 12345) % 1000003; v = x; }
  for (int r = 0; r < 20; r++) {
    int64_t s = 0;
    for (auto v : a) s += v;
    total += s;
  }
  std::cout << total << "\n";
}
