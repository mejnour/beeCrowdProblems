use std::io;

const PI: f64 = 3.14159;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("input failed");

    let mut iter = input.split_whitespace();

    let a: f64 = iter.next().unwrap().parse().unwrap();
    let b: f64 = iter.next().unwrap().parse().unwrap();
    let c: f64 = iter.next().unwrap().parse().unwrap();

    println!("TRIANGULO: {:.3}", (a * c)/2.0);
    println!("CIRCULO: {:.3}", (PI * c.powf(2.0)));
    println!("TRAPEZIO: {:.3}", ((a + b) * c)/2.0);
    println!("QUADRADO: {:.3}", b * b);
    println!("RETANGULO: {:.3}", a * b);
}