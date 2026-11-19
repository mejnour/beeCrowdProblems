use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let senha: i32 = input.trim().parse().unwrap();

        if senha == 2002 {
            println!("Acesso Permitido");
            break;
        } else {
            println!("Senha Invalida");
            continue;
        }
    }
}