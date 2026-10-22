struct Foo {
    x: i32,
}

fn main() {
    // Instanțiem structuri (Foo) și le legăm de variabile (foo)
    // pentru a crea resurse în memorie
    let foo = Foo { x: 42 };
    // foo este proprietarul
}
