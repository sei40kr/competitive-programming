#include <bits/stdc++.h>

// first-missing-positive.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define ALL(a) (a).begin(), (a).end()

using namespace std;

class Solution {
 public:
  int firstMissingPositive(vector<int> &nums) {
    size_t l = nums.size();
    vector<bool> found(l, false);
    for_each(ALL(nums), [l, &found](int num) {
      --num;
      if (0 <= num && num < l) {
        found[num] = true;
      }
    });

    return find(ALL(found), false) - found.begin() + 1;
  }
};

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  auto *sln = new Solution();

  vector<int> vec1 = {1, 2, 0};
  assert(sln->firstMissingPositive(vec1) == 3);
  vector<int> vec2 = {3, 4, -1, 1};
  assert(sln->firstMissingPositive(vec2) == 2);

  delete sln;
}
