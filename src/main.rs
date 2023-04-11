use std::io::stdin;

fn main() {

    println!("文字列を入力してください");

    let mut word = String::new();
    stdin().read_line(&mut word).ok();
    let answer = word.trim().to_string();

    println!("{}", answer)
    
}
