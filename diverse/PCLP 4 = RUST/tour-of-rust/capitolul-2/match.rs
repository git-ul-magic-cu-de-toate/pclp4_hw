fn main() {
    let x = 42;

    match x {
        0 => {
            println!("am găsit zero");
        }
        // putem face un caz pentru mai multe valori
        1 | 2 => {
            println!("am găsit 1 sau 2!");
        }
        // putem face un caz pentru o mulțime
        3..=9 => {
            println!("am găsit un număr între 3 și 9 inclusiv");
        }
        // putem pune numărul care respectă cazul într-o variabilă
        matched_num @ 10..=100 => {
            println!("am găsit numărul {} între 10 și 100!", matched_num);
        }
        // acesta este cazul implicit care trebuie să existe dacă
        // nu sunt abordate toate cazurile posibile
        _ => {
            println!("am găsit alt număr!");
        }
    }
}
