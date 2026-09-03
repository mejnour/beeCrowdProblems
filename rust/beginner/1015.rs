use std::io;

struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).expect("input1 failed");
    io::stdin().read_line(&mut input2).expect("input2 failed");

    let mut iter1 = input1.split_whitespace();
    let mut iter2 = input2.split_whitespace();

    let p1 = Point {
        x: iter1.next().unwrap().parse().unwrap(),
        y: iter1.next().unwrap().parse().unwrap(),
    };

    let p2 = Point {
        x: iter2.next().unwrap().parse().unwrap(),
        y: iter2.next().unwrap().parse().unwrap(),
    };

    // let delta_x = p2.x - p1.x;
    // let delta_y = p2.y - p1.y;
    //
    // println!("{}, {}", delta_x, delta_y);
    //
    // let delta_x_sqrd = delta_x.powf(2.0);
    // let delta_y_sqrd = delta_y.powf(2.0);
    //
    // println!("{delta_x_sqrd}, {delta_y_sqrd}");

    println!("{:.4}", ((p2.x - p1.x).powf(2.0) + (p2.y - p1.y).powf(2.0)).sqrt());
}