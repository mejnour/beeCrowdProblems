use std::io;

#[derive(Debug)]
struct Notes {
    n100: u32,
    n50: u32,
    n20: u32,
    n10: u32,
    n5: u32,
    n2: u32,
    n1: u32,
}

fn main() {
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("value failed");
    let value: i32 = value.trim().parse().unwrap();

    let mut nota = Notes {
        n100: 0,
        n50: 0,
        n20: 0,
        n10: 0,
        n5: 0,
        n2: 0,
        n1: 0,
    };

    let mut counter = value;
    loop {
        if counter == 0 {
            break;
        }

        if counter >= 100 && counter % 100 >= 0 {
            nota.n100 += 1;
            counter -= 100;
        } else if counter >= 50 && counter % 50 >= 0 {
            nota.n50 += 1;
            counter -= 50;
        } else if counter >= 20 && counter % 20 >= 0 {
            nota.n20 += 1;
            counter -= 20;
        } else if counter >= 10 && counter % 10 >= 0 {
            nota.n10 += 1;
            counter -= 10;
        } else if counter >= 5 && counter % 5 >= 0 {
            nota.n5 += 1;
            counter -= 5;
        } else if counter >= 2 && counter % 2 >= 0 {
            nota.n2 += 1;
            counter -= 2;
        } else if counter >= 1 && counter % 1 >= 0 {
            nota.n1 += 1;
            counter -= 1;
        } else {
            println!("NOTA NÃO IDENTIFICADA");
        }
    }

    println!("{}", value);
    println!("{} nota(s) de R$ 100,00", nota.n100);
    println!("{} nota(s) de R$ 50,00", nota.n50);
    println!("{} nota(s) de R$ 20,00", nota.n20);
    println!("{} nota(s) de R$ 10,00", nota.n10);
    println!("{} nota(s) de R$ 5,00", nota.n5);
    println!("{} nota(s) de R$ 2,00", nota.n2);
    println!("{} nota(s) de R$ 1,00", nota.n1);
}