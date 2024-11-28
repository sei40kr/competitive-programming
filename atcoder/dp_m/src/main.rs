use proconio::input;

static MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
        xs: [usize; n]
    }

    // dp[i][j] = i番目の子供に飴を配り終えた時点でj個の飴を配っていたときの総数
    let mut dp = vec![vec![0i64; k + 1]; n + 1];
    dp[0].fill(0);
    dp[0][0] = 1;

    for i in 1..=n {
        let x = xs[i - 1];

        let sums = dp[i - 1]
            .iter()
            .scan(0, |acc, &x| {
                *acc = (*acc + x) % MOD;
                Some(*acc)
            })
            .collect::<Vec<i64>>();

        for j in 0..=k {
            dp[i][j] += sums[j];
            if x < j {
                dp[i][j] -= sums[j - x - 1] + MOD;
            }
            dp[i][j] %= MOD;
        }
    }

    let ans = dp[n][k];
    println!("{}", ans);
}
