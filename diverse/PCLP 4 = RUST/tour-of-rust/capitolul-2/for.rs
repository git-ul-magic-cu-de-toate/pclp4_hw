fn main() {

    for x in 0..5 {
        print!("{} ", x);
    }
    println!();

    for x in 0..=5 {
        print!("{} ", x);
    }
    println!();

    let nums : [i32; 5] = [10, -1, 9, -80, 1];
    
    print!("{} : ", stringify!(nums));
    for el in nums {
        print!("{} ", el);
    }
    println!();

    print!("{} : ", stringify!(nums));
    for idx in 0 .. nums.len() {
        print!("{} ", nums[idx]);
    }
    println!();
}
