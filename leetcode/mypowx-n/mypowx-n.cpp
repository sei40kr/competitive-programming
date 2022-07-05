#include <bits/stdc++.h>

// mypowx-n.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

using namespace std;

class Solution {
 public:
  double myPow(double x, int n) {
    if (n < 0) {
      return 1 / myPow(x, static_cast<int>(-1l * n - 1)) / x;
    }
    if (n == 0) {
      return 1;
    }
    if (n % 2 == 0) {
      return myPow(x * x, n / 2);
    }
    return myPow(x * x, n / 2) * x;
  }
};

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  auto *sln = new Solution();

  assert(sln->myPow(2.0, 0) == 1.0);
  assert(sln->myPow(2.0, 2) == 4.0);
  assert(sln->myPow(2.0, 3) == 8.0);

  delete sln;
}
