fn main() {
    // immutable test
    let x: i32 = 4;
    println!("el pootis: {}", x);
    let x: i32 = x + 1; // hmm, feels very smart that you can do this
    println!("EL POOTIS!?!: {}", x);
    
    {
        let x: i32 = 2;
        println!("Hmm, this so big brained that I don't even know how I should use it: {}", x)
        // I guess there are downsides to this too maybe, one could assume that it means loops cannot inherit variables
        // lets test that
    }

    {
        // (test)
        let x: i32 = x - 1;
        println!("HMMMMM?!?!? {}", x);
        // this is some real big-brained stuff
        // it'll take a while to get used to this, but I like it

        // one thing that worries me with this though
        // is that i can imagine it creating more nesting than before
    }

    // mutable test
    let mut y: i32 = 1;
    println!("elfeurte@stinkmail.com: {}", y); 
    y = y + 1;
    println!("elfeurte@stinkmail.com: {}", y);

    // string formatting is strange, but I'd guess that it works
    // ahh, let's test something actually
    println!("StringTest: {} {}", x, y);
    // Yeah this is actually way better, amazing!

    // hmm, why not I guess, it doesn't really do much though
    // probably more of a side-effect of a feature than a feature
    let x: &str = "Gringus";
    println!("El: {}", x);

    // hmm this is interesting though
    let y: &str = "Cancel?";
    println!("HMM: {}", y);

    // what about mutables?
    let mut y: i32 = 1;
    println!("elfeurte@stinkmail.com: {}", y); 
    y = y + 1;
    println!("elfeurte@stinkmail.com: {}", y);

    let mut y: &str = "hmmm";
    y = "I dunno how to add strings tho"; // ah, and it sees that this is pointless and tells me
    println!("{}", y);

    // I guess you can do this too?, kinda useless again, but also once again, I guess why not
    let mut y: i32 = 1;
    println!("elfeurte@stinkmail.com: {}", y); 
    y = y + 1;
    println!("elfeurte@stinkmail.com: {}", y);

    let mut y: i32 = 1;
    println!("elfeurte@stinkmail.com: {}", y); 
    y = y + 1;
    println!("elfeurte@stinkmail.com: {}", y);
    

    // constant test
    const SNAKE_LONG: i32 = 300;
    // Yeah makes sense
    // sortof just as an organistation thing
    // it means you don't have to worry about it being redefined
    println!("SNAKE_LONG: {}", SNAKE_LONG);
}