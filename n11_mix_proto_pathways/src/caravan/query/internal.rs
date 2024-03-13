use proc_macro::*;

use super::*;

pub fn collect_query(caravan: Caravan, current: TokenTree) -> Result<(Caravan, String, Group), CaravanError> {
    return collect_until_bindings(caravan, current.to_string())
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
    if symbol == ";" { // Contine to next statement (End)
        return entity_step(caravan)
    }

    return expect_next(caravan, token);
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

fn collect_until_bindings(mut caravan: Caravan, mut output: String) -> Result<(Caravan, String, Group), CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Err(CaravanError::ExpectedBindings);
    };
    
    match token {
        TokenTree::Group(group) => {
            return end_if_bindings(caravan, output, group)
        },
        TokenTree::Punct(_) => {
            output.push_str(&token.to_string());
            return collect_until_bindings(caravan, output);
        },
        TokenTree::Ident(_) => {
            output.push_str(&token.to_string());
            return collect_until_bindings(caravan, output);
        },
        TokenTree::Literal(_) => {
            output.push_str(&token.to_string());
            return collect_until_bindings(caravan, output);
        },
    }
}

fn end_if_bindings(caravan: Caravan, mut output: String, current: Group) -> Result<(Caravan, String, Group), CaravanError> {
    if current.delimiter() != Delimiter::Parenthesis {
        output.push_str(&current.to_string());
        collect_until_bindings(caravan, output)
    }

    return Ok((caravan, output, current))
}