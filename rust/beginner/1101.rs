use std::io;

fn read_values() -> Vec<(i32, i32)> {
    let mut vec: Vec<(i32, i32)> = Vec::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();

        if x <= 0 || y <= 0 { break; }
        let (lo, hi) = if x < y { (x, y) } else { (y, x) };

        vec.push((lo, hi));
    }
    vec
}

fn main() {
    let vec: Vec<(i32, i32)> = read_values();
    
    for range in vec {
        let mut sum: i32 = 0;
        for i in range.0..=range.1 {
            sum += i;
            print!("{} ", i);
        }
        print!("Sum={}", sum);
        println!();
    }
}