use std::io;

fn main() {
    let mut start = String::new();
    let mut finish = String::new();
    io::stdin().read_line(&mut start).expect("start failed");
    io::stdin().read_line(&mut finish).expect("finish failed");
    let mut start: i32 = start.trim().parse().unwrap();
    let mut finish: i32 = finish.trim().parse().unwrap();

    let aux = start;
    if start > finish {
        start = finish;
        finish = aux;
    }

    let mut adder: i32 = 0;
    for number in start+1..finish {
        if number % 2 != 0 {
            adder += number;
        }
    }

    println!("{}", adder);
}