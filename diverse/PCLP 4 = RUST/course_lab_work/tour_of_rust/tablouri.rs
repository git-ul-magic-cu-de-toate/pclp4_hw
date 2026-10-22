fn main() {
    let nums: [i32; 3] = [1, 2, 3]; // nu se poate afisa cu {}
    println!("{:?}", nums);     // :? transforma vectorul intr-un string
    println!("{}", nums[1]);
    
    let asm : [u32; 4] = [1, 2, 3, 4];
    println!("{:?}", asm[1]);   // afisare cu DEBG
    println!("{:#?}", asm[1]);   // afisare cu DEBG
    
    println!("{:?}", asm);   // afisare cu DEBG
    println!("{:#?}", asm);   // afisare cu DEBG
    
    // folosind #, formateaza vectorul
}
