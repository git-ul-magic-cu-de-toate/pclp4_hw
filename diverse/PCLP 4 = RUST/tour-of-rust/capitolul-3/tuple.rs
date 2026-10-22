struct Location(i32, i32);

struct Student(String, String, u8, u8, String, String);
// nume, prenume, varsta, anul, Seria, Grupa

fn main() {
    // Acesta este tot o structură pe stivă
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);

    let _gps = Location(11, -12);

    let bgd = Student(
        "Trifan".to_string(), 
        "Bogdan".to_string(),
        19,
        2,
        "CD".to_string(), 
        "322".to_string()
    );

    println!("Student : {} {} \n\
             Varsta : {} \n\
             Anul : {} \n\
             gp : {} {}",
        bgd.0, bgd.1, bgd.2, bgd.3, bgd.5, bgd.4)
}
