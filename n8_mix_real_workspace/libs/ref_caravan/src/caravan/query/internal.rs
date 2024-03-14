use proc_macro::*;

use super::*;

pub fn collect_query(caravan: Caravan, current: TokenTree) -> Result<(Caravan, TokenStream, Group), CaravanError> {
    let mut vec = Vec::new();
    vec.push(current);
    return collect_until_bindings(caravan, vec)
}

/// Will not check for semi-colon
pub fn query_nested_next(mut caravan: Caravan) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan); // If nothing, exit
    };

    if token.to_string() == "," {
        return Ok(caravan);
    }

    return expect_next(caravan, token)
}

pub fn query_surface_next(mut caravan: Caravan) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    let symbol = token.to_string();
    if symbol == ";" { // Contine to next statement (End)
        return entity_step(caravan)
    }

    return expect_next(caravan, token);
}

fn expect_next(mut caravan: Caravan, current: TokenTree) -> Result<Caravan, CaravanError> {
    // Expect arrow ->
    let current = match current {
        TokenTree::Group(_) => return Err(CaravanError::Undefined),
        TokenTree::Ident(_) => return Err(CaravanError::ExpectedBindings),
        TokenTree::Punct(punct) => punct,
        TokenTree::Literal(_) => return Err(CaravanError::ExpectedComma),
    };
    if current != '-' {
        return Err(CaravanError::NoMatchingWildcard)
    }
    match current.spacing() {
        Spacing::Joint => (/* continue */),
        Spacing::Alone => return Err(CaravanError::ExpectedEntityClause),
    }
    let Some(current) = caravan.next() else {
        return Err(CaravanError::ExpectedSeperator)
    };
    let TokenTree::Punct(current) = current else {
        return Err(CaravanError::JoinSpansError)
    };
    if current != '>' {
        return Err(CaravanError::SpanToStringError);
    }

    // Go next
    return entity_step(caravan)   
}

fn collect_until_bindings(mut caravan: Caravan, mut collection: Vec<TokenTree>) -> Result<(Caravan, TokenStream, Group), CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Err(CaravanError::ExpectedBindings);
    };
    
    match token {
        TokenTree::Group(_) => {
            return end_if_bindings(caravan, collection, token)
        },
        TokenTree::Punct(_) => {
            collection.push(token);
            return collect_until_bindings(caravan, collection);
        },
        TokenTree::Ident(_) => {
            collection.push(token);
            return collect_until_bindings(caravan, collection);
        },
        TokenTree::Literal(_) => {
            collection.push(token);
            return collect_until_bindings(caravan, collection);
        },
    }
}

fn end_if_bindings(caravan: Caravan, mut collection: Vec<TokenTree>, current: TokenTree) -> Result<(Caravan, TokenStream, Group), CaravanError> {
    let TokenTree::Group(group) = current.clone() else{
        return Err(CaravanError::Undefined)
    };

    if group.delimiter() != Delimiter::Parenthesis {
        collection.push(current);
        return collect_until_bindings(caravan, collection);
    }

    let output = collection.into_iter();
    let output = TokenStream::from_iter(output);
    return Ok((caravan, output, group))
}