use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let mut starts: i32 = input.trim().parse().unwrap();

    let mut counter: i32 = 0;
    while counter < 6 {
        if starts % 2 != 0 {
            println!("{}", starts);
            counter += 1;
        }
        starts += 1;
    }
}