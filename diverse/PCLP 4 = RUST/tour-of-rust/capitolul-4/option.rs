// Un tip de structură definit parțial
struct Geantă<T> {
    // Tipul de parametru T poate fi folosit din Option
    item: Option<T>,
}

fn main() {
    // Observați: O geantă pentru i32, care nu conține nimic! Trebuie să
    // specificăm tipul, altfel Rust nu va ști ce tip poate ține geanta
    let i32_geantă = Geantă::<i32> { item: None };

    if i32_geantă.item.is_none() {
        println!("nu este nimic în geantă!")
    } else {
        println!("se află ceva în geantă!")
    }

    let i32_geantă = Geantă::<i32> { item: Some(42) };

    if i32_geantă.item.is_some() {
        println!("se află ceva în geantă!")
    } else {
        println!("nu este nimic în geantă!")
    }

    // match ne permite să destructurăm Option în mod elegant pentru
    // a gestiona toate cazurile posibile!
    match i32_geantă.item {
        Some(v) => println!("găsit {} în geantă!", v),
        None => println!("nu a fost găsit nimic"),
    }
}

