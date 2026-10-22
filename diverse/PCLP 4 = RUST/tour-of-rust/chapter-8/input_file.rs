use std::fs;    // `fs` stands for file system

fn main() -> Result<(), std::io::Error> {
    // it will automatically throw an error
    let file_contents = fs::read_to_string("example.txt")?;

    println!("File contents:\n{}", file_contents);
    return Ok(());
}
