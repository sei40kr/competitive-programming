#include <bits/stdc++.h>

// author: Seong Yong-ju ( @sei40kr )

using namespace std;

class Solution {
 public:
  int reverse(int x) {
    auto d = (int)log10(abs(x)) + 1;
    long res = 0;

    for (int i = 0; i < d; ++i) {
      res += (x / (int)pow(10, i) % 10) * pow(10, d - i - 1);
    }

    if (INT_MIN <= res && res <= INT_MAX) {
      return (int)res;
    }

    return 0;
  }
};

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  auto *sln = new Solution();

  assert(sln->reverse(123) == 321);
  assert(sln->reverse(-123) == -321);

  delete sln;
}
