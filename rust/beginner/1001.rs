use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("A Failed");

    io::stdin()
        .read_line(&mut b)
        .expect("B Failed");

    let a: i32 = a.trim().parse().expect("A NaN");
    let b: i32 = b.trim().parse().expect("B NaN");

    println!("X = {}", a + b);
}