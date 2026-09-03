use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    let mut d = String::new();

    io::stdin().read_line(&mut a).expect("a failed");
    io::stdin().read_line(&mut b).expect("b failed");
    io::stdin().read_line(&mut c).expect("c failed");
    io::stdin().read_line(&mut d).expect("d failed");

    let a: i32 = a.trim().parse().unwrap();
    let b: i32 = b.trim().parse().unwrap();
    let c: i32 = c.trim().parse().unwrap();
    let d: i32 = d.trim().parse().unwrap();

    println!("DIFERENCA = {}", ((a * b) - (c * d)));
}