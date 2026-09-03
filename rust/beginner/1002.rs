use std::io;

const PI: f64 = 3.14159;

fn main() {
    let mut input_r = String::new();
    io::stdin().read_line(&mut input_r).expect("input_r failed.");
    let input_r: f64 = input_r.trim().parse().unwrap();

    println!("A={:.4}", PI * (input_r * input_r));
}