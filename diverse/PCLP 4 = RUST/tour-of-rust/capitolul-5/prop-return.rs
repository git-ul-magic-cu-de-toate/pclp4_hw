struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo { x: 42 }
    // proprietatea este cedată
}

fn main() {
    let foo = do_something();
    // foo devine proprietar
    // se renunță la foo din cauza terminării funcției
}
