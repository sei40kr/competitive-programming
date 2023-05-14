struct Solution;

fn main() {}

// submission codes start here

impl Solution {
    #[allow(dead_code)]
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1.clone();
        let mut nums2 = nums2.clone();
        nums1.sort();
        nums2.sort();

        let mut i = 0;
        let mut j = 0;
        let mut v = Vec::new();

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] && v.last() != Some(&nums1[i]) {
                v.push(nums1[i]);
                i += 1;
                j += 1;
            } else if nums1[i] < nums2[j] {
                i += 1;
            } else {
                j += 1
            }
        }

        return v;
    }
}
