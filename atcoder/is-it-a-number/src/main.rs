use proconio::input;

fn main() {
    input! {
        s: String
    }

    if s.len() == 3 && s.chars().all(|c| c.is_ascii_digit()) {
        println!("{}", s.parse::<u32>().unwrap() * 2);
    } else {
        println!("error");
    }
}
