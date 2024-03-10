use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use super::*;

pub fn punct_to_entity_wildcard(caravan: Caravan, current: TokenTree) -> Result<Caravan, CaravanError> {
    if current.to_string() == "@" {
        return entity_wildcard_step(caravan, SingleEntityStep::Literal)
    }
    
    if current.to_string() == "^" {
        return entity_wildcard_step(caravan, SingleEntityStep::Lifted)
    }

    if current.to_string() == "~" {
        return entity_wildcard_step(caravan, SingleEntityStep::Overlap)
    }

    return Err(CaravanError::Undefined)
}

pub fn entity_wildcard_step(mut caravan: Caravan, kind: SingleEntityStep) -> Result<Caravan, CaravanError> {
    let iter = &mut caravan.iter;

    let token = iter.next();
    let Some(token) = token else {
        return Err(CaravanError::Undefined)
    };
    
    match token {
        TokenTree::Group(_) => {
            return Err(CaravanError::Undefined)
        },
        TokenTree::Ident(_) => {
            return single_entity_step(caravan, token.span(), kind);
        },
        TokenTree::Punct(_) => {
            return Err(CaravanError::Undefined)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::Undefined)
        },
    }
}

pub fn lift_entity_clause(mut entity_clause: String) -> Result<String, CaravanError> {
    // if format is "to_entity", removes the "to_"
    let to = &entity_clause[..3];
    if to == "to_" {
        entity_clause.replace_range(..3, "");
        return Ok(entity_clause)
    }

    // otherwise adds "_dest" to the end
    entity_clause = entity_clause + "_dest";
    return Ok(entity_clause);
}

/// Outputs span of entity clause
pub fn till_entity_clause(caravan: Caravan, current: Span,) -> Result<(Caravan, Span), CaravanError> {
    return join_until_seperator(caravan, current)
}

fn join_until_seperator(mut caravan: Caravan, current: Span) -> Result<(Caravan, Span), CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Err(CaravanError::Undefined);
    };
    
    match token {
        TokenTree::Group(_) => {
            return Err(CaravanError::Undefined);
        },
        TokenTree::Punct(_) => {
            return end_if_seperator(caravan, current, token);
        },
        TokenTree::Ident(_) => {
            let current = current.join(token.span());
            let Some(current) = current else {
                return Err(CaravanError::Undefined);
            };

            return join_until_seperator(caravan, current);
        },
        TokenTree::Literal(_) => {
            let current = current.join(token.span());
            let Some(current) = current else {
                return Err(CaravanError::Undefined);
            };

            return join_until_seperator(caravan, current);
        },
    }
}

fn end_if_seperator(mut caravan: Caravan, output: Span, current: TokenTree) -> Result<(Caravan, Span), CaravanError> {
    // If colon expect :: and end
    if current.to_string() == ":" {
        let next = caravan.next();
        let Some(next) = next else {
            return Err(CaravanError::Undefined);
        };
        
        let seperator = current.span().join(next.span());
        let Some(seperator) = seperator else {
            return Err(CaravanError::Undefined);
        };
        
        let seperator = seperator.source_text();
        let Some(seperator) = seperator else {
            return Err(CaravanError::Undefined);
        };

        if seperator != "::" {
            return Err(CaravanError::Undefined);
        }
        return Ok((caravan, output));
    }

    // if no colon, continue
    let output = output.join(current.span());
    let Some(output) = output else {
        return Err(CaravanError::Undefined);
    };
    return join_until_seperator(caravan, output);
}
