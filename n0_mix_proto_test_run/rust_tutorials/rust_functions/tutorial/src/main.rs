fn main() {
    println!("Hello, world!");
    body_slam();
    printprint("el gringo");
    do_math(3, 4);
    do_maath();
    weirdness();
    maath();
    bruh();
}

fn body_slam() {
    println!("BODYSLAM");
}

fn printprint(printprint : &str) {
    println!("{}", printprint);
}

fn do_math(x : i32, y: i32) {
    println!("{}", x + y);
}

fn do_maath(){
    println!("no");
}

fn weirdness(){
    let number = {
        let x = 2;
        x + 1
    };
    println!("{}", number);
}

fn maath() -> bool {
    println!("false");
    false
}

fn bruh() -> bool {
    println!("true");
    return true;
}