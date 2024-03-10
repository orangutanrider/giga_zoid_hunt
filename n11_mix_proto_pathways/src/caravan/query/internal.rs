use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use super::*;

pub fn till_query_fin(caravan: Caravan, current: Span,) -> Result<(Caravan, Span, Group), CaravanError> {
    return join_until_bindings(caravan, current)
}

fn join_until_bindings(mut caravan: Caravan, current: Span) -> Result<(Caravan, Span, Group), CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Err(CaravanError::Undefined);
    };
    
    match token {
        TokenTree::Group(group) => {
            return end_if_bindings(caravan, current, group)
        },
        TokenTree::Punct(_) => {
            let current = current.join(token.span());
            let Some(current) = current else {
                return Err(CaravanError::Undefined);
            };

            return join_until_bindings(caravan, current)
        },
        TokenTree::Ident(_) => {
            let current = current.join(token.span());
            let Some(current) = current else {
                return Err(CaravanError::Undefined);
            };

            return join_until_bindings(caravan, current)
        },
        TokenTree::Literal(_) => {
            let current = current.join(token.span());
            let Some(current) = current else {
                return Err(CaravanError::Undefined);
            };

            return join_until_bindings(caravan, current)
        },
    }
}

fn end_if_bindings(caravan: Caravan, output: Span, current: Group) -> Result<(Caravan, Span, Group), CaravanError> {
    if current.delimiter() != Delimiter::Parenthesis {
        return Err(CaravanError::Undefined)
    }

    return Ok((caravan, output, current))
}
