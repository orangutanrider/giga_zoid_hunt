mod entity;
mod query;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

struct Caravan {
    iter: TokenIter,
    output: String, // The collected code output, from all recursions
}

enum CaravanError {
    Undefined,
}

#[proc_macro]
pub fn ref_caravan(input: TokenStream) -> TokenStream {
    return TokenStream::new();
}