mod entity;
mod query;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

struct Caravan<'o> {
    iter: TokenIter,
    output: &'o mut String, // The collected code output, from all recursions
}
impl<'o> Caravan<'o> {
    fn new(iter: TokenIter, output: &'o mut String) -> Self {
        return Self{
            iter,
            output,
        }
    }
}

impl<'o> Caravan<'o> {
    fn next(&mut self) -> Option<TokenTree> {
        return self.iter.next()
    }
}

enum CaravanError {
    Undefined,
}

/// Format: entity_clause::query(bindings) -> ...
#[proc_macro]
pub fn ref_caravan(input: TokenStream) -> TokenStream {
    return TokenStream::new();
}