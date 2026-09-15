use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let mut iter = input.split_whitespace();

    let mut vec1: [i32; 3] = [
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap(),
        iter.next().unwrap().parse().unwrap()
    ];

    let vec1_orig = String::from(format!("{0}\n{1}\n{2}", vec1[0], vec1[1], vec1[2]));
    vec1.sort();

    println!("{0}\n{1}\n{2}\n", vec1[0], vec1[1], vec1[2]);
    println!("{}", vec1_orig);
}