use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let mut iter = input.split_whitespace();

    let mut vec_input: [f64; 3] = [
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap()
    ];

    vec_input.sort_by(|a, b| b.partial_cmp(a).unwrap());

    if vec_input[0] >= (vec_input[1] + vec_input[2]) {
        println!("NAO FORMA TRIANGULO");
        return;
    }

    if vec_input[0].powi(2) == (vec_input[1].powi(2) + vec_input[2].powi(2)) {
        println!("TRIANGULO RETANGULO");
    }

    if vec_input[0].powi(2) > (vec_input[1].powi(2) + vec_input[2].powi(2)) {
        println!("TRIANGULO OBTUSANGULO");
    }

    if vec_input[0].powi(2) < (vec_input[1].powi(2) + vec_input[2].powi(2)) {
        println!("TRIANGULO ACUTANGULO");
    }

    if vec_input[0] == vec_input[1] && vec_input[1] == vec_input[2] {
        println!("TRIANGULO EQUILATERO");
    }

    if (vec_input[0] == vec_input[1] && vec_input[1] != vec_input[2]) || (vec_input[1] == vec_input[2] && vec_input[1] != vec_input[0]) {
        println!("TRIANGULO ISOSCELES");
    }
}