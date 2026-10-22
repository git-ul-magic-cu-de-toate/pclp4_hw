fn main() {
    // Rust intuiește tipul de date pentru x
    let x = 13;
    println!("{}", x);

    /* let -> immutable (oarecum ca in Python)
    operatiile
    x = 10;
    x = 13;
    dau eroare


    */

    // Rust poate fi și explicit atunci când stabilește tipul de date
    let x: f64 = 3.14159;
    println!("{}", x);

    // Rust poate declara o variabilă și să o inițializeze mai târziu,
    // dar acest lucru nu se face des
    let x;
    x = 0;
    println!("{}", x);
    println!();

    let y = 10;     // va genera warning : variabila neutilizata
    let _y = 991;   // nu ve genera niciun warning    
    println!("{} = {}", stringify!(y), y);
    println!("{} = {}", stringify!(_y), _y);

}
