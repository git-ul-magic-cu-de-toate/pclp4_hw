struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // se renunță la f aici
}

fn main() {
    let foo = Foo { x: 42 };
    // foo este cedat funcției do_something
    do_something(foo);
    // foo nu mai poate fi folosit

    let mut x:u32 = 5;
    println!("{}", x);
    println!("{}", x);
    println!("{}", x);
    x = 7;
    println!("{}", x);

}
