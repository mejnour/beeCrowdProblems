use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let num: i32 = input.trim().parse().unwrap();

    for i in 1..=10000 {
        if i % num == 2 {
            println!("{}", i);
        }
    }
}