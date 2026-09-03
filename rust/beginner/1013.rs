use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");

    let mut iter = input.split_whitespace();

    let a: i32 = iter.next().unwrap().trim().parse().unwrap();
    let b: i32 = iter.next().unwrap().trim().parse().unwrap();
    let c: i32 = iter.next().unwrap().trim().parse().unwrap();

    let maior_ab = (a + b + (a - b).abs())/2;
    let maior_c = (maior_ab + c + (maior_ab - c).abs())/2;
    
    println!("{} eh o maior", maior_c);
}