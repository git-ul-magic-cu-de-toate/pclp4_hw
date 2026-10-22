fn execută_ceva_ce_poate_eșua(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("acesta este numărul corect"))
    }
}

// Main nu returnează nicio valoare, dar poate returna o eroare!
fn main() -> Result<(), String> {
    let result = execută_ceva_ce_poate_eșua(12);
    
    match result {
        Ok(v) => println!("găsit {}", v),
        Err(_e) => {
            // gestionează această eroare grațios
            
            // returneaza o nouă eroare din main care explică ce s-a întâmplat!
            return Err(String::from("ceva nu a mers bine în main!"));
        }
    }

    // Observați că folosim o unitate în interiorul unui Result Ok
    // ca să transmitem faptul că totul a mers bine
    Ok(())
}
