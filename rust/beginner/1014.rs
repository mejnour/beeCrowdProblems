use std::io;

fn main() {
    let mut distance = String::new();
    let mut spent_fuel = String::new();

    io::stdin().read_line(&mut distance).expect("distance failed");
    io::stdin().read_line(&mut spent_fuel).expect("spent_fuel failed");

    let distance: i32 = distance.trim().parse().unwrap();
    let spent_fuel: f32 = spent_fuel.trim().parse().unwrap();

    println!("{:.3} km/l", distance as f32 / spent_fuel);
}