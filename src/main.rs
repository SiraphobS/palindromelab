use std::io;

fn main() {

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("failed to read input.");
    x=x.trim().to_string();
    let f = x.chars().rev().collect::<String>();

    println!("x:{}",x);
    println!("f:{}",f);

    if x == f {
    println!("Given string is Palindrome");
    } else {
    println!("Given string is not Palindrome");
    }
}