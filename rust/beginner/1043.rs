use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let mut iter = input.split_whitespace();

    let a: f64 = iter.next().unwrap().parse().unwrap();
    let b: f64 = iter.next().unwrap().parse().unwrap();
    let c: f64 = iter.next().unwrap().parse().unwrap();

    let is_triangle: bool = (a < (b + c)) && (b < (a + c)) && (c < (a + b));

    if is_triangle {
        println!("Perimetro = {:.1}", (a + b + c));
    } else {
        println!("Area = {:.1}", ((a + b) * c)/2.0);
    }
}