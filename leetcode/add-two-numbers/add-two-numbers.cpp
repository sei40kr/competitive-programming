#include <bits/stdc++.h>

// add-two-numbers.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2018 Seong Yong-ju

using namespace std;

struct ListNode {
  int val;
  ListNode* next;
  explicit ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
 public:
  ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
    return addTwoNumbersInternal(l1, l2, 0);
  }

 private:
  ListNode* addTwoNumbersInternal(ListNode* l1, ListNode* l2, int increment) {
    if (l1 == NULL && l2 == NULL && increment == 0) {
      return NULL;
    }

    int sum =
        (l1 != NULL ? l1->val : 0) + (l2 != NULL ? l2->val : 0) + increment;

    ListNode* ans = new ListNode(sum % 10);
    ans->next = addTwoNumbersInternal((l1 != NULL ? l1->next : NULL),
                                      (l2 != NULL ? l2->next : NULL), sum / 10);

    return ans;
  }
};
