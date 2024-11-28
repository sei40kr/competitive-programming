use proconio::input;

static mut DP: [[[f64; 301]; 301]; 301] = [[[0.0; 301]; 301]; 301];

// 1個寿司が置かれた皿が c1 枚, 2個寿司が置かれた皿が c2 枚, 3個寿司が置かれた皿が c3 枚のとき、
// すべての皿がなくなるまでの操作回数の期待値を dp[c1][c2][c3] とすると
//
// dp[0][0][0] = 0
// dp[c1][c2][c3] = 1
//                + dp[c - 1][c2][c3] * c1 / n
//                + dp[c1 + 1][c2 - 1][c3] * c2 / n
//                + dp[c1][c2 + 1][c3 - 1] * c3 / n
//                + dp[c1][c2][c3] * (n - c1 - c2 - c3) / n
// (c1 + c2 + c3) / n * dp[c1][c2][c3] = 1
//                                     + dp[c - 1][c2][c3] * c1 / n
//                                     + dp[c1 + 1][c2 - 1][c3] * c2 / n
//                                     + dp[c1][c2 + 1][c3 - 1] * c3 / n
unsafe fn f(n: usize, c1: usize, c2: usize, c3: usize) -> f64 {
    if DP[c1][c2][c3] == 0.0 && (c1 != 0 || c2 != 0 || c3 != 0) {
        DP[c1][c2][c3] += 1.0;

        if 0 < c1 {
            DP[c1][c2][c3] += f(n, c1 - 1, c2, c3) * c1 as f64 / n as f64;
        }
        if 0 < c2 {
            DP[c1][c2][c3] += f(n, c1 + 1, c2 - 1, c3) * c2 as f64 / n as f64;
        }
        if 0 < c3 {
            DP[c1][c2][c3] += f(n, c1, c2 + 1, c3 - 1) * c3 as f64 / n as f64;
        }

        DP[c1][c2][c3] *= n as f64 / (c1 + c2 + c3) as f64;
    }

    DP[c1][c2][c3]
}

fn main() {
    input! {
        n: usize,
        xs: [usize; n]
    };

    let mut cs = [0usize; 4];
    for x in xs.into_iter() {
        cs[x] += 1;
    }

    let ans = unsafe { f(n, cs[1], cs[2], cs[3]) };
    println!("{}", ans);
}
