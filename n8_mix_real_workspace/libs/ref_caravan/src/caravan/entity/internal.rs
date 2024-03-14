use proc_macro::*;

use super::*;

pub fn punct_to_entity_wildcard(caravan: Caravan, current: TokenTree) -> Result<Caravan, CaravanError> {
    if current.to_string() == LITERAL {
        return entity_wildcard_step(caravan, SingleEntityStep::Literal)
    }
    
    if current.to_string() == LIFT {
        return entity_wildcard_step(caravan, SingleEntityStep::Lifted)
    }

    if current.to_string() == OVERLAP {
        return entity_wildcard_step(caravan, SingleEntityStep::Overlap)
    }

    return Err(CaravanError::NoMatchingWildcard)
}

pub fn entity_wildcard_step(mut caravan: Caravan, kind: SingleEntityStep) -> Result<Caravan, CaravanError> {
    let iter = &mut caravan.iter;

    let token = iter.next();
    let Some(token) = token else {
        return Err(CaravanError::ExpectedEntityClause)
    };
    
    match token {
        TokenTree::Group(_) => {
            return Err(CaravanError::ExpectedEntityClause)
        },
        TokenTree::Ident(_) => {
            return single_entity_step(caravan, token, kind);
        },
        TokenTree::Punct(_) => {
            return Err(CaravanError::ExpectedEntityClause)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::ExpectedEntityClause)
        },
    }
}

pub fn lift_entity_clause(mut entity_clause: String) -> String {
    // if format is "to_entity", removes the "to_"
    let to = &entity_clause[..3];
    if to == LIFT_REMOVE {
        entity_clause.replace_range(..3, "");
        return entity_clause
    }

    // otherwise adds "_dest" to the end
    entity_clause = entity_clause + LIFT_ADD;
    return entity_clause;
}

/// Outputs entity clause
pub fn collect_entity_clause(caravan: Caravan, current: TokenTree) -> Result<(Caravan, String), CaravanError> {
    let mut vec = Vec::new();
    vec.push(current);
    return collect_until_seperator(caravan, vec)
}

fn collect_until_seperator(mut caravan: Caravan, mut collection: Vec<TokenTree>) -> Result<(Caravan, String), CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Err(CaravanError::ExpectedSeperator);
    };

    match token {
        TokenTree::Group(_) => {
            collection.push(token);
            return collect_until_seperator(caravan, collection);
        },
        TokenTree::Punct(_) => {
            return end_if_seperator(caravan, collection, token);
        },
        TokenTree::Ident(_) => {
            collection.push(token);
            return collect_until_seperator(caravan, collection);
        },
        TokenTree::Literal(_) => {
            collection.push(token);
            return collect_until_seperator(caravan, collection);
        },
    }
}

fn end_if_seperator(mut caravan: Caravan, mut collection: Vec<TokenTree>, current: TokenTree) -> Result<(Caravan, String), CaravanError> {
    // If non :, add to string and continue
    if current.to_string() != ":" {
        collection.push(current);
        return collect_until_seperator(caravan, collection)
    }

    let TokenTree::Punct(current) = current else {
        return Err(CaravanError::Undefined)
    };
    
    // Expect ::
    match current.spacing() {
        Spacing::Joint => (/* continue */),
        Spacing::Alone => return Err(CaravanError::ExpectedSeperator), 
    }
    let Some(current) = caravan.next() else {
        return Err(CaravanError::ExpectedSeperator) 
    };
    let TokenTree::Punct(current) = current else {
        return Err(CaravanError::ExpectedSeperator)
    };
    if current != ':' {
        return Err(CaravanError::ExpectedSeperator) 
    }

    // End
    let output = collection.into_iter();
    let output = TokenStream::from_iter(output);
    let output = output.to_string();
    return Ok((caravan, output))
}
