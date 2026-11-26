use std::io;

fn main() {
    'outer: loop {
        let mut counter: u8 = 0;
        let mut accumulator: f64 = 0.0;
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("input failed");
            let grade: f64 = input.trim().parse().unwrap();

            if grade < 0.0 || grade > 10.0 { println!("nota invalida"); continue; }
            counter += 1; accumulator += grade;
            if counter == 2 { break; }
        }

        println!("media = {:.2}", accumulator/2.0);

        loop {
            println!("novo calculo (1-sim 2-nao)");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("input failed");
            let option: u8 = input.trim().parse().unwrap();
    
            if option != 2 && option != 1 { continue; }
            if option == 1 { continue 'outer; }
            if option == 2 { break 'outer; }
        }
    }
}