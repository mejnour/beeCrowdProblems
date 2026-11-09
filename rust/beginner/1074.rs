use std::io;

fn read_values(howmany: u32) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    for _i in 0..howmany {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let aux: i32 = input.trim().parse().expect("input failed");
        vec.push(aux);
    }
    return vec;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let tests: u32 = input.trim().parse().unwrap();

    let vec1: Vec<i32> = read_values(tests);

    for number in vec1 {
        if number == 0 {
            println!("NULL");
            continue;
        }

        let mut response = String::new();
        if number % 2 == 0 {
            response.push_str("EVEN ");
        } else {
            response.push_str("ODD ");
        }

        if number > 0 {
            response.push_str("POSITIVE");
        } else {
            response.push_str("NEGATIVE");
        }

        println!("{}", response);
    }
}