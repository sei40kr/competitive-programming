struct Solution;

fn main() {
    println!("Hello, world!");
}

// submission codes start here

impl Solution {
    #[allow(dead_code)]
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = std::collections::HashSet::new();

        for email in emails {
            let mut parts = email.split('@');
            let local = parts
                .next()
                .unwrap()
                .split('+')
                .next()
                .unwrap()
                .replace(".", "");
            let domain = parts.next().unwrap();

            set.insert(format!("{}@{}", local, domain));
        }

        return set.len() as i32;
    }
}
