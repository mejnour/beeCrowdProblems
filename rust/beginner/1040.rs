use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let mut iter = input.split_whitespace();

    let n1: f64 = iter.next().unwrap().parse().unwrap();
    let n2: f64 = iter.next().unwrap().parse().unwrap();
    let n3: f64 = iter.next().unwrap().parse().unwrap();
    let n4: f64 = iter.next().unwrap().parse().unwrap();

    let m1: f64 = ((n1 * 2.0) + (n2 * 3.0) + (n3 * 4.0) + (n4 * 1.0))/(2.0 + 3.0 + 4.0 + 1.0);
    println!("Media: {:.1}", m1);

    if m1 >= 7.0 {
        println!("Aluno aprovado.");
    } else if m1 < 7.0 && m1 >= 5.0 {
        println!("Aluno em exame.");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("input2 failed");
        let n5: f64 = input2.trim().parse().expect("n5 failed parsing");
        println!("Nota do exame: {:.1}", n5);

        let m2: f64 = (m1 + n5) / 2.0;

        if m2 >= 5.0 {
            println!("Aluno aprovado.");
        } else {
            println!("Aluno reprovado.");
        }

        println!("Media final: {:.1}", m2);
    } else {
        println!("Aluno reprovado.");
    }
}