use std::env;

// 3sum.rs --- 3Sum
// author: Seong Yong-ju <sei40kr@gmail.com>

trait Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 3 {
            return vec![];
        }

        let mut sorted_nums = nums;
        sorted_nums.sort();

        let mut results = (0..(n - 2))
            .map(|i| {
                ((i + 1)..(n - 1))
                    .filter_map(|j| {
                        match sorted_nums[(j + 1)..]
                            .binary_search(&(-sorted_nums[i] - sorted_nums[j]))
                        {
                            Ok(k) => {
                                Some(vec![sorted_nums[i], sorted_nums[j], sorted_nums[j + k + 1]])
                            }
                            Err(_) => None,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        results.sort();
        results.dedup();
        results
    }
}
