// 文字列の連結
use std::io::stdin;

fn main() {
    let mut s1 = String::new();
    stdin().read_line(&mut s1).ok();

    let mut s2 = String::new();
    stdin().read_line(&mut s2).ok();

    let mut combined = String::from(s1.trim());
    combined.push_str(&s2.trim());

    println!("{}", combined);
}
