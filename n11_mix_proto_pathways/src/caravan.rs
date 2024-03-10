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

/// Format: entity_clause::query(bindings) -> ...
#[proc_macro]
pub fn ref_caravan(input: TokenStream) -> TokenStream {
    return TokenStream::new();
}