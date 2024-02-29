use proc_macros::*;

fn main() {
    println!("Hello World");

    //print_as_string!(
    //    let x = 3;
    //);

    //turn_into_print!( 
    //    let x = 10; 
    //);

    //println!("{}", x);

    //double!(
    //    println!("el gringo");
    //);

    // Broken if you do a string, so basically it an inverted print 
    //turn_into_print!( 
    //    "let x = 10;"
    //);

    print_spans!(
        let x = 12;
    );

    print_tokens!(
        let x = 100;
    );

    print_kind!(
        let z = 30000;
    );

    print_kind!(
        let n = {
            let g = 3;
            g + 3
        }
    );

    print_kind!(({{{{{{{{{{{let e = 1;}}}}}}}}}}}));
    print_nesting_count!({{{{{{{{{{{let e = 1;}}}}}}}}}}});
}