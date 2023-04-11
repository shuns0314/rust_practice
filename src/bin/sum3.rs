// 足し算
use std::io::stdin;

fn input_to_int() -> i32{
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    let num: i32 = s.trim().parse().expect("入力された値が整数ではありません");
    return num;
}

fn main() {
    let num1 = input_to_int();
    let num2: i32 = input_to_int();

    let answer = num1 + num2;
    let combined = answer.to_string();

    println!("{}", combined);
}