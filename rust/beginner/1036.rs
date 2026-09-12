use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let mut iter = input.split_whitespace();

    let a: f64 = iter.next().unwrap().parse().unwrap();
    let b: f64 = iter.next().unwrap().parse().unwrap();
    let c: f64 = iter.next().unwrap().parse().unwrap();

    let delta: f64 = b * b - (4.0 * a * c);

    if a == 0.0 || delta < 0.0 {
        println!("Impossivel calcular");
    } else {
        let r1: f64 = (- b + (delta).sqrt())/(2.0 * a);
        let r2: f64 = (- b - (delta).sqrt())/(2.0 * a);
        println!("R1 = {:.5}", r1);
        println!("R2 = {:.5}", r2);
    }
}
