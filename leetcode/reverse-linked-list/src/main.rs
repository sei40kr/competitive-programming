#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
struct Solution;

fn main() {}

// submission codes start here

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_list(node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut node = node;

        while let Some(mut n) = node.take() {
            let next = n.next.take();
            n.next = prev.take();
            prev = Some(n);
            node = next;
        }

        return prev;
    }
}
