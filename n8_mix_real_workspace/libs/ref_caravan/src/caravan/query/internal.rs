use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use super::*;

pub fn till_query_fin(caravan: Caravan, current: Span,) -> Result<(Caravan, Span, Group), CaravanError> {
    return join_until_bindings(caravan, current)
}

/// Will not check for semi-colon
pub fn query_deep_next(mut caravan: Caravan) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    // get combined symbols
    let span = token.span();
    let token = caravan.next();
    let Some(token) = token else {
        return Err(CaravanError::Undefined);
    };
    let span = span.join(token.span());
    let Some(span) = span else {
        return Err(CaravanError::Undefined)
    };
    let span = span.source_text();
    let Some(span) = span else {
        return Err(CaravanError::Undefined)
    };

    // expect '->'
    if span != "->" {
        return Err(CaravanError::Undefined);
    }

    return entity_step(caravan)
}

pub fn query_surface_next(mut caravan: Caravan) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    let symbol = token.to_string();
    if symbol == ";" {
        return entity_step(caravan)
    }

    if symbol != "-" {
        return Err(CaravanError::Undefined)
    }

    // get combined symbols
    let span = token.span();
    let token = caravan.next();
    let Some(token) = token else {
        return Err(CaravanError::Undefined);
    };
    let span = span.join(token.span());
    let Some(span) = span else {
        return Err(CaravanError::Undefined)
    };
    let span = span.source_text();
    let Some(span) = span else {
        return Err(CaravanError::Undefined)
    };

    // expect '->'
    if span != "->" {
        return Err(CaravanError::Undefined);
    }

    return entity_step(caravan)
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