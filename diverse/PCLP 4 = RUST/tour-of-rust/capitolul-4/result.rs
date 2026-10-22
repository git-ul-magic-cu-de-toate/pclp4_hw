fn execută_ceva_ce_poate_eșua(i:i32) -> Result<f32,String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("acesta nu este numărul corect"))   
    }
}

fn main() {
    let result = execută_ceva_ce_poate_eșua(12);

    // match ne permite să destructurăm Result-ul într-un mod elegant
    // ca să gestionăm toate cazurile posibile
    match result {
        Ok(v) => println!("găsit {}", v),
        Err(e) => println!("Eroare: {}", e),
    }
}
