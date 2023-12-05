fn main() {
    println!("Hello, world!");

    // ===== THE STACK =====
    // last in first out
    // first in last out

    // lifo, filo

    // it's a stack, just imagine a stack of things
    // stack objects in a 'stack'
    // new objects push the stack down
    // remove from the top first, that is it

    // everything in the stack has 3 columns
    // address, name, value

    // so yeah stuff is removed from ram at the end of a curly bracket
    // and i'd guess the same happens when you redeclare a variable

    // ah, and the stack only contains immutable 'fixed size' stuff

    // okay and it does it removes stuff from the stack per scope
    // yeah pretty obvious i guess
    // intuitive

    // and function parameters are passed in again
    // again ontop of the old ones (if the old ones were stored in the stack)
    // again, intuitive, as it's basically how you write code anyways

    
    // ==== THE HEAP ====
    // okay so a string that is literally declared gets stored on the stack
    // and a string that is declared as a variable, is stored on the heap
    // and that is because the string is a dynamic array that can change

    // Oh, okay, and you can get pointers to the heap values
    // And store those in the stack
    // Yeah makes sense
}
