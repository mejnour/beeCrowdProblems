use std::io;

fn read_values(howmany: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    for _i in 0..howmany {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let aux: i32 = input.trim().parse().expect("input failed");
        vec.push(aux);
    }
    return vec;
}

fn main() {
    let vec1: Vec<i32> = read_values(5);

    let mut counter = 0;
    for number in vec1 {
        if number % 2 == 0 {
            counter += 1;
        }
    }

    println!("{} valores pares", counter);
}
