fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn last_digit(mut x: i32) -> u8 {
    
    if x < 0 {
        while x < -9 {
            x = x / 10;
        }
        return (-x) as u8;
    
    } else {
        while x > 9 {
            x = x / 10;
        }
        return x as u8;
    }
}

fn main() {
    println!("42 + 13 = {}", add(42, 13));
    println!("42 - 13 = {}", subtract(42, 13));
    
    let mut nr = 612129;
    println!("ultima cifra a lui {} = {}", nr, last_digit(nr));

    nr = -31324;
    println!("ultima cifra a lui {} = {}", nr, last_digit(nr));
}
