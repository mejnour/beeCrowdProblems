use std::io;

fn read_values() -> Vec<(i32, i32)> {
    let mut vec: Vec<(i32, i32)> = Vec::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();

        if x == y { break; }

        vec.push((x, y));
    }
    vec
}

fn main() {
    let vec: Vec<(i32, i32)> = read_values();
    
    for range in vec {
        if range.0 > range.1 {
            println!("Decrescente");
        } else {
            println!("Crescente");
        }
    }
}