mod entity;
mod query;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

struct Caravan<'o> {
    iter: TokenIter,
    output: &'o mut String, // The collected code output, from all recursions
    depth: u32,
}
impl<'o> Caravan<'o> {
    fn dig(iter: TokenIter, output: &'o mut String, depth: u32) -> Self {
        return Self {
            iter,
            output,
            depth,
        }
    }
}

impl<'o> Caravan<'o> {
    fn next(&mut self) -> Option<TokenTree> {
        return self.iter.next()
    }

    fn escape(&mut self) {
        self.depth = self.depth - 1;
    }
}

impl<'o> Caravan<'o> {
    fn deeper(&self) -> u32 {
        return self.depth + 1
    }

    fn at_surface(&self) -> bool {
        return self.depth == 0
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