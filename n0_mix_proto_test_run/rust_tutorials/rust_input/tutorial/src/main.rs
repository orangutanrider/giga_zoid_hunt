use std::io;

fn main() {
    println!("Hello");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);

    // Hmm
    // I sure hope this game engine has a lot of features for me
    // The idea of having to handle inputs myself just entered my head
    // And yeah... nah
}

