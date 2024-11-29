use std::iter;

use proconio::input;

// 逆にスライムを分解しながら分解前のスライムの大きさを加算していき、
// 与えられた大きさのスライム群を作ることを考える。
// xs の累積和を sums とし、 [l, r) に相当するスライムを分解するときの最小コストを dp[l][r] とおくと
// dp[i][i] = 0 (0 <= i <= n)
// dp[l][r] = min(dp[l][m] + dp[m][r]) + sums[r] - sums[l] (l < m < r)
static mut DP: [[Option<i64>; 401]; 401] = [[None; 401]; 401];

unsafe fn f(sums: &Vec<i64>, l: usize, r: usize) -> i64 {
    if l + 1 == r {
        return 0;
    }

    if let Some(cost) = DP[l][r] {
        return cost;
    }

    let cost = (l + 1..r)
        .map(|m| f(sums, l, m) + f(sums, m, r))
        .min()
        .unwrap()
        + sums[r]
        - sums[l];
    DP[l][r] = Some(cost);
    cost
}

fn main() {
    input! {
        n: usize,
        xs: [i64; n],
    }

    let sums = iter::once(0)
        .chain(xs.iter().scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        }))
        .collect::<Vec<_>>();

    let ans = unsafe { f(&sums, 0, n) };
    println!("{}", ans);
}
