use std::cmp::min;

// frog-1.rs --- Frog 1
// author: Seong Yong-ju <sei40kr@gmail.com>

// tanakah's input macro
// cf https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

fn main() {
    input! {
        n: usize,
        hs: [i32; n],
    };

    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;

    for i in 0..n - 2 {
        dp[i + 1] = min(dp[i] + (hs[i] - hs[i + 1]).abs(), dp[i + 1]);
        dp[i + 2] = min(dp[i] + (hs[i] - hs[i + 2]).abs(), dp[i + 2]);
    }
    dp[n - 1] = min(dp[n - 2] + (hs[n - 2] - hs[n - 1]).abs(), dp[n - 1]);

    let ans = dp[n - 1];
    println!("{}", ans);
}
