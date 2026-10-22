#![allow(dead_code)] // această linie oprește avertizările compilatorului

enum Specii {
    Crab,
    Caracatiță,
    Pește,
    Scoică
}

struct CreaturăMarină {
    Specii: Specii,
    nume: String,
    nr_mâini: i32,
    nr_picioare: i32,
    armă: String,
}

fn main() {
    let ferris = CreaturăMarină {
        Specii: Specii::Crab,
        nume: String::from("Ferris"),
        nr_mâini: 2,
        nr_picioare: 4,
        armă: String::from("ghiară"),
    };

    match ferris.Specii {
        Specii::Crab => println!("{} este crab",ferris.nume),
        Specii::Caracatiță => println!("{} este caracatiță",ferris.nume),
        Specii::Pește => println!("{} este pește",ferris.nume),
        Specii::Scoică => println!("{} este scoică",ferris.nume),
    }

}
