#include <bits/stdc++.h>

// longest-common-prefix.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

class Solution {
 public:
  string longestCommonPrefix(vector<string> &strs) {
    if (strs.empty()) {
      return "";
    }
    if (strs.size() == 1) {
      return strs[0];
    }

    const string &head = strs[0];
    int i = 0;
    for (; true; ++i) {
      char c = head[i];
      if (!all_of(strs.begin() + 1, strs.end(), [i, c](const string &str) {
            return i < str.length() && str[i] == c;
          })) {
        break;
      }
    }

    return head.substr(0,kstatic_cast<size_t>(i));
  }
};

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  auto *sln = new Solution();
  delete sln;
}
