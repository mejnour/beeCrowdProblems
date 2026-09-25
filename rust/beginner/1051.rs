use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");

    let salary: f64 = input.trim().parse().unwrap();

    if (salary - 2000.00) <= 0.0 {
        println!("Isento");
        return
    }

    let mut tax: f64 = 0.0;
    let mut layer1: f64 = 0.0;
    if (salary - 4500.00) > 0.0 {
        layer1 = salary - 4500.00;
        tax += layer1 * 0.28;
    }

    let mut layer2: f64 = 0.0;
    if (salary - 3000.00) > 0.0 {
        layer2 = (salary - 3000.00) - layer1;
        tax += layer2 * 0.18;
    }

    let mut layer3: f64 = 0.0;
    if (salary - 2000.00) > 0.0 {
        layer3 = (salary - 2000.00) - layer2 - layer1;
        tax += layer3 * 0.08;
    }

    if tax != 0.0 {
        println!("R$ {:.2}", tax);
    }
}