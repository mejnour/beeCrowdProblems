fn main() {
    let mut i: u32 = 1;
    for j in (0..=60).rev().step_by(5) {
        println!("I={} J={}", i, j);
        i += 3;
    }
}