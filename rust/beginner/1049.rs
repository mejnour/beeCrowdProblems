use std::io;

enum Vertebrata {
    Vertebrado(FamiliaeVert),
    Invertebrado(FamiliaeInvert),
}

enum FamiliaeVert {
    Ave(AveAlimentar),
    Mamifero(MamiferoAlimentar),
}

enum FamiliaeInvert {
    Inseto(InsetoAlimentar),
    Anelideo(AnelideoAlimentar),
}

enum AveAlimentar {
    Carnivoro(AveCarnivoro),
    Onivoro(AveOnivoro),
}

enum AveCarnivoro {
    Aguia
}

enum AveOnivoro {
    Pomba
}

enum MamiferoAlimentar {
    Herbivoro(MamiferoHerbivoro),
    Onivoro(MamiferoOnivoro),
}

enum MamiferoHerbivoro {
    Vaca
}

enum MamiferoOnivoro {
    Homem
}

enum InsetoAlimentar {
    Herbivoro(InsetoHerbivoro),
    Hematofago(InsetoHematofago),
}

enum InsetoHerbivoro {
    Lagarta
}

enum InsetoHematofago {
    Pulga
}

enum AnelideoAlimentar {
    Onivoro(AnelideoOnivoro),
    Hematofago(AnelideoHematofago),
}

enum AnelideoOnivoro {
    Minhoca,

}

enum AnelideoHematofago {
    Sanguessuga,
}

impl Vertebrata {
    fn from_input(v: &str, f: &str, a: &str) -> Option<Self> {
        let animal = match v {
            "vertebrado" => Vertebrata::Vertebrado(match f {
                "ave" => FamiliaeVert::Ave(match a {
                    "carnivoro" => AveAlimentar::Carnivoro(AveCarnivoro::Aguia),
                    "onivoro" => AveAlimentar::Onivoro(AveOnivoro::Pomba),
                    _ => return None,
                }),
                "mamifero" => FamiliaeVert::Mamifero(match a {
                    "onivoro" => MamiferoAlimentar::Onivoro(MamiferoOnivoro::Homem),
                    "herbivoro" => MamiferoAlimentar::Herbivoro(MamiferoHerbivoro::Vaca),
                    _ => return None,
                }),
                _ => return None,
            }),
            "invertebrado" => Vertebrata::Invertebrado(match f {
                "inseto" => FamiliaeInvert::Inseto(match a {
                    "hematofago" => InsetoAlimentar::Hematofago(InsetoHematofago::Pulga),
                    "herbivoro" => InsetoAlimentar::Herbivoro(InsetoHerbivoro::Lagarta),
                    _ => return None,
                }),
                "anelideo" => FamiliaeInvert::Anelideo(match a {
                    "hematofago" => AnelideoAlimentar::Hematofago(AnelideoHematofago::Sanguessuga),
                    "onivoro" => AnelideoAlimentar::Onivoro(AnelideoOnivoro::Minhoca),
                    _ => return None,
                }),
                _ => return None,
            }),
            _ => return None,
        };
        Some(animal)
    }

    fn nome(&self) -> &'static str {
        use FamiliaeInvert::*;
        use FamiliaeVert::*;
        use Vertebrata::*;
        match self {
            Vertebrado(Ave(AveAlimentar::Carnivoro(AveCarnivoro::Aguia))) => "aguia",
            Vertebrado(Ave(AveAlimentar::Onivoro(AveOnivoro::Pomba))) => "pomba",
            Vertebrado(Mamifero(MamiferoAlimentar::Onivoro(MamiferoOnivoro::Homem))) => "homem",
            Vertebrado(Mamifero(MamiferoAlimentar::Herbivoro(MamiferoHerbivoro::Vaca))) => "vaca",
            Invertebrado(Inseto(InsetoAlimentar::Hematofago(InsetoHematofago::Pulga))) => "pulga",
            Invertebrado(Inseto(InsetoAlimentar::Herbivoro(InsetoHerbivoro::Lagarta))) => "lagarta",
            Invertebrado(Anelideo(AnelideoAlimentar::Hematofago(AnelideoHematofago::Sanguessuga))) => "sanguessuga",
            Invertebrado(Anelideo(AnelideoAlimentar::Onivoro(AnelideoOnivoro::Minhoca))) => "minhoca",
        }
    }
}

fn main() {
    let mut vertebrata = String::new();
    io::stdin().read_line(&mut vertebrata).expect("vertebrata failed");

    let mut familiae = String::new();
    io::stdin().read_line(&mut familiae).expect("familiae failed");

    let mut alimentar = String::new();
    io::stdin().read_line(&mut alimentar).expect("alimentar failed");

    if let Some(animal) = Vertebrata::from_input(vertebrata.trim(), familiae.trim(), alimentar.trim()) {
        println!("{}", animal.nome());
    }
}