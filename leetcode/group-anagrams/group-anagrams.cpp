#include <bits/stdc++.h>

// group-anagrams.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define ALL(a) (a).begin(), (a).end()

using namespace std;

class Solution {
 public:
  vector<vector<string>> groupAnagrams(vector<string> &strs) {
    int ps[] = {2,  3,  5,  7,  11, 13, 17, 19, 23, 29, 31, 37, 41,
                43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101};
    map<long, list<string>> buf;

    for (string &str : strs) {
      buf[accumulate(ALL(str), 1l, [&ps](const long x, const char c) -> long {
        return 'a' <= c && c <= 'z' ? x * ps[c - 'a'] : x;
      })].push_back(str);
    }

    vector<vector<string>> res;
    res.reserve(buf.size());
    for (auto &pair : buf) {
      list<string> &list = pair.second;
      vector<string> vec(list.size());
      copy(ALL(list), vec.begin());
      res.push_back(vec);
    }

    return res;
  }
};

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  auto *sln = new Solution();
  delete sln;
}
