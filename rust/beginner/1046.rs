use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let mut iter = input.split_whitespace();

    let hr_init: i32 = iter.next().unwrap().parse().unwrap();
    let hr_finish: i32 = iter.next().unwrap().parse().unwrap();

    if hr_finish <= hr_init {
        println!("O JOGO DUROU {} HORA(S)", (hr_finish + 24) - hr_init);
    } else {
        println!("O JOGO DUROU {} HORA(S)", hr_finish - hr_init);
    }
}