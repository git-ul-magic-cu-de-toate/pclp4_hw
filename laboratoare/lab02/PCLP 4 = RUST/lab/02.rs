//! You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data);
    string_uppercase(&data);

    println!("Last character: {}", last_char);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) {
    let data_upper = data.to_uppercase();
    println!("{}", data_upper);
}
