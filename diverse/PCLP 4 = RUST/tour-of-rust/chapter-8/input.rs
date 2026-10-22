use std::io;

fn main() {
    let mut input = String::new();

    // the read will be stopped by `\n` character
    if let Ok(_) = io::stdin().read_line(&mut input) {
        println!("Input text : {}", input);
    }
}
