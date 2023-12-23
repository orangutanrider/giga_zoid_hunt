fn main() {
    let cond = 2 < 3;
    println!("{}", cond);

    let cond = 2 < (3.3 as i32);
    print!("{}", cond);

    // ! && || 
    // they are applied in this order

    let cond = true && false;
    print!("{}", cond);

    let cond = true && true;
    print!("{}", cond);

    let cond = false && false;
    print!("{}", cond);

    let cond = true || false;
    print!("{}", cond);

    let cond = !(true || false);
    print!("{}", cond);

    let gringus = "el blingo";

    if gringus == "el blingo"{
        println!("bruh");
    }
    else if gringus == "el gringo" {
        println!("no way");
    }
    else{
        println!("fun");
    }
}