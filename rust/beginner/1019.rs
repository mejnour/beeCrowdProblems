use std::io;

const HOUR: i32 = 60 * 60;

#[derive(Debug)]
struct TimeTable {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let input: i32 = input.trim().parse().unwrap();

    let mut times = TimeTable {
        hours: 0,
        minutes: 0,
        seconds: 0,
    };

    let mut counter = input;
    loop {
        if counter == 0 {
            break;
        }

        if counter >= HOUR && counter % HOUR >= 0 {
            counter -= HOUR;
            times.hours += 1;
        } else if counter >= 60 && counter % 60 >= 0 {
            counter -= 60;
            times.minutes += 1;
        } else {
            times.seconds += counter as u32;
            counter -= counter;
        }
    }

    println!("{}:{}:{}", times.hours, times.minutes, times.seconds);
}