use std::io;

fn read_values(howmany: u32) -> Vec<Vec<f64>> {
    let mut vec: Vec<Vec<f64>> = Vec::new();
    for _ in 0..howmany as usize {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");

        let row: Vec<f64> = input
            .split_whitespace()
            .map(|token| token.parse().expect("valor inválido"))
            .collect();

        vec.push(row);
    }

    vec
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let tests: u32 = input.trim().parse().unwrap();

    let pop_vec: Vec<Vec<f64>> = read_values(tests);
    for values in pop_vec {
        let avg_mean: f64 = ((values[0] * 2.0) + (values[1] * 3.0) + (values[2] * 5.0))/(2.0 + 3.0 + 5.0);
        println!("{:.1}", avg_mean);
    }
}