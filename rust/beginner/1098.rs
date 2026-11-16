fn main() {
    let i: f64 = 0.0;
    let mut j: f64 = 1.0;
    println!("I={} J={}", i, j);
    println!("I={} J={}", i, j + 1.0);
    println!("I={} J={}", i, j + 2.0);

    j += 9.0;
    for i in (2..=20).step_by(2){
        println!("I={} J={}", i as f64/10.0, (j + i as f64)/10.0);
        println!("I={} J={}", i as f64/10.0, 1.0 + (j + i as f64)/10.0);
        println!("I={} J={}", i as f64/10.0, 2.0 + (j + i as f64)/10.0);
    }
}