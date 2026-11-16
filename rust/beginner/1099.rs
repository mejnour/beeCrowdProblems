use std::io;

fn read_values(howmany: u32) -> Vec<u32> {
    let mut vec: Vec<u32> = vec![0; howmany as usize];
    for i in 0..howmany {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let mut iter = input.split_whitespace();

        let mut x: i32 = iter.next().unwrap().parse().unwrap();
        let mut y: i32 = iter.next().unwrap().parse().unwrap();

        if (x - y).abs() <= 1 {
            continue;
        }

        if x > y {
            std::mem::swap(&mut x, &mut y);
        }

        let mut sum: u32 = 0;
        for j in x+1..y {
            if j % 2 != 0 {
                sum += j as u32;
            }
        }

        vec[i as usize] = sum;
    }

    vec
}

fn main() {
    let mut tests = String::new();
    io::stdin().read_line(&mut tests).expect("tests failed");
    let tests: u32 = tests.trim().parse().unwrap();

    for item in read_values(tests) {
        println!("{}", item);
    }
}