use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    io::stdin().read_line(&mut a).expect("a failed");
    io::stdin().read_line(&mut b).expect("b failed");
    io::stdin().read_line(&mut c).expect("c failed");

    let a: f64 = a.trim().parse().unwrap();
    let b: f64 = b.trim().parse().unwrap();
    let c: f64 = c.trim().parse().unwrap();

    println!("MEDIA = {:.1}", ((a * 2.0) + (b * 3.0) + (c * 5.0))/(2.0 + 3.0 + 5.0));
}