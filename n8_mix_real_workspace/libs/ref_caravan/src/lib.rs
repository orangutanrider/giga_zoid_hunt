#![feature(proc_macro_span)]
#![feature(iter_next_chunk)]

mod caravan;

use std::str::FromStr;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use caravan::*;

/// Format: entity_clause::query(bindings) -> ...
#[proc_macro]
pub fn ref_caravan(input: TokenStream) -> TokenStream {
    use caravan::*;

    let iter: TokenIter = input.into_iter();
    let mut output = String::new();
    let caravan = Caravan::start(iter, &mut output);
    let caravan = entity_step(caravan);

    if let Err(error) = caravan {
        let error = error.to_stream();
        match error {
            Ok(error) => { return error },
            Err(lex) => { return TokenStream::new(); },
        }
    }
    let Ok(caravan) = caravan else {
        return TokenStream::new();
    };

    let output = caravan.unpack();
    let output = TokenStream::from_str(&output);
    let Ok(output) = output else {
        return TokenStream::new()
    };

    return output
}