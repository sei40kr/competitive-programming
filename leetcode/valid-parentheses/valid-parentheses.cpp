#include <bits/stdc++.h>

// valid-parentheses.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2018 Seong Yong-ju

using namespace std;

class Solution {
 public:
  bool isValid(string s) {
    stack<char> stack;

    for (char c : s) {
      if (c == '(' || c == '[' || c == '{') {
        stack.push(c);
      } else if (!stack.empty()) {
        char last = stack.top();
        stack.pop();

        if ((c != ')' || last != '(') && (c != ']' || last != '[') &&
            (c != '}' || last != '{')) {
          return false;
        }
      } else {
        return false;
      }
    }

    return stack.empty();
  }
};
