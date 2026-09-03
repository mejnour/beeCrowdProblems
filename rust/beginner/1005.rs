use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).expect("a failed");
    io::stdin().read_line(&mut b).expect("b failed");

    let a: f64 = a.trim().parse().unwrap();
    let b: f64 = b.trim().parse().unwrap();

    println!("MEDIA = {:.5}", ((a * 3.5) + (b * 7.5))/(3.5 + 7.5));
}