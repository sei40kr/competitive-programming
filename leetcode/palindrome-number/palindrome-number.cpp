#include <bits/stdc++.h>

// palindrome-number.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define DEBUG(x) cout << #x << ": " << x << endl

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

class Solution {
 public:
  bool isPalindrome(const int x) {
    if (x < 0) {
      return false;
    }

    const int d = (int)log10(x) + 1, l = d / 2;
    REP(i, l) {
      if (x / (int)pow(10, i) % 10 != x / (int)pow(10, d - i - 1) % 10) {
        return false;
      }
    }

    return true;
  }
};

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  auto *sln = new Solution();
  DEBUG(sln->isPalindrome(1));
  DEBUG(sln->isPalindrome(4));
  DEBUG(sln->isPalindrome(9));
  DEBUG(sln->isPalindrome(121));

  delete sln;
}
