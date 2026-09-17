use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let mut iter = input.split_whitespace();

    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();

    if (a % b) == 0 || (b % a) == 0 {
        println!("Sao Multiplos");
    } else {
        println!("Nao sao Multiplos");
    }
}