use std::io;

fn main() {
    let mut spent_time = String::new();
    let mut avg_speed = String::new();

    io::stdin().read_line(&mut spent_time).expect("spent_time failed");
    io::stdin().read_line(&mut avg_speed).expect("avg_speed failed");

    let spent_time: u32 = spent_time.trim().parse().unwrap();
    let avg_speed: u32 = avg_speed.trim().parse().unwrap();

    println!("{:.3}", (avg_speed as f32 * spent_time as f32) / 12 as f32);
}