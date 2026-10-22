fn main() {
    let mut x = 42;
    println!("{}", x);
    x = 15;
    println!("{}", x);
    x = 13;
    println!("{}", x);

    // folosind underscore inainte numelui, nu se va genera eroare daca variabila nu va fi folosita
    let mut _x = 10;       
    let mut x = 1112;
    println!("{} = {}", stringify!(_x), _x);
    println!("{} = {}", stringify!(x), x);

    _x = 9;
    x = 99999;
    println!("{} = {}", stringify!(_x), _x);
    println!("{} = {}", stringify!(x), x);
}
