use proc_macros::*;

fn main() {
    println!("Hello World");

    print_end_span_1!( &* );
    print_groups!( ( e ) );
    print_tokens!( ( e ) );

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

    //print_spans!(
    //    let x = 12; &&
    //);

    //print_resolved_spans!(
    //     & & 
    //);

    //print_located_spans!(
    //     & & 
    //);

    //print_joined_spans!(
    //     & & 
    //);

    //print_joined_spans!(
    //    &&
    //);

    //print_tokens!(
    //    let x = 100;
    //);

    //print_kind!(
    //    let z = 30000;
    //);

    //print_kind!(
    //    let n = {
    //        let g = 3;
    //        g + 3
    //    }
    //    //! e
    //);

    //print_kind!(
    //    //! e
    //);

    //print_kind!(({{{{{{{{{{{let e = 1;}}}}}}}}}}}));
    //print_nesting_count!({{{{{{{{{{{let e = 1;}}}}}}}}}}});

    //print_kind!(entity::query(,) -> entity::query(,););
}