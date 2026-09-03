use std::io;

const PI: f64 = 3.14159;

fn main() {
    let mut radius = String::new();

    io::stdin().read_line(&mut radius).expect("radius failed");

    let radius: f64 = radius.trim().parse().unwrap();

    println!("VOLUME = {:.3}", (4.0/3.0) * (PI * radius.powf(3.0)));
}