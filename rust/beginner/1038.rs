use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let mut iter = input.split_whitespace();

    let x: i32 = iter.next().unwrap().parse().unwrap();
    let y: i32 = iter.next().unwrap().parse().unwrap();

    if x == 1 {
        println!("Total: R$ {:.2}", y as f32 * 4.0);
    } else if x == 2 {
        println!("Total: R$ {:.2}", y as f32 * 4.5);
    } else if x == 3 {
        println!("Total: R$ {:.2}", y as f32 * 5.0);
    } else if x == 4 {
        println!("Total: R$ {:.2}", y as f32 * 2.0);
    } else if x == 5 {
        println!("Total: R$ {:.2}", y as f32 * 1.5);
    }
}