fn main() {
    // Putem fi expliciți cu privire la tip
    let mut i32_vec = Vec::<i32>::new(); // turbofish <3
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    // Rust este foarte inteligent când vine vorba de determinarea automată a tipului
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    // Este un macro superb!
    let string_vec = vec![String::from("Salutare"), String::from("Lume")];

    for cuvânt in string_vec.iter() {
        println!("{}", cuvânt);
    }
}
