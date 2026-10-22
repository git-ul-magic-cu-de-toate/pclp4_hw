fn execută_ceva_ce_poate_eșua(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("acesta nu este numărul corect"))
    }
}

fn main() -> Result<(), String> {
    // concis, dar este o presupunere și poate fi periculos
    let v = execută_ceva_ce_poate_eșua(42).unwrap();
    println!("găsit {}", v);
    
    // aceasta va returna panic!
    let v = execută_ceva_ce_poate_eșua(1).unwrap();
    println!("găsit {}", v);
    
    Ok(())
}
