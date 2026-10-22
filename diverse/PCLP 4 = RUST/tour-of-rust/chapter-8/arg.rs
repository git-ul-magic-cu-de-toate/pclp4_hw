use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("number of arguments = {}", args.len());
    println!("the inline arguments are : {:?}", args);
}
