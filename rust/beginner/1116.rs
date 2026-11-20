use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let tests: i32 = input.trim().parse().unwrap();

    for _ in 0..tests {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let mut iter = input.split_whitespace();

        let x: f64 = iter.next().unwrap().parse().unwrap();
        let y: f64 = iter.next().unwrap().parse().unwrap();

        if y == 0.0 {
            println!("divisao impossivel");
            continue;
        }

        if x == 0.0 {
            println!("0.0");
            continue;
        }

        println!("{:.1}", x/y);
    } 
}