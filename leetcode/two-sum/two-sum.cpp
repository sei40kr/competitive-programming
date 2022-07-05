#include <bits/stdc++.h>

// two-sum.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

class Solution {
 public:
  vector<int> twoSum(const vector<int> &nums, const int target) {
    const size_t l = nums.size();
    map<int, int> buf;
    vector<int> res(2);
    REP(i, l) {
      const int num = nums[i];
      if (target % 2 == 0 && num == target / 2) {
        const auto found = buf.find(num);
        if (found != buf.end()) {
          res[0] = found->second;
          res[1] = i;
          return res;
        }
      }

      buf.emplace(nums[i], (size_t)i);
    }

    REP(i, l) {
      const int num = nums[i];
      const auto found = buf.find(target - num);
      if (found != buf.end() && found->second != i) {
        res[0] = found->second;
        res[1] = i;
        sort(res.begin(), res.end());
        return res;
      }
    }

    res[0] = -1;
    res[1] = -1;
    return res;
  }
};

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  auto *sln = new Solution();

  vector<int> nums = {2, 7, 11, 15};
  vector<int> ans = sln->twoSum(nums, 9);
  assert(ans[0] == 0 && ans[1] == 1);

  delete sln;
}
