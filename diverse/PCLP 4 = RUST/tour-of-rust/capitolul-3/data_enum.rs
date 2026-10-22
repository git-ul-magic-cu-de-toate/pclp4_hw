#![allow(dead_code)] // această linie oprește avertizările compilatorului

enum Specii { Crab, Caracatiță, Pește, Scoică }
enum TipOtravă { Acid, Dureros, Letal }
enum Mărime { Mare, Mic }
enum Armă {
    Ghiară(i32, Mărime),
    Otravă(TipOtravă),
    None
}

struct CreaturăMarină {
    Specii: Specii,
    nume: String,
    nr_mâini: i32,
    nr_picioare: i32,
    Armă: Armă,
}

fn main() {
    // datele din CreaturăMarină sunt pe stivă
    let ferris = CreaturăMarină {
        // Struct-ul String este de asemenea pe stivă,
        // dar ține o referință a informației pe heap
        Specii: Specii,
        nume: String::from("Ferris"),
        nr_mâini: 2,
        nr_picioare: 4,
        Armă: Armă::Ghiară(2, Mărime::Mic),
    };

    match ferris.Specii {
        Specii::Crab => {
            match ferris.Armă {
                Armă::Ghiară(nr_ghiare, Mărime) => {
                    let mărime_descriere = match Mărime {
                        Mărime::Mare => "mari",
                        Mărime::Mic => "mici"
                    };
                    println!("ferris este un crab cu {} ghiare {}", nr_ghiare, mărime_descriere)
                },
                _ => println!("ferris este un crab cu alte arme")
            }
        },
        _ => println!("ferris este alt animal"),
    }
}
