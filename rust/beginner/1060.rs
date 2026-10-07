use std::io;

fn read_values(howmany: i32) -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::new();
    for _i in 0..howmany {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let aux: f64 = input.trim().parse().expect("input failed");
        vec.push(aux);
    }
    return vec;
}

fn main() {
    let vec1: Vec<f64> = read_values(6);

    let mut counter = 0;
    for number in vec1 {
        if number >= 0.0 {
            counter += 1;
        }
    }

    println!("{} valores positivos", counter);
}