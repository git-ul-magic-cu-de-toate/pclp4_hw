fn main() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "am găsit 13";
        }
    };
    println!("din buclă: {}", v);
}
