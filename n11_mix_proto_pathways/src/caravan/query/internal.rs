use proc_macro::*;

use super::*;

pub fn till_query_fin(caravan: Caravan, current: Span,) -> Result<(Caravan, Span, Group), CaravanError> {
    return join_until_bindings(caravan, current)
}

/// Will not check for semi-colon
pub fn query_nested_next(mut caravan: Caravan) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan); // If nothing, exit
    };

    return expect_next(caravan, token)
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
        return Err(CaravanError::ExpectedArrow)
    }

    // get combined symbols
    let span = token.span();
    let token = caravan.next();
    let Some(token) = token else {
        return Err(CaravanError::ExpectedArrow);
    };
    let span = span.join(token.span());
    let Some(span) = span else {
        return Err(CaravanError::JoinSpansError)
    };
    let span = span.source_text();
    let Some(span) = span else {
        return Err(CaravanError::SpanToStringError)
    };

    // expect '->'
    if span != "->" {
        return Err(CaravanError::ExpectedArrow);
    }

    return entity_step(caravan)
}

fn expect_next(mut caravan: Caravan, current: TokenTree) -> Result<Caravan, CaravanError> {
    // Expect arrow ->
    let current = match current {
        TokenTree::Group(_) => return Err(CaravanError::ExpectedArrow),
        TokenTree::Ident(_) => return Err(CaravanError::ExpectedArrow),
        TokenTree::Punct(punct) => punct,
        TokenTree::Literal(_) => return Err(CaravanError::ExpectedArrow),
    };
    if current != '-' {
        return Err(CaravanError::ExpectedArrow)
    }
    match current.spacing() {
        Spacing::Joint => (/* continue */),
        Spacing::Alone => return Err(CaravanError::ExpectedArrow),
    }
    let Some(current) = caravan.next() else {
        return Err(CaravanError::ExpectedArrow)
    };
    let TokenTree::Punct(current) = current else {
        return Err(CaravanError::ExpectedArrow)
    };
    if current != '>' {
        return Err(CaravanError::ExpectedArrow);
    }

    // Go next
    return entity_step(caravan)   
}

fn join_until_bindings(mut caravan: Caravan, current: Span) -> Result<(Caravan, Span, Group), CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Err(CaravanError::ExpectedBindings);
    };
    
    match token {
        TokenTree::Group(group) => {
            return end_if_bindings(caravan, current, group)
        },
        TokenTree::Punct(_) => {
            let current = current.join(token.span());
            let Some(current) = current else {
                return Err(CaravanError::JoinSpansError);
            };

            return join_until_bindings(caravan, current)
        },
        TokenTree::Ident(_) => {
            let current = current.join(token.span());
            let Some(current) = current else {
                return Err(CaravanError::JoinSpansError);
            };

            return join_until_bindings(caravan, current)
        },
        TokenTree::Literal(_) => {
            let current = current.join(token.span());
            let Some(current) = current else {
                return Err(CaravanError::JoinSpansError);
            };

            return join_until_bindings(caravan, current)
        },
    }
}

fn end_if_bindings(caravan: Caravan, output: Span, current: Group) -> Result<(Caravan, Span, Group), CaravanError> {
    if current.delimiter() != Delimiter::Parenthesis {
        return Err(CaravanError::IncorrectDelimiter)
    }

    return Ok((caravan, output, current))
}