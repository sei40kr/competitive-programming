struct Solution;

fn main() {}

// submission codes start here

use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    #[allow(dead_code)]
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let mut seen = vec![vec![false; k as usize]; k as usize];
        let mut v = Vec::with_capacity(k as usize);
        heap.push((Reverse(nums1[0] as i64 + nums2[0] as i64), 0, 0));

        while !heap.is_empty() && v.len() < k as usize {
            let (_, i, j) = heap.pop().unwrap();
            v.push(vec![nums1[i], nums2[j]]);

            if i + 1 < nums1.len() && i + 1 < k as usize && !seen[i + 1][j] {
                seen[i + 1][j] = true;
                heap.push((Reverse(nums1[i + 1] as i64 + nums2[j] as i64), i + 1, j));
            }
            if j + 1 < nums2.len() && j + 1 < k as usize && !seen[i][j + 1] {
                seen[i][j + 1] = true;
                heap.push((Reverse(nums1[i] as i64 + nums2[j + 1] as i64), i, j + 1));
            }
        }

        return v;
    }
}
