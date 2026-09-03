use std::io;

fn main() {
    let mut employee_num = String::new();
    let mut worked_hours = String::new();
    let mut amount_hour = String::new();

    io::stdin().read_line(&mut employee_num).expect("employee_num failed");
    io::stdin().read_line(&mut worked_hours).expect("worked_hours failed");
    io::stdin().read_line(&mut amount_hour).expect("amount_hour failed");

    let employee_num: i32 = employee_num.trim().parse().unwrap();
    let worked_hours: i32 = worked_hours.trim().parse().unwrap();
    let amount_hour: f64 = amount_hour.trim().parse().unwrap();

    println!("NUMBER = {}", employee_num);
    println!("SALARY = U$ {:.2}", worked_hours as f64 * amount_hour);
}