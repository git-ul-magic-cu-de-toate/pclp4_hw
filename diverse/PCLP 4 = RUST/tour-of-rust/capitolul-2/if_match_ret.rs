fn example() -> i32 {
    let x = 42;
    // expresia ternară concisă
    let v = if x < 42 { -1 } else { 1 };
    println!("din if: {}", v);

    let food = "hamburger";
    let result = match food {
        "hotdog" => "este un hotdog",
        // observați că acoladele sunt opționale când există
        // o expresie simplă de returnare
        _ => "nu este un hotdog",
    };
    println!("tipul de mâncare: {}", result);

    let v = {
        // Acest domeniu de vizibilitate ne permite să nu poluăm spațiul funcțiilor
        let a = 1;
        let b = 2;
        a + b
    };
    println!("din bloc: {}", v);

    // Modul idiomatic de a returna o valoare în Rust la sfâșitul unei funcții
    v + 4
}

fn main() {
    println!("din funcție: {}", example());
}
