use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let num: f64 = input.trim().parse().unwrap();

    if num >= 0.0 && num <= 25.0 {
        println!("Intervalo [0,25]");
    } else if num > 25.0 && num <= 50.0 {
        println!("Intervalo (25,50]");
    } else if num > 50.0 && num <= 75.0 {
        println!("Intervalo (50,75]");
    } else if num > 75.0 && num <= 100.0 {
        println!("Intervalo (75,100]");
    } else {
        println!("Fora de intervalo");
    }
}