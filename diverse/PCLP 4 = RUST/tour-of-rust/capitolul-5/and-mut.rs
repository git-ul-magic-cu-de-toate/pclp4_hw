struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // se renunță la f aici
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    // EȘEC: do_something(foo) va eșua pentru că
    // foo nu poate fi mutat în timp ce este împrumutat în mod mutabil

    // EȘEC: foo.x = 13; va eșua aici pentru că
    // foo nu poate fi modificat în timp ce este împrumutat în mod mutabil

    f.x = 13;
    // se renunță la f aici pentru că nu mai este folosit dupa acest moment
    
    println!("{}", foo.x);
    
    // acest lucru funcționează pentru s-a renunțat la toate referințele mutabile
    foo.x = 7;
    
    // cedăm proprietatea lui foo într-o funcție
    do_something(foo);
}
