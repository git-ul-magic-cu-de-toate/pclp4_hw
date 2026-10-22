// Un tip de structură definit parțial
struct BagOfHolding<T> {
    item: T,
}

fn adauga<T>(a: T, b: T) -> T where T: std::ops::Add<Output = T>, {
    return a + b
}

fn adauga_turbo<T>(a: T, b: T) -> T {
    return a + b
}


fn main() {
    // Observați: folosind tipuri generice, generăm tipuri create la momentul compilării
    // Turbofish ne lasă să fim expliciți
    let i32_bag = BagOfHolding::<i32> { item: 42 };
    let bool_bag = BagOfHolding::<bool> { item: true };

    // Rust poate deduce tipuri și pentru cele generice!
    let float_bag = BagOfHolding { item: 3.14 };

    // Atenție: nu puneți niciodată o geantă în altă geantă în viața reală
    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "boom!" },
    };

    println!(
        "{} {} {} {}",
        i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item
    );


    // cu torbofsih
    let rezultat_turbo_int = adauga_turbo::<i32>(5, 10);      // operator Turbofish to specify i32
    let rezultat_turbo_float = adauga_turbo::<f64>(3.5, 2.7); // operator Turbofish to specify f64

    println!("5 + 10 = {}", rezultat_turbo_int);
    println!("3.5 + 2.7 = {}", rezultat_turbo_float); 

    // fara turbofsih
    let rezultat_int = adauga(5, 10);       // fara turbofish
    let rezultat_float = adauga(3.5, 2.7);  // fara turbofish

    println!("5 + 10 = {}", rezultat_int);
    println!("3.5 + 2.7 = {}", rezultat_float);
}
