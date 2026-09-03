use std::io;

fn main() {
    let mut distance = String::new();
    io::stdin().read_line(&mut distance).expect("distance failed");
    let distance: u32 = distance.trim().parse().unwrap();

    println!("{} minutos", (distance * 2));
}