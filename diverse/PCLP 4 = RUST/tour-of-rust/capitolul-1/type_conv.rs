fn main() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);

    let x = 13_u32;
    let y = 29_u32;
    let mut z = (x as i32) - (y as i32);
    println!("{} - {} = {}",
            x, y, z);
    z = -z;
    println!("{}", z);
    
}
