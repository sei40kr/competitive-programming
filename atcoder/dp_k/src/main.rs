use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        xs: [i32; n]
    }

    // dp[k] = 山がk個の石からなっているときに先手が勝てるかどうか
    let mut dp = vec![None; k as usize + 1];
    dp[0] = Some(false);

    for i in 0..=k {
        dp[i as usize] = Some(
            xs.iter()
                .any(|&x| 0 <= i - x && !dp[(i - x) as usize].unwrap()),
        );
    }

    if dp[k as usize].unwrap() {
        println!("First")
    } else {
        println!("Second")
    }
}
