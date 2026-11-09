use std::io;

const READS_COUNT: i32 = 100;

fn read_values(howmany: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    for i in 0..howmany {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let aux: i32 = input.trim().parse().expect("input failed");
        
        if vec.len() == 0 {
            vec.push(aux);
            vec.push(i);
            continue;
        }

        if vec[0] < aux {
            vec[0] = aux;
            vec[1] = i;
        }
    }
    return vec;
}

fn main() {
    let vec: Vec<i32> = read_values(READS_COUNT);
    println!("{}\n{}", vec[0], vec[1]+1);
}
