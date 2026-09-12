use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let mut iter = input.split_whitespace();

    let x: f64 = iter.next().unwrap().parse().unwrap();
    let y: f64 = iter.next().unwrap().parse().unwrap();

    if x > 0.0 && y > 0.0 {
        println!("Q1");
    } else if x < 0.0 && y > 0.0 {
        println!("Q2");
    } else if x < 0.0 && y < 0.0 {
        println!("Q3");
    } else if x > 0.0 && y < 0.0 {
        println!("Q4");
    } else if y == 0.0 && x != 0.0  {
        println!("Eixo X");
    } else if x == 0.0 && y != 0.0  {
        println!("Eixo Y");
    } else {
        println!("Origem");
    }
}