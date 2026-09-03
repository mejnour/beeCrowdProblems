use std::io;

fn main() {
    let mut prod1 = String::new();
    let mut prod2 = String::new();

    io::stdin().read_line(&mut prod1).expect("prod1 failed");
    io::stdin().read_line(&mut prod2).expect("prod2 failed");

    let mut iter1 = prod1.split_whitespace();
    let mut iter2 = prod2.split_whitespace();

    let _prod1_code: i32 = iter1.next().unwrap().parse().unwrap();
    let prod1_units: i32 = iter1.next().unwrap().parse().unwrap();
    let prod1_price: f64 = iter1.next().unwrap().parse().unwrap();

    let _prod2_code: i32 = iter2.next().unwrap().parse().unwrap();
    let prod2_units: i32 = iter2.next().unwrap().parse().unwrap();
    let prod2_price: f64 = iter2.next().unwrap().parse().unwrap();

    println!("VALOR A PAGAR: R$ {:.2}", (prod1_units as f64 * prod1_price) + (prod2_units as f64 * prod2_price));
}