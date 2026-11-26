use std::io;

fn main() {
    let mut total: u32 = 0;
    let mut inter: u32 = 0;
    let mut gremio: u32 = 0;
    let mut empates: u32 = 0;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let mut iter = input.split_whitespace();
        let i_w: u8 = iter.next().unwrap().parse().unwrap();
        let g_w: u8 = iter.next().unwrap().parse().unwrap();
        
        if i_w > g_w { inter += 1; total += 1; } else if g_w > i_w { gremio += 1; total += 1; } else { empates += 1; total += 1; }
        
        println!("Novo grenal (1-sim 2-nao)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("input failed");
        let choice: u8 = input.trim().parse().unwrap();
        if choice == 2 { break; } else if choice != 1 && choice != 2 { continue; }
    }

    let vencedor = if gremio > inter { "Gremio venceu mais" } else if inter > gremio { "Inter venceu mais" } else { "Não houve vencedor" };
    println!("{} grenais", total);
    println!("Inter:{}", inter);
    println!("Gremio:{}", gremio);
    println!("Empates:{}", empates);
    println!("{}", vencedor);
}