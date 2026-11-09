use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let multi: u64 = input.trim().parse().unwrap();

    for i in 1..=10 {
        println!("{} x {} = {}", i, multi, multi * i);
    }
}