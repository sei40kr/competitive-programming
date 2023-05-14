struct Solution;

fn main() {}

// submission codes start here

impl Solution {
    #[allow(dead_code)]
    pub fn first_uniq_char(s: String) -> i32 {
        let mut cnts = [0; 26];

        for c in s.chars() {
            cnts[c as usize - 'a' as usize] += 1;
        }

        for (i, c) in s.chars().enumerate() {
            if cnts[c as usize - 'a' as usize] == 1 {
                return i as i32;
            }
        }

        return -1;
    }
}
