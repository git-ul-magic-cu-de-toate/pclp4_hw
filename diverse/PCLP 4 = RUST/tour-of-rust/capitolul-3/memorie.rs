struct CreaturăMarină {
    tip_animal: String,
    nume: String,
    nr_mâini: i32,
    nr_picioare: i32,
    armă: String,
}

fn main() {
    // datele din CreaturăMarină sunt pe stivă
    let ferris = CreaturăMarină {
        // Struct-ul String este de asemenea pe stivă,
        // dar ține o referință a informației pe heap
        tip_animal: String::from("crab"),
        nume: String::from("Ferris"),
        nr_mâini: 2,
        nr_picioare: 4,
        armă: String::from("ghiară"),
    };

    let sarah = CreaturăMarină {
        tip_animal: String::from("caracatiță"),
        nume: String::from("Sarah"),
        nr_mâini: 8,
        nr_picioare: 0,
        armă: String::from("creier"),
    };
    
    println!(
        "{} este {}. Are {} mâini, {} picioare, și {} pe post de armă",
        ferris.nume, ferris.tip_animal, ferris.nr_mâini, ferris.nr_picioare, ferris.armă
    );
    println!(
        "{} este {}. Are {} mâini, {} picioare. Nu are nicio armă..",
        sarah.nume, sarah.tip_animal, sarah.nr_mâini, sarah.nr_picioare
    );
}
