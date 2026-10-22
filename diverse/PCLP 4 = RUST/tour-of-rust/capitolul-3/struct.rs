struct SeaCreature {
    // String este o structură
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let calamar = SeaCreature{
        animal_type: String::from("Ink one"),
        // name : "Calamar",    asa o sa dea eroare
        name: String::from("Calamar"),
        arms: 2,
        legs: 8,
        weapon: String::from("Ink"),
    };

    println!("{} {} {} {} {} \n", 
            calamar.animal_type, calamar.name, calamar.arms, calamar.legs, calamar.weapon);

    // va genera warning daca variabila nu este utilizata
    let rechin = SeaCreature{
        animal_type:String::from("Pradator"),
        name:String::from("Rechin"),
        arms: 0,
        legs: 0,
        weapon:String::from("theeth"),
    };

    // nu va genera warning daca variabila nu este utilizata
    let mut _delphin = SeaCreature{
        animal_type: String::from("?"),
        name: String::from("Delfin"),
        arms: 0,
        legs: 0,
        weapon: String::from("?"),
    };

    // nu va genera warning daca variabila nu este utilizata
    let mut _testoasa = SeaCreature{
        animal_type: String::from("?"),
        name: String::from("Testoasa"),
        arms: 0,
        legs: 4,
        weapon: String::from("none"),
    };
    
    _testoasa.animal_type = String::from("nu stiu ce sa scriu");
    println!("{} {} \n", stringify!(_testoasa.weapon), _testoasa.weapon);

    // folosind underscore inainte numelui, nu se va genera warning daca variabila nu va fi folosita
    let mut _x = 10;       
    let mut x = 1112;
    println!("{} = {}", stringify!(_x), _x);
    println!("{} = {}", stringify!(x), x);

    _x = 9;
    x = 99999;
    println!("{} = {}", stringify!(_x), _x);
    println!("{} = {}", stringify!(x), x);
}