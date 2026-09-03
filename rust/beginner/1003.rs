use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).expect("a failed");
    io::stdin().read_line(&mut b).expect("b failed");

    let a: i32 = a.trim().parse().unwrap();
    let b: i32 = b.trim().parse().unwrap();

    println!("SOMA = {}", a + b);
}