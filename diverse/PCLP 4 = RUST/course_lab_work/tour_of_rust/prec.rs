struct Location(i32, i32);

struct Gps {
    x: u32,
    y: u32,
}

fn main() {
    // Acesta este tot o strukctură pe stivă
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
    
    let prec = Gps {
        x: 34,
        y: 45,
    };
    
    println!("{}, {}", prec.x, prec.y);
}
