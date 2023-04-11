// 足し算
use std::io::stdin;

fn main() {
    let mut s1 = String::new();
    stdin().read_line(&mut s1).ok();
    let num1: i32 = s1.trim().parse().expect("入力された値が整数ではありません");

    let mut s2 = String::new();
    stdin().read_line(&mut s2).ok();
    let num2: i32 = s2.trim().parse().expect("入力された値が整数ではありません");

    let answer = num1 + num2;
    let combined = answer.to_string();

    println!("{}", combined);
}
