 use std::io;
fn main(){
    let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed");
let numbers: Vec<i32> = input
    .trim()
    .split_whitespace()
    .map(|s| s.parse().expect("Parse error"))
    .collect();

println!{"{:?}", numbers};
}