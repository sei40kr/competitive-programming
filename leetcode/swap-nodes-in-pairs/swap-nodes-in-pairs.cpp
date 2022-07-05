#include <bits/stdc++.h>

// swap-nodes-in-pairs.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2018 Seong Yong-ju

using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
  ListNode *swapPairs(ListNode *head) {
    if (head == NULL) {
      return NULL;
    }

    ListNode *next = head->next;
    if (next == NULL) {
      return head;
    }

    ListNode *nextToNext = next->next;

    // Swap nodes
    next->next = head;
    head->next = this->swapPairs(nextToNext);

    return next;
  }
};

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);
}
