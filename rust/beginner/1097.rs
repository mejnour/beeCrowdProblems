fn main() {
    let mut j: u32 = 7;
    for i in (1..=9).step_by(2) {
        println!("I={} J={}", i, j);
        println!("I={} J={}", i, j-1);
        println!("I={} J={}", i, j-2);
        j += 2;
    }
}