fn sum_digits(mut x: i32) -> i32 {
    // folosind mut, nu modificam variabila globala
    // ci ne permite sa modificam doar variabila in cadrul functiei
    
    let mut sum = 0_i32;
    while x != 0 {
        sum = sum + x % 10;
        x = x / 10;
    }
    return sum;
}

fn main() {
    let mut x = 0;
    while x != 42 {
        x += 1;
    }
    println!("x este {}", x);

    x = 121;
    println!("suma cifrelor lui {} = {}",
                x, sum_digits(x));
    
}
