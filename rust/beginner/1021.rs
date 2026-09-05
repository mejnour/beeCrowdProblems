use std::io;

#[derive(Debug)]
struct Notes {
    n100: u32,
    n50: u32,
    n20: u32,
    n10: u32,
    n5: u32,
    n2: u32,
    m100: u32,
    m50: u32,
    m25: u32,
    m10: u32,
    m5: u32,
    m1: u32,
}

fn main() {
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("value failed");
    let value: f64 = value.trim().parse().unwrap();

    let mut nota = Notes {
        n100: 0,
        n50: 0,
        n20: 0,
        n10: 0,
        n5: 0,
        n2: 0,
        m100: 0,
        m50: 0,
        m25: 0,
        m10: 0,
        m5: 0,
        m1: 0,
    };

    let mut counter_int: i32 = value.floor() as i32;
    let mut counter_frac: i32 = ((value - counter_int as f64) * 100.0).floor() as i32;
    loop {
        if counter_int < 1 {
            break;
        }

        if counter_int >= 100 && counter_int % 100 >= 0 {
            nota.n100 += 1;
            counter_int -= 100;
        } else if counter_int >= 50 && counter_int % 50 >= 0 {
            nota.n50 += 1;
            counter_int -= 50;
        } else if counter_int >= 20 && counter_int % 20 >= 0 {
            nota.n20 += 1;
            counter_int -= 20;
        } else if counter_int >= 10 && counter_int % 10 >= 0 {
            nota.n10 += 1;
            counter_int -= 10;
        } else if counter_int >= 5 && counter_int % 5 >= 0 {
            nota.n5 += 1;
            counter_int -= 5;
        } else if counter_int >= 2 && counter_int % 2 >= 0 {
            nota.n2 += 1;
            counter_int -= 2;
        } else if counter_int >= 1 && counter_int % 1 >= 0 {
            counter_int -= 1;
            counter_frac += 100;
        }
    }

    loop {
        if counter_frac < 1 {
            break;
        }

        if counter_frac >= 100 && counter_frac % 100 >= 0 {
            nota.m100 += 1;
            counter_frac -= 100;
        } else if counter_frac >= 50 && counter_frac % 50 >= 0 {
            nota.m50 += 1;
            counter_frac -= 50;
        } else if counter_frac >= 25 && counter_frac % 25 >= 0 {
            nota.m25 += 1;
            counter_frac -= 25;
        } else if counter_frac >= 10 && counter_frac % 10 >= 0 {
            nota.m10 += 1;
            counter_frac -= 10;
        } else if counter_frac >= 5 && counter_frac % 5 >= 0 {
            nota.m5 += 1;
            counter_frac -= 5;
        } else if counter_frac >= 1 && counter_frac % 1 >= 0 {
            nota.m1 += 1;
            counter_frac -= 1;
        } else {
            println!("NÃO IDENTIFICADO");
        }
    }

    println!("NOTAS:");
    println!("{} nota(s) de R$ 100.00", nota.n100);
    println!("{} nota(s) de R$ 50.00", nota.n50);
    println!("{} nota(s) de R$ 20.00", nota.n20);
    println!("{} nota(s) de R$ 10.00", nota.n10);
    println!("{} nota(s) de R$ 5.00", nota.n5);
    println!("{} nota(s) de R$ 2.00", nota.n2);
    println!("MOEDAS:");
    println!("{} moeda(s) de R$ 1.00", nota.m100);
    println!("{} moeda(s) de R$ 0.50", nota.m50);
    println!("{} moeda(s) de R$ 0.25", nota.m25);
    println!("{} moeda(s) de R$ 0.10", nota.m10);
    println!("{} moeda(s) de R$ 0.05", nota.m5);
    println!("{} moeda(s) de R$ 0.01", nota.m1);
}