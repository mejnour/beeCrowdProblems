use std::io;

fn main() {
    let mut seller_name = String::new();
    let mut seller_salary = String::new();
    let mut total_sales = String::new();

    io::stdin().read_line(&mut seller_name).expect("seller_name failed");
    io::stdin().read_line(&mut seller_salary).expect("seller_salary failed");
    io::stdin().read_line(&mut total_sales).expect("total_sales failed");

    let seller_salary: f64 = seller_salary.trim().parse().unwrap();
    let total_sales: f64 = total_sales.trim().parse().unwrap();

    println!("TOTAL = R$ {:.2}", (seller_salary + (total_sales * 0.15)));
}