fn make_nothing() -> () {
    return ();
}

// tipul pe care îl returnează este în mod implicit ()
fn make_nothing2() {
    // această funcție va returna (), dacă nu este specificat altceva pentru returnare
}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();

    // Afișarea unui text de depanare pentru a și b
    // Pentru că e greu să printăm nimicul
    println!("Valoarea lui a: {:?}", a);
    println!("Valoarea lui b: {:?}", b);
}
