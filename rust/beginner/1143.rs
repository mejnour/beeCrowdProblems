use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let lines: u32 = input.trim().parse().unwrap();

    for i in 1..=lines {
        println!("{} {} {}", i, i*i, i*i*i);
    }
}