use std::fs::{self, File};      // file system
use std::io::{self, Write};     // input output

fn main() -> io::Result<()> {
    let file_name = "output.txt";

    // create the file if it doesn't already exists
    let mut file = File::create(file_name)?;

    let text_to_write = "Hello, World!\n\
                            This is a line of text.\n";
    file.write_all(text_to_write.as_bytes())?;

    let absolute_path = fs::canonicalize(file_name)?;
    println!("Text has been written to: {:?}", absolute_path);

    Ok(())
}
