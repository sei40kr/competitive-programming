use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
        e: i32,
        f: i32
    }

    let mut xs = [a, b, c, d, e, f];
    xs.sort();
    xs.reverse();

    println!("{}", xs[2]);
}
