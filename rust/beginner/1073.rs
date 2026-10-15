use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let int_input: i32 = input.trim().parse().unwrap();

    for i in 1..int_input+1 {
        if i % 2 == 0 {
            println!("{}^2 = {}", i, i*i);
        }
    }
}