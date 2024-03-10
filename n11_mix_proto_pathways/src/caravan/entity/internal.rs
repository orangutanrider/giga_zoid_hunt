use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use super::*;

fn lift_entity_binding(mut entity_binding: String, is_there: AdditionalPuncts) -> Result<String, CaravanError> {
    match is_there {
        AdditionalPuncts::Found => {
            return Err(CaravanError::Undefined)
        },
        AdditionalPuncts::NoneFound => { },
    }

    // if format is "to_entity", removes the "to_"
    let to = &entity_binding[..3];
    if to == "to_" {
        entity_binding.replace_range(..3, "");
        return Ok(entity_binding)
    }

    // otherwise adds "_dest" to the end
    entity_binding = entity_binding + "_dest";
    return Ok(entity_binding);
}

fn walk_to_entity_binding_end(mut caravan: Caravan, span: Span,) -> Result<(Caravan, Span, AdditionalPuncts), CaravanError> {
    return join_until_seperator(iter, span, AdditionalPuncts::NoneFound)
}

fn join_until_seperator(mut caravan: Caravan, span: Span, is_there: AdditionalPuncts) -> Result<(Caravan, Span, AdditionalPuncts), CaravanError> {
    let token = iter.next();
    let Some(token) = token else {
        return Err(CaravanError::Undefined);
    };
    
    match token {
        TokenTree::Group(_) => {
            return Err(CaravanError::Undefined);
        },
        TokenTree::Punct(_) => {
            return end_at_seperator(token, iter, span, is_there);
        },
        TokenTree::Ident(_) => {
            let span = span.join(token.span());
            let Some(span) = span else {
                return Err(CaravanError::Undefined);
            };

            return join_until_seperator(iter, span, is_there);
        },
        TokenTree::Literal(_) => {
            let span = span.join(token.span());
            let Some(span) = span else {
                return Err(CaravanError::Undefined);
            };

            return join_until_seperator(iter, span, is_there);
        },
    }
}

fn end_at_seperator(mut caravan: Caravan, current: TokenTree, span: Span, is_there: AdditionalPuncts) -> Result<(Caravan, Span, AdditionalPuncts), CaravanError> {
    // If colon expect :: and end
    if current.to_string() == ":" {
        let next = iter.next();
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
        return Ok((iter, span, is_there));
    }

    // if no colon, continue
    let span = span.join(current.span());
    let Some(span) = span else {
        return Err(CaravanError::Undefined);
    };
    return join_until_seperator(iter, span, AdditionalPuncts::Found);
}