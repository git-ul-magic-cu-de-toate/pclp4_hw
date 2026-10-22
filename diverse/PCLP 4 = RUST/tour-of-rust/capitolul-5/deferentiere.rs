fn main() {
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // primim o copie a valorii proprietarului
    *f = 13;      // setăm valoarea proprietarului prin referință
    println!("{}", bar);
    println!("{}", foo);
}
