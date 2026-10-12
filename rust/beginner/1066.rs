use std::io;

const READS_COUNT: i32 = 5;

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
    let vec1: Vec<i32> = read_values(READS_COUNT);

    let mut even_counter = 0;
    let mut positive_counter = 0;
    let mut negative_counter = 0;
    for number in vec1 {
        if number % 2 == 0 {
            even_counter += 1;
        }

        if number > 0 {
            positive_counter += 1;
        }

        if number < 0 {
            negative_counter += 1;
        }
    }

    println!("{} valor(es) par(es)", even_counter);
    println!("{} valor(es) impar(es)", READS_COUNT - even_counter);
    println!("{} valor(es) positivo(s)", positive_counter);
    println!("{} valor(es) negativo(s)", negative_counter);
}
