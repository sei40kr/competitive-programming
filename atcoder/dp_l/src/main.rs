use std::cmp;

use proconio::input;

static mut DP: [[Option<i64>; 3001]; 3001] = [[None; 3001]; 3001];

unsafe fn f(n: usize, xs: &Vec<i64>, l: usize, r: usize) -> i64 {
    if n <= l + r {
        return 0;
    }

    if let Some(d) = DP[l][r] {
        return d;
    }

    let d = cmp::max(
        -f(n, xs, l + 1, r) + xs[l],
        -f(n, xs, l, r + 1) + xs[n - r - 1],
    );

    DP[l][r] = Some(d);

    d
}

fn main() {
    input! {
        n: usize,
        xs: [i64; n],
    }

    let ans = unsafe { f(n, &xs, 0, 0) };
    println!("{}", ans);
}
