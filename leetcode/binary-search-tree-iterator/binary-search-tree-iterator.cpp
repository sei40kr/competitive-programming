#include <bits/stdc++.h>

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  explicit TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

/**
 * Definition for binary tree
 */
class BSTIterator {
 private:
  std::list<TreeNode *> list;

  void addTreeNodeToStack(TreeNode *node) {
    list.push_front(node);

    if (node->left != NULL) {
      addTreeNodeToStack(node->left);
    }
  }

 public:
  explicit BSTIterator(TreeNode *root) {
    if (root != NULL) {
      addTreeNodeToStack(root);
    }
  }

  /** @return whether we have a next smallest number */
  bool hasNext() { return !list.empty(); }

  /** @return the next smallest number */
  int next() {
    TreeNode *node = list.front();
    list.pop_front();

    if (node->right != NULL) {
      addTreeNodeToStack(node->right);
    }

    return node->val;
  }
};

/**
 * Your BSTIterator will be called like this:
 * BSTIterator i = BSTIterator(root);
 * while (i.hasNext()) cout << i.next();
 */
