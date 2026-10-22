use std::env;   // for interacting with the system's environment

fn main() {

    if let Ok(current_dir) = env::current_dir() {
        if let Some(pwd) = current_dir.to_str() {
            println!("The current working directory = {}", pwd);
        }
    }

    // $ echo $PWD      # environment variable
    // if you don't want to see Seme(...), you have to pattern match on Ok()
    println!("The current working directory = {:?}", env::var("PWD").ok());

    // $ echo $PWD      # environment variable
    if let Ok(pwd) = env::var("PWD") {
        println!("The current working directory = {}", pwd);
    }

    // $ echo $USER     # environment variable
    if let Ok(user) = env::var("USER") {
        println!("The current user is: {}", user);
    }


    // $ echo $IDK      # I suppose you didn't set this variable :)
    if let Err(err) = env::var("IDK") {
        eprintln!("IDK : {}", err);
        env::set_var("IDK", "= I don't know");
        println!("IDK = {:?}", env::var("IDK").ok());
        env::remove_var("IDK");
    }
}
