use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let mut iter = input.split_whitespace();

        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();

        if x == 0 || y == 0 { break; }

        if x > 0 && y > 0 {
            println!("primeiro");
        } else if x < 0 && y > 0 {
            println!("segundo");
        } else if x < 0 && y < 0 {
            println!("terceiro");
        } else {
            println!("quarto");
        }
    }
}