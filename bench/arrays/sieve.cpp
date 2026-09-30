#include <cstdint>
#include <iostream>
#include <vector>

int main() {
  const int64_t n = 50000000;
  std::vector<bool> composite(n, false);
  int64_t count = 0;
  for (int64_t i = 2; i < n; i++) {
    if (!composite[i]) {
      count++;
      for (int64_t j = i * i; j < n; j += i) composite[j] = true;
    }
  }
  std::cout << count << "\n";
}
