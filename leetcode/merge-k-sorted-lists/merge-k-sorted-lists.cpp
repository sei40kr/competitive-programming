#include <bits/stdc++.h>

// merge-k-sorted-lists.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

using namespace std;

struct ListNode {
  int val;
  ListNode *next;

  ListNode(int x) : val(x), next(nullptr) {}
};

class Solution {
 public:
  ListNode *mergeKLists(vector<ListNode *> &lists) {
    auto rev_comp = [](ListNode *a, ListNode *b) -> bool {
      return a->val > b->val;
    };
    priority_queue<ListNode *, vector<ListNode *>, decltype(rev_comp)> q(
        rev_comp);

    for_each(lists.begin(), lists.end(), [&q](ListNode *node) {
      if (node != nullptr) {
        q.push(node);
      }
    });

    if (q.empty()) {
      return nullptr;
    }
    ListNode *root, *leaf;
    root = leaf = q.top();
    q.pop();
    if (leaf->next != nullptr) {
      q.push(leaf->next);
    }

    while (!q.empty()) {
      ListNode *node = q.top();
      q.pop();
      if (node->next != nullptr) {
        q.push(node->next);
      }

      leaf = leaf->next = node;
    }

    return root;
  }
};

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  auto *sln = new Solution();
  delete sln;
}
