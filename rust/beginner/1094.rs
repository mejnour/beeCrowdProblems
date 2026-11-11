use std::io;

fn read_values(howmany: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = vec![0; 4];
    for _ in 0..howmany {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let mut iter = input.split_whitespace();

        let qtty: u32 = iter.next().unwrap().parse().unwrap();
        let kind: char = iter.next().unwrap().parse().unwrap();
        
        match kind {
            'C' => vec[1] += qtty,
            'R' => vec[2] += qtty,
            'S' => vec[3] += qtty,
            _ => {}
        }

        vec[0] += qtty;
    }

    vec
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let input_qtty: u32 = input.trim().parse().unwrap();

    let animals: Vec<u32> = read_values(input_qtty);

    println!("Total: {} cobaias", animals[0]);
    println!("Total de coelhos: {}", animals[1]);
    println!("Total de ratos: {}", animals[2]);
    println!("Total de sapos: {}", animals[3]);
    println!("Percentual de coelhos: {:.2} %", (animals[1] as f64 / animals[0] as f64) * 100.0);
    println!("Percentual de ratos: {:.2} %", (animals[2] as f64 / animals[0] as f64) * 100.0);
    println!("Percentual de sapos: {:.2} %", (animals[3] as f64 / animals[0] as f64) * 100.0);
}