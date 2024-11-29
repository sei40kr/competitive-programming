use proconio::input;

const MOD: u32 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        xs: [[i32; n]; n]
    }

    let mut dp = vec![0; 1 << n];
    dp[0] = 1;

    for s in 1..(1usize << n) {
        let i = s.count_ones() as usize - 1;

        for j in 0..n {
            if (s >> j) & 1 == 1 && xs[i][j] == 1 {
                dp[s] += dp[s ^ (1 << j)];
                dp[s] %= MOD;
            }
        }
    }

    let ans = dp[(1 << n) - 1];
    println!("{}", ans);
}
