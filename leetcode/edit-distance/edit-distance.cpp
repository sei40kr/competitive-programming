#include <algorithm>
#include <iostream>
#include <string>

// edit-distance.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2018 Seong Yong-ju

#define FOR(i, a, b) for (int i = (a); i < (b); ++i)
#define REP(i, n) for (int i = 0; i < (n); ++i)

class Solution {
public:
  int minDistance(std::string word1, std::string word2) {
    int size1 = static_cast<int>(word1.length());
    int size2 = static_cast<int>(word2.length());
    int dp[size1 + 1][size2 + 1];
    REP(i, size1 + 1) { dp[i][0] = i; }
    REP(i, size2 + 1) { dp[0][i] = i; }

    FOR(i, 1, size1 + 1) {
      FOR(j, 1, size2 + 1) {
        dp[i][j] = std::min(
            {dp[i - 1][j] + 1, dp[i][j - 1] + 1,
             dp[i - 1][j - 1] + (word1[i - 1] == word2[j - 1] ? 0 : 1)});
      }
    }

    return dp[size1][size2];
  }
};

int main() {
  std::cin.tie(nullptr);
  std::ios::sync_with_stdio(false);

  auto sln = new Solution();
  std::cout << sln->minDistance("kitten", "sitting") << std::endl;
  delete sln;
}
