fn main() {
    let x: i32 = 12;
    let y: i32 = 0;

    if y == 0 {
        eprintln!("cannot perform division by zero");
    } else {
        println!("{} / {} = {}", x, y, x / y);
    }

    return;
}
