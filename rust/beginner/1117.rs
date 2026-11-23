use std::io;

fn main() {
    let mut sum_nota: f64 = 0.0;
    let mut valid_counter: i8 = 0;
    loop {
        if valid_counter == 2 { break; }
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let nota: f64 = input.trim().parse().unwrap();

        if nota < 0.0 || nota > 10.0 {
            println!("nota invalida");
            continue;
        }
        
        valid_counter += 1;
        sum_nota += nota;
    }

    println!("media = {}", sum_nota / 2.0);
}