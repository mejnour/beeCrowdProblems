use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let read_num: u32 = input.trim().parse().unwrap();

    let mut count_in = 0;
    let mut count_out = 0;
    for _i in 1..=read_num {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let num: f64 = input.trim().parse().unwrap();

        if num >= 10.0 && num <= 20.0 {
            count_in += 1;
        } else {
            count_out += 1;
        }
    }
    println!("{} in", count_in);
    println!("{} out", count_out);
}