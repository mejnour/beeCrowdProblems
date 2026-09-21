use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let mut iter = input.split_whitespace();

    let initial_hour: i32 = iter.next().unwrap().parse().unwrap();
    let initial_minute: i32 = iter.next().unwrap().parse().unwrap();
    let finish_hour: i32 = iter.next().unwrap().parse().unwrap();
    let finish_minute: i32 = iter.next().unwrap().parse().unwrap();

    let elapsed_minutes: i32;
    let mut elapsed_hours: i32 = 0;

    if finish_minute < initial_minute {
        elapsed_minutes = (finish_minute + 60) - initial_minute;
        elapsed_hours -= 1;
    } else {
        elapsed_minutes = finish_minute - initial_minute;
    }

    if finish_hour > initial_hour {
        elapsed_hours += finish_hour - initial_hour;
    } else if finish_hour == initial_hour && finish_minute > initial_minute {
        elapsed_hours = 0;
    } else {
        elapsed_hours += (finish_hour + 24) - initial_hour;
    }

    println!("O JOGO DUROU {} HORA(S) E {} MINUTO(S)", elapsed_hours, elapsed_minutes);
}