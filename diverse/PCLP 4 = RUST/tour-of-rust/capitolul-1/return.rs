fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn main() {
    // returnează un tuplu de valori
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // destructurează tuplul în două variabile
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}
