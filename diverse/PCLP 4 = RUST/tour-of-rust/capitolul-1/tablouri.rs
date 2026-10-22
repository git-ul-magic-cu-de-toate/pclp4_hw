fn main() {
    let nums: [i32; 3] = [1, 2, 3];

    /* println!("{:}", nums); // da eroare
    folosind doar operatorul : , compilatorul nu stie ce afiseaza
    :? intra in modul de debugging, si astfel afiseaza vectorul
    :#? face acelasi lucru, doar ca pune fiecare element pe ccate o linie
    */

    println!("{:?}", nums);
    println!();
    
    println!("{:#?}", nums);
    
    println!("{}", nums[1]);
    println!();

    dbg!(nums);
    println!();
    /* dbg = debugging
    functia dbg! foloseste macrou-ul stringify!, care extrage numele unei variabile
    */

    print!("{} : ", stringify!(nums));
    for el in nums {
        print!("{} ", el);
    }
    println!();

    print!("{} : ", stringify!(nums));
    for idx in 0..nums.len() {
        // idx = {0, 1, .., len - 1}
        print!("{} ", nums[idx]);
    }
    println!();

    print!("{} : ", stringify!(nums));
    for idx in 0 ..= (nums.len() - 1) {
        // idx = {0, 1, .., len - 1}
        print!("{} ", nums[idx]);
    }
    println!();
}
