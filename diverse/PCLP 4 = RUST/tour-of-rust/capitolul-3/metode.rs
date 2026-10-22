struct Gps{
    x: i32,
    y: i32,
}

fn main() {
    // Folosim o metodă statică ca să creem o instanță String
    let s = String::from("Hello world!");
    // Folosim o metodă a instanței
    println!("{} are {} de caractere.", s, s.len());

    let mut _coord = Gps{
        x : 12,
        y : -45,
    };

    // metode ale instantei
    _coord.x = -20;
    _coord.y = 11;
}
