use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let rep: u32 = input.trim().parse().unwrap();

    let mut start: u32 = 1;
    for _ in 1..=rep {
        println!("{} {} {} PUM", start, start+1, start+2);
        start += 4;
    }
}