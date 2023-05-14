struct Solution;

fn main() {}

// submission codes start here

use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq: HashMap<i32, i32> = HashMap::new();

        for n in nums {
            freq.entry(n).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        let mut v: Vec<(i32, i32)> = freq.into_iter().collect();
        v.sort_by_key(|&(_, cnt)| -cnt);

        return v.into_iter().take(k as usize).map(|(k, _)| k).collect();
    }
}
