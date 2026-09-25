use std::io;

fn print_output(salary: f64, percentage: i32) {
    let percentage_dec: f64 = percentage as f64 / 100 as f64;
    println!("Novo salario: {:.2}\nReajuste ganho: {:.2}\nEm percentual: {:.2} %",
        (salary * percentage_dec) + salary,
        (salary * percentage_dec),
        percentage
    )
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");

    let salary: f64 = input.trim().parse().unwrap();

    if salary <= 400.00 {
        print_output(salary, 15);
    } else if salary > 400.00 && salary <= 800.00 {
        print_output(salary, 12);
    } else if salary > 800.00 && salary <= 1200.00 {
        print_output(salary, 10);
    } else if salary > 1200.00 && salary <= 2000.00 {
        print_output(salary, 7);
    } else {
        print_output(salary, 4);
    }
}