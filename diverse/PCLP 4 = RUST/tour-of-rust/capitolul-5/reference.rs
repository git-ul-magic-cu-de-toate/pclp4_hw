struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };

    // & = operatorul de referinta
    let f = &foo;
    println!("{}", f.x);
    // se renunță la f aici
    // se renunță la foo aici

}
