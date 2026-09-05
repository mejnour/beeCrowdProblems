use std::io;

const YEAR: i32 = 365;
const MONTH: i32 = 30;

#[derive(Debug)]
struct TimeTable {
    years: u8,
    months: u8,
    days: u8,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let input: i32 = input.trim().parse().unwrap();

    let mut times = TimeTable {
        years: 0,
        months: 0,
        days: 0,
    };

    let mut counter = input;
    loop {
        if counter == 0 {
            break;
        }

        if counter >= YEAR && counter % YEAR >= 0 {
            counter -= YEAR;
            times.years += 1;
        } else if counter >= MONTH && counter % MONTH >= 0 {
            counter -= MONTH;
            times.months += 1;
        } else {
            times.days += counter as u8;
            counter -= counter;
        }
    }

    println!("{} ano(s)", times.years);
    println!("{} mes(es)", times.months);
    println!("{} dia(s)", times.days);
}