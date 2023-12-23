fn main() {
    // i8
    // i16
    // i32
    // i64
    // i128

    // (non negative)
    // u8
    // u16 (and so on..)

    // f32
    // f64
    let floating_point: f32 = 32.1;
    println!("Single: {}", floating_point);
    let floating_point: f64 = 32.1;
    println!("Double: {}", floating_point);

    let trueFalse: bool = false;
    if trueFalse == true {
        println!("Not printed");
    }
    let trueFalse: bool = true;
    if trueFalse == true {
        println!("Printed");
    }

    // Tuples
    let tup1: (i32, bool, char, &str) = (1, true, 's', "Gringus");
    let tup2: (i8, bool, char, &str) = (1, true, 's', "Gringus");
    // kinda just like a weird struct

    println!("tup1: {} {} {} {}", tup1.0, tup1.1, tup1.2, tup1.3);

    // Arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr[3]);
}
