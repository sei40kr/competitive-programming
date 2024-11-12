use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        n: i32,
        xs: [i32; n]
    };

    xs.windows(2).for_each(|pair| match pair[0].cmp(&pair[1]) {
        Ordering::Less => {
            println!("up {}", pair[1] - pair[0]);
        }
        Ordering::Equal => {
            println!("stay");
        }
        Ordering::Greater => {
            println!("down {}", pair[0] - pair[1])
        }
    });
}
