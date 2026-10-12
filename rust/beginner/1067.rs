use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let target: i32 = input.trim().parse().unwrap();

    for number in 1..target + 1 {
        if number % 2 != 0 {
            println!("{}", number);
        }
    }
}