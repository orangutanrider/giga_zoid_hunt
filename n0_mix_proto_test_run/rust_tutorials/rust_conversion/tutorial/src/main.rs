use std::io;

fn main() {
    let x: u8 = 9;
    let y: i8 = 10;

    let z = x + (y as u8);
    println!("{}", z);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("msg");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input + 2);
    
    // hmm
    // i think the intelisense is worse than it is in C# and unity
    // though the methods and stuff do have nice documentation tooltips
}
