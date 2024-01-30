//https://www.youtube.com/watch?v=MWRPYBoCEaY
//https://doc.rust-lang.org/book/ch10-02-traits.html
//https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro

// custom derive macros?
// they have to be in their own crate as a library? Hmm...
// So now the question is, do I do that so I can have custom derive?
// I'd rather do it all in the one project. Hmm...
// Hmm. Theoretically it doesn't matter that much.
// Yeah okay I wont do that.
// I already have interfaces figured out here, it is enough.

macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

trait MinMax {
    fn min() -> Self;
    fn max() -> Self;
}
macro_rules! min_max_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl MinMax for $t {
            fn min() -> $t {
                return $min
            }
            fn max() -> $t {
                return $max
            }
        }
    };
}

trait Summary {
    fn summarise(&self) -> String;
}

struct ElGringo(String);
min_max_impl!(
    ElGringo, 
    ElGringo("min".to_string()), 
    ElGringo("max".to_string())
);

impl Summary for ElGringo {
    fn summarise(&self) -> String {
        return "gringo".to_string()
    }
}

fn main() {
    println!("Hello, world!");
    say_hello!();
}